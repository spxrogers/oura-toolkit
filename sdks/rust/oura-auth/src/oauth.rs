//! Token-endpoint calls: authorization-code exchange and refresh (with rotation).
//!
//! These are the non-interactive half of OAuth — plain HTTPS POSTs to the token endpoint.
//! The interactive half (browser + loopback listener) lives in `oura-cli`, which calls
//! [`exchange_code`] once it has caught the authorization code.

use serde::Deserialize;
use time::OffsetDateTime;

use crate::error::AuthError;
use crate::metadata::TOKEN_URL;
use crate::store::Tokens;

/// Raw token-endpoint response (Oura returns a rotated `refresh_token` on every call).
#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i64,
    #[serde(default)]
    token_type: Option<String>,
    #[serde(default)]
    scope: Option<String>,
}

/// Exchange an authorization `code` for tokens (confidential client: sends id + secret).
pub async fn exchange_code(
    http: &reqwest::Client,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<Tokens, AuthError> {
    exchange_code_at(
        TOKEN_URL,
        http,
        client_id,
        client_secret,
        code,
        redirect_uri,
    )
    .await
}

/// Refresh using the stored refresh token, preserving the confidential-client credentials and
/// persisting the **rotated** refresh token (Oura invalidates the previous one).
pub async fn refresh(http: &reqwest::Client, current: &Tokens) -> Result<Tokens, AuthError> {
    refresh_at(TOKEN_URL, http, current).await
}

// --- URL-injectable cores (so tests can point at a mock token endpoint) ----------------------

pub(crate) async fn exchange_code_at(
    token_url: &str,
    http: &reqwest::Client,
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> Result<Tokens, AuthError> {
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", client_id),
        ("client_secret", client_secret),
    ];
    let resp = post_token(token_url, http, &params).await?;
    Ok(Tokens {
        access_token: resp.access_token,
        // On the initial exchange a refresh token is always present.
        refresh_token: resp.refresh_token.unwrap_or_default(),
        expires_at: expires_at(resp.expires_in),
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        scope: resp.scope,
        token_type: resp.token_type,
    })
}

pub(crate) async fn refresh_at(
    token_url: &str,
    http: &reqwest::Client,
    current: &Tokens,
) -> Result<Tokens, AuthError> {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", current.refresh_token.as_str()),
        ("client_id", current.client_id.as_str()),
        ("client_secret", current.client_secret.as_str()),
    ];
    let resp = post_token(token_url, http, &params).await?;
    Ok(Tokens {
        access_token: resp.access_token,
        // Persist the rotated token; fall back to the old one only if the server omits it.
        refresh_token: resp
            .refresh_token
            .unwrap_or_else(|| current.refresh_token.clone()),
        expires_at: expires_at(resp.expires_in),
        client_id: current.client_id.clone(),
        client_secret: current.client_secret.clone(),
        scope: resp.scope.or_else(|| current.scope.clone()),
        token_type: resp.token_type.or_else(|| current.token_type.clone()),
    })
}

async fn post_token(
    token_url: &str,
    http: &reqwest::Client,
    params: &[(&str, &str)],
) -> Result<TokenResponse, AuthError> {
    let resp = http.post(token_url).form(params).send().await?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(AuthError::TokenEndpoint {
            status: status.as_u16(),
            body,
        });
    }
    Ok(resp.json::<TokenResponse>().await?)
}

fn expires_at(expires_in: i64) -> i64 {
    OffsetDateTime::now_utc().unix_timestamp() + expires_in
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{body_string_contains, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn current() -> Tokens {
        Tokens {
            access_token: "old_access".into(),
            refresh_token: "old_refresh".into(),
            expires_at: 0,
            client_id: "cid".into(),
            client_secret: "secret".into(),
            scope: Some("daily".into()),
            token_type: Some("Bearer".into()),
        }
    }

    #[tokio::test]
    async fn refresh_rotates_and_preserves_credentials() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_string_contains("grant_type=refresh_token"))
            .and(body_string_contains("client_secret=secret"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "new_access",
                "refresh_token": "new_refresh",
                "expires_in": 3600,
                "token_type": "Bearer"
            })))
            .mount(&server)
            .await;

        let http = reqwest::Client::new();
        let refreshed = refresh_at(&server.uri(), &http, &current()).await.unwrap();

        assert_eq!(refreshed.access_token, "new_access");
        assert_eq!(refreshed.refresh_token, "new_refresh"); // rotated
        assert_eq!(refreshed.client_id, "cid"); // preserved
        assert_eq!(refreshed.client_secret, "secret"); // preserved
        assert!(refreshed.expires_at > OffsetDateTime::now_utc().unix_timestamp() + 3000);
    }

    #[tokio::test]
    async fn refresh_keeps_old_token_if_server_omits_rotation() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "new_access",
                "expires_in": 3600
            })))
            .mount(&server)
            .await;

        let http = reqwest::Client::new();
        let refreshed = refresh_at(&server.uri(), &http, &current()).await.unwrap();
        assert_eq!(refreshed.refresh_token, "old_refresh");
    }

    #[tokio::test]
    async fn non_2xx_surfaces_status_and_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(400).set_body_string("invalid_grant"))
            .mount(&server)
            .await;

        let http = reqwest::Client::new();
        let err = refresh_at(&server.uri(), &http, &current())
            .await
            .unwrap_err();
        match err {
            AuthError::TokenEndpoint { status, body } => {
                assert_eq!(status, 400);
                assert!(body.contains("invalid_grant"));
            }
            other => panic!("expected TokenEndpoint, got {other:?}"),
        }
    }
}
