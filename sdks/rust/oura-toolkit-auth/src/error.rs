//! Error type for the auth companion.

/// Errors from the token store, the token endpoint, and the auth middleware.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    /// No credentials on disk. Callers (CLI/MCP) turn this into a "run `oura auth login`"
    /// message; the MCP server surfaces it as a structured tool error.
    #[error("not authenticated — run `oura auth login`")]
    NotAuthenticated,

    /// Could not resolve the config directory ($XDG_CONFIG_HOME or $HOME).
    #[error("could not determine the config directory ($XDG_CONFIG_HOME / $HOME unset)")]
    NoConfigDir,

    /// The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh token).
    #[error("token endpoint returned HTTP {status}: {body}")]
    TokenEndpoint { status: u16, body: String },

    /// The authorization-code exchange succeeded but returned no `refresh_token` — persisting
    /// that state would break the next refresh, so it is rejected up front.
    #[error("token endpoint returned no refresh_token on the initial exchange")]
    MissingRefreshToken,

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
