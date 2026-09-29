//! Helpers shared by the wiremock-backed integration suites (`data_commands.rs`,
//! `mcp_server.rs`, `headless_auth.rs`) — one definition so canned fixtures can't drift
//! between the CLI and MCP views of the same data plane.
//!
//! Each test binary uses a different subset, so helpers unused in a given binary are
//! expected — silence the per-binary dead-code warning rather than duplicate fixtures.
#![allow(dead_code)]

use wiremock::ResponseTemplate;

/// A grant from before openapi-1.41 (`spo2Daily`, no `heart_health`): stale for the
/// re-consent check (#116), which should report exactly `spo2 heart_health` missing.
pub const PRE_1_41_GRANT: &str = "personal daily heartrate workout tag session spo2Daily";

/// A grant covering every current default scope: the re-consent check stays silent.
pub fn current_grant() -> String {
    oura_toolkit_auth::metadata::default_scopes().join(" ")
}

/// Fresh tokens with a CURRENT grant, so suites that aren't about re-consent (#116) never
/// trip its notice. Tests of the notice set `scope` explicitly.
pub fn fresh_tokens(access: &str) -> oura_toolkit_auth::Tokens {
    oura_toolkit_auth::Tokens {
        access_token: access.into(),
        refresh_token: "rt-1".into(),
        expires_at: 4_102_444_800, // 2100 — never proactively refreshed
        scope: Some(current_grant()),
        token_type: None,
    }
}

pub fn sleep_doc(day: &str, score: i64) -> serde_json::Value {
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

pub fn page(data: Vec<serde_json::Value>, next: Option<&str>) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "data": data,
        "next_token": next,
    }))
}
