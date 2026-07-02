---
name: morning-checkin
description: >-
  A quick morning briefing from the user's Oura Ring: last night's sleep, today's
  readiness, and one actionable takeaway. Use when the user asks how they slept, how
  recovered they are, whether to train hard today, or for a morning check-in / daily
  briefing. Requires the oura MCP server (this plugin) and a completed `oura auth login`.
---

# Morning check-in

Produce a short, personal morning briefing from the Oura MCP tools. Keep it tight: the
user wants a glanceable answer, not a data dump.

## Fetch

1. `get_daily_sleep` with `start: "yesterday", end: "today"` — last night's sleep usually
   lands on today's date; fetching both days covers timezone/day-boundary drift. Use the
   most recent document.
2. `get_daily_readiness` with `start: "today", end: "today"` (fall back to yesterday if
   today's document isn't in yet — rings sync lazily).
3. Optionally `get_daily_stress` for yesterday when the user asks about stress or the
   readiness contributors point at recovery debt.

Call tools in parallel where possible. If a tool returns an authentication error, don't
retry: tell the user to run `oura auth login` in a terminal (or `oura auth setup` first
if they've never registered an Oura OAuth app), then stop.

## Interpret

- Scores are 1–100. Rough bands: 85+ excellent, 70–84 good, 60–69 fair, <60 pay
  attention. Never invent a score that isn't in the data.
- Sleep contributors (`deep_sleep`, `rem_sleep`, `efficiency`, `latency`, `restfulness`,
  `timing`, `total_sleep`) are also 1–100 — name the ONE weakest contributor rather than
  listing all seven.
- Readiness `temperature_deviation` is in °C; a sustained rise can signal strain or
  illness — mention it only when notably nonzero (≳0.3 °C).
- An empty result is not an error: the ring likely hasn't synced. Say so.

## Answer shape

Three beats, conversational, no headers:

1. **The verdict** — one sentence combining sleep + readiness ("Solid night — 82 sleep,
   88 readiness; you're good to push today.").
2. **The why** — the one contributor or signal that most explains the verdict.
3. **The action** — one concrete suggestion tied to the data (training intensity,
   earlier bedtime, hydration on a temperature bump). Skip generic wellness advice.

Mention specific numbers sparingly (two or three, not ten). If both sleep and readiness
documents are missing, say the ring hasn't synced yet and offer to check again later —
do not fabricate a briefing.
