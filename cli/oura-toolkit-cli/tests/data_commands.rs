//! Hermetic integration tests for the data commands (#9): the real command functions,
//! the real generated client, a wiremock Oura — no network, no credentials.
//!
//! These enforce the #9 addendum's claims: pagination auto-follows `next_token` across
//! ≥3 pages and terminates; the generated client's date newtypes serialize into the
//! LITERAL query strings the server must receive; a 401 triggers exactly one
//! refresh-then-retry; rendered output is pinned as goldens through the real pipeline.

use oura_toolkit_auth::{ClientCredentials, TokenManager, TokenStore, Tokens};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test doubles are built from the pieces the real binary uses; only the base URL (and,
// for the 401 test, the token endpoint) point at wiremock.
use oura_toolkit_cli::api::DateRange;
use oura_toolkit_cli::commands::{self, Ctx};
use oura_toolkit_cli::output::{Format, RenderOptions, Style};

fn plain() -> RenderOptions {
    RenderOptions {
        format: Format::Plain,
        style: Style::new(false),
    }
}

/// Fixed range so query-param assertions are deterministic (no local clock).
fn range() -> DateRange {
    let today = chrono::NaiveDate::parse_from_str("2026-07-02", "%Y-%m-%d").unwrap();
    DateRange::resolve(Some("2026-06-26"), Some("2026-07-02"), today).unwrap()
}

fn fresh_tokens(access: &str) -> Tokens {
    Tokens {
        access_token: access.into(),
        refresh_token: "rt-1".into(),
        expires_at: 4_102_444_800, // 2100 — never proactively refreshed
        scope: None,
        token_type: None,
    }
}

fn ctx(server: &MockServer, dir: &tempfile::TempDir, tokens: Tokens) -> Ctx {
    let store = TokenStore::with_dir(dir.path());
    store.save_tokens(&tokens).unwrap();
    let credentials = ClientCredentials {
        client_id: "cid".into(),
        client_secret: "sec".into(),
    };
    store.save_credentials(&credentials).unwrap();
    Ctx {
        manager: TokenManager::from_parts(store, Some(credentials), Some(tokens)),
        base_url: server.uri(),
        render: plain(),
    }
}

fn sleep_doc(day: &str, score: i64) -> serde_json::Value {
    serde_json::json!({
        "id": format!("doc-{day}"),
        "day": day,
        "score": score,
        "timestamp": format!("{day}T00:00:00+00:00"),
        "contributors": {
            "deep_sleep": 70, "efficiency": 90, "latency": 60,
            "rem_sleep": 80, "restfulness": 55, "timing": 40, "total_sleep": 85
        }
    })
}

fn page(data: Vec<serde_json::Value>, next: Option<&str>) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "data": data,
        "next_token": next,
    }))
}

/// Pagination follows `next_token` across three pages, sends the LITERAL date query params
/// each time, aggregates in order, and stops at the null token.
#[tokio::test]
async fn sleep_paginates_three_pages_with_exact_query_params() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    // MockBuilder isn't Clone; rebuild the shared matcher stack per page.
    let base = || {
        Mock::given(method("GET"))
            .and(path("/v2/usercollection/daily_sleep"))
            .and(query_param("start_date", "2026-06-26"))
            .and(query_param("end_date", "2026-07-02"))
    };

    // Page 1: no next_token param.
    base()
        .and(wiremock::matchers::query_param_is_missing("next_token"))
        .respond_with(page(vec![sleep_doc("2026-06-26", 80)], Some("t+1/a==")))
        .expect(1)
        .mount(&server)
        .await;
    base()
        .and(query_param("next_token", "t+1/a=="))
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

    let out = commands::sleep(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    // Golden plain output: three rows, aggregated in page order, tab-separated.
    assert_eq!(
        out,
        "2026-06-26\t80\t70\t80\t90\n2026-06-27\t81\t70\t80\t90\n2026-06-28\t82\t70\t80\t90\n"
    );
}

/// An empty result for the range is SUCCESS with empty output (documented in
/// docs/cli-contract.md → Dates), not an error.
#[tokio::test]
async fn an_empty_range_is_success_with_empty_output() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .respond_with(page(vec![], None))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::sleep(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    assert_eq!(out, "");
}

