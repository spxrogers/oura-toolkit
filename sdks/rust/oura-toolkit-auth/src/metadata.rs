//! OAuth2 metadata, read from the vendored spec at build time (see `build.rs`). Nothing here
//! is hardcoded — `AUTHORIZE_URL`, `TOKEN_URL`, and `ALL_SCOPES` come straight from the spec.

use url::Url;

// Brings in `AUTHORIZE_URL`, `TOKEN_URL`, `ALL_SCOPES` generated from the spec.
include!(concat!(env!("OUT_DIR"), "/oauth_metadata.rs"));

/// Scopes the toolkit requests by default: everything except `email`.
///
/// This is the toolkit's *policy*, not spec metadata — the set of scopes advertised by the
/// spec lives in [`ALL_SCOPES`]. Each entry is validated against `ALL_SCOPES` by
/// [`default_scopes`], which FAILS LOUD (panics, naming the missing scope) rather than
/// silently narrowing the request if a spec refresh ever renames one.
const DEFAULT_SCOPE_NAMES: &[&str] = &[
    "personal",
    "daily",
    "heartrate",
    "workout",
    "tag",
    "session",
    "spo2",
    "heart_health",
];

/// The default scopes, verified to all exist in the spec-advertised [`ALL_SCOPES`].
///
/// # Panics
///
/// Panics if a default scope is no longer advertised by the vendored spec — a silent
/// filter here would quietly shrink the consent we request. (The unit tests catch this in
/// CI on any spec refresh before it can panic at runtime.)
pub fn default_scopes() -> Vec<&'static str> {
    DEFAULT_SCOPE_NAMES
        .iter()
        .copied()
        .inspect(|s| {
            assert!(
                ALL_SCOPES.contains(s),
                "default scope {s:?} is not advertised by the vendored spec — \
                 update DEFAULT_SCOPE_NAMES to match the spec's OAuth2 scopes"
            );
        })
        .collect()
}

/// The default scopes a saved grant does NOT cover — what a fresh consent would add.
///
/// `granted` is the `scope` recorded with the tokens (the token endpoint's space-delimited
/// grant; commas are tolerated). Because [`default_scopes`] is spec-derived, this catches
/// every upstream scope rename/addition (1.41: `spo2Daily` → `spo2`, plus `heart_health`)
/// with no per-version code. A token refresh can't close the gap; only a new consent can.
///
/// An unrecorded grant (`None`) is treated as missing EVERY default: a login from before
/// the toolkit recorded scopes can't be shown to cover the current set.
pub fn missing_default_scopes(granted: Option<&str>) -> Vec<&'static str> {
    let granted: Vec<&str> = granted
        .map(|g| {
            g.split(|c: char| c.is_whitespace() || c == ',')
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();
    default_scopes()
        .into_iter()
        .filter(|s| !granted.contains(s))
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
        assert_eq!(ALL_SCOPES.len(), 9);
    }

    #[test]
    fn default_scopes_are_all_valid_and_exclude_email() {
        let scopes = default_scopes();
        assert_eq!(scopes.len(), DEFAULT_SCOPE_NAMES.len());
        assert!(!scopes.contains(&"email"));
        assert!(scopes.iter().all(|s| ALL_SCOPES.contains(s)));
    }

    #[test]
    fn missing_default_scopes_names_exactly_what_a_pre_1_41_grant_lacks() {
        // The real transition this guards: a grant from the spo2Daily era.
        let old = "personal daily heartrate workout tag session spo2Daily";
        assert_eq!(missing_default_scopes(Some(old)), ["spo2", "heart_health"]);
    }

    #[test]
    fn missing_default_scopes_is_empty_for_a_current_grant_in_any_order_or_delimiter() {
        let current = default_scopes().join(" ");
        assert_eq!(missing_default_scopes(Some(&current)), Vec::<&str>::new());
        // Order, commas, extra whitespace, and extra (e.g. `email`) scopes don't matter.
        let mut reversed = default_scopes();
        reversed.reverse();
        let messy = format!("email,  {}", reversed.join(","));
        assert_eq!(missing_default_scopes(Some(&messy)), Vec::<&str>::new());
    }

    #[test]
    fn missing_default_scopes_treats_an_unrecorded_grant_as_missing_everything() {
        assert_eq!(missing_default_scopes(None), default_scopes());
        assert_eq!(missing_default_scopes(Some("")), default_scopes());
    }

    #[test]
    fn missing_default_scopes_matches_whole_scopes_not_substrings() {
        // `spo2Daily` must not satisfy `spo2` (a substring match would hide the rename).
        let missing = missing_default_scopes(Some("spo2Daily heart_healthy"));
        assert!(missing.contains(&"spo2") && missing.contains(&"heart_health"));
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
