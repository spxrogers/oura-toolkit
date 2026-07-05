//! Interactive OAuth flows — the browser + loopback consent that lives ONLY in the CLI.
//!
//! `setup` guides the user through registering their own Oura OAuth app (BYO confidential
//! credentials) and collects `client_id`/`client_secret` via terminal prompts — the secret is
//! read with echo disabled and never leaves this process except to Oura's token endpoint.
//! `login` runs the Authorization Code flow against a fixed loopback `redirect_uri`, catches
//! the code, and exchanges it via `oura-toolkit-auth`. The non-interactive token machinery
//! (exchange/refresh/store) all lives in `oura-toolkit-auth`.
//!
//! Alongside the interactive flows live the non-interactive account commands (#18):
//! `status` / `logout` / `refresh` / `token` — thin, scriptable wrappers over the same
//! store and `TokenManager` (their contract is documented in docs/cli-contract.md → Auth
//! commands).

use anyhow::{anyhow, bail, Context, Result};
use oura_toolkit_auth::{
    exchange_code, metadata, AuthError, ClientCredentials, TokenManager, TokenStore, Tokens,
};
use tokio::net::TcpListener;

use crate::loopback::{self, Request};
use crate::output::{render_record, RenderOptions};

/// How long `auth login` waits for the browser callback before giving up.
const CALLBACK_TIMEOUT_SECS: u64 = 300;

/// `oura auth setup` — register an app (terminal prompts), then log in. `no_browser` runs
/// the paste-back login (#20) instead of the loopback flow, for SSH/containers.
pub async fn setup(port: u16, no_browser: bool) -> Result<()> {
    let redirect_uri = redirect_uri(port);
    let scopes = metadata::default_scopes();
    let scope_str = scopes.join(" ");

    guide("== Register your Oura OAuth application ==\n");
    if no_browser {
        guide("Register an application at:\n  https://cloud.ouraring.com/oauth/applications\n");
    } else {
        guide("Opening https://cloud.ouraring.com/oauth/applications in your browser…");
        let _ = open::that("https://cloud.ouraring.com/oauth/applications");
    }
    guide("Create an application with these EXACT values:\n");
    guide("  • Application name : oura-toolkit   (or any name you like)");
    guide(&format!("  • Redirect URI     : {redirect_uri}"));
    guide(&format!("  • Scopes           : {scope_str}\n"));

    // Browser mode binds the callback listener up front so a port conflict fails fast, before
    // the user has typed anything. --no-browser has no listener (the callback lands elsewhere).
    let listener = if no_browser {
        None
    } else {
        Some(bind(port).await?)
    };

    // Terminal prompts, no local HTTP surface: the client_id is echoed (it is not a secret);
    // the secret is read with echo disabled.
    guide("Paste the credentials Oura shows for your new application:");
    let credentials = ClientCredentials {
        client_id: prompt_required("  Client ID: ")?,
        client_secret: prompt_secret_required("  Client Secret (input hidden): ")?,
    };

    let store = TokenStore::new()?;
    persist_credentials_then_authorize(&store, credentials, |creds| async move {
        match listener {
            Some(listener) => run_authorization(&listener, port, &creds).await,
            None => authorize_no_browser(port, &creds).await,
        }
    })
    .await
}

/// The ordering guarantee of `auth setup` (#23), as a testable core: the credentials
/// record is persisted BEFORE the consent flow runs, so a failed or abandoned
/// authorization never loses the pasted secret — `oura auth login` can retry without
/// re-setup.
async fn persist_credentials_then_authorize<F, Fut>(
    store: &TokenStore,
    credentials: ClientCredentials,
    authorize: F,
) -> Result<()>
where
    F: FnOnce(ClientCredentials) -> Fut,
    Fut: std::future::Future<Output = Result<Tokens>>,
{
    store.save_credentials(&credentials)?;
    guide(&format!(
        "✓ Credentials saved to {} — `oura auth login` can reuse them any time.\n",
        store.credentials_path().display()
    ));

    guide("Continuing to login…");
    let tokens = authorize(credentials).await?;
    persist(store, &tokens)
}

/// `oura auth login` — Authorization Code flow using the stored client credentials.
/// `no_browser` prints the authorize URL and reads the pasted redirect back (#20), for hosts
/// where the loopback callback can't be reached (SSH, containers).
pub async fn login(port: u16, no_browser: bool) -> Result<()> {
    let store = TokenStore::new()?;
    // Typed error (not a bail! string) so the contract classifier can map it to exit 4
    // with the `oura auth setup` hint (#21).
    let credentials = store
        .load_credentials()?
        .ok_or(oura_toolkit_auth::AuthError::MissingClientCredentials)?;
    let tokens = if no_browser {
        authorize_no_browser(port, &credentials).await?
    } else {
        let listener = bind(port).await?;
        run_authorization(&listener, port, &credentials).await?
    };
    persist(&store, &tokens)
}

// --- Non-interactive account commands (#18): status / logout / refresh / token --------------
//
// Thin wrappers over `oura-toolkit-auth`, gh-style (`gh auth status|logout|refresh|token` is
// the benchmark). Each returns a rendered string and `main` owns the write: query results
// (`status`, `token`) go to stdout via `contract::emit`; mutation confirmations (`logout`,
// `refresh`) are prose and go to stderr via `contract::inform` (contract → Streams). None of
// them may ever print the client secret, and only `token` may print token values — that is
// its deliberate, documented output.

/// The `auth status` outcome: the rendered report (always emitted to stdout — it is the
/// command's result, including the partial state a user needs to see to fix their setup)
/// plus the typed failure when unauthenticated, which `main` routes through the contract
/// classifier for the documented exit 4 + remediation hint.
pub struct StatusReport {
    pub rendered: String,
    pub failure: Option<AuthError>,
}

/// Serde model behind `auth status --json`. Field names are part of the scripting surface.
/// Deliberately carries NO secret material: the client secret and the token values stay in
/// the store records.
#[derive(serde::Serialize)]
struct StatusModel {
    store: String,
    authenticated: bool,
    credentials: CredentialsStatus,
    tokens: TokensStatus,
}

#[derive(serde::Serialize)]
struct CredentialsStatus {
    present: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
}

#[derive(serde::Serialize)]
struct TokensStatus {
    present: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<i64>,
    /// Literal wall-clock expiry (`now >= expires_at`). The top-level `authenticated`
    /// additionally treats a token inside the manager's proactive-refresh window
    /// (`REFRESH_SKEW_SECS`) as needing a refresh — so `expired: false` with
    /// `authenticated: false` means "expires within the refresh window and there are no
    /// credentials to refresh with".
    #[serde(skip_serializing_if = "Option::is_none")]
    expired: Option<bool>,
}