/// A 401 triggers exactly one refresh (against the token endpoint) and one retry, which
/// then succeeds — enforced by expect() counts on all three mocks.
#[tokio::test]
async fn a_401_refreshes_once_and_retries_with_the_new_token() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    // First data call (old token) → 401. expect(1): the retry must NOT reuse the old token.
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .and(wiremock::matchers::header("authorization", "Bearer at-old"))
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
    // Retry with the rotated token → data.
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .and(wiremock::matchers::header("authorization", "Bearer at-new"))
        .respond_with(page(vec![sleep_doc("2026-07-01", 88)], None))
        .expect(1)
        .mount(&server)
        .await;

    let mut ctx = ctx(&server, &dir, fresh_tokens("at-old"));
    ctx.manager
        .override_token_url(format!("{}/oauth/token", server.uri()));

    let out = commands::sleep(&ctx, range()).await.unwrap();
    assert_eq!(out, "2026-07-01\t88\t70\t80\t90\n");
}

/// A 401 that persists after the refresh is a dead session: the typed NotAuthenticated
/// surfaces (the binary maps it to exit 4 + login hint — pinned in cli_contract.rs).
#[tokio::test]
async fn a_persistent_401_surfaces_not_authenticated() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
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

    let mut ctx = ctx(&server, &dir, fresh_tokens("at-old"));
    ctx.manager
        .override_token_url(format!("{}/oauth/token", server.uri()));

    let err = commands::sleep(&ctx, range()).await.unwrap_err();
    let auth = err
        .chain()
        .find_map(|c| c.downcast_ref::<oura_toolkit_auth::AuthError>());
    assert!(
        matches!(auth, Some(oura_toolkit_auth::AuthError::NotAuthenticated)),
        "must surface the typed auth error for exit-4 classification: {err:?}"
    );
}

/// Non-auth API failures surface as single-line runtime errors (exit 1 class) without
/// any refresh attempt.
#[tokio::test]
async fn a_500_is_a_runtime_error_with_no_refresh() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1) // no retry for non-401s (transient-retry policy is future work, #28)
        .mount(&server)
        .await;

    let err = commands::sleep(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap_err();
    assert!(
        err.to_string().contains("fetching daily sleep"),
        "action context per the error contract: {err}"
    );
    assert!(
        format!("{err:#}").contains("HTTP 500"),
        "status surfaces in the chain: {err:#}"
    );
}

