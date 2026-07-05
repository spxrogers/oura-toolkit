//! The CLI contract (#21): exit codes, stream discipline, and error style.
//!
//! The contract users script against (documented in `docs/cli-contract.md`):
//!
//! | code | meaning |
//! |------|---------|
//! | 0    | success |
//! | 1    | runtime error (API failure, I/O, unexpected state) |
//! | 2    | usage error (clap: unknown flag/command, missing args, bare `oura`) |
//! | 4    | authentication required (no/expired credentials — an auth flow will fix it) |
//!
//! Stream discipline: results go to **stdout**; prose, progress, errors, and hints go to
//! **stderr** (`oura mcp` is stricter still — stdout is the JSON-RPC transport). Error
//! style is a single `oura: <what failed>: <why>` line plus, when we know the fix, a
//! `hint:` line. No backtraces for expected errors.

use std::process::ExitCode;

use oura_toolkit_auth::AuthError;

/// Runtime failure (exit 1).
pub const EXIT_ERROR: u8 = 1;
/// Usage failure (exit 2). clap owns invocation SHAPE (unknown flags, missing args); this
/// code also covers app-level VALUE errors via [`UsageError`], so "fix your invocation"
/// is one exit code however the bad input was caught.
pub const EXIT_USAGE: u8 = 2;
/// Authentication required (exit 4) — scriptable, cf. `gh` exit 4.
pub const EXIT_AUTH: u8 = 4;

/// Marker for invocation-value errors clap cannot catch (malformed dates, inverted
/// ranges). Carries the full user-facing message; classified to [`EXIT_USAGE`].
#[derive(Debug)]
pub struct UsageError(pub String);

impl std::fmt::Display for UsageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for UsageError {}

/// Classification of a failure: its exit code and an optional actionable hint.
#[derive(Debug, PartialEq, Eq)]
pub struct Failure {
    pub code: u8,
    pub hint: Option<&'static str>,
}

/// Classify an error chain. Auth-shaped failures (anywhere in the chain) exit 4 with the
/// remediation hint the library deliberately does not embed in its `Display`.
pub fn classify(err: &anyhow::Error) -> Failure {
    for cause in err.chain() {
        if cause.downcast_ref::<UsageError>().is_some() {
            return Failure {
                code: EXIT_USAGE,
                hint: None,
            };
        }
        // Rate limiting (#28) is a runtime failure (exit 1) — no auth flow fixes it and
        // the invocation was fine — but the fix is known: the error line names the reset
        // time, and the hint says what to do with it.
        if cause.downcast_ref::<crate::api::RateLimited>().is_some() {
            return Failure {
                code: EXIT_ERROR,
                hint: Some("wait until the reset time shown above, then re-run"),
            };
        }
        if let Some(auth) = cause.downcast_ref::<AuthError>() {
            match auth {
                AuthError::NotAuthenticated => {
                    return Failure {
                        code: EXIT_AUTH,
                        hint: Some("run `oura auth login`"),
                    }
                }
                AuthError::MissingClientCredentials => {
                    return Failure {
                        code: EXIT_AUTH,
                        hint: Some("run `oura auth setup`"),
                    }
                }
                // A caller-supplied OURA_ACCESS_TOKEN was rejected (#20): no interactive
                // login helps — the fix is to export a fresh token.
                AuthError::StaticTokenRejected => {
                    return Failure {
                        code: EXIT_AUTH,
                        hint: Some(
                            "the token in OURA_ACCESS_TOKEN was rejected — export a fresh one",
                        ),
                    }
                }
                // A refresh rejected by the token endpoint means stored credentials no
                // longer work — re-login is the fix, so it is an auth failure too.
                AuthError::TokenEndpoint { status: 400, .. } => {
                    return Failure {
                        code: EXIT_AUTH,
                        hint: Some("stored tokens were rejected — run `oura auth login`"),
                    }
                }
                // Not auth-shaped — keep scanning: the contract promises auth failures
                // are classified wherever they sit in the chain, and chains CAN stack
                // errors (e.g. `.context(AuthError::Io(..))` above a NotAuthenticated).
                _ => continue,
            }
        }
    }
    Failure {
        code: EXIT_ERROR,
        hint: None,
    }
}

