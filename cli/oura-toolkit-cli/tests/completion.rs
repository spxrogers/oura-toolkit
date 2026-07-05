//! Binary-level contract for the `oura completion <shell>` and `oura man` code generators (#27).
//! They emit to stdout (the script / man page IS the result — see docs/cli-contract.md → Streams),
//! take no auth and no network, and reject an unknown shell with clap's usage exit 2. Asserted by
//! spawning the real `oura` binary, like the rest of cli/oura-toolkit-cli/tests.

use assert_cmd::Command;
use predicates::prelude::*;

fn oura() -> Command {
    // No store/env isolation needed: these commands never load tokens or config.
    let mut cmd = Command::cargo_bin("oura").expect("oura binary");
    cmd.env("NO_COLOR", "1");
    cmd
}

#[test]
fn completion_zsh_emits_a_loadable_zsh_script_to_stdout() {
    oura()
        .args(["completion", "zsh"])
        .assert()
        .success()
        // `#compdef oura` is the zsh completion-system directive; `_oura` is the generated widget.
        // Both must be present for `compinit` to actually wire up completion.
        .stdout(predicate::str::contains("#compdef oura").and(predicate::str::contains("_oura")))
        .stderr(predicate::str::is_empty());
}

#[test]
fn completion_bash_emits_a_loadable_bash_script_to_stdout() {
    oura()
        .args(["completion", "bash"])
        .assert()
        .success()
        // The generated bash function plus the `complete` registration that binds it to `oura`.
        .stdout(
            predicate::str::contains("_oura").and(predicate::str::contains("complete -F _oura")),
        )
        .stderr(predicate::str::is_empty());
}

#[test]
fn completion_supports_every_advertised_shell() {
    // The five shells the subcommand's help promises; each must emit a non-empty script.
    for shell in ["bash", "zsh", "fish", "powershell", "elvish"] {
        oura()
            .args(["completion", shell])
            .assert()
            .success()
            .stdout(predicate::str::is_empty().not());
    }
}

#[test]
fn completion_rejects_an_unknown_shell_with_usage_exit_2() {
    // clap value-enum rejection is a usage error: exit 2, error to stderr, stdout stays clean.
    oura()
        .args(["completion", "tcsh"])
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty());
}

#[test]
fn man_emits_a_roff_man_page_for_oura_section_1() {
    oura()
        .arg("man")
        .assert()
        .success()
        // `.TH` is roff's title-header macro; clap_mangen emits `.TH oura 1 …` for the binary.
        .stdout(predicate::str::contains(".TH").and(predicate::str::contains("oura")))
        .stderr(predicate::str::is_empty());
}
