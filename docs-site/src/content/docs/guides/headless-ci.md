---
title: Headless & CI
description: Run oura-toolkit on servers, in containers, and in CI with environment overrides and the paste-back login flow.
sidebar:
  order: 4
---

`oura-toolkit` is built to run where there's no browser and no interactive terminal — remote
hosts, containers, and CI. Three mechanisms cover those cases.

## Paste-back login (`--no-browser`)

On a remote or headless host where the OAuth loopback callback can't reach you (SSH,
containers), add `--no-browser` to either auth command:

```sh
oura auth login --no-browser
oura auth setup --no-browser
```

The CLI prints the authorize URL; you approve it in a browser on any machine, then paste the
full redirect URL back into the terminal. The same `state` CSRF check applies, and a mismatch
aborts. An SSH session is auto-detected and `--no-browser` is suggested for you.

## Environment overrides

Read once at startup; empty or whitespace-only values are ignored (treated as unset).

### `OURA_ACCESS_TOKEN`

A raw OAuth access token used by the data commands and `oura mcp`, bypassing the token store
entirely — no login, no refresh. It takes precedence over any stored login. When Oura rejects
it (expired/invalid), the command exits `4` and tells you to export a fresh one; the MCP server
returns a structured tool error. This is the intended path for CI and one-shot agents.

```sh
OURA_ACCESS_TOKEN=<token> oura sleep --json
OURA_ACCESS_TOKEN=<token> oura mcp
```

The `oura auth` account commands (`status` / `token` / `refresh` / `logout`) still operate on
the store, not this variable.

### `OURA_API_BASE_URL`

Points the client at an alternate Oura host, a proxy, or a mock (default
`https://api.ouraring.com`). A trailing slash is trimmed. Over plain `http://` the Bearer token
is sent in cleartext — use that only for a trusted local proxy or mock, never a real network
path.

```sh
OURA_API_BASE_URL=http://localhost:8080 oura personal-info
```

### `NO_COLOR`

Disables ANSI color (same as the `--no-color` flag), following
[no-color.org](https://no-color.org).

## Putting it together in CI

A typical CI step injects a short-lived token and reads data as JSON:

```sh
OURA_ACCESS_TOKEN="$OURA_TOKEN" oura readiness --json > readiness.json
status=$?
if [ "$status" -eq 4 ]; then
  echo "token rejected — export a fresh OURA_ACCESS_TOKEN" >&2
fi
```
