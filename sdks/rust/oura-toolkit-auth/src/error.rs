//! Error type for the auth companion.

/// Errors from the token store, the token endpoint, and the auth middleware.
///
/// `#[non_exhaustive]`: new failure modes can be added without a breaking change, so match
/// with a wildcard arm.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum AuthError {
    /// No tokens available. The library deliberately does not embed remediation hints in
    /// `Display` — callers own the UX: the CLI maps this to "run `oura auth login`", the
    /// MCP server to a structured tool error saying the same.
    #[error("not authenticated (no tokens stored)")]
    NotAuthenticated,

    /// Could not resolve the config directory from the platform's environment.
    #[cfg(not(windows))]
    #[error(
        "could not determine the config directory ($XDG_CONFIG_HOME / $HOME unset or not an absolute path)"
    )]
    NoConfigDir,

    /// Could not resolve the config directory from the platform's environment.
    #[cfg(windows)]
    #[error(
        "could not determine the config directory (%LOCALAPPDATA% unset or not an absolute path)"
    )]
    NoConfigDir,

    /// The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh token).
    /// `body` is the response body kept for diagnosis (`invalid_grant` means "log in again"),
    /// with every secret the request submitted (refresh token, client secret, authorization
    /// code) replaced by `[REDACTED]` and then capped at 1024 characters (`…` marks a cut) —
    /// conformance `rejected_token_responses`.
    #[error("token endpoint returned HTTP {status}: {body}")]
    TokenEndpoint { status: u16, body: String },

    /// The authorization-code exchange succeeded but returned no (or an empty)
    /// `refresh_token` — persisting that state would break the next refresh, so it is
    /// rejected up front.
    #[error("token endpoint returned no refresh_token on the initial exchange")]
    MissingRefreshToken,

    /// The token endpoint answered 2xx but the body is undecodable or unusable — the
    /// hostile-but-2xx family (#58): not valid UTF-8, not a single well-formed JSON token
    /// response (incl. a missing or wrong-typed required field, or trailing data), an empty
    /// `access_token`, or an `expires_in` outside `1..=2147483647`. Persisting it would
    /// install a blank/expired Bearer AND burn the still-valid rotated refresh token, so it
    /// is rejected up front and the store stays untouched. The message is static: it never
    /// quotes the server's body.
    #[error("token endpoint returned an unusable success response: {0}")]
    InvalidTokenResponse(&'static str),

    /// Tokens exist but the client credentials record is missing, so a refresh is impossible
    /// (confidential client: the token endpoint requires `client_id` + `client_secret`).
    /// Callers own the remediation hint (the CLI maps this to "run `oura auth setup`").
    #[error("no client credentials stored")]
    MissingClientCredentials,

    /// A caller-supplied access token (built via [`crate::TokenManager::from_access_token`],
    /// e.g. the CLI's `OURA_ACCESS_TOKEN` override) was rejected by the API and cannot be
    /// refreshed — there is no refresh token or credentials behind it. The caller owns the
    /// remediation hint (the CLI: "export a fresh token").
    #[error("the supplied access token was rejected and cannot be refreshed")]
    StaticTokenRejected,

    /// Filesystem error reading/writing the token store.
    #[error("token store i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// A store record (`file`) that can't be loaded: not valid UTF-8, malformed JSON, or a
    /// missing or wrong-typed field. `detail` says what and where (line/column, and a missing
    /// field's name) but NEVER quotes the file's values — the store holds secrets
    /// (conformance: hostile_store_files `must_not_echo`).
    #[error("token store format error in {file}: {detail}")]
    StoreFormat { file: &'static str, detail: String },

    /// Serializing a record to save it failed (never expected for these plain structs).
    #[error("token store serialization error: {0}")]
    Serde(serde_json::Error),

    /// Transport error talking to the token endpoint (connection, TLS, timeout, reading the
    /// body). A 2xx body that arrives intact but can't be used is
    /// [`AuthError::InvalidTokenResponse`] instead.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}
