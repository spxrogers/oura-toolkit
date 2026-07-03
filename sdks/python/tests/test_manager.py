"""TokenManager guarantees: rotation persistence, reload-adopt, the 400-reload-retry
arm, proactive skew, and the confidential-client request shape — all against a hermetic
loopback mock token endpoint (see conftest.py)."""

from __future__ import annotations

import time
from pathlib import Path

import pytest

from oura_toolkit.auth import (
    ClientCredentials,
    MissingClientCredentialsError,
    NotAuthenticatedError,
    TokenEndpointError,
    TokenManager,
    TokenStore,
    Tokens,
)

CREDENTIALS = ClientCredentials(client_id="cid", client_secret="secret")


def expired_tokens(refresh_token: str) -> Tokens:
    return Tokens(
        access_token=f"stale-access-{refresh_token}",
        refresh_token=refresh_token,
        expires_at=0,
    )


def rotate_to(access: str, refresh: str, form_expects: str):
    """Handler granting exactly one rotation for `form_expects`; anything else 500s
    with a message naming what arrived (a burned/replayed token fails the test)."""

    def handler(form):
        if form.get("refresh_token") != form_expects:
            return (500, {"error": f"unexpected refresh_token {form!r}"})
        return (
            200,
            {"access_token": access, "refresh_token": refresh, "expires_in": 3600},
        )

    return handler


def manager_for(endpoint, store: TokenStore, tokens) -> TokenManager:
    return TokenManager(store, CREDENTIALS, tokens, token_url=endpoint.url)


class TestPreconditions:
    def test_access_token_requires_authentication(self, tmp_path: Path) -> None:
        manager = TokenManager(TokenStore(tmp_path), CREDENTIALS, None)
        assert not manager.is_authenticated()
        with pytest.raises(NotAuthenticatedError):
            manager.access_token()

    def test_refresh_without_credentials_reports_missing_credentials(
        self, tmp_path: Path
    ) -> None:
        manager = TokenManager(TokenStore(tmp_path), None, expired_tokens("r1"))
        with pytest.raises(MissingClientCredentialsError):
            manager.access_token()


