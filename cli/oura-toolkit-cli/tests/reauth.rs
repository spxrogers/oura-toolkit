//! Binary-level enforcement of the re-consent notice (#116): spawn the REAL `oura` binary
//! with a saved login that predates a scope change, and pin the non-interactive contract
//! (docs/cli-contract.md → Scope changes):
//!
//! - the command still SUCCEEDS with its normal stdout result (the notice never blocks),
//! - exactly ONE `oura: note:` line naming the missing scopes goes to stderr,
//! - it is shown once per scope set (a second run is silent), and
//! - a current grant, or an `OURA_ACCESS_TOKEN` run, never sees it.
//!
//! A spawned child's stdin/stderr are pipes, never TTYs, so this is exactly the scripted
//! path. The interactive prompt (Y → login, n → remembered) is covered by `reauth`'s unit
//! tests through injected IO. `oura mcp`'s flavour, a note on the first tool result, is
//! pinned here at the process boundary too: `main` must hand the server the store (and must
//! NOT for an env-token server), and the note must ride inside JSON-RPC, never as stray
//! stdout. Hermetic: the Oura host is a loopback wiremock.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use oura_toolkit_auth::{metadata, ClientCredentials, TokenStore, Tokens};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer};

mod common;
use common::{page, sleep_doc};

const PRE_1_41_GRANT: &str = "personal daily heartrate workout tag session spo2Daily";

struct Fixture {
    server: MockServer,
    rt: tokio::runtime::Runtime,
    dir: tempfile::TempDir,
}

impl Fixture {
    /// A mock Oura serving one sleep doc, and a store holding a login with `scope`.
    fn new(scope: Option<&str>) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let server = rt.block_on(async {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/v2/usercollection/daily_sleep"))
                .respond_with(page(vec![sleep_doc("2026-06-26", 77)], None))
                .mount(&server)
                .await;
            server
        });
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path().join("oura-toolkit"));
        store
            .save_credentials(&ClientCredentials {
                client_id: "cid".into(),
                client_secret: "sec".into(),
            })
            .unwrap();
        store
            .save_tokens(&Tokens {
                access_token: "at".into(),
                refresh_token: "rt".into(),
                expires_at: 4_102_444_800, // 2100: never refreshed, so no token endpoint
                scope: scope.map(Into::into),
                token_type: None,
            })
            .unwrap();
        Fixture { server, rt, dir }
    }

    fn run(&self, extra_env: &[(&str, &str)]) -> Output {
        let _guard = self.rt.enter();
        let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
        cmd.env("XDG_CONFIG_HOME", self.dir.path())
            .env("HOME", self.dir.path())
            .env("LOCALAPPDATA", self.dir.path())
            .env("NO_COLOR", "1")
            .env_remove("OURA_ACCESS_TOKEN")
            .env("OURA_API_BASE_URL", self.server.uri())
            .args(["sleep", "--date", "2026-06-26"]);
        for (k, v) in extra_env {
            cmd.env(k, v);
        }
        cmd.output().expect("spawn oura")
    }
}

impl Fixture {
    /// Spawn `oura mcp` against this store, do the handshake plus ONE `get_daily_sleep` call,
    /// and return that call's `result`. Every stdout line must be JSON-RPC.
    fn mcp_tool_call(&self, extra_env: &[(&str, &str)]) -> serde_json::Value {
        let _guard = self.rt.enter();
        let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
        cmd.env("XDG_CONFIG_HOME", self.dir.path())
            .env("HOME", self.dir.path())
            .env("LOCALAPPDATA", self.dir.path())
            .env_remove("OURA_ACCESS_TOKEN")
            .env("OURA_API_BASE_URL", self.server.uri())
            .arg("mcp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        for (k, v) in extra_env {
            cmd.env(k, v);
        }
        let mut child = cmd.spawn().expect("spawn oura mcp");
        let mut stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        for msg in [
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#,
            r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_daily_sleep","arguments":{"date":"2026-06-26"}}}"#,
        ] {
            writeln!(stdin, "{msg}").unwrap();
        }
        let reader = std::thread::spawn(move || {
            for line in stdout.lines() {
                let line = line.expect("read stdout line");
                let msg: serde_json::Value = serde_json::from_str(&line)
                    .unwrap_or_else(|e| panic!("non-JSON on the MCP transport ({e}): {line:?}"));
                if msg["id"].as_i64() == Some(2) {
                    return Some(msg);
                }
            }
            None
        });
        let deadline = Instant::now() + Duration::from_secs(20);
        while !reader.is_finished() {
            if Instant::now() > deadline {
                let _ = child.kill();
                panic!("oura mcp did not answer the tool call within 20s");
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        let call = reader.join().expect("reader").expect("tools/call response");
        drop(stdin);
        let _ = child.wait();
        call["result"].clone()
    }
}

fn content_texts(result: &serde_json::Value) -> Vec<String> {
    result["content"]
        .as_array()
        .expect("content array")
        .iter()
        .map(|b| b["text"].as_str().unwrap_or_default().to_owned())
        .collect()
}

#[test]
fn oura_mcp_notes_a_stale_grant_on_the_tool_result() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let result = fx.mcp_tool_call(&[]);
    assert_ne!(result["isError"], true, "{result}");
    let texts = content_texts(&result);
    assert_eq!(texts.len(), 2, "data block + note block: {result}");
    assert!(
        texts[0].contains("2026-06-26"),
        "block 0 is the data: {result}"
    );
    assert!(
        texts[1].starts_with(oura_toolkit_cli::reauth::MCP_NOTICE_LEAD)
            && texts[1].contains("oura auth login"),
        "`main` must wire the store into `oura mcp`: {result}"
    );
}

#[test]
fn oura_mcp_with_an_env_token_never_notes() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let result = fx.mcp_tool_call(&[("OURA_ACCESS_TOKEN", "env-tok")]);
    assert_eq!(content_texts(&result).len(), 1, "{result}");
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn a_stale_login_gets_one_stderr_notice_and_the_command_still_succeeds() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));

    let first = fx.run(&[]);
    let (stdout, stderr) = (text(&first.stdout), text(&first.stderr));
    assert!(
        first.status.success(),
        "the notice must never block: {stderr}"
    );
    assert!(
        stdout.contains("2026-06-26"),
        "the result still reaches stdout: {stdout}"
    );
    assert!(
        !stdout.contains("oura: note"),
        "the notice is prose — stderr only (contract → Streams): {stdout}"
    );
    assert_eq!(
        stderr.lines().count(),
        1,
        "exactly one notice line: {stderr}"
    );
    assert!(
        stderr.starts_with("oura: note:") && stderr.contains("spo2 heart_health"),
        "the notice names the missing scopes: {stderr}"
    );
    assert!(stderr.contains("oura auth login"), "and the fix: {stderr}");

    let second = fx.run(&[]);
    assert!(second.status.success());
    assert_eq!(text(&second.stderr), "", "shown once per scope set");
}

#[test]
fn a_current_login_is_never_noticed() {
    let fx = Fixture::new(Some(&metadata::default_scopes().join(" ")));
    let out = fx.run(&[]);
    assert!(out.status.success());
    assert_eq!(text(&out.stderr), "");
}

#[test]
fn an_env_token_run_bypasses_the_store_and_the_notice() {
    // OURA_ACCESS_TOKEN never touches the store, so a stale stored grant is irrelevant.
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let out = fx.run(&[("OURA_ACCESS_TOKEN", "env-tok")]);
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert_eq!(text(&out.stderr), "");
}
