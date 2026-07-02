//! Interactive OAuth flows — the browser + loopback consent that lives ONLY in the CLI.
//!
//! `setup` guides the user through registering their own Oura OAuth app (BYO confidential
//! credentials), collects `client_id`/`client_secret` via a localhost paste box (the secret
//! never leaves the machine), then chains into `login`. `login` runs the Authorization Code
//! flow against a fixed loopback `redirect_uri`, catches the code, and exchanges it via
//! `oura-toolkit-auth`. The non-interactive token machinery (exchange/refresh/store) all lives
//! in `oura-toolkit-auth`.

use anyhow::{anyhow, bail, Context, Result};
use oura_toolkit_auth::{exchange_code, metadata, TokenStore, Tokens};
use tokio::net::TcpListener;

use crate::loopback::{self, Request};

/// How long `auth login` waits for the browser callback before giving up.
const CALLBACK_TIMEOUT_SECS: u64 = 300;

/// `oura auth setup` — register an app (paste box), then log in.
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

    let listener = bind(port).await?;
    let paste_url = format!("http://localhost:{port}/");
    println!("Then paste your Client ID and Client Secret into:\n  {paste_url}\n");
    let _ = open::that(&paste_url);

    // Per-run anti-CSRF nonce: while the paste box listens, any webpage the user visits could
    // fire a cross-origin form POST at /save — only the form we served knows the nonce.
    let nonce = uuid::Uuid::new_v4().to_string();
    let (client_id, client_secret) =
        collect_credentials(&listener, &redirect_uri, &scope_str, &nonce)
            .await
            .context("collecting client credentials")?;
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

/// Serve the paste box until the user submits non-empty credentials with a valid nonce.
async fn collect_credentials(
    listener: &TcpListener,
    redirect_uri: &str,
    scopes: &str,
    nonce: &str,
) -> Result<(String, String)> {
    loop {
        let (mut stream, _) = listener.accept().await?;
        let req = match loopback::read_request(&mut stream).await {
            Ok(r) => r,
            Err(_) => continue,
        };
        match (req.method.as_str(), req.path.as_str()) {
            ("GET", "/") => {
                let page = paste_page(redirect_uri, scopes, nonce);
                loopback::respond_html(&mut stream, "200 OK", &page)
                    .await
                    .ok();
            }
            ("POST", "/save") => {
                // Reject POSTs that don't carry the nonce we embedded in the served form —
                // they didn't come from our page (drive-by cross-origin form post).
                if req.form.get("nonce").map(String::as_str) != Some(nonce) {
                    loopback::respond_html(
                        &mut stream,
                        "403 Forbidden",
                        &page_message(
                            "Rejected",
                            "<p>This submission did not come from the form this run served. \
                             <a href=\"/\">Reload the form</a> and try again.</p>",
                        ),
                    )
                    .await
                    .ok();
                    continue;
                }
                let id = req
                    .form
                    .get("client_id")
                    .map(|s| s.trim())
                    .unwrap_or_default();
                let secret = req
                    .form
                    .get("client_secret")
                    .map(|s| s.trim())
                    .unwrap_or_default();
                if id.is_empty() || secret.is_empty() {
                    loopback::respond_html(
                        &mut stream,
                        "400 Bad Request",
                        &page_message(
                            "Both fields are required.",
                            "<p><a href=\"/\">Go back</a></p>",
                        ),
                    )
                    .await
                    .ok();
                    continue;
                }
                loopback::respond_html(
                    &mut stream,
                    "200 OK",
                    &page_message(
                        "Credentials received ✓",
                        "<p>Return to your terminal — continuing to authorize with Oura.</p>",
                    ),
                )
                .await
                .ok();
                return Ok((id.to_string(), secret.to_string()));
            }
            _ => {
                loopback::respond_html(
                    &mut stream,
                    "404 Not Found",
                    &page_message("Not found", ""),
                )
                .await
                .ok();
            }
        }
    }
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

fn paste_page(redirect_uri: &str, scopes: &str, nonce: &str) -> String {
    format!(
        r#"<!doctype html><html><head><meta charset="utf-8"><title>oura-toolkit setup</title>
<style>body{{font:15px/1.5 system-ui,sans-serif;max-width:32rem;margin:3rem auto;padding:0 1rem}}
label{{display:block;margin:1rem 0 .25rem;font-weight:600}}input{{width:100%;padding:.5rem;font:inherit}}
button{{margin-top:1.5rem;padding:.6rem 1.2rem;font:inherit;cursor:pointer}}code{{background:#f2f2f2;padding:.1rem .3rem}}</style>
</head><body>
<h1>Connect your Oura app</h1>
<p>Register an app at <code>cloud.ouraring.com/oauth/applications</code> with Redirect URI
<code>{redirect}</code> and scopes <code>{scopes}</code>, then paste its credentials below.
They are sent only to this local page and never leave your machine.</p>
<form method="POST" action="/save">
<input type="hidden" name="nonce" value="{nonce}">
<label for="cid">Client ID</label><input id="cid" name="client_id" autocomplete="off" required>
<label for="sec">Client Secret</label><input id="sec" name="client_secret" type="password" autocomplete="off" required>
<button type="submit">Save &amp; continue</button>
</form></body></html>"#,
        redirect = loopback::escape(redirect_uri),
        scopes = loopback::escape(scopes),
        nonce = loopback::escape(nonce),
    )
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
            form: HashMap::new(),
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

    async fn post_form(addr: std::net::SocketAddr, body: &str) -> String {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpStream;
        let req = format!(
            "POST /save HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let mut c = TcpStream::connect(addr).await.unwrap();
        c.write_all(req.as_bytes()).await.unwrap();
        let mut resp = String::new();
        c.read_to_string(&mut resp).await.unwrap();
        resp
    }

    #[tokio::test]
    async fn paste_box_requires_nonce_then_captures_credentials() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpStream;

        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            collect_credentials(
                &listener,
                "http://localhost:8788/callback",
                "daily personal",
                "test-nonce",
            )
            .await
        });

        // GET / returns the paste form carrying the nonce.
        let mut c = TcpStream::connect(addr).await.unwrap();
        c.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        let mut page = String::new();
        c.read_to_string(&mut page).await.unwrap();
        assert!(page.contains("Client ID"), "form should render");
        assert!(page.contains("test-nonce"), "form should embed the nonce");

        // A drive-by POST without the nonce is rejected and the flow keeps waiting.
        let rejected = post_form(addr, "client_id=evil&client_secret=evil&nonce=wrong").await;
        assert!(
            rejected.contains("403"),
            "bad nonce must be rejected: {rejected}"
        );

        // POST /save with the nonce completes the flow.
        let ack = post_form(
            addr,
            "client_id=cid123&client_secret=sec%2F456&nonce=test-nonce",
        )
        .await;
        assert!(ack.contains("received"), "should acknowledge receipt");

        let (id, secret) = server.await.unwrap().unwrap();
        assert_eq!(id, "cid123");
        assert_eq!(secret, "sec/456"); // percent-decoded
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
