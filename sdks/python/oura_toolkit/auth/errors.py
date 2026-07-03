"""Error types for the auth companion (mirrors the Rust crate's ``AuthError``)."""

from __future__ import annotations


class AuthError(Exception):
    """Base class for every error this companion raises itself.

    Transport-level failures talking to the token endpoint (timeouts, connection
    errors) are urllib3 exceptions and propagate as-is.
    """


class NotAuthenticatedError(AuthError):
    """No tokens available. The library deliberately does not embed remediation hints —
    callers own the UX (the CLI maps this to "run ``oura auth login``")."""

    def __init__(self) -> None:
        super().__init__("not authenticated (no tokens stored)")


class NoConfigDirError(AuthError):
    """Could not resolve the config directory from the platform's environment."""


class MissingClientCredentialsError(AuthError):
    """Tokens exist but the client-credentials record is missing, so a refresh is
    impossible (confidential client: the token endpoint requires ``client_id`` +
    ``client_secret``). Callers own the remediation hint ("run ``oura auth setup``")."""

    def __init__(self) -> None:
        super().__init__("no client credentials stored")


class MissingRefreshTokenError(AuthError):
    """The token endpoint returned no ``refresh_token`` where one is required —
    persisting that state would break the next refresh, so it is rejected up front."""

    def __init__(self) -> None:
        super().__init__("token endpoint returned no refresh_token")


class TokenEndpointError(AuthError):
    """The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh
    token). ``body`` is the server's response text, verbatim."""

    def __init__(self, status: int, body: str) -> None:
        super().__init__(f"token endpoint returned HTTP {status}: {body}")
        self.status = status
        self.body = body


class StoreFormatError(AuthError):
    """A store record exists but does not parse as the expected JSON shape."""
