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

/// The mock response for a fixture case: `raw_body_base64` (bytes JSON can't hold) or
/// `raw_body` sent verbatim, else the JSON `body`.
fn response_for(case: &serde_json::Value) -> ResponseTemplate {
    if let Some(b64) = case.get("raw_body_base64").and_then(|v| v.as_str()) {
        ResponseTemplate::new(200).set_body_raw(base64_decode(b64), "application/json")
    } else if let Some(raw) = case.get("raw_body").and_then(|v| v.as_str()) {
        ResponseTemplate::new(200).set_body_raw(raw.as_bytes().to_vec(), "application/json")
    } else {
        ResponseTemplate::new(200).set_body_json(case["body"].clone())
    }
}

/// Standard-alphabet base64 (padding required) — enough for the fixture, no extra dep.
fn base64_decode(s: &str) -> Vec<u8> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let sextet = |c: u8| {
        ALPHABET
            .iter()
            .position(|&a| a == c)
            .unwrap_or_else(|| panic!("fixture: invalid base64 byte {c:?}")) as u32
    };
    let bytes = s.as_bytes();
    assert_eq!(
        bytes.len() % 4,
        0,
        "fixture: base64 length must be a multiple of 4"
    );
    let mut out = Vec::new();
    for chunk in bytes.chunks(4) {
        let pad = chunk.iter().filter(|&&c| c == b'=').count();
        let n = chunk
            .iter()
            .map(|&c| if c == b'=' { 0 } else { sextet(c) })
            .fold(0u32, |acc, v| (acc << 6) | v);
        out.extend_from_slice(&n.to_be_bytes()[1..4 - pad]);
    }
    out
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
    assert!(cases.len() >= 24, "fixture shrank? {} cases", cases.len());

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
    assert!(cases.len() >= 24, "fixture shrank? {} cases", cases.len());

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
    assert!(cases.len() >= 17, "fixture shrank? {} cases", cases.len());

    for case in cases {
        let name = case["name"].as_str().unwrap();
        let expected = &case["expected"];
        let server = MockServer::start().await;
        Mock::given(method("POST"))
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