/// `oura auth status` — report what is stored (never the secret), scopes, and expiry.
pub fn status(store: &TokenStore, render: RenderOptions) -> Result<StatusReport> {
    status_at(store, render, chrono::Utc::now().timestamp())
}

/// Testable core of [`status`]: `now` is injected so expiry text and the
/// authenticated decision are deterministic under test.
fn status_at(store: &TokenStore, render: RenderOptions, now: i64) -> Result<StatusReport> {
    let credentials = store.load_credentials()?;
    let tokens = store.load_tokens()?;

    // `expired` is the literal wall-clock state (what the report and --json show). The
    // authenticated decision instead uses the manager's proactive-refresh window
    // (REFRESH_SKEW_SECS): a data command refreshes inside it, so a token that cannot be
    // refreshed is already dead for scripting purposes even slightly before expires_at.
    let expired = tokens.as_ref().map(|t| now >= t.expires_at);
    let would_refresh = tokens
        .as_ref()
        .map(|t| now + oura_toolkit_auth::REFRESH_SKEW_SECS >= t.expires_at);
    // Authenticated = a data command would get a token: tokens exist AND are either
    // outside the refresh window or refreshable (a confidential-client call needing the
    // credentials record).
    let authenticated = matches!(
        (would_refresh, &credentials),
        (Some(false), _) | (Some(true), Some(_))
    );
    // The remediation the classifier will hint: no tokens → login (or setup when the
    // credentials are missing too); expired-and-unrefreshable → setup.
    let failure = match (&tokens, authenticated, &credentials) {
        (_, true, _) => None,
        (None, false, Some(_)) => Some(AuthError::NotAuthenticated),
        _ => Some(AuthError::MissingClientCredentials),
    };

    let mut fields: Vec<(&str, String)> = vec![
        ("Store", store.dir().display().to_string()),
        (
            "Credentials",
            match &credentials {
                Some(c) => format!("present (client_id: {})", c.client_id),
                None => "none".into(),
            },
        ),
        (
            "Tokens",
            if tokens.is_some() {
                "present".into()
            } else {
                "none".into()
            },
        ),
    ];
    if let Some(t) = &tokens {
        fields.push((
            "Scope",
            t.scope.clone().unwrap_or_else(|| "(not recorded)".into()),
        ));
        fields.push((
            "Access token",
            expiry_phrase(t.expires_at, now, credentials.is_some()),
        ));
    }
    fields.push((
        "Authenticated",
        if authenticated { "yes" } else { "no" }.into(),
    ));

    let model = StatusModel {
        store: store.dir().display().to_string(),
        authenticated,
        credentials: CredentialsStatus {
            present: credentials.is_some(),
            client_id: credentials.map(|c| c.client_id),
        },
        tokens: TokensStatus {
            present: tokens.is_some(),
            scope: tokens.as_ref().and_then(|t| t.scope.clone()),
            expires_at: tokens.as_ref().map(|t| t.expires_at),
            expired,
        },
    };
    Ok(StatusReport {
        rendered: render_record(&model, &fields, render)?,
        failure,
    })
}

/// `oura auth logout` — delete stored tokens; `--all` also removes the client credentials
/// (the only sanctioned way to remove a stored secret). Idempotent: nothing stored is
/// success, not an error.
pub fn logout(store: &TokenStore, all: bool) -> Result<String> {
    // Same coordination as `persist`: don't delete out from under a peer's (CLI or MCP
    // server) reload → refresh → persist critical section (#22). The lock only closes the
    // intra-section window — the other half of the guarantee lives in `TokenManager`,
    // which treats the deleted record as authoritative on its NEXT refresh (reporting
    // unauthenticated instead of re-persisting from stale memory; see client.rs).
    let _lock = match store.try_lock_exclusive()? {
        Some(lock) => lock,
        None => {
            eprintln!("waiting for another oura process to finish refreshing tokens…");
            store.lock_exclusive()?
        }
    };

    let removed_tokens = store.delete_tokens()?;
    let removed_credentials = all && store.delete_credentials()?;

    let mut out = String::new();
    if removed_tokens {
        out.push_str(&format!(
            "✓ Logged out — removed {}\n",
            store.tokens_path().display()
        ));
    } else {
        out.push_str("No tokens stored — already logged out.\n");
    }
    if all {
        if removed_credentials {
            out.push_str(&format!(
                "✓ Removed client credentials — {}\n",
                store.credentials_path().display()
            ));
        } else {
            out.push_str("No client credentials stored.\n");
        }
    } else if store.credentials_path().exists() {
        out.push_str("Client credentials kept — remove them too with `oura auth logout --all`.\n");
    }
    Ok(out)
}

/// `oura auth refresh` — force a token refresh (the debugging tool for rotation issues).
/// The rotated refresh token is persisted by the manager's locked critical section (#22).
pub async fn refresh(manager: &TokenManager, store: &TokenStore) -> Result<String> {
    // No `upgrade_unauthenticated` here: the manager checks credentials BEFORE tokens, so
    // an empty store already surfaces MissingClientCredentials (the setup hint) on its own.
    manager.force_refresh().await.context("refreshing tokens")?;
    let tokens = store
        .load_tokens()?
        .context("tokens missing from the store after a successful refresh")?;
    Ok(format!(
        "✓ Tokens refreshed and persisted to {} — access token {}\n",
        store.tokens_path().display(),
        expiry_phrase(tokens.expires_at, chrono::Utc::now().timestamp(), true),
    ))
}

/// `oura auth token` — print a valid access token (refreshing if needed) and NOTHING else
/// to stdout: the scripting workhorse (`curl -H "Authorization: Bearer $(oura auth token)"`).
pub async fn token(manager: &TokenManager, store: &TokenStore) -> Result<String> {
    let token = manager
        .access_token()
        .await
        .map_err(|e| upgrade_unauthenticated(store, e))
        .context("obtaining an access token")?;
    // Fail closed on a token carrying control bytes: real OAuth tokens are opaque
    // URL-safe strings, and this is the one output path that deliberately bypasses
    // `output::sanitize` — printing escapes/newlines verbatim would let a hostile token
    // endpoint drive the terminal or split `$(oura auth token)` into multiple words.
    if token.chars().any(char::is_control) {
        bail!("the stored access token contains control characters; refusing to print it");
    }
    Ok(format!("{token}\n"))
}