class TestRefresh:
    def test_refresh_rotates_persists_and_sends_client_credentials(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        token_endpoint.handler = rotate_to("fresh-access", "r2", form_expects="r1")
        store = TokenStore(tmp_path)
        store.save_tokens(expired_tokens("r1"))
        manager = manager_for(token_endpoint, store, expired_tokens("r1"))

        assert manager.access_token() == "fresh-access"

        # The ROTATED refresh token is persisted (Oura invalidated r1: losing r2
        # here would 400 the next refresh).
        on_disk = store.load_tokens()
        assert on_disk is not None and on_disk.refresh_token == "r2"
        assert on_disk.access_token == "fresh-access"
        assert on_disk.expires_at > int(time.time()) + 3000  # expires_in honored

        # Confidential-client contract: grant_type + BOTH credentials on the wire.
        (request,) = token_endpoint.requests
        assert request["grant_type"] == "refresh_token"
        assert request["client_id"] == "cid"
        assert request["client_secret"] == "secret"
        assert request["refresh_token"] == "r1"

    def test_refresh_keeps_old_refresh_token_if_server_omits_rotation(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        token_endpoint.handler = lambda form: (
            200,
            {"access_token": "fresh-access", "expires_in": 3600},
        )
        store = TokenStore(tmp_path)
        manager = manager_for(
            token_endpoint,
            store,
            Tokens(
                access_token="stale",
                refresh_token="keep-me",
                expires_at=0,
                scope="daily",
                token_type="Bearer",
            ),
        )
        assert manager.access_token() == "fresh-access"
        on_disk = store.load_tokens()
        assert on_disk is not None
        assert on_disk.refresh_token == "keep-me"
        assert on_disk.scope == "daily"  # carried over like the Rust `or_else`
        assert on_disk.token_type == "Bearer"

    def test_refresh_treats_explicit_null_scope_and_token_type_as_omitted(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        # A present-but-null scope/token_type is NOT a new value: fall back to the
        # current ones (Rust: `resp.scope.or_else(|| current.scope)`). The old
        # `.get("scope", current.scope)` handed back None for a present null key.
        token_endpoint.handler = lambda form: (
            200,
            {
                "access_token": "fresh-access",
                "refresh_token": "r2",
                "expires_in": 3600,
                "scope": None,
                "token_type": None,
            },
        )
        store = TokenStore(tmp_path)
        manager = manager_for(
            token_endpoint,
            store,
            Tokens(
                access_token="stale",
                refresh_token="r1",
                expires_at=0,
                scope="daily personal",
                token_type="Bearer",
            ),
        )
        assert manager.access_token() == "fresh-access"
        on_disk = store.load_tokens()
        assert on_disk is not None
        assert on_disk.scope == "daily personal", (
            "explicit null scope must fall back to the current scope, not become None"
        )
        assert on_disk.token_type == "Bearer", (
            "explicit null token_type must fall back to the current one"
        )

    def test_proactive_refresh_uses_the_skew_window(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        token_endpoint.handler = rotate_to("fresh-access", "r2", form_expects="r1")
        soon = Tokens(
            access_token="still-valid",
            refresh_token="r1",
            expires_at=int(time.time()) + 30,  # inside the default 60s skew
        )
        store = TokenStore(tmp_path)

        # skew 0: 30s of validity left => NO endpoint call, current token returned.
        no_skew = TokenManager(
            store, CREDENTIALS, soon, token_url=token_endpoint.url, skew_secs=0
        )
        assert no_skew.access_token() == "still-valid"
        assert token_endpoint.requests == [], "no refresh may happen outside the skew"

        # default skew 60: the same token is treated as expiring => refresh.
        skewed = manager_for(token_endpoint, store, soon)
        assert skewed.access_token() == "fresh-access"
        assert len(token_endpoint.requests) == 1


class TestHostileTokenEndpointBodies:
    """A 2xx token-endpoint response whose body is broken or hostile must surface as the
    typed TokenEndpointError — never a raw JSONDecodeError/KeyError/ValueError detonating
    downstream (Rust parity: `resp.json::<TokenResponse>()?` -> AuthError::Serde) — and
    the surfaced message must NEVER echo token material (CLAUDE.md rule 5; ISSUE A)."""

    @pytest.mark.parametrize(
        "body",
        [
            "this is not json at all",  # non-JSON body
            "",  # empty body
            {"expires_in": 3600},  # missing access_token
            {"refresh_token": "r2", "expires_in": 3600},  # still missing access_token
            {
                "access_token": "SECRET-AT-123",
                "expires_in": "not-a-number",
            },  # non-numeric expires_in (body carries a secret access token)
            {"access_token": "SECRET-AT-123"},  # missing expires_in
        ],
        ids=[
            "non-json",
            "empty",
            "missing-access-token",
            "missing-access-token-2",
            "non-numeric-expires-in",
            "missing-expires-in",
        ],
    )
    def test_malformed_2xx_body_raises_typed_error_without_leaking_secrets(
        self, token_endpoint, tmp_path: Path, body: object
    ) -> None:
        token_endpoint.handler = lambda form: (200, body)
        store = TokenStore(tmp_path)
        manager = manager_for(token_endpoint, store, expired_tokens("r1"))

        with pytest.raises(TokenEndpointError) as excinfo:
            manager.access_token()

        message = str(excinfo.value)
        for secret in ("SECRET-AT-123", "stale-access-r1", "secret"):
            assert secret not in message, (
                f"the malformed-body error leaked {secret!r}: {message}"
            )
        # The broken response must not have been persisted as a usable token.
        assert store.load_tokens() is None, (
            "a malformed token-endpoint body must never be written to the store"
        )


class TestCrossProcessProtocol:
    def test_second_manager_adopts_rotation_from_disk_without_calling_endpoint(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        # Exactly ONE refresh is allowed, and only with r1: a second call — replaying
        # the invalidated r1 or burning the rotated r2 — trips the handler's 500 arm.
        token_endpoint.handler = rotate_to("fresh-access", "r2", form_expects="r1")
        store = TokenStore(tmp_path)
        store.save_tokens(expired_tokens("r1"))

        # Both managers start from the same stale state (the pre-#22 failure mode:
        # B's refresh would replay the invalidated r1 and 400).
        a = manager_for(token_endpoint, store, expired_tokens("r1"))
        b = manager_for(token_endpoint, store, expired_tokens("r1"))

        assert a.access_token() == "fresh-access"  # burns r1, persists r2
        assert b.access_token() == "fresh-access"  # adopts disk state, NO call
        assert len(token_endpoint.requests) == 1, (
            "the second manager must adopt the persisted rotation instead of "
            "calling the endpoint"
        )
        assert store.load_tokens().refresh_token == "r2"

    def test_force_refresh_adopts_fresher_disk_state(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        token_endpoint.handler = rotate_to("fresh-access", "r2", form_expects="r1")
        store = TokenStore(tmp_path)
        store.save_tokens(expired_tokens("r1"))

        a = manager_for(token_endpoint, store, expired_tokens("r1"))
        b = manager_for(token_endpoint, store, expired_tokens("r1"))

        assert a.access_token() == "fresh-access"
        # B's request 401'd (stale memory) and it force-refreshes: it must adopt the
        # disk rotation rather than burn r2 with another endpoint call.
        b.force_refresh()
        assert b.access_token() == "fresh-access"
        assert len(token_endpoint.requests) == 1

    def test_refresh_400_reloads_disk_and_retries_exactly_once(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        store = TokenStore(tmp_path)
        store.save_tokens(expired_tokens("r1"))

        def handler(form):
            token = form.get("refresh_token")
            if token == "r1":
                # Simulate an uncoordinated writer rotating to r2 while our r1
                # request is in flight (expired, so the retry must truly refresh),
                # then reject the now-stale r1.
                store.save_tokens(
                    Tokens(access_token="r2-access", refresh_token="r2", expires_at=0)
                )
                return (400, "invalid_grant")
            if token == "r2":
                return (
                    200,
                    {
                        "access_token": "r3-access",
                        "refresh_token": "r3",
                        "expires_in": 3600,
                    },
                )
            return (500, {"error": f"unexpected refresh_token {form!r}"})

        token_endpoint.handler = handler
        manager = manager_for(token_endpoint, store, expired_tokens("r1"))
        assert manager.access_token() == "r3-access"
        assert store.load_tokens().refresh_token == "r3"
        assert len(token_endpoint.requests_with("r1")) == 1
        assert len(token_endpoint.requests_with("r2")) == 1

    def test_genuinely_invalid_refresh_token_surfaces_400_without_blind_retry(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        token_endpoint.handler = lambda form: (400, "invalid_grant")
        store = TokenStore(tmp_path)
        store.save_tokens(expired_tokens("r-dead"))
        manager = manager_for(token_endpoint, store, expired_tokens("r-dead"))

        with pytest.raises(TokenEndpointError) as excinfo:
            manager.access_token()
        assert excinfo.value.status == 400
        assert "invalid_grant" in excinfo.value.body
        assert len(token_endpoint.requests) == 1, (
            "the reload-retry only fires when disk moved past what we sent"
        )


class TestConfigurationSeam:
    def test_configuration_reads_a_fresh_token_through_the_manager(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        token_endpoint.handler = rotate_to("fresh-access", "r2", form_expects="r1")
        store = TokenStore(tmp_path)
        manager = manager_for(token_endpoint, store, expired_tokens("r1"))

        config = manager.configuration()
        from oura_toolkit.api import Configuration

        assert isinstance(config, Configuration)
        # The generated client reads config.access_token per request; the read must
        # route through the manager (which refreshes the expired r1 proactively).
        assert config.access_token == "fresh-access"
        auth = config.auth_settings()
        assert auth["BearerAuth"]["value"] == "Bearer fresh-access"

    def test_configuration_access_token_cannot_be_detached_from_the_manager(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        token_endpoint.handler = rotate_to("fresh-access", "r2", form_expects="r1")
        manager = manager_for(
            token_endpoint, TokenStore(tmp_path), expired_tokens("r1")
        )
        config = manager.configuration()
        config.access_token = "stale-pin"  # e.g. generated __init__ default plumbing
        assert config.access_token == "fresh-access", (
            "assignment must not detach the Configuration from the manager"
        )
