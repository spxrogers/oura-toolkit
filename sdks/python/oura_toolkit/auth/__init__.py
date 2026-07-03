"""oura_toolkit.auth — hand-written OAuth2 auth companion for the Oura Ring API v2.

The non-interactive half of auth (the same shape as the Rust ``oura-toolkit-auth`` crate):

- a token store at the fixed, invocation-independent per-platform path
  (``$XDG_CONFIG_HOME/oura-toolkit`` -> ``~/.config/oura-toolkit`` on Unix/macOS,
  ``%LOCALAPPDATA%\\oura-toolkit`` on Windows) shared with the ``oura`` CLI,
- refresh with rotation (Oura invalidates the previous refresh token on every refresh),
  cross-process safe: every refresh runs under the store's exclusive advisory ``.lock``
  and re-reads the store first, so a rotation another process (e.g. the CLI or its MCP
  server) already performed is adopted instead of re-burned,
- OAuth metadata (authorize/token URLs, scopes) pinned to the vendored OpenAPI spec,
- a seam into the generated ``oura_toolkit.api`` client.

Interactive OAuth (browser + loopback listener) deliberately lives in the CLI, never
here — bring your own tokens (run ``oura auth setup`` / ``oura auth login`` once, or
persist tokens you obtained yourself with :meth:`TokenStore.save_tokens`).

Example — a client that keeps its Bearer token fresh across calls::

    from oura_toolkit.api import ApiClient
    from oura_toolkit.auth import TokenManager

    manager = TokenManager.load()  # reads ~/.config/oura-toolkit/{credentials,tokens}.json
    with ApiClient(manager.configuration()) as client:
        ...  # every request carries a proactively refreshed access token
"""

from .errors import (
    AuthError,
    MissingClientCredentialsError,
    NoConfigDirError,
    NotAuthenticatedError,
    StoreFormatError,
    TokenEndpointError,
)
from .manager import DEFAULT_SKEW_SECS, TOKEN_ENDPOINT_TIMEOUT, TokenManager
from .metadata import (
    ALL_SCOPES,
    AUTHORIZE_URL,
    TOKEN_URL,
    authorize_url,
    default_scopes,
)
from .store import APP_DIR_NAME, ClientCredentials, TokenStore, Tokens, config_dir

__all__ = [
    "ALL_SCOPES",
    "APP_DIR_NAME",
    "AUTHORIZE_URL",
    "AuthError",
    "ClientCredentials",
    "DEFAULT_SKEW_SECS",
    "MissingClientCredentialsError",
    "NoConfigDirError",
    "NotAuthenticatedError",
    "StoreFormatError",
    "TOKEN_ENDPOINT_TIMEOUT",
    "TOKEN_URL",
    "TokenEndpointError",
    "TokenManager",
    "TokenStore",
    "Tokens",
    "authorize_url",
    "config_dir",
    "default_scopes",
]
