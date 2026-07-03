"""Token-store guarantees: the Rust-compatible on-disk schema, the locked config-dir
resolution rules, file hygiene, and expiry skew.

The env-lookup tests never touch the real environment — `config_dir` takes an injected
lookup, mirroring the Rust crate's `config_dir_from` pattern.
"""

from __future__ import annotations

import json
import os
import stat
from pathlib import Path

import pytest

from oura_toolkit.auth import (
    APP_DIR_NAME,
    ClientCredentials,
    NoConfigDirError,
    StoreFormatError,
    TokenStore,
    Tokens,
)
from oura_toolkit.auth.store import _config_dir_from

# Transcribed from oura-toolkit-auth/src/store.rs's sample_credentials()/sample_tokens().
SAMPLE_CREDENTIALS = ClientCredentials(client_id="cid", client_secret="SECRET-CS-789")
SAMPLE_TOKENS = Tokens(
    access_token="SECRET-AT-123",
    refresh_token="SECRET-RT-456",
    expires_at=4_102_444_800,  # 2100-01-01
    scope="daily personal",
    token_type="Bearer",
)

# What the Rust crate's serde serialization of those samples puts on disk (field names,
# nesting, and value types transcribed from store.rs). The stores MUST interoperate:
# the CLI writes these records; this companion reads them, and vice versa.
RUST_CREDENTIALS_JSON = """{
  "client_id": "cid",
  "client_secret": "SECRET-CS-789"
}"""
RUST_TOKENS_JSON = """{
  "access_token": "SECRET-AT-123",
  "refresh_token": "SECRET-RT-456",
  "expires_at": 4102444800,
  "scope": "daily personal",
  "token_type": "Bearer"
}"""


class TestStoreSchema:
    """The on-disk shape is pinned to the Rust serde output (fixture test)."""

    def test_saved_records_match_the_rust_serde_shape(self, tmp_path: Path) -> None:
        store = TokenStore(tmp_path)
        store.save_credentials(SAMPLE_CREDENTIALS)
        store.save_tokens(SAMPLE_TOKENS)

        assert json.loads(store.credentials_path.read_text()) == json.loads(
            RUST_CREDENTIALS_JSON
        )
        assert json.loads(store.tokens_path.read_text()) == json.loads(
            RUST_TOKENS_JSON
        )
        # expires_at must be a JSON integer (Rust i64), not a float or string.
        assert isinstance(
            json.loads(store.tokens_path.read_text())["expires_at"], int
        )

    def test_rust_written_records_load_into_equal_objects(
        self, tmp_path: Path
    ) -> None:
        store = TokenStore(tmp_path)
        os.makedirs(tmp_path, exist_ok=True)
        store.credentials_path.write_text(RUST_CREDENTIALS_JSON)
        store.tokens_path.write_text(RUST_TOKENS_JSON)

        assert store.load_credentials() == SAMPLE_CREDENTIALS
        assert store.load_tokens() == SAMPLE_TOKENS

    def test_none_optionals_are_omitted_like_serde_skip_serializing_if(
        self, tmp_path: Path
    ) -> None:
        store = TokenStore(tmp_path)
        store.save_tokens(
            Tokens(access_token="a", refresh_token="r", expires_at=1)
        )
        on_disk = json.loads(store.tokens_path.read_text())
        assert set(on_disk) == {"access_token", "refresh_token", "expires_at"}, (
            "scope/token_type must be OMITTED when None (Rust: "
            "skip_serializing_if = Option::is_none), not serialized as null"
        )

    def test_missing_records_load_as_none(self, tmp_path: Path) -> None:
        store = TokenStore(tmp_path / "never-created")
        assert store.load_credentials() is None
        assert store.load_tokens() is None

    def test_corrupt_record_raises_a_typed_error(self, tmp_path: Path) -> None:
        store = TokenStore(tmp_path)
        store.tokens_path.write_text("{not json")
        with pytest.raises(StoreFormatError):
            store.load_tokens()
        store.tokens_path.write_text('{"access_token": "a"}')  # missing fields
        with pytest.raises(StoreFormatError):
            store.load_tokens()


