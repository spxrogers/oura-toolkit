//! Hermetic tests for `oura api` (#19), the authenticated passthrough.
//!
//! Binary-level tests spawn the REAL `oura` binary against a wiremock "Oura host" (the
//! `mock_oura` pattern from `headless_auth.rs`) with `OURA_ACCESS_TOKEN` + `OURA_API_BASE_URL`
//! pointing at it, proving the auth header crosses the process boundary, that `--paginate`
//! walks the `next_token` chain, and that a non-2xx fails with the status on stderr and an
//! empty stdout. The 401-refresh-retry needs a mockable token endpoint, which the binary
//! can't be pointed at, so it's covered in-process (like
//! `data_commands.rs::a_401_refreshes_once_and_retries_with_the_new_token`) via the
//! `test-util` token-URL override.

use std::process::Stdio;

use oura_toolkit_auth::{ClientCredentials, TokenManager, TokenStore, Tokens};
use wiremock::matchers::{body_string, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
use common::fresh_tokens;

// --- binary-level (wiremock + the real `oura` binary) ----------------------------------

/// A wiremock "Oura host" plus the runtime that owns it (kept alive for the test).
struct MockOura {
    server: MockServer,
    _rt: tokio::runtime::Runtime,
}

fn mock_oura(
    mount: impl FnOnce(&MockServer) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + '_>>,
) -> MockOura {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let server = rt.block_on(async {
        let server = MockServer::start().await;
        mount(&server).await;
        server
    });
    MockOura { server, _rt: rt }
}

fn oura_cmd(oura: &MockOura, dir: &std::path::Path) -> std::process::Command {
    let mut cmd = std::process::Command::new(assert_cmd::cargo::cargo_bin("oura"));
    cmd.env("XDG_CONFIG_HOME", dir)
        .env("HOME", dir)
        .env("LOCALAPPDATA", dir)
        .env("NO_COLOR", "1")
        .env("OURA_ACCESS_TOKEN", "env-tok-xyz")
        .env("OURA_API_BASE_URL", oura.server.uri())
        // No body is piped; a null stdin keeps read_stdin_body() from blocking on the
        // inherited test-runner stdin.
        .stdin(Stdio::null());
    cmd
}

#[test]
fn api_get_prints_the_json_body_and_carries_the_bearer() {
    // The mock only answers when the Bearer equals the env token, so a green body PROVES the
    // stored auth crossed the process boundary into the Authorization header (load-bearing).
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/personal_info"))
                .and(header("authorization", "Bearer env-tok-xyz"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "id": "u-1", "age": 34
                })))
                .expect(1)
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();

    let out = oura_cmd(&oura, dir.path())
        .args(["api", "/v2/usercollection/personal_info"])
        .output()
        .expect("run oura api");

    assert!(
        out.status.success(),
        "oura api failed (status {:?}); stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).unwrap();
    // The raw JSON response is on stdout, verbatim (not a rendered table).
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).expect("stdout is JSON");
    assert_eq!(parsed["id"], "u-1");
    assert_eq!(parsed["age"], 34);
}

#[test]
fn api_wrong_bearer_is_rejected() {
    // Negative half: a Bearer the mock doesn't recognise gets no 200 (wiremock 404s the
    // unmatched request), confirming the header matcher above is load-bearing.
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/personal_info"))
                .and(header("authorization", "Bearer the-right-one"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();
    let out = oura_cmd(&oura, dir.path())
        .args(["api", "/v2/usercollection/personal_info"])
        .output()
        .expect("run oura api");
    assert!(
        !out.status.success(),
        "a wrong Bearer must not yield a successful passthrough"
    );
}

#[test]
fn api_paginate_walks_the_next_token_chain_and_aggregates_in_order() {
    // Three chained pages: the aggregated {"data":[…]} must contain every item, in page order.
    let oura = mock_oura(|server| {
        Box::pin(async move {
            let base = || Mock::given(method("GET")).and(path("/v2/usercollection/daily_sleep"));
            base()
                .and(wiremock::matchers::query_param_is_missing("next_token"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "data": [{"day": "d1"}], "next_token": "t1"
                })))
                .expect(1)
                .mount(server)
                .await;
            base()
                .and(query_param("next_token", "t1"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "data": [{"day": "d2"}], "next_token": "t2"
                })))
                .expect(1)
                .mount(server)
                .await;
            base()
                .and(query_param("next_token", "t2"))
                .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                    "data": [{"day": "d3"}], "next_token": null
                })))
                .expect(1)
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();

    let out = oura_cmd(&oura, dir.path())
        .args(["api", "--paginate", "/v2/usercollection/daily_sleep"])
        .output()
        .expect("run oura api --paginate");
    assert!(
        out.status.success(),
        "paginate failed; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("aggregated JSON");
    let days: Vec<&str> = parsed["data"]
        .as_array()
        .expect("data array")
        .iter()
        .map(|d| d["day"].as_str().unwrap())
        .collect();
    assert_eq!(
        days,
        vec!["d1", "d2", "d3"],
        "every page's item, in page order: {stdout}"
    );
}

