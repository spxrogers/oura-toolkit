"""Cross-language auth-companion conformance (#58) — the PYTHON leg.

Iterates ``codegen/conformance/auth-cases.json`` (the single source for the hostile
token-endpoint responses, hostile store files, successful-refresh fallbacks, and
canonical store records that every companion suite must exercise; new cases are added
THERE, never here — its ``$comment`` is the contract):

- hostile-but-2xx token responses (``body`` JSON, ``raw_body`` verbatim string, or
  ``raw_body_base64`` verbatim bytes — e.g. invalid UTF-8) -> the typed
  :class:`TokenEndpointError` (an :class:`AuthError` subclass — never a bare
  ``KeyError``/``TypeError``/``OverflowError``/``json.JSONDecodeError`` escaping), and
  ``tokens.json`` byte-identical afterwards (the rotated refresh token is never burned
  by persisting a blank/expired Bearer);
- hostile store files -> the typed :class:`StoreFormatError`, never a default-filled
  record that makes ``is_authenticated`` lie, and never an untyped exception;
- successful refreshes from the fixture's stored ``prior`` record -> the persisted
  record carries EXACTLY the case's ``expected`` access_token, refresh_token, scope and
  token_type (an omitted/null/empty refresh_token or token_type, and a blank or
  non-string scope, keep the prior value), and ``expires_at`` = refresh time +
  ``expected.expires_in``;
- canonical valid records -> load with exactly the fixture's field values and
  round-trip through this companion's own persist path (the cross-language store
  compatibility check — field names are the shared wire format, #54).

Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs):
same fixture-shrink guards (>= 24 hostile token responses, >= 8 hostile store files,
>= 17 refresh-success cases) plus an exact top-level table-set guard, so a renamed or
added table can't be silently skipped.
Monorepo-only: the fixture is resolved by walking up from ``__file__`` to the repo
root (nearest ancestor holding the justfile + README), never from the cwd.
"""

from __future__ import annotations

import base64
import json
import time
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
REFRESH_SUCCESS = FIXTURE["refresh_success_cases"]
REFRESH_SUCCESS_PRIOR = REFRESH_SUCCESS["prior"]
REFRESH_SUCCESS_CASES = REFRESH_SUCCESS["cases"]

#: The exact top-level tables this leg iterates. A table added to (or renamed in) the
#: fixture that this suite doesn't know about would otherwise be silently ignored.
EXPECTED_TABLES = {
    "hostile_token_responses",
    "hostile_store_files",
    "refresh_success_cases",
    "valid_records",
}

#: The mutually exclusive ways a case gives its token-endpoint response body.
BODY_COLUMNS = ("body", "raw_body", "raw_body_base64")

CREDENTIALS = ClientCredentials(client_id="cid", client_secret="cs")


def original_tokens() -> Tokens:
    return Tokens(
        access_token="at-original",
        refresh_token="rt-original",
        expires_at=0,  # expired, so the refresh genuinely calls the endpoint
    )


def response_body(case: dict) -> object:
    """The case's response body, for the conftest mock: ``raw_body_base64`` -> the
    decoded bytes (sent verbatim — bytes JSON can't hold, e.g. invalid UTF-8),
    ``raw_body`` -> the string (sent verbatim), ``body`` -> the JSON value (json.dumps'd
    by the mock). Exactly one column must be present — an ambiguous case is a fixture
    bug, not something to resolve by precedence."""
    present = [column for column in BODY_COLUMNS if column in case]
    assert len(present) == 1, (
        f"case {case['name']}: expected exactly one of {BODY_COLUMNS}, got {present}"
    )
    column = present[0]
    if column == "raw_body_base64":
        return base64.b64decode(case[column], validate=True)
    return case[column]


def test_fixture_tables_are_exactly_the_ones_this_suite_iterates() -> None:
    """Table-set guard: a renamed table (e.g. refresh_scope_cases ->
    refresh_success_cases) would KeyError at import, but an ADDED table would be
    silently ignored — pin the exact set so every fixture table has a harness here."""
    tables = set(FIXTURE) - {"$comment"}
    assert tables == EXPECTED_TABLES, (
        "auth-cases.json top-level tables changed: "
        f"unknown to this suite {sorted(tables - EXPECTED_TABLES)}, "
        f"missing from the fixture {sorted(EXPECTED_TABLES - tables)}"
    )


