"""Persistent credential store at the fixed, invocation-independent per-platform path.

Same store, same records, same JSON shape as the Rust ``oura-toolkit-auth`` crate (the CLI
and its MCP server) — this module is a co-tenant, not a fork:

- ``$XDG_CONFIG_HOME/oura-toolkit/`` (-> ``~/.config/oura-toolkit/``) on Unix/macOS,
  ``%LOCALAPPDATA%\\oura-toolkit\\`` on Windows (Local, NOT Roaming: roaming profiles sync
  ``%APPDATA%`` off the machine, which would copy plaintext secrets to file servers).
- Two records: ``credentials.json`` (:class:`ClientCredentials` — the user's own OAuth app
  id + secret, BYO confidential client) and ``tokens.json`` (:class:`Tokens` —
  access/refresh/expiry/scope, rewritten on every refresh rotation). The JSON field names
  and optional-field omission match the Rust crate's serde output exactly (pinned by the
  store-schema fixture test).
- File hygiene: on Unix, records are written ``0600`` via a uniquely named temp file +
  atomic ``os.replace`` and the directory is ``0700``. On Windows the chmods are no-ops —
  protection relies on ``%LOCALAPPDATA%``'s user-private ACLs.
- Cross-process coordination: :meth:`TokenStore.lock_exclusive` takes a blocking exclusive
  advisory lock on a ``.lock`` file in the store dir. On Unix this is ``fcntl.flock``,
  which interoperates with the Rust CLI's ``std::fs::File::lock`` (also ``flock``) — the
  two implementations genuinely exclude each other. On Windows it is ``msvcrt.locking``
  (byte-range) while Rust uses ``LockFileEx``; treat mutual exclusion there as
  best-effort and rely on the reload-adopt + 400-reload-retry protocol
  (:class:`~oura_toolkit.auth.manager.TokenManager`), which is the universal guarantee.
"""

from __future__ import annotations

import json
import ntpath
import os
import posixpath
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Optional, Union

from .errors import NoConfigDirError, StoreFormatError

#: The locked config-directory name (CLAUDE.md -> NAMING), identical under every install
#: method on every platform, and identical to the Rust crate's ``APP_DIR_NAME``.
APP_DIR_NAME = "oura-toolkit"

_IS_WINDOWS = os.name == "nt"

EnvLookup = Callable[[str], Optional[str]]

#: Sentinel distinguishing an ABSENT store record (file not found) from a record whose
#: content is the JSON literal ``null`` — the latter is a format error (Rust parity: serde
#: rejects ``null`` for a struct), the former is simply "not logged in yet".
_MISSING = object()


def _require_str(data: dict, field: str, *, record: str) -> str:
    """Presence + type check for a required string field (Rust serde rejects both a
    missing field and a wrong-typed one at load). Names the field, never the value, so
    the message can never leak a secret."""
    try:
        value = data[field]
    except KeyError as e:
        raise StoreFormatError(f"{record} record missing field {field!r}") from e
    if not isinstance(value, str):
        raise StoreFormatError(
            f"{record} record field {field!r} must be a string, "
            f"got {type(value).__name__}"
        )
    return value


def _optional_str(data: dict, field: str, *, record: str) -> Optional[str]:
    """Type check for an optional string field: absent/``null`` is fine, a wrong type is
    a format error. Names the field, never the value."""
    value = data.get(field)
    if value is not None and not isinstance(value, str):
        raise StoreFormatError(
            f"{record} record field {field!r} must be a string or absent, "
            f"got {type(value).__name__}"
        )
    return value


def _require_int(data: dict, field: str, *, record: str) -> int:
    """Presence + type check for a required integer field. ``bool`` is a subclass of
    ``int`` but JSON booleans are NOT valid here (Rust i64 rejects them), so they are
    excluded. Names the field, never the value."""
    try:
        value = data[field]
    except KeyError as e:
        raise StoreFormatError(f"{record} record missing field {field!r}") from e
    if not isinstance(value, int) or isinstance(value, bool):
        raise StoreFormatError(
            f"{record} record field {field!r} must be an integer, "
            f"got {type(value).__name__}"
        )
    return value


