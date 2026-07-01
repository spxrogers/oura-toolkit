//! The runtime auth layer: a [`TokenManager`] that owns the token state and refreshes it, an
//! [`AuthMiddleware`] that injects a fresh Bearer token on every request, and a builder that
//! composes auth + transient-retry into a `ClientWithMiddleware` for the generated SDK client.
//!
//! Refresh strategy: **proactive** — the middleware refreshes when the access token is expired
//! or within a skew window, so requests carry a valid token. A reqwest `Middleware` cannot
//! re-run the request chain, so reactive refresh-on-401-then-retry is done by callers via
//! [`TokenManager::force_refresh`] (wired at the CLI data commands #9 and MCP tool calls #10).

use std::sync::Arc;

use http::Extensions;
use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::{Request, Response};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Middleware, Next};
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use tokio::sync::Mutex;

use crate::error::AuthError;
use crate::oauth::refresh;
use crate::store::{TokenStore, Tokens};

/// Refresh this many seconds before the token's actual expiry.
const DEFAULT_SKEW_SECS: i64 = 60;

/// Owns the current tokens and the machinery to keep them fresh. Shared (behind `Arc`) by the
/// CLI's SDK calls and the MCP server's tool calls — one auth layer, two consumers.
pub struct TokenManager {
    store: TokenStore,
    tokens: Mutex<Option<Tokens>>,
    http: reqwest::Client,
    skew_secs: i64,
}

impl TokenManager {
    /// Load from the default token store (no error if absent — [`Self::access_token`] reports
    /// [`AuthError::NotAuthenticated`] on first use).
    pub fn load() -> Result<Self, AuthError> {
        let store = TokenStore::new()?;
        let tokens = store.load()?;
        Ok(Self::from_parts(store, tokens))
    }

    /// Construct from an explicit store + optional in-memory tokens.
    pub fn from_parts(store: TokenStore, tokens: Option<Tokens>) -> Self {
        Self {
            store,
            tokens: Mutex::new(tokens),
            // A plain client (no auth middleware) for token-endpoint calls, to avoid recursion.
            http: reqwest::Client::new(),
            skew_secs: DEFAULT_SKEW_SECS,
        }
    }

    /// Whether any credentials are loaded (does not validate them).
    pub async fn is_authenticated(&self) -> bool {
        self.tokens.lock().await.is_some()
    }

    /// Return a valid access token, refreshing (and persisting the rotation) if needed.
    pub async fn access_token(&self) -> Result<String, AuthError> {
        let mut guard = self.tokens.lock().await;
        let tokens = guard.as_ref().ok_or(AuthError::NotAuthenticated)?;
        if tokens.is_expired(self.skew_secs) {
            let refreshed = refresh(&self.http, tokens).await?;
            self.store.save(&refreshed)?;
            *guard = Some(refreshed);
        }
        Ok(guard.as_ref().expect("tokens present").access_token.clone())
    }

    /// Force a refresh regardless of expiry (used by callers on a 401), persisting the rotation.
    pub async fn force_refresh(&self) -> Result<(), AuthError> {
        let mut guard = self.tokens.lock().await;
        let tokens = guard.as_ref().ok_or(AuthError::NotAuthenticated)?;
        let refreshed = refresh(&self.http, tokens).await?;
        self.store.save(&refreshed)?;
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

    #[tokio::test]
    async fn access_token_requires_authentication() {
        let dir = tempfile::tempdir().unwrap();
        let manager = TokenManager::from_parts(TokenStore::with_dir(dir.path()), None);
        assert!(!manager.is_authenticated().await);
        assert!(matches!(
            manager.access_token().await,
            Err(AuthError::NotAuthenticated)
        ));
    }
}
