//! `oura api` (#19): an authenticated escape hatch to any Oura endpoint, like `gh api`.
//!
//! This is the ONE sanctioned hand-rolled request in the CLI: the typed data commands go
//! through the generated `oura-toolkit-api` client, but an arbitrary-path passthrough has
//! no generated operation to call, so it issues a raw `reqwest` request. It reuses the
//! rest of the data plane's contract — the `oura-toolkit-auth` Bearer, the single
//! 401-refresh-retry (mirroring `api::attempt`), and `api::paginate`'s `next_token` loop —
//! and prints the raw JSON response to stdout unchanged (it is machine-consumable data,
//! like every other data command; only error bodies bound for stderr are sanitized).

use anyhow::{Context, Result};
use oura_toolkit_auth::{AuthError, TokenManager};
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Method, StatusCode};

use crate::contract::UsageError;

/// The fully resolved request an `oura api` invocation will send (per attempt): the method,
/// the absolute URL, the query params (already including any `-f` fields for a query
/// method), and the optional raw JSON body. Pagination appends `next_token` to [`query`] at
/// send time; the Bearer header is injected per attempt so a post-refresh retry picks up the
/// rotated token.
#[derive(Debug)]
struct RequestPlan {
    method: Method,
    url: String,
    query: Vec<(String, String)>,
    body: Option<String>,
}

/// Resolve a positional `path` against the data-plane base URL: `{base_url}{path}`, with a
/// leading `/` prepended when the user omitted it, so both `oura api v2/…` and
/// `oura api /v2/…` reach `{base_url}/v2/…`.
fn resolve_url(base_url: &str, path: &str) -> String {
    if path.starts_with('/') {
        format!("{base_url}{path}")
    } else {
        format!("{base_url}/{path}")
    }
}

/// Parse a `-f/--field` argument into `(key, value)`. A field without `=` is a usage error
/// (exit 2): `gh api -f foo` is a mistake, not an empty-valued field.
fn parse_field(raw: &str) -> Result<(String, String)> {
    match raw.split_once('=') {
        Some((k, v)) => Ok((k.to_string(), v.to_string())),
        None => Err(UsageError(format!(
            "invalid field {raw:?} — expected key=value (e.g. -f start_date=2026-06-01)"
        ))
        .into()),
    }
}

/// Whether `-f` fields become query params (GET/HEAD/DELETE) rather than a JSON body.
fn is_query_method(method: &Method) -> bool {
    matches!(method, &Method::GET | &Method::HEAD | &Method::DELETE)
}

impl RequestPlan {
    /// Build the plan from the CLI inputs, applying the field/body rules:
    /// - GET/HEAD/DELETE → `-f` fields are query params; a stdin body (if any) is still sent
    ///   raw (no conflict — the fields aren't a body).
    /// - other methods → `-f` fields build a `{"key":"value"}` JSON body; supplying BOTH a
    ///   stdin body AND `-f` fields is a usage error (two bodies).
    fn build(
        base_url: &str,
        path: &str,
        method_str: &str,
        fields: &[String],
        stdin_body: Option<String>,
    ) -> Result<Self> {
        let url = resolve_url(base_url, path);
        let method = Method::from_bytes(method_str.to_uppercase().as_bytes())
            .map_err(|_| UsageError(format!("invalid HTTP method {method_str:?}")))?;

        let parsed: Vec<(String, String)> = fields
            .iter()
            .map(|f| parse_field(f))
            .collect::<Result<_>>()?;

        let (query, body) = if is_query_method(&method) {
            (parsed, stdin_body)
        } else {
            if stdin_body.is_some() && !parsed.is_empty() {
                return Err(UsageError(
                    "cannot combine a request body on stdin with -f/--field body fields — \
                     pass one or the other"
                        .to_string(),
                )
                .into());
            }
            let body = match stdin_body {
                Some(b) => Some(b),
                None if parsed.is_empty() => None,
                None => {
                    let obj: serde_json::Map<String, serde_json::Value> = parsed
                        .into_iter()
                        .map(|(k, v)| (k, serde_json::Value::String(v)))
                        .collect();
                    Some(serde_json::to_string(&serde_json::Value::Object(obj))?)
                }
            };
            (Vec::new(), body)
        };

        Ok(Self {
            method,
            url,
            query,
            body,
        })
    }
}

