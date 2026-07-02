//! The runtime auth layer: a [`TokenManager`] that owns the token state and refreshes it, an
//! [`AuthMiddleware`] that injects a fresh Bearer token on every request, and a builder that
//! composes auth + transient-retry into a `ClientWithMiddleware` for the generated SDK client.
//!
//! Refresh strategy: **proactive** — the manager refreshes when the access token is expired
//! or within a skew window, so requests carry a valid token. A reqwest `Middleware` cannot
//! re-run the request chain, so reactive refresh-on-401-then-retry is done by callers via
//! [`TokenManager::force_refresh`] (wired at the CLI data commands #9 and MCP tool calls #10).
//!
//! Cross-process safety (issue #22): Oura invalidates the previous refresh token on every
//! rotation, and the CLI and the long-running MCP server share one on-disk store. Every
//! refresh therefore runs under the store's exclusive advisory lock and **re-reads the store
//! first** — if another process already rotated, its fresher tokens are adopted instead of
//! burning (and thereby invalidating) that rotation with a second refresh. A refresh that
//! still 400s is retried once against freshly reloaded disk state before surfacing
//! "re-login".

use std::sync::Arc;

use http::Extensions;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::{Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use tokio::sync::Mutex;

use crate::error::AuthError;
use crate::metadata::TOKEN_URL;
use crate::oauth::refresh_at;
use crate::store::{ClientCredentials, TokenStore, Tokens};

/// Refresh this many seconds before the token's actual expiry.
const DEFAULT_SKEW_SECS: i64 = 60;

/// Owns the current tokens and the machinery to keep them fresh. Shared (behind `Arc`) by the
/// CLI's SDK calls and the MCP server's tool calls — one auth layer, two consumers.
pub struct TokenManager {
    store: TokenStore,
    credentials: Option<ClientCredentials>,
    tokens: Mutex<Option<Tokens>>,
    http: reqwest::Client,
    skew_secs: i64,
    /// The spec-derived token endpoint; overridden only by tests (mock server).
    token_url: String,
}

impl TokenManager {
    /// Load from the default token store (no error if records are absent —
    /// [`Self::access_token`] reports [`AuthError::NotAuthenticated`] on first use).
    pub fn load() -> Result<Self, AuthError> {
        let store = TokenStore::new()?;
        let credentials = store.load_credentials()?;
        let tokens = store.load_tokens()?;
        Ok(Self::from_parts(store, credentials, tokens))
    }

    /// Construct from an explicit store + optional in-memory records.
    pub fn from_parts(
        store: TokenStore,
        credentials: Option<ClientCredentials>,
        tokens: Option<Tokens>,
    ) -> Self {
        Self {
            store,
            credentials,
            tokens: Mutex::new(tokens),
            // A plain client (no auth middleware) for token-endpoint calls, to avoid recursion.
            http: reqwest::Client::new(),
            skew_secs: DEFAULT_SKEW_SECS,
            token_url: TOKEN_URL.to_string(),
        }
    }

    /// Whether any tokens are loaded (does not validate them).
    pub async fn is_authenticated(&self) -> bool {
        self.tokens.lock().await.is_some()
    }

    /// Return a valid access token, refreshing (and persisting the rotation) if needed.
    pub async fn access_token(&self) -> Result<String, AuthError> {
        let mut guard = self.tokens.lock().await;
        let expired = match guard.as_ref() {
            Some(t) => t.is_expired(self.skew_secs),
            None => return Err(AuthError::NotAuthenticated),
        };
        if expired {
            self.refresh_critical_section(&mut guard).await?;
        }
        Ok(guard.as_ref().expect("tokens present").access_token.clone())
    }

    /// Force a refresh regardless of expiry (used by callers on a 401), persisting the
    /// rotation. If another process already rotated, its fresher tokens are adopted instead
    /// of burning that rotation with a second refresh.
    pub async fn force_refresh(&self) -> Result<(), AuthError> {
        let mut guard = self.tokens.lock().await;
        self.refresh_critical_section(&mut guard).await
    }

    /// The reload → refresh → persist critical section, run under the store's exclusive
    /// advisory lock so only one process rotates at a time.
    ///
    /// The adopt rule covers both entry points: if disk holds tokens that differ from memory
    /// and aren't expired, another process already rotated — adopt them. (On the proactive
    /// path memory is expired, so anything fresher is strictly better; on the `force` path
    /// memory just 401'd, so a *different* fresh token is the fix and an *identical* one
    /// means we must rotate.)
    async fn refresh_critical_section(&self, guard: &mut Option<Tokens>) -> Result<(), AuthError> {
        let credentials = self
            .credentials
            .as_ref()
            .ok_or(AuthError::MissingClientCredentials)?;

        // Blocking flock, held only for the short reload/refresh/persist window; contention
        // is rare (two processes refreshing the same expired token at the same moment).
        let _lock = self.store.lock_exclusive()?;

        if let Some(disk) = self.store.load_tokens()? {
            let mem_access = guard.as_ref().map(|t| t.access_token.as_str());
            let differs = mem_access != Some(disk.access_token.as_str());
            if differs && !disk.is_expired(self.skew_secs) {
                *guard = Some(disk);
                return Ok(());
            }
            // Refresh from the freshest persisted rotation, never from stale memory.
            *guard = Some(disk);
        }
        let current = guard.as_ref().ok_or(AuthError::NotAuthenticated)?;

        let refreshed = match refresh_at(&self.token_url, &self.http, credentials, current).await {
            Ok(t) => t,
            // A 400 usually means the refresh token we sent is no longer valid. If disk has
            // moved past what we sent (a rotation by a process not using the lock), retry
            // once with the fresher token before surfacing "re-login".
            Err(AuthError::TokenEndpoint { status: 400, body }) => {
                match self.store.load_tokens()? {
                    Some(d) if d.refresh_token != current.refresh_token => {
                        refresh_at(&self.token_url, &self.http, credentials, &d).await?
                    }
                    _ => return Err(AuthError::TokenEndpoint { status: 400, body }),
                }
            }
            Err(e) => return Err(e),
        };
        self.store.save_tokens(&refreshed)?;
        *guard = Some(refreshed);
        Ok(())
    }
}

/// reqwest middleware that injects `Authorization: Bearer <token>` (refreshing proactively).
pub struct AuthMiddleware {
    manager: Arc<TokenManager>,
}

impl AuthMiddleware {
    pub fn new(manager: Arc<TokenManager>) -> Self {
        Self { manager }
    }
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(
        &self,
        mut req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        let token = self
            .manager
            .access_token()
            .await
            .map_err(reqwest_middleware::Error::middleware)?;
        let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(reqwest_middleware::Error::middleware)?;
        value.set_sensitive(true); // keep the token out of debug/log output
        req.headers_mut().insert(AUTHORIZATION, value);
        next.run(req, extensions).await
    }
}

/// Build an authenticated client for the generated SDK: transient-retry (outer) wrapping the
/// Bearer-injecting auth middleware (inner), so every retry re-injects a fresh token.
pub fn build_authenticated_client(manager: Arc<TokenManager>) -> ClientWithMiddleware {
    let retry = ExponentialBackoff::builder().build_with_max_retries(3);
    ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry))
        .with(AuthMiddleware::new(manager))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_string_contains, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn credentials() -> ClientCredentials {
        ClientCredentials {
            client_id: "cid".into(),
            client_secret: "secret".into(),
        }
    }

    fn expired_tokens(refresh_token: &str) -> Tokens {
        Tokens {
            access_token: format!("stale-access-{refresh_token}"),
            refresh_token: refresh_token.into(),
            expires_at: 0,
            scope: None,
            token_type: None,
        }
    }

    /// A manager whose token-endpoint calls hit the mock server.
    fn test_manager(
        server: &MockServer,
        store: TokenStore,
        tokens: Option<Tokens>,
    ) -> TokenManager {
        let mut m = TokenManager::from_parts(store, Some(credentials()), tokens);
        m.token_url = server.uri();
        m
    }

    #[tokio::test]
    async fn access_token_requires_authentication() {
        let dir = tempfile::tempdir().unwrap();
        let manager =
            TokenManager::from_parts(TokenStore::with_dir(dir.path()), Some(credentials()), None);
        assert!(!manager.is_authenticated().await);
        assert!(matches!(
            manager.access_token().await,
            Err(AuthError::NotAuthenticated)
        ));
    }

    #[tokio::test]
    async fn refresh_without_credentials_reports_missing_credentials() {
        let dir = tempfile::tempdir().unwrap();
        let manager = TokenManager::from_parts(
            TokenStore::with_dir(dir.path()),
            None,
            Some(expired_tokens("r1")),
        );
        assert!(matches!(
            manager.access_token().await,
            Err(AuthError::MissingClientCredentials)
        ));
    }

    #[tokio::test]
    async fn second_manager_adopts_rotation_from_disk_without_calling_endpoint() {
        let server = MockServer::start().await;
        // Exactly ONE refresh is allowed, and only with r1. A second call — replaying the
        // invalidated r1 or burning the rotated r2 — would fail the expectation.
        Mock::given(method("POST"))
            .and(body_string_contains("refresh_token=r1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "fresh-access",
                "refresh_token": "r2",
                "expires_in": 3600
            })))
            .expect(1)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&expired_tokens("r1")).unwrap();

        // Both managers start from the same stale state (r1, expired) — the pre-#22 failure
        // mode: B's next refresh would replay the invalidated r1 and 400.
        let a = test_manager(&server, store.clone(), Some(expired_tokens("r1")));
        let b = test_manager(&server, store.clone(), Some(expired_tokens("r1")));

        // A refreshes: burns r1, persists r2 + a fresh access token.
        assert_eq!(a.access_token().await.unwrap(), "fresh-access");

        // B reloads under the lock and adopts disk state instead of calling the endpoint
        // (the mock's expect(1) enforces that no second call happens).
        assert_eq!(b.access_token().await.unwrap(), "fresh-access");
        assert_eq!(
            store.load_tokens().unwrap().unwrap().refresh_token,
            "r2",
            "rotation persisted exactly once"
        );
    }

    #[tokio::test]
    async fn force_refresh_adopts_fresher_disk_state() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_string_contains("refresh_token=r1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "fresh-access",
                "refresh_token": "r2",
                "expires_in": 3600
            })))
            .expect(1)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&expired_tokens("r1")).unwrap();

        let a = test_manager(&server, store.clone(), Some(expired_tokens("r1")));
        let b = test_manager(&server, store.clone(), Some(expired_tokens("r1")));

        assert_eq!(a.access_token().await.unwrap(), "fresh-access");

        // B's request 401'd (stale token) and it force-refreshes: it must adopt the disk
        // rotation rather than burn r2 with another endpoint call.
        b.force_refresh().await.unwrap();
        assert_eq!(b.access_token().await.unwrap(), "fresh-access");
    }

    #[tokio::test]
    async fn genuinely_invalid_refresh_token_surfaces_the_400_without_blind_retry() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(400).set_body_string("invalid_grant"))
            .expect(1) // the reload-retry only fires when disk moved past what we sent
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&expired_tokens("r-dead")).unwrap();

        let m = test_manager(&server, store, Some(expired_tokens("r-dead")));
        let err = m.access_token().await.unwrap_err();
        assert!(matches!(err, AuthError::TokenEndpoint { status: 400, .. }));
    }
}
