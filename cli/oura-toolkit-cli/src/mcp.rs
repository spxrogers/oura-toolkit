//! `oura mcp` — the STDIO MCP server (#10): the CLI's 8 curated data surfaces exposed as
//! MCP tools over rmcp.
//!
//! Design (decided 2026-07-02, recorded in CLAUDE.md → MCP): tool DISPATCH is hand-wired
//! to the same `commands::fetch_*` data plane the CLI subcommands use — one auth layer
//! (silent 401 refresh with rotation persisted under the cross-process lock, retry once,
//! typed `NotAuthenticated`), one pagination loop, two presentations. Tool DESCRIPTIONS
//! are spec-derived at build time (`build.rs` → curated lead + the spec's own field
//! inventory), and tool RESULTS are the spec-generated models serialized to JSON — so
//! everything textual/structural that CAN flow from the spec, does.
//!
//! Contract (CLAUDE.md → MCP):
//! - stdout carries NOTHING but JSON-RPC — no prompts, no browser, no prose.
//! - `initialize` succeeds with or without stored tokens.
//! - A tool call without usable tokens returns a TOOL-LEVEL error (the model sees it)
//!   telling the user to run `oura auth login` — never a protocol error, never a prompt.
//! - Malformed arguments are protocol errors (`invalid_params`) per MCP convention.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{CallToolResult, ContentBlock, Implementation, ServerCapabilities, ServerInfo};
use rmcp::{tool, tool_handler, tool_router, ErrorData, ServerHandler, ServiceExt};

use oura_toolkit_auth::TokenManager;
use oura_toolkit_health::HealthStore;

use crate::api::{self, DateRange};
use crate::commands;
use crate::contract::{self, EXIT_AUTH};
use crate::health;

/// Build-time generated LLM-facing tool descriptions (curated lead + spec field
/// inventory). See `build.rs`.
mod descriptions {
    include!(concat!(env!("OUT_DIR"), "/mcp_tool_descriptions.rs"));
}

/// Tool name → build-time generated description. One row per curated tool; applied (and
/// drift-guarded) in [`OuraMcp::new`].
const DESCRIPTIONS: &[(&str, &str)] = &[
    ("get_daily_sleep", descriptions::DAILY_SLEEP),
    ("get_daily_readiness", descriptions::DAILY_READINESS),
    ("get_daily_activity", descriptions::DAILY_ACTIVITY),
    ("get_daily_stress", descriptions::DAILY_STRESS),
    ("get_heart_rate", descriptions::HEART_RATE),
    ("get_sessions", descriptions::SESSIONS),
    ("get_workouts", descriptions::WORKOUTS),
    ("get_personal_info", descriptions::PERSONAL_INFO),
];

