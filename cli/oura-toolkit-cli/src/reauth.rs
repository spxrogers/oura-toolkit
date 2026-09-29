//! Re-consent prompt for when the toolkit's default scopes outgrow a saved login (#116).
//!
//! Oura renames and adds OAuth scopes between API exports (openapi-1.41: `spo2Daily` →
//! `spo2`, plus a new `heart_health`). A token refresh can't add scopes; only a new consent
//! can. So a login from before such a change quietly lacks them. Detection is
//! **data-driven**: the grant recorded in `tokens.json` vs [`metadata::default_scopes`]
//! (itself spec-derived, via [`metadata::missing_default_scopes`]). Every future scope change
//! is caught with no per-version code.
//!
//! It fires **once per scope-set change**, before a store-backed command (the data commands
//! and `oura api`):
//! - **interactive** (stdin + stderr are TTYs): explain what's missing and ask to
//!   re-authorize. Yes runs `oura auth login` (default port), then the original command
//!   continues on the new tokens. No is remembered for this scope set.
//! - **non-interactive**: one stderr notice, never blocking. It's remembered separately, so
//!   a script running first doesn't use up the human's prompt.
//!
//! `oura mcp` can't prompt (stdout is its transport, and stdio MCP auth is out of band), so
//! it gets the MCP flavour, [`mcp_notice`]: a note on the first tool result of each session,
//! for the model to relay ("run `oura auth login` in a terminal"). An `OURA_ACCESS_TOKEN` run
//! (no store) sees neither. The bookkeeping lives in [`STATE_FILE`] next to the token records.
//! It's CLI-only and holds no secrets.

use std::future::Future;
use std::io::{BufRead, Write};

use oura_toolkit_auth::{metadata, TokenStore, Tokens};
use serde::{Deserialize, Serialize};

/// The bookkeeping record, in the store dir alongside `credentials.json` / `tokens.json`.
pub const STATE_FILE: &str = "scope-notice.json";

/// How the one-line non-interactive notice starts (documented; pinned by docs_tripwire.rs).
pub const NOTICE_PREFIX: &str = "oura: note:";

/// The interactive question (documented; pinned by docs_tripwire.rs).
pub const PROMPT_QUESTION: &str = "Re-authorize now? [Y/n]";

/// How the MCP tool-result note starts (documented; pinned by docs_tripwire.rs).
pub const MCP_NOTICE_LEAD: &str = "Note: Oura changed its API permissions (OAuth scopes)";

/// What the user has already been told, keyed by the space-joined default scope set it was
/// told about — so the next scope change (a different key) asks again.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeState {
    /// The scope set an interactive prompt was declined for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declined: Option<String>,
    /// The scope set a non-interactive run already printed its one notice for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notified: Option<String>,
}

