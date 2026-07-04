//! The crate's error type. Import/parse failures carry what was being parsed; the
//! engine's refusal to extrapolate from thin history is a first-class, typed outcome
//! (surfaced verbatim by both the CLI and the MCP tools), not a generic error string.

/// Everything that can go wrong in the health store, the importers, or the engine.
#[derive(Debug, thiserror::Error)]
pub enum HealthError {
    #[error(
        "could not determine the data directory ($XDG_DATA_HOME / $HOME unset or not an absolute path)"
    )]
    NoDataDir,

    #[error("health store I/O: {0}")]
    Io(#[from] std::io::Error),

    #[error("health store format: {0}")]
    Format(String),

    /// A malformed or unrecognizable input file. `what` names the importer's input kind
    /// ("Apple Health export", "calendar (.ics)", "Toggl CSV") so the CLI's one-line
    /// error is self-explanatory.
    #[error("parsing {what}: {detail}")]
    Parse { what: &'static str, detail: String },

    /// The engine refuses to report analogs/capacity below a minimum history: an answer
    /// synthesized from two data points would be false confidence, which is worse than
    /// no answer (same doctrine as the coverage-floor rule).
    #[error(
        "not enough history: the engine needs at least {needed} past weeks with both \
         schedule context and health data, found {have} — import more history \
         (`oura import calendar`, `oura sync`, `oura import apple-health`)"
    )]
    InsufficientHistory { needed: u32, have: u32 },

    /// A habit name that normalization cannot make safe/canonical (empty after
    /// sanitization, or too long). The CLI classifies this as a usage error — the
    /// invocation, not the store, is what's wrong.
    #[error("invalid habit name: {reason} (use letters/digits, e.g. `strength-training`)")]
    InvalidHabitName { reason: String },
}

impl From<serde_json::Error> for HealthError {
    fn from(e: serde_json::Error) -> Self {
        HealthError::Format(e.to_string())
    }
}
