# oura-toolkit

Your [Oura Ring](https://ouraring.com) data, everywhere you work: a fast Rust CLI
(`oura`), a local [MCP](https://modelcontextprotocol.io) server for AI assistants, a
Claude plugin with wellness skills, and generated SDK clients in six languages (Rust,
TypeScript, Python, Go, Java and C#) — all driven by Oura's own OpenAPI spec. Plus a
local life↔health engine: import your Apple Watch data (Apple Health export), calendar
and Toggl Track history, and ask how much more your week can take (`oura capacity`).

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

> **Pre-release note:** the npx / brew / bun paths — and the MCP + plugin routes below,
> which launch via npx — activate with the first tagged release. Until then, build from
> source: `cargo install --git https://github.com/spxrogers/oura-toolkit oura-toolkit-cli`.

## One-time setup: register your Oura app and log in

Oura's API uses OAuth with a confidential client, so you register your own (free) Oura
application and keep the credentials on your machine. The CLI walks you through all of
it:

```sh
oura auth setup
```

which does the following, interactively:

1. Opens <https://cloud.ouraring.com/oauth/applications> in your browser and prints the
   exact values to enter on Oura's form:
   - **Application name**: anything (e.g. `oura-toolkit`)
   - **Redirect URI**: `http://localhost:8788/callback` — must match exactly
   - **Scopes**: `personal daily heartrate workout tag session spo2Daily`
2. Prompts for the app's **client id** and **client secret** in the terminal (the secret
   with hidden input — it never leaves your machine).
3. Chains straight into `oura auth login`: your browser opens Oura's consent page, a
   local listener on port 8788 catches the callback, and tokens are stored.

Already registered? Just run `oura auth login`. If port 8788 is taken, use
`--port <n>` — and register the matching redirect URI. No browser on the machine (or it
doesn't open)? Both flows print the URL to visit manually.

Tokens and credentials live in `~/.config/oura-toolkit/` (owner-only file modes; on
Windows, `%LOCALAPPDATA%\oura-toolkit\` under your profile's private ACLs) and refresh
automatically from then on.

## CLI

Eight read commands over your Oura data:

```sh
oura sleep            # daily sleep scores + contributors
oura readiness        # daily readiness + temperature deviation
oura activity         # score, steps, calories
oura stress           # high-stress vs recovery time
oura heartrate        # bpm time series (frequent samples; expect many rows)
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

## Local health store: imports, capacity, analogs

Four more commands feed and read a **local day-grain store** — the life↔health engine
([design doc](docs/vision-2026-07-03-personal-health-context.md)):

```sh
oura sync                              # pull your Oura dailies into the store (default: last 90 days)
oura import apple-health export.zip    # Apple Health export (your Apple Watch data)
oura import calendar work.ics          # calendar history AND future events
oura import toggl report.csv           # Toggl Track detailed-report CSV
oura capacity                          # how much more this week can take (0-100%)
oura context                           # the merged day-grain records the engine sees
```

**Apple Health (Apple Watch)** — on the iPhone: Health app → your picture (top right) →
"Export All Health Data", AirDrop the `export.zip` to this machine (AirDrop is
peer-to-peer; it never transits iCloud), then `oura import apple-health
~/Downloads/export.zip`. The importer streams the export into per-day aggregates (sleep
stages, resting HR, HRV, steps, workouts, SpO2, wrist temperature) and never retains
raw samples. Delete the zip afterwards — it holds your full plaintext health history —
or pass `--remove-source` to have the import do it for you.

**Calendar (.ics)** — export from Google Calendar, Apple Calendar, or Outlook. Only
derived numbers are stored (meeting hours, event counts, evening events, first/last
event times) — never titles, attendees, or locations. Future events matter: they are
what lets `oura capacity` score the weeks ahead.

**Toggl Track** — the detailed-report CSV export. Tracked time-blocks become per-day
shape (hours, entry count, longest block); entry descriptions are never retained.

**`oura capacity`** answers *"how much more can this week take?"* from your own
history: 100 minus attributed deductions for **recovery debt** (recent readiness vs
your baseline), **week load** (this week's schedule vs your norm), and **analog risk**
(how the most similar past weeks turned out, including the two weeks after them).
Bands: comfortable ≥ 70, stretched 40–69, overloaded < 40. Below 8 weeks of usable
history it refuses instead of guessing — and everything it reports is n=1
observational data: *"in your history, weeks like this were followed by…"*, never a
prediction.

### Where the data lives, and its safety properties

- The store is `~/.local/share/oura-toolkit/` (`$XDG_DATA_HOME`; Windows:
  `%LOCALAPPDATA%\oura-toolkit\data\`) — owner-only file modes and atomic writes, the
  same discipline as the token store.
- **Day-grain aggregates only**: raw samples never land on disk.
- It is a **rebuildable cache**: sources of truth stay in Apple Health / Oura / your
  calendar. Losing it costs a re-import, nothing more.
- Imports **warn if the store path resolves inside a cloud-synced folder**
  (Dropbox / iCloud Drive / OneDrive / Google Drive) — health data should not silently
  replicate off the machine.
- Local-first means every egress is a choice you make. There are exactly two: requests
  to Oura with your own credentials, and MCP tool results read by whatever AI assistant
  you connect.
- Oura HRV (rMSSD) and Apple HRV (SDNN) are different statistics — every surface keeps
  them provider-tagged and never blends them.

## MCP server (AI assistants)

```sh
oura mcp
```

runs a STDIO MCP server exposing twelve curated, described tools: eight over your Oura
data (`get_daily_sleep`, `get_daily_readiness`, …) and four over the local store
(`get_capacity`, `find_analog_weeks`, `get_upcoming_load`, `get_day_context`). Tool
results are read by whichever AI assistant you connect — that's the point — while
credentials stay local. The server reuses the same stored tokens and refreshes them
silently; if you haven't logged in, tool calls return a structured error telling you to
run `oura auth login` — the server never prompts or opens a browser itself. The
local-store tools need no Oura auth at all, only imported data.

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

Every language gets the same shape under `sdks/<lang>/`: a data-plane client generated
from Oura's OpenAPI spec (auth-agnostic — bring a Bearer token) plus a hand-written auth
companion (token store + refresh). What exists today:

| Language | Generated client | Auth companion | Package name (reserved) |
|---|---|---|---|
| Rust | `sdks/rust/oura-toolkit-api` (the CLI runs on it — dogfooded end to end) | `oura-toolkit-auth` ✅ | `oura-toolkit-api` / `oura-toolkit-auth` (crates.io) |
| TypeScript | `sdks/typescript/api` | planned | `@oura-toolkit/api` (npm) |
| Python | `sdks/python` (`oura_toolkit.api`) | planned (`oura_toolkit.auth`) | `oura-toolkit` (PyPI) |
| Go | `sdks/go` | planned | module `github.com/spxrogers/oura-toolkit/sdks/go` |
| Java | `sdks/java/api` | planned | `com.ouratoolkit:api` (Maven Central) |
| C# | `sdks/csharp/api` | planned | `OuraToolkit.Api` (NuGet) |

The breadth clients are compile-checked and drift-checked in CI, and smoke-tested against
Oura's live sandbox (TypeScript, Python and Go today; Java/C# smokes arrive with their
auth companions), but **not yet published** to their registries — consume them from
source for now. Until each language's auth companion lands, supply your own access token.

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
