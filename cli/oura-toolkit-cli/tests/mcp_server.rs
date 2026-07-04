//! Hermetic integration tests for the `oura mcp` server (#10): a real rmcp client talks
//! to the real `OuraMcp` handler over an in-memory duplex transport; the data plane hits
//! wiremock. No network, no credentials, no stdio.
//!
//! These enforce the CLAUDE.md → MCP contract: initialize succeeds WITHOUT tokens; the
//! first tool call without tokens is a TOOL-LEVEL structured error naming
//! `oura auth login` (never a prompt, never a protocol error); tool calls auto-paginate
//! and return the generated models as JSON; a 401 triggers exactly one silent refresh
//! whose ROTATED token is persisted to the store; malformed dates are protocol
//! `invalid_params`.

use rmcp::model::{CallToolRequestParams, CallToolResult};
use rmcp::ServiceExt;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use oura_toolkit_auth::{ClientCredentials, TokenManager, TokenStore, Tokens};

mod common;
use common::{fresh_tokens, page, sleep_doc};
use oura_toolkit_cli::mcp::OuraMcp;

/// A store WITH credentials+tokens (authenticated) or empty (fresh machine).
fn manager(dir: &tempfile::TempDir, tokens: Option<Tokens>) -> TokenManager {
    let store = TokenStore::with_dir(dir.path());
    let credentials = ClientCredentials {
        client_id: "cid".into(),
        client_secret: "sec".into(),
    };
    if let Some(t) = &tokens {
        store.save_credentials(&credentials).unwrap();
        store.save_tokens(t).unwrap();
    }
    TokenManager::from_parts(store, tokens.is_some().then_some(credentials), tokens)
}

/// Serve `OuraMcp` over an in-memory duplex and hand back a connected client. The
/// health store lives in its own tempdir per server (kept alive by the moved handle's
/// dir; the tests that exercise the store pass their own via [`connect_with_store`]).
async fn connect(
    manager: TokenManager,
    base_url: String,
) -> rmcp::service::RunningService<rmcp::RoleClient, ()> {
    let store_dir = tempfile::tempdir().expect("store tempdir");
    let store = oura_toolkit_health::HealthStore::with_dir(store_dir.path().join("health"));
    let client = connect_with_store(manager, base_url, store).await;
    // The server task owns the HealthStore (a path), but the TempDir handle would delete
    // the directory on drop — keep it alive for the test process instead.
    std::mem::forget(store_dir);
    client
}

/// [`connect`] with an explicit health store (local-store tool tests).
async fn connect_with_store(
    manager: TokenManager,
    base_url: String,
    store: oura_toolkit_health::HealthStore,
) -> rmcp::service::RunningService<rmcp::RoleClient, ()> {
    let (server_io, client_io) = tokio::io::duplex(1 << 16);
    tokio::spawn(async move {
        if let Ok(running) = OuraMcp::new(manager, base_url, store)
            .serve(server_io)
            .await
        {
            let _ = running.waiting().await;
        }
    });
    ().serve(client_io).await.expect("client connects")
}

fn text_of(result: &CallToolResult) -> &str {
    &result.content[0].as_text().expect("text content").text
}

fn sleep_args(start: &str, end: &str) -> serde_json::Map<String, serde_json::Value> {
    let mut args = serde_json::Map::new();
    args.insert("start".into(), start.into());
    args.insert("end".into(), end.into());
    args
}

