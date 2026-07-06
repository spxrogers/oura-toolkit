---
title: CLI reference
description: The complete oura command reference, generated from the binary's own --help output.
sidebar:
  order: 1
---

{/* Frontmatter + intro for the generated CLI reference. `just docs-gen-cli` copies this file
    to reference.md and appends one section per command captured from the `oura` binary's own
    `--help`. This underscore-prefixed fragment is ignored by Astro (never its own page). */}

The complete `oura` command reference below is generated from the binary's own `--help`, so it
always matches the installed CLI — a regenerate-and-diff check ([`just docs-gen-cli-check`]) fails
CI if it drifts. For a narrative tour of the commands, see [CLI usage](/guides/cli-usage/).

[`just docs-gen-cli-check`]: https://github.com/spxrogers/oura-toolkit/blob/main/justfile

## `oura`

```text
CLI and STDIO MCP server for the Oura Ring API v2 (auth setup/login, data commands, mcp).

Usage: oura [OPTIONS] [COMMAND]

Commands:
  auth           Authentication (OAuth) flows
  sleep          Daily sleep summaries (score + contributors)
  readiness      Daily readiness summaries
  activity       Daily activity summaries (score, steps, calories)
  stress         Daily stress summaries
  heartrate      Heart-rate time series (frequent bpm samples)
  sessions       Moment/session records (meditation, naps, …)
  workouts       Workout records
  personal-info  Your Oura profile (age, height, weight, …)
  api            Authenticated passthrough to an arbitrary Oura API endpoint (like `gh api`)
  mcp            Run as a STDIO MCP server (8 read-only Oura data tools)
  completion     Print a shell completion script to stdout (bash, zsh, fish, powershell, elvish)
  man            Print the `oura` man page (roff) to stdout
  help           Print this message or the help of the given subcommand(s)

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
  -V, --version   Print version
```

## `oura auth`

```text
Authentication (OAuth) flows

Usage: oura auth [OPTIONS] <COMMAND>

Commands:
  setup    Guided Oura OAuth app registration (terminal prompts), then login
  login    Authorization Code login using stored client credentials
  status   Show stored auth state: client_id, scopes, token expiry
  logout   Delete stored tokens (log out). Keeps the client credentials unless --all is given
  refresh  Force a token refresh now and persist the rotated refresh token
  token    Print a valid access token (refreshing if needed) to stdout — and nothing else
  help     Print this message or the help of the given subcommand(s)

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

### `oura auth setup`

```text
Guided Oura OAuth app registration (terminal prompts), then login

Usage: oura auth setup [OPTIONS]

Options:
      --json         Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --port <PORT>  Loopback port for the redirect URI (must match your registered app) [default: 8788]
      --no-browser   Skip the local browser+loopback: print the URL and paste the redirect back (for SSH/containers where the callback can't reach this host)
      --no-color     Disable colored output (also honored: the NO_COLOR env var)
  -h, --help         Print help
```

### `oura auth login`

```text
Authorization Code login using stored client credentials

Usage: oura auth login [OPTIONS]

Options:
      --json         Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --port <PORT>  Loopback port for the redirect URI (must match your registered app) [default: 8788]
      --no-browser   Skip the local browser+loopback: print the URL and paste the redirect back (for SSH/containers where the callback can't reach this host)
      --no-color     Disable colored output (also honored: the NO_COLOR env var)
  -h, --help         Print help
```

### `oura auth status`

```text
Show stored auth state: client_id, scopes, token expiry

Usage: oura auth status [OPTIONS]

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

### `oura auth logout`

```text
Delete stored tokens (log out). Keeps the client credentials unless --all is given

Usage: oura auth logout [OPTIONS]

Options:
      --all       Also remove the stored client credentials (client_id + client_secret)
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

### `oura auth refresh`

```text
Force a token refresh now and persist the rotated refresh token

Usage: oura auth refresh [OPTIONS]

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

### `oura auth token`

```text
Print a valid access token (refreshing if needed) to stdout — and nothing else

Usage: oura auth token [OPTIONS]

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

## `oura sleep`

```text
Daily sleep summaries (score + contributors)

Usage: oura sleep [OPTIONS]

Options:
      --json           Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --start <START>  Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)
      --end <END>      End date: today, yesterday, or YYYY-MM-DD (default: today)
      --no-color       Disable colored output (also honored: the NO_COLOR env var)
      --date <DATE>    A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end
  -h, --help           Print help
```

## `oura readiness`

```text
Daily readiness summaries

Usage: oura readiness [OPTIONS]

Options:
      --json           Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --start <START>  Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)
      --end <END>      End date: today, yesterday, or YYYY-MM-DD (default: today)
      --no-color       Disable colored output (also honored: the NO_COLOR env var)
      --date <DATE>    A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end
  -h, --help           Print help
