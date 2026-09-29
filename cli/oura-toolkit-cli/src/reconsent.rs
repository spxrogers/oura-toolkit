//! Re-consent when the toolkit's default scopes outgrow a saved login (#116).
//!
//! Oura renames and adds OAuth scopes between API exports (openapi-1.41: `spo2Daily` →
//! `spo2`, plus a new `heart_health`). A token refresh can't add scopes; only a new consent
//! can. So a login from before such a change quietly lacks them. Detection is
//! **data-driven**: the grant recorded in `tokens.json` vs the spec-validated
//! [`metadata::default_scopes`] ([`missing_default_scopes`]). A scope change needs no
//! migration code, only the spec bump (whose tests force the default list to follow).
//!
//! It fires **once per scope-set change**, before a store-backed command (the data commands
//! and `oura api`), after the command's own arguments are validated:
//! - **interactive** (stdin + stderr are TTYs): explain what's missing and ask
//!   [`PROMPT_QUESTION`]. Yes runs `oura auth login` (default port; the paste-back flow over
//!   SSH), then the original command continues on the new tokens. No, EOF, or a login that
//!   fails or still grants too little are remembered, and the command carries on with the
//!   existing login: re-consent is optional, so it never costs the user their command.
//! - **non-interactive**: one [`NOTICE_PREFIX`] stderr line, never blocking. It's remembered
//!   separately, so a script running first doesn't use up the human's prompt.
//!
//! `oura mcp` can't prompt (stdout is its transport, and stdio MCP auth is out of band), so
//! it gets the MCP flavour, [`mcp_notice`]: a note on the first successful tool result of each
//! session, for the model to relay. An `OURA_ACCESS_TOKEN` run (no store) sees neither.
//!
//! The bookkeeping lives in [`STATE_FILE`] next to the token records (0600, atomic, removed by
//! `oura auth logout --all`). It's CLI-only and holds no secrets: this is CLI policy, not
//! companion API, so the six-language store records and conformance fixture are untouched.

use std::future::Future;
use std::io::{BufRead, Write};

use oura_toolkit_auth::{metadata, TokenStore, Tokens};
use serde::{Deserialize, Serialize};

use crate::auth::{APP_REGISTRATION_URL, DEFAULT_LOGIN_PORT};

/// The bookkeeping record, in the store dir alongside `credentials.json` / `tokens.json`.
pub const STATE_FILE: &str = "scope-notice.json";

/// How the one-line non-interactive notice starts (documented; pinned by docs_tripwire.rs).
pub const NOTICE_PREFIX: &str = "oura: note:";

/// How the interactive prompt starts (documented; pinned by docs_tripwire.rs).
pub const PROMPT_LEAD: &str = "Oura's API permissions (OAuth scopes) changed";

/// The interactive question (documented; pinned by docs_tripwire.rs).
pub const PROMPT_QUESTION: &str = "Re-authorize now? [Y/n]";

/// How the MCP tool-result note starts (documented; pinned by docs_tripwire.rs).
pub const MCP_NOTICE_LEAD: &str = "Note: Oura changed its API permissions (OAuth scopes)";

/// The grant recorded with the tokens, or `None` when there is none to go by. A blank value
/// counts as unrecorded: it can't be shown to cover anything.
fn recorded_grant(tokens: &Tokens) -> Option<&str> {
    tokens.scope.as_deref().filter(|s| !s.trim().is_empty())
}

/// The default scopes a saved grant does NOT cover — what a fresh consent would add.
///
/// `granted` is the recorded grant (the token endpoint's space-delimited `scope`; commas are
/// tolerated), matched as whole scopes, so `spo2Daily` never satisfies `spo2`. An unrecorded
/// grant (`None`) is treated as missing EVERY default: a login from before the toolkit
/// recorded scopes can't be shown to cover the current set.
pub(crate) fn missing_default_scopes(granted: Option<&str>) -> Vec<&'static str> {
    let granted: Vec<&str> = granted
        .map(|g| {
            g.split(|c: char| c.is_whitespace() || c == ',')
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();
    metadata::default_scopes()
        .into_iter()
        .filter(|s| !granted.contains(s))
        .collect()
}

/// The default scopes `tokens` lacks, plus the wording for how sure we are about it.
pub(crate) fn scope_gap(tokens: &Tokens) -> (Vec<&'static str>, &'static str) {
    let grant = recorded_grant(tokens);
    let coverage = if grant.is_some() {
        "doesn't cover"
    } else {
        "may not cover"
    };
    (missing_default_scopes(grant), coverage)
}

/// What the user has already been told, keyed by the default scope set it was told about, so
/// the next scope change (a different key) asks again.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct NoticeState {
    /// The scope set an interactive prompt was declined for (or couldn't complete).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    declined: Option<String>,
    /// The scope set a non-interactive run already printed its one notice for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    notified: Option<String>,
}

