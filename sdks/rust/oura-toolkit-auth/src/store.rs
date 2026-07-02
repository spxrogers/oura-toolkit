//! Persistent credential store at a fixed, invocation-independent XDG path.
//!
//! Two records live in `$XDG_CONFIG_HOME/oura-toolkit/` (→ `~/.config/oura-toolkit/`):
//!
//! - **`credentials.json`** — [`ClientCredentials`] (the user's own OAuth app id + secret).
//!   Exists from `oura auth setup` onward, independent of any login.
//! - **`tokens.json`** — [`Tokens`] (access/refresh/expiry/scope). Exists only after a
//!   successful login and is rewritten on every refresh rotation.
//!
//! Separating the two means a failed or abandoned login never loses the pasted secret, and
//! `oura auth login` never has to dig client credentials out of a token record (issue #23).
//!
//! Everything is written `0600` via an atomic temp-file + rename. The path MUST be identical
//! whether the CLI is invoked via `npx`, `bunx`, or a brew binary — hence it derives only
//! from the XDG env, never from the invocation location.
//!
//! Cross-process coordination: [`TokenStore::lock_exclusive`] takes a blocking advisory lock
//! on a `.lock` file in the store dir. `TokenManager` holds it across its
//! reload-refresh-persist critical section so two processes (CLI + MCP server) can share the
//! store despite Oura invalidating the previous refresh token on every rotation (issue #22).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::error::AuthError;

/// The user's own Oura OAuth application credentials (BYO confidential client).
///
/// `Debug` is implemented manually and REDACTS the secret, so a stray `{:?}`/`dbg!` can
/// never leak it into logs (see the "no secrets in logs" rule).
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub client_id: String,
    pub client_secret: String,
}

impl std::fmt::Debug for ClientCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientCredentials")
            .field("client_id", &self.client_id)
            .field("client_secret", &"[REDACTED]")
            .finish()
    }
}

/// The persisted OAuth token set. Client credentials live in their own record
/// ([`ClientCredentials`]), not here.
///
/// `Debug` is implemented manually and REDACTS the token fields.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    /// Oura rotates this on every refresh and invalidates the previous value — always persist
    /// the newly returned one or the next refresh 400s.
    pub refresh_token: String,
    /// Absolute expiry as a Unix timestamp (seconds).
    pub expires_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

impl Tokens {
    /// True if the access token is expired (or within `skew_secs` of expiring).
    pub fn is_expired(&self, skew_secs: i64) -> bool {
        let now = OffsetDateTime::now_utc().unix_timestamp();
        now + skew_secs >= self.expires_at
    }
}

impl std::fmt::Debug for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tokens")
            .field("access_token", &"[REDACTED]")
            .field("refresh_token", &"[REDACTED]")
            .field("expires_at", &self.expires_at)
            .field("scope", &self.scope)
            .field("token_type", &self.token_type)
            .finish()
    }
}

/// Handle to the on-disk store directory.
#[derive(Debug, Clone)]
pub struct TokenStore {
    dir: PathBuf,
}

/// An exclusive advisory lock on the store, released on drop (or process exit).
pub struct StoreLock(fs::File);

impl Drop for StoreLock {
    fn drop(&mut self) {
        let _ = self.0.unlock();
    }
}

impl TokenStore {
    /// Store at the default XDG location.
    pub fn new() -> Result<Self, AuthError> {
        Ok(Self { dir: config_dir()? })
    }