/// Initialize succeeds on a machine with NO stored tokens, and the 14 curated tools are
/// listed — the 8 Oura ones carrying the build-time spec-derived description (curated
/// lead + the spec's field inventory), the 6 local-store ones their hand-curated
/// descriptions — with out-of-band auth named in the server instructions.
#[tokio::test]
async fn initialize_succeeds_without_tokens_and_lists_the_14_described_tools() {
    let dir = tempfile::tempdir().unwrap();
    let client = connect(manager(&dir, None), "http://unused.invalid".into()).await;

    let info = client.peer_info().expect("initialize handshake completed");
    assert_eq!(info.server_info.name, "oura-toolkit");
    assert!(
        info.instructions
            .as_deref()
            .unwrap_or_default()
            .contains("oura auth login"),
        "instructions must name the out-of-band auth flow"
    );

    let mut tools = client.list_all_tools().await.unwrap();
    tools.sort_by(|a, b| a.name.cmp(&b.name));
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert_eq!(
        names,
        [
            "find_analog_weeks",
            "get_capacity",
            "get_daily_activity",
            "get_daily_readiness",
            "get_daily_sleep",
            "get_daily_stress",
            "get_day_context",
            "get_habits",
            "get_heart_rate",
            "get_personal_info",
            "get_sessions",
            "get_upcoming_load",
            "get_workouts",
            "log_habit",
        ],
        "exactly the curated tool surface, nothing else"
    );
    let local = [
        "find_analog_weeks",
        "get_capacity",
        "get_day_context",
        "get_upcoming_load",
        "get_habits",
        "log_habit",
    ];
    for tool in &tools {
        let description = tool.description.as_deref().unwrap_or_default();
        if local.contains(&tool.name.as_ref()) {
            assert!(
                description.contains("local") && description.contains("store"),
                "{}: local tools must name the local store as their source: {description:?}",
                tool.name
            );
            continue;
        }
        assert!(
            description.contains("Oura API operation:"),
            "{}: spec-derived section missing: {description:?}",
            tool.name
        );
        assert!(
            description.contains("Documents contain:"),
            "{}: spec field inventory missing: {description:?}",
            tool.name
        );
    }
    // Spot-pin one spec-derived field so a codegen regression can't ship empty inventories.
    let sleep = tools.iter().find(|t| t.name == "get_daily_sleep").unwrap();
    assert!(
        sleep
            .description
            .as_deref()
            .unwrap()
            .contains("contributors"),
        "sleep inventory must list the contributors field"
    );

    client.cancel().await.unwrap();
}

/// CLAUDE.md → MCP: absent tokens do NOT fail the handshake, do NOT prompt, and the
/// first tool call returns a TOOL-LEVEL error (the model sees it) naming
/// `oura auth login`.
#[tokio::test]
async fn first_tool_call_without_tokens_is_a_structured_auth_error() {
    let dir = tempfile::tempdir().unwrap();
    let client = connect(manager(&dir, None), "http://unused.invalid".into()).await;

    let result = client
        .call_tool(CallToolRequestParams::new("get_daily_sleep"))
        .await
        .expect("must be a tool-level error, not a protocol error");
    assert_eq!(result.is_error, Some(true));
    let text = text_of(&result);
    assert!(
        text.contains("oura auth login"),
        "auth remediation must be in the model-visible text: {text}"
    );

    client.cancel().await.unwrap();
}

/// Tool calls fetch through the real data plane: literal date query params on the wire,
/// auto-pagination across 3 pages, and the GENERATED models serialized as JSON content.
#[tokio::test]
async fn tool_calls_auto_paginate_and_return_generated_model_json() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    let base = || {
        Mock::given(method("GET"))
            .and(path("/v2/usercollection/daily_sleep"))
            .and(query_param("start_date", "2026-06-26"))
            .and(query_param("end_date", "2026-07-02"))
    };
    base()
        .and(wiremock::matchers::query_param_is_missing("next_token"))
        .respond_with(page(vec![sleep_doc("2026-06-26", 80)], Some("t1")))
        .expect(1)
        .mount(&server)
        .await;
    base()
        .and(query_param("next_token", "t1"))
        .respond_with(page(vec![sleep_doc("2026-06-27", 81)], Some("t2")))
        .expect(1)
        .mount(&server)
        .await;
    base()
        .and(query_param("next_token", "t2"))
        .respond_with(page(vec![sleep_doc("2026-06-28", 82)], None))
        .expect(1)
        .mount(&server)
        .await;

    let client = connect(manager(&dir, Some(fresh_tokens("at-1"))), server.uri()).await;
    let result = client
        .call_tool(
            CallToolRequestParams::new("get_daily_sleep")
                .with_arguments(sleep_args("2026-06-26", "2026-07-02")),
        )
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false));

    let docs: serde_json::Value = serde_json::from_str(text_of(&result)).unwrap();
    let days: Vec<&str> = docs
        .as_array()
        .expect("JSON array of documents")
        .iter()
        .map(|d| d["day"].as_str().unwrap())
        .collect();
    assert_eq!(
        days,
        ["2026-06-26", "2026-06-27", "2026-06-28"],
        "all three pages aggregated in order"
    );
    assert_eq!(
        docs[0]["score"], 80,
        "generated-model fields survive to JSON"
    );

    client.cancel().await.unwrap();
}

