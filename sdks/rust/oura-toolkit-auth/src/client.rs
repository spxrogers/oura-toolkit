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
//! "re-login". The store is authoritative in the other direction too: a token record
//! DELETED on disk (a peer's `oura auth logout`) makes the refresh report
//! [`AuthError::NotAuthenticated`] and drop the in-memory copy — never resurrect the
//! record by refreshing from stale memory.

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

/// Refresh this many seconds before the token's actual expiry. Public so callers that
/// PREDICT the manager's behavior (the CLI's `auth status`) use the same window instead
/// of a drifting copy.
pub const REFRESH_SKEW_SECS: i64 = 60;

/// Hard timeout on each token-endpoint call. This bounds how long the store's exclusive
/// lock can be held (the refresh runs under it) — without it, one process's stalled refresh
/// would wedge every other process waiting on the lock. Worst case is ~2× this value: the
/// 400-retry arm can chain a second endpoint call under the same lock.
const TOKEN_ENDPOINT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

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
    /// True when the manager wraps a caller-supplied access token
    /// ([`Self::from_access_token`]): it never touches the store and cannot refresh, so a
    /// 401 surfaces as [`AuthError::StaticTokenRejected`] rather than a store/credentials
    /// error the env-token user can't act on.
    env_token: bool,
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
    ///
    /// Both records are independently optional because both partial states are legitimate:
    /// credentials-without-tokens is `auth setup` completed but no login yet (refresh-able
    /// once tokens arrive), and tokens-without-credentials is a caller-supplied token that can
    /// be used until expiry but not refreshed ([`AuthError::MissingClientCredentials`]). (The
    /// `OURA_ACCESS_TOKEN` override, #20, does NOT use this path — it goes through the
    /// dedicated [`Self::from_access_token`], which reports [`AuthError::StaticTokenRejected`]
    /// on a 401 rather than `MissingClientCredentials`.)
    ///
    /// Refresh additionally requires the token record to still EXIST on disk: the store is
    /// the cross-process source of truth, so in-memory tokens whose record was deleted (a
    /// peer's `oura auth logout`) refresh to [`AuthError::NotAuthenticated`], never to a
    /// resurrected record.
    pub fn from_parts(
        store: TokenStore,
        credentials: Option<ClientCredentials>,
        tokens: Option<Tokens>,
    ) -> Self {
        Self {
            store,
            credentials,
            tokens: Mutex::new(tokens),
            // A plain client (no auth middleware) for token-endpoint calls, to avoid
            // recursion. The timeout is load-bearing: the call runs under the store's
            // exclusive lock, so an unbounded hang would block other processes too.
            http: reqwest::Client::builder()
                .timeout(TOKEN_ENDPOINT_TIMEOUT)
                .build()
                .expect("default reqwest client"),
            skew_secs: REFRESH_SKEW_SECS,
            token_url: TOKEN_URL.to_string(),
            env_token: false,
        }
    }

    /// A manager backed by a **caller-supplied access token** (the CLI's `OURA_ACCESS_TOKEN`
    /// override, #20): [`Self::access_token`] returns it verbatim, the store is never read or
    /// written, and it cannot refresh. A 401 (the token is expired or invalid) surfaces as
    /// [`AuthError::StaticTokenRejected`] so the caller can tell the user to supply a fresh
    /// token instead of pointing at an interactive login it wouldn't help.
    ///
    /// The token is marked non-expiring (`expires_at = i64::MAX`) precisely so the proactive
    /// path never fires — the only way to leave "valid" is a real 401 from the API, which is
    /// the case where refresh is genuinely impossible. The store handle is a throwaway
    /// (`temp_dir`) that is never touched.
    pub fn from_access_token(token: String) -> Self {
        let tokens = Tokens {
            access_token: token,
            refresh_token: String::new(),
            expires_at: i64::MAX,
            scope: None,
            token_type: None,
        };
        let mut manager = Self::from_parts(
            TokenStore::with_dir(std::env::temp_dir()),
            None,
            Some(tokens),
        );
        manager.env_token = true;
        manager
    }

    /// Point token-endpoint calls at a mock server (hermetic downstream tests only).
    #[cfg(feature = "test-util")]
    pub fn override_token_url(&mut self, url: String) {
        self.token_url = url;
    }

    /// Whether tokens are loaded (does not validate them, and does not imply a refresh is
    /// possible — refresh additionally needs the client-credentials record).
    pub async fn is_authenticated(&self) -> bool {
        self.tokens.lock().await.is_some()
    }

    /// Return a valid access token, refreshing (and persisting the rotation) if needed.
    ///
    /// Must be called within a **tokio runtime**: refresh acquires the store lock on the
    /// runtime's blocking pool (calling from a non-tokio executor panics at that point).
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
    ///
    /// Must be called within a **tokio runtime** (see [`Self::access_token`]).
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
        // A caller-supplied token (OURA_ACCESS_TOKEN) has no refresh path; a 401 here means
        // it is expired/invalid. Report that BEFORE the store/credentials machinery so the
        // env-token user gets an actionable error, not "no client credentials stored".
        if self.env_token {
            return Err(AuthError::StaticTokenRejected);
        }
        let credentials = self
            .credentials
            .as_ref()
            .ok_or(AuthError::MissingClientCredentials)?;

        // The lock acquisition is a blocking syscall that can wait on another process's
        // refresh, so it must not sit on an executor thread (on a current-thread runtime it
        // would halt the whole process — e.g. a stdio MCP server's JSON-RPC loop). Acquire
        // it on the blocking pool; the hold duration is bounded by TOKEN_ENDPOINT_TIMEOUT.
        let store = self.store.clone();
        let _lock = tokio::task::spawn_blocking(move || store.lock_exclusive())
            .await
            .map_err(|e| {
                AuthError::Io(std::io::Error::other(format!(
                    "store lock task failed: {e}"
                )))
            })??;

        match self.store.load_tokens()? {
            Some(disk) => {
                let mem_access = guard.as_ref().map(|t| t.access_token.as_str());
                let differs = mem_access != Some(disk.access_token.as_str());
                if differs && !disk.is_expired(self.skew_secs) {
                    *guard = Some(disk);
                    return Ok(());
                }
                // Refresh from the freshest persisted rotation, never from stale memory.
                *guard = Some(disk);
            }
            None => {
                // No record on disk means another process deleted it (`oura auth logout`).
                // The store is authoritative: refreshing from our stale in-memory copy here
                // would re-persist tokens the user just asked to remove (the "logout
                // resurrection" hole). Drop the copy and report unauthenticated instead.
                *guard = None;
                return Err(AuthError::NotAuthenticated);
            }
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
    async fn static_token_manager_returns_the_token_and_never_expires() {
        // OURA_ACCESS_TOKEN path (#20): the token is returned verbatim, and repeated reads
        // never trip the proactive-refresh window (expires_at = i64::MAX), so the store is
        // never touched — no MockServer is mounted, so any endpoint call would fail the test.
        let m = TokenManager::from_access_token("env-abc".into());
        assert!(m.is_authenticated().await);
        assert_eq!(m.access_token().await.unwrap(), "env-abc");
        assert_eq!(
            m.access_token().await.unwrap(),
            "env-abc",
            "a static token must not expire into a refresh"
        );
    }

    #[tokio::test]
    async fn static_token_manager_rejects_on_forced_refresh() {
        // A 401 drives force_refresh; a caller-supplied token has no refresh path, so it must
        // surface StaticTokenRejected — NOT MissingClientCredentials, which would send the
        // env-token user to `oura auth setup` that can't help. Break-verify: drop the
        // `env_token` short-circuit in refresh_critical_section and this reports
        // MissingClientCredentials instead.
        let m = TokenManager::from_access_token("env-abc".into());
        let err = m.force_refresh().await.unwrap_err();
        assert!(matches!(err, AuthError::StaticTokenRejected), "{err:?}");
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

    /// The lock's reason to exist: two managers refreshing CONCURRENTLY must serialize and
    /// produce exactly one endpoint call — the loser adopts the winner's persisted rotation.
    /// (Sequential versions of this test would pass even with a no-op lock.)
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrent_refreshes_serialize_to_a_single_endpoint_call() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_string_contains("refresh_token=r1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "fresh-access",
                "refresh_token": "r2",
                "expires_in": 3600
            })))
            .expect(1) // a concurrent double-refresh would send r1 twice (or burn r2)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&expired_tokens("r1")).unwrap();

        let a = std::sync::Arc::new(test_manager(
            &server,
            store.clone(),
            Some(expired_tokens("r1")),
        ));
        let b = std::sync::Arc::new(test_manager(
            &server,
            store.clone(),
            Some(expired_tokens("r1")),
        ));

        let (ra, rb) = tokio::join!(a.access_token(), b.access_token());
        assert_eq!(ra.unwrap(), "fresh-access");
        assert_eq!(rb.unwrap(), "fresh-access");
        assert_eq!(store.load_tokens().unwrap().unwrap().refresh_token, "r2");
    }

    /// The 400-retry arm: a rotation performed by a writer NOT holding the lock lands on
    /// disk after our reload but before our request is answered. The endpoint 400s the
    /// stale token; the manager must reload, see the fresher refresh token, and retry once
    /// with it — successfully.
    #[tokio::test]
    async fn refresh_400_retries_once_against_fresher_disk_state() {
        struct RotateDiskThen400 {
            store: TokenStore,
        }
        impl wiremock::Respond for RotateDiskThen400 {
            fn respond(&self, _req: &wiremock::Request) -> ResponseTemplate {
                // Simulate an uncoordinated writer rotating to r2 while our r1 request is
                // in flight, then reject the (now stale) r1.
                self.store
                    .save_tokens(&Tokens {
                        access_token: "r2-access".into(),
                        refresh_token: "r2".into(),
                        expires_at: 0, // expired, so the retry must actually refresh
                        scope: None,
                        token_type: None,
                    })
                    .unwrap();
                ResponseTemplate::new(400).set_body_string("invalid_grant")
            }
        }

        let server = MockServer::start().await;
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&expired_tokens("r1")).unwrap();

        Mock::given(method("POST"))
            .and(body_string_contains("refresh_token=r1"))
            .respond_with(RotateDiskThen400 {
                store: store.clone(),
            })
            .expect(1)
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(body_string_contains("refresh_token=r2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "r3-access",
                "refresh_token": "r3",
                "expires_in": 3600
            })))
            .expect(1)
            .mount(&server)
            .await;

        let m = test_manager(&server, store.clone(), Some(expired_tokens("r1")));
        assert_eq!(m.access_token().await.unwrap(), "r3-access");
        assert_eq!(store.load_tokens().unwrap().unwrap().refresh_token, "r3");
    }

    /// The cross-process liveness guarantee, both halves: the lock is genuinely HELD while
    /// the refresh is in flight (a peer would block), and a hung endpoint releases it at
    /// the timeout instead of wedging peers forever. Uses the same-module seam to shrink
    /// the timeout.
    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn hung_endpoint_holds_the_lock_then_times_out_and_releases_it() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!({"access_token": "late", "expires_in": 3600}))
                    .set_delay(std::time::Duration::from_secs(30)),
            )
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&expired_tokens("r1")).unwrap();

        let mut m = TokenManager::from_parts(
            TokenStore::with_dir(dir.path()),
            Some(credentials()),
            Some(expired_tokens("r1")),
        );
        m.token_url = server.uri();
        // 2s timeout: long enough to observe the lock being held mid-flight without racing,
        // short enough to keep the test fast.
        m.http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .unwrap();

        let started = std::time::Instant::now();
        let refresh = tokio::spawn(async move { m.access_token().await });

        // While the refresh stalls on the hung endpoint, the lock must be observed HELD
        // (this is what a second process would block on). Poll until we see it; if the
        // lock were never taken, this loop exhausts and fails.
        let mut observed_held = false;
        for _ in 0..100 {
            if store.try_lock_exclusive().unwrap().is_none() {
                observed_held = true;
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
        assert!(
            observed_held,
            "the store lock must be held during an in-flight refresh"
        );

        // The stalled refresh must time out (bounded) and release the lock.
        let err = refresh.await.unwrap().unwrap_err();
        assert!(
            matches!(err, AuthError::Http(_)),
            "expected timeout error, got {err:?}"
        );
        assert!(
            started.elapsed() < std::time::Duration::from_secs(15),
            "timeout must bound the stall"
        );
        assert!(
            store.try_lock_exclusive().unwrap().is_some(),
            "lock must be released after a timed-out refresh"
        );
    }

    /// The logout contract's cross-process half (#18): a live manager whose token record
    /// was DELETED by another process (`oura auth logout`) must NOT resurrect it. The store
    /// is authoritative — the refresh reports unauthenticated, drops the in-memory copy,
    /// and never calls the endpoint or re-persists.
    #[tokio::test]
    async fn refresh_after_a_peer_logout_reports_unauthenticated_instead_of_resurrecting() {
        let server = MockServer::start().await;
        // ANY token-endpoint call here would mint a token the user just deleted — forbid all.
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "resurrected",
                "refresh_token": "r2",
                "expires_in": 3600
            })))
            .expect(0)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&expired_tokens("r1")).unwrap();
        let m = test_manager(&server, store.clone(), Some(expired_tokens("r1")));

        // Another process logs out while this manager still holds tokens in memory.
        store.delete_tokens().unwrap();

        let err = m.access_token().await.unwrap_err();
        assert!(matches!(err, AuthError::NotAuthenticated), "{err:?}");
        assert!(
            !store.tokens_path().exists(),
            "the refresh must not re-create the deleted token record"
        );
        assert!(
            !m.is_authenticated().await,
            "the stale in-memory copy must be dropped"
        );

        // force_refresh runs the same critical section — same guarantee.
        let err = m.force_refresh().await.unwrap_err();
        assert!(matches!(err, AuthError::NotAuthenticated), "{err:?}");
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