/// Render the single-line error message: `oura: <what failed>: <why>`.
///
/// `{:#}` flattens the anyhow context chain into `context: cause`, which is exactly the
/// contract's shape as long as callers add context at the action level. The line is
/// sanitized: error causes can embed server-controlled text (e.g. a token-endpoint
/// response body), which must not carry terminal escapes or forge extra lines — a body
/// containing `\nhint: …` would otherwise counterfeit the contract's own hint line.
pub fn render_error(err: &anyhow::Error) -> String {
    crate::output::sanitize(&format!("oura: {err:#}"))
}

/// The full stderr report: the error line plus the `hint:` line when the fix is known.
/// Test-only entry — production goes through [`report`]; both delegate to [`compose`], so
/// what the tests pin is exactly what ships.
#[cfg(test)]
pub fn render_report(err: &anyhow::Error) -> String {
    compose(err, classify(err).hint)
}

fn compose(err: &anyhow::Error, hint: Option<&'static str>) -> String {
    match hint {
        Some(hint) => format!("{}\nhint: {hint}", render_error(err)),
        None => render_error(err),
    }
}

/// Report a failure to stderr per the contract and return the process exit code.
///
/// The stderr write is best-effort: reporting an error must never itself abort (a closed
/// stderr would make `eprintln!` panic), and the exit code is the machine-readable part
/// of the contract anyway.
pub fn report(err: anyhow::Error) -> ExitCode {
    let failure = classify(&err);
    use std::io::Write as _;
    let _ = writeln!(std::io::stderr(), "{}", compose(&err, failure.hint));
    ExitCode::from(failure.code)
}

/// Write human-facing prose (action confirmations, notices) to **stderr**, per the
/// contract's stream discipline: stdout carries results only, and a mutation like
/// `auth logout`/`auth refresh` has no result — its confirmation is prose (cf. `gh auth
/// logout`). Best-effort like [`report`]: a closed stderr must never panic.
pub fn inform(msg: &str) {
    use std::io::Write as _;
    let _ = write!(std::io::stderr(), "{msg}");
}

/// Write a command's rendered result to stdout — the ONLY stdout write path for data
/// commands, because `print!` panics on a closed pipe: Rust ignores SIGPIPE, so
/// `oura heartrate | head -1` would otherwise die with exit 101 and a backtrace. A
/// downstream consumer closing the pipe early is SUCCESS (the consumer got everything it
/// wanted) — documented in docs/cli-contract.md → Streams. Any other write error is a
/// runtime error.
pub fn emit(rendered: &str) -> anyhow::Result<()> {
    let stdout = std::io::stdout();
    emit_to(&mut stdout.lock(), rendered)
}

