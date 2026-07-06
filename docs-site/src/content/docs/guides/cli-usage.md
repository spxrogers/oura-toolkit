---
title: CLI usage
description: The oura command surface — read commands, date windows, output formats, exit codes, rate limits, and the raw api passthrough.
sidebar:
  order: 3
---

The `oura` binary is designed to be scriptable: stable exit codes, tab-separated output when
piped, and `--json` on request. The full per-command breakdown — every flag and default — is on
the generated [CLI reference](/cli/reference/); this page is the tour.

## Read commands

Eight read commands cover your data:

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

## Date windows

Every windowed command takes `--start` / `--end` (`today`, `yesterday`, or `YYYY-MM-DD`, in
your local timezone) and defaults to the last 7 days. `--date <day>` is a single-day shorthand:

```sh
oura sleep --date yesterday
oura activity --start 2026-06-01 --end 2026-06-07
```

Dates are interpreted in your **local** timezone, because Oura's daily summaries are
user-local days. Cursor pagination is followed automatically. An empty result for a range is
success, not an error.

## Output formats

Output adapts to context:

- **On a terminal**: a human-readable aligned table.
- **Piped**: stable tab-separated lines with no header — safe for `cut` / `awk`.
- **`--json`**: pretty JSON from the command's data model, on request. Never a default.

Color is only used on a terminal, and is disabled by `--no-color` or a non-empty `NO_COLOR`
environment variable.

## Exit codes

Exit codes are a documented contract you can build scripts on:

| code | meaning |
|------|---------|
| `0` | success (data returned, or an empty range) |
| `1` | runtime error (Oura API 5xx, network/IO failure, rate limited after the retry) |
| `2` | usage error (unknown flag/command, bad `--start`/`--end`/`--date`) |
| `4` | authentication required (no tokens, a rejected refresh, or a rejected `OURA_ACCESS_TOKEN`) |

## Rate limits

Oura rate-limits the API per access token and per application (rolling windows). When a request
is rate-limited, the toolkit waits out a short `Retry-After` once — at most 10 seconds, and at
most 3 rate-limit waits across one command — then retries. If the API is still throttling, the
command fails with `rate limited until <time>` rather than retry-storming.

## Raw API passthrough

Need an endpoint the curated commands don't cover? `oura api` is an authenticated passthrough
(like `gh api`): it attaches your stored Bearer token and prints the raw JSON response to
stdout.

```sh
oura api /v2/usercollection/personal_info            # GET (default)
oura api /v2/usercollection/daily_sleep --paginate   # follow next_token, aggregate {"data":[…]}
oura api -f start_date=2026-06-01 /v2/usercollection/daily_sleep
```

`-X` / `--method` sets the method; `-f` / `--field key=value` is repeatable (query params on
GET, JSON body fields otherwise); `--paginate` (GET only) follows `next_token` to the end. The
response is printed unchanged, so it pipes cleanly into `jq`.

## Completions and man page

```sh
oura completion zsh > ~/.zfunc/_oura    # also: bash, fish, powershell, elvish
oura man > oura.1                        # roff man page
```

Both are pure generators (no auth, no network) that write to stdout. Release archives also ship
`oura.1` plus bash/zsh/fish completions, so you don't have to generate them yourself.
