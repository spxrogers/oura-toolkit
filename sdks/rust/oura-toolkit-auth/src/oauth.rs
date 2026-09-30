//! Token-endpoint calls: authorization-code exchange and refresh (with rotation).
//!
//! These are the non-interactive half of OAuth — plain HTTPS POSTs to the token endpoint.
//! The interactive half (browser + loopback listener) lives in `oura-toolkit-cli`, which calls
//! [`exchange_code`] once it has caught the authorization code. Oura is a confidential
//! client, so both calls carry the caller's [`ClientCredentials`]; the credentials live in
//! their own store record and are never embedded in the returned [`Tokens`].

use serde::Deserialize;
use time::OffsetDateTime;

use crate::error::AuthError;
use crate::metadata::TOKEN_URL;
use crate::store::{ClientCredentials, Tokens};

/// Raw token-endpoint response (Oura returns a rotated `refresh_token` on every call).
///
/// `Debug` is implemented manually and REDACTS the token fields — this struct holds the same
/// secret material as [`Tokens`], so a stray `{:?}`/`dbg!`/`tracing::debug!(?resp)` while
/// debugging the token endpoint must not leak it into logs (the "no secrets in logs" rule).
#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    refresh_token: Option<String>,
    expires_in: i64,
    #[serde(default)]
    token_type: Option<String>,
    /// Informational, so lenient: a non-string `scope` reads as absent (keep the prior grant)
    /// rather than failing a refresh whose rotated refresh token would then be burned
    /// (conformance `refresh_success_cases`: `scope_wrong_type`).
    #[serde(default, deserialize_with = "string_or_absent")]
    scope: Option<String>,
}

/// `Some` for a JSON string, `None` for anything else (null, a number, an object, …).
fn string_or_absent<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    Ok(match serde_json::Value::deserialize(d)? {
        serde_json::Value::String(s) => Some(s),
        _ => None,
    })
}

impl std::fmt::Debug for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenResponse")
            .field("access_token", &"[REDACTED]")
            // Redact whether a token was even present, not just its value: a bare
            // `Some("[REDACTED]")` vs `None` still discloses rotation behavior.
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("expires_in", &self.expires_in)
            .field("token_type", &self.token_type)
            .field("scope", &self.scope)
            .finish()
    }
}

/// Exchange an authorization `code` for tokens (confidential client: sends id + secret).
pub async fn exchange_code(
    http: &reqwest::Client,
    credentials: &ClientCredentials,
    code: &str,
    redirect_uri: &str,
) -> Result<Tokens, AuthError> {
    exchange_code_at(TOKEN_URL, http, credentials, code, redirect_uri).await
}

/// Refresh using the stored refresh token; the response carries a **rotated** refresh token
/// which the caller MUST persist (Oura invalidates the previous one).
pub async fn refresh(
    http: &reqwest::Client,
    credentials: &ClientCredentials,
    current: &Tokens,
) -> Result<Tokens, AuthError> {
    refresh_at(TOKEN_URL, http, credentials, current).await
}

// --- URL-injectable cores (so tests can point at a mock token endpoint) ----------------------

pub async fn exchange_code_at(
    token_url: &str,
    http: &reqwest::Client,
    credentials: &ClientCredentials,
    code: &str,
    redirect_uri: &str,
) -> Result<Tokens, AuthError> {
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", redirect_uri),
        ("client_id", credentials.client_id.as_str()),
        ("client_secret", credentials.client_secret.as_str()),
    ];
    let resp = post_token(token_url, http, &params).await?;
    // The initial exchange must return a refresh token — persisting an absent or EMPTY one
    // would only surface as a baffling 400 on the NEXT refresh, long after the cause. Fail
    // loud now (there is no prior token to fall back to, unlike a refresh).
    let refresh_token = resp
        .refresh_token
        .filter(|t| !t.is_empty())
        .ok_or(AuthError::MissingRefreshToken)?;
    Ok(Tokens {
        access_token: resp.access_token,
        refresh_token,
        expires_at: expires_at(resp.expires_in),
        scope: resp.scope,
        token_type: resp.token_type,
    })
}

