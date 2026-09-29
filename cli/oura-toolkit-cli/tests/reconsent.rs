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
//! path. Which commands run the check (data + `oura api`, after argument validation; never
//! `auth *` or the generators) is pinned here too. On Linux, `script(1)` gives the child a
//! real pseudo-terminal, so the INTERACTIVE wiring (both stdin and stderr must be TTYs; the
//! prompt goes to the terminal, never stdout) is exercised end to end; the prompt's branches
//! are covered by `reconsent`'s unit tests. `oura mcp`'s flavour, a note on the first tool
//! result, is
//! pinned here at the process boundary too: `main` must hand the server the store (and must
//! NOT for an env-token server), and the note must ride inside JSON-RPC, never as stray
//! stdout. Hermetic: the Oura host is a loopback wiremock.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

use oura_toolkit_auth::{ClientCredentials, TokenStore, Tokens};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer};

mod common;
use common::{current_grant, page, sleep_doc, PRE_1_41_GRANT};
use oura_toolkit_cli::reconsent::{MCP_NOTICE_LEAD, NOTICE_PREFIX, STATE_FILE};
// Only the Linux PTY tests read the prompt's strings (rule 6: platform-gated tests must not
// leave unused imports on the other CI legs).
#[cfg(target_os = "linux")]
use oura_toolkit_cli::reconsent::{PROMPT_LEAD, PROMPT_QUESTION};

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

    /// `oura sleep --date 2026-06-26` against this fixture.
    fn run(&self, extra_env: &[(&str, &str)]) -> Output {
        self.run_args(&["sleep", "--date", "2026-06-26"], extra_env)
    }

    /// The bookkeeping record's path in this fixture's store.
    fn state_file(&self) -> std::path::PathBuf {
        self.dir.path().join("oura-toolkit").join(STATE_FILE)
    }

    /// Point `cmd` at this fixture: the isolated store, the mock Oura, no color, and no
    /// ambient `OURA_ACCESS_TOKEN` or SSH markers leaking in from the test runner.
    fn isolate(&self, cmd: &mut Command) {
        cmd.env("XDG_CONFIG_HOME", self.dir.path())
            .env("HOME", self.dir.path())
            .env("LOCALAPPDATA", self.dir.path())
            .env("NO_COLOR", "1")
            .env_remove("OURA_ACCESS_TOKEN")
            .env_remove("SSH_CONNECTION")
            .env_remove("SSH_TTY")
            .env("OURA_API_BASE_URL", self.server.uri());
    }

    fn run_args(&self, args: &[&str], extra_env: &[(&str, &str)]) -> Output {
        let _guard = self.rt.enter();
        let mut cmd = Command::new(assert_cmd::cargo::cargo_bin("oura"));
        self.isolate(&mut cmd);
        cmd.args(args);
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
        self.isolate(&mut cmd);
        cmd.arg("mcp")
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
        texts[1].starts_with(MCP_NOTICE_LEAD) && texts[1].contains("oura auth login"),
        "`main` must wire the store into `oura mcp`: {result}"
    );
}

#[test]
fn oura_mcp_with_an_env_token_never_notes() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let result = fx.mcp_tool_call(&[("OURA_ACCESS_TOKEN", "env-tok")]);
    assert_ne!(
        result["isError"], true,
        "a real success, not an error: {result}"
    );
    let texts = content_texts(&result);
    assert_eq!(texts.len(), 1, "data only, no note: {result}");
    assert!(texts[0].contains("2026-06-26"), "{result}");
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
        !stdout.contains(NOTICE_PREFIX),
        "the notice is prose — stderr only (contract → Streams): {stdout}"
    );
    assert_eq!(
        stderr.lines().count(),
        1,
        "exactly one notice line: {stderr}"
    );
    assert!(
        stderr.starts_with(NOTICE_PREFIX) && stderr.contains("spo2 heart_health"),
        "the notice names the missing scopes: {stderr}"
    );
    assert!(stderr.contains("oura auth login"), "and the fix: {stderr}");

    let second = fx.run(&[]);
    assert!(second.status.success());
    assert_eq!(text(&second.stderr), "", "shown once per scope set");
}