/// Run an `oura api` invocation and return the JSON to print (raw response body, or — for
/// `--paginate` — the aggregated `{"data":[…]}` object, pretty-printed).
///
/// `stdin_body` is `Some` only when stdin was a non-empty non-TTY stream (see
/// `main::read_stdin_body`).
pub async fn run(
    manager: &TokenManager,
    base_url: &str,
    path: &str,
    method: &str,
    fields: &[String],
    stdin_body: Option<String>,
    paginate: bool,
) -> Result<String> {
    let plan = RequestPlan::build(base_url, path, method, fields, stdin_body)?;
    // Same 30s per-request timeout as `api::authorized_client`; a fresh Bearer is injected
    // per attempt inside `send`, so this client carries no default auth header.
    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("building the HTTP client")?;

    if paginate {
        let all = crate::api::paginate(|token| {
            let http = &http;
            let plan = &plan;
            async move {
                let body = send_and_read(http, manager, plan, token.as_deref(), path).await?;
                let json: serde_json::Value = serde_json::from_str(&body)
                    .context("parsing the paginated response body as JSON")?;
                let data = json
                    .get("data")
                    .and_then(|d| d.as_array())
                    .cloned()
                    .unwrap_or_default();
                let next = json
                    .get("next_token")
                    .and_then(|t| t.as_str())
                    .map(str::to_string);
                Ok((data, next))
            }
        })
        .await?;
        let aggregated = serde_json::json!({ "data": all });
        Ok(
            serde_json::to_string_pretty(&aggregated)
                .context("serializing the aggregated pages")?
                + "\n",
        )
    } else {
        send_and_read(&http, manager, &plan, None, path).await
    }
}

/// Send one request (with the single 401-refresh-retry) and read its body. On a non-2xx
/// response the (sanitized) body is written to stderr and a runtime error is returned; on
/// 2xx the raw body is returned for stdout.
async fn send_and_read(
    http: &reqwest::Client,
    manager: &TokenManager,
    plan: &RequestPlan,
    next_token: Option<&str>,
    path: &str,
) -> Result<String> {
    let resp = send(http, manager, plan, next_token).await?;
    let status = resp.status();
    let body = resp.text().await.context("reading the response body")?;
    if !status.is_success() {
        // The error body is server-controlled text going to a terminal, so sanitize it
        // (escape sequences / forged lines) — unlike the success body, which is machine-
        // consumable data emitted verbatim to stdout.
        crate::contract::inform(&format!("{}\n", crate::output::sanitize(&body)));
        return Err(anyhow::anyhow!("HTTP {}", status.as_u16())
            .context(format!("api request to {path} failed")));
    }
    Ok(body)
}

