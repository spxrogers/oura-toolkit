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

use crate::api::{self, DateRange};
use crate::commands;
use crate::contract::{self, EXIT_AUTH};

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

/// The MCP server: same data plane as the CLI, `base_url` injected so tests point at
/// wiremock.
pub struct OuraMcp {
    manager: TokenManager,
    base_url: String,
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
    pub fn new(manager: TokenManager, base_url: String) -> Self {
        let mut tool_router = Self::tool_router();
        // The `#[tool]` attribute only accepts literal descriptions, so the build-time
        // spec-derived ones are injected here. `expect` = drift guard: a tool renamed in
        // the macro without updating this table fails every construction, loudly.
        for (name, description) in DESCRIPTIONS {
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
                 activity, stress, heart rate, sessions, workouts, profile). \
                 Authentication is out of band: if a tool reports that the user is not \
                 authenticated, ask them to run `oura auth login` in a terminal, then \
                 retry.",
            )
    }
}

/// Serve over stdio until the client disconnects. stdout is the JSON-RPC transport;
/// nothing else may write to it (enforced at the binary level in tests/mcp_stdio.rs).
pub async fn serve(manager: TokenManager) -> anyhow::Result<()> {
    let server = OuraMcp::new(manager, api::API_BASE.to_string());
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
    use super::DESCRIPTIONS;

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
}
