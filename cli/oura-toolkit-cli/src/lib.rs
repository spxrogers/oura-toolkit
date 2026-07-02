//! oura-toolkit-cli library target: the CLI's modules, exposed so the integration tests
//! (`tests/`) can exercise the REAL command functions against wiremock — per the release
//! gate, the code that ships is the code that's tested, not a parallel copy.
//!
//! This crate is an app, not an SDK (CLAUDE.md): the library target exists solely for the
//! binary and its tests. External consumers want `oura-toolkit-api`/`oura-toolkit-auth`.

pub mod api;
pub mod auth;
pub mod commands;
pub mod contract;
pub mod loopback;
pub mod mcp;
pub mod output;
