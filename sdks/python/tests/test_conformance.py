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
  by persisting a blank/expired Bearer); a case's ``must_not_echo`` string appears
  nowhere in the error's ``str()``/``repr()``, ``args`` or attributes, nor in any
  error it chains (``__cause__``/``__context__``) — and must actually occur in the
  case's decoded body bytes (a needle absent from its payload would pass vacuously);
- implementation-defined 2xx token responses (a UTF-8 BOM, duplicate keys, deep
  nesting, an integral float ``expires_in``, an upper-case key) -> EITHER a successful
  refresh that persists ``access_token == "at-refreshed"``, a refresh_token of
  ``"rt-refreshed"`` or the prior stored one (never empty), and ``expires_at`` =
  refresh time + 3600, OR the typed
  :class:`TokenEndpointError` (2xx status) with ``tokens.json`` byte-identical — never
  an untyped exception (e.g. ``RecursionError``) or a half-written store;
- hostile store files (``content`` text or ``content_base64`` raw bytes — exactly one;
  e.g. invalid UTF-8) -> the typed :class:`StoreFormatError`, never a default-filled
  record that makes ``is_authenticated`` lie, and never an untyped exception (plus the
  same ``must_not_echo`` rule, needle-in-payload check included — the store holds
  secrets);
- rejected (non-2xx) token responses, with the stored refresh token and the
  credentials' client secret seeded from the table's ``submitted`` values -> the typed
  :class:`TokenEndpointError` carrying the case's ``status``, ``tokens.json``
  byte-identical, ``must_echo`` present in the error text (the body is kept for
  diagnosis), ``must_not_echo`` (a submitted secret the server echoed) absent from
  every text in the error chain (``str``/``repr``/``args``/attributes such as
  ``.body``), and no such text longer than ``max_error_chars``;
- implementation-defined store files (nesting past a parser's depth limit) -> EITHER
  exactly the case's ``expected`` record OR the typed :class:`StoreFormatError` —
  never an untyped exception (e.g. ``RecursionError``);
- successful refreshes from the fixture's stored ``prior`` record -> the persisted
  record carries EXACTLY the case's ``expected`` access_token, refresh_token, scope and
  token_type (an omitted/null/empty refresh_token or token_type, and a blank or
  non-string scope, keep the prior value), and ``expires_at`` = refresh time +
  ``expected.expires_in``;
- canonical valid records -> load with exactly the fixture's field values and
  round-trip through this companion's own persist path (the cross-language store
  compatibility check — field names are the shared wire format, #54).

Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs):
same fixture-shrink guards (>= 30 hostile token responses, >= 5 implementation-defined
token responses, >= 21 hostile store files, >= 4 rejected token responses, >= 1
implementation-defined store file, >= 18 refresh-success cases; >= 4 hostile token
and >= 5 hostile store cases carrying
``must_not_echo``) plus an exact top-level table-set guard, so a renamed or added table
can't be silently skipped.
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
IMPLEMENTATION_DEFINED_TOKEN_RESPONSES = FIXTURE["implementation_defined_token_responses"]
HOSTILE_STORE_FILES = FIXTURE["hostile_store_files"]
REJECTED = FIXTURE["rejected_token_responses"]
REJECTED_SUBMITTED = REJECTED["submitted"]
REJECTED_MAX_ERROR_CHARS = REJECTED["max_error_chars"]
REJECTED_CASES = REJECTED["cases"]
IMPLEMENTATION_DEFINED_STORE_FILES = FIXTURE["implementation_defined_store_files"]
VALID_RECORDS = FIXTURE["valid_records"]
REFRESH_SUCCESS = FIXTURE["refresh_success_cases"]
REFRESH_SUCCESS_PRIOR = REFRESH_SUCCESS["prior"]
REFRESH_SUCCESS_CASES = REFRESH_SUCCESS["cases"]

#: The exact top-level tables this leg iterates. A table added to (or renamed in) the
#: fixture that this suite doesn't know about would otherwise be silently ignored.
EXPECTED_TABLES = {
    "hostile_token_responses",
    "implementation_defined_token_responses",
    "hostile_store_files",
    "rejected_token_responses",
    "implementation_defined_store_files",
    "refresh_success_cases",
    "valid_records",
}

