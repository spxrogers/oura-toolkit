//! Hermetic integration tests for the local health-store surfaces: `oura sync`'s
//! fetch→store mapping through wiremock, and the binary-level contract of
//! `oura import` / `oura capacity` / `oura context` (exit codes, stream discipline, and
//! the data-safety behaviors the vision doc's threat model mandates: the
//! `--remove-source` path, the plaintext-export reminder, and the cloud-sync-root
//! warning).

use std::collections::BTreeMap;

use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer};

use oura_toolkit_auth::{ClientCredentials, TokenManager, TokenStore, Tokens};
use oura_toolkit_cli::api::DateRange;
use oura_toolkit_cli::health;
use oura_toolkit_cli::output::{Format, RenderOptions, Style};
use oura_toolkit_health::{HealthStore, OuraDay};

mod common;
use common::{fresh_tokens, page, sleep_doc};

fn plain() -> RenderOptions {
    RenderOptions {
        format: Format::Plain,
        style: Style::new(false),
    }
}

fn manager(dir: &tempfile::TempDir, tokens: Tokens) -> TokenManager {
    let store = TokenStore::with_dir(dir.path());
    store.save_tokens(&tokens).unwrap();
    let credentials = ClientCredentials {
        client_id: "cid".into(),
        client_secret: "sec".into(),
    };
    store.save_credentials(&credentials).unwrap();
    TokenManager::from_parts(store, Some(credentials), Some(tokens))
}

fn range() -> DateRange {
    let today = "2026-07-02".parse().unwrap();
    DateRange::resolve(Some("2026-06-26"), Some("2026-07-02"), today).unwrap()
}

/// Mount all four daily endpoints with one-page canned data for 2026-06-29/30.
async fn mount_dailies(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_sleep"))
        .respond_with(page(vec![sleep_doc("2026-06-29", 82)], None))
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_readiness"))
        .respond_with(page(
            vec![serde_json::json!({
                "id": "r-1", "day": "2026-06-29", "score": 90,
                "temperature_deviation": -0.2, "contributors": {},
                "timestamp": "2026-06-29T00:00:00+00:00"
            })],
            None,
        ))
        .mount(server)
        .await;
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
        .mount(server)
        .await;
    Mock::given(method("GET"))
        .and(path("/v2/usercollection/daily_stress"))
        .respond_with(page(
            vec![serde_json::json!({
                "id": "s-1", "day": "2026-06-29", "day_summary": "normal",
                "stress_high": 3600, "recovery_high": 7200
            })],
            None,
        ))
        .mount(server)
        .await;
}

/// `oura sync` folds the four Oura dailies into per-day OuraDay records: fields land on
/// the right days, days missing from one endpoint simply lack that field.
#[tokio::test]
async fn sync_folds_the_four_dailies_into_per_day_records() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();
    mount_dailies(&server).await;

    let store = HealthStore::with_dir(dir.path().join("health"));
    let manager = manager(&dir, fresh_tokens("at-1"));
    let out = health::sync(&manager, &server.uri(), &store, range(), plain())
        .await
        .unwrap();
    assert!(
        out.contains("source\tOura") && out.contains("days\t2 (2 added, 0 updated, 0 unchanged)"),
        "summary names the source and the day stats: {out:?}"
    );

    let days = store.load().unwrap();
    let d29 = &days[&"2026-06-29".parse().unwrap()];
    let oura = d29.oura.as_ref().unwrap();
    assert_eq!(oura.sleep_score, Some(82.0));
    assert_eq!(oura.readiness_score, Some(90.0));
    assert_eq!(oura.temperature_deviation_c, Some(-0.2));
    assert_eq!(oura.stress_high_seconds, Some(3600.0));
    assert_eq!(oura.recovery_high_seconds, Some(7200.0));
    assert_eq!(oura.activity_score, None, "no activity doc for the 29th");
    let d30 = &days[&"2026-06-30".parse().unwrap()];
    let oura = d30.oura.as_ref().unwrap();
    assert_eq!(oura.activity_score, Some(85.0));
    assert_eq!(oura.steps, Some(9000.0));
    assert_eq!(oura.sleep_score, None, "no sleep doc for the 30th");
}

