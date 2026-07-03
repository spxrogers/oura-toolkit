# Feasibility: Apple HealthKit ingestion & the multi-source future

**Status: PROPOSAL (feasibility study, 2026-07-03). Nothing in this document describes
shipped behavior.** It analyzes how Apple Watch / HealthKit data could enter this
toolkit, how the existing spec-driven architecture does and does not generalize, and
what it would take for oura-toolkit to become an aggregation point for other providers
(Garmin, Fitbit, Whoop, …). Decisions here are recommendations awaiting owner sign-off;
none of CLAUDE.md's locked decisions are changed by this file.

---

## 1. The framework we'd be extending (current state)

Everything in this repo hangs off one design spine:

```
spec/openapi.json  ──overlays──►  generated clients (Rust via progenitor; TS/Py/Go/Java/C# via openapi-generator)
        │
        ├──build-time read──►  oura-toolkit-auth (OAuth URLs + scopes as constants)
        └──build-time read──►  MCP tool descriptions (cli build.rs)

cli/oura-toolkit-cli
        commands::fetch_*  (returns GENERATED models; auth + 401-retry + pagination)
              ├──► CLI rendering (table / TSV / --json)
              └──► MCP tools (same models serialized to JSON)
```

Load-bearing properties, verbatim from CLAUDE.md, that any new source must be measured
against:

1. **Spec is the source of truth** — the vendored OpenAPI file drives the client, the
   MCP tools, and the SDKs.
2. **Never hand-write transport** — HTTP clients are generated, not written.
3. **One auth layer, one data plane, two presentations** — `fetch_*` feeds both the CLI
   and MCP.
4. **BYO credentials, local-first** — secrets and tokens never leave the machine;
   the README promises "your credentials are only ever sent to Oura."
5. **Guarantee = test** — anything documented gets an enforcing, break-verified test.

The feasibility question is really: *which of these properties survive a provider that
has no API?*

---

## 2. Apple HealthKit: there are no endpoints

This is the single most important finding, so it comes first.

**HealthKit is not a cloud service.** There is no REST API, no OAuth server, no
OpenAPI spec, and no `api.apple.com/health`. HealthKit is an **on-device framework**:
Apple Watch data syncs into an encrypted store on the paired iPhone, and the only
sanctioned programmatic reader is a **native iOS/watchOS app** with HealthKit
entitlements, running on that phone, after per-data-type user consent. iCloud's Health
sync is end-to-end encrypted with no server-side API. Apple deliberately has no
web-accessible health data surface (unlike Oura, Fitbit, Whoop, Withings, Polar).

So the request "list out the endpoints to integrate Apple HealthKit" has a precise
answer: **there are none to list.** What exists instead is a set of on-device *data
types* (§4) and three realistic *transport strategies* for getting them onto the
machine where `oura` runs (§3).

Consequences for the framework:

- Property 1 (spec-driven codegen) **does not apply** — there is no spec to vendor.
  A HealthKit integration is an *ingestion parser*, not a generated client.
- Property 2 (no hand-written transport) **is not violated** — there is no transport.
  A file parser is hand-written domain code, the same category as the auth companion.
- Properties 3–5 **apply fully** and are where the existing architecture pays off:
  a `fetch_*`-shaped accessor over a local store slots straight into the existing
  CLI/MCP dual-presentation pattern.

---

## 3. Transport strategies, ranked by feasibility

### Option A — Apple Health export file ingestion (RECOMMENDED first step)

Every iPhone can produce a full export today: Health app → profile → **"Export All
Health Data"** → `export.zip` containing `export.xml` (every sample ever recorded,
with `sourceName`, device, and metadata attributes) plus workout GPX routes and ECG
CSVs. No developer account, no companion app, no third party.

- **Shape**: `oura import apple-health ~/Downloads/export.zip` → parse → normalize →
  persist to a local store → existing report/MCP surfaces read from it.
- **Effort**: small. One streaming-XML parser (exports from long-time Apple Watch
  wearers reach multiple GB — `quick-xml` streaming, never DOM), one local store, one
  set of fixtures. Roughly the size of the auth companion; 1–2 weeks of focused work
  including the test regime §7 demands.
