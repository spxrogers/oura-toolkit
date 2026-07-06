---
title: MCP server
description: Run oura mcp as a local STDIO MCP server exposing eight curated Oura data tools to any AI assistant.
sidebar:
  order: 5
---

```sh
oura mcp
```

runs a **STDIO [MCP](https://modelcontextprotocol.io) server** exposing eight curated,
well-described tools. Tool results are read by whichever AI assistant you connect — that's the
point — while your credentials stay local.

## The tools

| Tool | Data |
|------|------|
| `get_daily_sleep` | daily sleep score + contributors |
| `get_daily_readiness` | daily readiness + temperature deviation |
| `get_daily_activity` | daily activity score, steps, calories |
| `get_daily_stress` | daily high-stress vs recovery time |
| `get_heart_rate` | heart-rate time series (bpm samples) |
| `get_sessions` | moment/session records (meditation, naps, …) |
| `get_workouts` | workout records |
| `get_personal_info` | your Oura profile |

Each takes a curated date range (local-timezone dates; cursor pagination is handled for you)
and returns both a text block and structured content.

## Authentication

The server reuses the same stored tokens as the CLI and refreshes them silently on a `401`. If
you haven't logged in, tool calls return a structured error telling you to run `oura auth
login` — the server never prompts, opens a browser, or writes to stdout (stdout is the JSON-RPC
transport). In a container with no stored login, inject a token instead:

```sh
OURA_ACCESS_TOKEN=<token> oura mcp
```

## Connecting Claude Code

```sh
claude mcp add oura -- npx -y oura-toolkit mcp
```

Then ask your assistant "how did I sleep?" and it will call `get_daily_sleep` for you. For a
batteries-included experience with ready-made wellness skills, use the
[Claude plugin](/guides/claude-plugin/) instead.

:::note
stdio MCP authentication is out-of-band by design — the server is local and STDIO-only. It is
never a remote/HTTP server or a hosted OAuth broker.
:::