pub(crate) async fn refresh_at(
    token_url: &str,
    http: &reqwest::Client,
    credentials: &ClientCredentials,
    current: &Tokens,
) -> Result<Tokens, AuthError> {
    let params = [
        ("grant_type", "refresh_token"),
        ("refresh_token", current.refresh_token.as_str()),
        ("client_id", credentials.client_id.as_str()),
        ("client_secret", credentials.client_secret.as_str()),
    ];
    let resp = post_token(token_url, http, &params).await?;
    Ok(Tokens {
        access_token: resp.access_token,
        // Persist the rotated token; fall back to the old one only if the server omits it
        // (absent, null or EMPTY — persisting "" would make the next refresh 400; conformance
        // `refresh_success_cases`: refresh_token_empty).
        refresh_token: resp
            .refresh_token
            .filter(|t| !t.is_empty())
            .unwrap_or_else(|| current.refresh_token.clone()),
        expires_at: expires_at(resp.expires_in),
        // An omitted OR blank scope keeps the recorded grant (RFC 6749 §5.1 lets the server
        // omit an unchanged scope; a blank would erase the grant the CLI's re-consent check
        // reads — conformance `refresh_success_cases`).
        scope: resp
            .scope
            .filter(|s| !s.trim().is_empty())
            .or_else(|| current.scope.clone()),
        token_type: resp
            .token_type
            .filter(|t| !t.is_empty())
            .or_else(|| current.token_type.clone()),
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
            body: redacted_error_body(&body, params),
        });
    }
    let bytes = resp.bytes().await?;
    // The body must be UTF-8 JSON text (RFC 8259 §8.1) — checked over the WHOLE body, since
    // serde_json doesn't validate the strings of fields it skips. Conformance:
    // body_invalid_utf8_in_scope / body_invalid_utf8_in_unknown_field.
    if std::str::from_utf8(&bytes).is_err() {
        return Err(AuthError::InvalidTokenResponse("body is not valid UTF-8"));
    }
    // Static messages: a serde error can quote the server's values back.
    let resp = serde_json::from_slice::<TokenResponse>(&bytes)
        .map_err(|_| AuthError::InvalidTokenResponse("body is not a well-formed token response"))?;
    // Hostile-but-2xx burn prevention (#58, conformance: codegen/conformance/
    // auth-cases.json): a 200 whose payload would install a blank or already-expired
    // Bearer must fail typed BEFORE any caller can persist it — persisting would also
    // burn the still-valid rotated refresh token. (Missing/wrong-typed fields already
    // fail serde decode above; these are the well-typed-but-unusable shapes.)
    if resp.access_token.is_empty() {
        return Err(AuthError::InvalidTokenResponse("empty access_token"));
    }
    if !(1..=MAX_EXPIRES_IN_SECS).contains(&resp.expires_in) {
        return Err(AuthError::InvalidTokenResponse("expires_in out of range"));
    }
    Ok(resp)
}

/// The most of a rejected response's body an error carries (characters): enough for an OAuth
/// error (`{"error":"invalid_grant",…}`), not a whole HTML error page in stderr or an MCP result.
pub(crate) const MAX_ERROR_BODY_CHARS: usize = 1024;

/// A non-2xx body, kept for diagnosis but with every secret this request SUBMITTED (the refresh
/// token, the client secret, an authorization code) replaced by `[REDACTED]` — some OAuth
/// servers echo the value they reject — and capped at [`MAX_ERROR_BODY_CHARS`] (redacted first,
/// so a cut can't expose a partial secret). Conformance: `rejected_token_responses`.
fn redacted_error_body(body: &str, params: &[(&str, &str)]) -> String {
    let mut body = body.to_owned();
    for (key, value) in params {
        if matches!(*key, "refresh_token" | "client_secret" | "code") && !value.is_empty() {
            body = body.replace(value, "[REDACTED]");
        }
    }
    match body.char_indices().nth(MAX_ERROR_BODY_CHARS) {
        Some((cut, _)) => format!("{}…", &body[..cut]),
        None => body,
    }
}

/// The largest `expires_in` (seconds, ~68 years) any companion accepts. Capping it keeps
/// `now + expires_in` exact in every language's store (a JS double, Rust's `i64`) so no
/// companion can write an `expires_at` another can't read back; Oura's real lifetimes are
/// about a day. Shared contract: conformance expires_in_above_cap / expires_in_at_cap.
pub(crate) const MAX_EXPIRES_IN_SECS: i64 = 2_147_483_647;

