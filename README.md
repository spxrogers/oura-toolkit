# oura-toolkit

Your [Oura Ring](https://ouraring.com) data, everywhere you work: a fast Rust CLI
(`oura`), a local [MCP](https://modelcontextprotocol.io) server for AI assistants, a
Claude plugin with wellness skills, and (coming) generated SDKs for TypeScript, Python,
Go and Rust — all driven by Oura's own OpenAPI spec.

```
$ oura sleep
DAY         SCORE  DEEP  REM  EFFICIENCY
2026-06-27  82     70    85   93
2026-06-28  77     64    79   90
...

$ oura readiness --json | jq '.[0].score'
88
```

- **Read-only and local-first**: your credentials are only ever sent to Oura — never to
  any third party.
- **Bring your own Oura app**: OAuth with *your* client id/secret (Oura deprecated
  personal access tokens in Dec 2025). Guided registration takes ~2 minutes, once.
- **Scriptable**: stable exit codes, TSV when piped, `--json` — see the
  [CLI contract](docs/cli-contract.md).

## Install

**npx (recommended — nothing to install):**

```sh
npx -y oura-toolkit --help
```

**Faster paths for regular use:**

```sh
brew install spxrogers/tap/oura-toolkit   # Homebrew (macOS/Linux)
bun install -g oura-toolkit               # Bun
cargo install oura-toolkit-cli            # from source; installs the `oura` binary
```

Shell and PowerShell installers ship with every
[GitHub release](https://github.com/spxrogers/oura-toolkit/releases). Whichever route you
choose, the command is `oura` (via npx: `npx -y oura-toolkit <args>`).

> **Pre-release note:** package registries populate with the first tagged release. Until
> then, build from source: `cargo install --git https://github.com/spxrogers/oura-toolkit oura-toolkit-cli`.

## One-time setup: register your Oura app and log in

Oura's API uses OAuth with a confidential client, so you register your own (free) Oura
application and keep the credentials on your machine. The CLI walks you through all of
it:

```sh
oura auth setup
```

which does the following, interactively:

1. Opens <https://cloud.ouraring.com/oauth/applications> in your browser. Create an app
   with:
   - **App name**: anything (e.g. `my-oura-toolkit`)
   - **Redirect URI**: `http://localhost:8788/callback` — must match exactly
2. Prompts for the app's **client id** and **client secret** in the terminal (the secret
   with hidden input — it never leaves your machine).
3. Chains straight into `oura auth login`: your browser opens Oura's consent page
   (scopes requested: `personal daily heartrate workout tag session spo2Daily`), a local
   listener on port 8788 catches the callback, and tokens are stored.

Already registered? Just run `oura auth login`. If port 8788 is taken, use
`--port <n>` — and register the matching redirect URI.

Tokens and credentials live in `~/.config/oura-toolkit/` (owner-only file modes; on
Windows, `%LOCALAPPDATA%\oura-toolkit\` under your profile's private ACLs) and refresh
automatically from then on.

## CLI

Eight read commands over your data:

```sh
oura sleep            # daily sleep scores + contributors
oura readiness        # daily readiness + temperature deviation
oura activity         # score, steps, calories
oura stress           # high-stress vs recovery time
oura heartrate        # bpm time series (~5-minute samples)
oura sessions         # meditation, naps, breathing sessions
oura workouts         # workouts with intensity + calories
oura personal-info    # your profile
```

Every windowed command takes `--start` / `--end` (`today`, `yesterday`, or
`YYYY-MM-DD`, in your local timezone) and defaults to the last 7 days. Cursor pagination
is followed automatically.

Output adapts to context: aligned tables on a terminal, stable tab-separated lines when
piped (`cut`/`awk`-safe), pretty JSON with `--json`. Exit codes are a documented
contract (`0` ok, `1` runtime, `2` usage, `4` auth needed) — details in
[docs/cli-contract.md](docs/cli-contract.md).

## MCP server (AI assistants)

```sh
oura mcp
```

runs a STDIO MCP server exposing eight curated, described tools (`get_daily_sleep`,
`get_daily_readiness`, …). Tool results are read by whichever AI assistant you connect —
that's the point — while credentials stay local. The server reuses the same stored
tokens and refreshes them silently;
if you haven't logged in, tool calls return a structured error telling you to run
`oura auth login` — the server never prompts or opens a browser itself.

For Claude Code:

```sh
claude mcp add oura -- npx -y oura-toolkit mcp
```

## Claude plugin

The batteries-included route — MCP server plus `/morning-checkin` and
`/wellness-report` skills:

```
/plugin marketplace add spxrogers/oura-toolkit
/plugin install oura-toolkit@oura-toolkit
```

Then ask Claude "how did I sleep?". See [plugins/oura-toolkit](plugins/oura-toolkit/)
for details.

## SDKs

`sdks/rust/oura-toolkit-api` is the generated Rust client (the CLI runs on it —
dogfooded end to end) and `sdks/rust/oura-toolkit-auth` the token-store/refresh
companion. TypeScript, Python and Go SDKs are planned under `sdks/<lang>/` with the same
client + auth-companion shape.

## Developing

One prerequisite: install [`just`](https://github.com/casey/just). Everything else is a
recipe:

```sh
just setup   # toolchains + codegen/release tooling
just ci      # what CI runs: fmt-check + lint + tests
just         # list all recipes
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the dev loop, and
[CLAUDE.md](CLAUDE.md) for the architecture decisions and the testing law this repo is
built under ("green CI is the release decision").

## License

MIT