/// LLM-facing descriptions for the LOCAL-STORE tools. Hand-curated (there is no spec to
/// derive them from — the local store is this repo's own surface); injected in
/// [`OuraMcp::new`] exactly like the spec-derived eight. Each must state where the data
/// comes from and what the model may/may not claim from it.
const LOCAL_DESCRIPTIONS: &[(&str, &str)] = &[
    (
        "get_capacity",
        "How much more the CURRENT week can take, computed locally from the user's own \
         history: a 0-100 capacity percentage with a band (comfortable ≥70, stretched \
         40-69, overloaded <40) and a full attribution — points deducted for recovery \
         debt (recent readiness vs personal baseline), this week's schedule load vs \
         baseline, and how similar past weeks turned out (analog risk). Also returns \
         the matched analog weeks. Data source: the local day-grain store fed by `oura \
         sync` (Oura dailies) and `oura import` (Apple Health, calendar .ics, Toggl). \
         This is n=1 observational data: present results as 'in your history, weeks \
         like this were followed by…', never as predictions. Errors when history is \
         too thin rather than extrapolating — relay the error's remediation verbatim.",
    ),
    (
        "find_analog_weeks",
        "The user's historical weeks most similar to a target week by schedule load \
         (meeting hours, event counts, evening events, tracked hours; z-score \
         nearest-neighbor), each with health outcomes DURING the week and over the two \
         weeks AFTER it (readiness/sleep-score means from Oura, sleep-minutes/HRV from \
         Apple - provider-tagged, never blended), plus the personal baseline for \
         comparison. Works for future weeks too when calendar imports carry upcoming \
         events. Args: week (today, yesterday, or YYYY-MM-DD - any day of the target \
         week; defaults to the current week). Data source: the local store (`oura \
         sync` / `oura import`). n=1 observational data - describe what followed \
         similar weeks, never predict. Errors when history is too thin.",
    ),
    (
        "get_upcoming_load",
        "Schedule load for the coming weeks (default 4, starting with the current \
         week), summed per week from imported calendar/Toggl context: meeting hours, \
         event count, evening events, tracked hours. Weeks with no imported context \
         are listed with empty features so coverage gaps are visible - say so rather \
         than treating them as free. Args: weeks (1-26). Data source: the local store; \
         future context exists only if the user imported a calendar (.ics) that \
         includes future events (`oura import calendar`).",
    ),
    (
        "get_day_context",
        "The merged day-grain records the capacity/analog engine sees: per local \
         calendar day, the Oura dailies (sleep/readiness/activity scores, stress), \
         Apple Health aggregates (sleep stages, resting HR, HRV SDNN, steps, workouts, \
         SpO2, wrist temperature), calendar schedule shape (meeting hours, event \
         counts - derived numbers only, never event titles), and Toggl tracked-time \
         shape. Args: start/end (today, yesterday, or YYYY-MM-DD; default last 7 \
         days). Data source: the local store. Note Oura HRV (rMSSD) and Apple HRV \
         (SDNN) are different statistics - never compare them to each other.",
    ),
];

/// The server's tool names, exposed for the docs tripwire (`tests/docs_tripwire.rs`,
/// #45): README claims about tool names must fail CI when a `#[tool(name = …)]` rename
/// orphans them (CLAUDE.md → DOCS STAY TRUE TO THE CODE).
pub fn tool_names() -> impl Iterator<Item = &'static str> {
    DESCRIPTIONS
        .iter()
        .chain(LOCAL_DESCRIPTIONS)
        .map(|(name, _)| *name)
}

/// The MCP server: same data plane as the CLI, `base_url` injected so tests point at
/// wiremock and `store` injected so tests use a tempdir.
pub struct OuraMcp {
    manager: TokenManager,
    base_url: String,
    store: HealthStore,
    tool_router: ToolRouter<Self>,
}

/// Shared date-window parameters for every windowed tool. Deliberately CURATED, not the
/// raw spec params: the cursor is hidden (tools auto-paginate to completion) and dates
/// use the CLI's local-timezone semantics (docs/cli-contract.md → Dates).
#[derive(serde::Deserialize, schemars::JsonSchema, Default)]
pub struct DateRangeParams {
    /// Start date: `today`, `yesterday`, or `YYYY-MM-DD` in the user's local timezone.
    /// Default: 6 days before `end` (a 7-day window).
    #[serde(default)]
    pub start: Option<String>,
    /// End date: `today`, `yesterday`, or `YYYY-MM-DD` in the user's local timezone.
    /// Default: today.
    #[serde(default)]
    pub end: Option<String>,
}

impl DateRangeParams {
    /// Malformed dates / inverted ranges are the caller's arguments being wrong →
    /// protocol `invalid_params`, mirroring the CLI's exit-2 classification.
    fn resolve(&self) -> Result<DateRange, ErrorData> {
        DateRange::resolve(
            self.start.as_deref(),
            self.end.as_deref(),
            api::local_today(),
        )
        // Sanitized: the message echoes the caller's argument, and MCP clients render
        // this text — control bytes must die here, not rely on Debug-escaping upstream
        // staying that way.
        .map_err(|err| {
            ErrorData::invalid_params(crate::output::sanitize(&format!("{err:#}")), None)
        })
    }
}