fn expires_at(expires_in: i64) -> i64 {
    // Can't overflow: post_token capped expires_in at MAX_EXPIRES_IN_SECS.
    OffsetDateTime::now_utc().unix_timestamp() + expires_in
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

    fn current() -> Tokens {
        Tokens {
            access_token: "old_access".into(),
            refresh_token: "old_refresh".into(),
            expires_at: 0,
            scope: Some("daily".into()),
            token_type: Some("Bearer".into()),
        }
    }

    #[tokio::test]
    async fn refresh_rotates_and_sends_client_credentials() {
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
        let refreshed = refresh_at(&server.uri(), &http, &credentials(), &current())
            .await
            .unwrap();

        assert_eq!(refreshed.access_token, "new_access");
        assert_eq!(refreshed.refresh_token, "new_refresh"); // rotated
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
        let refreshed = refresh_at(&server.uri(), &http, &credentials(), &current())
            .await
            .unwrap();
        assert_eq!(refreshed.refresh_token, "old_refresh");
    }

    #[tokio::test]
    async fn exchange_without_refresh_token_errors() {
        // Absent, null and EMPTY all mean "no refresh token": the exchange has no prior
        // token to fall back to, so each must fail loud rather than persist a dead login.
        for (name, body) in [
            (
                "absent",
                json!({ "access_token": "new_access", "expires_in": 3600 }),
            ),
            (
                "null",
                json!({ "access_token": "new_access", "expires_in": 3600, "refresh_token": null }),
            ),
            (
                "empty",
                json!({ "access_token": "new_access", "expires_in": 3600, "refresh_token": "" }),
            ),
        ] {
            let server = MockServer::start().await;
            Mock::given(method("POST"))
                .respond_with(ResponseTemplate::new(200).set_body_json(body))
                .mount(&server)
                .await;

            let http = reqwest::Client::new();
            let err = exchange_code_at(
                &server.uri(),
                &http,
                &credentials(),
                "code",
                "http://localhost:8788/callback",
            )
            .await
            .unwrap_err();
            assert!(
                matches!(err, AuthError::MissingRefreshToken),
                "{name} refresh_token: expected MissingRefreshToken, got {err:?}"
            );
        }
    }

    #[tokio::test]
    async fn non_2xx_surfaces_status_and_body() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(400).set_body_string("invalid_grant"))
            .mount(&server)
            .await;

        let http = reqwest::Client::new();
        let err = refresh_at(&server.uri(), &http, &credentials(), &current())
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

    #[test]
    fn token_response_debug_redacts_secrets() {
        // TokenResponse holds the same secret material as `Tokens`; its `Debug` must never
        // leak it (the "no secrets in logs" rule). Break-verify: swap either `[REDACTED]` for
        // the real field in the manual impl and this fails naming the leaked token.
        let resp = TokenResponse {
            access_token: "SECRET-AT-123".into(),
            refresh_token: Some("SECRET-RT-456".into()),
            expires_in: 3600,
            token_type: Some("Bearer".into()),
            scope: Some("daily".into()),
        };
        let dbg = format!("{resp:?}");
        assert!(!dbg.contains("SECRET-AT-123"), "access token leaked: {dbg}");
        assert!(
            !dbg.contains("SECRET-RT-456"),
            "refresh token leaked: {dbg}"
        );
        assert!(
            dbg.contains("[REDACTED]"),
            "expected redaction marker: {dbg}"
        );
        // Non-secret fields stay visible so `Debug` remains useful for endpoint debugging.
        assert!(
            dbg.contains("3600"),
            "expires_in should remain visible: {dbg}"
        );
        assert!(
            dbg.contains("Bearer"),
            "token_type should remain visible: {dbg}"
        );

        // A `None` refresh token must render as `None`, not `Some("[REDACTED]")` — the latter
        // would still disclose that the server rotated a token on this call.
        let no_refresh = TokenResponse {
            refresh_token: None,
            ..resp
        };
        let dbg = format!("{no_refresh:?}");
        assert!(
            dbg.contains("refresh_token: None"),
            "absent refresh token should render as None: {dbg}"
        );
    }
}
