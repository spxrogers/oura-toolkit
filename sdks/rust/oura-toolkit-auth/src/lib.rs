//! oura-toolkit-auth — reusable, non-interactive auth companion for the Oura Ring API.
//!
//! Provides the token store, confidential-client token exchange + refresh (with refresh-token
//! rotation), spec-derived OAuth metadata, and the Bearer-injecting `reqwest` middleware shared
//! by BOTH the CLI's SDK calls and the MCP server's tool calls. Interactive OAuth (browser +
//! loopback listener) lives only in `oura-toolkit-cli`, never here.
//!
//! ```no_run
//! use std::sync::Arc;
//! use oura_toolkit_auth::{TokenManager, build_authenticated_client};
//!
//! # async fn demo() -> Result<(), oura_toolkit_auth::AuthError> {
//! let manager = Arc::new(TokenManager::load()?);
//! let client = build_authenticated_client(manager); // hand to the generated SDK client
//! # let _ = client;
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
pub mod metadata;
mod oauth;
mod store;

pub use client::{build_authenticated_client, AuthMiddleware, TokenManager};
pub use error::AuthError;
pub use oauth::{exchange_code, refresh};
pub use store::{TokenStore, Tokens};
