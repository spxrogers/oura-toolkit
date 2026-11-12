# Reviewing the CLI, the MCP server and the plugin

The law is `CLAUDE.md` → *MCP* and *Plugin*, and `docs/cli-contract.md` for what the CLI
promises its users.

## Never

- Anything on stdout from `oura mcp` that is not JSON-RPC. Stdout is the transport.
- A prompt, a browser launch or an OAuth flow from `oura mcp`. With no tokens, `initialize`
  still succeeds and the first tool call returns a structured error that says to run
  `oura auth login`.
- A remote or HTTP transport for the MCP server.
- A second plugin. One plugin ships both the MCP server config and the skills.
- A new `oura` subcommand added only to render documentation: it would appear in shell
  completions.
- `oura api`'s raw request generalised to the typed commands.

## Check

- **The contract is asserted on the binary.** Exit codes, which stream output goes to, and the
  shape of errors and hints are tested by spawning `oura`
  (`cli/oura-toolkit-cli/tests/`), not only by unit-testing a classifier.
- **A changed command, flag or output** changes `docs/cli-contract.md`, the README, the
  generated CLI reference (`just docs-gen-cli-check`) and the completions
  (`just gen-completions-check`) in the same PR.
- **MCP tools** are few and curated. Descriptions come from the spec at build time, and a
  curated operation that vanishes from the spec fails the build. Dispatch calls the same
  `commands::fetch_*` functions the CLI renders from.
- **A tool result** is a text block and a `structured_content` value. A collection's array is
  enveloped under `data`; an object passes through.
- **MCP auth** is the same companion as the CLI: refresh silently on 401, persist the rotated
  tokens, retry.
- **The scope-change note** is added to the first successful tool result of a session only. It
  leaves the data and `structured_content` untouched.
- **Plugin pins.** `plugin.json`'s version and the `npx` pin in `.mcp.json`, argument position
  included, equal the workspace version: `just version-check`, `just plugin-check`.
- **Output sanitization.** Data that reaches a terminal is escaped; test it with escape
  sequences and forged lines, not only clean input.

## Gotchas this repo has met

- **`serde_json` alphabetizes.** `serde_json::Value` is a `BTreeMap` here, so sending the MCP
  text block through `to_value` reordered its keys. The text block is serialized from the typed
  data; `to_value` is for the structured envelope only.
- **`#[tool_handler]` builds a fresh router per request** unless it is given
  `router = self.tool_router`, and the build-time descriptions then vanish without an error.
- **`oura api` is Bearer-only.** It reaches the read API. Oura's webhook endpoints use a
  client-id and secret scheme it does not send, so an example that posts to them cannot work.
- **A global flag with no command.** `oura --json` parses but names no command: help goes to
  stderr, stdout stays empty, and the exit code is 2
  (`cli/oura-toolkit-cli/tests/cli_contract.rs`).