@dataclass(frozen=True, repr=False)
class ClientCredentials:
    """The user's own Oura OAuth application credentials (BYO confidential client).

    ``repr()``/``str()`` REDACT the secret, so a stray log/f-string can never leak it.
    """

    client_id: str
    client_secret: str

    def __repr__(self) -> str:
        return f"ClientCredentials(client_id={self.client_id!r}, client_secret='[REDACTED]')"

    def _to_json_dict(self) -> dict:
        return {"client_id": self.client_id, "client_secret": self.client_secret}

    @classmethod
    def _from_json_dict(cls, data: object) -> "ClientCredentials":
        if not isinstance(data, dict):
            raise StoreFormatError("credentials record is not a JSON object")
        return cls(
            client_id=_require_str(data, "client_id", record="credentials"),
            client_secret=_require_str(data, "client_secret", record="credentials"),
        )


@dataclass(frozen=True, repr=False)
class Tokens:
    """The persisted OAuth token set (client credentials live in their own record).

    Oura ROTATES ``refresh_token`` on every refresh and invalidates the previous value —
    always persist the newly returned one or the next refresh 400s.

    ``expires_at`` is the absolute expiry as a Unix timestamp (seconds).
    ``repr()``/``str()`` REDACT both token fields.
    """

    access_token: str
    refresh_token: str
    expires_at: int
    scope: Optional[str] = None
    token_type: Optional[str] = None

    def is_expired(self, skew_secs: int = 0) -> bool:
        """True if the access token is expired (or within ``skew_secs`` of expiring)."""
        return int(time.time()) + skew_secs >= self.expires_at

    def __repr__(self) -> str:
        return (
            "Tokens(access_token='[REDACTED]', refresh_token='[REDACTED]', "
            f"expires_at={self.expires_at!r}, scope={self.scope!r}, "
            f"token_type={self.token_type!r})"
        )

    def _to_json_dict(self) -> dict:
        # Field order and None-omission mirror the Rust struct's serde output
        # (`skip_serializing_if = "Option::is_none"` on scope/token_type).
        data = {
            "access_token": self.access_token,
            "refresh_token": self.refresh_token,
            "expires_at": self.expires_at,
        }
        if self.scope is not None:
            data["scope"] = self.scope
        if self.token_type is not None:
            data["token_type"] = self.token_type
        return data

    @classmethod
    def _from_json_dict(cls, data: object) -> "Tokens":
        if not isinstance(data, dict):
            raise StoreFormatError("tokens record is not a JSON object")
        return cls(
            access_token=_require_str(data, "access_token", record="tokens"),
            refresh_token=_require_str(data, "refresh_token", record="tokens"),
            expires_at=_require_int(data, "expires_at", record="tokens"),
            scope=_optional_str(data, "scope", record="tokens"),
            token_type=_optional_str(data, "token_type", record="tokens"),
        )


def config_dir(env: EnvLookup = os.environ.get) -> Path:
    """The fixed, invocation-independent config dir (see the module docstring).

    ``env`` is an injectable lookup (tests pass a closure; nothing mutates the real
    environment). Empty and RELATIVE values are treated as absent (XDG spec) — a relative
    base would make where secrets land depend on the process cwd, breaking the
    invocation-independence invariant.
    """
    return _config_dir_from(env, windows=_IS_WINDOWS)


