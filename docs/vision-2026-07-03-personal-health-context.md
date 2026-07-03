# Vision: the personal solution — data safety, onboarding, and the life↔health engine

**Status: PROPOSAL (companion to
[`feasibility-2026-07-03-apple-healthkit-multi-source.md`](feasibility-2026-07-03-apple-healthkit-multi-source.md),
2026-07-03). Nothing here is shipped behavior.** That document answered *"can we
ingest Apple Watch data?"* This one answers the three questions that follow:

1. What does the **personal solution** look like day to day — and once health data is
   exported out of Apple's encrypted garden, **how does it stay safe**?
2. What makes this **effortless for a brand-new user**, and what would a
   **first-party iOS app** actually look like in practice?
3. How does the toolkit become the endpoint for **life-context data** (schedules,
   calendars, travel) so it can find **historical analogs** — "the last time your
   fall looked like this, here's what it did to your body" — and help **plan future
   schedules** to avoid the health dips?

---

## 1. The personal solution, and keeping the data safe

### What daily use looks like (steady state)

```
Continuous, automatic:   Oura API ──► oura CLI / MCP (already works today)
Weekly-ish, one command: iPhone Health export ──► oura import apple-health export.zip
Monthly-ish:             calendar export (.ics) ──► oura import calendar work.ics
Anytime:                 "how am I trending?" ──► Claude via the MCP server / skills
```

Everything lands in one **local store** on your machine. Nothing runs in a cloud;
there is no account, no server of ours, no telemetry. The system is useful the day
the importer ships and gets *more* useful every time an import adds history.

### The threat model, honestly

Inside Apple Health the data is encrypted at rest and end-to-end encrypted in iCloud.
The moment you tap "Export All Health Data" you hold a **plaintext zip** — that is
the real safety boundary crossing, and it happens on Apple's side, not ours. So the
job is: minimize how long plaintext exists outside protected locations, and make the
toolkit's copy at least as protected as the rest of your digital life.

