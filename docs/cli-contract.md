# CLI contract

The scripting contract for the `oura` binary (#21). Changes to any of this are breaking
changes for users' scripts and require a deliberate decision.

## Exit codes

| code | meaning | example |
|------|---------|---------|
| `0` | success | data returned, auth flow completed |
| `1` | runtime error | Oura API 5xx, network failure, I/O error, rate limited (429) after the bounded retry |
| `2` | usage error | unknown flag/command, missing argument, bare `oura`, invalid flag values (malformed `--start`/`--end` date, inverted range) |
| `4` | authentication required | no stored tokens/credentials, a refresh rejected by the token endpoint (HTTP 400, e.g. `invalid_grant`), or a rejected `OURA_ACCESS_TOKEN` — usually an auth flow fixes it; a rejected env token needs a fresh one |

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

## Shell completions and man page

- `oura completion <shell>` (`bash`, `zsh`, `fish`, `powershell`, `elvish`) prints a
  completion script to **stdout**; `oura man` prints the roff man page to **stdout**. Both
  are the result, so they follow the stream rules above (redirect stdout to a file).
- Pure generators: no auth, no network, no config read — safe to run anywhere.
- Success is exit `0`; an unknown shell is a usage error (exit `2`, message on stderr,
  stdout empty), like any other bad argument value.

## Output formats (data commands and `auth status`)

- **TTY default:** human-readable aligned table.
- **Piped default:** stable tab-separated lines, no header — safe for `cut`/`awk`.
- **`--json`:** pretty JSON from the command's data model. Never a default.
- **Color:** only on a TTY; disabled by `--no-color` or a non-empty `NO_COLOR` env var
  ([no-color.org](https://no-color.org)).

## Dates (data commands)

- `--start` / `--end` accept `today`, `yesterday`, or `YYYY-MM-DD`.
- Defaults: `--end` is today; `--start` is 6 days before `--end` (a 7-day window).
- Dates are interpreted in the **user's local timezone** — Oura's daily summaries are
  user-local days, so `today` means the wearer's today, not UTC's.
- Daily endpoints send the dates as-is. Time-series endpoints (heartrate) expand the range
  to local `00:00:00`–`23:59:59` and convert those instants to UTC on the wire.
- An empty result for a date range is **success** (exit 0) with empty output, not an error.

## Auth commands

`oura auth setup` and `oura auth login` are interactive; the account commands
`status` / `logout` / `refresh` / `token` are non-interactive and scriptable:

- **`oura auth setup` / `oura auth login`** — interactive walkthroughs with no scriptable
  result: every line of guidance, the visible prompts, and the `✓ Done` confirmation go to
  **stderr** (the hidden secret prompt reads straight from the terminal), so stdout stays
  empty — a piped `oura auth token`/`status` is never polluted by a login run. `--no-browser`
  swaps the loopback callback for a paste-back flow (print the authorize URL, read the pasted
  redirect from stdin) for hosts the callback can't reach; the same `state` CSRF check
  applies, and a mismatch aborts (exit 1, stdout empty).
- **`oura auth status`** — the state report (store path, `client_id` — never the client
  secret — scopes, access-token expiry) is the command's **result** and goes to stdout;
  `--json` emits a machine-readable model instead. Exit `0` when a data command would
  get a token (tokens valid beyond the proactive-refresh window, or refreshable), exit
  `4` when one would fail auth — the report is still printed first, and the stderr hint
  names the fix (`setup` vs `login`).
- **`oura auth logout`** — deletes stored tokens; `--all` also deletes the client
  credentials (the sanctioned way to remove the stored secret). Idempotent: nothing
  stored is success (exit `0`), not an error. As a mutation it has no result: stdout
  stays empty and the confirmation is prose on stderr. A concurrently running process
  (e.g. `oura mcp`) keeps its in-memory access token until expiry, but treats the
  deleted record as authoritative — it cannot refresh or re-persist after logout.
- **`oura auth refresh`** — forces a token refresh and persists the rotated refresh
  token. Like `logout`, the confirmation goes to stderr and stdout stays empty.
  Auth-shaped failures exit `4` with the usual hints.
- **`oura auth token`** — prints a valid access token (refreshing first if needed) to
  stdout: the token, one trailing newline, **nothing else**, so
  `curl -H "Authorization: Bearer $(oura auth token)" …` composes cleanly. When
  unauthenticated: exit `4`, stdout stays empty.

## Environment

Overrides for headless/CI/container use. Read once at startup; empty or whitespace-only
values are ignored (treated as unset).

- **`OURA_ACCESS_TOKEN`** — a raw OAuth access token used by the **data commands** and
  **`oura mcp`**, bypassing the token store: no login, no refresh. Takes precedence over any
  stored tokens. When the API rejects it (expired/invalid) the command exits `4` with a
  "export a fresh one" hint; the MCP server returns a structured tool error saying to restart
  with a fresh token. NOT honored by the `oura auth` account commands (`status`/`token`/
  `refresh`/`logout`), which operate on the store itself.
- **`OURA_API_BASE_URL`** — the data-plane base URL (default `https://api.ouraring.com`).
  Point it at a proxy, an alternate Oura host, or a mock. A trailing slash is trimmed. Over
  plain `http://` the Bearer token is sent in cleartext — use that only for a trusted local
  proxy or mock, never a real network path.
- **`NO_COLOR`** — disables ANSI color (same as `--no-color`).

## Error style

One line to stderr: `oura: <what failed>: <why>` — followed by a `hint:` line when the fix
is known:

```
oura: fetching sleep data: not authenticated (no tokens stored)
hint: run `oura auth login`
```

No backtraces for expected errors.

## Rate limits (429)

Oura enforces per-access-token and per-application limits (rolling windows; the 429
response headers carry the specifics). On a 429 the toolkit honors `Retry-After` **once**,
and only when it asks for ≤ 10 seconds — then the request fails as a runtime error
(exit `1`) whose message names the reset time:

```
oura: fetching daily sleep: the Oura API rate limit was exceeded — rate limited until 2026-07-04T18:30:00Z
hint: wait until the reset time shown above, then re-run
```

Never more than one rate-limit retry per request (no retry storms), and never more than
3 rate-limit waits across one command invocation — auto-followed pagination shares that
budget, so a throttle-every-page server cannot stretch a command indefinitely, and
pagination stops at the first rate-limited page that fails. MCP tool calls surface the
same message and hint as a structured tool error.
