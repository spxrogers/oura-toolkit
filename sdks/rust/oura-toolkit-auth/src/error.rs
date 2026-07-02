//! Error type for the auth companion.

/// Errors from the token store, the token endpoint, and the auth middleware.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// No tokens available. The library deliberately does not embed remediation hints in
    /// `Display` — callers own the UX: the CLI maps this to "run `oura auth login`", the
    /// MCP server to a structured tool error saying the same.
    #[error("not authenticated (no tokens stored)")]
    NotAuthenticated,

    /// Could not resolve the config directory from the platform's environment.
    #[cfg(not(windows))]
    #[error("could not determine the config directory ($XDG_CONFIG_HOME / $HOME unset)")]
    NoConfigDir,

    /// Could not resolve the config directory from the platform's environment.
    #[cfg(windows)]
    #[error("could not determine the config directory (%LOCALAPPDATA% unset)")]
    NoConfigDir,

    /// The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh token).
    #[error("token endpoint returned HTTP {status}: {body}")]
    TokenEndpoint { status: u16, body: String },

    /// The authorization-code exchange succeeded but returned no `refresh_token` — persisting
    /// that state would break the next refresh, so it is rejected up front.
    #[error("token endpoint returned no refresh_token on the initial exchange")]
    MissingRefreshToken,

    /// Tokens exist but the client credentials record is missing, so a refresh is impossible
    /// (confidential client: the token endpoint requires `client_id` + `client_secret`).
    /// Callers own the remediation hint (the CLI maps this to "run `oura auth setup`").
    #[error("no client credentials stored")]
    MissingClientCredentials,

    /// Filesystem error reading/writing the token store.
    #[error("token store i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// (De)serialization error for the stored credentials.
    #[error("token store format error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Transport error talking to the token endpoint.
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
}
