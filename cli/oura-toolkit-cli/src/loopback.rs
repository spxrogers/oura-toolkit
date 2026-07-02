//! A minimal single-purpose loopback HTTP server for the interactive OAuth flows.
//!
//! This is NOT a general HTTP client/transport (which the toolkit never hand-writes) — it is the
//! tiny localhost listener the OAuth spec requires: it reads one request, hands back the parsed
//! method/path/query/form, and writes a small HTML page. Bodies are `application/x-www-form-
//! urlencoded` (the paste box). Everything stays on 127.0.0.1.

use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const MAX_REQUEST_BYTES: usize = 64 * 1024;

/// A parsed HTTP request (only the parts the OAuth flows need).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub form: HashMap<String, String>,
}

/// Read and parse one request from a connection.
pub async fn read_request(stream: &mut TcpStream) -> Result<Request> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 2048];
    let header_end = loop {
        let n = stream.read(&mut chunk).await?;
        if n == 0 {
            bail!("connection closed before the request completed");
        }
        buf.extend_from_slice(&chunk[..n]);
        if let Some(pos) = find(&buf, b"\r\n\r\n") {
            break pos;
        }
        if buf.len() > MAX_REQUEST_BYTES {
            bail!("request headers exceeded {MAX_REQUEST_BYTES} bytes");
        }
    };

    let head = String::from_utf8_lossy(&buf[..header_end]).into_owned();
    let mut req = parse_head(&head)?;

    // Read the body if the request declares one (POST from the paste box). The declared
    // length is attacker-controlled (anything can connect to the loopback port), so cap it
    // BEFORE allocating/reading rather than trusting the header.
    if let Some(len) = content_length(&head) {
        if len > MAX_REQUEST_BYTES {
            bail!("request body exceeded {MAX_REQUEST_BYTES} bytes");
        }
        let mut body = buf[header_end + 4..].to_vec();
        while body.len() < len {
            let n = stream.read(&mut chunk).await?;
            if n == 0 {
                break;
            }
            body.extend_from_slice(&chunk[..n]);
        }
        body.truncate(len);
        if req.method == "POST" {
            req.form = parse_urlencoded(&String::from_utf8_lossy(&body));
        }
    }
    Ok(req)
}

/// Parse the request line + headers (no body).
fn parse_head(head: &str) -> Result<Request> {
    let mut lines = head.split("\r\n");
    let request_line = lines.next().context("empty request")?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or("/").to_string();

    let (path, query_str) = match target.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (target, String::new()),
    };
    Ok(Request {
        method,
        path,
        query: parse_urlencoded(&query_str),
        form: HashMap::new(),
    })
}

fn content_length(head: &str) -> Option<usize> {
    head.split("\r\n")
        .filter_map(|l| l.split_once(':'))
        .find(|(k, _)| k.trim().eq_ignore_ascii_case("content-length"))
        .and_then(|(_, v)| v.trim().parse().ok())
}

fn parse_urlencoded(s: &str) -> HashMap<String, String> {
    url::form_urlencoded::parse(s.as_bytes())
        .into_owned()
        .collect()
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

/// Write a small HTML response and close the connection.
pub async fn respond_html(stream: &mut TcpStream, status: &str, html: &str) -> Result<()> {
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{html}",
        html.len()
    );
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}

/// Minimal HTML escaping for values interpolated into response pages.
pub fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_get_with_query() {
        let req = parse_head("GET /callback?code=abc123&state=xyz HTTP/1.1").unwrap();
        assert_eq!(req.method, "GET");
        assert_eq!(req.path, "/callback");
        assert_eq!(req.query.get("code").unwrap(), "abc123");
        assert_eq!(req.query.get("state").unwrap(), "xyz");
    }

    #[test]
    fn parses_percent_encoded_form_body() {
        let form = parse_urlencoded("client_id=abc&client_secret=s%2Fecret%3D");
        assert_eq!(form.get("client_id").unwrap(), "abc");
        assert_eq!(form.get("client_secret").unwrap(), "s/ecret=");
    }

    #[test]
    fn reads_content_length() {
        let head = "POST /save HTTP/1.1\r\nHost: localhost\r\nContent-Length: 42\r\n";
        assert_eq!(content_length(head), Some(42));
    }

    #[test]
    fn escapes_html() {
        assert_eq!(escape("<a>&\"'"), "&lt;a&gt;&amp;&quot;'");
    }

    #[tokio::test]
    async fn rejects_oversized_declared_body() {
        use tokio::io::AsyncWriteExt;
        use tokio::net::{TcpListener, TcpStream};

        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();
        let client = tokio::spawn(async move {
            let mut s = TcpStream::connect(addr).await.unwrap();
            s.write_all(
                b"POST /save HTTP/1.1\r\nHost: localhost\r\nContent-Length: 999999999\r\n\r\n",
            )
            .await
            .unwrap();
            s // keep the connection open so the server side decides, not EOF
        });

        let (mut stream, _) = listener.accept().await.unwrap();
        let err = read_request(&mut stream).await.unwrap_err();
        assert!(
            err.to_string().contains("body exceeded"),
            "oversized Content-Length must be rejected before reading: {err}"
        );
        drop(client.await.unwrap());
    }
}