#[test]
fn a_current_login_is_never_noticed() {
    let fx = Fixture::new(Some(&current_grant()));
    let out = fx.run(&[]);
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(text(&out.stdout).contains("2026-06-26"), "a real result");
    assert_eq!(text(&out.stderr), "");
}

#[test]
fn an_env_token_run_bypasses_the_store_and_the_notice() {
    // OURA_ACCESS_TOKEN never touches the store, so a stale stored grant is irrelevant.
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let out = fx.run(&[("OURA_ACCESS_TOKEN", "env-tok")]);
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(text(&out.stdout).contains("2026-06-26"), "a real result");
    assert_eq!(text(&out.stderr), "");
}

#[test]
fn oura_api_is_store_backed_and_gets_the_notice() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let out = fx.run_args(&["api", "/v2/usercollection/daily_sleep"], &[]);
    let stderr = text(&out.stderr);
    assert!(out.status.success(), "{stderr}");
    assert!(
        text(&out.stdout).contains("2026-06-26"),
        "the raw JSON result"
    );
    assert!(
        stderr.starts_with(NOTICE_PREFIX),
        "`oura api` runs the check: {stderr}"
    );
}

#[test]
fn account_commands_and_generators_never_run_the_check() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    for args in [
        &["auth", "status"][..],
        &["auth", "token"],
        &["completion", "bash"],
        &["man"],
    ] {
        let out = fx.run_args(args, &[]);
        assert!(out.status.success(), "{args:?}: {}", text(&out.stderr));
        assert!(
            !text(&out.stderr).contains(NOTICE_PREFIX),
            "{args:?} must not run the re-consent check: {}",
            text(&out.stderr)
        );
    }
    assert!(
        !fx.state_file().exists(),
        "nothing was told, so nothing recorded"
    );
}

#[test]
fn a_bad_argument_fails_before_the_check_runs() {
    // A usage error must be exactly that (exit 2) — never preceded by a notice (or, on a TTY,
    // a prompt and a browser login) the user didn't need. Covers both preflights: a data
    // command's date window and each of `oura api`'s own usage checks.
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    for args in [
        &["sleep", "--date", "not-a-date"][..],
        &[
            "api",
            "/v2/usercollection/daily_sleep",
            "-f",
            "no-equals-sign",
        ],
        &[
            "api",
            "/v2/usercollection/daily_sleep",
            "-X",
            "NOT A METHOD",
        ],
        &["api", "/v2/x", "-X", "POST", "--paginate"],
    ] {
        let out = fx.run_args(args, &[]);
        let stderr = text(&out.stderr);
        assert_eq!(
            out.status.code(),
            Some(2),
            "{args:?} is a usage error: {stderr}"
        );
        assert!(!stderr.contains(NOTICE_PREFIX), "{args:?}: {stderr}");
    }
    assert!(!fx.state_file().exists(), "the check never ran");
}

