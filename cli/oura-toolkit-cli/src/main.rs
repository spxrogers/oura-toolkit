//! oura — the Oura Ring toolkit CLI: interactive auth flows, data commands, and `oura mcp`.
//!
//! Thin binary over the library target (which exists so integration tests exercise the
//! real modules). Exit codes, stream discipline, and error style are a documented
//! contract — see `docs/cli-contract.md` and the `contract` module (#21).

use clap::{Parser, Subcommand};
use oura_toolkit_cli::{api, auth, commands, contract, health, output};

/// Oura Ring toolkit — CLI + MCP server for the Oura API v2.
#[derive(Parser)]
#[command(name = "oura", version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Output JSON instead of the default table/plain rendering (data commands).
    #[arg(long, global = true)]
    json: bool,

    /// Disable colored output (also honored: the NO_COLOR env var).
    #[arg(long, global = true)]
    no_color: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

/// Shared date-window flags for the data commands. Dates are interpreted in the user's
/// LOCAL timezone (Oura data is user-local — see docs/cli-contract.md → Dates).
#[derive(clap::Args)]
struct RangeArgs {
    /// Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end).
    #[arg(long)]
    start: Option<String>,
    /// End date: today, yesterday, or YYYY-MM-DD (default: today).
    #[arg(long)]
    end: Option<String>,
}

impl RangeArgs {
    fn resolve(&self) -> anyhow::Result<api::DateRange> {
        api::DateRange::resolve(
            self.start.as_deref(),
            self.end.as_deref(),
            api::local_today(),
        )
    }
}

#[derive(Subcommand)]
enum Command {
    /// Authentication (OAuth) flows.
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// Daily sleep summaries (score + contributors).
    Sleep(RangeArgs),
    /// Daily readiness summaries.
    Readiness(RangeArgs),
    /// Daily activity summaries (score, steps, calories).
    Activity(RangeArgs),
    /// Daily stress summaries.
    Stress(RangeArgs),
    /// Heart-rate time series (frequent bpm samples).
    Heartrate(RangeArgs),
    /// Moment/session records (meditation, naps, …).
    Sessions(RangeArgs),
    /// Workout records.
    Workouts(RangeArgs),
    /// Your Oura profile (age, height, weight, …).
    PersonalInfo,
    /// Pull Oura daily summaries into the local health store (default: last 90 days).
    Sync(RangeArgs),
    /// Import a local data export (Apple Health, calendar, Toggl) into the health store.
    Import {
        #[command(subcommand)]
        source: ImportSource,
    },
    /// How much more this week can take, from your own history (0–100%).
    Capacity,
    /// The merged day-grain records (health + schedule context) the engine sees.
    Context(RangeArgs),
    /// Track boolean habits, read as moving-average rates (days/week), not streaks.
    Habit {
        #[command(subcommand)]
        action: HabitAction,
    },
    /// Render the local dashboard (a self-contained HTML file) and open it.
    Dashboard {
        /// Write the HTML here instead of the store directory.
        #[arg(long)]
        out: Option<std::path::PathBuf>,
        /// Only write the file; don't open a browser.
        #[arg(long)]
        no_open: bool,
    },
    /// Run as a STDIO MCP server (14 tools: Oura data + the local store + habits).
    // A subcommand, not a `--mcp` flag: modes and modifiers don't mix, and clap makes the
    // nonsense states unrepresentable. Decided 2026-07-02 (#21).
    Mcp,
}

#[derive(Subcommand)]
enum HabitAction {
    /// Mark a habit done for a day (names canonicalize: "Strength Training" ==
    /// strength-training).
    Log {
        /// The habit, e.g. exercise, meditate, strength-training.
        name: String,
        /// Day to log: today, yesterday, or YYYY-MM-DD.
        #[arg(long, default_value = "today")]
        date: String,
    },
    /// Remove a habit log (the undo of `log`).
    Undo {
        /// The habit to un-log.
        name: String,
        /// Day to undo: today, yesterday, or YYYY-MM-DD.
        #[arg(long, default_value = "today")]
        date: String,
    },
    /// Every habit's moving-average rates: days/week over 7/28/91-day windows.
    Stats,
}

#[derive(Subcommand)]
enum ImportSource {
    /// An Apple Health export: export.zip (or the extracted export.xml).
    AppleHealth {
        /// Path to export.zip / export.xml.
        file: std::path::PathBuf,
        /// Delete the export file after a successful import (it holds your full
        /// plaintext health history — see the README's data-safety section).
        #[arg(long)]
        remove_source: bool,
    },
    /// A calendar export (.ics): history AND future events (future weeks feed capacity).
    Calendar {
        /// Path to the .ics file.
        file: std::path::PathBuf,
    },
    /// A Toggl Track detailed-report CSV export.
    Toggl {
        /// Path to the .csv file.
        file: std::path::PathBuf,
    },
}

