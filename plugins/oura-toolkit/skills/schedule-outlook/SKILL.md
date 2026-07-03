---
name: schedule-outlook
description: >-
  Score the user's upcoming weeks against their own history: schedule load ahead, the
  most similar past weeks (historical analogs) and the health outcomes that followed
  them, plus current capacity. Use when the user asks how their coming weeks/month/
  season look, whether a planned stretch will burn them out, what their fall/quarter
  will do to their health, or how much more they can take on. Requires the oura MCP
  server (this plugin) and an imported local store (`oura sync` + `oura import
  calendar`).
---

# Schedule outlook

Turn the user's upcoming calendar into a health-aware plan, anchored ONLY in their own
history. The engine computes; you narrate and suggest.

## Fetch

Call in parallel:
- `get_capacity` — this week's capacity (0–100%), its component attribution, and the
  current week's analogs.
- `get_upcoming_load` — the coming weeks' schedule load (pass `weeks` to match the
  user's horizon: "this month" → 5, "the fall/quarter" → 13; default 4).
- For each upcoming week that stands out (heaviest load, or the user asked about it),
  `find_analog_weeks` with `week` set to any date in it.
- Add `get_day_context` only when you need day-level texture for a specific week.

On a "not enough history" error: relay the tool's own remediation (it names the exact
`oura sync` / `oura import` commands), explain that the engine refuses to guess from
thin history, and stop — do not improvise an outlook without it.

## Analyze

- **Rank the horizon**: which upcoming weeks are heaviest, and on what (meeting hours
  vs evening events vs tracked hours). A week with empty features means NO imported
  context — say so; never treat it as a free week.
- **Anchor in analogs**: for the standout weeks, report what actually followed the most
  similar past weeks — readiness/sleep during and in the TWO WEEKS AFTER (the lag is
  where schedule damage usually lands). Compare against the baseline the report
  carries.
- **Capacity now**: lead with the percentage and band, then the component attribution
  (recovery debt / week load / analog risk) in plain words.

## Deliver

- Lead with the one-glance answer: capacity now, and the single riskiest upcoming week.
- For each flagged week: why it's heavy, what followed its analogs, and ONE concrete,
  actionable protective move (a lighter following week, guarded recovery days, moving
  evening events) — grounded in what the analogs show helped, when the data shows it.
- Phrase everything as the user's own history: "in your history, weeks like Oct 12
  were followed by…". NEVER present a prediction or a causal claim; this is n=1
  observational data.
- Keep it under ~300 words unless asked to go deeper. Numbers only where they carry
  the point.
