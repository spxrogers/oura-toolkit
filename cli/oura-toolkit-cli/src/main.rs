//! oura — the Oura Ring toolkit CLI: interactive auth flows, data commands, and `oura mcp`.
//!
//! Thin binary over the library target (which exists so integration tests exercise the
//! real modules). Exit codes, stream discipline, and error style are a documented
//! contract — see `docs/cli-contract.md` and the `contract` module (#21).

use clap::{Parser, Subcommand};
use oura_toolkit_cli::{api, auth, commands, contract, output};

/// Oura Ring toolkit — CLI + MCP server for the Oura API v2.
#[derive(Parser)]
#[command(name = "oura", version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Output JSON instead of the default table/plain rendering (data commands and `auth status`).
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
    /// Run as a STDIO MCP server (8 read-only Oura data tools).
    // A subcommand, not a `--mcp` flag: modes and modifiers don't mix, and clap makes the
    // nonsense states unrepresentable. Decided 2026-07-02 (#21).
    Mcp,
    /// Print a shell completion script to stdout (bash, zsh, fish, powershell, elvish).
    Completion {
        /// Shell to generate the completion script for.
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
    /// Print the `oura` man page (roff) to stdout.
    Man,
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
    /// Show stored auth state: client_id, scopes, token expiry.
    Status,
    /// Delete stored tokens (log out). Keeps the client credentials unless --all is given.
    Logout {
        /// Also remove the stored client credentials (client_id + client_secret).
        #[arg(long)]
        all: bool,
    },
    /// Force a token refresh now and persist the rotated refresh token.
    Refresh,
    /// Print a valid access token (refreshing if needed) to stdout — and nothing else.
    Token,
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
            AuthAction::Status => {
                let report = auth::status(&oura_toolkit_auth::TokenStore::new()?, render)?;
                // The report always reaches stdout (it IS the result — including the
                // partial state that tells the user what to fix); the typed failure then
                // rides the classifier to the documented exit 4 + hint on stderr.
                contract::emit(&report.rendered)?;
                match report.failure {
                    Some(err) => Err(err.into()),
                    None => Ok(()),
                }
            }
            AuthAction::Logout { all } => {
                // Mutations have no result: the confirmation is prose and goes to stderr
                // (contract → Streams); stdout stays empty like `gh auth logout`.
                contract::inform(&auth::logout(&oura_toolkit_auth::TokenStore::new()?, all)?);
                Ok(())
            }
            AuthAction::Refresh => {
                let store = oura_toolkit_auth::TokenStore::new()?;
                let manager = oura_toolkit_auth::TokenManager::load()?;
                contract::inform(&auth::refresh(&manager, &store).await?);
                Ok(())
            }
            AuthAction::Token => {
                let store = oura_toolkit_auth::TokenStore::new()?;
                let manager = oura_toolkit_auth::TokenManager::load()?;
                contract::emit(&auth::token(&manager, &store).await?)?;
                Ok(())
            }
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
        Some(Command::Mcp) => {
            // STDIO MCP server (#10): stdout is the JSON-RPC transport from here on.
            // Absent tokens are NOT an error at startup — initialize must succeed and
            // the first tool call reports the structured auth error (CLAUDE.md → MCP).
            oura_toolkit_cli::mcp::serve(oura_toolkit_auth::TokenManager::load()?).await
        }
        // Pure code generators: no auth, no network. The script/man page IS the result, so it
        // goes to stdout through the same broken-pipe-tolerant path as every other result
        // (contract → Streams). Generate into a buffer first so a closed stdout is a clean
        // exit, not a panic inside clap_complete.
        Some(Command::Completion { shell }) => {
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            let mut buf = Vec::new();
            clap_complete::generate(shell, &mut cmd, "oura", &mut buf);
            contract::emit(&String::from_utf8(buf).expect("completion scripts are valid UTF-8"))?;
            Ok(())
        }
        Some(Command::Man) => {
            use clap::CommandFactory;
            let mut buf = Vec::new();
            clap_mangen::Man::new(Cli::command())
                .render(&mut buf)
                .expect("rendering a man page to an in-memory buffer cannot fail");
            contract::emit(&String::from_utf8(buf).expect("man pages are valid UTF-8"))?;
            Ok(())
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
