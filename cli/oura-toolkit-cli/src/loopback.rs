//! A minimal single-purpose loopback HTTP listener for the interactive OAuth flows.
//!
//! This is NOT a general HTTP client/transport (which the toolkit never hand-writes) — it is
//! the tiny localhost listener the OAuth loopback redirect requires: it reads one request's
//! head, hands back the parsed method/path/query, and writes a small HTML page. It is
//! deliberately GET-only — request bodies are never read (credentials arrive via terminal
//! prompts, not HTTP), which keeps the surface to "parse one request line". Everything stays
//! on 127.0.0.1.

use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

/// Cap on the request head (request line + headers). Anything can connect to the loopback
/// port, so never trust a peer to terminate its request.
const MAX_REQUEST_BYTES: usize = 64 * 1024;

/// A parsed HTTP request (only the parts the OAuth callback needs).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
}

/// Read one request's head from a connection and parse it. Any body is ignored — the
/// callback flow only ever needs the request line's query parameters.
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
    parse_head(&head)
}

/// Parse the request line (headers and any body are irrelevant to the callback flow).
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
    })
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
    fn parses_percent_encoded_query() {
        let req = parse_head("GET /callback?code=a%2Fb%3D&state=x%20y HTTP/1.1").unwrap();
        assert_eq!(req.query.get("code").unwrap(), "a/b=");
        assert_eq!(req.query.get("state").unwrap(), "x y");
    }

    #[test]
    fn escapes_html() {
        assert_eq!(escape("<a>&\"'"), "&lt;a&gt;&amp;&quot;'");
    }

    #[tokio::test]
    async fn rejects_unterminated_oversized_head() {
        use tokio::io::AsyncWriteExt;
        use tokio::net::{TcpListener, TcpStream};

        let listener = TcpListener::bind(("127.0.0.1", 0)).await.unwrap();
        let addr = listener.local_addr().unwrap();
        let client = tokio::spawn(async move {
            let mut s = TcpStream::connect(addr).await.unwrap();
            // Stream endless header bytes with no terminating blank line.
            let junk = vec![b'a'; 8 * 1024];
            for _ in 0..16 {
                if s.write_all(&junk).await.is_err() {
                    break;
                }
            }
            s
        });

        let (mut stream, _) = listener.accept().await.unwrap();
        let err = read_request(&mut stream).await.unwrap_err();
        assert!(
            err.to_string().contains("headers exceeded"),
            "unterminated head must be capped: {err}"
        );
        drop(client.await.unwrap());
    }
}