#: The mutually exclusive ways a case gives its token-endpoint response body.
BODY_COLUMNS = ("body", "raw_body", "raw_body_base64")

#: The mutually exclusive ways a case gives a store file's content.
STORE_CONTENT_COLUMNS = ("content", "content_base64")

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


def response_body_bytes(case: dict) -> bytes:
    """The exact bytes the conftest mock sends for the case (same rule as its handler:
    bytes verbatim, a str as UTF-8, anything else json.dumps'd) — the payload a
    ``must_not_echo`` needle is checked against."""
    body = response_body(case)
    if isinstance(body, bytes):
        return body
    if isinstance(body, str):
        return body.encode("utf-8")
    return json.dumps(body).encode("utf-8")


def store_file_bytes(case: dict) -> bytes:
    """The case's raw store-file bytes: ``content`` -> its UTF-8 encoding,
    ``content_base64`` -> the decoded bytes (bytes JSON can't hold, e.g. invalid
    UTF-8). Exactly one column must be present — an ambiguous case is a fixture bug."""
    present = [column for column in STORE_CONTENT_COLUMNS if column in case]
    assert len(present) == 1, (
        f"case {case['name']}: expected exactly one of {STORE_CONTENT_COLUMNS}, "
        f"got {present}"
    )
    column = present[0]
    if column == "content_base64":
        return base64.b64decode(case[column], validate=True)
    return case[column].encode("utf-8")


def assert_needle_in_payload(case: dict, payload: bytes) -> None:
    """Vacuity guard for the ``must_not_echo`` rule: the needle must actually occur in
    the case's payload (decoded body/content bytes). A needle the input never
    contains can't leak, so the no-echo check would pass whatever the error says."""
    secret = case.get("must_not_echo")
    if secret is None:
        return
    assert isinstance(secret, str) and secret, (
        f"case {case['name']}: must_not_echo must be a non-empty string"
    )
    assert secret.encode("utf-8") in payload, (
        f"case {case['name']}: must_not_echo {secret!r} does not occur in the case's "
        "payload — the no-echo check would be vacuous"
    )


def error_texts(err: BaseException) -> list:
    """Every text a leak could surface through, for each error in ``err``'s chain:
    ``str()``/``repr()`` of the error, of each of its ``args``, and of each instance
    attribute (e.g. ``TokenEndpointError.body``) — a traceback, log line, or caller
    formatting ``err.body`` can render any of them. Returns (label, text) pairs."""
    texts: list = []
    for link in error_chain(err):
        name = type(link).__name__
        texts.append((f"str() of the chained {name}", str(link)))
        texts.append((f"repr() of the chained {name}", repr(link)))
        for i, arg in enumerate(link.args):
            texts.append((f"{name}.args[{i}]", str(arg)))
            texts.append((f"repr({name}.args[{i}])", repr(arg)))
        for attr, value in vars(link).items():
            texts.append((f"{name}.{attr}", str(value)))
            texts.append((f"repr({name}.{attr})", repr(value)))
    return texts


def error_chain(err: BaseException) -> list:
    """``err`` plus every error it chains — ``__cause__`` (``raise ... from``) and
    ``__context__`` (implicit), recursively; a visited set guards against cycles."""
    chain: list = []
    seen: set = set()
    pending = [err]
    while pending:
        current = pending.pop()
        if current is None or id(current) in seen:
            continue
        seen.add(id(current))
        chain.append(current)
        pending.extend((current.__cause__, current.__context__))
    return chain