def _config_dir_from(env: EnvLookup, *, windows: bool) -> Path:
    """Testable core of :func:`config_dir`: both platform branches are driven by the
    ``windows`` flag so each is unit-tested on any OS (absoluteness is judged with the
    target platform's path rules, not the host's)."""
    isabs = ntpath.isabs if windows else posixpath.isabs

    def usable(key: str) -> Optional[str]:
        value = env(key)
        if not value or not isabs(value):
            return None
        return value

    if windows:
        base = usable("LOCALAPPDATA")
        if base is not None:
            return Path(base) / APP_DIR_NAME
        raise NoConfigDirError(
            "could not determine the config directory "
            "(%LOCALAPPDATA% unset or not an absolute path)"
        )

    xdg = usable("XDG_CONFIG_HOME")
    if xdg is not None:
        return Path(xdg) / APP_DIR_NAME
    home = usable("HOME")
    if home is not None:
        return Path(home) / ".config" / APP_DIR_NAME
    raise NoConfigDirError(
        "could not determine the config directory "
        "($XDG_CONFIG_HOME / $HOME unset or not an absolute path)"
    )


def _write_secure(path: Path, data: bytes) -> None:
    """Atomic write with owner-only perms: uniquely named temp file in the same
    directory (``mkstemp`` creates it ``0600``), fsync, ``os.replace`` into place.
    The unique name means two concurrent writers can never truncate each other's
    in-flight temp file; the atomic rename makes the outcome last-writer-wins,
    never a corrupt mix."""
    fd, tmp = tempfile.mkstemp(dir=str(path.parent), prefix=".tmp")
    try:
        with os.fdopen(fd, "wb") as f:
            f.write(data)
            f.flush()
            os.fsync(f.fileno())
        os.replace(tmp, path)
    except BaseException:
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise
    if not _IS_WINDOWS:
        os.chmod(path, 0o600)


def _open_owner_only(path: Path) -> int:
    """Open (creating if needed, never truncating) with owner-only perms where
    supported; returns a raw fd."""
    return os.open(str(path), os.O_RDWR | os.O_CREAT, 0o600)


if _IS_WINDOWS:  # pragma: no cover — exercised only on a Windows host
    import msvcrt

    def _try_lock_fd(fd: int) -> bool:
        try:
            msvcrt.locking(fd, msvcrt.LK_NBLCK, 1)
            return True
        except OSError:
            return False

    def _lock_fd_blocking(fd: int) -> None:
        # msvcrt.LK_LOCK gives up after ~10s; loop for genuinely blocking semantics.
        while not _try_lock_fd(fd):
            time.sleep(0.05)

    def _unlock_fd(fd: int) -> None:
        os.lseek(fd, 0, os.SEEK_SET)
        msvcrt.locking(fd, msvcrt.LK_UNLCK, 1)

else:
    import fcntl

    def _try_lock_fd(fd: int) -> bool:
        try:
            fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
            return True
        except OSError:
            return False

    def _lock_fd_blocking(fd: int) -> None:
        fcntl.flock(fd, fcntl.LOCK_EX)

    def _unlock_fd(fd: int) -> None:
        fcntl.flock(fd, fcntl.LOCK_UN)


class StoreLock:
    """An exclusive advisory lock on the store, released by ``release()`` / context
    exit (or process death). Bind it for the critical section."""

    def __init__(self, fd: int) -> None:
        self._fd: Optional[int] = fd

    def release(self) -> None:
        if self._fd is not None:
            fd, self._fd = self._fd, None
            try:
                _unlock_fd(fd)
            finally:
                os.close(fd)

    def __enter__(self) -> "StoreLock":
        return self

    def __exit__(self, *exc_info: object) -> None:
        self.release()