/// What to do before the command runs.
#[derive(Debug, PartialEq, Eq)]
enum Action {
    /// The grant is current, there are no tokens, or the user was already told.
    Nothing,
    /// Ask to re-authorize; carries the missing default scopes.
    Prompt(Vec<&'static str>),
    /// Print the one-line notice; carries the missing default scopes.
    Notify(Vec<&'static str>),
}

/// The key a [`NoticeState`] entry is recorded under: the current default scope set, sorted,
/// so reordering the default list doesn't re-ask users who already answered.
fn scope_set_key() -> String {
    let mut scopes = metadata::default_scopes();
    scopes.sort_unstable();
    scopes.join(" ")
}

/// The pure decision. No tokens → [`Action::Nothing`] (the command's own
/// not-authenticated error applies; there is no grant to be stale).
fn decide(tokens: Option<&Tokens>, state: &NoticeState, interactive: bool) -> Action {
    let Some(tokens) = tokens else {
        return Action::Nothing;
    };
    let (missing, _) = scope_gap(tokens);
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

/// `y`, `yes`, or a bare Enter (the default) accept; anything else declines.
fn parse_answer(line: &str) -> bool {
    matches!(line.trim().to_ascii_lowercase().as_str(), "" | "y" | "yes")
}

fn state_path(store: &TokenStore) -> std::path::PathBuf {
    store.dir().join(STATE_FILE)
}

fn load_state(store: &TokenStore) -> NoticeState {
    // Best-effort: a missing or corrupt record just means "not told yet". Never fail the
    // user's command over bookkeeping.
    std::fs::read(state_path(store))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default()
}

/// Persist `state` like the store's own records: owner-only (0600 on Unix) and atomic (a
/// fresh temp file renamed over the target, so a symlink planted at the target is replaced,
/// not followed). Best-effort, like [`load_state`]: failing only means the notice may repeat.
fn save_state(store: &TokenStore, state: &NoticeState) {
    let Ok(data) = serde_json::to_vec_pretty(state) else {
        return;
    };
    let target = state_path(store);
    let tmp = store
        .dir()
        .join(format!(".{STATE_FILE}.{}.tmp", std::process::id()));
    let write = || -> std::io::Result<()> {
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt as _;
            options.mode(0o600);
        }
        let mut file = options.open(&tmp)?;
        file.write_all(&data)?;
        file.sync_all()?;
        std::fs::rename(&tmp, &target)
    };
    if write().is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
}

/// Forget everything the user was told (`oura auth logout --all`: a full reset re-arms the
/// notice). Idempotent.
pub(crate) fn forget(store: &TokenStore) -> std::io::Result<()> {
    match std::fs::remove_file(state_path(store)) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

/// Run the check against `store`: decide, then prompt/notify through the injected `input` /
/// `out` (stderr in production), invoking `login(headless)` on a yes (`headless` = use the
/// paste-back flow). Never fails the command: a login that errors is reported and remembered,
/// and the command continues on the existing login.
async fn check<L, Fut>(
    store: &TokenStore,
    interactive: bool,
    headless: bool,
    input: &mut dyn BufRead,
    out: &mut dyn Write,
    login: L,
) where
    L: FnOnce(bool) -> Fut,
    Fut: Future<Output = anyhow::Result<()>>,
{
    // An unreadable token record is the command's own error to report, not ours.
    let Ok(tokens) = store.load_tokens() else {
        return;
    };
    let mut state = load_state(store);
    let coverage = tokens.as_ref().map_or("doesn't cover", |t| scope_gap(t).1);
    match decide(tokens.as_ref(), &state, interactive) {
        Action::Nothing => {}
        Action::Notify(missing) => {
            let _ = writeln!(
                out,
                "{NOTICE_PREFIX} your saved login {coverage} this version's Oura permissions: \
                 {} — run `oura auth login` to re-authorize (shown once).",
                missing.join(" ")
            );
            state.notified = Some(scope_set_key());
            save_state(store, &state);
        }
        Action::Prompt(missing) => {
            let how = if headless {
                "the paste-back flow, since this looks like an SSH session"
            } else {
                "your browser"
            };
            let _ = write!(
                out,
                "{PROMPT_LEAD}, and your saved login {coverage}: {}\n\
                 A token refresh can't add scopes; a new login (consent) can.\n\
                 If your Oura app doesn't list them yet, add them first at \
                 {APP_REGISTRATION_URL}\n\
                 (Logs in via {how} on port {DEFAULT_LOGIN_PORT}. If your app is registered on \
                 another port, answer n and run `oura auth login --port <n>`.)\n\
                 {PROMPT_QUESTION} ",
                missing.join(" ")
            );
            let _ = out.flush();
            let mut line = String::new();
            // EOF (0 bytes) declines: only an actual Enter is the default yes.
            let accepted =
                matches!(input.read_line(&mut line), Ok(n) if n > 0) && parse_answer(&line);
            if !accepted {
                let _ = writeln!(
                    out,
                    "Skipped. Run `oura auth login` any time to grant them (not asked again \
                     for this change)."
                );
            } else if let Err(err) = login(headless).await {
                // Re-consent is optional: the existing login still works for everything it
                // covered, so a failed or abandoned login must not cost the user their command.
                let _ = writeln!(
                    out,
                    "Re-authorization didn't complete: {}\nContinuing with your existing \
                     login. Run `oura auth login` yourself to retry (not asked again for this \
                     change).",
                    crate::output::sanitize(&format!("{err:#}"))
                );
            } else {
                // Oura may still grant less than asked (the app's registration lacks a scope,
                // or the user unticked one). Say so, or the next run would prompt again.
                let still = store
                    .load_tokens()
                    .ok()
                    .flatten()
                    .map(|t| scope_gap(&t).0)
                    .unwrap_or_default();
                if still.is_empty() {
                    let _ = writeln!(out, "Continuing…");
                    return;
                }
                let _ = writeln!(
                    out,
                    "Logged in, but Oura didn't grant: {}. Add them to your app at \
                     {APP_REGISTRATION_URL}, then run `oura auth login` (not asked again for \
                     this change). Continuing…",
                    still.join(" ")
                );
            }
            // Every non-success outcome is remembered for this scope set, so a user is never
            // looped through the same prompt (or a 300s callback wait) on every run.
            state.declined = Some(scope_set_key());
            save_state(store, &state);
        }
    }
}

/// The MCP flavour: the note to append to a tool result, or `None`.
///
/// It uses the same decision as the interactive prompt (`decide(…, true)`): a human is on the
/// other end, just not at a TTY we own. So a scope set the user already declined at the CLI
/// prompt stays quiet here too, while a script's stderr notice (which never reached the person
/// in the chat) doesn't silence it. It's read-only: "once" is per MCP session, tracked by the
/// server.
pub(crate) fn mcp_notice(store: &TokenStore) -> Option<String> {
    let tokens = store.load_tokens().ok()??;
    let Action::Prompt(missing) = decide(Some(&tokens), &load_state(store), true) else {
        return None;
    };
    let (_, coverage) = scope_gap(&tokens);
    Some(format!(
        "{MCP_NOTICE_LEAD}, and the user's saved login {coverage} them: {}. The data above is \
         still valid. Tell the user once: to grant them, run `oura auth login` in a terminal. \
         If their Oura app doesn't list these scopes yet, they add them first at \
         {APP_REGISTRATION_URL}. To silence this without re-authorizing, they can answer `n` \
         when any `oura` data command asks in a terminal.",
        missing.join(" ")
    ))
}

/// The production entry point: real stdin/stderr, TTY-detected interactivity (BOTH stdin and
/// stderr, or a prompt could block unseen, e.g. `oura sleep 2>log`), and `oura auth login` on
/// the default port (paste-back over SSH).
pub async fn run(store: &TokenStore) {
    use std::io::IsTerminal as _;
    let interactive = std::io::stdin().is_terminal() && std::io::stderr().is_terminal();
    let headless = crate::auth::looks_headless(|k| std::env::var(k).ok());
    let stdin = std::io::stdin();
    check(
        store,
        interactive,
        headless,
        &mut stdin.lock(),
        &mut std::io::stderr(),
        |no_browser| crate::auth::login(DEFAULT_LOGIN_PORT, no_browser),
    )
    .await;
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

    fn current_grant() -> String {
        metadata::default_scopes().join(" ")
    }

    fn store_with(scope: Option<&str>) -> (TokenStore, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        store.save_tokens(&tokens(scope)).unwrap();
        (store, dir)
    }

    /// What a scripted `check` run did: stderr text, whether login was invoked, and with
    /// which `no_browser` flag.
    struct Ran {
        out: String,
        logged_in: bool,
        no_browser: Option<bool>,
    }

    /// Run `check` with a scripted stdin. `login` is `Ok(Some(grant))` to simulate a
    /// successful re-consent persisting `grant`, or `Err(msg)` to simulate a failed login.
    async fn run_with(
        store: &TokenStore,
        interactive: bool,
        headless: bool,
        stdin: &str,
        login: Result<&str, &str>,
    ) -> Ran {
        let called = Cell::new(None);
        let mut out = Vec::new();
        check(
            store,
            interactive,
            headless,
            &mut stdin.as_bytes(),
            &mut out,
            |no_browser| {
                called.set(Some(no_browser));
                async move {
                    match login {
                        Ok(grant) => {
                            store.save_tokens(&tokens(Some(grant))).unwrap();
                            Ok(())
                        }
                        Err(msg) => Err(anyhow::anyhow!(msg.to_owned())),
                    }
                }
            },
        )
        .await;
        Ran {
            out: String::from_utf8(out).unwrap(),
            logged_in: called.get().is_some(),
            no_browser: called.get(),
        }
    }

    /// The common case: a successful re-consent that grants every default scope.
    async fn run_check(store: &TokenStore, interactive: bool, stdin: &str) -> Ran {
        let grant = current_grant();
        run_with(store, interactive, false, stdin, Ok(&grant)).await
    }

    // --- missing_default_scopes / scope_gap (pure) -----------------------------------------

    #[test]
    fn missing_default_scopes_names_exactly_what_a_pre_1_41_grant_lacks() {
        assert_eq!(
            missing_default_scopes(Some(PRE_1_41)),
            ["spo2", "heart_health"]
        );
    }

    #[test]
    fn missing_default_scopes_is_empty_for_a_current_grant_in_any_order_or_delimiter() {
        assert_eq!(
            missing_default_scopes(Some(&current_grant())),
            Vec::<&str>::new()
        );
        let mut reversed = metadata::default_scopes();
        reversed.reverse();
        let messy = format!("email,  {}", reversed.join(",\t"));
        assert_eq!(missing_default_scopes(Some(&messy)), Vec::<&str>::new());
    }

    #[test]
    fn missing_default_scopes_matches_whole_scopes_not_substrings() {
        // `spo2Daily` must not satisfy `spo2` (a substring match would hide the rename).
        let missing = missing_default_scopes(Some("spo2Daily heart_healthy"));
        assert!(missing.contains(&"spo2") && missing.contains(&"heart_health"));
    }

    #[test]
    fn an_unrecorded_or_blank_grant_is_missing_everything_and_hedged() {
        for scope in [None, Some(""), Some("  \t")] {
            let (missing, coverage) = scope_gap(&tokens(scope));
            assert_eq!(missing, metadata::default_scopes(), "{scope:?}");
            assert_eq!(
                coverage, "may not cover",
                "{scope:?} must hedge, not assert"
            );
        }
        assert_eq!(scope_gap(&tokens(Some(PRE_1_41))).1, "doesn't cover");
    }

    // --- decide (pure) ---------------------------------------------------------------------

    #[test]
    fn a_current_grant_never_prompts_or_notifies() {
        let t = tokens(Some(&current_grant()));
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
    fn the_state_key_is_the_default_set_independent_of_list_order() {
        let key = scope_set_key();
        let mut words: Vec<&str> = key.split(' ').collect();
        assert_eq!(words.len(), metadata::default_scopes().len());
        let before = words.clone();
        words.sort_unstable();
        assert_eq!(
            words, before,
            "the key must be sorted, not list-ordered: {key}"
        );
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
    async fn yes_runs_login_continues_and_records_nothing() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let ran = run_check(&store, true, "\n").await;
        assert!(ran.logged_in, "Enter (default yes) must run the login");
        assert!(ran.out.starts_with(PROMPT_LEAD), "{}", ran.out);
        assert!(
            ran.out.contains("spo2 heart_health"),
            "names the gap: {}",
            ran.out
        );
        assert!(ran.out.contains(PROMPT_QUESTION), "{}", ran.out);
        assert!(ran.out.ends_with("Continuing…\n"), "{}", ran.out);
        assert_eq!(load_state(&store), NoticeState::default());
    }

    #[tokio::test]
    async fn over_ssh_the_prompt_says_and_the_login_uses_the_paste_back_flow() {
        let grant = current_grant();
        let (store, _dir) = store_with(Some(PRE_1_41));
        let ran = run_with(&store, true, true, "y\n", Ok(&grant)).await;
        assert!(ran.out.contains("paste-back"), "{}", ran.out);
        assert_eq!(
            ran.no_browser,
            Some(true),
            "a loopback can't be reached over SSH"
        );
        let (store, _dir) = store_with(Some(PRE_1_41));
        let ran = run_with(&store, true, false, "y\n", Ok(&grant)).await;
        assert!(ran.out.contains("your browser"), "{}", ran.out);
        assert_eq!(
            ran.no_browser,
            Some(false),
            "locally, the browser + loopback flow"
        );
    }

    #[tokio::test]
    async fn a_login_that_still_grants_too_little_says_so_and_does_not_loop() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        // The user's Oura app lacks heart_health, so the re-consent grants everything else.
        let narrow = "personal daily heartrate workout tag session spo2";
        let ran = run_with(&store, true, false, "y\n", Ok(narrow)).await;
        assert!(ran.logged_in);
        assert!(
            ran.out.contains("didn't grant: heart_health"),
            "must name what Oura withheld: {}",
            ran.out
        );
        let again = run_with(&store, true, false, "y\n", Ok(narrow)).await;
        assert!(
            !again.logged_in,
            "must not re-prompt into a login loop: {}",
            again.out
        );
        assert_eq!(again.out, "");
    }

    #[tokio::test]
    async fn a_failed_login_continues_the_command_and_is_not_asked_again() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let ran = run_with(&store, true, false, "y\n", Err("timed out after 300s")).await;
        assert!(ran.logged_in);
        assert!(
            ran.out.contains("didn't complete: timed out after 300s")
                && ran.out.contains("Continuing with your existing login"),
            "a failed optional login must report and carry on: {}",
            ran.out
        );
        assert_eq!(load_state(&store).declined, Some(scope_set_key()));
        let again = run_with(&store, true, false, "y\n", Err("timed out")).await;
        assert!(!again.logged_in, "no 300s wait on every run: {}", again.out);
    }

    #[tokio::test]
    async fn a_failed_logins_error_text_is_sanitized() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let ran = run_with(&store, true, false, "y\n", Err("boom\u{1b}[2J\u{7}")).await;
        assert!(
            !ran.out.contains('\u{1b}') && !ran.out.contains('\u{7}'),
            "{:?}",
            ran.out
        );
    }

    #[tokio::test]
    async fn no_is_remembered_and_the_next_run_is_silent() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let ran = run_check(&store, true, "n\n").await;
        assert!(!ran.logged_in);
        assert!(ran.out.contains("Skipped"), "{}", ran.out);
        assert_eq!(load_state(&store).declined, Some(scope_set_key()));

        let again = run_check(&store, true, "\n").await;
        assert!(
            !again.logged_in,
            "a declined scope set must not prompt again"
        );
        assert_eq!(again.out, "");
    }

