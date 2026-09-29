"""The runtime auth layer: a :class:`TokenManager` that owns the token state, refreshes
it proactively, and hands the generated ``oura_toolkit.api`` client a Configuration
whose Bearer token is always fresh.

Refresh strategy: **proactive** — the manager refreshes when the access token is expired
or within a skew window, so requests carry a valid token. Reactive refresh-on-401 is the
caller's move via :meth:`TokenManager.force_refresh` (retry the request once after it).

Cross-process safety (the same protocol as the Rust crate, issue #22): Oura invalidates
the previous refresh token on every rotation, and this store is shared with the ``oura``
CLI and its long-running MCP server. Every refresh therefore runs under the store's
exclusive advisory lock and **re-reads the store first** — if another process already
rotated, its fresher tokens are adopted instead of burning (and thereby invalidating)
that rotation with a second refresh. A refresh that still 400s is retried once against
freshly reloaded disk state before surfacing "re-login". On Unix the lock genuinely
excludes the Rust processes (both sides ``flock``); on Windows the lock is best-effort
across implementations and the reload-adopt + 400-reload-retry protocol is the
universal guarantee.
"""

from __future__ import annotations

import json
import threading
import time
from typing import TYPE_CHECKING, Optional
from urllib.parse import urlencode

import urllib3

from . import metadata
from .errors import (
    MissingClientCredentialsError,
    NotAuthenticatedError,
    TokenEndpointError,
)
from .store import ClientCredentials, TokenStore, Tokens

if TYPE_CHECKING:  # pragma: no cover
    from oura_toolkit.api.configuration import Configuration

#: Refresh this many seconds before the token's actual expiry.
DEFAULT_SKEW_SECS = 60

#: Hard timeout (seconds) on each token-endpoint call. This bounds how long the store's
#: exclusive lock can be held (the refresh runs under it) — without it, one process's
#: stalled refresh would wedge every other process waiting on the lock. Worst case is
#: ~2x this value: the 400-retry arm can chain a second endpoint call under the same lock.
TOKEN_ENDPOINT_TIMEOUT = 30.0

#: Largest accepted token-endpoint ``expires_in`` (seconds; i32::MAX, ~68 years). The
#: conformance contract (#58, ``auth-cases.json``) requires a JSON integer in
#: ``1..=MAX_EXPIRES_IN_SECS``: the cap keeps ``now + expires_in`` exact and readable by
#: EVERY companion's store — incl. Rust's i64 ``expires_at`` and JavaScript's
#: double-backed numbers — so a Python-persisted record can never become a hostile store
#: file for another language. Anything larger is a broken server, not a real lifetime.
MAX_EXPIRES_IN_SECS = 2_147_483_647


def _is_valid_unicode(value: str) -> bool:
    """False when ``value`` holds a lone surrogate (which ``json.loads`` accepts from a
    ``\\uD800``-style escape) — i.e. it can't be encoded as UTF-8 and so can't be
    persisted."""
    try:
        value.encode("utf-8")
    except UnicodeEncodeError:
        return False
    return True