/// Run `command` (a shell line) under `script(1)`, which gives it a real pseudo-terminal for
/// stdin/stdout/stderr, feeding `input` as the user's keystrokes. The line reaches the binary
/// and any output files through `$OURA_BIN` / `$OUT` (quoted in the line; nothing is
/// interpolated), so unusual temp paths can't break it. Returns the terminal transcript.
/// Linux-only: util-linux `script` (on every CI Linux runner).
#[cfg(target_os = "linux")]
fn on_a_terminal(fx: &Fixture, command: &str, input: &str, extra_env: &[(&str, &str)]) -> String {
    let _guard = fx.rt.enter();
    let mut cmd = Command::new("script");
    fx.isolate(&mut cmd);
    cmd.args(["-qec", command, "/dev/null"])
        .env("OURA_BIN", assert_cmd::cargo::cargo_bin("oura"))
        .env("OUT", fx.dir.path().join("captured.txt"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    let mut child = cmd
        .spawn()
        .expect("util-linux `script` must be installed to exercise the TTY path");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() > deadline {
            let _ = child.kill();
            panic!("hung on the terminal: a prompt was answered with a login, or deadlocked");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success(), "{}", text(&out.stderr));
    text(&out.stdout)
}

/// What the command under `on_a_terminal` redirected into `$OUT`.
#[cfg(target_os = "linux")]
fn captured(fx: &Fixture) -> String {
    std::fs::read_to_string(fx.dir.path().join("captured.txt")).unwrap()
}

#[cfg(target_os = "linux")]
#[test]
fn on_a_terminal_the_prompt_goes_to_the_terminal_and_the_result_to_stdout() {
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let transcript = on_a_terminal(
        &fx,
        r#""$OURA_BIN" sleep --date 2026-06-26 > "$OUT""#,
        "n\n",
        &[],
    );
    assert!(
        transcript.contains(PROMPT_LEAD) && transcript.contains(PROMPT_QUESTION),
        "stdin+stderr are TTYs, so the prompt must show on the terminal: {transcript:?}"
    );
    assert!(
        transcript.contains("your browser"),
        "not SSH: {transcript:?}"
    );
    assert!(transcript.contains("Skipped"), "{transcript:?}");
    let stdout = captured(&fx);
    assert!(
        stdout.contains("2026-06-26"),
        "the command still ran: {stdout:?}"
    );
    assert!(
        !stdout.contains(PROMPT_QUESTION),
        "the prompt is prose — never on stdout: {stdout:?}"
    );
    let state = std::fs::read_to_string(fx.state_file()).unwrap();
    assert!(state.contains("declined"), "the n was remembered: {state}");
}

#[cfg(target_os = "linux")]
#[test]
fn a_terminal_stdin_with_redirected_stderr_never_prompts() {
    // `oura sleep 2>log` on a terminal: a prompt would block on a question the user can't
    // see. Both streams must be TTYs; otherwise it's the one-line notice. The keystrokes
    // would ACCEPT a prompt (and hang on a browser login), so a prompt fails this loudly.
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let transcript = on_a_terminal(
        &fx,
        r#""$OURA_BIN" sleep --date 2026-06-26 2> "$OUT""#,
        "y\n",
        &[],
    );
    let stderr = captured(&fx);
    assert!(stderr.starts_with(NOTICE_PREFIX), "{stderr:?}");
    assert!(
        !stderr.contains(PROMPT_QUESTION) && !transcript.contains(PROMPT_QUESTION),
        "no prompt without a stderr TTY: {stderr:?} / {transcript:?}"
    );
    assert!(
        transcript.contains("2026-06-26"),
        "the command ran: {transcript:?}"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn a_terminal_stderr_with_redirected_stdin_never_prompts() {
    // The other half of "both must be TTYs": stdin from /dev/null can't answer a prompt.
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let transcript = on_a_terminal(
        &fx,
        r#""$OURA_BIN" sleep --date 2026-06-26 < /dev/null"#,
        "",
        &[],
    );
    assert!(transcript.contains(NOTICE_PREFIX), "{transcript:?}");
    assert!(!transcript.contains(PROMPT_QUESTION), "{transcript:?}");
    assert!(
        transcript.contains("2026-06-26"),
        "the command ran: {transcript:?}"
    );
}

#[cfg(target_os = "linux")]
#[test]
fn over_ssh_yes_runs_the_paste_back_login_without_deadlocking_and_a_bad_paste_continues() {
    // The SSH path end to end: the prompt names the paste-back flow, "y" starts it, and its
    // OWN stdin read must work, which deadlocks if the prompt still holds the stdin lock.
    // A garbage paste fails the login; the command must then carry on with the old login.
    let fx = Fixture::new(Some(PRE_1_41_GRANT));
    let transcript = on_a_terminal(
        &fx,
        r#""$OURA_BIN" sleep --date 2026-06-26 > "$OUT""#,
        "y\nnot-a-redirect-url\n",
        &[("SSH_CONNECTION", "10.0.0.1 5000 10.0.0.2 22")],
    );
    assert!(
        transcript.contains("paste-back"),
        "SSH is detected: {transcript:?}"
    );
    assert!(
        transcript.contains("Paste the full redirect URL"),
        "the paste-back login ran (and could read stdin): {transcript:?}"
    );
    assert!(
        transcript.contains("didn't complete") && transcript.contains("Continuing"),
        "a failed paste reports and carries on: {transcript:?}"
    );
    assert!(
        captured(&fx).contains("2026-06-26"),
        "the command still ran"
    );
}
