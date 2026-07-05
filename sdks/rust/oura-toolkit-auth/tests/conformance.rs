//! Cross-language auth-companion conformance (#58) — the RUST reference leg.
//!
//! Iterates `codegen/conformance/auth-cases.json` (the single source for the hostile
//! token-endpoint responses, hostile store files, and canonical store records that every
//! companion suite must exercise):
//!
//! - hostile-but-2xx token responses → typed [`AuthError`], store UNTOUCHED (the rotated
//!   refresh token is never burned by persisting a blank/expired Bearer);
//! - hostile store files → typed store-format error, never a default-filled record or a
//!   panic;
//! - canonical valid records → load with exactly the fixture's field values and
//!   round-trip through this crate's own persist path (the cross-language store
//!   compatibility check — field names are the shared wire format, #54).
//!
//! Monorepo-only (walks to the repo root for the fixture; `exclude = ["tests/"]` keeps it
//! out of the published package). Needs the `test-util` feature for the token-endpoint
//! seam — always on under `cargo test --workspace` (unified from the CLI's dev-deps);
//! compiles to nothing under a bare `cargo test -p oura-toolkit-auth`.
#![cfg(feature = "test-util")]

use oura_toolkit_auth::{AuthError, ClientCredentials, TokenManager, TokenStore, Tokens};
use wiremock::matchers::method;
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Repo root: nearest ancestor holding the justfile + README (same walk as bundled_spec).
fn repo_root() -> std::path::PathBuf {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if dir.join("justfile").is_file() && dir.join("README.md").is_file() {
            return dir;
        }
        assert!(dir.pop(), "repo root not found above CARGO_MANIFEST_DIR");
    }
}

fn fixture() -> serde_json::Value {
    let path = repo_root().join("codegen/conformance/auth-cases.json");
    serde_json::from_str(&std::fs::read_to_string(&path).expect("reading the fixture"))
        .expect("fixture is valid JSON")
}

fn original_tokens() -> Tokens {
    Tokens {
        access_token: "at-original".into(),
        refresh_token: "rt-original".into(),
        expires_at: 0, // expired, so force_refresh genuinely calls the endpoint
        scope: None,
        token_type: None,
    }
}

fn credentials() -> ClientCredentials {
    ClientCredentials {
        client_id: "cid".into(),
        client_secret: "cs".into(),
    }
}

/// Every hostile-but-2xx token response must fail the refresh with a TYPED [`AuthError`]
/// and leave the persisted record byte-identical — the rotated refresh token is never
/// burned by a blank/expired Bearer.
#[tokio::test]
async fn hostile_2xx_token_responses_fail_typed_and_leave_the_store_untouched() {
    let cases = fixture()["hostile_token_responses"]
        .as_array()
        .expect("hostile_token_responses table")
        .clone();
    assert!(cases.len() >= 8, "fixture shrank? {} cases", cases.len());

    for case in cases {
        let name = case["name"].as_str().unwrap();
        let template = match case.get("raw_body").and_then(|v| v.as_str()) {
            Some(raw) => ResponseTemplate::new(200).set_body_string(raw),
            None => ResponseTemplate::new(200).set_body_json(case["body"].clone()),
        };

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(template)
            .expect(1)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_credentials(&credentials()).unwrap();
        store.save_tokens(&original_tokens()).unwrap();
        let bytes_before = std::fs::read(store.tokens_path()).unwrap();

        let mut manager =
            TokenManager::from_parts(store.clone(), Some(credentials()), Some(original_tokens()));
        manager.override_token_url(server.uri());

        let err = manager
            .force_refresh()
            .await
            .expect_err(&format!("case {name}: a hostile 2xx must not succeed"));
        // Typed: the decode/validation family — never a panic (the test reaching this
        // line proves that), and never a mis-filed auth-flow error that would trigger
        // remediation hints for a server-side fault. Deliberately a SET of variants where
        // other legs pin one wrapper type: Rust's taxonomy surfaces decode failures as
        // Serde/Http rather than folding them into the endpoint error; the load-bearing
        // properties (typed + store untouched + expect(1)) are identical.
        assert!(
            matches!(
                err,
                AuthError::InvalidTokenResponse(_) | AuthError::Http(_) | AuthError::Serde(_)
            ),
            "case {name}: expected a typed invalid-response error, got {err:?}"
        );
        assert_eq!(
            std::fs::read(store.tokens_path()).unwrap(),
            bytes_before,
            "case {name}: the store must be UNTOUCHED (rotation not burned)"
        );
    }
}

/// Every hostile store file must load as a TYPED store-format error — never a
/// default-filled record, never a panic.
#[test]
fn hostile_store_files_fail_typed() {
    let cases = fixture()["hostile_store_files"]
        .as_array()
        .expect("hostile_store_files table")
        .clone();
    assert!(cases.len() >= 8, "fixture shrank? {} cases", cases.len());

    for case in cases {
        let name = case["name"].as_str().unwrap();
        let file = case["file"].as_str().unwrap();
        let content = case["content"].as_str().unwrap();

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        std::fs::write(dir.path().join(file), content).unwrap();

        match file {
            "tokens.json" => {
                let err = store
                    .load_tokens()
                    .expect_err(&format!("case {name}: hostile tokens.json must not load"));
                assert!(
                    matches!(err, AuthError::Serde(_)),
                    "case {name}: expected the typed store-format error, got {err:?}"
                );
            }
            "credentials.json" => {
                let err = store.load_credentials().expect_err(&format!(
                    "case {name}: hostile credentials.json must not load"
                ));
                assert!(
                    matches!(err, AuthError::Serde(_)),
                    "case {name}: expected the typed store-format error, got {err:?}"
                );
            }
            other => panic!("fixture names an unknown store file {other:?}"),
        }
    }
}

/// The canonical records load with exactly the fixture's values and survive a round-trip
/// through this crate's own persist path — the shared wire format every language reads.
#[test]
fn canonical_valid_records_load_and_round_trip() {
    let fixture = fixture();
    let valid = &fixture["valid_records"];

    let dir = tempfile::tempdir().unwrap();
    let store = TokenStore::with_dir(dir.path());
    std::fs::write(
        dir.path().join("credentials.json"),
        serde_json::to_string_pretty(&valid["credentials.json"]).unwrap(),
    )
    .unwrap();
    std::fs::write(
        dir.path().join("tokens.json"),
        serde_json::to_string_pretty(&valid["tokens.json"]).unwrap(),
    )
    .unwrap();

    let creds = store.load_credentials().unwrap().expect("credentials load");
    assert_eq!(creds.client_id, "cid-conformance");
    assert_eq!(creds.client_secret, "cs-conformance");

    let tokens = store.load_tokens().unwrap().expect("tokens load");
    assert_eq!(tokens.access_token, "at-conformance");
    assert_eq!(tokens.refresh_token, "rt-conformance");
    assert_eq!(tokens.expires_at, 4_102_444_800);
    assert_eq!(tokens.scope.as_deref(), Some("personal daily"));
    assert_eq!(tokens.token_type.as_deref(), Some("Bearer"));

    // Round-trip: this crate's persist path must re-emit records the loader (and, by the
    // shared fixture, every other language) still reads identically.
    store.save_credentials(&creds).unwrap();
    store.save_tokens(&tokens).unwrap();
    assert_eq!(store.load_credentials().unwrap().unwrap(), creds);
    assert_eq!(store.load_tokens().unwrap().unwrap(), tokens);
}
