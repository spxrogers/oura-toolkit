//! Binary-level enforcement of the auth account commands (#18): `oura auth
//! status|logout|refresh|token` behave with and without stored records, with the
//! documented exit codes and stream discipline (docs/cli-contract.md → Auth commands) —
//! asserted by spawning the real `oura` binary against an isolated store. Hermetic: no
//! network is reachable from any path exercised here (refresh success paths live in
//! auth.rs's unit tests behind the wiremock token-endpoint seam).

use assert_cmd::Command;
use oura_toolkit_auth::{ClientCredentials, TokenStore, Tokens};
use predicates::prelude::*;

const CLIENT_SECRET: &str = "SECRET-CS-789";
const ACCESS_TOKEN: &str = "SECRET-AT-123";
const REFRESH_TOKEN: &str = "SECRET-RT-456";

/// The `oura` binary pointed at an isolated config dir (same isolation as
/// cli_contract.rs), plus the [`TokenStore`] view of that dir for seeding records.
fn oura(args: &[&str]) -> (Command, TokenStore, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut cmd = Command::cargo_bin("oura").expect("oura binary");
    cmd.env("XDG_CONFIG_HOME", dir.path())
        .env("HOME", dir.path())
        .env("LOCALAPPDATA", dir.path())
        .env("NO_COLOR", "1")
        .args(args);
    // The binary derives its store dir from the env above; seed through the same layout.
    let store = TokenStore::with_dir(dir.path().join("oura-toolkit"));
    (cmd, store, dir)
}

fn credentials() -> ClientCredentials {
    ClientCredentials {
        client_id: "cid-123".into(),
        client_secret: CLIENT_SECRET.into(),
    }
}

fn tokens(expires_at: i64) -> Tokens {
    Tokens {
        access_token: ACCESS_TOKEN.into(),
        refresh_token: REFRESH_TOKEN.into(),
        expires_at,
        scope: Some("personal daily".into()),
        token_type: Some("Bearer".into()),
    }
}

/// Far-future expiry: never proactively refreshed, so no network is ever touched.
const FRESH: i64 = 4_102_444_800; // 2100-01-01

/// The secret-material attack test shared by every case: whatever the command did, the
/// client secret must appear on NEITHER stream, and token values only where deliberate.
fn assert_no_secrets(output: &std::process::Output, allow_access_token_on_stdout: bool) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stdout.contains(CLIENT_SECRET) && !stderr.contains(CLIENT_SECRET),
        "client secret leaked:\nstdout: {stdout}\nstderr: {stderr}"
    );
    assert!(
        !stdout.contains(REFRESH_TOKEN) && !stderr.contains(REFRESH_TOKEN),
        "refresh token leaked:\nstdout: {stdout}\nstderr: {stderr}"
    );
    if !allow_access_token_on_stdout {
        assert!(
            !stdout.contains(ACCESS_TOKEN),
            "access token leaked to stdout: {stdout}"
        );
    }
    assert!(
        !stderr.contains(ACCESS_TOKEN),
        "access token leaked to stderr: {stderr}"
    );
}

// --- status ----------------------------------------------------------------------------

#[test]
fn status_on_an_empty_store_reports_to_stdout_and_exits_4_toward_setup() {
    let (mut cmd, _store, _dir) = oura(&["auth", "status"]);
    let assert = cmd
        .assert()
        .code(4)
        // The report is the RESULT and still lands on stdout — the partial state is what
        // tells the user what to fix; the hint rides stderr per the contract.
        .stdout(predicate::str::contains("Credentials\tnone"))
        .stdout(predicate::str::contains("Tokens\tnone"))
        .stdout(predicate::str::contains("Authenticated\tno"))
        .stderr(predicate::str::contains("hint: run `oura auth setup`"));
    assert_no_secrets(assert.get_output(), false);
}

#[test]
fn status_with_credentials_only_shows_the_client_id_and_hints_login() {
    let (mut cmd, store, _dir) = oura(&["auth", "status"]);
    store.save_credentials(&credentials()).unwrap();
    let assert = cmd
        .assert()
        .code(4)
        .stdout(predicate::str::contains(
            "Credentials\tpresent (client_id: cid-123)",
        ))
        .stdout(predicate::str::contains("Tokens\tnone"))
        .stderr(predicate::str::contains("hint: run `oura auth login`"));
    assert_no_secrets(assert.get_output(), false);
}

#[test]
fn status_authenticated_exits_0_with_scope_and_expiry_and_no_secret_material() {
    let (mut cmd, store, _dir) = oura(&["auth", "status"]);
    store.save_credentials(&credentials()).unwrap();
    store.save_tokens(&tokens(FRESH)).unwrap();
    let assert = cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("client_id: cid-123"))
        .stdout(predicate::str::contains("Scope\tpersonal daily"))
        .stdout(predicate::str::contains("Access token\texpires in"))
        .stdout(predicate::str::contains("Authenticated\tyes"))
        .stderr(predicate::str::is_empty());
    assert_no_secrets(assert.get_output(), false);
}