/// What to do before the command runs.
#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Nothing,
    /// Ask to re-authorize; carries the missing default scopes.
    Prompt(Vec<&'static str>),
    /// Print the one-line notice; carries the missing default scopes.
    Notify(Vec<&'static str>),
}

/// The key a [`NoticeState`] entry is recorded under: the current default scope set.
fn scope_set_key() -> String {
    metadata::default_scopes().join(" ")
}

/// The pure decision. No tokens → [`Action::Nothing`] (the command's own
/// not-authenticated error applies; there is no grant to be stale).
pub fn decide(tokens: Option<&Tokens>, state: &NoticeState, interactive: bool) -> Action {
    let Some(tokens) = tokens else {
        return Action::Nothing;
    };
    let missing = metadata::missing_default_scopes(tokens.scope.as_deref());
    if missing.is_empty() {
        return Action::Nothing;
    }
    let key = scope_set_key();
    let declined = state.declined.as_deref() == Some(key.as_str());
    let notified = state.notified.as_deref() == Some(key.as_str());
    match (interactive, declined, notified) {
        // A human who said "no" to this exact set isn't asked again, and scripts don't
        // re-announce what the human already declined.
        (_, true, _) => Action::Nothing,
        (true, false, _) => Action::Prompt(missing),
        (false, false, false) => Action::Notify(missing),
        (false, false, true) => Action::Nothing,
    }
}

/// `y`, `yes`, or a bare Enter (the default) accept; anything else, including EOF, declines.
pub fn parse_answer(line: &str) -> bool {
    matches!(line.trim().to_ascii_lowercase().as_str(), "" | "y" | "yes")
}

fn load_state(store: &TokenStore) -> NoticeState {
    // Best-effort: a missing or corrupt record just means "not told yet". Never fail the
    // user's command over bookkeeping.
    std::fs::read(store.dir().join(STATE_FILE))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default()
}

fn save_state(store: &TokenStore, state: &NoticeState) {
    // Best-effort, like `load_state`: failing to write only means the notice may repeat.
    if let Ok(data) = serde_json::to_vec_pretty(state) {
        let _ = std::fs::write(store.dir().join(STATE_FILE), data);
    }
}

/// Run the check against `store`: decide, then prompt/notify through the injected `input` /
/// `out` (stderr in production), invoking `login` on a yes. Errors only if that login fails.
/// Then the command stops, and nothing is recorded, so the next run asks again.
pub async fn check<L, Fut>(
    store: &TokenStore,
    interactive: bool,
    input: &mut dyn BufRead,
    out: &mut dyn Write,
    login: L,
) -> anyhow::Result<()>
where
    L: FnOnce() -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    // An unreadable token record is the command's own error to report, not ours.
    let Ok(tokens) = store.load_tokens() else {
        return Ok(());
    };
    let mut state = load_state(store);
    let unrecorded = tokens.as_ref().is_some_and(|t| t.scope.is_none());
    match decide(tokens.as_ref(), &state, interactive) {
        Action::Nothing => Ok(()),
        Action::Notify(missing) => {
            let _ =
                writeln!(
                out,
                "{NOTICE_PREFIX} your saved login {} this version's Oura permissions: {} — run \
                 `oura auth login` to re-authorize (shown once).",
                if unrecorded { "may not cover" } else { "doesn't cover" },
                missing.join(" ")
            );
            state.notified = Some(scope_set_key());
            save_state(store, &state);
            Ok(())
        }
        Action::Prompt(missing) => {
            let _ = write!(
                out,
                "Oura's API permissions (OAuth scopes) changed, and your saved login {}: {}\n\
                 A token refresh can't add scopes; a new login (consent) can.\n\
                 If your Oura app doesn't list them yet, add them first at \
                 https://cloud.ouraring.com/oauth/applications\n\
                 (Uses the default port 8788. For --port or --no-browser, answer n and run \
                 `oura auth login …` yourself.)\n\
                 {PROMPT_QUESTION} ",
                if unrecorded {
                    "may not cover"
                } else {
                    "doesn't cover"
                },
                missing.join(" ")
            );
            let _ = out.flush();
            let mut line = String::new();
            let accepted =
                matches!(input.read_line(&mut line), Ok(n) if n > 0) && parse_answer(&line);
            if accepted {
                login().await?;
                // Oura may still grant less than asked (the app's registration lacks a scope,
                // or the user unticked one). Say so and remember it, or the next run would
                // prompt again and loop.
                let now_missing = store
                    .load_tokens()
                    .ok()
                    .flatten()
                    .map(|t| metadata::missing_default_scopes(t.scope.as_deref()))
                    .unwrap_or_default();
                if now_missing.is_empty() {
                    let _ = writeln!(out, "Continuing…");
                } else {
                    let _ = writeln!(
                        out,
                        "Logged in, but Oura didn't grant: {}. Add them to your app at \
                         https://cloud.ouraring.com/oauth/applications, then run `oura auth \
                         login` (not asked again for this change). Continuing…",
                        now_missing.join(" ")
                    );
                    state.declined = Some(scope_set_key());
                    save_state(store, &state);
                }
            } else {
                let _ = writeln!(
                    out,
                    "Skipped. Run `oura auth login` any time to grant them (not asked again \
                     for this change)."
                );
                state.declined = Some(scope_set_key());
                save_state(store, &state);
            }
            Ok(())
        }
    }
}