    /// Store rooted at an explicit directory (used by tests).
    pub fn with_dir(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// The store directory.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Path of the client-credentials record.
    pub fn credentials_path(&self) -> PathBuf {
        self.dir.join("credentials.json")
    }

    /// Path of the token record.
    pub fn tokens_path(&self) -> PathBuf {
        self.dir.join("tokens.json")
    }

    /// Load the client credentials, or `None` if `auth setup` has never run.
    pub fn load_credentials(&self) -> Result<Option<ClientCredentials>, AuthError> {
        match fs::read(self.credentials_path()) {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Persist the client credentials (`0600`, atomic).
    pub fn save_credentials(&self, credentials: &ClientCredentials) -> Result<(), AuthError> {
        self.ensure_dir()?;
        let data = serde_json::to_vec_pretty(credentials)?;
        write_secure(&self.credentials_path(), &data)?;
        Ok(())
    }

    /// Load the tokens, or `None` if no login has succeeded yet.
    pub fn load_tokens(&self) -> Result<Option<Tokens>, AuthError> {
        match fs::read(self.tokens_path()) {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Persist the tokens (`0600`, atomic). Callers refreshing MUST persist the rotated
    /// refresh token (Oura invalidates the previous one).
    ///
    /// Concurrent writers cannot corrupt the record (unique temp file + atomic rename;
    /// last-writer-wins) — but **rotation coordination** is not automatic: anything that
    /// refreshes and rewrites tokens must do so under [`Self::lock_exclusive`], as
    /// `TokenManager` does, or it can burn a rotation another process just persisted.
    pub fn save_tokens(&self, tokens: &Tokens) -> Result<(), AuthError> {
        self.ensure_dir()?;
        let data = serde_json::to_vec_pretty(tokens)?;
        write_secure(&self.tokens_path(), &data)?;
        Ok(())
    }

    /// Take a **blocking** exclusive lock on the store; hold the returned guard across a
    /// reload → refresh → persist critical section (see `TokenManager`).
    ///
    /// Semantics (std `File::lock`): advisory `flock` on Unix, `LockFileEx` on Windows —
    /// either way it is **cooperative**: it excludes only other holders of this method, not
    /// arbitrary writers. Mutual exclusion also rests on the `.lock` file's inode
    /// continuity — deleting the file while a process holds the lock lets the next locker
    /// acquire a fresh inode and defeats coordination.
    ///
    /// This blocks the calling thread; from async code, acquire it on a blocking pool
    /// (`spawn_blocking`) as `TokenManager` does.
    pub fn lock_exclusive(&self) -> Result<StoreLock, AuthError> {
        self.ensure_dir()?;
        let file = open_owner_only(&self.dir.join(".lock"))?;
        file.lock()?;
        Ok(StoreLock(file))
    }

    fn ensure_dir(&self) -> Result<(), AuthError> {
        fs::create_dir_all(&self.dir)?;
        set_dir_private(&self.dir)?;
        Ok(())
    }
}

/// `$XDG_CONFIG_HOME/oura-toolkit`, falling back to `$HOME/.config/oura-toolkit`.
fn config_dir() -> Result<PathBuf, AuthError> {
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return Ok(PathBuf::from(xdg).join("oura-toolkit"));
        }
    }
    let home = std::env::var("HOME").map_err(|_| AuthError::NoConfigDir)?;
    if home.is_empty() {
        return Err(AuthError::NoConfigDir);
    }
    Ok(PathBuf::from(home).join(".config").join("oura-toolkit"))
}

/// Open (creating if needed) with owner-only perms where supported.
#[cfg(unix)]
fn open_owner_only(path: &Path) -> std::io::Result<fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn open_owner_only(path: &Path) -> std::io::Result<fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
}

/// Atomic write with owner-only perms: write a **uniquely named** temp file in the same
/// directory, fsync, rename into place. The unique name (vs a fixed `*.tmp`) means two
/// concurrent writers — say a locked refresh and an `oura auth login` — can never truncate
/// each other's in-flight temp file; the atomic rename makes the outcome last-writer-wins,
/// never a corrupt mix.
fn write_secure(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let dir = path.parent().expect("store paths always have a parent dir");
    // NamedTempFile creates with a random name and 0600 perms (on Unix).
    let mut tmp = tempfile::NamedTempFile::new_in(dir)?;
    tmp.write_all(data)?;
    tmp.as_file().sync_all()?;
    tmp.persist(path).map_err(|e| e.error)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    }
    Ok(())
}

#[cfg(unix)]
fn set_dir_private(dir: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
}

#[cfg(not(unix))]
fn set_dir_private(_dir: &Path) -> std::io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn sample_credentials() -> ClientCredentials {
        ClientCredentials {
            client_id: "cid".into(),
            client_secret: "SECRET-CS-789".into(),
        }
    }

    pub(crate) fn sample_tokens() -> Tokens {
        Tokens {
            access_token: "SECRET-AT-123".into(),
            refresh_token: "SECRET-RT-456".into(),
            expires_at: 4_102_444_800, // 2100-01-01
            scope: Some("daily personal".into()),
            token_type: Some("Bearer".into()),
        }
    }

    #[test]
    fn both_records_roundtrip_and_are_owner_only() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        assert!(store.load_credentials().unwrap().is_none());
        assert!(store.load_tokens().unwrap().is_none());

        store.save_credentials(&sample_credentials()).unwrap();
        store.save_tokens(&sample_tokens()).unwrap();
        assert_eq!(
            store.load_credentials().unwrap().unwrap(),
            sample_credentials()
        );
        assert_eq!(store.load_tokens().unwrap().unwrap(), sample_tokens());

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for path in [store.credentials_path(), store.tokens_path()] {
                let mode = fs::metadata(&path).unwrap().permissions().mode();
                assert_eq!(mode & 0o777, 0o600, "{path:?} must be 0600");
            }
        }
    }

    #[test]
    fn debug_redacts_secrets_in_both_records() {
        let creds = format!("{:?}", sample_credentials());
        assert!(
            !creds.contains("SECRET-CS-789"),
            "client secret leaked: {creds}"
        );
        assert!(creds.contains("cid"), "client_id should remain visible");

        let tokens = format!("{:?}", sample_tokens());
        assert!(
            !tokens.contains("SECRET-AT-123"),
            "access token leaked: {tokens}"
        );
        assert!(
            !tokens.contains("SECRET-RT-456"),
            "refresh token leaked: {tokens}"
        );
        assert!(tokens.contains("[REDACTED]"));
    }

    #[test]
    fn corrupt_record_errors_instead_of_panicking() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        fs::create_dir_all(dir.path()).unwrap();
        fs::write(store.tokens_path(), b"{not json").unwrap();
        assert!(
            matches!(store.load_tokens(), Err(AuthError::Serde(_))),
            "corrupt tokens.json must surface a typed parse error"
        );
    }

    #[test]
    fn expiry_uses_skew() {
        let mut t = sample_tokens();
        t.expires_at = OffsetDateTime::now_utc().unix_timestamp() + 30;
        assert!(!t.is_expired(0), "30s out, no skew => not expired");
        assert!(t.is_expired(60), "30s out, 60s skew => treated as expired");
    }

    #[test]
    fn store_lock_is_exclusive_and_released_on_drop() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());

        let guard = store.lock_exclusive().unwrap();
        // A second handle must not be able to take the lock while the first holds it.
        let contended = open_owner_only(&dir.path().join(".lock")).unwrap();
        assert!(
            matches!(contended.try_lock(), Err(std::fs::TryLockError::WouldBlock)),
            "second lock must not succeed while held"
        );
        drop(contended);
        drop(guard);
        let free = open_owner_only(&dir.path().join(".lock")).unwrap();
        assert!(free.try_lock().is_ok(), "lock must be free after drop");
    }
}
