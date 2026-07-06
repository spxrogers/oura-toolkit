---
title: Claude plugin
description: The batteries-included route — the MCP server plus morning-checkin and wellness-report skills for Claude.
sidebar:
  order: 6
---

The Claude plugin is the batteries-included route: it bundles the `oura mcp` server config
**and** two wellness skills, so you go from install to "how did I sleep?" without wiring
anything up yourself.

## Install

```
/plugin marketplace add spxrogers/oura-toolkit
/plugin install oura-toolkit@oura-toolkit
```

The plugin's MCP server launches via `npx -y oura-toolkit@<version> mcp`, so it always runs a
pinned release. It uses the same local token store as the CLI.

## Skills

- **`/morning-checkin`** — a quick read on how you slept and whether you're recovered, pulling
  daily sleep, readiness, and activity.
- **`/wellness-report`** — a broader look across a date range you choose.

Both skills call the MCP tools under the hood. If you haven't logged in yet, they point you at
`oura auth login` — the same one-time [authentication](/guides/authentication/) step the CLI
uses.

## Requirements

You still register your own Oura app and log in once (see
[Authentication](/guides/authentication/)); the plugin uses your local credentials and never
sends them anywhere but Oura.