/// Map a fetch outcome to the MCP result shape: success → the generated models as JSON;
/// failure → a TOOL-LEVEL error (the model sees the message). Auth-shaped failures reuse
/// the CLI contract's classifier so the remediation text is identical everywhere.
fn tool_result<T: serde::Serialize>(
    fetched: anyhow::Result<T>,
) -> Result<CallToolResult, ErrorData> {
    match fetched {
        Ok(data) => {
            let json = serde_json::to_string_pretty(&data).map_err(|e| {
                ErrorData::internal_error(format!("serializing response data: {e}"), None)
            })?;
            Ok(CallToolResult::success(vec![ContentBlock::text(json)]))
        }
        Err(err) => {
            let failure = contract::classify(&err);
            let mut message = contract::render_error(&err);
            if failure.code == EXIT_AUTH {
                // The structured auth error CLAUDE.md mandates: no prompt, no browser —
                // tell the user (via the model) exactly what to run, out of band.
                message.push_str(
                    "\nNot authenticated with Oura. Ask the user to run `oura auth login` \
                     in a terminal (or `oura auth setup` first if they have never \
                     registered credentials), then retry this tool.",
                );
            } else if let Some(hint) = failure.hint {
                // Defensive: every hint today is auth-shaped (consumed above); this
                // keeps a future non-auth hint from being silently dropped.
                message.push_str(&format!("\nhint: {hint}"));
            }
            Ok(CallToolResult::error(vec![ContentBlock::text(message)]))
        }
    }
}

#[tool_router]
impl OuraMcp {
    pub fn new(manager: TokenManager, base_url: String, store: HealthStore) -> Self {
        let mut tool_router = Self::tool_router();
        // The `#[tool]` attribute only accepts literal descriptions, so the build-time
        // spec-derived ones (and the hand-curated local-store ones) are injected here.
        // `expect` = drift guard: a tool renamed in the macro without updating its table
        // fails every construction, loudly.
        for (name, description) in DESCRIPTIONS.iter().chain(LOCAL_DESCRIPTIONS) {
            tool_router
                .map
                .get_mut(*name)
                .unwrap_or_else(|| panic!("tool {name} not registered by the macro"))
                .attr
                .description = Some((*description).into());
        }
        Self {
            manager,
            base_url,
            store,
            tool_router,
        }
    }

    #[tool(name = "get_daily_sleep")]
    async fn get_daily_sleep(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(commands::fetch_sleep(&self.manager, &self.base_url, range).await)
    }

    #[tool(name = "get_daily_readiness")]
    async fn get_daily_readiness(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(commands::fetch_readiness(&self.manager, &self.base_url, range).await)
    }

    #[tool(name = "get_daily_activity")]
    async fn get_daily_activity(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(commands::fetch_activity(&self.manager, &self.base_url, range).await)
    }

    #[tool(name = "get_daily_stress")]
    async fn get_daily_stress(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(commands::fetch_stress(&self.manager, &self.base_url, range).await)
    }

    #[tool(name = "get_heart_rate")]
    async fn get_heart_rate(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(commands::fetch_heartrate(&self.manager, &self.base_url, range).await)
    }

    #[tool(name = "get_sessions")]
    async fn get_sessions(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(commands::fetch_sessions(&self.manager, &self.base_url, range).await)
    }

    #[tool(name = "get_workouts")]
    async fn get_workouts(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(commands::fetch_workouts(&self.manager, &self.base_url, range).await)
    }

    #[tool(name = "get_personal_info")]
    async fn get_personal_info(&self) -> Result<CallToolResult, ErrorData> {
        tool_result(commands::fetch_personal_info(&self.manager, &self.base_url).await)
    }

    // ---- Local-store tools ------------------------------------------------------------
    // No Oura auth involved: these read the day-grain store (`oura sync` / `oura
    // import`). Thin-history refusals surface as TOOL-LEVEL errors whose message carries
    // the remediation (HealthError::InsufficientHistory names the import commands).

