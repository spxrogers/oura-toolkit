"""OAuth2 metadata pinned to the vendored Oura OpenAPI spec.

Nothing here is invented: :data:`AUTHORIZE_URL`, :data:`TOKEN_URL`, and
:data:`ALL_SCOPES` are transcriptions of ``components.securitySchemes.OAuth2`` in
``spec/openapi.json``, and the hermetic sync test (``tests/test_metadata.py``) fails CI
if a spec refresh ever changes them — the same "read from the spec, never hardcode"
guarantee the Rust crate gets from its build script, delivered here as pinned constants
because an installed wheel has no repo spec to read.
"""

from __future__ import annotations

from typing import Sequence, Tuple
from urllib.parse import urlencode

from .errors import AuthError

#: ``flows.authorizationCode.authorizationUrl`` from the spec's OAuth2 security scheme.
AUTHORIZE_URL = "https://cloud.ouraring.com/oauth/authorize"

#: ``flows.authorizationCode.tokenUrl`` from the spec's OAuth2 security scheme.
TOKEN_URL = "https://api.ouraring.com/oauth/token"

#: Every scope the spec's OAuth2 security scheme advertises (all 8).
ALL_SCOPES: Tuple[str, ...] = (
    "email",
    "personal",
    "daily",
    "heartrate",
    "workout",
    "tag",
    "session",
    "spo2Daily",
)

# Scopes the toolkit requests by default: everything except `email`. This is the
# toolkit's *policy*, not spec metadata — the spec-advertised set is ALL_SCOPES.
_DEFAULT_SCOPE_NAMES: Tuple[str, ...] = (
    "personal",
    "daily",
    "heartrate",
    "workout",
    "tag",
    "session",
    "spo2Daily",
)


def default_scopes() -> Tuple[str, ...]:
    """The default scopes, each verified to exist in the spec-advertised
    :data:`ALL_SCOPES`.

    FAILS LOUD (raises :class:`AuthError` naming the missing scope) rather than
    silently narrowing the consent request if a spec refresh ever renames one.
    """
    return _validated_scopes(_DEFAULT_SCOPE_NAMES)


def _validated_scopes(names: Sequence[str]) -> Tuple[str, ...]:
    for scope in names:
        if scope not in ALL_SCOPES:
            raise AuthError(
                f"default scope {scope!r} is not advertised by the vendored spec — "
                "update the default scope names to match the spec's OAuth2 scopes"
            )
    return tuple(names)


def authorize_url(
    client_id: str, redirect_uri: str, scopes: Sequence[str], state: str
) -> str:
    """Build the authorization-code consent URL from spec metadata.

    ``scopes`` are space-joined per OAuth2; ``state`` is an opaque CSRF token the
    caller generates and later verifies on the callback. (The interactive dance that
    USES this URL lives in the CLI, never in this companion.)
    """
    query = urlencode(
        [
            ("response_type", "code"),
            ("client_id", client_id),
            ("redirect_uri", redirect_uri),
            ("scope", " ".join(scopes)),
            ("state", state),
        ]
    )
    return f"{AUTHORIZE_URL}?{query}"