| Threat | Exposure | Mitigation (proposed as *shipped behavior*, not user homework) |
|---|---|---|
| Export zip lingering in Downloads / iCloud Drive / AirDrop inbox | **Highest** — synced folders replicate it to other devices and backups | Importer prints a post-import reminder and offers `--remove-source` to delete the zip after a verified import |
| Store lands in a cloud-synced folder | High — silent replication | Store path is **fixed** (`$XDG_DATA_HOME/oura-toolkit/`, the data-dir sibling of the locked config-dir convention), never user-relocatable into Dropbox/iCloud; importer **warns if the resolved path is inside a known sync root** |
| Another user / process on the machine reads the store | Medium | Inherit the token store's proven discipline verbatim: `0700` dir, `0600` files, atomic writes — all already implemented and break-verified in `oura-toolkit-auth`'s store module; the health store reuses the same primitives |
| Stolen laptop | Medium | Full-disk encryption (FileVault/BitLocker) is the correct layer; docs say so plainly. Optional **at-rest encryption of the store** (age/SQLCipher, key in the OS keychain) is a Phase-2 hardening, same tier as the existing DPAPI/keyring issue (#26) for tokens |
| Data leaving the machine via the AI assistant | **Deliberate, not accidental** | Be explicit in docs: MCP tool results are read by whatever assistant you connect — that's the product. Tools return **aggregates and windows you asked for**, never a bulk dump of the store; no tool exposes "give me everything" |

Two principles worth writing into the eventual README section:

- **Local-first means every egress is a choice you make**, and there are exactly two:
  requests to the providers you authenticated (Oura today), and MCP tool results to
  the assistant you connected. Nothing else, ever, enforced by the same
  guarantee-equals-test law as the rest of the repo (a "no other hosts" tripwire is
  mechanizable with a network-recording test double).
- **The store is not a backup.** Source of truth stays in Apple Health / Oura's
  cloud; the local store is a re-buildable cache of imports. Losing it costs you a
  re-import, which also means at-rest encryption can be added later without a
  migration story.

### For John specifically, starting now (before any code ships)

The safe habit-loop with today's manual export: export on the phone → AirDrop to the
Mac (peer-to-peer, doesn't transit iCloud) → import → delete the zip. One command and
one deletion, weekly. Boring on purpose.

---

## 2. The new-user story, and the first-party app in practice

### Streamlined onboarding without any app (Phase 1)

The repo already has the pattern: `oura auth setup` walks a non-technical user
through an OAuth app registration in ~2 minutes by *printing exactly what to do*.
Apple import gets the same treatment — the wizard IS the docs:

```
$ oura import apple-health
No export file given. To create one (takes ~2 min on your iPhone):

  1. Open the Health app → tap your picture (top right)
  2. Scroll down → "Export All Health Data" → Export
  3. AirDrop the export.zip to this computer

Then run:  oura import apple-health ~/Downloads/export.zip
```

And an umbrella `oura setup` that detects state and offers the next step ("Oura:
connected ✓ · Apple Health: no data imported yet → want the walkthrough?"). Every
path stays npx-runnable — `npx -y oura-toolkit setup` remains the entire install
story, which is what "pick it up and roll" means in practice: **zero
infrastructure, two guided minutes per data source, standard files only** (.zip
export, .ics calendar). Nothing about John's setup is bespoke — a second user does
the identical steps against their own accounts and devices.

The honest UX ceiling of Phase 1: freshness. Data is as current as the last export.
For weekly trend reports that's fine; for a daily morning check-in with Watch data
it isn't — which is exactly the gap the first-party app closes.

### The first-party iOS app, concretely

What it would actually be:

- **A small SwiftUI app** (working name: the toolkit's sync companion) whose entire
  UI is: a list of HealthKit data-type toggles (sleep, heart rate, HRV, workouts,
  activity, mindfulness, SpO2, wrist temperature), a pairing screen, and a "last
  synced" status line. No charts, no accounts, no cloud — the phone-side twin of the
  CLI's philosophy.
- **How data flows**: on first launch it requests HealthKit read permission per
  toggled type (Apple's consent sheet — the user sees exactly what's shared). It
  registers `HKObserverQuery` + background delivery so iOS wakes it when new samples
  land. New data is packaged as JSON deltas.
- **How it reaches your machine** — the key design decision, three candidate
  channels:
  1. **Local-network push (preferred)**: `oura pair` shows a QR code; the app scans
     it and thereafter pushes deltas to the CLI's listener over the LAN (Bonjour
     discovery, key exchange at pairing, mutual auth). Pure peer-to-peer, never
     transits any cloud. Cost: both devices must occasionally share a network, and
     we own a small sync protocol.
  2. **iCloud Drive drop-folder**: the app writes delta files into its app
     container; the Mac importer watches the folder. Simplest to build and works
     apart, but data transits Apple's cloud (E2E-encrypted only if the user enables
     Advanced Data Protection) — a softer privacy stance that must be labeled as
     such.
  3. **Share-sheet manual export**: the app is just a nicer, incremental
     "Export All Health Data". Fallback, not the design.
- **In practice, for the user**: install from the App Store → toggle data types →
  scan the QR from `oura pair` → done forever. The morning-checkin skill now has
  last night's Watch sleep without anyone touching an export file.
- **The real costs, stated plainly**: Apple Developer Program ($99/yr) and App Store
  review (health apps get extra scrutiny); Swift/HealthKit is a different competency
  from this repo's Rust/codegen core; background delivery is famously fiddly across
  iOS versions; and it's an ongoing maintenance commitment, not a one-time build.
  Months of work. Open-sourcing it in-repo keeps the trust story ("read the code
  that touches your health data") but doesn't reduce the effort.
- **Sequencing verdict** (unchanged from the feasibility doc, now with the reason
  spelled out): ship the export importer first because *the analysis layer doesn't
  care how data arrived*. Every hour spent on §3 pays off for both transports; the
  app only upgrades freshness. Build it when weekly-export friction is the proven
  top complaint — and Phase 1's store/normalized model is exactly what the app
  would sync into, so nothing is thrown away.

---

## 3. The life↔health engine: historical analogs and forward planning

This is the reframe that makes the toolkit *yours* rather than another dashboard:
health data alone answers "how am I?", but pairing it with **what your life looked
like at the time** answers "**why** — and what will next season do to me?"

### 3.1 Context imports (the other half of the data)

Same importer archetype as Apple Health — standard files, local parse, no cloud:

- **Calendar (.ics)** — the highest-value context source and a genuinely open
  format. Google Calendar, Apple Calendar, and Outlook all export it, including
  years of history. `oura import calendar work.ics --tag work` (multiple calendars,
  tagged). Future events come along for free, which §3.4 depends on.
- **Derived, not raw**: what the store keeps per day is a small **context feature
  vector**, not your meeting titles — `meeting_hours`, `event_count`,
  `first_event_start` / `last_event_end`, `evening_events`, `travel_day` (detected
  from event locations/timezone hints), `weekend_worked`. Privacy falls out of the
  design: the analysis layer never needs event *content*, so by default it never
  stores it. (An opt-in flag can retain titles for LLM color; default off.)
- Later, same pattern: travel history (TripIt/airline .ics feeds), manual tags the
  CLI already has an Oura analog for, journals. Anything that can become a per-day
  number or flag can join.

### 3.2 The day-grain store

One SQLite database, two families of columns joined on the date:

```
day | sleep_score hrv_rmssd(oura) hrv_sdnn(apple) resting_hr readiness activity_score …
    | meeting_hours event_count evening_events travel_day tz_shift weekend_worked …
```

- Everything from §1/§2 (Oura API, Apple import, app sync) and §3.1 lands here.
- Provider-tagged metrics stay separate columns — the feasibility doc's HRV
  comparability hazard (rMSSD ≠ SDNN) is enforced by the schema itself.
- Rebuildable from sources at any time; SQLite gives the analysis layer real queries
  and gives power users an escape hatch (`sqlite3`, pandas, Datasette) for free.

### 3.3 Historical analogs — deterministic core, LLM narration

The division of labor follows the repo's existing law: **Rust computes, the
assistant narrates.** Statistics live in tested, deterministic code; the LLM never
does arithmetic, it interprets.

Deterministic layer (new analysis module + MCP tools over it):

- **Week signatures**: each historical week reduced to its context vector
  (meeting-hour total, evening-event count, travel days, tz shifts…), normalized
  against *your* baseline.
- **`find_analog_weeks(target)`**: nearest-neighbor match of a target week's
  signature against history — "the weeks most like this one" — returning, for each
  analog, the health outcomes **during and for 2 weeks after** (sleep score delta,
  HRV trend, readiness dip depth and recovery time). Lag matters: schedule damage
  often lands the *following* week, so outcome windows trail the context window.
- **`get_load_outcome_profile()`**: your personal dose-response curve — for each
  band of weekly load, the distribution of outcome deltas. This is where "how does
  what's externally going on affect me internally" becomes a number.
- Honest statistical posture, encoded in the tool descriptions themselves: this is
  n=1 observational data. The tools report *"in your history, weeks like this were
  followed by X"* — never causal claims, never predictions dressed as certainty,
  and they refuse (return a structured "insufficient history" result) below a
  minimum analog count rather than extrapolate from two data points.

### 3.4 Forward planning — the fall-schedule question

Because .ics imports include future events, the same machinery points forward:

- **`get_upcoming_load(weeks)`**: signatures for the next N scheduled weeks.
- The planning flow (a new skill, sibling of `wellness-report` — working name
  `/schedule-outlook`): fetch upcoming signatures → `find_analog_weeks` per week →
  the assistant synthesizes:

  > Week of Oct 12 is your densest of the fall (31 meeting-hours, 2 travel days).
  > Its closest analogs are the March conference week and last September's launch —
  > after both, readiness dropped 12–18 points and took 8–10 days to recover, with
  > sleep debt concentrated in the 3 nights after travel. Historically, analogs
  > where the following week stayed under 15 meeting-hours recovered in half the
  > time. The week of Oct 19 is currently at 24. Two options that change the
  > profile: …

- The assistant can go one step further where it has calendar *write* access (via
  whatever calendar MCP connector the user runs — not this toolkit's job): propose
  the specific protective moves. The toolkit stays read-only; recommendations are
  the human's to act on.
- The loop closes over time: fall arrives, the data records what actually happened,
  and next year's analogs are better. The system's value compounds with exactly the
  data you're already generating.

### 3.5 Why this scales beyond John

Nothing above is John-specific: standard input formats, per-user baselines computed
from each user's own history (not hardcoded thresholds), wizard onboarding per
source, and skills that read whatever sources exist ("no calendar data imported —
run `oura import calendar` to unlock schedule analysis" is a graceful degradation,
same pattern as the existing auth-required tool error). A new user's first hour:
`npx -y oura-toolkit setup`, connect Oura (2 min), import a Health export (2 min),
import a calendar (1 min) — then ask Claude how their spring compared to their
summer.

---

## 4. Revised phasing (supersedes the feasibility doc's table where they differ)

| Phase | Deliverable | Unlocks |
|---|---|---|
| 0 | Owner decisions: store format (SQLite recommended by §3.2), layout home for importer/analysis crates, `--source` UX, context-feature privacy default | — |
| 1 | Day-grain store + Apple export importer + `oura import apple-health` (+ `--remove-source`, sync-root warning) + `--source apple` on the overlapping commands | Watch data in reports; the safety posture of §1 |
| 2 | Calendar importer (.ics → context features) + analog analysis module + MCP tools (`find_analog_weeks`, `get_load_outcome_profile`, `get_upcoming_load`) + `/schedule-outlook` skill | The life↔health engine, §3 — including the fall-planning question |
| 3 | Hardening: optional at-rest store encryption (keychain-held key); Health Auto Export JSON as a second input | Deeper safety; semi-automated freshness |
| 4 | Second cloud provider (Fitbit or Whoop) via the existing spec→codegen→companion pipeline | Multi-provider breadth |
| 5 | First-party iOS sync app (§2), LAN-push channel first | Continuous freshness; daily morning check-ins with Watch data |

Phase 2 is deliberately ahead of new providers: John's stated goal is
context-correlation, and one health source (Oura) plus a calendar already answers
most of it. Apple data enriches the outcome side but isn't a prerequisite for the
engine.

Per CLAUDE.md's process note, each phase starts with a concrete file-by-file plan
for explicit confirmation before implementation.
