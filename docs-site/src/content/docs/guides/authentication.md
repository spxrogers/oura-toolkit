---
title: Authentication
description: How oura-toolkit authenticates with Oura — bring-your-own OAuth app, the login flow, the token store, and headless options.
sidebar:
  order: 2
---

Oura's API uses **OAuth 2.0 with a confidential client** (Authorization Code flow only —
personal access tokens were deprecated in December 2025). Because a client secret can't be
safely embedded in a distributed binary, `oura-toolkit` is **bring-your-own-credentials**:
you register your own free Oura application and the credentials stay on your machine, sent
only to Oura.

## Guided setup

```sh
oura auth setup
```

runs the whole thing interactively:

1. Opens <https://cloud.ouraring.com/oauth/applications> in your browser and prints the exact
   values to enter on Oura's form:
   - **Application name**: anything (e.g. `oura-toolkit`)
   - **Redirect URI**: `http://localhost:8788/callback` — must match exactly
   - **Scopes**: `personal daily heartrate workout tag session spo2Daily`
2. Prompts for the app's **client id** and **client secret** in the terminal (the secret with
   hidden input — it never leaves your machine).
3. Chains straight into `oura auth login`: your browser opens Oura's consent page, a local
   listener on port 8788 catches the callback, and your tokens are stored.

Already registered? Just run `oura auth login`. If port 8788 is taken, pass `--port <n>` and
register the matching redirect URI on Oura's form (Oura requires an exact match).

## Where credentials live

Tokens and credentials are stored under a fixed, invocation-independent path with owner-only
file permissions:

- **macOS / Linux**: `~/.config/oura-toolkit/`
- **Windows**: `%LOCALAPPDATA%\oura-toolkit\` (Local, not Roaming — under your profile's
  private ACLs)

Two records are kept side by side: the client credentials and the OAuth tokens, so a failed
login never loses the client secret you pasted. Refresh tokens rotate on every refresh and are
persisted automatically, so once you're logged in it stays that way.

## Managing stored state

The account commands are non-interactive and scriptable:

```sh
oura auth status     # what's stored: client id, scopes, token expiry (exit 4 = run setup/login)
oura auth token      # print a fresh access token — for curl and scripts
oura auth refresh    # force a token refresh now (persists the rotated refresh token)
oura auth logout     # remove tokens; --all also removes the app credentials
```

`oura auth status` prints the client id and scopes but **never** the client secret. `oura auth
token` prints only the token and a trailing newline, so it composes cleanly:

```sh
curl -H "Authorization: Bearer $(oura auth token)" https://api.ouraring.com/v2/usercollection/personal_info
```

## Headless, SSH, and containers

No local browser, or a remote box the OAuth callback can't reach?

- **`oura auth login --no-browser`** (also `oura auth setup --no-browser`) — the CLI prints the
  authorize URL, you approve it in a browser on any machine, then paste the full redirect URL
  back into the terminal. The same `state` CSRF check applies. An SSH session is auto-detected
  and `--no-browser` is suggested for you.
- **`OURA_ACCESS_TOKEN`** — inject a raw OAuth access token that bypasses the store entirely
  (no login, no refresh). See [Headless & CI](/guides/headless-ci/) for the environment
  overrides in full.