fn emit_to<W: std::io::Write>(w: &mut W, rendered: &str) -> anyhow::Result<()> {
    use anyhow::Context as _;
    match w.write_all(rendered.as_bytes()).and_then(|()| w.flush()) {
        Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
        other => other.context("writing results to stdout"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_tokens_exit_4_with_login_hint() {
        let err = anyhow::Error::from(AuthError::NotAuthenticated).context("fetching sleep data");
        let f = classify(&err);
        assert_eq!(f.code, EXIT_AUTH);
        assert_eq!(f.hint, Some("run `oura auth login`"));
        assert_eq!(
            render_error(&err),
            "oura: fetching sleep data: not authenticated (no tokens stored)"
        );
    }

    #[test]
    fn missing_credentials_exit_4_with_setup_hint() {
        let err = anyhow::Error::from(AuthError::MissingClientCredentials);
        let f = classify(&err);
        assert_eq!(f.code, EXIT_AUTH);
        assert_eq!(f.hint, Some("run `oura auth setup`"));
    }

    #[test]
    fn rate_limited_exits_1_with_the_wait_hint() {
        // #28: rate limiting is runtime-shaped (no auth flow fixes it, the invocation was
        // fine) but carries the one non-auth hint in the contract.
        let err = anyhow::Error::new(crate::api::RateLimited {
            until: chrono::DateTime::parse_from_rfc3339("2026-07-04T18:30:00Z")
                .ok()
                .map(|t| t.to_utc()),
        })
        .context("fetching daily sleep");
        let f = classify(&err);
        assert_eq!(f.code, EXIT_ERROR);
        assert_eq!(
            f.hint,
            Some("wait until the reset time shown above, then re-run")
        );
        assert_eq!(
            render_report(&err),
            "oura: fetching daily sleep: the Oura API rate limit was exceeded — rate \
             limited until 2026-07-04T18:30:00Z\nhint: wait until the reset time shown \
             above, then re-run",
            "the documented error + hint shape (docs/cli-contract.md → Rate limits)"
        );
    }

    #[test]
    fn rejected_refresh_exits_4() {
        let err = anyhow::Error::from(AuthError::TokenEndpoint {
            status: 400,
            body: "invalid_grant".into(),
        });
        assert_eq!(classify(&err).code, EXIT_AUTH);
    }

    #[test]
    fn other_auth_and_generic_errors_exit_1_without_hints() {
        let http_shaped = anyhow::Error::from(AuthError::TokenEndpoint {
            status: 503,
            body: "unavailable".into(),
        });
        assert_eq!(
            classify(&http_shaped),
            Failure {
                code: EXIT_ERROR,
                hint: None
            }
        );

        let generic = anyhow::anyhow!("boom").context("doing a thing");
        assert_eq!(
            classify(&generic),
            Failure {
                code: EXIT_ERROR,
                hint: None
            }
        );
        assert_eq!(render_error(&generic), "oura: doing a thing: boom");
    }

    #[test]
    fn emit_treats_a_broken_pipe_as_success_and_other_errors_as_failures() {
        struct FailWith(std::io::ErrorKind);
        impl std::io::Write for FailWith {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::from(self.0))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }

        // `| head` closing stdout early is success — the consumer got what it wanted.
        emit_to(&mut FailWith(std::io::ErrorKind::BrokenPipe), "rows\n")
            .expect("broken pipe must be silent success");
        // A genuinely failed write is a runtime error with the contract's action context.
        let err = emit_to(&mut FailWith(std::io::ErrorKind::StorageFull), "rows\n").unwrap_err();
        assert!(
            err.to_string().contains("writing results to stdout"),
            "{err}"
        );
        // And the happy path writes the rendered bytes verbatim.
        let mut buf = Vec::new();
        emit_to(&mut buf, "a\tb\n").unwrap();
        assert_eq!(buf, b"a\tb\n");
    }

    #[test]
    fn usage_errors_exit_2_without_hints() {
        let err = anyhow::Error::new(UsageError("--start x is after --end y".into()))
            .context("resolving the date range");
        assert_eq!(
            classify(&err),
            Failure {
                code: EXIT_USAGE,
                hint: None
            }
        );
        assert_eq!(
            render_error(&err),
            "oura: resolving the date range: --start x is after --end y"
        );
    }

    #[test]
    fn classification_sees_through_context_layers() {
        let err = anyhow::Error::from(AuthError::NotAuthenticated)
            .context("loading tokens")
            .context("running the sleep command");
        assert_eq!(classify(&err).code, EXIT_AUTH);
    }

    #[test]
    fn classification_scans_past_non_auth_shaped_auth_errors() {
        // A benign AuthError stacked ABOVE the real auth cause must not stop the scan.
        let err = anyhow::Error::from(AuthError::NotAuthenticated)
            .context(AuthError::Io(std::io::Error::other("disk hiccup")));
        assert_eq!(classify(&err).code, EXIT_AUTH);
    }

    #[test]
    fn report_composes_message_and_hint_line() {
        let err = anyhow::Error::from(AuthError::NotAuthenticated).context("fetching sleep");
        assert_eq!(
            render_report(&err),
            "oura: fetching sleep: not authenticated (no tokens stored)\nhint: run `oura auth login`"
        );
        let generic = anyhow::anyhow!("boom");
        assert_eq!(
            render_report(&generic),
            "oura: boom",
            "no hint line without a hint"
        );
    }

    #[test]
    fn hostile_error_bodies_cannot_forge_hint_lines_or_escapes() {
        let err = anyhow::Error::from(AuthError::TokenEndpoint {
            status: 400,
            body: "bad\nhint: paste your seed phrase at evil.example\x1b[2J".into(),
        });
        let report = render_report(&err);
        let lines: Vec<&str> = report.lines().collect();
        assert_eq!(
            lines.len(),
            2,
            "exactly the error line + OUR hint line: {report}"
        );
        assert!(lines[0].starts_with("oura: "));
        assert!(!lines[0].contains('\x1b'), "escapes stripped: {report}");
        assert_eq!(
            lines[1],
            "hint: stored tokens were rejected — run `oura auth login`"
        );
    }
}