#[test]
fn api_non_2xx_exits_1_with_the_status_on_stderr_and_empty_stdout() {
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/nope"))
                .respond_with(ResponseTemplate::new(404).set_body_string("not found here"))
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();
    let out = oura_cmd(&oura, dir.path())
        .args(["api", "/v2/usercollection/nope"])
        .output()
        .expect("run oura api");

    // Exit 1 specifically (runtime error, docs/cli-contract.md → oura api), not the exit-4
    // auth path — a 404 is not an auth failure.
    assert_eq!(
        out.status.code(),
        Some(1),
        "a non-2xx must be a runtime error (exit 1); stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty(), "stdout must be empty on failure");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("api request to /v2/usercollection/nope failed") && stderr.contains("404"),
        "the error line names the path and HTTP status: {stderr}"
    );
    assert!(
        stderr.contains("not found here"),
        "the response body is echoed to stderr: {stderr}"
    );
}

#[test]
fn api_missing_field_equals_is_a_usage_error_exit_2() {
    let oura = mock_oura(|server| {
        Box::pin(async move {
            // Never reached — a malformed -f fails before any request.
            Mock::given(method("GET"))
                .respond_with(ResponseTemplate::new(200))
                .expect(0)
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();
    let out = oura_cmd(&oura, dir.path())
        .args(["api", "/v2/x", "-f", "nokey"])
        .output()
        .expect("run oura api");
    assert_eq!(
        out.status.code(),
        Some(2),
        "a field without `=` is a usage error (exit 2); stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty(), "stdout empty on a usage error");
}

#[test]
fn api_error_body_is_sanitized_before_it_reaches_the_terminal() {
    // Attack test (CLAUDE.md TESTING rule 5): a non-2xx body is server-controlled text bound
    // for stderr, so it must be sanitized — terminal escapes stripped, no forged extra lines.
    // Break-verify: drop the `sanitize()` call in passthrough.rs::send_and_read and the
    // escape-byte assertion fails.
    let hostile = "denied\u{1b}[2Jinjected\noura: forged: line";
    let oura = mock_oura(move |server| {
        Box::pin(async move {
            Mock::given(method("GET"))
                .and(path("/v2/x"))
                .respond_with(ResponseTemplate::new(403).set_body_string(hostile))
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();
    let out = oura_cmd(&oura, dir.path())
        .args(["api", "/v2/x"])
        .output()
        .expect("run oura api");
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains('\u{1b}'),
        "the escape byte must be stripped from the echoed error body: {stderr:?}"
    );
    // The text still conveys the message, just without the control byte.
    assert!(
        stderr.contains("denied") && stderr.contains("injected"),
        "the sanitized body is still shown: {stderr:?}"
    );
}

#[test]
fn api_pipes_a_stdin_body_with_json_content_type() {
    // The stdin raw-body path end-to-end: `read_stdin_body()` reads the piped bytes and they
    // reach the wire as the request body with `Content-Type: application/json`. The mock only
    // answers 200 when BOTH the exact body and the content-type match, so `.success()` proves
    // the body crossed the process boundary (a regression 404s → exit 1 → the assert fails).
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("POST"))
                .and(path("/v2/x"))
                .and(header("content-type", "application/json"))
                .and(body_string(r#"{"raw":"from-stdin"}"#))
                .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"ok":true}"#))
                .expect(1)
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = assert_cmd::Command::cargo_bin("oura").unwrap();
    cmd.env("XDG_CONFIG_HOME", dir.path())
        .env("HOME", dir.path())
        .env("LOCALAPPDATA", dir.path())
        .env("NO_COLOR", "1")
        .env("OURA_ACCESS_TOKEN", "env-tok-xyz")
        .env("OURA_API_BASE_URL", oura.server.uri())
        .args(["api", "-X", "POST", "/v2/x"])
        .write_stdin(r#"{"raw":"from-stdin"}"#)
        .assert()
        .success()
        .stdout(predicates::prelude::predicate::str::contains("ok"));
}

#[test]
fn api_paginate_with_a_non_get_method_is_a_usage_error_exit_2() {
    // "GET only" (main.rs help + cli-contract) is now enforced, not just documented: the
    // usage error fires before any request (expect(0)).
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("POST"))
                .respond_with(ResponseTemplate::new(200))
                .expect(0)
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();
    let out = oura_cmd(&oura, dir.path())
        .args(["api", "-X", "POST", "--paginate", "/v2/x"])
        .output()
        .expect("run oura api");
    assert_eq!(
        out.status.code(),
        Some(2),
        "--paginate with a non-GET method must be a usage error (exit 2); stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty(), "stdout empty on a usage error");
}

// --- in-process (401 → refresh → retry) -------------------------------------------------

fn seeded_manager(server: &MockServer, dir: &tempfile::TempDir, tokens: Tokens) -> TokenManager {
    let store = TokenStore::with_dir(dir.path());
    store.save_tokens(&tokens).unwrap();
    let credentials = ClientCredentials {
        client_id: "cid".into(),
        client_secret: "sec".into(),
    };
    store.save_credentials(&credentials).unwrap();
    let mut manager = TokenManager::from_parts(store, Some(credentials), Some(tokens));
    manager.override_token_url(format!("{}/oauth/token", server.uri()));
    manager
}

/// A 401 forces exactly one refresh (against the mocked token endpoint) and one retry with
/// the rotated Bearer, which then succeeds — the raw body comes back. expect() counts on all
/// three mocks pin "exactly once", mirroring the data-command 401 test. Store-backed (not
/// OURA_ACCESS_TOKEN, which can't refresh), so the token endpoint is mockable; hence
/// in-process rather than through the binary.
#[tokio::test]
async fn api_401_refreshes_once_and_retries_with_the_new_token() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    // First call (old token) → 401. expect(1): the retry must NOT reuse the old token.
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/personal_info"))
        .and(header("authorization", "Bearer at-old"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&server)
        .await;
    // Token endpoint: rotates to at-new. expect(1): exactly one refresh, no storm.
    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "at-new",
            "refresh_token": "rt-2",
            "expires_in": 3600
        })))
        .expect(1)
        .mount(&server)
        .await;
    // Retry with the rotated token → the data body.
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/personal_info"))
        .and(header("authorization", "Bearer at-new"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})))
        .expect(1)
        .mount(&server)
        .await;

    let manager = seeded_manager(&server, &dir, fresh_tokens("at-old"));
    let out = oura_toolkit_cli::passthrough::run(
        &manager,
        &server.uri(),
        "/v2/usercollection/personal_info",
        "GET",
        &[],
        None,
        false,
    )
    .await
    .unwrap();
    let parsed: serde_json::Value = serde_json::from_str(out.trim()).unwrap();
    assert_eq!(
        parsed["ok"], true,
        "the retried request's body is returned: {out}"
    );
}

/// A 401 that persists after the refresh surfaces the typed NotAuthenticated (exit 4 + login
/// hint via the classifier). expect(2) on the data mock turns a retry storm into a failure.
#[tokio::test]
async fn api_persistent_401_surfaces_not_authenticated() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/personal_info"))
        .respond_with(ResponseTemplate::new(401))
        .expect(2) // original + exactly one retry — never a third attempt
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "at-still-bad",
            "refresh_token": "rt-2",
            "expires_in": 3600
        })))
        .expect(1)
        .mount(&server)
        .await;

    let manager = seeded_manager(&server, &dir, fresh_tokens("at-old"));
    let err = oura_toolkit_cli::passthrough::run(
        &manager,
        &server.uri(),
        "/v2/usercollection/personal_info",
        "GET",
        &[],
        None,
        false,
    )
    .await
    .unwrap_err();
    assert!(
        matches!(
            err.chain()
                .find_map(|c| c.downcast_ref::<oura_toolkit_auth::AuthError>()),
            Some(oura_toolkit_auth::AuthError::NotAuthenticated)
        ),
        "must surface the typed auth error for exit-4 classification: {err:?}"
    );
}

/// A POST with `-f` fields sends a JSON body (in-process, so the request body is asserted on
/// the wire) with the stored Bearer.
#[tokio::test]
async fn api_post_fields_send_a_json_body() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("POST"))
        .and(path("/v2/webhook/subscription"))
        .and(header("content-type", "application/json"))
        .and(wiremock::matchers::body_json(serde_json::json!({
            "event_type": "create",
            "data_type": "tag"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": "sub-1"})))
        .expect(1)
        .mount(&server)
        .await;

    let manager = seeded_manager(&server, &dir, fresh_tokens("at-1"));
    let out = oura_toolkit_cli::passthrough::run(
        &manager,
        &server.uri(),
        "/v2/webhook/subscription",
        "POST",
        &["event_type=create".to_string(), "data_type=tag".to_string()],
        None,
        false,
    )
    .await
    .unwrap();
    assert!(
        out.contains("sub-1"),
        "the response body is returned: {out}"
    );
}
