//! Binary-level enforcement of the `oura mcp` stdio contract: spawn the REAL binary,
//! speak JSON-RPC over its stdin/stdout, and assert stream discipline at the process
//! boundary — stdout carries NOTHING but JSON-RPC (CLAUDE.md → MCP; it is the transport),
//! initialize succeeds on a machine with no tokens, and EOF on stdin shuts the server
//! down cleanly.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::Duration;

/// Spawn `oura mcp` with an empty, isolated config dir (no tokens anywhere).
fn spawn_mcp(dir: &tempfile::TempDir) -> std::process::Child {
    Command::new(assert_cmd::cargo::cargo_bin("oura"))
        .arg("mcp")
        .env("XDG_CONFIG_HOME", dir.path())
        .env("HOME", dir.path())
        .env("LOCALAPPDATA", dir.path())
        .env("NO_COLOR", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn oura mcp")
}

#[test]
fn stdio_speaks_only_json_rpc_and_shuts_down_on_eof() {
    let dir = tempfile::tempdir().unwrap();
    let mut child = spawn_mcp(&dir);
    let mut stdin = child.stdin.take().unwrap();
    let stdout = BufReader::new(child.stdout.take().unwrap());

    // MCP handshake: initialize → (response) → initialized → tools/list → (response).
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"protocolVersion":"2025-06-18","capabilities":{{}},"clientInfo":{{"name":"contract-test","version":"0"}}}}}}"#
    )
    .unwrap();
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","method":"notifications/initialized"}}"#
    )
    .unwrap();
    writeln!(
        stdin,
        r#"{{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{{}}}}"#
    )
    .unwrap();

    let mut initialize_response = None;
    let mut tools_response = None;
    for line in stdout.lines() {
        let line = line.expect("read stdout line");
        // THE stream-discipline assertion: every stdout line is JSON-RPC, nothing else.
        let message: serde_json::Value = serde_json::from_str(&line)
            .unwrap_or_else(|e| panic!("non-JSON bytes on the MCP transport ({e}): {line:?}"));
        assert_eq!(message["jsonrpc"], "2.0", "not JSON-RPC: {line:?}");
        match message["id"].as_i64() {
            Some(1) => initialize_response = Some(message),
            Some(2) => {
                tools_response = Some(message);
                break;
            }
            _ => {} // server-initiated notifications are fine — they parsed as JSON-RPC
        }
    }

    // Initialize succeeded WITHOUT tokens (CLAUDE.md: auth failures surface on tool
    // calls, never the handshake).
    let init = initialize_response.expect("initialize response");
    assert_eq!(init["result"]["serverInfo"]["name"], "oura-toolkit");
    assert!(
        init["result"]["instructions"]
            .as_str()
            .unwrap_or_default()
            .contains("oura auth login"),
        "instructions name the out-of-band auth flow"
    );

    let tools = tools_response.expect("tools/list response");
    let listed = tools["result"]["tools"].as_array().expect("tools array");
    assert_eq!(listed.len(), 8, "exactly the curated tool surface");

    // EOF on stdin = client gone → clean shutdown, exit 0.
    drop(stdin);
    let status = wait_with_timeout(&mut child, Duration::from_secs(10));
    assert!(
        status.success(),
        "clean exit after client disconnect: {status:?}"
    );
}

/// Poll-wait so a hung server fails the test instead of hanging CI.
fn wait_with_timeout(
    child: &mut std::process::Child,
    timeout: Duration,
) -> std::process::ExitStatus {
    let deadline = std::time::Instant::now() + timeout;
    loop {
        if let Some(status) = child.try_wait().expect("try_wait") {
            return status;
        }
        if std::time::Instant::now() > deadline {
            let _ = child.kill();
            panic!("oura mcp did not exit within {timeout:?} after stdin EOF");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
