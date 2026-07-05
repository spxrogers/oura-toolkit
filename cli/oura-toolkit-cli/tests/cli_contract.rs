//! Binary-level enforcement of the CLI contract (`docs/cli-contract.md`): exit codes,
//! stream discipline, and error/hint shape, asserted by spawning the real `oura` binary.
//!
//! Unit tests pin the classifier and renderers; these pin the process boundary — the part
//! scripts actually see. Hermetic: every invocation gets an empty, throwaway config dir on
//! every platform (XDG for Unix, LOCALAPPDATA for Windows, HOME as the XDG fallback).

use assert_cmd::Command;
use predicates::prelude::*;

/// The `oura` binary pointed at an empty, isolated config dir (returned to keep it alive).
fn oura() -> (Command, tempfile::TempDir) {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut cmd = Command::cargo_bin("oura").expect("oura binary");
    cmd.env("XDG_CONFIG_HOME", dir.path())
        .env("HOME", dir.path())
        .env("LOCALAPPDATA", dir.path())
        .env("NO_COLOR", "1");
    (cmd, dir)
}

#[test]
fn login_without_credentials_exits_4_with_hint_on_stderr() {
    let (mut cmd, _dir) = oura();
    cmd.args(["auth", "login"])
        .assert()
        .code(4)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains(
            "oura: no client credentials stored",
        ))
        .stderr(predicate::str::contains("hint: run `oura auth setup`"));
}

#[test]
fn auth_setup_walkthrough_and_prompt_go_to_stderr_not_stdout() {
    // #37: interactive auth prose must obey the stream contract (docs/cli-contract.md →
    // Streams: prose/prompts → stderr, stdout is results-only). It historically went to
    // stdout via `println!`. `auth setup` prints its walkthrough, then blocks on the
    // credential prompt; assert_cmd gives the child an empty stdin, so the prompt read hits
    // EOF and the command exits 1. stdout MUST stay empty; the guidance AND the prompt label
    // MUST land on stderr. `--port 0` binds an ephemeral loopback port so the listener never
    // collides with a real 8788.
    //
    // This reaches only the PRE-credential prose (EOF stops at the Client ID prompt); the
    // post-credential lines need a full OAuth callback + token exchange, which isn't
    // hermetic. `auth_module_never_writes_prose_to_stdout` (below) guards every site,
    // including those, at the source level. Break-verify: turn the header `guide(...)` back
    // into `println!` and the stdout-empty assertion fails.
    let (mut cmd, _dir) = oura();
    cmd.args(["auth", "setup", "--port", "0"])
        .assert()
        .code(1)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains(
            "Register your Oura OAuth application",
        ))
        .stderr(predicate::str::contains("Client ID:"));
}

#[test]
fn auth_module_never_writes_prose_to_stdout() {
    // #37 guard for the prose sites the binary test above cannot reach without a full OAuth
    // callback + exchange (the post-credential "✓ Credentials saved" / "Continuing to
    // login…" / "Opening your browser…" / "✓ Done" lines). The auth module must NEVER write
    // to stdout: command results are RETURNED and emitted by main.rs via contract::emit;
    // everything the module prints itself is human-facing → stderr (guide()/contract::inform/
    // eprintln!). Mechanized like the repo's other tripwires (mcp tool names, README claims).
    //
    // Scrub the stderr `eprintln!(`/`eprint!(` macros first — "println!(" is a substring of
    // "eprintln!(" — then any surviving `println!(`/`print!(` is a real stdout write. Break-
    // verify: add `println!("x");` to auth.rs → this fails; an `eprintln!` alone does not.
    let src = include_str!("../src/auth.rs");
    let scrubbed = src.replace("eprintln!(", "«e»").replace("eprint!(", "«e»");
    assert!(
        !scrubbed.contains("println!(") && !scrubbed.contains("print!("),
        "auth.rs writes prose to stdout — route it through guide()/contract::inform/eprintln! \
         so stdout stays results-only (docs/cli-contract.md → Streams)"
    );
}

#[test]
fn data_command_without_tokens_exits_4_with_login_hint() {
    // The everyday failure mode (#9): a data command before `oura auth login`. The typed
    // NotAuthenticated must cross the process boundary as exit 4 + the login hint, with
    // stdout untouched — even when --json was requested.
    let (mut cmd, _dir) = oura();
    cmd.args(["sleep", "--json"])
        .assert()
        .code(4)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains(
            "oura: fetching daily sleep: not authenticated",
        ))
        .stderr(predicate::str::contains("hint: run `oura auth login`"));
}

#[test]
fn data_command_rejects_an_inverted_date_range() {
    // Range validation happens before any network/auth work: usage-shaped, exit 2.
    let (mut cmd, _dir) = oura();
    cmd.args(["sleep", "--start", "today", "--end", "yesterday"])
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("after"));
}

#[test]
fn data_command_rejects_date_combined_with_a_range() {
    // #39: --date is a single-day shorthand, mutually exclusive with --start/--end. Combining
    // them is a usage error (exit 2) caught before any network/auth work.
    let (mut cmd, _dir) = oura();
    cmd.args(["sleep", "--date", "today", "--start", "yesterday"])
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("cannot be combined"));
}

#[test]
fn data_command_rejects_a_malformed_date() {
    // The other UsageError path docs/cli-contract.md names explicitly: a value clap
    // can't validate is still exit 2, with the accepted forms spelled out.
    let (mut cmd, _dir) = oura();
    cmd.args(["sleep", "--start", "not-a-date"])
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("today, yesterday, or YYYY-MM-DD"));
}

#[test]
fn mcp_with_no_client_exits_cleanly_and_writes_nothing_to_stdout() {
    // Stream discipline matters double here: stdout is the JSON-RPC transport, and the
    // server must not write to it unprompted. assert_cmd closes stdin immediately →
    // instant EOF → clean shutdown, silent stdout. (The full handshake is exercised in
    // tests/mcp_stdio.rs.)
    let (mut cmd, _dir) = oura();
    cmd.arg("mcp")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}

#[test]
fn bare_invocation_is_a_usage_error() {
    let (mut cmd, _dir) = oura();
    cmd.assert().code(2).stdout(predicate::str::is_empty());
}

#[test]
fn unknown_flag_is_a_usage_error() {
    let (mut cmd, _dir) = oura();
    cmd.args(["auth", "login", "--bogus"])
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty());
}

#[test]
fn lone_global_flag_prints_help_to_stderr_and_exits_2() {
    // Pins the PR #34 review-loop fix: `oura --json` parses (so arg_required_else_help
    // does not fire) but names no command — help must go to STDERR, stdout stays
    // results-only, and the exit code matches clap's usage errors.
    let (mut cmd, _dir) = oura();
    cmd.arg("--json")
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn help_and_version_exit_0() {
    let (mut cmd, _dir) = oura();
    cmd.arg("--help").assert().success();
    let (mut cmd, _dir) = oura();
    cmd.arg("--version").assert().success();
}