- **Limitations**: manual and point-in-time. The user re-exports to refresh (the export
  is always full-history, so re-import is idempotent upsert, not append). Perfectly
  adequate for a weekly `wellness-report`; wrong tool for a live morning check-in.
- **Risk**: the export format is undocumented but has been stable for ~a decade and is
  parsed by a healthy open-source ecosystem (e.g. `healthkit-to-sqlite`). Pin fixtures;
  treat format drift like spec drift (a failing tripwire test, not a surprise).

### Option B — bridge-app ingestion (semi-automated, optional later)

Third-party iOS apps (the established one is **Health Auto Export — JSON+CSV**) read
HealthKit with the user's consent and push JSON/CSV on a schedule — to a folder
(iCloud Drive/Dropbox) or to a user-supplied REST endpoint on the LAN.

- **Shape**: accept that JSON as a second input format for the same importer
  (`oura import apple-health --format hae ...`), or (much later) a
  `oura import listen` mode receiving pushes.
- **Effort**: incremental over Option A (a second deserializer into the same normalized
  model). The listener mode is more (a local HTTP receiver + pairing story) and is
  explicitly out of scope for a first pass.
- **Caveats**: depends on a paid third-party app we don't control; data still never
  leaves the user's machines, so the local-first posture holds. Never make it the
  primary path — treat it as an accelerant for users who already run such an app.

### Option C — first-party companion iOS app (the "someday" endgame)

A native Swift app using `HKObserverQuery` + background delivery, syncing continuously
to the toolkit's store (LAN push, file share, or a paired-device protocol).

- **Effort**: a different project, not a feature. Apple Developer Program ($99/yr),
  Swift/SwiftUI + HealthKit expertise, App Store review (health data gets extra
  scrutiny), a device sync protocol, and an ongoing maintenance treadmill across iOS
  releases. Months, not weeks, and it drags the repo outside its Rust/codegen
  competency.
- **Verdict**: technically feasible, strategically premature. Only worth revisiting if
  Options A/B prove demand for continuous Apple data. Note CLAUDE.md's spirit ("do NOT
  build browser-automation onboarding") — heavyweight side-channels need to earn their
  keep.