class TokenManager:
    """Owns the current tokens and the machinery to keep them fresh. Thread-safe
    (an internal mutex serializes token access within the process; the store lock
    serializes rotation across processes).

    ``repr()`` never contains credentials or tokens.
    """

    def __init__(
        self,
        store: TokenStore,
        credentials: Optional[ClientCredentials] = None,
        tokens: Optional[Tokens] = None,
        *,
        skew_secs: int = DEFAULT_SKEW_SECS,
        token_url: Optional[str] = None,
    ) -> None:
        """Construct from an explicit store + optional in-memory records.

        Both records are independently optional: credentials-without-tokens is
        ``auth setup`` done but no login yet; tokens-without-credentials is a
        caller-supplied token that can be used until expiry but not refreshed
        (:class:`MissingClientCredentialsError`).

        ``token_url`` overrides the spec-derived token endpoint — a test seam for
        hermetic mock servers, never needed in production.
        """
        self._store = store
        self._credentials = credentials
        self._tokens = tokens
        self._skew_secs = skew_secs
        self._token_url = token_url if token_url is not None else metadata.TOKEN_URL
        self._mutex = threading.Lock()
        # A dedicated pool (no auth, no retries) for token-endpoint calls. The timeout
        # is load-bearing: the call runs under the store's exclusive lock, so an
        # unbounded hang would block other processes too.
        self._http = urllib3.PoolManager(
            timeout=urllib3.Timeout(total=TOKEN_ENDPOINT_TIMEOUT), retries=False
        )

    @classmethod
    def load(cls, store: Optional[TokenStore] = None) -> "TokenManager":
        """Load from the (default) token store. Absent records are not an error —
        :meth:`access_token` raises :class:`NotAuthenticatedError` on first use."""
        store = store if store is not None else TokenStore()
        return cls(store, store.load_credentials(), store.load_tokens())

    @property
    def store(self) -> TokenStore:
        """The underlying token store."""
        return self._store

    def is_authenticated(self) -> bool:
        """Whether tokens are loaded (does not validate them, and does not imply a
        refresh is possible — refresh additionally needs the client credentials)."""
        with self._mutex:
            return self._tokens is not None

    def access_token(self) -> str:
        """A valid access token, refreshing (and persisting the rotation) if the
        current one is expired or within the skew window."""
        with self._mutex:
            if self._tokens is None:
                raise NotAuthenticatedError()
            if self._tokens.is_expired(self._skew_secs):
                self._refresh_critical_section()
            assert self._tokens is not None
            return self._tokens.access_token

    def force_refresh(self) -> None:
        """Force a refresh regardless of expiry (call this after a 401, then retry the
        request once). If another process already rotated, its fresher tokens are
        adopted instead of burning that rotation with a second refresh."""
        with self._mutex:
            self._refresh_critical_section()

    def configuration(self) -> "Configuration":
        """A ready ``oura_toolkit.api.Configuration`` whose ``access_token`` is
        sourced from this manager on every read — each request through the generated
        client carries a proactively refreshed Bearer token.

        Example::

            from oura_toolkit.api import ApiClient
            from oura_toolkit.auth import TokenManager

            manager = TokenManager.load()
            with ApiClient(manager.configuration()) as client:
                ...
        """
        from ._config import RefreshingConfiguration

        return RefreshingConfiguration(self)

    def __repr__(self) -> str:
        with self._mutex:
            authenticated = self._tokens is not None
        return (
            f"TokenManager(store={self._store!r}, "
            f"credentials={'[REDACTED]' if self._credentials else None}, "
            f"authenticated={authenticated})"
        )

    # -- the reload -> refresh -> persist critical section ---------------------------

    def _refresh_critical_section(self) -> None:
        """Runs under the store's exclusive advisory lock so only one process rotates
        at a time. Caller holds ``self._mutex``.

        The adopt rule covers both entry points: if disk holds tokens that differ from
        memory and aren't expired, another process already rotated — adopt them. (On
        the proactive path memory is expired, so anything fresher is strictly better;
        on the ``force`` path memory just 401'd, so a *different* fresh token is the
        fix and an *identical* one means we must rotate.)
        """
        if self._credentials is None:
            raise MissingClientCredentialsError()

        with self._store.lock_exclusive():
            disk = self._store.load_tokens()
            if disk is not None:
                mem = self._tokens
                differs = mem is None or mem.access_token != disk.access_token
                if differs and not disk.is_expired(self._skew_secs):
                    self._tokens = disk
                    return
                # Refresh from the freshest persisted rotation, never stale memory.
                self._tokens = disk
            current = self._tokens
            if current is None:
                raise NotAuthenticatedError()

            try:
                refreshed = self._refresh_call(current)
            except TokenEndpointError as e:
                # A 400 usually means the refresh token we sent is no longer valid.
                # If disk has moved past what we sent (a rotation by a writer not
                # using the lock), retry ONCE with the fresher token before
                # surfacing "re-login".
                if e.status != 400:
                    raise
                fresher = self._store.load_tokens()
                if fresher is None or fresher.refresh_token == current.refresh_token:
                    raise
                refreshed = self._refresh_call(fresher)

            self._store.save_tokens(refreshed)
            self._tokens = refreshed

    def _refresh_call(self, current: Tokens) -> Tokens:
        """One POST to the token endpoint. The response carries a ROTATED refresh
        token which the caller MUST persist (Oura invalidates the previous one)."""
        assert self._credentials is not None
        body = urlencode(
            [
                ("grant_type", "refresh_token"),
                ("refresh_token", current.refresh_token),
                ("client_id", self._credentials.client_id),
                ("client_secret", self._credentials.client_secret),
            ]
        )
        resp = self._http.request(
            "POST",
            self._token_url,
            body=body,
            headers={"Content-Type": "application/x-www-form-urlencoded"},
        )
        if not 200 <= resp.status < 300:
            raise TokenEndpointError(
                resp.status, resp.data.decode("utf-8", errors="replace")
            )
        # A hostile/broken 2xx body must surface as the typed TokenEndpointError, never
        # a raw JSONDecodeError/KeyError/ValueError/RecursionError detonating downstream
        # (mirrors the Rust crate, which maps EVERY decode failure to
        # `AuthError::InvalidTokenResponse` with a static message). The error body is a
        # FIXED, secret-free description — the raw response is NOT echoed, since a
        # partial 2xx body may carry token material.
        #
        # Strict UTF-8 first (conformance `body_invalid_utf8_*`; RFC 8259 §8.1): a body
        # that isn't valid UTF-8 ANYWHERE — unknown fields included — isn't JSON text at
        # all. Decoding explicitly (never `json.loads(bytes)`, which sniffs UTF-16/32)
        # and strictly (never errors="replace", which would persist U+FFFD) makes the
        # whole response malformed, even though the server already rotated.
        try:
            text = resp.data.decode("utf-8")
        except UnicodeDecodeError as e:
            raise TokenEndpointError(
                resp.status, "token-endpoint response was not valid UTF-8"
            ) from e
        # RecursionError too (conformance `body_deeply_nested`, an
        # implementation_defined_token_responses case): json.loads recurses per nesting
        # level, so a body nested past the interpreter's recursion limit — even inside
        # an unknown field — raises RecursionError (a RuntimeError, NOT a ValueError).
        # Rejecting it is allowed; letting it escape untyped is not.
        try:
            payload = json.loads(text)
        except ValueError as e:
            raise TokenEndpointError(
                resp.status, "token-endpoint response was not valid JSON"
            ) from e
        except RecursionError as e:
            raise TokenEndpointError(
                resp.status, "token-endpoint response was nested too deeply"
            ) from e
        if not isinstance(payload, dict):
            raise TokenEndpointError(
                resp.status, "token-endpoint response was not a JSON object"
            )
        try:
            access_token = payload["access_token"]
        except KeyError as e:
            raise TokenEndpointError(
                resp.status, "token-endpoint response missing 'access_token'"
            ) from e
        if not isinstance(access_token, str):
            raise TokenEndpointError(
                resp.status, "token-endpoint response 'access_token' was not a string"
            )
        # Hostile-but-2xx guard family (#58, mirrors the Rust crate guard in
        # oauth.rs post_token): a 200 whose payload would install a blank or
        # already-expired Bearer must fail typed BEFORE persisting — persisting would
        # also burn the still-valid rotated refresh token.
        if not access_token:
            raise TokenEndpointError(
                resp.status, "token-endpoint response 'access_token' was empty"
            )
        if "expires_in" not in payload:
            raise TokenEndpointError(
                resp.status, "token-endpoint response missing 'expires_in'"
            )
        expires_in = payload["expires_in"]
        # expires_in must be a JSON INTEGER (conformance `expires_in_fractional` /
        # `_numeric_string` / `_overflows_double`): never coerce with int(), which
        # would truncate 3600.5, parse "3600", and raise an untyped OverflowError on
        # the `inf` that json.loads yields for 1e400. `bool` is an int subclass in
        # Python but `true` is not a JSON integer, so it is excluded explicitly.
        if type(expires_in) is not int:
            raise TokenEndpointError(
                resp.status, "token-endpoint response 'expires_in' was not an integer"
            )
        # Second half of the hostile-2xx guard family (#58): a zero/negative lifetime
        # is an already-expired Bearer, and one beyond MAX_EXPIRES_IN_SECS
        # (`expires_in_above_cap` / `_i64_max`) would persist an expires_at other
        # companions can't read exactly — reject both, keep the store untouched.
        if not 1 <= expires_in <= MAX_EXPIRES_IN_SECS:
            raise TokenEndpointError(
                resp.status, "token-endpoint response 'expires_in' was out of range"
            )
        rotated = payload.get("refresh_token")
        returned_token_type = payload.get("token_type")
        # Type guard (conformance `wrong_type_refresh_token` / `wrong_type_token_type`):
        # both fields are persisted verbatim, so a non-string (42, {"a":1}, …) would
        # be written into tokens.json and corrupt the store — fail typed instead.
        # Omitted/null/empty stays lenient (keeps the stored value, below), like
        # Rust's Option<String>. Unlike `scope`, these are not informational.
        for field, value in (
            ("refresh_token", rotated),
            ("token_type", returned_token_type),
        ):
            if value is not None and not isinstance(value, str):
                raise TokenEndpointError(
                    resp.status,
                    f"token-endpoint response '{field}' was not a string",
                )
        # Malformed-Unicode guard (conformance `*_lone_surrogate` cases): json.loads
        # accepts a lone-surrogate escape like "\ud800", yielding a str that is NOT
        # valid Unicode — the store writer would then raise an untyped
        # UnicodeEncodeError AFTER the server already rotated the refresh token. Any
        # of the four fields we read must encode as UTF-8, or the whole RESPONSE is
        # malformed: fail typed here, before anything is written. (Non-string scopes
        # stay lenient below; this only rejects strings that aren't valid Unicode.
        # Unknown fields are never read, so they are deliberately NOT validated —
        # conformance `unknown_field_lone_surrogate` must succeed.)
        for field in ("access_token", "refresh_token", "scope", "token_type"):
            value = payload.get(field)
            if isinstance(value, str) and not _is_valid_unicode(value):
                raise TokenEndpointError(
                    resp.status,
                    f"token-endpoint response '{field}' was not valid Unicode",
                )
        # Scope (conformance `refresh_success_cases`): an omitted, null, non-string,
        # empty, or whitespace-only scope keeps the prior grant (RFC 6749 §5.1 lets
        # the server omit an unchanged scope; persisting a blank would erase the grant
        # the CLI's re-consent check reads). Only a real scope string replaces it.
        returned_scope = payload.get("scope")
        scope = (
            returned_scope
            if isinstance(returned_scope, str) and returned_scope.strip()
            else current.scope
        )
        return Tokens(
            access_token=access_token,
            # Persist the rotated token. An omitted, null or EMPTY refresh_token means
            # the server didn't rotate it (conformance `refresh_token_empty`): keep the
            # current one — persisting "" would make the next refresh 400.
            refresh_token=rotated if rotated else current.refresh_token,
            expires_at=int(time.time()) + expires_in,
            scope=scope,
            # Same fallback for an omitted/null/empty token_type (`token_type_empty`).
            token_type=(
                returned_token_type if returned_token_type else current.token_type
            ),
        )