class TokenStore:
    """Handle to the on-disk store directory (see the module docstring)."""

    def __init__(self, dir: Union[str, Path, None] = None) -> None:
        """Store rooted at ``dir`` (tests pass a tempdir), or at the default
        per-platform config location when ``dir`` is omitted."""
        self._dir = Path(dir) if dir is not None else config_dir()

    @property
    def dir(self) -> Path:
        """The store directory."""
        return self._dir

    @property
    def credentials_path(self) -> Path:
        """Path of the client-credentials record."""
        return self._dir / "credentials.json"

    @property
    def tokens_path(self) -> Path:
        """Path of the token record."""
        return self._dir / "tokens.json"

    def load_credentials(self) -> Optional[ClientCredentials]:
        """The client credentials, or ``None`` if ``oura auth setup`` has never run."""
        data = self._read_record(self.credentials_path)
        return None if data is _MISSING else ClientCredentials._from_json_dict(data)

    def save_credentials(self, credentials: ClientCredentials) -> None:
        """Persist the client credentials (``0600``, atomic)."""
        self._ensure_dir()
        _write_secure(
            self.credentials_path, _to_json_bytes(credentials._to_json_dict())
        )

    def load_tokens(self) -> Optional[Tokens]:
        """The tokens, or ``None`` if no login has succeeded yet."""
        data = self._read_record(self.tokens_path)
        return None if data is _MISSING else Tokens._from_json_dict(data)

    def save_tokens(self, tokens: Tokens) -> None:
        """Persist the tokens (``0600``, atomic). Callers refreshing MUST persist the
        rotated refresh token (Oura invalidates the previous one), and MUST do so under
        :meth:`lock_exclusive` (as ``TokenManager`` does) or they can burn a rotation
        another process just persisted."""
        self._ensure_dir()
        _write_secure(self.tokens_path, _to_json_bytes(tokens._to_json_dict()))

    def lock_exclusive(self) -> StoreLock:
        """Take a BLOCKING exclusive advisory lock on the store; hold the returned
        guard across a reload -> refresh -> persist critical section.

        Cooperative: it excludes only other holders of this protocol (including the
        Rust CLI/MCP server on Unix — both sides use ``flock``). Mutual exclusion also
        rests on the ``.lock`` file's inode continuity — deleting the file while a
        process holds the lock defeats coordination."""
        self._ensure_dir()
        fd = _open_owner_only(self._dir / ".lock")
        try:
            _lock_fd_blocking(fd)
        except BaseException:
            os.close(fd)
            raise
        return StoreLock(fd)

    def try_lock_exclusive(self) -> Optional[StoreLock]:
        """Non-blocking variant of :meth:`lock_exclusive`: ``None`` if another process
        currently holds the lock."""
        self._ensure_dir()
        fd = _open_owner_only(self._dir / ".lock")
        try:
            acquired = _try_lock_fd(fd)
        except BaseException:
            os.close(fd)
            raise
        if not acquired:
            os.close(fd)
            return None
        return StoreLock(fd)

    def __repr__(self) -> str:
        return f"TokenStore(dir={str(self._dir)!r})"

    def _ensure_dir(self) -> None:
        os.makedirs(self._dir, exist_ok=True)
        if not _IS_WINDOWS:
            os.chmod(self._dir, 0o700)

    @staticmethod
    def _read_record(path: Path) -> object:
        """Read + JSON-parse a record. Returns :data:`_MISSING` when the file does not
        exist (not an error — "not logged in yet"); any OTHER read failure (e.g. the
        path is a directory) is a typed :class:`StoreFormatError`, never a raw
        ``OSError``. The parsed value (which may be the JSON literal ``null``) is
        returned as-is; the caller's ``_from_json_dict`` rejects a non-object."""
        try:
            raw = path.read_bytes()
        except FileNotFoundError:
            return _MISSING
        except OSError as e:
            raise StoreFormatError(f"cannot read store record {path.name}: {e}") from e
        try:
            return json.loads(raw)
        except ValueError as e:
            raise StoreFormatError(f"corrupt store record {path.name}: {e}") from e


def _to_json_bytes(data: dict) -> bytes:
    # Pretty-printed to match the Rust crate's `serde_json::to_vec_pretty` JSON shape;
    # ensure_ascii=False keeps non-ASCII as UTF-8 (serde_json does not \u-escape either).
    return json.dumps(data, indent=2, ensure_ascii=False).encode("utf-8")