**Non-options, for the record**: scraping iCloud (E2E-encrypted, no API); Shortcuts
automations (too lossy/fragile for structured bulk export); any cloud aggregator SaaS
(Terra/Spike/ROOK — commercial middlemen that would break the "your data never touches
a third party" promise).

---

## 4. Data mapping: Oura surface ↔ HealthKit types

What we can actually fill in, per existing command/tool. HealthKit identifiers are the
`HKQuantityType`/`HKCategoryType` names an importer would encounter in `export.xml`.

| Oura endpoint (today's surface) | CLI / MCP | Closest HealthKit data | Fidelity notes |
|---|---|---|---|
| `daily_sleep` | `oura sleep` / `get_daily_sleep` | `SleepAnalysis` (asleepCore/Deep/REM/awake stages, watchOS-recorded) | Stages map well; **Oura's 0–100 score has no Apple equivalent** — never synthesize one. |
| `daily_readiness` | `oura readiness` / `get_daily_readiness` | — (no Apple readiness score). Raw inputs exist: `AppleSleepingWristTemperature`, `RestingHeartRate`, `HeartRateVariabilitySDNN` | Report Apple *vitals* alongside Oura *readiness*; don't fabricate a score. |
| `daily_activity` | `oura activity` / `get_daily_activity` | `StepCount`, `ActiveEnergyBurned`, `BasalEnergyBurned`, `AppleExerciseTime`, `AppleStandTime`, `DistanceWalkingRunning` | Strong overlap; Apple ring metrics ≈ Oura activity contributors. |
| `daily_stress` | `oura stress` / `get_daily_stress` | — (no stress metric). Nearest proxies: HRV, `MindfulSession` | Gap. Surface as "not available from Apple Watch". |
| `heartrate` | `oura heartrate` / `get_heart_rate` | `HeartRate` (continuous), `RestingHeartRate`, `WalkingHeartRateAverage`, `HeartRateRecoveryOneMinute` | Best-in-class overlap; Watch sampling is denser than the ring's. |
| `session` | `oura sessions` / `get_sessions` | `MindfulSession` | Direct analog for meditation/breathing. |
| `workout` | `oura workouts` / `get_workouts` | `HKWorkout` (+ GPX routes in the export) | Direct analog; Apple has richer per-workout series. |
| `personal_info` | `oura personal-info` / `get_personal_info` | export `Me` record (DOB, sex, blood type) | Trivial. |
| `daily_spo2` (spec'd, not yet surfaced) | — | `OxygenSaturation` | Direct analog if/when surfaced. |
| `vO2_max` (spec'd, not yet surfaced) | — | `VO2Max` ("Cardio Fitness") | Direct analog. |
| `daily_cardiovascular_age`, `daily_resilience`, `sleep_time`, `rest_mode_period`, `ring_*`, tags | — | — | Oura-proprietary or device-specific; no mapping. |

**The one mapping that will mislead people if done naively**: HRV. Oura reports
**rMSSD**; Apple reports **SDNN** (`HeartRateVariabilitySDNN`). They are different
statistics over different sampling windows and are **not comparable numbers**. Any
merged view must label the metric per source and never chart them on one axis. The
same class of caveat applies to sleep staging (different algorithms) and calories
(different energy models): cross-source data is for *side-by-side context*, not
averaging.

---

## 5. Proposed integration shape (how it fits this repo)

The design that preserves the architecture's spine with the least distortion:

1. **A normalized domain layer, new crate `oura-toolkit-health-model`** (name TBD):
   plain Rust types for the cross-provider concepts — sleep night (stages + duration),
   HR sample, workout, daily activity, vitals. Provider adapters map *into* it:
   the Oura adapter from the progenitor-generated models (generated code remains the
   source for Oura — nothing regresses to hand-written transport), the Apple adapter
   from the export parser. Scores stay provider-tagged and optional.
2. **An importer + local store** for file-based providers:
   `oura import apple-health <export.zip>`. Store under the existing config-dir
   convention (`$XDG_DATA_HOME/oura-toolkit/` — note *data*, not *config*; imported
   health data is not a credential) as SQLite or partitioned JSON — decide at
   implementation. Same 0600/atomic-write discipline as the token store.
3. **Source-aware surfaces**: existing commands gain `--source oura|apple|all`
   (default `oura` — zero behavior change until opted in), or Apple data ships first
   as read-only siblings (`oura sleep --source apple`). MCP grows a small number of
   curated tools (e.g. `get_apple_sleep`, `get_apple_workouts`) or a `source` param on
   existing ones — decide with the same curation care as the original eight. The
   `wellness-report` skill then gets its Apple Watch section by calling one more tool.
4. **Justfile discipline unchanged**: `just test` stays hermetic (checked-in fixture
   exports, tempdir stores); a `just import-fixture` style recipe for local dev.

What this deliberately does **not** do: touch the spec pipeline, the generated
clients, the auth companion, or the plugin count. HealthKit ingestion is additive.

### Where the constitution needs an owner decision (flagged, not relitigated)

- **Layout**: `sdks/<lang>/` holds spec-generated clients + auth companions. A
  hand-written parser crate fits neither `sdks/` nor `cli/`. Recommendation: a new
  top-level `sources/` (or workspace crates under `sdks/rust/` if you'd rather not add
  a root dir). Needs a CLAUDE.md layout amendment either way.
- **Branding**: the binary is `oura` and the README's promise is Oura-specific. `oura
  import apple-health` is defensible ("your Oura toolkit can *also* read your Apple
  export"), but the full aggregation vision (§6) eventually strains a ring-branded
  name. The NAMING section is locked; if multi-source becomes the strategy rather than
  a feature, an umbrella rename is a real (breaking, pre-1.0-only) decision to make
  deliberately — not something any PR should back into.
- **README trust language**: "your credentials are only ever sent to Oura" stays true
  (Apple ingestion involves no credentials at all), but the sentence should be
  re-audited in the same PR that lands the importer (DOCS STAY TRUE TO THE CODE rule 1).

---

## 6. The bigger vision: one toolkit, many providers

Feasibility snapshot for the providers worth considering, measured against this repo's
model (BYO credentials, published spec, OAuth2, local-first). Verify each provider's
current terms at implementation time — this table is a 2026-07 snapshot and API
programs drift:

| Provider | API reality | Fits the BYO/spec-driven model? | Feasibility |
|---|---|---|---|
| **Apple HealthKit** | No cloud API; on-device only (§2) | No spec, no OAuth — importer path (§3) | **High** via export file; the odd one out architecturally |
| **Fitbit** | Open Web API, OAuth2, self-serve app registration, Swagger/OpenAPI published | **Yes — closest analog to Oura.** Same vendored-spec → generated-client → auth-companion pipeline | **High** — the natural second *cloud* provider |
| **Whoop** | Open developer platform, OAuth2, publishes an OpenAPI spec | Yes | **High** |
| **Withings / Polar (AccessLink)** | OAuth2, self-serve registration, documented REST | Yes (spec quality varies) | **Medium-high** |
| **Garmin** | Health API exists but requires **approved commercial developer access** — no hobbyist BYO-credentials path; unofficial `garminconnect` libraries scrape the consumer login (ToS-fragile) | **Poorly.** Official path gatekept; unofficial path is the kind of liability this repo avoids | **Low-medium** — park it; a partial escape hatch is Garmin's manual data export, which could ride the same importer framework as Apple |
| **Google/Android (Health Connect)** | On-device store (the HealthKit of Android); Google Fit REST API shut down | Same importer-class problem as Apple | Later, same pattern as Apple |

The strategic read: the **two integration archetypes** are (1) *cloud OAuth providers
with specs* — Oura today; Fitbit/Whoop generalize the existing pipeline almost
mechanically (vendor spec → overlay → generate → hand-write a thin auth companion →
per-provider token store subdir) — and (2) *on-device/file providers* — Apple, Health
Connect, Garmin-export — which need the importer + normalized-store machinery of §5.
Build each archetype once, and every subsequent provider is incremental. HealthKit is
the right first move on archetype 2 precisely because it's the one users ask for and
it forces the normalized model into existence; Fitbit or Whoop would be the right
proof of archetype 2's cloud sibling.

---

## 7. Testing & verification implications (non-negotiable per CLAUDE.md)

- **Hermetic fixtures**: a synthetic, checked-in `export.zip` (small, hand-built,
  covering every mapped type + a malformed-entry case). No real personal exports in
  the repo, ever.
- **Guarantee = test**: import idempotency (re-import ≠ duplicates), store perms/
  atomicity (reuse the token-store test patterns), source labeling (an "Apple HRV
  never rendered as Oura HRV" tripwire — this is the §4 comparability hazard turned
  into CI), streaming-parse memory bounds if we claim them.
- **Docs tripwires**: the moment `import` and `--source` are documented, their
  enumerable claims (command names, store path, supported formats) get pinned tests
  like the existing README tripwires (#45).
- **Break-verified**: per rule 3, every guard above is neutered-and-watched-failing
  before it counts.

---

## 8. Recommendation & phasing

**Feasible: yes — with the crucial reframe that HealthKit integration is an *importer*,
not an *API client*.** The existing framework's fetch/render split, MCP curation
pattern, store discipline, and testing law all transfer; only the spec-codegen leg
doesn't, because Apple ships no spec to generate from.

| Phase | Deliverable | Size |
|---|---|---|
| 0 | Owner decisions: layout home for source crates, `--source` UX, store format, branding stance (§5) | discussion |
| 1 | Normalized model crate + Apple export importer + `oura import apple-health` + `--source apple` on sleep/activity/heartrate/workouts | ~1–2 weeks |
| 2 | MCP tools for Apple data + `wellness-report` / `morning-checkin` skill updates ("Apple Watch section") | days |
| 3 | Health Auto Export JSON as a second input format (optional) | days |
| 4 | Second cloud provider (Fitbit or Whoop) reusing the full spec→codegen→auth pipeline; per-provider token store | ~2–3 weeks |
| 5 | Companion iOS app | not now (§3C) |

Per CLAUDE.md's process note, Phase 1 would begin with a concrete file-by-file plan
for explicit confirmation — this document is the feasibility groundwork for that
conversation, not a license to start building.