    #[tool(name = "get_capacity")]
    async fn get_capacity(&self) -> Result<CallToolResult, ErrorData> {
        tool_result(health::fetch_capacity(&self.store, api::local_today()))
    }

    #[tool(name = "find_analog_weeks")]
    async fn find_analog_weeks(
        &self,
        Parameters(params): Parameters<WeekParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let week = params.resolve()?;
        tool_result(health::fetch_analogs(
            &self.store,
            week,
            oura_toolkit_health::engine::DEFAULT_ANALOG_COUNT,
        ))
    }

    #[tool(name = "get_upcoming_load")]
    async fn get_upcoming_load(
        &self,
        Parameters(params): Parameters<UpcomingParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let weeks = params.resolve()?;
        tool_result(health::fetch_upcoming(
            &self.store,
            api::local_today(),
            weeks,
        ))
    }

    #[tool(name = "get_day_context")]
    async fn get_day_context(
        &self,
        Parameters(params): Parameters<DateRangeParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let range = params.resolve()?;
        tool_result(health::fetch_day_context(&self.store, range))
    }
}

/// Target-week parameter for `find_analog_weeks`: any day of the week of interest.
#[derive(serde::Deserialize, schemars::JsonSchema, Default)]
pub struct WeekParams {
    /// Any day of the target week: `today`, `yesterday`, or `YYYY-MM-DD` in the user's
    /// local timezone. Default: today (the current week).
    #[serde(default)]
    pub week: Option<String>,
}

impl WeekParams {
    fn resolve(&self) -> Result<chrono::NaiveDate, ErrorData> {
        match &self.week {
            None => Ok(api::local_today()),
            Some(s) => api::parse_date(s, api::local_today()).map_err(|err| {
                ErrorData::invalid_params(crate::output::sanitize(&format!("{err:#}")), None)
            }),
        }
    }
}

/// Horizon parameter for `get_upcoming_load`.
#[derive(serde::Deserialize, schemars::JsonSchema, Default)]
pub struct UpcomingParams {
    /// How many weeks ahead to report, starting with the current week (1–26).
    /// Default: 4.
    #[serde(default)]
    pub weeks: Option<u32>,
}

impl UpcomingParams {
    fn resolve(&self) -> Result<u32, ErrorData> {
        let weeks = self.weeks.unwrap_or(4);
        if !(1..=26).contains(&weeks) {
            return Err(ErrorData::invalid_params(
                format!("weeks must be between 1 and 26, got {weeks}"),
                None,
            ));
        }
        Ok(weeks)
    }
}

// `router = self.tool_router` (the FIELD): the macro's default re-derives a fresh
// `Self::tool_router()` per request, which would silently drop the spec-derived
// descriptions injected in `new`.
#[tool_handler(router = self.tool_router)]
impl ServerHandler for OuraMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                "oura-toolkit",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "Read-only access to the user's Oura Ring data (sleep, readiness, \
                 activity, stress, heart rate, sessions, workouts, profile) plus their \
                 LOCAL day-grain health+schedule store (capacity, analog weeks, \
                 upcoming load, day context — fed by `oura sync` and `oura import`). \
                 Authentication is out of band: if a tool reports that the user is not \
                 authenticated, ask them to run `oura auth login` in a terminal, then \
                 retry. If a local-store tool reports not enough history, relay its \
                 import instructions. Local-store results are n=1 observational data — \
                 describe what followed similar weeks in the user's own history; never \
                 present them as predictions.",
            )
    }
}

