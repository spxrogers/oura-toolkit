"""OAuth metadata stays pinned to the vendored spec (never-hardcode rule): the
constants are transcriptions, and this sync test fails on any drift from
``spec/openapi.json`` — the Python analogue of the Rust crate's build-time spec read
(walk-up in the mold of cli/oura-toolkit-cli/tests/docs_tripwire.rs)."""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from oura_toolkit.auth import (
    ALL_SCOPES,
    AUTHORIZE_URL,
    TOKEN_URL,
    AuthError,
    authorize_url,
    default_scopes,
)
from oura_toolkit.auth.metadata import _validated_scopes


def repo_root() -> Path:
    """The nearest ancestor holding both the justfile and the README (same walk-up as
    the Rust docs tripwire). These tests run via `just sdk-test-py`, always in-repo."""
    d = Path(__file__).resolve().parent
    while not ((d / "justfile").is_file() and (d / "README.md").is_file()):
        assert d.parent != d, "repo root (justfile + README.md) not found above tests/"
        d = d.parent
    return d


def spec_oauth_flow() -> dict:
    spec = json.loads((repo_root() / "spec" / "openapi.json").read_text())
    return spec["components"]["securitySchemes"]["OAuth2"]["flows"][
        "authorizationCode"
    ]


class TestSpecSync:
    def test_urls_match_the_spec(self) -> None:
        flow = spec_oauth_flow()
        assert AUTHORIZE_URL == flow["authorizationUrl"], (
            "metadata.AUTHORIZE_URL drifted from spec/openapi.json — a spec refresh "
            "changed the authorize URL; update oura_toolkit/auth/metadata.py"
        )
        assert TOKEN_URL == flow["tokenUrl"], (
            "metadata.TOKEN_URL drifted from spec/openapi.json — a spec refresh "
            "changed the token URL; update oura_toolkit/auth/metadata.py"
        )

    def test_all_scopes_match_the_spec_exactly(self) -> None:
        spec_scopes = set(spec_oauth_flow()["scopes"])
        assert set(ALL_SCOPES) == spec_scopes, (
            "metadata.ALL_SCOPES drifted from the spec's OAuth2 scopes; update "
            "oura_toolkit/auth/metadata.py"
        )
        assert len(ALL_SCOPES) == len(set(ALL_SCOPES)), "duplicate scope in ALL_SCOPES"


class TestDefaultScopes:
    def test_default_scopes_are_all_scopes_minus_email(self) -> None:
        scopes = default_scopes()
        assert "email" not in scopes
        assert set(scopes) == set(ALL_SCOPES) - {"email"}
        assert len(scopes) == 7

    def test_unknown_default_scope_fails_loud_naming_the_scope(self) -> None:
        with pytest.raises(AuthError, match="bogusScope"):
            _validated_scopes(("daily", "bogusScope"))


class TestAuthorizeUrl:
    def test_authorize_url_encodes_params(self) -> None:
        url = authorize_url(
            "cid", "http://localhost:8788/callback", ["daily", "tag"], "xyz"
        )
        assert url.startswith(AUTHORIZE_URL + "?")
        assert "response_type=code" in url
        assert "client_id=cid" in url
        assert "redirect_uri=http%3A%2F%2Flocalhost%3A8788%2Fcallback" in url
        assert "scope=daily+tag" in url
        assert "state=xyz" in url