/// A 401 mid-session triggers exactly ONE silent refresh; the retry carries the new
/// token; and the ROTATED refresh token is persisted to the store (Oura invalidates the
/// old one — losing it would strand every later process).
#[tokio::test]
async fn a_401_silently_refreshes_once_and_persists_the_rotated_token() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .and(wiremock::matchers::header("authorization", "Bearer at-old"))
        .respond_with(ResponseTemplate::new(401))
        .expect(1)
        .mount(&server)
        .await;
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
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .and(wiremock::matchers::header("authorization", "Bearer at-new"))
        .respond_with(page(vec![sleep_doc("2026-07-01", 88)], None))
        .expect(1)
        .mount(&server)
        .await;

    let mut mgr = manager(&dir, Some(fresh_tokens("at-old")));
    mgr.override_token_url(format!("{}/oauth/token", server.uri()));
    let client = connect(mgr, server.uri()).await;

    let result = client
        .call_tool(
            CallToolRequestParams::new("get_daily_sleep")
                .with_arguments(sleep_args("2026-07-01", "2026-07-01")),
        )
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false), "{:?}", text_of(&result));

    // The rotation must have hit DISK, not just memory: reload the store cold.
    let reloaded = TokenStore::with_dir(dir.path()).load_tokens().unwrap();
    assert_eq!(
        reloaded.expect("tokens present").refresh_token,
        "rt-2",
        "rotated refresh token persisted"
    );

    client.cancel().await.unwrap();
}

/// Malformed arguments are the CALLER's fault: protocol `invalid_params`, mirroring the
/// CLI's exit-2 classification — never a tool-level "the tool failed" result.
#[tokio::test]
async fn malformed_dates_are_protocol_invalid_params() {
    let dir = tempfile::tempdir().unwrap();
    let client = connect(manager(&dir, None), "http://unused.invalid".into()).await;

    let mut args = serde_json::Map::new();
    args.insert("start".into(), "not-a-date".into());
    let err = client
        .call_tool(CallToolRequestParams::new("get_daily_sleep").with_arguments(args))
        .await
        .expect_err("protocol error expected");
    let message = err.to_string();
    assert!(
        message.contains("today, yesterday, or YYYY-MM-DD"),
        "actionable message reaches the caller: {message}"
    );

    client.cancel().await.unwrap();
}

/// Hostile tool arguments (rule 5): control bytes in a date value are echoed back in the
/// invalid_params message — they must arrive NEUTRALIZED (MCP clients render this text).
#[tokio::test]
async fn hostile_date_arguments_are_sanitized_in_the_error_echo() {
    let dir = tempfile::tempdir().unwrap();
    let client = connect(manager(&dir, None), "http://unused.invalid".into()).await;

    let mut args = serde_json::Map::new();
    args.insert("start".into(), "199\u{1b}[2J6-01-01\ttab".into());
    let err = client
        .call_tool(CallToolRequestParams::new("get_daily_sleep").with_arguments(args))
        .await
        .expect_err("protocol error expected");
    let message = err.to_string();
    assert!(
        !message.contains('\u{1b}') && !message.contains('\t'),
        "control bytes must not survive into the rendered error: {message:?}"
    );
    assert!(
        message.contains("[2J"),
        "the echo itself survives, defanged: {message:?}"
    );

    client.cancel().await.unwrap();
}

/// MCP clients parallelize tool calls; the handler shares one TokenManager across them.
/// Two concurrent calls to different tools must both complete correctly.
#[tokio::test]
async fn concurrent_tool_calls_share_the_manager_safely() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .respond_with(page(vec![sleep_doc("2026-07-01", 88)], None))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/personal_info"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "u-1", "age": 34, "biological_sex": "male",
            "height": 1.8, "weight": 76.5, "email": "user@example.com"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = connect(manager(&dir, Some(fresh_tokens("at-1"))), server.uri()).await;
    let sleep_call = client.call_tool(
        CallToolRequestParams::new("get_daily_sleep")
            .with_arguments(sleep_args("2026-07-01", "2026-07-01")),
    );
    let info_call = client.call_tool(CallToolRequestParams::new("get_personal_info"));
    let (sleep_result, info_result) = tokio::join!(sleep_call, info_call);

    let sleep_result = sleep_result.unwrap();
    assert_eq!(sleep_result.is_error, Some(false));
    assert!(text_of(&sleep_result).contains("2026-07-01"));
    let info_result = info_result.unwrap();
    assert_eq!(info_result.is_error, Some(false));
    assert!(text_of(&info_result).contains("user@example.com"));

    client.cancel().await.unwrap();
}

