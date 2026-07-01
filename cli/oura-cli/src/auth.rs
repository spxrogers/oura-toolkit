//! Interactive OAuth flows — the browser + loopback consent that lives ONLY in the CLI.
//!
//! `setup` guides the user through registering their own Oura OAuth app (BYO confidential
//! credentials), collects `client_id`/`client_secret` via a localhost paste box (the secret
//! never leaves the machine), then chains into `login`. `login` runs the Authorization Code
//! flow against a fixed loopback `redirect_uri`, catches the code, and exchanges it via
//! `oura-auth`. The non-interactive token machinery (exchange/refresh/store) all lives in
//! `oura-auth`.

use anyhow::{bail, Context, Result};
use oura_auth::{exchange_code, metadata, TokenStore, Tokens};
use tokio::net::TcpListener;

use crate::loopback::{self, Request};

/// `oura-cli auth setup` — register an app (paste box), then log in.
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

    let (client_id, client_secret) = collect_credentials(&listener, &redirect_uri, &scope_str)
        .await
        .context("collecting client credentials")?;
    println!("✓ Received credentials — the secret stays on this machine.\n");

    println!("Continuing to login…");
    let tokens = run_authorization(&listener, port, &client_id, &client_secret).await?;
    persist(&tokens)?;
    Ok(())
}

/// `oura-cli auth login` — Authorization Code flow using stored client credentials.
pub async fn login(port: u16) -> Result<()> {
    let store = TokenStore::new()?;
    let (client_id, client_secret) = match store.load()? {
        Some(t) => (t.client_id, t.client_secret),
        None => bail!("no client credentials found — run `oura-cli auth setup` first"),
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

/// Serve the paste box until the user submits non-empty credentials.
async fn collect_credentials(
    listener: &TcpListener,
    redirect_uri: &str,
    scopes: &str,
) -> Result<(String, String)> {
    loop {
        let (mut stream, _) = listener.accept().await?;
        let req = match loopback::read_request(&mut stream).await {
            Ok(r) => r,
            Err(_) => continue,
        };
        match (req.method.as_str(), req.path.as_str()) {
            ("GET", "/") => {
                let page = paste_page(redirect_uri, scopes);
                loopback::respond_html(&mut stream, "200 OK", &page)
                    .await
                    .ok();
            }
            ("POST", "/save") => {
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

    let (code, returned_state) = wait_for_callback(listener).await?;
    if returned_state != state {
        bail!("state mismatch on the OAuth callback — possible CSRF; aborting");
    }

    let http = reqwest::Client::new();
    exchange_code(&http, client_id, client_secret, &code, &redirect_uri)
        .await
        .context("token exchange with the Oura token endpoint failed")
}

/// Accept connections until the `/callback` arrives; returns `(code, state)`.
async fn wait_for_callback(listener: &TcpListener) -> Result<(String, String)> {
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
                return Ok((code, state));
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

fn paste_page(redirect_uri: &str, scopes: &str) -> String {
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
<label for="cid">Client ID</label><input id="cid" name="client_id" autocomplete="off" required>
<label for="sec">Client Secret</label><input id="sec" name="client_secret" type="password" autocomplete="off" required>
<button type="submit">Save &amp; continue</button>
</form></body></html>"#,
        redirect = loopback::escape(redirect_uri),
        scopes = loopback::escape(scopes),
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

    #[tokio::test]
    async fn paste_box_serves_form_then_captures_credentials() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpStream;

        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            collect_credentials(
                &listener,
                "http://localhost:8788/callback",
                "daily personal",
            )
            .await
        });

        // GET / returns the paste form.
        let mut c = TcpStream::connect(addr).await.unwrap();
        c.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        let mut page = String::new();
        c.read_to_string(&mut page).await.unwrap();
        assert!(page.contains("Client ID"), "form should render");

        // POST /save with credentials completes the flow.
        let body = "client_id=cid123&client_secret=sec%2F456";
        let req = format!(
            "POST /save HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        let mut c2 = TcpStream::connect(addr).await.unwrap();
        c2.write_all(req.as_bytes()).await.unwrap();
        let mut ack = String::new();
        c2.read_to_string(&mut ack).await.unwrap();
        assert!(ack.contains("received"), "should acknowledge receipt");

        let (id, secret) = server.await.unwrap().unwrap();
        assert_eq!(id, "cid123");
        assert_eq!(secret, "sec/456"); // percent-decoded
    }
}