/// On a completely empty store, `NotAuthenticated` would hint `oura auth login` — which
/// then fails asking for `auth setup`. Point at the real first step instead.
fn upgrade_unauthenticated(store: &TokenStore, err: AuthError) -> AuthError {
    match err {
        AuthError::NotAuthenticated if matches!(store.load_credentials(), Ok(None)) => {
            AuthError::MissingClientCredentials
        }
        other => other,
    }
}

/// `expires in 53m 20s` / `expired 5m 0s ago (…)` — the human half of the expiry claims;
/// the machine half is `expires_at`/`expired` in the `--json` model.
fn expiry_phrase(expires_at: i64, now: i64, refreshable: bool) -> String {
    if now < expires_at {
        let base = format!("expires in {}", human_duration(expires_at - now));
        if refreshable {
            base
        } else {
            format!("{base} (no client credentials — cannot refresh)")
        }
    } else if refreshable {
        format!(
            "expired {} ago (refreshes automatically on next use)",
            human_duration(now - expires_at)
        )
    } else {
        format!(
            "expired {} ago (no client credentials — cannot refresh)",
            human_duration(now - expires_at)
        )
    }
}

/// Non-negative duration as its two most significant units: `2d 5h`, `1h 3m`, `53m 20s`, `7s`.
fn human_duration(secs: i64) -> String {
    let secs = secs.max(0);
    let (days, rem) = (secs / 86_400, secs % 86_400);
    let (hours, rem) = (rem / 3_600, rem % 3_600);
    let (mins, secs) = (rem / 60, rem % 60);
    if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else if mins > 0 {
        format!("{mins}m {secs}s")
    } else {
        format!("{secs}s")
    }
}

fn redirect_uri(port: u16) -> String {
    // Oura requires the redirect_uri to be pre-registered and match EXACTLY; `localhost` is the
    // registered host, served by the 127.0.0.1 loopback listener.
    format!("http://localhost:{port}/callback")
}

async fn bind(port: u16) -> Result<TcpListener> {
    TcpListener::bind(("127.0.0.1", port))
        .await
        .with_context(|| {
            format!(
                "could not bind loopback listener on 127.0.0.1:{port} (is it in use? try --port)"
            )
        })
}

/// Interactive guidance prose → **stderr**, per the stream-discipline contract (docs/
/// cli-contract.md → Streams): stdout carries results only, so the `setup`/`login` walkthrough
/// never pollutes a piped `oura auth token`/`status`. Newline-terminated wrapper over
/// [`crate::contract::inform`] (best-effort; a closed stderr never panics).
fn guide(msg: &str) {
    crate::contract::inform(&format!("{msg}\n"));
}

/// Prompt (echoed) until the user enters a non-empty value.
fn prompt_required(label: &str) -> Result<String> {
    loop {
        // The prompt label is human-facing → stderr (contract → Streams), keeping stdout
        // results-only. std stderr is unbuffered, so the label shows before we read the reply.
        crate::contract::inform(label);
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line)? == 0 {
            bail!("stdin closed before a value was entered");
        }
        match require_non_empty(&line) {
            Some(value) => return Ok(value),
            None => eprintln!("A value is required."),
        }
    }
}

/// Prompt with echo DISABLED until the user enters a non-empty value.
fn prompt_secret_required(label: &str) -> Result<String> {
    loop {
        let line = rpassword::prompt_password(label).context("reading the secret")?;
        match require_non_empty(&line) {
            Some(value) => return Ok(value),
            None => eprintln!("A value is required."),
        }
    }
}

fn require_non_empty(input: &str) -> Option<String> {
    let value = input.trim();
    (!value.is_empty()).then(|| value.to_string())
}

/// Open the authorize URL and catch the callback code, then exchange for tokens.
async fn run_authorization(
    listener: &TcpListener,
    port: u16,
    credentials: &ClientCredentials,
) -> Result<Tokens> {
    let redirect_uri = redirect_uri(port);
    let state = uuid::Uuid::new_v4().to_string();
    let scopes = metadata::default_scopes();
    let authorize_url =
        metadata::authorize_url(&credentials.client_id, &redirect_uri, &scopes, &state);

    // On a remote/headless session the loopback callback lands on the machine that opened the
    // browser, not necessarily this one — suggest the paste-back flow up front (#20).
    if looks_headless(|k| std::env::var(k).ok()) {
        guide(
            "Note: this looks like an SSH/remote session — the loopback callback may not reach \
             this host. If login hangs, re-run with `oura auth login --no-browser`.",
        );
    }
    guide("Opening your browser to authorize with Oura…");
    guide(&format!(
        "If it doesn't open automatically, visit:\n  {authorize_url}\n"
    ));
    if open::that(&authorize_url).is_err() {
        guide(
            "(Couldn't open a browser automatically — open the URL above, or re-run with \
             `--no-browser` to paste the redirect back.)",
        );
    }

    let code = tokio::time::timeout(
        std::time::Duration::from_secs(CALLBACK_TIMEOUT_SECS),
        wait_for_callback(listener, &state),
    )
    .await
    .map_err(|_| {
        anyhow!(
            "timed out after {CALLBACK_TIMEOUT_SECS}s waiting for the OAuth callback — \
             re-run the command (or open the authorize URL printed above)"
        )
    })??;

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("building the HTTP client")?;
    exchange_code(&http, credentials, &code, &redirect_uri)
        .await
        .context("token exchange with the Oura token endpoint failed")
}

/// The `--no-browser` half of the Authorization Code flow (#20): print the authorize URL for
/// the user to open on ANY machine, then read the pasted redirect URL back and exchange the
/// code. No loopback listener — the OAuth callback may land on a different host entirely, so
/// the `code`/`state` come back through the terminal instead of an HTTP request.
async fn authorize_no_browser(port: u16, credentials: &ClientCredentials) -> Result<Tokens> {
    let redirect_uri = redirect_uri(port);
    let state = uuid::Uuid::new_v4().to_string();
    let scopes = metadata::default_scopes();
    let authorize_url =
        metadata::authorize_url(&credentials.client_id, &redirect_uri, &scopes, &state);

    guide("Open this URL in a browser on ANY machine and approve access:\n");
    guide(&format!("  {authorize_url}\n"));
    guide(&format!(
        "After you approve, the browser is redirected to a {redirect_uri}?code=…&state=… URL. \
         On a remote or headless host that page will fail to load — that is expected. Copy the \
         FULL address from the browser's address bar and paste it here.\n"
    ));
    let pasted = prompt_required("Paste the full redirect URL: ")?;
    let code = extract_code_from_paste(&pasted, &state)?;

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .context("building the HTTP client")?;
    exchange_code(&http, credentials, &code, &redirect_uri)
        .await
        .context("token exchange with the Oura token endpoint failed")
}

