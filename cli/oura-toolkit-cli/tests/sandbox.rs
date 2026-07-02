//! Live integration tests against Oura's sandbox routes (`/v2/sandbox/usercollection/*`,
//! canned data) — run via `just test-sandbox`.
//!
//! These need NETWORK but no credentials: the sandbox accepts any Authorization string.
//! They are `#[ignore]`d so `just test` / `just ci` stay hermetic; their value is
//! validating the real wire — TLS, the generated client's query serialization against the
//! actual server, real-payload deserialization into the generated models, and real
//! `next_token` pagination — the class of bug wiremock can only approximate.

use oura_toolkit_auth::{ClientCredentials, TokenManager, TokenStore, Tokens};
use oura_toolkit_cli::api::{authorized_client, paginate, API_BASE};

/// Production wiring with a throwaway store: the sandbox honors any bearer string.
fn sandbox_manager(dir: &tempfile::TempDir) -> TokenManager {
    let store = TokenStore::with_dir(dir.path());
    let tokens = Tokens {
        access_token: "sandbox".into(),
        refresh_token: "unused".into(),
        expires_at: 4_102_444_800, // 2100 — never proactively refreshed
        scope: None,
        token_type: None,
    };
    store.save_tokens(&tokens).unwrap();
    let credentials = ClientCredentials {
        client_id: "sandbox".into(),
        client_secret: "sandbox".into(),
    };
    store.save_credentials(&credentials).unwrap();
    TokenManager::from_parts(store, Some(credentials), Some(tokens))
}

#[tokio::test]
#[ignore = "network: hits the live Oura sandbox (just test-sandbox)"]
async fn sandbox_daily_sleep_roundtrips_through_the_real_wire() {
    let dir = tempfile::tempdir().unwrap();
    let manager = sandbox_manager(&dir);

    let start = chrono::NaiveDate::from_ymd_opt(2026, 6, 25).unwrap();
    let end = chrono::NaiveDate::from_ymd_opt(2026, 7, 1).unwrap();

    let docs = paginate(|token| {
        let manager = &manager;
        async move {
            let client = authorized_client(manager, API_BASE).await?;
            let resp = client
                .sandbox_multiple_daily_sleep_documents_v2_sandbox_usercollection_daily_sleep_get(
                    Some(&end),
                    token.as_deref(),
                    Some(&start),
                )
                .await
                .map_err(|e| anyhow::anyhow!("sandbox daily_sleep failed: {e}"))?;
            let inner = resp.into_inner();
            Ok((inner.data, inner.next_token))
        }
    })
    .await
    .unwrap();

    // Canned data, but its exact values are the server's business — assert the contract:
    // documents exist, deserialized into the real model, and every day is within range.
    assert!(!docs.is_empty(), "sandbox returned no documents");
    // `day` is the generated IsoDate (a YYYY-MM-DD string): lexicographic == chronological.
    let (start, end) = (start.to_string(), end.to_string());
    for d in &docs {
        assert!(
            *d.day >= start && *d.day <= end,
            "document day {} outside requested range",
            *d.day
        );
    }
}
