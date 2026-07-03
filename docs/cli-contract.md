# CLI contract

The scripting contract for the `oura` binary (#21). Changes to any of this are breaking
changes for users' scripts and require a deliberate decision.

## Exit codes

| code | meaning | example |
|------|---------|---------|
| `0` | success | data returned, auth flow completed |
| `1` | runtime error | Oura API 5xx, network failure, I/O error |
| `2` | usage error | unknown flag/command, missing argument, bare `oura`, invalid flag values (malformed `--start`/`--end` date, inverted range) |
| `4` | authentication required | no stored tokens/credentials, or a refresh rejected as invalid by the token endpoint (HTTP 400, e.g. `invalid_grant`) — an auth flow will fix it |

Scripting example (`oura auth login` is interactive, so an unattended script reports
rather than retries; note `$?` is captured immediately — an `if !` wrapper would clobber
it):

```sh
oura sleep --json > sleep.json
status=$?
if [ "$status" -ne 0 ]; then
  case "$status" in
    4) echo "authentication required — run 'oura auth login', then re-run" >&2 ;;
    *) echo "oura failed with exit $status" >&2 ;;
  esac
  exit 1
fi
```

## Streams

- **stdout** — results only (tables, plain lines, JSON). `oura mcp` is stricter: stdout is
  the JSON-RPC transport and carries nothing else; the client disconnecting (EOF on
  stdin, even before the handshake) is a clean exit 0.
- **stderr** — everything meant for humans: prose, progress, prompts, errors, hints.
- **Early-closed pipe** — a downstream consumer closing stdout early (`oura heartrate |
  head -1`) is **success** (exit 0, nothing on stderr): the consumer got everything it
  wanted. Never a panic/backtrace.

## Output formats (data commands)

- **TTY default:** human-readable aligned table.
- **Piped default:** stable tab-separated lines, no header — safe for `cut`/`awk`.
- **`--json`:** pretty JSON from the command's data model. Never a default.
- **Color:** only on a TTY; disabled by `--no-color` or a non-empty `NO_COLOR` env var
  ([no-color.org](https://no-color.org)).

## Dates (data commands)

- `--start` / `--end` accept `today`, `yesterday`, or `YYYY-MM-DD`.
- Defaults: `--end` is today; `--start` is 6 days before `--end` (a 7-day window).
  Exception: `oura sync` defaults `--start` to 89 days before `--end` (a 90-day
  window) — the engine's baselines want history, and the window is explicit in
  `oura sync --help`.
- Dates are interpreted in the **user's local timezone** — Oura's daily summaries are
  user-local days, so `today` means the wearer's today, not UTC's.
- Daily endpoints send the dates as-is. Time-series endpoints (heartrate) expand the range
  to local `00:00:00`–`23:59:59` and convert those instants to UTC on the wire.
- An empty result for a date range is **success** (exit 0) with empty output, not an error.

## Local store commands (`sync` / `import` / `capacity` / `context`)

- `oura sync` and `oura import …` write the day-grain health store
  (`~/.local/share/oura-toolkit/`, `$XDG_DATA_HOME`-honoring; Windows
  `%LOCALAPPDATA%\oura-toolkit\data\`); `oura capacity` / `oura context` only read it.
  The import summary (source, day counts, store path) is the RESULT and goes to
  stdout; safety notices go to stderr.
- A malformed input file (`not an Apple Health export/.ics/Toggl CSV`) is a **runtime
  error (exit 1)** naming the file and what failed to parse — not a usage error: the
  invocation was fine, the file wasn't.
- `oura capacity` (and the analog MCP tools) **refuse below 8 weeks of usable
  history** with exit 1 and a message naming the import commands that fix it. The
  refusal is deliberate contract, not a bug: thin history would make the number
  fabricated confidence.
- Two stderr safety notices are contract: imports warn when the store directory
  resolves inside a cloud-synced folder, and `oura import apple-health` without
  `--remove-source` reminds you the plaintext export still exists.
- Re-running the same import or sync is idempotent (`unchanged` in the summary); a
  re-import REPLACES that source's slot for the days it covers and never touches other
  sources' data.

## Error style

One line to stderr: `oura: <what failed>: <why>` — followed by a `hint:` line when the fix
is known:

```
oura: fetching sleep data: not authenticated (no tokens stored)
hint: run `oura auth login`
```

No backtraces for expected errors.
