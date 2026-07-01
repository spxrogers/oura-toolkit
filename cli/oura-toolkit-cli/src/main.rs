//! oura-cli — the Oura Ring toolkit CLI: interactive auth flows, data commands, and `--mcp`.
//!
//! Interactive OAuth (browser + loopback) lives here, never in the SDKs. Data commands (#9) and
//! the STDIO MCP server (#10) are wired next; both reuse the `oura-toolkit-auth` companion.

mod auth;
mod loopback;

use clap::{Parser, Subcommand};

/// Oura Ring toolkit — CLI + MCP server for the Oura API v2.
#[derive(Parser)]
#[command(name = "oura", version, about, long_about = None)]
struct Cli {
    /// Run as a STDIO MCP server (see issue #10).
    #[arg(long, global = true)]
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
    /// Guided Oura OAuth app registration (loopback paste box), then login.
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
        // The STDIO MCP server is implemented in #10. Nothing may be written to stdout here.
        eprintln!("oura --mcp: the MCP server is not yet implemented (see issue #10)");
        return Ok(());
    }

    match cli.command {
        Some(Command::Auth { action }) => match action {
            AuthAction::Setup { port } => auth::setup(port).await,
            AuthAction::Login { port } => auth::login(port).await,
        },
        None => {
            eprintln!("oura: no command given. Try `oura auth setup` or `oura --help`.");
            Ok(())
        }
    }
}
