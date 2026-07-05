"""Cross-language auth-companion conformance (#58) — the PYTHON leg.

Iterates ``codegen/conformance/auth-cases.json`` (the single source for the hostile
token-endpoint responses, hostile store files, and canonical store records that every
companion suite must exercise; new cases are added THERE, never here):

- hostile-but-2xx token responses -> the typed :class:`TokenEndpointError` (an
  :class:`AuthError` subclass — never a bare ``KeyError``/``TypeError``/
  ``json.JSONDecodeError`` escaping), and ``tokens.json`` byte-identical afterwards
  (the rotated refresh token is never burned by persisting a blank/expired Bearer);
- hostile store files -> the typed :class:`StoreFormatError`, never a default-filled
  record that makes ``is_authenticated`` lie, and never an untyped exception;
- canonical valid records -> load with exactly the fixture's field values and
  round-trip through this companion's own persist path (the cross-language store
  compatibility check — field names are the shared wire format, #54).

Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs):
same three-test structure, same fixture-shrink guards (>= 8 cases per hostile table).
Monorepo-only: the fixture is resolved by walking up from ``__file__`` to the repo
root (nearest ancestor holding the justfile + README), never from the cwd.
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from oura_toolkit.auth import (
    AuthError,
    ClientCredentials,
    StoreFormatError,
    TokenEndpointError,
    TokenManager,
    TokenStore,
    Tokens,
)


def _repo_root() -> Path:
    """Repo root: nearest ancestor holding the justfile + README (same walk as the
    Rust/TS/Go legs) — `just sdk-test-py` runs pytest from the repo root, but the
    fixture is resolved from __file__ so the suite works from any cwd."""
    directory = Path(__file__).resolve().parent
    while True:
        if (directory / "justfile").is_file() and (directory / "README.md").is_file():
            return directory
        parent = directory.parent
        assert parent != directory, "repo root not found above __file__"
        directory = parent


FIXTURE_PATH = _repo_root() / "codegen" / "conformance" / "auth-cases.json"
FIXTURE = json.loads(FIXTURE_PATH.read_text(encoding="utf-8"))

HOSTILE_TOKEN_RESPONSES = FIXTURE["hostile_token_responses"]
HOSTILE_STORE_FILES = FIXTURE["hostile_store_files"]
VALID_RECORDS = FIXTURE["valid_records"]

CREDENTIALS = ClientCredentials(client_id="cid", client_secret="cs")


def original_tokens() -> Tokens:
    return Tokens(
        access_token="at-original",
        refresh_token="rt-original",
        expires_at=0,  # expired, so the refresh genuinely calls the endpoint
    )


def test_fixture_has_not_shrunk() -> None:
    """Shrink guard: a fixture edit that drops hostile cases weakens EVERY language's
    suite at once — fail loudly here (>= 8 per hostile table, like the other legs)."""
    assert len(HOSTILE_TOKEN_RESPONSES) >= 8, (
        f"fixture shrank? {len(HOSTILE_TOKEN_RESPONSES)} hostile_token_responses cases"
    )
    assert len(HOSTILE_STORE_FILES) >= 8, (
        f"fixture shrank? {len(HOSTILE_STORE_FILES)} hostile_store_files cases"
    )


@pytest.mark.parametrize(
    "case", HOSTILE_TOKEN_RESPONSES, ids=[c["name"] for c in HOSTILE_TOKEN_RESPONSES]
)
def test_hostile_2xx_token_response_fails_typed_and_leaves_the_store_untouched(
    token_endpoint, tmp_path: Path, case: dict
) -> None:
    # raw_body verbatim when present, else the JSON-encoded body — same rule as the
    # Rust leg's ResponseTemplate selection (the conftest mock sends str bodies
    # verbatim and json.dumps's everything else).
    payload = case["raw_body"] if "raw_body" in case else case["body"]
    token_endpoint.handler = lambda form: (200, payload)

    store = TokenStore(tmp_path)
    store.save_credentials(CREDENTIALS)
    store.save_tokens(original_tokens())
    bytes_before = store.tokens_path.read_bytes()

    manager = TokenManager(
        store, CREDENTIALS, original_tokens(), token_url=token_endpoint.url
    )

    with pytest.raises(Exception) as excinfo:
        manager.force_refresh()
    err = excinfo.value
    # Typed: the companion's own error class — never a bare KeyError/TypeError/
    # JSONDecodeError from the decode detonating downstream, and never a mis-filed
    # variant that would trigger remediation hints for a server-side fault.
    assert not isinstance(err, (KeyError, TypeError, json.JSONDecodeError)), (
        f"case {case['name']}: an untyped {type(err).__name__} escaped: {err!r}"
    )
    assert isinstance(err, AuthError), (
        f"case {case['name']}: expected a typed AuthError subclass, "
        f"got {type(err).__name__}: {err!r}"
    )
    assert isinstance(err, TokenEndpointError), (
        f"case {case['name']}: expected the TokenEndpointError variant, "
        f"got {type(err).__name__}"
    )
    assert err.status == 200, (
        f"case {case['name']}: the error must carry the hostile 2xx status, "
        f"got {err.status}"
    )
    assert len(token_endpoint.requests) == 1, (
        f"case {case['name']}: a hostile 2xx must not trigger the 400-reload-retry arm"
    )
    # Burn-prevention: the on-disk record is byte-identical — the still-valid rotated
    # refresh token was never overwritten by a blank/expired Bearer.
    assert store.tokens_path.read_bytes() == bytes_before, (
        f"case {case['name']}: tokens.json must be byte-identical "
        "(store UNTOUCHED, rotation not burned)"
    )


@pytest.mark.parametrize(
    "case", HOSTILE_STORE_FILES, ids=[c["name"] for c in HOSTILE_STORE_FILES]
)
def test_hostile_store_file_fails_with_the_typed_store_format_error(
    tmp_path: Path, case: dict
) -> None:
    store = TokenStore(tmp_path)
    (tmp_path / case["file"]).write_text(case["content"], encoding="utf-8")

    if case["file"] == "tokens.json":
        load = store.load_tokens
    elif case["file"] == "credentials.json":
        load = store.load_credentials
    else:
        pytest.fail(f"fixture names an unknown store file {case['file']!r}")

    # Must raise — never return a default/None-filled record that makes
    # is_authenticated lie — and the raise must be the TYPED store-format error,
    # never an untyped JSONDecodeError/KeyError/TypeError escaping the parse or a
    # field access.
    with pytest.raises(Exception) as excinfo:
        load()
    err = excinfo.value
    assert isinstance(err, StoreFormatError), (
        f"case {case['name']}: expected the typed StoreFormatError, "
        f"got {type(err).__name__}: {err!r}"
    )


def test_canonical_valid_records_load_exactly_and_round_trip(tmp_path: Path) -> None:
    store = TokenStore(tmp_path)
    # json.dumps of the fixture objects — the canonical on-disk wire format shared by
    # every language (source of truth: oura-toolkit-auth's store.rs; #54).
    store.credentials_path.write_text(
        json.dumps(VALID_RECORDS["credentials.json"], indent=2), encoding="utf-8"
    )
    store.tokens_path.write_text(
        json.dumps(VALID_RECORDS["tokens.json"], indent=2), encoding="utf-8"
    )

    creds = store.load_credentials()
    assert creds is not None, "credentials must load"
    assert creds.client_id == "cid-conformance"
    assert creds.client_secret == "cs-conformance"

    tokens = store.load_tokens()
    assert tokens is not None, "tokens must load"
    assert tokens.access_token == "at-conformance"
    assert tokens.refresh_token == "rt-conformance"
    assert tokens.expires_at == 4102444800
    assert tokens.scope == "personal daily"
    assert tokens.token_type == "Bearer"

    # Round-trip: this companion's persist path must re-emit records the loader (and,
    # by the shared fixture, every other language) still reads identically.
    store.save_credentials(creds)
    store.save_tokens(tokens)

    creds2 = store.load_credentials()
    assert creds2 == creds, "credentials must round-trip through the persist path"

    tokens2 = store.load_tokens()
    assert tokens2 is not None, "tokens must reload after the round-trip"
    assert tokens2 == tokens, "tokens must round-trip through the persist path"
    assert tokens2.access_token == "at-conformance"
    assert tokens2.refresh_token == "rt-conformance"
    assert tokens2.expires_at == 4102444800
    assert tokens2.scope == "personal daily"
    assert tokens2.token_type == "Bearer"