/// Re-running the same sync is an idempotent no-op (unchanged, not duplicated).
#[tokio::test]
async fn resync_is_idempotent() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().unwrap();
    mount_dailies(&server).await;

    let store = HealthStore::with_dir(dir.path().join("health"));
    let manager = manager(&dir, fresh_tokens("at-1"));
    health::sync(&manager, &server.uri(), &store, range(), plain())
        .await
        .unwrap();
    let before = store.load().unwrap();
    let out = health::sync(&manager, &server.uri(), &store, range(), plain())
        .await
        .unwrap();
    assert!(
        out.contains("days\t2 (0 added, 0 updated, 2 unchanged)"),
        "re-sync must be a no-op: {out:?}"
    );
    assert_eq!(store.load().unwrap(), before);
}

/// `oura sync` without `--start` covers SYNC_DEFAULT_DAYS, not the data commands' 7.
#[test]
fn sync_default_range_is_90_days() {
    let today: chrono::NaiveDate = "2026-07-02".parse().unwrap();
    let range =
        health::resolve_range_with_default(None, None, today, health::SYNC_DEFAULT_DAYS).unwrap();
    assert_eq!(range.end, today);
    assert_eq!(
        (range.end - range.start).num_days() + 1,
        health::SYNC_DEFAULT_DAYS as i64
    );
    // And an inverted explicit range stays a usage error (exit-2 class).
    let err = health::resolve_range_with_default(
        Some("2026-07-02"),
        Some("2026-07-01"),
        today,
        health::SYNC_DEFAULT_DAYS,
    )
    .unwrap_err();
    assert!(
        err.chain().any(|c| c
            .downcast_ref::<oura_toolkit_cli::contract::UsageError>()
            .is_some()),
        "inverted range must classify as usage: {err:?}"
    );
}

// ---------------------------------------------------------------------------------------
// Binary-level contract (assert_cmd): exit codes, streams, and the safety behaviors.
// ---------------------------------------------------------------------------------------

/// An isolated `oura` invocation: config+data stores in tempdirs via env.
fn oura(home: &tempfile::TempDir, data: &std::path::Path) -> assert_cmd::Command {
    let mut cmd = assert_cmd::Command::cargo_bin("oura").expect("oura binary");
    cmd.env("XDG_CONFIG_HOME", home.path())
        .env("XDG_DATA_HOME", data)
        .env("HOME", home.path())
        .env("LOCALAPPDATA", home.path())
        .env("NO_COLOR", "1");
    cmd
}

const TOGGL_CSV: &str = "\
User,Email,Client,Project,Task,Description,Billable,Start date,Start time,End date,End time,Duration,Tags,Amount ()
u,u@example.com,,Proj,,deep work,No,2026-07-01,09:00:00,2026-07-01,10:30:00,01:30:00,,
";

/// A minimal but valid bare export.xml (the zip-less input path).
const APPLE_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<HealthData locale="en_US">
 <Record type="HKQuantityTypeIdentifierStepCount" sourceName="iPhone" unit="count" startDate="2026-07-01 08:00:00 -0500" endDate="2026-07-01 08:10:00 -0500" value="612"/>
</HealthData>
"#;

#[test]
fn import_toggl_succeeds_and_the_summary_goes_to_stdout() {
    let home = tempfile::tempdir().unwrap();
    let data = home.path().join("data");
    let csv = home.path().join("report.csv");
    std::fs::write(&csv, TOGGL_CSV).unwrap();

    let out = oura(&home, &data)
        .args(["import", "toggl"])
        .arg(&csv)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).unwrap();
    assert!(
        stdout.contains("source\tToggl") && stdout.contains("1 added"),
        "summary on stdout: {stdout:?}"
    );
}

#[test]
fn import_apple_health_keeps_the_export_by_default_and_says_so_on_stderr() {
    let home = tempfile::tempdir().unwrap();
    let data = home.path().join("data");
    let xml = home.path().join("export.xml");
    std::fs::write(&xml, APPLE_XML).unwrap();

    let out = oura(&home, &data)
        .args(["import", "apple-health"])
        .arg(&xml)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(xml.exists(), "without --remove-source the export survives");
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.contains("still exists") && stderr.contains("--remove-source"),
        "the plaintext-export reminder is a documented safety behavior: {stderr:?}"
    );
}

