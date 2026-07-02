//! oura-cli — the Oura Ring toolkit CLI: interactive auth flows, data commands, and `--mcp`.
//!
//! Interactive OAuth (browser + loopback) lives here, never in the SDKs. Data commands (#9) and
//! the STDIO MCP server (#10) are wired next; both reuse the `oura-toolkit-auth` companion.

mod auth;
mod loopback;

use clap::{Parser, Subcommand};

/// Oura Ring toolkit — CLI + MCP server for the Oura API v2.
#[derive(Parser)]
#[command(name = "oura", version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    /// Run as a STDIO MCP server (see issue #10).
    // Deliberately NOT `global`: `oura auth login --mcp` must be a usage error, not a silent
    // mode switch. The flag-vs-subcommand decision is tracked in issue #21.
    #[arg(long)]
    mcp: bool,

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
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.mcp {
        // `--mcp` is a mode, not a modifier — combining it with a subcommand is a usage error
        // (clap can't express a flag/subcommand conflict, so enforce it here).
        if cli.command.is_some() {
            eprintln!("oura: --mcp cannot be combined with a subcommand");
            std::process::exit(2);
        }
        // The STDIO MCP server is implemented in #10. Nothing may be written to stdout here,
        // and an unimplemented mode must exit non-zero (an MCP client must not see success).
        anyhow::bail!("the STDIO MCP server is not yet implemented (see issue #10)");
    }

    match cli.command {
        Some(Command::Auth { action }) => match action {
            AuthAction::Setup { port } => auth::setup(port).await,
            AuthAction::Login { port } => auth::login(port).await,
        },
        None => {
            // Unreachable in practice: `arg_required_else_help` makes bare `oura` print help
            // and exit 2 before we get here. Kept as a defensive usage error.
            use clap::CommandFactory;
            Cli::command().print_help()?;
            std::process::exit(2);
        }
    }
}