```

## `oura activity`

```text
Daily activity summaries (score, steps, calories)

Usage: oura activity [OPTIONS]

Options:
      --json           Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --start <START>  Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)
      --end <END>      End date: today, yesterday, or YYYY-MM-DD (default: today)
      --no-color       Disable colored output (also honored: the NO_COLOR env var)
      --date <DATE>    A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end
  -h, --help           Print help
```

## `oura stress`

```text
Daily stress summaries

Usage: oura stress [OPTIONS]

Options:
      --json           Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --start <START>  Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)
      --end <END>      End date: today, yesterday, or YYYY-MM-DD (default: today)
      --no-color       Disable colored output (also honored: the NO_COLOR env var)
      --date <DATE>    A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end
  -h, --help           Print help
```

## `oura heartrate`

```text
Heart-rate time series (frequent bpm samples)

Usage: oura heartrate [OPTIONS]

Options:
      --json           Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --start <START>  Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)
      --end <END>      End date: today, yesterday, or YYYY-MM-DD (default: today)
      --no-color       Disable colored output (also honored: the NO_COLOR env var)
      --date <DATE>    A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end
  -h, --help           Print help
```

## `oura sessions`

```text
Moment/session records (meditation, naps, …)

Usage: oura sessions [OPTIONS]

Options:
      --json           Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --start <START>  Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)
      --end <END>      End date: today, yesterday, or YYYY-MM-DD (default: today)
      --no-color       Disable colored output (also honored: the NO_COLOR env var)
      --date <DATE>    A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end
  -h, --help           Print help
```

## `oura workouts`

```text
Workout records

Usage: oura workouts [OPTIONS]

Options:
      --json           Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --start <START>  Start date: today, yesterday, or YYYY-MM-DD (default: 6 days before --end)
      --end <END>      End date: today, yesterday, or YYYY-MM-DD (default: today)
      --no-color       Disable colored output (also honored: the NO_COLOR env var)
      --date <DATE>    A single day (today, yesterday, or YYYY-MM-DD) — shorthand for --start X --end X. Mutually exclusive with --start/--end
  -h, --help           Print help
```

## `oura personal-info`

```text
Your Oura profile (age, height, weight, …)

Usage: oura personal-info [OPTIONS]

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

## `oura api`

```text
Authenticated passthrough to an arbitrary Oura API endpoint (like `gh api`).

GET by default; the stored auth is attached and the raw JSON response is printed to stdout. `-f key=value` adds query params (GET/HEAD/DELETE) or JSON body fields (other methods); a request body can also be piped on stdin. `--paginate` follows `next_token`.

Usage: oura api [OPTIONS] <PATH>

Arguments:
  <PATH>
          Request path, resolved against the API base URL (e.g. /v2/usercollection/personal_info). A leading `/` is optional

Options:
      --json
          Output JSON instead of the default table/plain rendering (data commands and `auth status`)

  -X, --method <METHOD>
          HTTP method (default GET)
          
          [default: GET]

  -f, --field <FIELD>
          Add a field (key=value). Query param for GET/HEAD/DELETE, else a JSON body field. Repeatable

      --no-color
          Disable colored output (also honored: the NO_COLOR env var)

      --paginate
          Follow `next_token` pagination and aggregate every page's `data` array into one `{"data":[…]}` object (GET only)

  -h, --help
          Print help (see a summary with '-h')
```

## `oura mcp`

```text
Run as a STDIO MCP server (8 read-only Oura data tools)

Usage: oura mcp [OPTIONS]

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

## `oura completion`

```text
Print a shell completion script to stdout (bash, zsh, fish, powershell, elvish)

Usage: oura completion [OPTIONS] <SHELL>

Arguments:
  <SHELL>  Shell to generate the completion script for [possible values: bash, elvish, fish, powershell, zsh]

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```

## `oura man`

```text
Print the `oura` man page (roff) to stdout

Usage: oura man [OPTIONS]

Options:
      --json      Output JSON instead of the default table/plain rendering (data commands and `auth status`)
      --no-color  Disable colored output (also honored: the NO_COLOR env var)
  -h, --help      Print help
```