/// Serve over stdio until the client disconnects. stdout is the JSON-RPC transport;
/// nothing else may write to it (enforced at the binary level in tests/mcp_stdio.rs).
pub async fn serve(manager: TokenManager) -> anyhow::Result<()> {
    // No sync-root warning here: the server reads the store, and stderr noise on every
    // assistant launch would be crying wolf — the write paths (`oura sync`/`import`) own
    // that warning.
    let store = HealthStore::open_default()
        .map_err(|e| anyhow::anyhow!("locating the health data store: {e}"))?;
    let server = OuraMcp::new(manager, api::API_BASE.to_string(), store);
    let running = match server.serve(rmcp::transport::stdio()).await {
        Ok(running) => running,
        // stdin closing before/during the handshake is "no client connected", not a
        // server failure: exit 0, silently (nothing may be written to stdout, and a
        // vanished client is not an error worth stderr noise either).
        Err(rmcp::service::ServerInitializeError::ConnectionClosed(_)) => return Ok(()),
        Err(e) => return Err(anyhow::anyhow!("starting the MCP server: {e}")),
    };
    running
        .waiting()
        .await
        .map_err(|e| anyhow::anyhow!("MCP server terminated abnormally: {e}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{DESCRIPTIONS, LOCAL_DESCRIPTIONS};

    /// The plugin skills instruct Claude to call tools BY NAME — that's a functional
    /// contract, not prose. A `#[tool(name = …)]` rename that orphans a skill must fail
    /// CI. Monorepo-only by nature (the plugin lives beside the crate).
    #[test]
    fn plugin_skills_reference_only_real_tool_names() {
        let skills_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../plugins/oura-toolkit/skills");
        assert!(
            skills_dir.is_dir(),
            "plugin skills dir missing at {skills_dir:?} — moved without updating this guard?"
        );
        let known: Vec<&str> = super::tool_names().collect();
        let mut checked = 0;
        for entry in std::fs::read_dir(&skills_dir).unwrap() {
            let skill = entry.unwrap().path().join("SKILL.md");
            let text = std::fs::read_to_string(&skill).unwrap();
            // Every `get_…`/`find_…` token in a skill must be one of the server's tool
            // names (the local tools include the non-`get_` `find_analog_weeks`).
            for token in text
                .split(|c: char| !(c.is_ascii_lowercase() || c == '_'))
                .filter(|t| t.starts_with("get_") || t.starts_with("find_"))
            {
                assert!(
                    known.contains(&token),
                    "{skill:?} references unknown tool {token:?} — rename orphaned the skill?"
                );
                checked += 1;
            }
        }
        assert!(
            checked >= 8,
            "suspiciously few tool references ({checked}) — tokenizer broken?"
        );
    }

    /// The tool descriptions are the LLM-facing selection surface: each must carry the
    /// curated lead AND both spec-derived sections, and stay within a budget — the spec
    /// has field descriptions (activity's class_5_min) that would otherwise bloat a
    /// description into kilobytes of noise on every tools/list.
    #[test]
    fn descriptions_have_the_curated_shape_and_stay_within_budget() {
        for (name, description) in DESCRIPTIONS {
            assert!(
                description.contains("Oura API operation:"),
                "{name}: spec summary section missing"
            );
            assert!(
                description.contains("Documents contain:"),
                "{name}: spec field inventory missing"
            );
            let len = description.chars().count();
            assert!(
                (150..=900).contains(&len),
                "{name}: description length {len} outside the 150..=900 budget"
            );
        }
    }

    /// The local-store tools' descriptions are hand-curated (no spec to derive from) but
    /// carry the same duties: name the data source (so the model knows this is the local
    /// store, not the Oura API) and the n=1 framing rule where outcomes are involved —
    /// and stay within the same anti-bloat budget.
    #[test]
    fn local_descriptions_name_their_source_and_stay_within_budget() {
        for (name, description) in LOCAL_DESCRIPTIONS {
            assert!(
                description.contains("local") && description.contains("store"),
                "{name}: must tell the model the data comes from the local store"
            );
            let len = description.chars().count();
            assert!(
                (150..=1000).contains(&len),
                "{name}: description length {len} outside the 150..=1000 budget"
            );
        }
        for (name, description) in [
            LOCAL_DESCRIPTIONS[0], // get_capacity
            LOCAL_DESCRIPTIONS[1], // find_analog_weeks
        ] {
            assert!(
                description.contains("never") && description.contains("predict"),
                "{name}: outcome-bearing tools must carry the n=1 no-predictions framing"
            );
        }
    }
}