/// Parse the `code` out of a pasted OAuth redirect (a full URL or a bare `code=…&state=…`
/// query), enforcing the SAME `state` CSRF check as the loopback flow: the paste MUST carry a
/// `state` and it MUST equal the one we generated. An `error=` param aborts. A bare code with
/// no state is rejected — the state binding is the CSRF defense, not optional.
fn extract_code_from_paste(pasted: &str, expected_state: &str) -> Result<String> {
    let pasted = pasted.trim();
    if pasted.is_empty() {
        bail!("nothing pasted — copy the full redirect URL from your browser and try again");
    }
    let query: std::collections::HashMap<String, String> = if let Ok(url) = url::Url::parse(pasted)
    {
        url.query_pairs().into_owned().collect()
    } else if pasted.contains('=') {
        // A bare query string (`code=…&state=…`), possibly with a leading `?`.
        url::form_urlencoded::parse(pasted.trim_start_matches(['?', '/']).as_bytes())
            .into_owned()
            .collect()
    } else {
        bail!(
            "that doesn't look like the redirect — paste the full \
             http://localhost:…/callback?code=…&state=… address, not just the code"
        );
    };

    if let Some(err) = query.get("error") {
        bail!("authorization was denied or failed: {err}");
    }
    let state = query.get("state").filter(|s| !s.is_empty()).context(
        "the pasted URL has no `state` — paste the FULL redirect URL, not just the code",
    )?;
    if state.as_str() != expected_state {
        bail!(
            "state mismatch — the pasted URL is from a different login attempt (possible CSRF); \
             re-run `oura auth login --no-browser`"
        );
    }
    query
        .get("code")
        .filter(|c| !c.is_empty())
        .cloned()
        .context("the pasted URL has no `code` parameter — copy the full redirect address")
}

/// True when the environment looks like a remote/headless session where the loopback callback
/// likely can't reach this host — used only to SUGGEST `--no-browser` (#20). Injected env
/// lookup so the predicate is unit-tested without touching process env.
fn looks_headless(env: impl Fn(&str) -> Option<String>) -> bool {
    env("SSH_CONNECTION").is_some() || env("SSH_TTY").is_some()
}

/// Accept connections until a valid `/callback` arrives; validates `state` BEFORE rendering
/// the success page (so the browser never claims success on a CSRF mismatch) and returns the
/// authorization `code`.
async fn wait_for_callback(listener: &TcpListener, expected_state: &str) -> Result<String> {
    loop {
        let (mut stream, _) = listener.accept().await?;
        let req = match loopback::read_request(&mut stream).await {
            Ok(r) => r,
            Err(_) => continue,
        };
        if req.path != "/callback" {
            loopback::respond_html(&mut stream, "404 Not Found", &page_message("Not found", ""))
                .await
                .ok();
            continue;
        }
        if let Some(err) = req.query.get("error") {
            let detail = loopback::escape(err);
            loopback::respond_html(
                &mut stream,
                "400 Bad Request",
                &page_message("Authorization failed", &format!("<p>{detail}</p>")),
            )
            .await
            .ok();
            bail!("authorization was denied or failed: {err}");
        }
        match extract_code_state(&req) {
            Some((code, state)) => {
                if state != expected_state {
                    loopback::respond_html(
                        &mut stream,
                        "400 Bad Request",
                        &page_message(
                            "State mismatch",
                            "<p>The OAuth <code>state</code> did not match this login attempt \
                             — possible CSRF. Aborting; re-run <code>oura auth login</code>.</p>",
                        ),
                    )
                    .await
                    .ok();
                    bail!("state mismatch on the OAuth callback — possible CSRF; aborting");
                }
                loopback::respond_html(
                    &mut stream,
                    "200 OK",
                    &page_message(
                        "Authorized ✓",
                        "<p>You can close this tab and return to your terminal.</p>",
                    ),
                )
                .await
                .ok();
                return Ok(code);
            }
            None => {
                loopback::respond_html(
                    &mut stream,
                    "400 Bad Request",
                    &page_message("Missing authorization code", ""),
                )
                .await
                .ok();
            }
        }
    }
}

fn extract_code_state(req: &Request) -> Option<(String, String)> {
    let code = req.query.get("code")?.clone();
    let state = req.query.get("state").cloned().unwrap_or_default();
    Some((code, state))
}

fn persist(store: &TokenStore, tokens: &Tokens) -> Result<()> {
    // Take the store lock so a login can't interleave with a refresh a concurrently
    // running MCP server is performing (#22). If it's busy, say why we're pausing
    // (bounded: a peer's refresh holds the lock for at most ~2× its endpoint timeout)
    // instead of blocking silently.
    let _lock = match store.try_lock_exclusive()? {
        Some(lock) => lock,
        None => {
            eprintln!("waiting for another oura process to finish refreshing tokens…");
            store.lock_exclusive()?
        }
    };
    store.save_tokens(tokens)?;
    guide(&format!(
        "✓ Done. Tokens saved to {}",
        store.tokens_path().display()
    ));
    Ok(())
}

