//! Persistent token store at a fixed, invocation-independent XDG path.
//!
//! `$XDG_CONFIG_HOME/oura-toolkit/credentials.json` (→ `~/.config/oura-toolkit/credentials.json`),
//! written `0600` via an atomic temp-file + rename. The path MUST be identical whether the
//! CLI is invoked via `npx`, `bunx`, or a brew binary — hence it derives only from the XDG
//! env, never from the invocation location.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::error::AuthError;

/// The persisted credential set. Holds the OAuth tokens **and** the confidential-client
/// credentials, because refresh requires `client_id` + `client_secret`.
///
/// `Debug` is implemented manually and REDACTS the token/secret fields, so a stray
/// `{:?}`/`dbg!` can never leak them into logs (see the "no secrets in logs" rule).
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    /// Oura rotates this on every refresh and invalidates the previous value — always persist
    /// the newly returned one or the next refresh 400s.
    pub refresh_token: String,
    /// Absolute expiry as a Unix timestamp (seconds).
    pub expires_at: i64,
    pub client_id: String,
    pub client_secret: String,
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
            .field("client_id", &self.client_id)
            .field("client_secret", &"[REDACTED]")
            .field("scope", &self.scope)
            .field("token_type", &self.token_type)
            .finish()
    }
}

/// Handle to the on-disk credential file.
#[derive(Debug, Clone)]
pub struct TokenStore {
    path: PathBuf,
}

impl TokenStore {
    /// Store at the default XDG location.
    pub fn new() -> Result<Self, AuthError> {
        Ok(Self {
            path: config_dir()?.join("credentials.json"),
        })
    }

    /// Store rooted at an explicit directory (used by tests).
    pub fn with_dir(dir: impl Into<PathBuf>) -> Self {
        Self {
            path: dir.into().join("credentials.json"),
        }
    }

    /// The resolved credential file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Load credentials, or `None` if the file does not exist yet.
    pub fn load(&self) -> Result<Option<Tokens>, AuthError> {
        match fs::read(&self.path) {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Persist credentials atomically with `0600` permissions, creating the parent dir (`0700`)
    /// if needed.
    pub fn save(&self, tokens: &Tokens) -> Result<(), AuthError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
            set_dir_private(parent)?;
        }
        let data = serde_json::to_vec_pretty(tokens)?;
        write_secure(&self.path, &data)?;
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

/// Atomic write with owner-only perms: write a temp file, fsync, rename into place.
#[cfg(unix)]
fn write_secure(path: &Path, data: &[u8]) -> std::io::Result<()> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    let tmp = path.with_extension("tmp");
    {
        let mut f = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    fs::set_permissions(path, fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn write_secure(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(data)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)?;
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

    fn sample() -> Tokens {
        Tokens {
            access_token: "access".into(),
            refresh_token: "refresh".into(),
            expires_at: 4_102_444_800, // 2100-01-01
            client_id: "cid".into(),
            client_secret: "secret".into(),
            scope: Some("daily personal".into()),
            token_type: Some("Bearer".into()),
        }
    }

    #[test]
    fn roundtrips_and_is_owner_only() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        assert!(store.load().unwrap().is_none());

        let tokens = sample();
        store.save(&tokens).unwrap();
        assert_eq!(store.load().unwrap().unwrap(), tokens);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = fs::metadata(store.path()).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600, "credential file must be 0600");
        }
    }

    #[test]
    fn debug_redacts_secrets() {
        let mut t = sample();
        t.access_token = "SECRET-AT-123".into();
        t.refresh_token = "SECRET-RT-456".into();
        t.client_secret = "SECRET-CS-789".into();
        let rendered = format!("{t:?}");
        assert!(
            !rendered.contains("SECRET-AT-123"),
            "access token leaked: {rendered}"
        );
        assert!(
            !rendered.contains("SECRET-RT-456"),
            "refresh token leaked: {rendered}"
        );
        assert!(
            !rendered.contains("SECRET-CS-789"),
            "client secret leaked: {rendered}"
        );
        assert!(rendered.contains("[REDACTED]"));
        assert!(
            rendered.contains("cid"),
            "non-secret client_id should remain visible"
        );
    }

    #[test]
    fn expiry_uses_skew() {
        let mut t = sample();
        t.expires_at = OffsetDateTime::now_utc().unix_timestamp() + 30;
        assert!(!t.is_expired(0), "30s out, no skew => not expired");
        assert!(t.is_expired(60), "30s out, 60s skew => treated as expired");
    }
}