#[derive(Subcommand)]
enum AuthAction {
    /// Guided Oura OAuth app registration (terminal prompts), then login.
    Setup {
        /// Loopback port for the redirect URI (must match your registered app).
        #[arg(long, default_value_t = 8788)]
        port: u16,
    },
    /// Authorization Code login using stored client credentials.
    Login {
        /// Loopback port for the redirect URI (must match your registered app).
        #[arg(long, default_value_t = 8788)]
        port: u16,
    },
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    // Usage errors (unknown flags, bare `oura`) are handled by clap and exit 2 before we
    // get here; everything else routes through the contract's classifier (exit 1/4).
    match run().await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => contract::report(err),
    }
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Resolved once so every data command inherits the same decision; the auth flows are
    // interactive prose and don't render tables.
    let render = output::RenderOptions::from_flags(cli.json, cli.no_color);
    let data_ctx = || -> anyhow::Result<commands::Ctx> {
        Ok(commands::Ctx {
            manager: oura_toolkit_auth::TokenManager::load()?,
            base_url: api::API_BASE.to_string(),
            render,
        })
    };

    match cli.command {
        Some(Command::Auth { action }) => match action {
            AuthAction::Setup { port } => auth::setup(port).await,
            AuthAction::Login { port } => auth::login(port).await,
        },
        Some(Command::Sleep(range)) => {
            contract::emit(&commands::sleep(&data_ctx()?, range.resolve()?).await?)?;
            Ok(())
        }
        Some(Command::Readiness(range)) => {
            contract::emit(&commands::readiness(&data_ctx()?, range.resolve()?).await?)?;
            Ok(())
        }
        Some(Command::Activity(range)) => {
            contract::emit(&commands::activity(&data_ctx()?, range.resolve()?).await?)?;
            Ok(())
        }
        Some(Command::Stress(range)) => {
            contract::emit(&commands::stress(&data_ctx()?, range.resolve()?).await?)?;
            Ok(())
        }
        Some(Command::Heartrate(range)) => {
            contract::emit(&commands::heartrate(&data_ctx()?, range.resolve()?).await?)?;
            Ok(())
        }
        Some(Command::Sessions(range)) => {
            contract::emit(&commands::sessions(&data_ctx()?, range.resolve()?).await?)?;
            Ok(())
        }
        Some(Command::Workouts(range)) => {
            contract::emit(&commands::workouts(&data_ctx()?, range.resolve()?).await?)?;
            Ok(())
        }
        Some(Command::PersonalInfo) => {
            contract::emit(&commands::personal_info(&data_ctx()?).await?)?;
            Ok(())
        }
        Some(Command::Sync(range)) => {
            let ctx = data_ctx()?;
            let range = health::resolve_range_with_default(
                range.start.as_deref(),
                range.end.as_deref(),
                api::local_today(),
                health::SYNC_DEFAULT_DAYS,
            )?;
            let store = health::open_store(true)?;
            contract::emit(
                &health::sync(&ctx.manager, &ctx.base_url, &store, range, render).await?,
            )?;
            Ok(())
        }
        Some(Command::Import { source }) => {
            let store = health::open_store(true)?;
            let rendered = match source {
                ImportSource::AppleHealth {
                    file,
                    remove_source,
                } => health::import_apple(&store, &file, remove_source, render)?,
                ImportSource::Calendar { file } => {
                    health::import_calendar(&store, &file, api::local_today(), render)?
                }
                ImportSource::Toggl { file } => health::import_toggl(&store, &file, render)?,
            };
            contract::emit(&rendered)?;
            Ok(())
        }
        Some(Command::Capacity) => {
            let store = health::open_store(false)?;
            contract::emit(&health::capacity(&store, api::local_today(), render)?)?;
            Ok(())
        }
        Some(Command::Context(range)) => {
            let store = health::open_store(false)?;
            contract::emit(&health::context(&store, range.resolve()?, render)?)?;
            Ok(())
        }
        Some(Command::Habit { action }) => {
            let rendered = match action {
                HabitAction::Log { name, date } => {
                    let store = health::open_store(true)?;
                    let date = api::parse_date(&date, api::local_today())?;
                    health::habit_write_cmd(&store, &name, date, false, render)?
                }
                HabitAction::Undo { name, date } => {
                    let store = health::open_store(true)?;
                    let date = api::parse_date(&date, api::local_today())?;
                    health::habit_write_cmd(&store, &name, date, true, render)?
                }
                HabitAction::Stats => {
                    let store = health::open_store(false)?;
                    health::habit_stats_cmd(&store, api::local_today(), render)?
                }
            };
            contract::emit(&rendered)?;
            Ok(())
        }
        Some(Command::Dashboard { out, no_open }) => {
            let store = health::open_store(false)?;
            contract::emit(&health::dashboard(
                &store,
                api::local_today(),
                out.as_deref(),
                !no_open,
            )?)?;
            Ok(())
        }
        Some(Command::Mcp) => {
            // STDIO MCP server (#10): stdout is the JSON-RPC transport from here on.
            // Absent tokens are NOT an error at startup — initialize must succeed and
            // the first tool call reports the structured auth error (CLAUDE.md → MCP).
            oura_toolkit_cli::mcp::serve(oura_toolkit_auth::TokenManager::load()?).await
        }
        None => {
            // Reachable: `arg_required_else_help` only fires with ZERO args, so a lone
            // global flag (`oura --json`) parses to no command and lands here. It's a
            // usage error, so help goes to STDERR (stdout stays results-only) and we
            // exit 2 like clap's own usage errors. Best-effort write, like
            // `contract::report`: a closed stderr must not turn a usage error into a
            // panic — the exit code is the machine-readable part.
            use clap::CommandFactory;
            use std::io::Write as _;
            let _ = writeln!(std::io::stderr(), "{}", Cli::command().render_help());
            std::process::exit(2);
        }
    }
}