fn page_message(title: &str, body_html: &str) -> String {
    format!(
        r#"<!doctype html><html><head><meta charset="utf-8"><title>oura-toolkit</title>
<style>body{{font:15px/1.5 system-ui,sans-serif;max-width:32rem;margin:3rem auto;padding:0 1rem}}</style>
</head><body><h1>{title}</h1>{body}</body></html>"#,
        title = loopback::escape(title),
        body = body_html,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn req_with_query(pairs: &[(&str, &str)]) -> Request {
        let mut query = HashMap::new();
        for (k, v) in pairs {
            query.insert(k.to_string(), v.to_string());
        }
        Request {
            method: "GET".into(),
            path: "/callback".into(),
            query,
        }
    }

    #[test]
    fn redirect_uri_matches_registered_shape() {
        assert_eq!(redirect_uri(8788), "http://localhost:8788/callback");
    }

    #[test]
    fn extracts_code_and_state() {
        let req = req_with_query(&[("code", "abc"), ("state", "xyz")]);
        assert_eq!(extract_code_state(&req), Some(("abc".into(), "xyz".into())));
    }

    #[test]
    fn missing_code_yields_none() {
        let req = req_with_query(&[("state", "xyz")]);
        assert_eq!(extract_code_state(&req), None);
    }

    // --- --no-browser paste parsing (#20) ------------------------------------------------

    #[test]
    fn paste_accepts_a_full_url_or_bare_query_with_matching_state() {
        assert_eq!(
            extract_code_from_paste(
                "http://localhost:8788/callback?code=the-code&state=abc",
                "abc"
            )
            .unwrap(),
            "the-code"
        );
        assert_eq!(
            extract_code_from_paste("code=c1&state=s1", "s1").unwrap(),
            "c1"
        );
        assert_eq!(
            extract_code_from_paste("?code=c1&state=s1", "s1").unwrap(),
            "c1"
        );
    }

    #[test]
    fn paste_rejects_a_state_mismatch() {
        // Same CSRF defense as the loopback flow: a paste from a different attempt is refused.
        // Break-verify: drop the `state != expected_state` check and this stops failing.
        let err = extract_code_from_paste(
            "http://localhost:8788/callback?code=x&state=forged",
            "expected",
        )
        .unwrap_err();
        assert!(err.to_string().contains("state mismatch"), "{err}");
    }

    #[test]
    fn paste_requires_state_and_code_and_surfaces_errors() {
        // A code with no state is refused — the state binding is not optional.
        let err =
            extract_code_from_paste("http://localhost:8788/callback?code=x", "exp").unwrap_err();
        assert!(err.to_string().contains("no `state`"), "{err}");
        // A bare code (no query at all) is refused too.
        assert!(extract_code_from_paste("just-a-code", "exp")
            .unwrap_err()
            .to_string()
            .contains("doesn't look like"));
        // Matching state but no code.
        let err =
            extract_code_from_paste("http://localhost:8788/callback?state=exp", "exp").unwrap_err();
        assert!(err.to_string().contains("no `code`"), "{err}");
        // An OAuth error param aborts.
        let err =
            extract_code_from_paste("code=&error=access_denied&state=exp", "exp").unwrap_err();
        assert!(err.to_string().contains("access_denied"), "{err}");
        // Nothing pasted.
        assert!(extract_code_from_paste("   ", "exp")
            .unwrap_err()
            .to_string()
            .contains("nothing pasted"));
    }

    #[test]
    fn looks_headless_detects_ssh_env_only() {
        assert!(looks_headless(
            |k| (k == "SSH_CONNECTION").then(|| "1.2.3.4 5 6.7.8.9 22".to_string())
        ));
        assert!(looks_headless(
            |k| (k == "SSH_TTY").then(|| "/dev/pts/0".to_string())
        ));
        assert!(!looks_headless(|_| None), "a local session is not headless");
    }

    #[tokio::test]
    async fn setup_persists_credentials_before_consent_so_failure_keeps_them() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        let creds = ClientCredentials {
            client_id: "cid".into(),
            client_secret: "sec".into(),
        };

        let err = persist_credentials_then_authorize(&store, creds.clone(), |_| async {
            anyhow::bail!("user closed the consent tab")
        })
        .await
        .unwrap_err();
        assert!(err.to_string().contains("consent tab"));

        assert_eq!(
            store.load_credentials().unwrap().unwrap(),
            creds,
            "the pasted secret must survive an abandoned consent flow"
        );
        assert!(
            store.load_tokens().unwrap().is_none(),
            "no tokens on failure"
        );
    }

    #[tokio::test]
    async fn setup_happy_path_persists_both_records() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        let creds = ClientCredentials {
            client_id: "cid".into(),
            client_secret: "sec".into(),
        };
        let tokens = Tokens {
            access_token: "at".into(),
            refresh_token: "rt".into(),
            expires_at: 4_102_444_800,
            scope: None,
            token_type: None,
        };

        let t2 = tokens.clone();
        persist_credentials_then_authorize(&store, creds.clone(), move |_| async move { Ok(t2) })
            .await
            .unwrap();

        assert_eq!(store.load_credentials().unwrap().unwrap(), creds);
        assert_eq!(store.load_tokens().unwrap().unwrap(), tokens);
    }

    #[test]
    fn persist_fast_path_saves_immediately_when_uncontended() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        let tokens = Tokens {
            access_token: "at".into(),
            refresh_token: "rt".into(),
            expires_at: 4_102_444_800,
            scope: None,
            token_type: None,
        };
        persist(&store, &tokens).unwrap();
        assert_eq!(store.load_tokens().unwrap().unwrap(), tokens);
        // The fast path must leave the lock free (guard dropped, not leaked).
        assert!(store.try_lock_exclusive().unwrap().is_some());
    }

    #[test]
    fn persist_waits_for_the_store_lock_then_saves() {
        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        let tokens = Tokens {
            access_token: "at".into(),
            refresh_token: "rt".into(),
            expires_at: 4_102_444_800,
            scope: None,
            token_type: None,
        };

        // Simulate a concurrent MCP-server refresh holding the store lock.
        let guard = store.lock_exclusive().unwrap();

        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let (store2, tokens2) = (store.clone(), tokens.clone());
        let writer = std::thread::spawn(move || {
            persist(&store2, &tokens2).unwrap();
            done_tx.send(()).unwrap();
        });

        // While the lock is held, persist must block and nothing may be written.
        assert!(
            done_rx
                .recv_timeout(std::time::Duration::from_millis(300))
                .is_err(),
            "persist must not complete while the store lock is held"
        );
        assert!(store.load_tokens().unwrap().is_none());

        drop(guard);
        done_rx
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("persist must proceed once the lock is released");
        writer.join().unwrap();
        assert_eq!(store.load_tokens().unwrap().unwrap(), tokens);
    }

    #[test]
    fn prompt_values_are_trimmed_and_required() {
        assert_eq!(require_non_empty("  cid-123 \n"), Some("cid-123".into()));
        assert_eq!(require_non_empty("   \n"), None);
        assert_eq!(require_non_empty(""), None);
    }

    // --- status / logout / refresh / token (#18) ---------------------------------------

    use crate::output::{Format, Style};

    fn plain() -> RenderOptions {
        RenderOptions {
            format: Format::Plain,
            style: Style::new(false),
        }
    }

    fn seeded_store(
        dir: &tempfile::TempDir,
        credentials: bool,
        tokens: Option<Tokens>,
    ) -> TokenStore {
        let store = TokenStore::with_dir(dir.path());
        if credentials {
            store
                .save_credentials(&ClientCredentials {
                    client_id: "cid-123".into(),
                    client_secret: "SECRET-CS-789".into(),
                })
                .unwrap();
        }
        if let Some(t) = tokens {
            store.save_tokens(&t).unwrap();
        }
        store
    }

    fn tokens_expiring_at(expires_at: i64) -> Tokens {
        Tokens {
            access_token: "SECRET-AT-123".into(),
            refresh_token: "SECRET-RT-456".into(),
            expires_at,
            scope: Some("personal daily".into()),
            token_type: Some("Bearer".into()),
        }
    }

    #[test]
    fn human_duration_uses_the_two_most_significant_units() {
        assert_eq!(human_duration(0), "0s");
        assert_eq!(human_duration(59), "59s");
        assert_eq!(human_duration(60), "1m 0s");
        assert_eq!(human_duration(3_599), "59m 59s");
        assert_eq!(human_duration(3_600), "1h 0m");
        assert_eq!(human_duration(86_400 + 3_600), "1d 1h");
        assert_eq!(
            human_duration(-5),
            "0s",
            "negative clamps, never underflows"
        );
    }

    #[test]
    fn status_on_an_empty_store_reports_none_and_fails_toward_setup() {
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, false, None);
        let report = status_at(&store, plain(), 1_000).unwrap();
        assert_eq!(
            report.rendered,
            format!(
                "Store\t{}\nCredentials\tnone\nTokens\tnone\nAuthenticated\tno\n",
                dir.path().display()
            )
        );
        assert!(
            matches!(report.failure, Some(AuthError::MissingClientCredentials)),
            "empty store must route to the `auth setup` hint"
        );
    }

    #[test]
    fn status_with_credentials_only_shows_the_client_id_and_fails_toward_login() {
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, true, None);
        let report = status_at(&store, plain(), 1_000).unwrap();
        assert!(
            report
                .rendered
                .contains("Credentials\tpresent (client_id: cid-123)"),
            "client_id is deliberately shown: {}",
            report.rendered
        );
        assert!(
            matches!(report.failure, Some(AuthError::NotAuthenticated)),
            "credentials-without-tokens must route to the `auth login` hint"
        );
    }

    #[test]
    fn status_authenticated_reports_scope_and_humanized_expiry() {
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let store = seeded_store(&dir, true, Some(tokens_expiring_at(now + 3_600)));
        let report = status_at(&store, plain(), now).unwrap();
        assert!(report.failure.is_none(), "valid tokens = authenticated");
        assert_eq!(
            report.rendered,
            format!(
                "Store\t{}\nCredentials\tpresent (client_id: cid-123)\nTokens\tpresent\n\
                 Scope\tpersonal daily\nAccess token\texpires in 1h 0m\nAuthenticated\tyes\n",
                dir.path().display()
            )
        );
    }

    #[test]
    fn status_never_renders_secret_material_in_any_format() {
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, true, Some(tokens_expiring_at(2_000)));
        for format in [Format::Plain, Format::Table, Format::Json] {
            let opts = RenderOptions {
                format,
                style: Style::new(false),
            };
            let rendered = status_at(&store, opts, 1_000).unwrap().rendered;
            for secret in ["SECRET-CS-789", "SECRET-AT-123", "SECRET-RT-456"] {
                assert!(
                    !rendered.contains(secret),
                    "{format:?} status output leaked {secret}: {rendered}"
                );
            }
        }
    }

    #[test]
    fn status_with_expired_but_refreshable_tokens_is_authenticated() {
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let store = seeded_store(&dir, true, Some(tokens_expiring_at(now - 125)));
        let report = status_at(&store, plain(), now).unwrap();
        assert!(
            report.failure.is_none(),
            "refreshable = a data command works"
        );
        assert!(
            report
                .rendered
                .contains("expired 2m 5s ago (refreshes automatically on next use)"),
            "{}",
            report.rendered
        );
        assert!(report.rendered.contains("Authenticated\tyes"));
    }

    #[test]
    fn status_with_expired_unrefreshable_tokens_fails_toward_setup() {
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let store = seeded_store(&dir, false, Some(tokens_expiring_at(now - 60)));
        let report = status_at(&store, plain(), now).unwrap();
        assert!(
            matches!(report.failure, Some(AuthError::MissingClientCredentials)),
            "expired + no credentials cannot recover without setup"
        );
        assert!(
            report
                .rendered
                .contains("expired 1m 0s ago (no client credentials — cannot refresh)"),
            "{}",
            report.rendered
        );
        assert!(report.rendered.contains("Authenticated\tno"));
    }

    #[test]
    fn status_with_unexpired_tokens_but_no_credentials_is_usable_until_expiry() {
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let store = seeded_store(&dir, false, Some(tokens_expiring_at(now + 600)));
        let report = status_at(&store, plain(), now).unwrap();
        assert!(
            report.failure.is_none(),
            "a valid token works until expiry even without credentials"
        );
        assert!(
            report
                .rendered
                .contains("expires in 10m 0s (no client credentials — cannot refresh)"),
            "the unrefreshable caveat must be visible up front: {}",
            report.rendered
        );
    }

    #[test]
    fn status_inside_the_refresh_window_without_credentials_is_unauthenticated() {
        // The skew alignment (#62 review): a data command proactively refreshes within
        // REFRESH_SKEW_SECS of expiry, so an unrefreshable token there already fails auth —
        // status must predict that, not report the literal not-yet-expired state as OK.
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let inside_window = now + oura_toolkit_auth::REFRESH_SKEW_SECS / 2;

        let store = seeded_store(&dir, false, Some(tokens_expiring_at(inside_window)));
        let report = status_at(&store, plain(), now).unwrap();
        assert!(
            matches!(report.failure, Some(AuthError::MissingClientCredentials)),
            "inside the refresh window with no credentials = a data command fails auth"
        );
        assert!(
            report.rendered.contains("Authenticated\tno"),
            "{}",
            report.rendered
        );

        // Same expiry WITH credentials: the manager would just refresh — authenticated.
        let store2 = seeded_store(&dir, true, Some(tokens_expiring_at(inside_window)));
        let report2 = status_at(&store2, plain(), now).unwrap();
        assert!(
            report2.failure.is_none(),
            "refreshable inside the window is fine"
        );
    }

    #[test]
    fn status_at_the_exact_expiry_instant_reports_expired() {
        // Boundary pin: `now >= expires_at` and the phrase's `now < expires_at` are exact
        // complements — a drift to strict `>` / `<=` flips this case.
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let store = seeded_store(&dir, false, Some(tokens_expiring_at(now)));
        let opts = RenderOptions {
            format: Format::Json,
            style: Style::new(false),
        };
        let report = status_at(&store, opts, now).unwrap();
        let v: serde_json::Value = serde_json::from_str(&report.rendered).unwrap();
        assert_eq!(v["tokens"]["expired"], true, "now == expires_at is expired");
        assert_eq!(v["authenticated"], false);

        let text = status_at(&store, plain(), now).unwrap();
        assert!(
            text.rendered
                .contains("expired 0s ago (no client credentials — cannot refresh)"),
            "{}",
            text.rendered
        );
    }

    #[test]
    fn status_sanitizes_a_hostile_scope_in_every_text_format() {
        // The scope string comes from the token endpoint (server-controlled): it must not
        // inject terminal escapes or forge lines/fields in the rendered report (CLAUDE.md
        // TESTING rule 5 — attack test per sanitization invariant).
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let mut tokens = tokens_expiring_at(now + 3_600);
        tokens.scope = Some("personal\u{1b}[2Jdaily\ttag\nforged\tline".into());
        let store = seeded_store(&dir, true, Some(tokens));

        for format in [Format::Plain, Format::Table] {
            let opts = RenderOptions {
                format,
                style: Style::new(false),
            };
            let rendered = status_at(&store, opts, now).unwrap().rendered;
            assert!(
                !rendered.contains('\u{1b}'),
                "{format:?}: escape byte must be stripped: {rendered:?}"
            );
            assert_eq!(
                rendered.lines().count(),
                6,
                "{format:?}: an embedded newline must not forge a report line: {rendered:?}"
            );
        }
        // Plain format specifically: exactly one tab per line (the label separator) — an
        // embedded tab in the scope must not forge an extra field.
        let rendered = status_at(&store, plain(), now).unwrap().rendered;
        for line in rendered.lines() {
            assert_eq!(
                line.matches('\t').count(),
                1,
                "forged field in plain line: {line:?}"
            );
        }
        // JSON is serde-escaped: it must round-trip the raw value, not corrupt output.
        let opts = RenderOptions {
            format: Format::Json,
            style: Style::new(false),
        };
        let rendered = status_at(&store, opts, now).unwrap().rendered;
        let v: serde_json::Value = serde_json::from_str(&rendered).unwrap();
        assert_eq!(
            v["tokens"]["scope"],
            "personal\u{1b}[2Jdaily\ttag\nforged\tline"
        );
    }

    #[test]
    fn status_json_is_the_serde_model_without_secret_material() {
        let dir = tempfile::tempdir().unwrap();
        let now = 1_000_000;
        let store = seeded_store(&dir, true, Some(tokens_expiring_at(now + 3_600)));
        let opts = RenderOptions {
            format: Format::Json,
            style: Style::new(false),
        };
        let rendered = status_at(&store, opts, now).unwrap().rendered;
        let v: serde_json::Value = serde_json::from_str(&rendered).unwrap();
        assert_eq!(v["authenticated"], true);
        assert_eq!(v["credentials"]["present"], true);
        assert_eq!(v["credentials"]["client_id"], "cid-123");
        assert_eq!(v["tokens"]["present"], true);
        assert_eq!(v["tokens"]["scope"], "personal daily");
        assert_eq!(v["tokens"]["expires_at"], now + 3_600);
        assert_eq!(v["tokens"]["expired"], false);
        assert_eq!(v["store"], dir.path().display().to_string());
    }

    #[test]
    fn logout_removes_tokens_keeps_credentials_and_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, true, Some(tokens_expiring_at(2_000)));

        let out = logout(&store, false).unwrap();
        assert!(out.contains("✓ Logged out — removed"), "{out}");
        assert!(
            out.contains(
                "Client credentials kept — remove them too with `oura auth logout --all`."
            ),
            "{out}"
        );
        assert!(!store.tokens_path().exists(), "tokens.json must be deleted");
        assert!(
            store.load_credentials().unwrap().is_some(),
            "plain logout must keep the registered app credentials"
        );

        let again = logout(&store, false).unwrap();
        assert!(
            again.contains("No tokens stored — already logged out."),
            "idempotent: {again}"
        );
    }

    #[test]
    fn logout_all_removes_the_client_credentials_too() {
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, true, Some(tokens_expiring_at(2_000)));

        let out = logout(&store, true).unwrap();
        assert!(out.contains("✓ Removed client credentials —"), "{out}");
        assert!(!store.tokens_path().exists());
        assert!(
            !store.credentials_path().exists(),
            "--all is the sanctioned way to remove the stored secret"
        );
        assert!(!out.contains("--all`"), "no upsell once everything is gone");
    }

    #[test]
    fn logout_all_with_credentials_but_no_tokens_reports_both_accurately() {
        // The mixed state: nothing to log out of, but a secret to remove.
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, true, None);
        let out = logout(&store, true).unwrap();
        assert!(
            out.contains("No tokens stored — already logged out."),
            "{out}"
        );
        assert!(out.contains("✓ Removed client credentials —"), "{out}");
        assert!(!store.credentials_path().exists());
    }

    #[test]
    fn logout_waits_for_the_store_lock_before_deleting() {
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, false, Some(tokens_expiring_at(2_000)));

        // Simulate a concurrent refresh holding the store lock (#22).
        let guard = store.lock_exclusive().unwrap();

        let (done_tx, done_rx) = std::sync::mpsc::channel();
        let store2 = store.clone();
        let deleter = std::thread::spawn(move || {
            logout(&store2, false).unwrap();
            done_tx.send(()).unwrap();
        });

        assert!(
            done_rx
                .recv_timeout(std::time::Duration::from_millis(300))
                .is_err(),
            "logout must not delete while a peer holds the store lock"
        );
        assert!(
            store.tokens_path().exists(),
            "nothing deleted under the lock"
        );

        drop(guard);
        done_rx
            .recv_timeout(std::time::Duration::from_secs(5))
            .expect("logout must proceed once the lock is released");
        deleter.join().unwrap();
        assert!(!store.tokens_path().exists());
    }

    #[tokio::test]
    async fn refresh_persists_the_rotation_and_reports_the_new_expiry() {
        use wiremock::matchers::{body_string_contains, method};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_string_contains("refresh_token=r1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "SECRET-NEW-ACCESS",
                "refresh_token": "SECRET-NEW-REFRESH",
                "expires_in": 3600
            })))
            .expect(1)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        let credentials = ClientCredentials {
            client_id: "cid".into(),
            client_secret: "sec".into(),
        };
        store.save_credentials(&credentials).unwrap();
        let stale = Tokens {
            access_token: "stale".into(),
            refresh_token: "r1".into(),
            expires_at: 0,
            scope: None,
            token_type: None,
        };
        store.save_tokens(&stale).unwrap();
        let mut manager = TokenManager::from_parts(store.clone(), Some(credentials), Some(stale));
        manager.override_token_url(server.uri());

        let out = refresh(&manager, &store).await.unwrap();
        assert!(out.contains("✓ Tokens refreshed and persisted to"), "{out}");
        assert!(out.contains("expires in"), "{out}");
        // Only `auth token` may print token material — the refresh confirmation is the one
        // success output produced while fresh tokens are in scope, so pin the negative.
        assert!(
            !out.contains("SECRET-NEW-ACCESS") && !out.contains("SECRET-NEW-REFRESH"),
            "refresh confirmation must not echo token material: {out}"
        );
        assert_eq!(
            store.load_tokens().unwrap().unwrap().refresh_token,
            "SECRET-NEW-REFRESH",
            "the rotated refresh token MUST be persisted (Oura invalidates r1)"
        );
    }

    #[tokio::test]
    async fn token_prints_exactly_the_access_token_and_a_newline() {
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, true, None);
        let fresh = Tokens {
            access_token: "at-fresh".into(),
            refresh_token: "rt".into(),
            expires_at: 4_102_444_800,
            scope: None,
            token_type: None,
        };
        store.save_tokens(&fresh).unwrap();
        let manager = TokenManager::from_parts(
            store.clone(),
            store.load_credentials().unwrap(),
            Some(fresh),
        );
        assert_eq!(token(&manager, &store).await.unwrap(), "at-fresh\n");
    }

    #[tokio::test]
    async fn token_refreshes_an_expired_token_before_printing() {
        use wiremock::matchers::{body_string_contains, method};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(body_string_contains("refresh_token=r1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "refreshed-access",
                "refresh_token": "r2",
                "expires_in": 3600
            })))
            .expect(1)
            .mount(&server)
            .await;

        let dir = tempfile::tempdir().unwrap();
        let store = TokenStore::with_dir(dir.path());
        let credentials = ClientCredentials {
            client_id: "cid".into(),
            client_secret: "sec".into(),
        };
        store.save_credentials(&credentials).unwrap();
        let stale = Tokens {
            access_token: "stale".into(),
            refresh_token: "r1".into(),
            expires_at: 0,
            scope: None,
            token_type: None,
        };
        store.save_tokens(&stale).unwrap();
        let mut manager = TokenManager::from_parts(store.clone(), Some(credentials), Some(stale));
        manager.override_token_url(server.uri());

        assert_eq!(token(&manager, &store).await.unwrap(), "refreshed-access\n");
        assert_eq!(store.load_tokens().unwrap().unwrap().refresh_token, "r2");
    }

    #[tokio::test]
    async fn token_refuses_to_print_a_token_containing_control_characters() {
        // Attack test for the fail-closed check: `auth token` is the one output that
        // bypasses `output::sanitize`, so a hostile token endpoint must not be able to
        // smuggle terminal escapes or word-splitting newlines through it.
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, true, None);
        let hostile = Tokens {
            access_token: "evil\u{1b}[2J\ntoken".into(),
            refresh_token: "rt".into(),
            expires_at: 4_102_444_800,
            scope: None,
            token_type: None,
        };
        store.save_tokens(&hostile).unwrap();
        let manager = TokenManager::from_parts(
            store.clone(),
            store.load_credentials().unwrap(),
            Some(hostile),
        );
        let err = token(&manager, &store).await.unwrap_err();
        let msg = format!("{err:#}");
        assert!(msg.contains("control characters"), "{msg}");
        assert!(
            !msg.contains('\u{1b}'),
            "the error itself must not echo the hostile bytes: {msg:?}"
        );
    }

    #[tokio::test]
    async fn token_upgrades_not_authenticated_on_an_empty_store() {
        // Empty store: NotAuthenticated would hint `auth login`, which itself fails asking
        // for setup — the commands must point at the real first step instead.
        let dir = tempfile::tempdir().unwrap();
        let store = seeded_store(&dir, false, None);
        let manager = TokenManager::from_parts(store.clone(), None, None);

        let err = token(&manager, &store).await.unwrap_err();
        assert!(
            matches!(
                err.downcast_ref::<AuthError>(),
                Some(AuthError::MissingClientCredentials)
            ),
            "{err:?}"
        );

        // With credentials stored, the plain NotAuthenticated (→ login hint) is correct.
        let store2 = seeded_store(&dir, true, None);
        let manager2 =
            TokenManager::from_parts(store2.clone(), store2.load_credentials().unwrap(), None);
        let err2 = token(&manager2, &store2).await.unwrap_err();
        assert!(
            matches!(
                err2.downcast_ref::<AuthError>(),
                Some(AuthError::NotAuthenticated)
            ),
            "{err2:?}"
        );
    }

    #[tokio::test]
    async fn callback_state_mismatch_rejects_and_aborts() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpStream;

        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server =
            tokio::spawn(async move { wait_for_callback(&listener, "expected-state").await });

        let mut c = TcpStream::connect(addr).await.unwrap();
        c.write_all(
            b"GET /callback?code=abc&state=forged HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        )
        .await
        .unwrap();
        let mut resp = String::new();
        c.read_to_string(&mut resp).await.unwrap();
        assert!(resp.contains("400"), "browser must NOT see success: {resp}");
        assert!(resp.contains("State mismatch"));

        let err = server.await.unwrap().unwrap_err();
        assert!(err.to_string().contains("state mismatch"));
    }

    #[tokio::test]
    async fn callback_with_valid_state_returns_code() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpStream;

        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server =
            tokio::spawn(async move { wait_for_callback(&listener, "expected-state").await });

        let mut c = TcpStream::connect(addr).await.unwrap();
        c.write_all(
            b"GET /callback?code=abc&state=expected-state HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        )
        .await
        .unwrap();
        let mut resp = String::new();
        c.read_to_string(&mut resp).await.unwrap();
        assert!(resp.contains("Authorized"));

        assert_eq!(server.await.unwrap().unwrap(), "abc");
    }
}
