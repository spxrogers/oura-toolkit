//! Cross-language auth-companion conformance (#58) — the RUST reference leg.
//!
//! Iterates `codegen/conformance/auth-cases.json` (the single source for the hostile
//! token-endpoint responses, hostile store files, and canonical store records that every
//! companion suite must exercise):
//!
//! - hostile-but-2xx token responses → typed [`AuthError`], store UNTOUCHED (the rotated
//!   refresh token is never burned by persisting a blank/expired Bearer) — on the refresh
//!   AND the authorization-code exchange;
//! - successful refreshes (`refresh_success_cases`) → exactly the expected persisted
//!   record, incl. the omitted/null/blank fallbacks for scope, refresh_token, token_type;
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

use oura_toolkit_auth::{
    exchange_code_at, AuthError, ClientCredentials, TokenManager, TokenStore, Tokens,
};
use wiremock::matchers::{body_string_contains, method};
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

/// The mock response for a fixture case: exactly one of `body` (JSON), `raw_body` (sent
/// verbatim) or `raw_body_base64` (bytes JSON can't hold) — a case naming two would silently
/// test only one of them.
fn response_for(case: &serde_json::Value) -> ResponseTemplate {
    use base64::Engine as _;
    let name = case["name"].as_str().unwrap_or("<unnamed>");
    let columns: Vec<&str> = ["body", "raw_body", "raw_body_base64"]
        .into_iter()
        .filter(|k| case.get(*k).is_some())
        .collect();
    assert_eq!(
        columns.len(),
        1,
        "fixture case {name}: exactly one body column, got {columns:?}"
    );
    let raw = |k: &str| {
        case[k]
            .as_str()
            .unwrap_or_else(|| panic!("case {name}: {k} is a string"))
    };
    match columns[0] {
        "raw_body_base64" => ResponseTemplate::new(200).set_body_raw(
            base64::engine::general_purpose::STANDARD
                .decode(raw("raw_body_base64"))
                .unwrap_or_else(|e| panic!("case {name}: bad base64: {e}")),
            "application/json",
        ),
        "raw_body" => ResponseTemplate::new(200)
            .set_body_raw(raw("raw_body").as_bytes().to_vec(), "application/json"),
        _ => ResponseTemplate::new(200).set_body_json(case["body"].clone()),
    }
}

fn unix_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
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
    assert!(cases.len() >= 26, "fixture shrank? {} cases", cases.len());

    for case in cases {
        let name = case["name"].as_str().unwrap();
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(response_for(&case))
            .named(name)
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
        // Typed: the invalid-response variant — never a panic (the test reaching this line
        // proves that), and never a mis-filed auth-flow or transport error that would
        // trigger remediation hints for a server-side fault.
        assert!(
            matches!(err, AuthError::InvalidTokenResponse(_)),
            "case {name}: expected AuthError::InvalidTokenResponse, got {err:?}"
        );
        assert_eq!(
            std::fs::read(store.tokens_path()).unwrap(),
            bytes_before,
            "case {name}: the store must be UNTOUCHED (rotation not burned)"
        );
    }
}

/// The authorization-code exchange runs every hostile body through the same guards as a
/// refresh (the fixture says "refresh/exchange"): typed error, nothing to persist.
#[tokio::test]
async fn hostile_2xx_token_responses_fail_the_code_exchange_typed() {
    let cases = fixture()["hostile_token_responses"]
        .as_array()
        .expect("hostile_token_responses table")
        .clone();
    assert!(cases.len() >= 26, "fixture shrank? {} cases", cases.len());

    for case in cases {
        let name = case["name"].as_str().unwrap();
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(response_for(&case))
            .named(name)
            .expect(1)
            .mount(&server)
            .await;
        let err = exchange_code_at(
            &server.uri(),
            &reqwest::Client::new(),
            &credentials(),
            "code",
            "http://localhost:8788/callback",
        )
        .await
        .expect_err(&format!("case {name}: a hostile 2xx must not exchange"));
        assert!(
            matches!(err, AuthError::InvalidTokenResponse(_)),
            "case {name}: expected AuthError::InvalidTokenResponse, got {err:?}"
        );
    }
}

