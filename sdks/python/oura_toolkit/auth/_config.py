"""The seam into the generated ``oura_toolkit.api`` client: a Configuration whose
``access_token`` is read from a :class:`~oura_toolkit.auth.manager.TokenManager` on
every access, so the generated client's per-request ``auth_settings()`` lookup always
sees a proactively refreshed Bearer token.

In its own module (imported lazily by ``TokenManager.configuration``) so that
``oura_toolkit.auth`` itself never imports the generated client at import time.
"""

from __future__ import annotations

from typing import TYPE_CHECKING, Optional

from oura_toolkit.api import Configuration

if TYPE_CHECKING:  # pragma: no cover
    from .manager import TokenManager


class RefreshingConfiguration(Configuration):
    """A ``Configuration`` bound to a :class:`TokenManager`.

    ``access_token`` is a read-through property: the generated client reads it while
    building each request's auth headers, and every read goes to the manager (which
    refreshes proactively and persists rotations). Assignments to ``access_token``
    (including the generated ``__init__``'s default ``None``) are ignored — the
    manager is the single source of the token.
    """

    def __init__(self, manager: "TokenManager", **kwargs: object) -> None:
        # Set before super().__init__: the parent constructor assigns
        # `self.access_token = ...`, which routes through the property setter below.
        self._manager = manager
        super().__init__(**kwargs)

    @property
    def access_token(self) -> Optional[str]:
        return self._manager.access_token()

    @access_token.setter
    def access_token(self, value: Optional[str]) -> None:
        # Deliberately a no-op: the token is always sourced from the manager.
        pass
