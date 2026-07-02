//! Interactive OAuth flows — the browser + loopback consent that lives ONLY in the CLI.
//!
//! `setup` guides the user through registering their own Oura OAuth app (BYO confidential
//! credentials) and collects `client_id`/`client_secret` via terminal prompts — the secret is
//! read with echo disabled and never leaves this process except to Oura's token endpoint.
//! `login` runs the Authorization Code flow against a fixed loopback `redirect_uri`, catches
//! the code, and exchanges it via `oura-toolkit-auth`. The non-interactive token machinery
//! (exchange/refresh/store) all lives in `oura-toolkit-auth`.

use anyhow::{anyhow, bail, Context, Result};
use oura_toolkit_auth::{exchange_code, metadata, TokenStore, Tokens};
use tokio::net::TcpListener;

use crate::loopback::{self, Request};

/// How long `auth login` waits for the browser callback before giving up.
const CALLBACK_TIMEOUT_SECS: u64 = 300;

/// `oura auth setup` — register an app (terminal prompts), then log in.
pub async fn setup(port: u16) -> Result<()> {
    let redirect_uri = redirect_uri(port);
    let scopes = metadata::default_scopes();
    let scope_str = scopes.join(" ");

    println!("== Register your Oura OAuth application ==\n");
    println!("Opening https://cloud.ouraring.com/oauth/applications in your browser…");
    let _ = open::that("https://cloud.ouraring.com/oauth/applications");
    println!("Create an application with these EXACT values:\n");
    println!("  • Application name : oura-toolkit   (or any name you like)");
    println!("  • Redirect URI     : {redirect_uri}");
    println!("  • Scopes           : {scope_str}\n");

    // Bind the callback listener up front so a port conflict fails fast, before the user
    // has typed anything.
    let listener = bind(port).await?;

    // Terminal prompts, no local HTTP surface: the client_id is echoed (it is not a secret);
    // the secret is read with echo disabled.
    println!("Paste the credentials Oura shows for your new application:");
    let client_id = prompt_required("  Client ID: ")?;
    let client_secret = prompt_secret_required("  Client Secret (input hidden): ")?;
    println!("✓ Received credentials — the secret stays on this machine.\n");

    println!("Continuing to login…");
    let tokens = run_authorization(&listener, port, &client_id, &client_secret).await?;
    persist(&tokens)?;
    Ok(())
}

/// `oura auth login` — Authorization Code flow using stored client credentials.
pub async fn login(port: u16) -> Result<()> {
    let store = TokenStore::new()?;
    let (client_id, client_secret) = match store.load()? {
        Some(t) => (t.client_id, t.client_secret),
        None => bail!("no client credentials found — run `oura auth setup` first"),
    };
    let listener = bind(port).await?;
    let tokens = run_authorization(&listener, port, &client_id, &client_secret).await?;
    persist(&tokens)?;
    Ok(())
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

/// Prompt (echoed) until the user enters a non-empty value.
fn prompt_required(label: &str) -> Result<String> {
    use std::io::Write as _;
    loop {
        print!("{label}");
        std::io::stdout().flush()?;
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
    client_id: &str,
    client_secret: &str,
) -> Result<Tokens> {
    let redirect_uri = redirect_uri(port);
    let state = uuid::Uuid::new_v4().to_string();
    let scopes = metadata::default_scopes();
    let authorize_url = metadata::authorize_url(client_id, &redirect_uri, &scopes, &state);

    println!("Opening your browser to authorize with Oura…");
    println!("If it doesn't open automatically, visit:\n  {authorize_url}\n");
    let _ = open::that(&authorize_url);

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

    let http = reqwest::Client::new();
    exchange_code(&http, client_id, client_secret, &code, &redirect_uri)
        .await
        .context("token exchange with the Oura token endpoint failed")
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

fn persist(tokens: &Tokens) -> Result<()> {
    let store = TokenStore::new()?;
    store.save(tokens)?;
    println!("✓ Done. Credentials saved to {}", store.path().display());
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

    #[test]
    fn prompt_values_are_trimmed_and_required() {
        assert_eq!(require_non_empty("  cid-123 \n"), Some("cid-123".into()));
        assert_eq!(require_non_empty("   \n"), None);
        assert_eq!(require_non_empty(""), None);
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