class TestStoreFieldValidation:
    """Field TYPES are validated at LOAD, not just presence (ISSUE B): a wrong-typed
    field is a typed StoreFormatError at load time (Rust: serde rejects it), never a
    TypeError detonating later (e.g. `is_expired` on a string ``expires_at``)."""

    def test_expires_at_as_string_is_a_typed_format_error(
        self, tmp_path: Path
    ) -> None:
        store = TokenStore(tmp_path)
        os.makedirs(tmp_path, exist_ok=True)
        store.tokens_path.write_text(
            json.dumps(
                {"access_token": "a", "refresh_token": "r", "expires_at": "notint"}
            )
        )
        with pytest.raises(StoreFormatError):
            store.load_tokens()

    def test_expires_at_as_bool_is_a_typed_format_error(self, tmp_path: Path) -> None:
        # bool is a subclass of int in Python, but a JSON boolean is not a valid i64
        # (Rust rejects it); guard the subclass hole explicitly.
        store = TokenStore(tmp_path)
        os.makedirs(tmp_path, exist_ok=True)
        store.tokens_path.write_text(
            json.dumps({"access_token": "a", "refresh_token": "r", "expires_at": True})
        )
        with pytest.raises(StoreFormatError):
            store.load_tokens()

    def test_access_token_as_number_is_a_typed_format_error(
        self, tmp_path: Path
    ) -> None:
        store = TokenStore(tmp_path)
        os.makedirs(tmp_path, exist_ok=True)
        store.tokens_path.write_text(
            json.dumps({"access_token": 123, "refresh_token": "r", "expires_at": 1})
        )
        with pytest.raises(StoreFormatError):
            store.load_tokens()

    def test_literal_null_tokens_file_is_a_typed_format_error(
        self, tmp_path: Path
    ) -> None:
        # A file whose whole content is the JSON literal `null` is NOT "absent" (which
        # loads as None); it is a corrupt record. Rust's serde rejects null for the
        # Tokens struct — before this fix json.loads("null") -> None read as absent.
        store = TokenStore(tmp_path)
        os.makedirs(tmp_path, exist_ok=True)
        store.tokens_path.write_text("null")
        with pytest.raises(StoreFormatError):
            store.load_tokens()

    def test_genuinely_absent_tokens_file_still_loads_as_none(
        self, tmp_path: Path
    ) -> None:
        # The flip side of the null fix: a file that does not exist is still None, not
        # an error (this is "not logged in yet").
        store = TokenStore(tmp_path / "never-created")
        assert store.load_tokens() is None

    def test_store_path_that_is_a_directory_surfaces_a_typed_error(
        self, tmp_path: Path
    ) -> None:
        # A non-FileNotFound OSError (here: tokens.json is a DIRECTORY) must be a typed
        # StoreFormatError, never a raw OSError escaping to the caller.
        store = TokenStore(tmp_path)
        os.makedirs(store.tokens_path)  # tokens.json is a directory
        with pytest.raises(StoreFormatError):
            store.load_tokens()


