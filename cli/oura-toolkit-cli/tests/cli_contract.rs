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
fn unimplemented_mcp_exits_1_with_nothing_on_stdout() {
    // Stream discipline matters double here: stdout is the future JSON-RPC transport.
    let (mut cmd, _dir) = oura();
    cmd.arg("mcp")
        .assert()
        .code(1)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("not yet implemented"));
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