def assert_does_not_echo(case: dict, err: BaseException) -> None:
    """The fixture's ``must_not_echo`` rule: the secret appears NOWHERE in the error's
    text or in any error it chains (a parser message can quote the input, and the
    input carries token material). Every text :func:`error_texts` yields is checked
    — ``str()``/``repr()``, ``args`` and attributes — since any of them can be
    rendered."""
    secret = case.get("must_not_echo")
    if secret is None:
        return
    assert secret, f"case {case['name']}: must_not_echo must be a non-empty string"
    for label, text in error_texts(err):
        assert secret not in text, (
            f"case {case['name']}: must_not_echo contract — the secret "
            f"{secret!r} leaked into {label}"
        )


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
    suite at once — fail loudly here (>= 30 hostile token responses, >= 5
    implementation-defined token responses, >= 21 hostile store files, >= 4 rejected
    token responses, >= 1 implementation-defined store file, >= 18 refresh-success
    cases, like the other
    legs). pytest SKIPS a parametrize over an
    empty list, so an emptied table would otherwise pass silently."""
    assert len(HOSTILE_TOKEN_RESPONSES) >= 30, (
        f"fixture shrank? {len(HOSTILE_TOKEN_RESPONSES)} hostile_token_responses cases"
    )
    assert len(IMPLEMENTATION_DEFINED_TOKEN_RESPONSES) >= 5, (
        "fixture shrank? "
        f"{len(IMPLEMENTATION_DEFINED_TOKEN_RESPONSES)} "
        "implementation_defined_token_responses cases"
    )
    assert len(HOSTILE_STORE_FILES) >= 21, (
        f"fixture shrank? {len(HOSTILE_STORE_FILES)} hostile_store_files cases"
    )
    assert len(REJECTED_CASES) >= 4, (
        f"fixture shrank? {len(REJECTED_CASES)} rejected_token_responses cases"
    )
    assert len(IMPLEMENTATION_DEFINED_STORE_FILES) >= 1, (
        "fixture shrank? "
        f"{len(IMPLEMENTATION_DEFINED_STORE_FILES)} "
        "implementation_defined_store_files cases"
    )
    assert len(REFRESH_SUCCESS_CASES) >= 18, (
        f"fixture shrank? {len(REFRESH_SUCCESS_CASES)} refresh_success_cases cases"
    )


def test_must_not_echo_coverage_has_not_shrunk() -> None:
    """Floor on the no-echo cases: dropping ``must_not_echo`` from a case (or deleting
    the case) silently retires a leak check in every language at once — at least 4
    hostile token responses and 5 hostile store files must carry one."""
    token_needles = [c["name"] for c in HOSTILE_TOKEN_RESPONSES if "must_not_echo" in c]
    store_needles = [c["name"] for c in HOSTILE_STORE_FILES if "must_not_echo" in c]
    assert len(token_needles) >= 4, (
        "fixture shrank? only "
        f"{len(token_needles)} hostile_token_responses carry must_not_echo: "
        f"{token_needles}"
    )
    assert len(store_needles) >= 5, (
        "fixture shrank? only "
        f"{len(store_needles)} hostile_store_files carry must_not_echo: {store_needles}"
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
    assert_needle_in_payload(case, response_body_bytes(case))
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
    assert_does_not_echo(case, err)
    # Burn-prevention: the on-disk record is byte-identical — the still-valid rotated
    # refresh token was never overwritten by a blank/expired Bearer.
    assert store.tokens_path.read_bytes() == bytes_before, (
        f"case {case['name']}: tokens.json must be byte-identical "
        "(store UNTOUCHED, rotation not burned)"
    )


@pytest.mark.parametrize(
    "case", REJECTED_CASES, ids=[c["name"] for c in REJECTED_CASES]
)
def test_rejected_token_response_fails_typed_redacted_and_bounded(
    token_endpoint, tmp_path: Path, case: dict
) -> None:
    """A non-2xx token-endpoint response: the refresh sends the table's ``submitted``
    secrets, the server answers ``status`` + ``raw_body`` to EVERY request (so a 400's
    one reload-retry sees the same answer), and the error must be the typed
    TokenEndpointError carrying ``status`` whose text keeps ``must_echo`` (diagnosis),
    never contains ``must_not_echo`` anywhere in its chain (a submitted secret the
    server echoed), and never exceeds ``max_error_chars`` — with tokens.json
    byte-identical."""
    contract = (
        "rejected_token_responses contract: typed TokenEndpointError carrying the "
        "status, store untouched, must_echo kept, submitted secrets redacted, no "
        f"error text over max_error_chars ({REJECTED_MAX_ERROR_CHARS})"
    )
    raw_body = case["raw_body"]
    assert isinstance(raw_body, str), f"case {case['name']}: raw_body must be a string"
    assert_needle_in_payload(case, raw_body.encode("utf-8"))
    status = case["status"]
    assert not 200 <= status < 300, (
        f"case {case['name']}: a rejected response must carry a non-2xx status"
    )
    token_endpoint.handler = lambda form: (status, raw_body)

    submitted_refresh = REJECTED_SUBMITTED["refresh_token"]
    submitted_secret = REJECTED_SUBMITTED["client_secret"]
    credentials = ClientCredentials(client_id="cid", client_secret=submitted_secret)
    seeded = Tokens(
        access_token="at-original",
        refresh_token=submitted_refresh,
        expires_at=0,  # expired, so the refresh genuinely calls the endpoint
    )
    store = TokenStore(tmp_path)
    store.save_credentials(credentials)
    store.save_tokens(seeded)
    bytes_before = store.tokens_path.read_bytes()

    manager = TokenManager(store, credentials, seeded, token_url=token_endpoint.url)

    with pytest.raises(Exception) as excinfo:
        manager.force_refresh()
    err = excinfo.value
    assert isinstance(err, TokenEndpointError), (
        f"case {case['name']}: expected the typed TokenEndpointError, "
        f"got {type(err).__name__} ({contract})"
    )
    assert err.status == status, (
        f"case {case['name']}: the error must carry HTTP {status}, got {err.status} "
        f"({contract})"
    )
    # Harness sanity: the refresh really submitted the seeded secrets, so a
    # must_not_echo needle equal to one of them is a genuine echo-back.
    assert token_endpoint.requests, f"case {case['name']}: the endpoint was never called"
    for form in token_endpoint.requests:
        assert form.get("refresh_token") == submitted_refresh, (
            f"case {case['name']}: the refresh must submit the seeded refresh token"
        )
        assert form.get("client_secret") == submitted_secret, (
            f"case {case['name']}: the refresh must submit the seeded client secret"
        )
    assert case["must_echo"] in str(err), (
        f"case {case['name']}: must_echo {case['must_echo']!r} must appear in the "
        f"error text — the body is kept for diagnosis ({contract})"
    )
    texts = error_texts(err)
    secret = case.get("must_not_echo")
    for label, text in texts:
        if secret is not None:
            assert secret not in text, (
                f"case {case['name']}: must_not_echo contract — the submitted secret "
                f"{secret!r} leaked into {label} ({contract})"
            )
        assert len(text) <= REJECTED_MAX_ERROR_CHARS, (
            f"case {case['name']}: max_error_chars contract — {label} is "
            f"{len(text)} chars, over {REJECTED_MAX_ERROR_CHARS} ({contract})"
        )
    assert store.tokens_path.read_bytes() == bytes_before, (
        f"case {case['name']}: tokens.json must be byte-identical ({contract})"
    )


@pytest.mark.parametrize(
    "case",
    IMPLEMENTATION_DEFINED_TOKEN_RESPONSES,
    ids=[c["name"] for c in IMPLEMENTATION_DEFINED_TOKEN_RESPONSES],
)
def test_implementation_defined_2xx_token_response_succeeds_or_fails_typed_cleanly(
    token_endpoint, tmp_path: Path, case: dict
) -> None:
    """Either outcome is allowed, but only in its clean form: a success persists
    access_token == "at-refreshed", a refresh_token that is "rt-refreshed" or the
    prior stored one (never empty), and expires_at = refresh time + 3600 (the whole
    record reloads from disk); a failure is
    the typed TokenEndpointError carrying the 2xx status with tokens.json
    byte-identical. An untyped exception, a wrong persisted token, or a half-written
    store fails, naming the case. Seeded exactly like the hostile harness."""
    payload = response_body(case)
    token_endpoint.handler = lambda form: (200, payload)

    store = TokenStore(tmp_path)
    store.save_credentials(CREDENTIALS)
    store.save_tokens(original_tokens())
    bytes_before = store.tokens_path.read_bytes()

    manager = TokenManager(
        store, CREDENTIALS, original_tokens(), token_url=token_endpoint.url
    )

    contract = (
        "implementation_defined_token_responses contract: EITHER succeed and persist "
        "access_token 'at-refreshed', refresh_token 'rt-refreshed' or the prior one "
        "(never empty), expires_at = refresh time + 3600, OR fail with the typed "
        "TokenEndpointError (2xx) and leave tokens.json untouched"
    )
    prior_refresh_token = original_tokens().refresh_token
    t0 = int(time.time())
    try:
        manager.force_refresh()
    except Exception as err:  # noqa: BLE001 — classifying ANY escape is the point
        assert isinstance(err, TokenEndpointError), (
            f"case {case['name']}: an untyped {type(err).__name__} escaped ({contract})"
        )
        assert 200 <= err.status < 300, (
            f"case {case['name']}: the typed error must carry the 2xx status, "
            f"got {err.status} ({contract})"
        )
        assert store.tokens_path.read_bytes() == bytes_before, (
            f"case {case['name']}: a failed refresh must leave tokens.json "
            f"byte-identical ({contract})"
        )
    else:
        t1 = int(time.time())
        persisted = store.load_tokens()  # a half-written record raises StoreFormatError
        assert persisted is not None, (
            f"case {case['name']}: a successful refresh must persist tokens ({contract})"
        )
        assert persisted.access_token == "at-refreshed", (
            f"case {case['name']}: a successful refresh must persist access_token "
            f"'at-refreshed', got {persisted.access_token!r} ({contract})"
        )
        assert persisted.refresh_token in ("rt-refreshed", prior_refresh_token), (
            f"case {case['name']}: a successful refresh must persist refresh_token "
            f"'rt-refreshed' or the prior {prior_refresh_token!r}, "
            f"got {persisted.refresh_token!r} ({contract})"
        )
        assert t0 + 3600 <= persisted.expires_at <= t1 + 3600, (
            f"case {case['name']}: expires_at must be (time of the refresh) + 3600, "
            f"i.e. within [{t0 + 3600}, {t1 + 3600}], got {persisted.expires_at} "
            f"({contract})"
        )
        assert manager.access_token() == "at-refreshed", (
            f"case {case['name']}: the manager must hand out the persisted token "
            f"({contract})"
        )
    assert len(token_endpoint.requests) == 1, (
        f"case {case['name']}: the refresh must call the token endpoint exactly once"
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
    content = store_file_bytes(case)
    assert_needle_in_payload(case, content)
    (tmp_path / case["file"]).write_bytes(content)

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
        f"got {type(err).__name__}"
    )
    assert_does_not_echo(case, err)


@pytest.mark.parametrize(
    "case",
    IMPLEMENTATION_DEFINED_STORE_FILES,
    ids=[c["name"] for c in IMPLEMENTATION_DEFINED_STORE_FILES],
)
def test_implementation_defined_store_file_loads_exactly_or_fails_typed(
    tmp_path: Path, case: dict
) -> None:
    """Either outcome is allowed, but only in its clean form: the load returns EXACTLY
    the case's ``expected`` fields, or raises the typed StoreFormatError. An untyped
    exception (e.g. the RecursionError json.loads raises past the interpreter's
    recursion limit) or a record with other values fails, naming the case."""
    store = TokenStore(tmp_path)
    (tmp_path / case["file"]).write_bytes(store_file_bytes(case))
    assert case["file"] == "tokens.json", (
        f"case {case['name']}: this harness only knows tokens.json, "
        f"got {case['file']!r}"
    )
    contract = (
        "implementation_defined_store_files contract: EITHER load exactly `expected` "
        "OR fail with the typed StoreFormatError"
    )
    expected = case["expected"]
    try:
        tokens = store.load_tokens()
    except Exception as err:  # noqa: BLE001 — classifying ANY escape is the point
        assert isinstance(err, StoreFormatError), (
            f"case {case['name']}: an untyped {type(err).__name__} escaped ({contract})"
        )
    else:
        assert tokens is not None, (
            f"case {case['name']}: an existing record must not load as None ({contract})"
        )
        for field, value in expected.items():
            assert getattr(tokens, field) == value, (
                f"case {case['name']}: loaded {field} must be {value!r}, "
                f"got {getattr(tokens, field)!r} ({contract})"
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
