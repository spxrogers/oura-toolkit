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

use oura_toolkit_auth::{AuthError, TokenManager};

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

/// The server's tool names, exposed for the docs tripwire (`tests/docs_tripwire.rs`,
/// #45): README claims about tool names must fail CI when a `#[tool(name = …)]` rename
/// orphans them (CLAUDE.md → DOCS STAY TRUE TO THE CODE).
pub fn tool_names() -> impl Iterator<Item = &'static str> {
    DESCRIPTIONS.iter().map(|(name, _)| *name)
}

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
    /// A single day (`today`, `yesterday`, or `YYYY-MM-DD`, local timezone) — the shorthand
    /// for the common single-day question. Sets start = end = this day. Mutually exclusive
    /// with `start`/`end`.
    #[serde(default)]
    pub date: Option<String>,
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
    /// Malformed dates / inverted ranges / combining `date` with a range are the caller's
    /// arguments being wrong → protocol `invalid_params`, mirroring the CLI's exit-2
    /// classification.
    fn resolve(&self) -> Result<DateRange, ErrorData> {
        DateRange::resolve_with_date(
            self.date.as_deref(),
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
///
/// Success carries BOTH a text block (the pretty JSON — kept for clients that don't yet read
/// structured results) AND `structured_content` (#40), the same data as a typed value clients
/// can consume without re-parsing. MCP types `structuredContent` as a JSON **object**, so a
/// collection tool's array is enveloped as `{"data":[…]}`; an object result (personal info)
/// passes through unchanged. Output schemas are deliberately NOT advertised yet — the
/// progenitor models don't derive `JsonSchema`; untyped structured content is the clean first
/// step (build-time output-schema generation can follow).
fn tool_result<T: serde::Serialize>(
    fetched: anyhow::Result<T>,
) -> Result<CallToolResult, ErrorData> {
    match fetched {
        Ok(data) => {
            let value = serde_json::to_value(&data).map_err(|e| {
                ErrorData::internal_error(format!("serializing response data: {e}"), None)
            })?;
            let json = serde_json::to_string_pretty(&value).map_err(|e| {
                ErrorData::internal_error(format!("serializing response data: {e}"), None)
            })?;
            // MCP's structuredContent MUST be an object; a collection serializes to an array,
            // so envelope it under `data` (matching Oura's own response shape). Objects pass
            // through as-is.
            let structured = match value {
                serde_json::Value::Object(_) => value,
                other => serde_json::json!({ "data": other }),
            };
            let mut result = CallToolResult::success(vec![ContentBlock::text(json)]);
            result.structured_content = Some(structured);
            Ok(result)
        }
        Err(err) => {
            let failure = contract::classify(&err);
            let mut message = contract::render_error(&err);
            let static_token_rejected = err.chain().any(|c| {
                matches!(
                    c.downcast_ref::<AuthError>(),
                    Some(AuthError::StaticTokenRejected)
                )
            });
            if static_token_rejected {
                // A server started with OURA_ACCESS_TOKEN (#20): no interactive login exists
                // in that deployment (a container), so pointing at `oura auth login` would
                // misdirect — the fix is a fresh token + restart.
                message.push_str(
                    "\nThe OURA_ACCESS_TOKEN this server was started with was rejected \
                     (expired or invalid). Restart `oura mcp` with a fresh token.",
                );
            } else if failure.code == EXIT_AUTH {
                // The structured auth error CLAUDE.md mandates: no prompt, no browser —
                // tell the user (via the model) exactly what to run, out of band.
                message.push_str(
                    "\nNot authenticated with Oura. Ask the user to run `oura auth login` \
                     in a terminal (or `oura auth setup` first if they have never \
                     registered credentials), then retry this tool.",
                );
            } else if let Some(hint) = failure.hint {
                // Non-auth hints ride along verbatim — today that is the rate-limit
                // hint (#28): the error line names the reset time, the hint tells the
                // model/user to wait for it and retry the tool.
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
///
/// `base_url` is the resolved data-plane host (default [`api::API_BASE`], or `OURA_API_BASE_URL`
/// — #20), so a containerized server can point at a proxy/mock just like the CLI.
pub async fn serve(manager: TokenManager, base_url: String) -> anyhow::Result<()> {
    let server = OuraMcp::new(manager, base_url);
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
        let known: Vec<&str> = DESCRIPTIONS.iter().map(|(name, _)| *name).collect();
        let mut checked = 0;
        for entry in std::fs::read_dir(&skills_dir).unwrap() {
            let skill = entry.unwrap().path().join("SKILL.md");
            let text = std::fs::read_to_string(&skill).unwrap();
            // Every `get_…` token in a skill must be one of the server's tool names.
            for token in text
                .split(|c: char| !(c.is_ascii_lowercase() || c == '_'))
                .filter(|t| t.starts_with("get_"))
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

    /// The single-day `date` convenience param (#39): it collapses to a one-day window, and
    /// combining it with `start`/`end` is `invalid_params` (the MCP mirror of the CLI's
    /// exit-2 usage error).
    #[test]
    fn date_param_makes_a_single_day_window_and_conflicts_with_a_range() {
        use super::{DateRangeParams, ErrorData};
        let single = DateRangeParams {
            date: Some("yesterday".into()),
            start: None,
            end: None,
        }
        .resolve()
        .expect("a lone `date` resolves");
        assert_eq!(single.start, single.end, "`date` sets start == end");

        let err = DateRangeParams {
            date: Some("today".into()),
            start: Some("yesterday".into()),
            end: None,
        }
        .resolve()
        .expect_err("date + start must be rejected");
        assert!(
            err.message.contains("cannot be combined"),
            "invalid_params must name the conflict: {}",
            err.message
        );
        // The CODE, not just the message: a wrong impl returning internal_error with the same
        // text must not pass — the conflict is the caller's fault (invalid_params).
        assert_eq!(
            err.code,
            ErrorData::invalid_params("x", None).code,
            "a date/range conflict must classify as invalid_params"
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
}