/// Field-mapping goldens for the remaining commands: one page each, pinning which model
/// fields land in which columns and that absent optionals render as `-`.
#[tokio::test]
async fn readiness_maps_score_and_temperature_columns() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_readiness"))
        .respond_with(page(
            vec![
                serde_json::json!({
                    "id": "r-1", "day": "2026-06-29", "score": 90,
                    "temperature_deviation": -0.2, "contributors": {},
                    "timestamp": "2026-06-29T00:00:00+00:00"
                }),
                serde_json::json!({
                    "id": "r-2", "day": "2026-06-30", "contributors": {},
                    "timestamp": "2026-06-30T00:00:00+00:00"
                }),
            ],
            None,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::readiness(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    assert_eq!(out, "2026-06-29\t90\t-0.2\n2026-06-30\t-\t-\n");
}

#[tokio::test]
async fn activity_maps_steps_and_calorie_columns() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_activity"))
        .respond_with(page(
            vec![serde_json::json!({
                "id": "a-1", "day": "2026-06-30", "score": 85,
                "steps": 9000, "active_calories": 450,
                "average_met_minutes": 1.4, "contributors": {},
                "equivalent_walking_distance": 6000,
                "high_activity_met_minutes": 30, "high_activity_time": 600,
                "inactivity_alerts": 0,
                "low_activity_met_minutes": 100, "low_activity_time": 3000,
                "medium_activity_met_minutes": 60, "medium_activity_time": 1200,
                "met": {"interval": 60.0, "items": [1.2, 1.3],
                        "timestamp": "2026-06-30T04:00:00+00:00"},
                "meters_to_target": 1000, "non_wear_time": 0, "resting_time": 20000,
                "sedentary_met_minutes": 300, "sedentary_time": 30000,
                "target_calories": 500, "target_meters": 9000,
                "timestamp": "2026-06-30T00:00:00+00:00", "total_calories": 2500
            })],
            None,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::activity(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    assert_eq!(out, "2026-06-30\t85\t9000\t450\n");
}

#[tokio::test]
async fn stress_maps_summary_and_duration_columns() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_stress"))
        .respond_with(page(
            vec![
                serde_json::json!({
                    "id": "s-1", "day": "2026-06-28", "day_summary": "normal",
                    "stress_high": 3600, "recovery_high": 7200
                }),
                serde_json::json!({"id": "s-2", "day": "2026-06-29"}),
            ],
            None,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::stress(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    assert_eq!(out, "2026-06-28\tnormal\t3600\t7200\n2026-06-29\t-\t-\t-\n");
}

#[tokio::test]
async fn sessions_map_type_and_datetime_columns() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/session"))
        .respond_with(page(
            vec![serde_json::json!({
                "id": "se-1", "day": "2026-06-27", "type": "meditation",
                "start_datetime": "2026-06-27T08:00:00+03:00",
                "end_datetime": "2026-06-27T08:20:00+03:00"
            })],
            None,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::sessions(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    assert_eq!(
        out,
        "2026-06-27\tmeditation\t2026-06-27T08:00:00+03:00\t2026-06-27T08:20:00+03:00\n"
    );
}

#[tokio::test]
async fn workouts_map_activity_and_intensity_columns() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/workout"))
        .respond_with(page(
            vec![
                serde_json::json!({
                    "id": "w-1", "day": "2026-06-26", "activity": "running",
                    "intensity": "hard", "calories": 320.5, "source": "manual",
                    "start_datetime": "2026-06-26T07:00:00+00:00",
                    "end_datetime": "2026-06-26T07:45:00+00:00"
                }),
                serde_json::json!({
                    "id": "w-2", "day": "2026-06-27", "activity": "cycling",
                    "intensity": "easy", "source": "autodetected",
                    "start_datetime": "2026-06-27T07:00:00+00:00",
                    "end_datetime": "2026-06-27T07:45:00+00:00"
                }),
            ],
            None,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::workouts(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    assert_eq!(
        out,
        "2026-06-26\trunning\thard\t320.5\n2026-06-27\tcycling\teasy\t-\n"
    );
}

/// Matches iff the request carries `start_datetime`/`end_datetime` equal to the UTC
/// bounds resolved from [`range`]. The conversion MATH (incl. DST gaps/folds) is pinned
/// by `utc_bounds_in` unit tests in `api.rs`; this pins that the resolved bounds actually
/// go on the wire — an implementation sending `None` (whole history) must 404 here.
struct HeartrateBoundsSent;

impl wiremock::Match for HeartrateBoundsSent {
    fn matches(&self, request: &wiremock::Request) -> bool {
        let (want_start, want_end) = range().as_utc_bounds();
        let param = |key: &str| {
            request
                .url
                .query_pairs()
                .find(|(k, _)| k == key)
                .and_then(|(_, v)| chrono::DateTime::parse_from_rfc3339(&v).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc))
        };
        param("start_datetime") == Some(want_start) && param("end_datetime") == Some(want_end)
    }
}

/// The heartrate command unwraps the anyOf time-series envelope and sends UTC datetime
/// bounds derived from the local-date range (docs/cli-contract.md → Dates).
#[tokio::test]
async fn heartrate_unwraps_the_timeseries_envelope() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/heartrate"))
        .and(HeartrateBoundsSent)
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {"bpm": 62, "source": "awake", "timestamp": "2026-07-01T10:00:00+00:00",
                 "timestamp_unix": 1782381600}
            ],
            "next_token": null
        })))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::heartrate(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    assert_eq!(out, "2026-07-01T10:00:00+00:00\t62\tawake\n");
}

/// personal-info renders the vertical record layout (and JSON from the data model).
#[tokio::test]
async fn personal_info_renders_a_record() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/personal_info"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "u-1", "age": 34, "biological_sex": "male",
            "height": 1.8, "weight": 76.5, "email": "user@example.com"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::personal_info(&ctx(&server, &dir, fresh_tokens("at-1")))
        .await
        .unwrap();
    assert_eq!(
        out,
        "Age\t34\nBiological sex\tmale\nHeight (m)\t1.8\nWeight (kg)\t76.5\nEmail\tuser@example.com\n"
    );
}

/// LIST-shaped `--json` (the `render_result` path every windowed command uses) is the
/// Vec of GENERATED models serialized — never re-encoded table cells. Exact golden
/// through a real command; the record-shaped sibling is pinned below.
#[tokio::test]
async fn workouts_json_is_the_generated_model_list() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/workout"))
        .respond_with(page(
            vec![serde_json::json!({
                "id": "w-1", "day": "2026-06-26", "activity": "running",
                "intensity": "hard", "calories": 320.5, "source": "manual",
                "start_datetime": "2026-06-26T07:00:00+00:00",
                "end_datetime": "2026-06-26T07:45:00+00:00"
            })],
            None,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let mut ctx = ctx(&server, &dir, fresh_tokens("at-1"));
    ctx.render = RenderOptions {
        format: Format::Json,
        style: Style::new(false),
    };
    let out = commands::workouts(&ctx, range()).await.unwrap();
    assert_eq!(
        out,
        r#"[
  {
    "activity": "running",
    "calories": 320.5,
    "day": "2026-06-26",
    "end_datetime": "2026-06-26T07:45:00+00:00",
    "id": "w-1",
    "intensity": "hard",
    "source": "manual",
    "start_datetime": "2026-06-26T07:00:00+00:00"
  }
]
"#
    );
}

/// `--json` on a real command is the GENERATED model serialized — pinned as an exact,
/// newline-terminated golden so the scripting contract (docs/cli-contract.md → Output
/// formats) covers the real serde path, not just a synthetic struct.
#[tokio::test]
async fn personal_info_json_is_the_generated_model_newline_terminated() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/personal_info"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "u-1", "age": 34, "biological_sex": "male",
            "height": 1.8, "weight": 76.5, "email": "user@example.com"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let mut ctx = ctx(&server, &dir, fresh_tokens("at-1"));
    ctx.render = RenderOptions {
        format: Format::Json,
        style: Style::new(false),
    };
    let out = commands::personal_info(&ctx).await.unwrap();
    assert_eq!(
        out,
        r#"{
  "age": 34,
  "biological_sex": "male",
  "email": "user@example.com",
  "height": 1.8,
  "id": "u-1",
  "weight": 76.5
}
"#
    );
}

/// Attack test through the REAL command pipeline (release-gate rule 5): hostile bytes in
/// a server-controlled OPEN string field — terminal escapes, tabs (TSV forgery), newlines
/// (row forgery) — are neutralized to spaces in the rendered output. `workout.activity`
/// is the open `String` field; closed enums (e.g. stress `day_summary`) already reject
/// unknown values at deserialization.
#[tokio::test]
async fn hostile_field_bytes_cannot_forge_output_structure() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();

    Mock::given(method("GET"))
        .and(path("/v2/usercollection/workout"))
        .respond_with(page(
            vec![serde_json::json!({
                "id": "w-1", "day": "2026-06-26",
                "activity": "bad\u{1b}[2Jrun\tning\nx",
                "intensity": "hard", "calories": 320.5, "source": "manual",
                "start_datetime": "2026-06-26T07:00:00+00:00",
                "end_datetime": "2026-06-26T07:45:00+00:00"
            })],
            None,
        ))
        .expect(1)
        .mount(&server)
        .await;

    let out = commands::workouts(&ctx(&server, &dir, fresh_tokens("at-1")), range())
        .await
        .unwrap();
    // ESC, tab, and newline each collapse to a space; the only tabs/newlines left are the
    // structural ones. `[2J` survives as inert text because its ESC is gone.
    assert_eq!(out, "2026-06-26\tbad [2Jrun ning x\thard\t320.5\n");
}