@pytest.mark.skipif(os.name == "nt", reason="0600/0700 modes are Unix hygiene")
class TestUnixFileHygiene:
    def test_records_are_0600_and_dir_is_0700(self, tmp_path: Path) -> None:
        store = TokenStore(tmp_path / "store")
        store.save_credentials(SAMPLE_CREDENTIALS)
        store.save_tokens(SAMPLE_TOKENS)
        for path in (store.credentials_path, store.tokens_path):
            mode = stat.S_IMODE(os.stat(path).st_mode)
            assert mode == 0o600, f"{path} must be 0600, is {oct(mode)}"
        dir_mode = stat.S_IMODE(os.stat(store.dir).st_mode)
        assert dir_mode == 0o700, f"store dir must be 0700, is {oct(dir_mode)}"

    def test_lock_file_is_owner_only(self, tmp_path: Path) -> None:
        store = TokenStore(tmp_path / "store")
        with store.lock_exclusive():
            mode = stat.S_IMODE(os.stat(store.dir / ".lock").st_mode)
            assert mode == 0o600, f".lock must be 0600, is {oct(mode)}"

    def test_writes_are_atomic_no_partial_record_ever_visible(
        self, tmp_path: Path
    ) -> None:
        # save_tokens over an existing record must go through a temp file + rename:
        # the target path must never hold a truncated intermediate. We can't freeze
        # time mid-write, but we CAN assert the mechanism: no write opens the final
        # path directly (the record's inode changes on every save).
        store = TokenStore(tmp_path)
        store.save_tokens(SAMPLE_TOKENS)
        first_inode = os.stat(store.tokens_path).st_ino
        store.save_tokens(SAMPLE_TOKENS)
        assert os.stat(store.tokens_path).st_ino != first_inode, (
            "save_tokens must replace the record atomically (new inode via "
            "os.replace), never rewrite the live file in place"
        )


class TestConfigDirResolution:
    """The locked store-path rules, both platform branches unit-tested on any OS
    (the `windows` flag selects the branch; absoluteness uses that platform's rules)."""

    @staticmethod
    def env(pairs):
        return lambda key: dict(pairs).get(key)

    def test_prefers_xdg_config_home(self) -> None:
        d = _config_dir_from(
            self.env({"XDG_CONFIG_HOME": "/xdg", "HOME": "/home/u"}), windows=False
        )
        assert d == Path("/xdg") / APP_DIR_NAME

    def test_falls_back_to_home_dot_config(self) -> None:
        d = _config_dir_from(self.env({"HOME": "/home/u"}), windows=False)
        assert d == Path("/home/u/.config") / APP_DIR_NAME

    @pytest.mark.parametrize("bad", ["", "relative/config"])
    def test_empty_or_relative_xdg_falls_back_to_home(self, bad: str) -> None:
        d = _config_dir_from(
            self.env({"XDG_CONFIG_HOME": bad, "HOME": "/home/u"}), windows=False
        )
        assert d == Path("/home/u/.config") / APP_DIR_NAME, (
            f"XDG_CONFIG_HOME={bad!r} must be ignored"
        )

    @pytest.mark.parametrize("bad", ["", "relative/home"])
    def test_empty_or_relative_home_errors(self, bad: str) -> None:
        with pytest.raises(NoConfigDirError):
            _config_dir_from(self.env({"HOME": bad}), windows=False)

    def test_errors_when_neither_is_set(self) -> None:
        with pytest.raises(NoConfigDirError):
            _config_dir_from(self.env({}), windows=False)

    def test_windows_uses_local_appdata_never_roaming(self) -> None:
        d = _config_dir_from(
            self.env(
                {
                    "LOCALAPPDATA": r"C:\Users\u\AppData\Local",
                    "APPDATA": r"C:\Users\u\AppData\Roaming",
                }
            ),
            windows=True,
        )
        assert d == Path(r"C:\Users\u\AppData\Local") / APP_DIR_NAME, (
            "must use machine-local %LOCALAPPDATA%, never the roaming profile"
        )

    @pytest.mark.parametrize("bad", ["", r"relative\path"])
    def test_empty_or_relative_localappdata_errors(self, bad: str) -> None:
        with pytest.raises(NoConfigDirError):
            _config_dir_from(self.env({"LOCALAPPDATA": bad}), windows=True)

    def test_windows_error_names_the_windows_variable(self) -> None:
        with pytest.raises(NoConfigDirError, match=r"%LOCALAPPDATA%"):
            _config_dir_from(self.env({}), windows=True)


class TestExpirySkew:
    def test_expiry_uses_skew(self) -> None:
        import time

        t = Tokens(
            access_token="a",
            refresh_token="r",
            expires_at=int(time.time()) + 30,
        )
        assert not t.is_expired(0), "30s out, no skew => not expired"
        assert t.is_expired(60), "30s out, 60s skew => treated as expired"