def test_fixture_has_not_shrunk() -> None:
    """Shrink guard: a fixture edit that drops hostile cases weakens EVERY language's
    suite at once — fail loudly here (>= 24 hostile token responses, >= 8 hostile store
    files, >= 17 refresh-success cases, like the other legs). pytest SKIPS a
    parametrize over an empty list, so an emptied table would otherwise pass silently."""
    assert len(HOSTILE_TOKEN_RESPONSES) >= 24, (
        f"fixture shrank? {len(HOSTILE_TOKEN_RESPONSES)} hostile_token_responses cases"
    )
    assert len(HOSTILE_STORE_FILES) >= 8, (
        f"fixture shrank? {len(HOSTILE_STORE_FILES)} hostile_store_files cases"
    )
    assert len(REFRESH_SUCCESS_CASES) >= 17, (
        f"fixture shrank? {len(REFRESH_SUCCESS_CASES)} refresh_success_cases cases"
    )


@pytest.mark.parametrize(
    "case", HOSTILE_TOKEN_RESPONSES, ids=[c["name"] for c in HOSTILE_TOKEN_RESPONSES]
)
def test_hostile_2xx_token_response_fails_typed_and_leaves_the_store_untouched(
    token_endpoint, tmp_path: Path, case: dict
) -> None:
    # raw_body_base64 / raw_body verbatim, else the JSON-encoded body — same rule as
    # the Rust leg's ResponseTemplate selection.
    payload = response_body(case)
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
    assert not isinstance(
        err, (KeyError, TypeError, OverflowError, json.JSONDecodeError)
    ), (
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
    "case", REFRESH_SUCCESS_CASES, ids=[c["name"] for c in REFRESH_SUCCESS_CASES]
)
def test_refresh_success_cases_persist_exactly_the_expected_record(
    token_endpoint, tmp_path: Path, case: dict
) -> None:
    payload = response_body(case)
    token_endpoint.handler = lambda form: (200, payload)

    prior = Tokens(
        access_token=REFRESH_SUCCESS_PRIOR["access_token"],
        refresh_token=REFRESH_SUCCESS_PRIOR["refresh_token"],
        expires_at=0,  # already expired, so the refresh genuinely calls the endpoint
        scope=REFRESH_SUCCESS_PRIOR["scope"],
        token_type=REFRESH_SUCCESS_PRIOR["token_type"],
    )
    store = TokenStore(tmp_path)
    store.save_credentials(CREDENTIALS)
    store.save_tokens(prior)

    manager = TokenManager(store, CREDENTIALS, prior, token_url=token_endpoint.url)
    expected = case["expected"]
    t0 = int(time.time())
    returned = manager.access_token()  # the proactive path: prior is expired
    t1 = int(time.time())

    assert len(token_endpoint.requests) == 1, (
        f"case {case['name']}: the refresh must call the token endpoint exactly once"
    )
    assert token_endpoint.requests[0].get("refresh_token") == prior.refresh_token, (
        f"case {case['name']}: the refresh must send the prior refresh token"
    )
    assert returned == expected["access_token"], (
        f"case {case['name']}: access_token() must hand out the refreshed token, "
        f"got {returned!r}"
    )
    persisted = store.load_tokens()
    assert persisted is not None, f"case {case['name']}: tokens must be persisted"
    for field in ("access_token", "refresh_token", "scope", "token_type"):
        assert getattr(persisted, field) == expected[field], (
            f"case {case['name']}: refresh_success_cases contract — persisted {field} "
            f"must be {expected[field]!r} (an omitted/null/empty refresh_token or "
            "token_type, or a blank/non-string scope, keeps the prior "
            f"{REFRESH_SUCCESS_PRIOR.get(field)!r}), got {getattr(persisted, field)!r}"
        )
    lifetime = expected["expires_in"]
    assert t0 + lifetime <= persisted.expires_at <= t1 + lifetime, (
        f"case {case['name']}: expires_at must be (time of the refresh) + "
        f"{lifetime}, i.e. within [{t0 + lifetime}, {t1 + lifetime}], "
        f"got {persisted.expires_at}"
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