#[test]
fn status_json_is_machine_readable_and_secret_free() {
    let (mut cmd, store, _dir) = oura(&["auth", "status", "--json"]);
    store.save_credentials(&credentials()).unwrap();
    store.save_tokens(&tokens(FRESH)).unwrap();
    let assert = cmd.assert().success();
    let output = assert.get_output();
    let v: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("--json status must be valid JSON");
    assert_eq!(v["authenticated"], true);
    assert_eq!(v["credentials"]["client_id"], "cid-123");
    assert_eq!(v["tokens"]["expired"], false);
    assert_eq!(v["tokens"]["expires_at"], FRESH);
    assert_no_secrets(output, false);
}

#[test]
fn status_json_when_unauthenticated_still_emits_json_and_exits_4() {
    // The scripting combo: parse stdout, branch on the exit code.
    let (mut cmd, _store, _dir) = oura(&["auth", "status", "--json"]);
    let assert = cmd.assert().code(4);
    let v: serde_json::Value = serde_json::from_slice(&assert.get_output().stdout)
        .expect("--json status must be valid JSON even when unauthenticated");
    assert_eq!(v["authenticated"], false);
    assert_eq!(v["credentials"]["present"], false);
    assert_eq!(v["tokens"]["present"], false);
}

// --- token -----------------------------------------------------------------------------

#[test]
fn token_prints_exactly_the_access_token_and_nothing_else() {
    let (mut cmd, store, _dir) = oura(&["auth", "token"]);
    store.save_credentials(&credentials()).unwrap();
    store.save_tokens(&tokens(FRESH)).unwrap();
    cmd.assert()
        .success()
        // The deliberate output: the token, one newline, nothing else — so
        // `curl -H "Authorization: Bearer $(oura auth token)"` composes cleanly.
        .stdout(predicate::eq(format!("{ACCESS_TOKEN}\n")))
        .stderr(predicate::str::is_empty());
}

#[test]
fn token_without_tokens_exits_4_with_an_empty_stdout() {
    let (mut cmd, store, _dir) = oura(&["auth", "token"]);
    store.save_credentials(&credentials()).unwrap();
    let assert = cmd
        .assert()
        .code(4)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("hint: run `oura auth login`"));
    assert_no_secrets(assert.get_output(), false);
}

#[test]
fn token_on_an_empty_store_hints_setup_not_login() {
    let (mut cmd, _store, _dir) = oura(&["auth", "token"]);
    cmd.assert()
        .code(4)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("hint: run `oura auth setup`"));
}

// --- logout ----------------------------------------------------------------------------

#[test]
fn logout_deletes_tokens_keeps_credentials_and_is_idempotent() {
    let (mut cmd, store, dir) = oura(&["auth", "logout"]);
    store.save_credentials(&credentials()).unwrap();
    store.save_tokens(&tokens(FRESH)).unwrap();
    let assert = cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ Logged out — removed"))
        .stdout(predicate::str::contains("oura auth logout --all"));
    assert_no_secrets(assert.get_output(), false);
    assert!(!store.tokens_path().exists(), "tokens.json must be deleted");
    assert!(
        store.credentials_path().exists(),
        "plain logout keeps the registered app credentials"
    );

    // Second run: nothing left to remove is still success (idempotent, scriptable).
    let mut again = Command::cargo_bin("oura").expect("oura binary");
    again
        .env("XDG_CONFIG_HOME", dir.path())
        .env("HOME", dir.path())
        .env("LOCALAPPDATA", dir.path())
        .env("NO_COLOR", "1")
        .args(["auth", "logout"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No tokens stored — already logged out.",
        ));
}

#[test]
fn logout_all_deletes_the_client_credentials_too() {
    let (mut cmd, store, _dir) = oura(&["auth", "logout", "--all"]);
    store.save_credentials(&credentials()).unwrap();
    store.save_tokens(&tokens(FRESH)).unwrap();
    let assert = cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("✓ Removed client credentials —"));
    assert_no_secrets(assert.get_output(), false);
    assert!(!store.tokens_path().exists());
    assert!(
        !store.credentials_path().exists(),
        "--all is the sanctioned way to remove the stored secret (#18)"
    );
}

// --- refresh ---------------------------------------------------------------------------

#[test]
fn refresh_without_tokens_exits_4_with_the_login_hint() {
    let (mut cmd, store, _dir) = oura(&["auth", "refresh"]);
    store.save_credentials(&credentials()).unwrap();
    let assert = cmd
        .assert()
        .code(4)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("oura: refreshing tokens:"))
        .stderr(predicate::str::contains("hint: run `oura auth login`"));
    assert_no_secrets(assert.get_output(), false);
}

#[test]
fn refresh_on_an_empty_store_hints_setup() {
    let (mut cmd, _store, _dir) = oura(&["auth", "refresh"]);
    cmd.assert()
        .code(4)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("hint: run `oura auth setup`"));
}
