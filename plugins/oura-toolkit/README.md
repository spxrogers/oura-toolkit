# oura-toolkit plugin

Your Oura Ring data in Claude: a local MCP server exposing fourteen curated tools —
eight over your Oura data (daily sleep, readiness, activity, stress, heart rate,
sessions, workouts, personal info) and six over the toolkit's local health+schedule
store (capacity, analog weeks, upcoming load, day context, habit rates, and habit
logging — the one writing tool, local-only) — plus three skills:
`/morning-checkin`, `/wellness-report`, and `/schedule-outlook`.

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
- **/schedule-outlook** — the weeks ahead scored against your own history: upcoming
  schedule load, the most similar past weeks and what followed them, and this week's
  capacity. Needs imported context — run `npx -y oura-toolkit sync` and
  `npx -y oura-toolkit import calendar <file.ics>` first (see the repo README's
  "Local health store" section).

All are also invoked automatically when you ask Claude the matching questions ("how
did I sleep?", "how was my week?", "what does my fall look like?").