#[test]
fn import_apple_health_remove_source_deletes_the_export_after_success() {
    let home = tempfile::tempdir().unwrap();
    let data = home.path().join("data");
    let xml = home.path().join("export.xml");
    std::fs::write(&xml, APPLE_XML).unwrap();

    oura(&home, &data)
        .args(["import", "apple-health", "--remove-source"])
        .arg(&xml)
        .assert()
        .success();
    assert!(!xml.exists(), "--remove-source must delete the export");
}

#[test]
fn a_store_inside_a_sync_root_warns_on_stderr_but_still_imports() {
    let home = tempfile::tempdir().unwrap();
    // XDG_DATA_HOME deliberately inside a Dropbox-shaped path.
    let data = home.path().join("Dropbox").join("base");
    let csv = home.path().join("report.csv");
    std::fs::write(&csv, TOGGL_CSV).unwrap();

    let out = oura(&home, &data)
        .args(["import", "toggl"])
        .arg(&csv)
        .output()
        .unwrap();
    assert!(out.status.success());
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.contains("cloud-synced"),
        "sync-root warning must reach stderr: {stderr:?}"
    );
}

#[test]
fn a_malformed_import_is_a_runtime_error_naming_the_file() {
    let home = tempfile::tempdir().unwrap();
    let data = home.path().join("data");
    let junk = home.path().join("junk.csv");
    std::fs::write(&junk, "definitely,not,a\ntoggl,export,file\n").unwrap();

    let out = oura(&home, &data)
        .args(["import", "toggl"])
        .arg(&junk)
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1), "malformed input is exit 1");
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.starts_with("oura: importing") && stderr.contains("junk.csv"),
        "single-line contract error naming the file: {stderr:?}"
    );
    assert!(out.stdout.is_empty(), "errors never write to stdout");
}

#[test]
fn capacity_with_an_empty_store_refuses_with_the_import_remediation() {
    let home = tempfile::tempdir().unwrap();
    let data = home.path().join("data");

    let out = oura(&home, &data).arg("capacity").output().unwrap();
    assert_eq!(
        out.status.code(),
        Some(1),
        "thin history is a runtime refusal, not a crash: stderr {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stderr = String::from_utf8(out.stderr).unwrap();
    assert!(
        stderr.contains("not enough history") && stderr.contains("oura import"),
        "the refusal must carry its remediation: {stderr:?}"
    );
    assert!(out.stdout.is_empty(), "errors never write to stdout");
}

#[test]
fn context_renders_the_stored_records() {
    let home = tempfile::tempdir().unwrap();
    let data = home.path().join("data");
    let csv = home.path().join("report.csv");
    std::fs::write(&csv, TOGGL_CSV).unwrap();
    oura(&home, &data)
        .args(["import", "toggl"])
        .arg(&csv)
        .assert()
        .success();

    let out = oura(&home, &data)
        .args(["context", "--start", "2026-07-01", "--end", "2026-07-01"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let stdout = String::from_utf8(out.stdout).unwrap();
    // Piped → plain TSV: day, then the toggl columns carry the tracked time.
    assert!(
        stdout.starts_with("2026-07-01\t") && stdout.contains("1.5"),
        "the imported 1.5 tracked hours render for their day: {stdout:?}"
    );
}

/// The store the engine and the MCP server read is the SAME one imports write: pinned
/// here at the API level (upsert → fetch_day_context round trip).
#[test]
fn fetch_day_context_windows_the_store() {
    let dir = tempfile::tempdir().unwrap();
    let store = HealthStore::with_dir(dir.path().join("health"));
    let mut days = BTreeMap::new();
    for d in ["2026-06-30", "2026-07-01", "2026-07-02"] {
        days.insert(
            d.parse().unwrap(),
            OuraDay {
                readiness_score: Some(80.0),
                ..Default::default()
            },
        );
    }
    store.upsert(days).unwrap();

    let today = "2026-07-02".parse().unwrap();
    let range = DateRange::resolve(Some("2026-07-01"), Some("2026-07-01"), today).unwrap();
    let window = health::fetch_day_context(&store, range).unwrap();
    assert_eq!(window.len(), 1, "only the requested day");
    assert!(window.contains_key(&"2026-07-01".parse().unwrap()));
}
