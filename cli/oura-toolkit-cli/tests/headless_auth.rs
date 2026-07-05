//! Binary-level enforcement of the headless-auth surface (#20): spawn the REAL `oura`
//! binary and prove the env overrides are wired end to end —
//!
//! - `OURA_ACCESS_TOKEN` reaches an actual request as the `Authorization: Bearer` with NO
//!   store present (the CLI data path and the MCP server), and
//! - `OURA_API_BASE_URL` points that request at an alternate host, and
//! - a tool call that hits a 401 surfaces a STRUCTURED error while stdout stays pure
//!   JSON-RPC (the deferred silent-401 assertion from PR #38's review loop), and
//! - `oura auth login --no-browser` enforces the `state` CSRF check on the pasted redirect,
//!   with no network and stdout empty.
//!
//! Hermetic: the "Oura host" is a wiremock server on loopback; no real credentials, no
//! network. The in-process data-plane behaviours (pagination, 401→refresh→retry, rate
//! limiting) live in `data_commands.rs`; these pin the PROCESS boundary that only the env
//! wiring can break.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use oura_toolkit_auth::{ClientCredentials, TokenStore};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod common;
use common::{page, sleep_doc};

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

fn base_env(cmd: &mut Command, dir: &std::path::Path) {
    cmd.env("XDG_CONFIG_HOME", dir)
        .env("HOME", dir)
        .env("LOCALAPPDATA", dir)
        .env("NO_COLOR", "1");
}

#[test]
fn env_token_authorizes_a_data_command_with_no_store() {
    // The acceptance: `OURA_ACCESS_TOKEN=… oura sleep` works with NO store present. The mock
    // only answers when the Bearer equals the env token, so a green result PROVES the token
    // crossed the process boundary into the Authorization header (not just that a request was
    // made). --port/store are absent; the token bypasses the store entirely.
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/daily_sleep"))
                .and(header("authorization", "Bearer env-tok-xyz"))
                .respond_with(page(vec![sleep_doc("2026-06-26", 77)], None))
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
    base_env(&mut cmd, dir.path());
    let out = cmd
        .args([
            "sleep",
            "--json",
            "--start",
            "2026-06-26",
            "--end",
            "2026-06-26",
        ])
        .env("OURA_ACCESS_TOKEN", "env-tok-xyz")
        .env("OURA_API_BASE_URL", oura.server.uri())
        .output()
        .expect("run oura sleep");

    assert!(
        out.status.success(),
        "env-token data command failed (status {:?}); stderr: {}",
        out.status.code(),
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("2026-06-26") && stdout.contains("77"),
        "expected the sleep record in stdout: {stdout}"
    );
}

#[test]
fn wrong_env_token_is_rejected_without_creating_a_store() {
    // The negative half: a Bearer the mock doesn't recognise gets no 200 (wiremock 404s an
    // unmatched request), so the command fails — confirming the header matcher above is
    // load-bearing (a green test with ANY token would be self-masking).
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/daily_sleep"))
                .and(header("authorization", "Bearer the-right-one"))
                .respond_with(page(vec![sleep_doc("2026-06-26", 77)], None))
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
    base_env(&mut cmd, dir.path());
    let out = cmd
        .args([
            "sleep",
            "--json",
            "--start",
            "2026-06-26",
            "--end",
            "2026-06-26",
        ])
        .env("OURA_ACCESS_TOKEN", "the-WRONG-one")
        .env("OURA_API_BASE_URL", oura.server.uri())
        .output()
        .expect("run oura sleep");
    assert!(
        !out.status.success(),
        "a wrong env token must not yield a successful data pull"
    );
    // The env-token path bypasses the store entirely: it must not even create the config dir.
    assert!(
        !dir.path().join("oura-toolkit").exists(),
        "OURA_ACCESS_TOKEN must not touch/create the token store"
    );
}