/// Issue the request with the contract's 401 semantics, mirroring `api::attempt` for a raw
/// reqwest request: attach a fresh Bearer, and on a 401 force a refresh (adopting a peer's
/// rotation if one exists) and retry EXACTLY once — a second 401 is a dead session, surfaced
/// as the typed [`AuthError::NotAuthenticated`] (exit 4 + login hint). The request is rebuilt
/// per attempt via a closure because a `reqwest` body is not cloneable.
async fn send(
    http: &reqwest::Client,
    manager: &TokenManager,
    plan: &RequestPlan,
    next_token: Option<&str>,
) -> Result<reqwest::Response> {
    let build = |token: &str| -> Result<reqwest::RequestBuilder> {
        let mut req = http.request(plan.method.clone(), &plan.url);
        if !plan.query.is_empty() || next_token.is_some() {
            let mut query = plan.query.clone();
            if let Some(t) = next_token {
                query.push(("next_token".to_string(), t.to_string()));
            }
            req = req.query(&query);
        }
        let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
            .context("access token contains bytes not valid in an HTTP header")?;
        value.set_sensitive(true); // keep the token out of any debug/log output
        req = req.header(AUTHORIZATION, value);
        if let Some(body) = &plan.body {
            req = req
                .header(CONTENT_TYPE, "application/json")
                .body(body.clone());
        }
        Ok(req)
    };

    let token = manager.access_token().await?;
    let resp = build(&token)?.send().await.context("sending the request")?;
    if resp.status() != StatusCode::UNAUTHORIZED {
        return Ok(resp);
    }

    // 401 → refresh once and retry with the rotated token.
    manager.force_refresh().await?;
    let token = manager.access_token().await?;
    let resp = build(&token)?.send().await.context("sending the request")?;
    if resp.status() == StatusCode::UNAUTHORIZED {
        return Err(AuthError::NotAuthenticated.into());
    }
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_usage_error(err: &anyhow::Error) -> bool {
        err.downcast_ref::<UsageError>().is_some()
    }

    #[test]
    fn resolve_url_prepends_a_missing_leading_slash_and_joins() {
        assert_eq!(
            resolve_url(
                "https://api.ouraring.com",
                "/v2/usercollection/personal_info"
            ),
            "https://api.ouraring.com/v2/usercollection/personal_info"
        );
        // A path without a leading slash gets exactly one added (not doubled).
        assert_eq!(
            resolve_url(
                "https://api.ouraring.com",
                "v2/usercollection/personal_info"
            ),
            "https://api.ouraring.com/v2/usercollection/personal_info"
        );
    }

    #[test]
    fn get_fields_become_query_params_not_a_body() {
        let plan = RequestPlan::build(
            "http://h",
            "/v2/x",
            "get",
            &[
                "start_date=2026-06-01".to_string(),
                "end_date=2026-06-07".to_string(),
            ],
            None,
        )
        .unwrap();
        assert_eq!(plan.method, Method::GET, "method is uppercased");
        assert_eq!(
            plan.query,
            vec![
                ("start_date".to_string(), "2026-06-01".to_string()),
                ("end_date".to_string(), "2026-06-07".to_string()),
            ]
        );
        assert!(plan.body.is_none(), "a query method must not build a body");
    }

    #[test]
    fn body_method_fields_build_a_json_object_and_leave_the_query_empty() {
        let plan = RequestPlan::build(
            "http://h",
            "/v2/x",
            "post",
            &["a=1".to_string(), "b=two".to_string()],
            None,
        )
        .unwrap();
        assert_eq!(plan.method, Method::POST);
        assert!(
            plan.query.is_empty(),
            "body-method fields are not query params"
        );
        // Values are JSON strings (gh-style -f); serde_json::Map preserves insertion order.
        assert_eq!(plan.body.as_deref(), Some(r#"{"a":"1","b":"two"}"#));
    }

    #[test]
    fn a_field_without_an_equals_is_a_usage_error() {
        let err = RequestPlan::build("http://h", "/v2/x", "get", &["broken".to_string()], None)
            .unwrap_err();
        assert!(is_usage_error(&err), "{err:#}");
        assert!(
            err.to_string().contains("expected key=value"),
            "message names the fix: {err:#}"
        );
    }

    #[test]
    fn an_unknown_method_is_a_usage_error() {
        let err = RequestPlan::build("http://h", "/v2/x", "flo p", &[], None).unwrap_err();
        assert!(is_usage_error(&err), "{err:#}");
    }

    #[test]
    fn a_stdin_body_plus_body_fields_conflict_is_a_usage_error() {
        let err = RequestPlan::build(
            "http://h",
            "/v2/x",
            "POST",
            &["a=1".to_string()],
            Some("{\"already\":\"a body\"}".to_string()),
        )
        .unwrap_err();
        assert!(is_usage_error(&err), "{err:#}");
        assert!(
            err.to_string().contains("stdin"),
            "message names the conflict: {err:#}"
        );
    }

    #[test]
    fn a_stdin_body_is_used_raw_on_a_body_method_with_no_fields() {
        let plan = RequestPlan::build(
            "http://h",
            "/v2/x",
            "PUT",
            &[],
            Some(r#"{"raw":true}"#.to_string()),
        )
        .unwrap();
        assert_eq!(plan.body.as_deref(), Some(r#"{"raw":true}"#));
    }

    #[test]
    fn a_get_can_carry_both_query_fields_and_a_stdin_body_without_conflict() {
        // A stdin body is only a conflict with -f on a BODY method (where -f is also a body).
        // On GET the fields are query params, so both coexist.
        let plan = RequestPlan::build(
            "http://h",
            "/v2/x",
            "GET",
            &["q=1".to_string()],
            Some(r#"{"raw":true}"#.to_string()),
        )
        .unwrap();
        assert_eq!(plan.query, vec![("q".to_string(), "1".to_string())]);
        assert_eq!(plan.body.as_deref(), Some(r#"{"raw":true}"#));
    }
}
