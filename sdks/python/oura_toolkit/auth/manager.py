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
        # a raw JSONDecodeError/KeyError/ValueError detonating downstream (mirrors the
        # Rust crate's `resp.json::<TokenResponse>()?` -> AuthError::Serde mapping). The
        # error body is a FIXED, secret-free description — the raw response is NOT echoed,
        # since a partial 2xx body may carry token material.
        try:
            payload = json.loads(resp.data)
        except ValueError as e:
            raise TokenEndpointError(
                resp.status, "token-endpoint response was not valid JSON"
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
        try:
            expires_in = int(payload["expires_in"])
        except KeyError as e:
            raise TokenEndpointError(
                resp.status, "token-endpoint response missing 'expires_in'"
            ) from e
        except (TypeError, ValueError) as e:
            raise TokenEndpointError(
                resp.status, "token-endpoint response 'expires_in' was not numeric"
            ) from e
        rotated = payload.get("refresh_token")
        return Tokens(
            access_token=access_token,
            # Persist the rotated token; keep the old one only if the server omits it.
            refresh_token=rotated if rotated is not None else current.refresh_token,
            expires_at=int(time.time()) + expires_in,
            # Explicit null is treated like omission (Rust: `resp.scope.or_else(...)`) —
            # `.get(k, default)` would hand back None for a present-but-null key.
            scope=payload.get("scope") or current.scope,
            token_type=payload.get("token_type") or current.token_type,
        )
