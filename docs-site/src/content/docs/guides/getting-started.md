---
title: Getting started
description: Install oura-toolkit, register your Oura app, and pull your first data in a couple of minutes.
sidebar:
  order: 1
---

`oura-toolkit` gives you your [Oura Ring](https://ouraring.com) data everywhere you work: a
fast CLI (`oura`), a local MCP server for AI assistants, a Claude plugin, and generated SDK
clients in six languages. This page gets you from nothing to your first data in about two
minutes.

## Install

The recommended route needs nothing installed — `npx` fetches and runs the latest release:

```sh
npx -y oura-toolkit --help
```

Prefer a permanent install? Any of these give you an `oura` command on your `PATH`:

```sh
brew install spxrogers/tap/oura-toolkit   # Homebrew (macOS/Linux)
bun install -g oura-toolkit               # Bun
cargo install oura-toolkit-cli            # from source; installs the `oura` binary
```

Shell and PowerShell installers ship with every
[GitHub release](https://github.com/spxrogers/oura-toolkit/releases). Whichever route you
pick, the command you type is `oura` (via npx, `npx -y oura-toolkit <args>`).

:::note[Pre-release]
The npx / brew / bun paths — and the MCP + plugin routes, which launch via npx — activate
with the first tagged release. Until then, build from source:
`cargo install --git https://github.com/spxrogers/oura-toolkit oura-toolkit-cli`.
:::

## Register your Oura app and log in

Oura's API uses OAuth with a confidential client, so you register your own free Oura
application and keep the credentials on your machine. The CLI walks you through all of it:

```sh
oura auth setup
```

This opens Oura's application page, prints the exact values to paste, collects your client id
and secret in the terminal, and then logs you in. The full walkthrough — including the
headless/SSH path — is on the [Authentication](/guides/authentication/) page.

## Pull your first data

Once you're logged in, every read command just works:

```sh
oura sleep
oura readiness --json | jq '.[0].score'
```

```
DAY         SCORE  DEEP  REM  EFFICIENCY
2026-06-27  82     70    85   93
2026-06-28  77     64    79   90
```

From here:

- **[CLI usage](/guides/cli-usage/)** — all eight read commands, date ranges, output formats,
  exit codes, and the `oura api` passthrough.
- **[MCP server](/guides/mcp-server/)** — expose your data to AI assistants with `oura mcp`.
- **[Claude plugin](/guides/claude-plugin/)** — the batteries-included wellness skills.
- **[API reference](/api/)** — every Oura API v2 endpoint, generated from the spec.
