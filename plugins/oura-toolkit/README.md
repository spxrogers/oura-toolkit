# oura-toolkit plugin

Your Oura Ring data in Claude: a local MCP server exposing eight read-only tools
(daily sleep, readiness, activity, stress, heart rate, sessions, workouts, personal
info) plus two skills — `/morning-checkin` and `/wellness-report`.

## Install

```
/plugin marketplace add spxrogers/oura-toolkit
/plugin install oura-toolkit@oura-toolkit
```

The MCP server runs locally via `npx -y oura-toolkit@<version> mcp`: the server itself
talks only to the Oura API, and your credentials are only ever sent to Oura — never to
Anthropic or any third party. Tool results are then read by Claude like any other tool
output — that's what powers the skills.

## One-time authentication (bring your own Oura app)

Oura personal access tokens are deprecated; the toolkit uses OAuth with YOUR OWN Oura
application, so your credentials never touch anyone else's infrastructure:

```
npx -y oura-toolkit auth setup   # guided app registration + login
```

`auth setup` opens Oura's app-registration page, tells you exactly what to paste,
collects your client id/secret with hidden input, and chains into the browser login.
Tokens are stored at `~/.config/oura-toolkit/` (Windows: `%LOCALAPPDATA%\oura-toolkit\`)
and refresh automatically.

If a tool call reports "not authenticated", run `npx -y oura-toolkit auth login`.

## Skills

- **/morning-checkin** — last night's sleep + today's readiness + one actionable
  takeaway.
- **/wellness-report** — multi-day trends across sleep/readiness/activity/stress with
  cross-signal insights.

Both are also invoked automatically when you ask Claude the matching questions ("how
did I sleep?", "how was my week?").