/// A successful refresh persists exactly the fixture's `expected` record: the returned
/// values, or the prior ones where the response omits/nulls/blanks a field (conformance
/// `refresh_success_cases`).
#[tokio::test]
async fn refresh_success_cases_persist_the_expected_record() {
    let table = fixture()["refresh_success_cases"].clone();
    let prior = &table["prior"];
    let field = |v: &serde_json::Value, k: &str| -> String {
        v[k].as_str()
            .unwrap_or_else(|| panic!("fixture: {k} must be a string in {v}"))
            .to_string()
    };
    let cases = table["cases"].as_array().expect("cases").clone();
    assert!(cases.len() >= 18, "fixture shrank? {} cases", cases.len());

    for case in cases {
        let name = case["name"].as_str().unwrap();
        let expected = &case["expected"];
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            // The refresh must SEND the stored (prior) refresh token; `.expect(1)` then fails
            // a request that sent anything else.
            .and(body_string_contains(format!(
                "refresh_token={}",
                field(prior, "refresh_token")
            )))
            .respond_with(response_for(&case))
            .named(name) // a failed `.expect(1)` (checked on drop) then names the case
            .expect(1)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        let before = Tokens {
            access_token: field(prior, "access_token"),
            refresh_token: field(prior, "refresh_token"),
            expires_at: 0, // expired, so the refresh genuinely calls the endpoint
            scope: Some(field(prior, "scope")),
            token_type: Some(field(prior, "token_type")),
        };
        store.save_credentials(&credentials()).unwrap();
        store.save_tokens(&before).unwrap();
        let mut manager =
            TokenManager::from_parts(store.clone(), Some(credentials()), Some(before));
        manager.override_token_url(server.uri());

        let t0 = unix_now();
        manager
            .force_refresh()
            .await
            .unwrap_or_else(|e| panic!("case {name}: a valid refresh must succeed: {e:?}"));
        let t1 = unix_now();
        let saved = store
            .load_tokens()
            .unwrap()
            .expect("refreshed tokens persisted");
        assert_eq!(
            saved.access_token,
            field(expected, "access_token"),
            "case {name}: access_token"
        );
        assert_eq!(
            saved.refresh_token,
            field(expected, "refresh_token"),
            "case {name}: refresh_token"
        );
        assert_eq!(
            saved.scope.as_deref(),
            Some(field(expected, "scope").as_str()),
            "case {name}: scope"
        );
        assert_eq!(
            saved.token_type.as_deref(),
            Some(field(expected, "token_type").as_str()),
            "case {name}: token_type"
        );
        let lifetime = expected["expires_in"]
            .as_i64()
            .expect("fixture: expected.expires_in");
        assert!(
            (t0 + lifetime..=t1 + lifetime).contains(&saved.expires_at),
            "case {name}: expires_at {} not within [{}, {}]",
            saved.expires_at,
            t0 + lifetime,
            t1 + lifetime
        );
    }
}

/// Bodies where companions may legitimately differ (BOM, duplicate keys, deep nesting, …):
/// the refresh either succeeds with the returned access token, or fails with the typed
/// invalid-response error and leaves the store byte-identical — never a panic, never a
/// half-written store.
#[tokio::test]
async fn implementation_defined_token_responses_succeed_or_fail_typed() {
    let cases = fixture()["implementation_defined_token_responses"]
        .as_array()
        .expect("implementation_defined_token_responses table")
        .clone();
    assert!(cases.len() >= 5, "fixture shrank? {} cases", cases.len());

    for case in cases {
        let name = case["name"].as_str().unwrap();
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(response_for(&case))
            .named(name)
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

        match manager.force_refresh().await {
            Ok(_) => {
                let saved = store.load_tokens().unwrap().expect("tokens persisted");
                assert_eq!(
                    saved.access_token, "at-refreshed",
                    "case {name}: a success must persist the returned access token"
                );
            }
            Err(err) => {
                assert!(
                    matches!(err, AuthError::InvalidTokenResponse(_)),
                    "case {name}: a failure must be AuthError::InvalidTokenResponse, got {err:?}"
                );
                assert_eq!(
                    std::fs::read(store.tokens_path()).unwrap(),
                    bytes_before,
                    "case {name}: a failure must leave the store UNTOUCHED"
                );
            }
        }
    }
}

/// Every fixture table is exercised by this suite: a table added to the fixture must be
/// mapped here (and in the other five suites), never silently ignored.
#[test]
fn every_fixture_table_is_mapped_by_this_suite() {
    let fixture = fixture();
    let tables: std::collections::BTreeSet<&str> = fixture
        .as_object()
        .expect("fixture is an object")
        .keys()
        .map(String::as_str)
        .filter(|k| *k != "$comment")
        .collect();
    assert_eq!(
        tables,
        [
            "hostile_store_files",
            "hostile_token_responses",
            "implementation_defined_token_responses",
            "refresh_success_cases",
            "valid_records"
        ]
        .into_iter()
        .collect(),
        "the fixture's tables changed — map the new table in every companion suite"
    );
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
