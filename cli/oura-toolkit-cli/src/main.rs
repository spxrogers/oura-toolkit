//! oura-cli — the Oura Ring toolkit CLI: interactive auth flows, data commands, and `oura mcp`.
//!
//! Interactive OAuth (browser + loopback) lives here, never in the SDKs. Data commands (#9) and
//! the STDIO MCP server (#10) are wired next; both reuse the `oura-toolkit-auth` companion.
//!
//! Exit codes, stream discipline, and error style are a documented contract — see
//! `docs/cli-contract.md` and the `contract` module (#21).

mod auth;
mod contract;
mod loopback;
mod output;

use clap::{Parser, Subcommand};

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

#[derive(Subcommand)]
enum Command {
    /// Authentication (OAuth) flows.
    Auth {
        #[command(subcommand)]
        action: AuthAction,
    },
    /// Run as a STDIO MCP server (see issue #10).
    // A subcommand, not a `--mcp` flag: modes and modifiers don't mix, and clap makes the
    // nonsense states unrepresentable. Decided 2026-07-02 (#21).
    Mcp,
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

    // Resolved once here so every data command (#9) inherits the same decision; the auth
    // flows are interactive prose and don't render tables.
    let _render = output::RenderOptions::from_flags(cli.json, cli.no_color);

    match cli.command {
        Some(Command::Auth { action }) => match action {
            AuthAction::Setup { port } => auth::setup(port).await,
            AuthAction::Login { port } => auth::login(port).await,
        },
        Some(Command::Mcp) => {
            // The STDIO MCP server is implemented in #10. Nothing may be written to stdout
            // here, and an unimplemented mode must exit non-zero (an MCP client must not see
            // success).
            anyhow::bail!("the STDIO MCP server is not yet implemented (see issue #10)");
        }
        None => {
            // Unreachable in practice: `arg_required_else_help` makes bare `oura` print help
            // and exit 2 before we get here. Kept as a defensive usage error.
            use clap::CommandFactory;
            Cli::command().print_help()?;
            std::process::exit(2);
        }
    }
}