/// The MCP flavour: the note to append to a tool result, or `None`.
///
/// It uses the same decision as the interactive prompt (`decide(…, true)`): a human is on the
/// other end, just not at a TTY we own. So a scope set the user already declined at the CLI
/// prompt stays quiet here too. It's read-only: "once" is per MCP session, tracked by the
/// server, so each new session reminds once until the user re-consents or declines.
pub fn mcp_notice(store: &TokenStore) -> Option<String> {
    let tokens = store.load_tokens().ok()??;
    let Action::Prompt(missing) = decide(Some(&tokens), &load_state(store), true) else {
        return None;
    };
    let coverage = if tokens.scope.is_none() {
        "may not cover"
    } else {
        "doesn't cover"
    };
    Some(format!(
        "{MCP_NOTICE_LEAD}, and the user's saved login {coverage} them: {}. The data above is \
         still valid. Tell the user once: to grant them, run `oura auth login` in a terminal. \
         If their Oura app doesn't list these scopes yet, they add them first at \
         https://cloud.ouraring.com/oauth/applications. To silence this without \
         re-authorizing, they can answer `n` when any `oura` data command asks in a terminal.",
        missing.join(" ")
    ))
}

/// The production entry point: real stdin/stderr, TTY-detected interactivity, and
/// `oura auth login` on the default port.
pub async fn run(store: &TokenStore) -> anyhow::Result<()> {
    use std::io::IsTerminal as _;
    let interactive = std::io::stdin().is_terminal() && std::io::stderr().is_terminal();
    let stdin = std::io::stdin();
    check(
        store,
        interactive,
        &mut stdin.lock(),
        &mut std::io::stderr(),
        || crate::auth::login(crate::auth::DEFAULT_LOGIN_PORT, false),
    )
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn tokens(scope: Option<&str>) -> Tokens {
        Tokens {
            access_token: "at".into(),
            refresh_token: "rt".into(),
            expires_at: 4_102_444_800,
            scope: scope.map(Into::into),
            token_type: None,
        }
    }

    const PRE_1_41: &str = "personal daily heartrate workout tag session spo2Daily";

    fn store_with(scope: Option<&str>) -> (TokenStore, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&tokens(scope)).unwrap();
        (store, dir)
    }

    /// Run `check` with a scripted stdin; returns (stderr text, whether login was invoked).
    /// The fake login persists a grant covering every default scope, like a real re-consent.
    async fn run_check(store: &TokenStore, interactive: bool, stdin: &str) -> (String, bool) {
        run_check_granting(store, interactive, stdin, &scope_set_key()).await
    }

    /// As [`run_check`], but the fake login persists `granted` as the new grant.
    async fn run_check_granting(
        store: &TokenStore,
        interactive: bool,
        stdin: &str,
        granted: &str,
    ) -> (String, bool) {
        let called = Cell::new(false);
        let mut out = Vec::new();
        check(
            store,
            interactive,
            &mut stdin.as_bytes(),
            &mut out,
            || async {
                called.set(true);
                store.save_tokens(&tokens(Some(granted))).unwrap();
                Ok(())
            },
        )
        .await
        .unwrap();
        (String::from_utf8(out).unwrap(), called.get())
    }

    // --- decide (pure) ---------------------------------------------------------------------

    #[test]
    fn a_current_grant_never_prompts_or_notifies() {
        let t = tokens(Some(&scope_set_key()));
        assert_eq!(
            decide(Some(&t), &NoticeState::default(), true),
            Action::Nothing
        );
        assert_eq!(
            decide(Some(&t), &NoticeState::default(), false),
            Action::Nothing
        );
    }

    #[test]
    fn no_tokens_is_not_this_modules_error() {
        assert_eq!(decide(None, &NoticeState::default(), true), Action::Nothing);
    }

    #[test]
    fn a_stale_grant_prompts_interactively_and_notifies_otherwise() {
        let t = tokens(Some(PRE_1_41));
        let missing = vec!["spo2", "heart_health"];
        assert_eq!(
            decide(Some(&t), &NoticeState::default(), true),
            Action::Prompt(missing.clone())
        );
        assert_eq!(
            decide(Some(&t), &NoticeState::default(), false),
            Action::Notify(missing)
        );
    }

    #[test]
    fn a_decline_silences_this_scope_set_but_not_the_next_change() {
        let t = tokens(Some(PRE_1_41));
        let declined = NoticeState {
            declined: Some(scope_set_key()),
            notified: None,
        };
        assert_eq!(decide(Some(&t), &declined, true), Action::Nothing);
        assert_eq!(decide(Some(&t), &declined, false), Action::Nothing);
        // A decline recorded for an OLDER scope set doesn't cover the current one.
        let stale_decline = NoticeState {
            declined: Some(PRE_1_41.into()),
            notified: None,
        };
        assert!(matches!(
            decide(Some(&t), &stale_decline, true),
            Action::Prompt(_)
        ));
    }

    #[test]
    fn a_script_notice_does_not_use_up_the_humans_prompt() {
        let t = tokens(Some(PRE_1_41));
        let notified = NoticeState {
            declined: None,
            notified: Some(scope_set_key()),
        };
        assert_eq!(decide(Some(&t), &notified, false), Action::Nothing);
        assert!(matches!(
            decide(Some(&t), &notified, true),
            Action::Prompt(_)
        ));
    }

    #[test]
    fn answers_default_to_yes_and_anything_unrecognized_declines() {
        for yes in ["", "\n", "y", "Y\n", "yes", " YES \n"] {
            assert!(parse_answer(yes), "{yes:?} must accept");
        }
        for no in ["n", "no\n", "N", "nope", "later"] {
            assert!(!parse_answer(no), "{no:?} must decline");
        }
    }

    // --- check (IO + persistence) ----------------------------------------------------------

    #[tokio::test]
    async fn yes_runs_login_and_records_nothing() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let (out, logged_in) = run_check(&store, true, "\n").await;
        assert!(logged_in, "Enter (default yes) must run the login");
        assert!(
            out.contains("spo2 heart_health"),
            "prompt must name the missing scopes: {out}"
        );
        assert!(out.contains("[Y/n]"), "{out}");
        assert_eq!(load_state(&store), NoticeState::default());
    }

    #[tokio::test]
    async fn a_login_that_still_grants_too_little_says_so_and_does_not_loop() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        // The user's Oura app lacks heart_health, so the re-consent grants everything else.
        let narrow = "personal daily heartrate workout tag session spo2";
        let (out, logged_in) = run_check_granting(&store, true, "y\n", narrow).await;
        assert!(logged_in);
        assert!(
            out.contains("didn't grant: heart_health"),
            "must name what Oura withheld: {out}"
        );
        let (again, logged_in) = run_check_granting(&store, true, "y\n", narrow).await;
        assert!(!logged_in, "must not re-prompt into a login loop: {again}");
        assert_eq!(again, "");
    }

    #[tokio::test]
    async fn no_is_remembered_and_the_next_run_is_silent() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let (out, logged_in) = run_check(&store, true, "n\n").await;
        assert!(!logged_in);
        assert!(out.contains("Skipped"), "{out}");
        assert_eq!(load_state(&store).declined, Some(scope_set_key()));

        let (again, logged_in) = run_check(&store, true, "\n").await;
        assert!(!logged_in, "a declined scope set must not prompt again");
        assert_eq!(again, "");
    }

    #[tokio::test]
    async fn eof_on_the_prompt_declines_instead_of_logging_in() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let (_out, logged_in) = run_check(&store, true, "").await;
        assert!(!logged_in, "EOF must not be read as the default yes");
    }

    #[tokio::test]
    async fn a_failed_login_propagates_and_records_nothing() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let mut out = Vec::new();
        let err = check(&store, true, &mut "y\n".as_bytes(), &mut out, || async {
            Err(anyhow::anyhow!("consent denied"))
        })
        .await
        .unwrap_err();
        assert!(err.to_string().contains("consent denied"));
        assert_eq!(load_state(&store), NoticeState::default(), "retry next run");
    }

    #[tokio::test]
    async fn non_interactive_notices_once_and_never_reads_stdin_or_logs_in() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        // A stdin that would accept — proves the non-interactive path never consults it.
        let (out, logged_in) = run_check(&store, false, "y\n").await;
        assert!(!logged_in);
        assert!(
            out.starts_with("oura: note:") && out.contains("spo2 heart_health"),
            "{out}"
        );
        assert_eq!(out.lines().count(), 1, "the notice is a single line: {out}");
        let (again, _) = run_check(&store, false, "y\n").await;
        assert_eq!(again, "", "the notice is shown once per scope set");
    }

    #[tokio::test]
    async fn an_unrecorded_grant_is_flagged_as_uncertain_not_as_definitely_missing() {
        let (store, _dir) = store_with(None);
        let (out, _) = run_check(&store, false, "").await;
        assert!(out.contains("may not cover"), "{out}");
    }

    #[tokio::test]
    async fn a_current_grant_is_silent_and_writes_no_state() {
        let (store, _dir) = store_with(Some(&scope_set_key()));
        let (out, logged_in) = run_check(&store, true, "y\n").await;
        assert_eq!((out.as_str(), logged_in), ("", false));
        assert!(!store.dir().join(STATE_FILE).exists());
    }

    #[test]
    fn mcp_notice_names_the_gap_for_a_stale_grant_and_points_at_login() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let note = mcp_notice(&store).expect("a stale grant gets the MCP note");
        assert!(note.starts_with(MCP_NOTICE_LEAD), "{note}");
        assert!(
            note.contains(": spo2 heart_health."),
            "names the gap: {note}"
        );
        assert!(note.contains("oura auth login"), "names the fix: {note}");
    }

    #[test]
    fn mcp_notice_is_silent_for_a_current_grant_no_tokens_or_a_cli_decline() {
        let (current, _d1) = store_with(Some(&scope_set_key()));
        assert_eq!(mcp_notice(&current), None);

        let empty_dir = tempfile::tempdir().unwrap();
        assert_eq!(mcp_notice(&TokenStore::with_dir(empty_dir.path())), None);

        let (declined, _d2) = store_with(Some(PRE_1_41));
        save_state(
            &declined,
            &NoticeState {
                declined: Some(scope_set_key()),
                notified: None,
            },
        );
        assert_eq!(mcp_notice(&declined), None, "a CLI decline covers MCP too");
    }

    #[test]
    fn mcp_notice_ignores_a_script_notice_and_writes_no_state() {
        // A script's stderr note never reached the human in the chat. It must not silence MCP.
        let (store, _dir) = store_with(Some(PRE_1_41));
        save_state(
            &store,
            &NoticeState {
                declined: None,
                notified: Some(scope_set_key()),
            },
        );
        assert!(mcp_notice(&store).is_some());
        assert_eq!(load_state(&store).declined, None, "read-only");
    }

    #[tokio::test]
    async fn a_corrupt_state_file_is_treated_as_untold_not_as_an_error() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        std::fs::write(store.dir().join(STATE_FILE), b"{not json").unwrap();
        let (out, _) = run_check(&store, false, "").await;
        assert!(out.starts_with("oura: note:"), "{out}");
    }
}