#[test]
fn mcp_keeps_stdout_pure_json_rpc_when_a_tool_call_hits_a_401() {
    // The deferred silent-401 assertion (PR #38 review loop, unblocked by #20's base-URL
    // seam): a server started with OURA_ACCESS_TOKEN whose data endpoint 401s must surface a
    // STRUCTURED tool error — never a stdout leak, prompt, or crash. With a static env token
    // the 401 can't refresh, so it fails typed (StaticTokenRejected) without a token endpoint,
    // keeping the test hermetic.
    let oura = mock_oura(|server| {
        Box::pin(async move {
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/daily_sleep"))
                .respond_with(ResponseTemplate::new(401))
                .mount(server)
                .await;
        })
    });
    let dir = tempfile::tempdir().unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
    base_env(&mut cmd, dir.path());
    let mut child = cmd
        .arg("mcp")
        .env("OURA_ACCESS_TOKEN", "env-tok")
        .env("OURA_API_BASE_URL", oura.server.uri())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn oura mcp");
    let mut stdin = child.stdin.take().unwrap();
    let stdout = BufReader::new(child.stdout.take().unwrap());

    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"protocolVersion":"2025-06-18","capabilities":{{}},"clientInfo":{{"name":"t","version":"0"}}}}}}"#
    )
    .unwrap();
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","method":"notifications/initialized"}}"#
    )
    .unwrap();
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{{"name":"get_daily_sleep","arguments":{{}}}}}}"#
    )
    .unwrap();

    let reader = std::thread::spawn(move || {
        let mut call_response = None;
        for line in stdout.lines() {
            let line = line.expect("read stdout line");
            // THE stream-discipline assertion: every stdout byte is JSON-RPC, even while a
            // tool call is failing auth. A refresh notice or backtrace leaking here fails.
            let msg: serde_json::Value = serde_json::from_str(&line)
                .unwrap_or_else(|e| panic!("non-JSON on the MCP transport ({e}): {line:?}"));
            assert_eq!(msg["jsonrpc"], "2.0", "not JSON-RPC: {line:?}");
            if msg["id"].as_i64() == Some(2) {
                call_response = Some(msg);
                break;
            }
        }
        call_response
    });
    let deadline = Instant::now() + Duration::from_secs(20);
    while !reader.is_finished() {
        if Instant::now() > deadline {
            let _ = child.kill();
            panic!("oura mcp did not answer the tool call within 20s");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let call = reader
        .join()
        .expect("reader thread")
        .expect("tool/call response");

    // A structured tool error, not a protocol error: isError, with the env-token-specific
    // remediation (restart with a fresh token, NOT "run oura auth login" which a container
    // has no way to do).
    assert_eq!(
        call["result"]["isError"], true,
        "the 401 must surface as a tool error: {call}"
    );
    let text = call["result"]["content"][0]["text"]
        .as_str()
        .unwrap_or_default();
    assert!(
        text.contains("rejected") && text.contains("OURA_ACCESS_TOKEN"),
        "tool error must name the rejected env token: {text:?}"
    );

    drop(stdin);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if let Some(status) = child.try_wait().expect("try_wait") {
            assert!(status.success(), "clean exit after disconnect: {status:?}");
            break;
        }
        if Instant::now() > deadline {
            let _ = child.kill();
            panic!("oura mcp did not exit after stdin EOF");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[test]
fn no_browser_login_rejects_a_state_mismatch_without_network() {
    // --no-browser enforces the same CSRF `state` check as the loopback flow: a pasted
    // redirect from a different attempt is refused BEFORE any token exchange (so no network),
    // and the prose/error stays on stderr with stdout empty (the #37 contract).
    let dir = tempfile::tempdir().unwrap();
    // Seed credentials at the resolved store dir so login proceeds past the setup gate to the
    // paste step (APP_DIR_NAME subdir under XDG_CONFIG_HOME).
    let store = TokenStore::with_dir(dir.path().join("oura-toolkit"));
    store
        .save_credentials(&ClientCredentials {
            client_id: "cid".into(),
            client_secret: "sec".into(),
        })
        .unwrap();

    let mut cmd = assert_cmd::Command::cargo_bin("oura").unwrap();
    cmd.env("XDG_CONFIG_HOME", dir.path())
        .env("HOME", dir.path())
        .env("LOCALAPPDATA", dir.path())
        .env("NO_COLOR", "1")
        .args(["auth", "login", "--no-browser"])
        .write_stdin("http://localhost:8788/callback?code=abc&state=FORGED\n");
    // Exit 1 specifically (docs/cli-contract.md → --no-browser: "a mismatch aborts, exit 1"),
    // not the exit-4 auth path — a drift to 4 would pass a bare `.failure()`.
    cmd.assert()
        .code(1)
        .stdout(predicates::prelude::predicate::str::is_empty())
        .stderr(predicates::prelude::predicate::str::contains(
            "state mismatch",
        ));
}

#[test]
fn browser_login_over_ssh_suggests_no_browser() {
    // The documented auto-suggest (README "Headless"): browser-mode login on an SSH session
    // prints a `--no-browser` hint up front. The predicate (`looks_headless`) is unit-tested;
    // this pins the actual EMISSION at the process boundary. The callback never comes
    // (`--port 0`, no browser), so we read stderr for the hint with a deadline, then kill —
    // never waiting out the 5-minute callback timeout.
    let dir = tempfile::tempdir().unwrap();
    let store = TokenStore::with_dir(dir.path().join("oura-toolkit"));
    store
        .save_credentials(&ClientCredentials {
            client_id: "cid".into(),
            client_secret: "sec".into(),
        })
        .unwrap();

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
    base_env(&mut cmd, dir.path());
    let mut child = cmd
        .args(["auth", "login", "--port", "0"])
        .env("SSH_CONNECTION", "1.2.3.4 5 6.7.8.9 22")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn oura auth login");
    let stderr = BufReader::new(child.stderr.take().unwrap());

    let reader = std::thread::spawn(move || {
        for line in stderr.lines() {
            let Ok(line) = line else { break };
            if line.contains("SSH") && line.contains("--no-browser") {
                return true;
            }
        }
        false
    });
    let deadline = Instant::now() + Duration::from_secs(15);
    while !reader.is_finished() {
        if Instant::now() > deadline {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    // Kill regardless: found → we're done; not found → unblock the reader off the pipe EOF.
    let _ = child.kill();
    let _ = child.wait();
    assert!(
        reader.join().unwrap_or(false),
        "browser-mode login on an SSH session must suggest `--no-browser` on stderr"
    );
}