/// Local-store tools serve WITHOUT Oura tokens (their data plane is the day-grain
/// store): `get_day_context` returns exactly the imported records as JSON.
#[tokio::test]
async fn get_day_context_reads_the_local_store_without_tokens() {
    use oura_toolkit_health::{CalendarDay, HealthStore};

    let dir = tempfile::tempdir().unwrap();
    let store = HealthStore::with_dir(dir.path().join("health"));
    store
        .upsert(std::collections::BTreeMap::from([(
            "2026-07-01".parse::<chrono::NaiveDate>().unwrap(),
            CalendarDay {
                meeting_hours: 4.5,
                event_count: 6,
                evening_event_count: 1,
                ..Default::default()
            },
        )]))
        .unwrap();

    let client =
        connect_with_store(manager(&dir, None), "http://unused.invalid".into(), store).await;
    let result = client
        .call_tool(
            CallToolRequestParams::new("get_day_context")
                .with_arguments(sleep_args("2026-07-01", "2026-07-01")),
        )
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false), "no auth needed: {result:?}");
    let json: serde_json::Value = serde_json::from_str(text_of(&result)).unwrap();
    assert_eq!(json["2026-07-01"]["calendar"]["meeting_hours"], 4.5);
    assert_eq!(json["2026-07-01"]["calendar"]["event_count"], 6);

    client.cancel().await.unwrap();
}

/// A thin-history store makes `get_capacity` a TOOL-LEVEL error whose message carries
/// the import remediation — the local-store mirror of the structured auth error.
#[tokio::test]
async fn get_capacity_with_thin_history_is_a_structured_tool_error() {
    let dir = tempfile::tempdir().unwrap();
    let client = connect(manager(&dir, None), "http://unused.invalid".into()).await;

    let result = client
        .call_tool(CallToolRequestParams::new("get_capacity"))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(true), "thin history is a tool error");
    let message = text_of(&result);
    assert!(
        message.contains("not enough history"),
        "names the refusal: {message}"
    );
    assert!(
        message.contains("oura import"),
        "carries the remediation: {message}"
    );

    client.cancel().await.unwrap();
}

/// The habit write→read loop over real MCP: `log_habit` canonicalizes and persists,
/// re-logging is an idempotent no-op, `get_habits` reports the long-grain rates, and
/// `undo` reverses the write. A hostile habit name is `invalid_params`, not a write.
#[tokio::test]
async fn log_habit_and_get_habits_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let store = oura_toolkit_health::HealthStore::with_dir(dir.path().join("health"));
    let client = connect_with_store(
        manager(&dir, None),
        "http://unused.invalid".into(),
        store.clone(),
    )
    .await;

    let log_args = |name: &str, undo: bool| {
        let mut args = serde_json::Map::new();
        args.insert("name".into(), name.into());
        args.insert("date".into(), "2026-07-01".into());
        if undo {
            args.insert("undo".into(), true.into());
        }
        args
    };

    // Write: the name canonicalizes and the change is reported.
    let result = client
        .call_tool(
            CallToolRequestParams::new("log_habit")
                .with_arguments(log_args("Strength Training", false)),
        )
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false), "{result:?}");
    let json: serde_json::Value = serde_json::from_str(text_of(&result)).unwrap();
    assert_eq!(json["habit"], "strength-training");
    assert_eq!(json["changed"], true);

    // Idempotent re-log.
    let result = client
        .call_tool(
            CallToolRequestParams::new("log_habit")
                .with_arguments(log_args("strength-training", false)),
        )
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_str(text_of(&result)).unwrap();
    assert_eq!(json["changed"], false, "re-log is a no-op");

    // Read: the rates view sees the log.
    let result = client
        .call_tool(CallToolRequestParams::new("get_habits"))
        .await
        .unwrap();
    assert_eq!(result.is_error, Some(false));
    let json: serde_json::Value = serde_json::from_str(text_of(&result)).unwrap();
    assert_eq!(json[0]["name"], "strength-training");
    assert_eq!(json[0]["total_days"], 1);

    // Undo reverses it; the store really is empty afterwards.
    let result = client
        .call_tool(
            CallToolRequestParams::new("log_habit")
                .with_arguments(log_args("strength training", true)),
        )
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_str(text_of(&result)).unwrap();
    assert_eq!(json["changed"], true);
    assert!(store.load().unwrap().is_empty(), "undo left no husks");

    // A hostile name is the caller's arguments being wrong: protocol invalid_params.
    let err = client
        .call_tool(CallToolRequestParams::new("log_habit").with_arguments(log_args("!!!", false)))
        .await
        .expect_err("invalid habit name must be a protocol error");
    assert!(
        err.to_string().contains("invalid habit name"),
        "names the problem: {err}"
    );

    client.cancel().await.unwrap();
}