    #[tokio::test]
    async fn eof_on_the_prompt_declines_and_is_remembered() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        let ran = run_check(&store, true, "").await;
        assert!(!ran.logged_in, "EOF must not be read as the default yes");
        assert_eq!(
            load_state(&store).declined,
            Some(scope_set_key()),
            "EOF is a decline, and a decline is remembered"
        );
    }

    #[tokio::test]
    async fn non_interactive_notices_once_and_never_reads_stdin_or_logs_in() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        // A stdin that would accept — proves the non-interactive path never consults it.
        let ran = run_check(&store, false, "y\n").await;
        assert!(!ran.logged_in);
        assert!(
            ran.out.starts_with(NOTICE_PREFIX) && ran.out.contains("spo2 heart_health"),
            "{}",
            ran.out
        );
        assert_eq!(
            ran.out.lines().count(),
            1,
            "the notice is a single line: {}",
            ran.out
        );
        let again = run_check(&store, false, "y\n").await;
        assert_eq!(again.out, "", "the notice is shown once per scope set");
    }

    #[tokio::test]
    async fn an_unrecorded_grant_is_flagged_as_uncertain_not_as_definitely_missing() {
        let (store, _dir) = store_with(None);
        let ran = run_check(&store, false, "").await;
        assert!(ran.out.contains("may not cover"), "{}", ran.out);
    }

    #[tokio::test]
    async fn a_current_grant_is_silent_and_writes_no_state() {
        let (store, _dir) = store_with(Some(&current_grant()));
        let ran = run_check(&store, true, "y\n").await;
        assert_eq!((ran.out.as_str(), ran.logged_in), ("", false));
        assert!(!state_path(&store).exists());
    }

    #[tokio::test]
    async fn a_corrupt_state_file_is_treated_as_untold_not_as_an_error() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        std::fs::write(state_path(&store), b"{not json").unwrap();
        let ran = run_check(&store, false, "").await;
        assert!(ran.out.starts_with(NOTICE_PREFIX), "{}", ran.out);
    }

    #[tokio::test]
    async fn an_unwritable_state_file_never_fails_the_command() {
        // A DIRECTORY where the record belongs makes every write fail — even as root, which
        // bypasses permission bits. The notice still prints and nothing panics.
        let (store, _dir) = store_with(Some(PRE_1_41));
        std::fs::create_dir(state_path(&store)).unwrap();
        let ran = run_check(&store, false, "").await;
        assert!(ran.out.starts_with(NOTICE_PREFIX), "{}", ran.out);
        assert!(state_path(&store).is_dir(), "left as it was");
        let leftovers: Vec<_> = std::fs::read_dir(store.dir())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().ends_with(".tmp"))
            .collect();
        assert!(
            leftovers.is_empty(),
            "no temp file left behind: {leftovers:?}"
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn the_state_file_is_owner_only_and_replaces_a_planted_symlink() {
        use std::os::unix::fs::PermissionsExt as _;
        let (store, dir) = store_with(Some(PRE_1_41));
        // A symlink planted at the record must be REPLACED, never written through.
        let victim = dir.path().join("victim.txt");
        std::fs::write(&victim, "untouched").unwrap();
        std::os::unix::fs::symlink(&victim, state_path(&store)).unwrap();
        run_check(&store, false, "").await;
        assert_eq!(std::fs::read_to_string(&victim).unwrap(), "untouched");
        let meta = std::fs::symlink_metadata(state_path(&store)).unwrap();
        assert!(
            meta.file_type().is_file(),
            "the symlink was replaced by a file"
        );
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    }

    #[tokio::test]
    async fn forget_removes_the_record_and_is_idempotent() {
        let (store, _dir) = store_with(Some(PRE_1_41));
        run_check(&store, true, "n\n").await;
        assert!(state_path(&store).exists());
        forget(&store).unwrap();
        assert!(!state_path(&store).exists());
        forget(&store).unwrap();
    }

    // --- mcp_notice ------------------------------------------------------------------------

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
        let (current, _d1) = store_with(Some(&current_grant()));
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
}
