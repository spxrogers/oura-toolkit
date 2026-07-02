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
/// Authentication required (exit 4) — scriptable, cf. `gh` exit 4.
pub const EXIT_AUTH: u8 = 4;

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
pub fn render_report(err: &anyhow::Error) -> String {
    let failure = classify(err);
    match failure.hint {
        Some(hint) => format!("{}\nhint: {hint}", render_error(err)),
        None => render_error(err),
    }
}

/// Report a failure to stderr per the contract and return the process exit code.
pub fn report(err: anyhow::Error) -> ExitCode {
    eprintln!("{}", render_report(&err));
    ExitCode::from(classify(&err).code)
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
