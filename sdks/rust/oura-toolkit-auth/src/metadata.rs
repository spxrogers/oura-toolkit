//! OAuth2 metadata, read from the vendored spec at build time (see `build.rs`). Nothing here
//! is hardcoded — `AUTHORIZE_URL`, `TOKEN_URL`, and `ALL_SCOPES` come straight from the spec.

use url::Url;

// Brings in `AUTHORIZE_URL`, `TOKEN_URL`, `ALL_SCOPES` generated from the spec.
include!(concat!(env!("OUT_DIR"), "/oauth_metadata.rs"));

/// Scopes the toolkit requests by default: everything except `email`.
///
/// This is the toolkit's *policy*, not spec metadata — the set of scopes advertised by the
/// spec lives in [`ALL_SCOPES`]. Each entry is validated against `ALL_SCOPES` by
/// [`default_scopes`] so a spec change can't silently drift this list.
const DEFAULT_SCOPE_NAMES: &[&str] = &[
    "personal",
    "daily",
    "heartrate",
    "workout",
    "tag",
    "session",
    "spo2Daily",
];

/// The default scopes, verified to all exist in the spec-advertised [`ALL_SCOPES`].
pub fn default_scopes() -> Vec<&'static str> {
    DEFAULT_SCOPE_NAMES
        .iter()
        .copied()
        .filter(|s| ALL_SCOPES.contains(s))
        .collect()
}

/// Build the authorization-code consent URL from spec metadata.
///
/// `scopes` are space-joined per OAuth2; `state` is an opaque CSRF token the caller generates
/// and later verifies on the callback.
pub fn authorize_url(client_id: &str, redirect_uri: &str, scopes: &[&str], state: &str) -> String {
    let mut url = Url::parse(AUTHORIZE_URL).expect("spec authorizationUrl is a valid URL");
    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", &scopes.join(" "))
        .append_pair("state", state);
    url.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_comes_from_spec() {
        assert_eq!(AUTHORIZE_URL, "https://cloud.ouraring.com/oauth/authorize");
        assert_eq!(TOKEN_URL, "https://api.ouraring.com/oauth/token");
        assert!(ALL_SCOPES.contains(&"personal"));
        assert_eq!(ALL_SCOPES.len(), 8);
    }

    #[test]
    fn default_scopes_are_all_valid_and_exclude_email() {
        let scopes = default_scopes();
        assert_eq!(scopes.len(), DEFAULT_SCOPE_NAMES.len());
        assert!(!scopes.contains(&"email"));
        assert!(scopes.iter().all(|s| ALL_SCOPES.contains(s)));
    }

    #[test]
    fn authorize_url_encodes_params() {
        let url = authorize_url(
            "cid",
            "http://localhost:8788/callback",
            &["daily", "tag"],
            "xyz",
        );
        assert!(url.starts_with("https://cloud.ouraring.com/oauth/authorize?"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("client_id=cid"));
        assert!(url.contains("redirect_uri=http%3A%2F%2Flocalhost%3A8788%2Fcallback"));
        assert!(url.contains("scope=daily+tag"));
        assert!(url.contains("state=xyz"));
    }
}
