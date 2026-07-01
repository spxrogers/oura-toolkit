//! oura-auth — reusable, non-interactive auth companion for the Oura Ring API.
//!
//! Provides the token store, confidential-client refresh (with refresh-token rotation), and
//! the Bearer-injecting `reqwest` middleware shared by BOTH the CLI's SDK calls and the MCP
//! server's tool calls. Interactive OAuth (browser + loopback) lives only in `oura-cli`,
//! never here.
//!
//! Implementation tracked in <https://github.com/spxrogers/oura-toolkit/issues/7>
