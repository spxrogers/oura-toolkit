# CLAUDE.md — oura-toolkit

This file captures the **final architecture decisions and hard constraints** for this
project so every future session inherits the same context. The architecture below is
**MOSTLY FINAL** — do not relitigate the language, transport, client-generation, auth
model, layout, or task-runner convention unless you have **high conviction** on a specific
item; in that case, present the issue + suggestion(s) and wait for an explicit answer
before acting.

---

## PROJECT

**oura-toolkit**: a Rust CLI for the Oura Ring API v2, plus a single Claude plugin and
auto-generated multi-language SDKs. Monorepo (cargo workspace + plugin marketplace).

The CLI is written in Rust, but **Rust is NOT privileged** — it's one language among the
SDKs. The CLI is an *app*, not an SDK (hence `cli/`). Every language sits under
`sdks/<lang>/` with the same client + companion shape.

API base: `https://api.ouraring.com/v2`

---

## NAMING (LOCKED)

Availability verified against npm, crates.io, PyPI — and, for Java/C# (added 2026-07-03,
owner-locked): Maven Central (zero "oura" artifacts) and NuGet (`OuraToolkit.*` free; the
unrelated `Oura` package does not collide). **Apply these names; do not "improve" them.** Rationale is included only where it prevents a well-meaning refactor.

| Layer | Name | Notes |
|---|---|---|
| Project / repo / brand | `oura-toolkit` | umbrella name everywhere |
| CLI command (binary) | `oura` | what the user types — NOT `oura-toolkit`, NOT `oura-cli` |
| npm scope | `@oura-toolkit` | verified free |
| npm packages | `@oura-toolkit/api`, `@oura-toolkit/auth` | function-named leaves under the scope |
| Rust crates | `oura-toolkit-*` | `oura-toolkit-api`, `oura-toolkit-auth`, `oura-toolkit-cli` |
| Python distribution | `oura-toolkit` | single dist; submodules below |
| Python modules | `oura_toolkit.api`, `oura_toolkit.auth` | NOT separate PyPI packages |
| Java (Maven Central) | `com.ouratoolkit:api`, `com.ouratoolkit:auth` | owner owns ouratoolkit.com; packages `com.ouratoolkit.api` etc. |
| C# (NuGet) | `OuraToolkit.Api`, `OuraToolkit.Auth` | namespaces match the package ids |
| Claude plugin | `oura-toolkit` | |

**Per-ecosystem namespacing** (do NOT mirror the npm layout everywhere):
- **npm** — scoped `@oura-toolkit/api`, `@oura-toolkit/auth`. Keep `api`; never rename to `sdk`
  (the whole scope IS the SDK). Model: `@octokit/core` + `@octokit/auth-token`.
- **Rust** — no namespaces → flat prefixed crates `oura-toolkit-{api,auth,cli}`. Hyphens in the
  crate name, underscores on import (`use oura_toolkit_api::…`). The short `oura-api` is taken on
  crates.io anyway.
- **Python** — single distribution `oura-toolkit` with submodules `oura_toolkit.api` /
  `oura_toolkit.auth`. No per-function micro-packages.
- **Java** — group `com.ouratoolkit` (domain-verified: the owner bought ouratoolkit.com),
  artifacts `api` / `auth` — the npm-scope model mapped onto Maven. One-time pre-publish
  prerequisite: verify the com.ouratoolkit namespace with Central (DNS TXT record).
- **C#** — `OuraToolkit.Api` / `OuraToolkit.Auth`, PascalCase dotted ids per .NET
  convention; assembly namespaces match the package ids.

**Binary ≠ crate name (the one Cargo gotcha):** the CLI crate is `oura-toolkit-cli` but installs a
binary named `oura` (`[[bin]] name = "oura"`). `cargo install oura-toolkit-cli` → `oura` on PATH.
Command surface reads clean: `oura auth login`, `oura sleep list`.

**Do NOT:** name any package/module `sdk`; suffix the binary `-cli`; publish Python sub-packages
per function; try to claim the taken names `oura` / `oura-api` / `oura-cli` (crates), `oura` (npm/
PyPI).

**Each crate's directory matches its crate name** — `sdks/rust/oura-toolkit-api`,
`sdks/rust/oura-toolkit-auth`, `cli/oura-toolkit-cli` — so nothing has to be "patched over" with a
diverging `[package] name` in Cargo.toml. (Dir length is not a feature; a dir/crate-name mismatch
is a maintenance overhead.) The token store lives at `$XDG_CONFIG_HOME/oura-toolkit/` (see AUTH).

Prior art (awareness only, do NOT copy): `github.com/terry-li-hm/oura-cli` — an unrelated read-only
personal CLI (crate `oura-cli`, binary `oura`). Useful only as a reference for CLI verb shape:
`sleep` / `readiness` / `activity` / `hrv` / `stress`, with dates `today` / `yesterday` /
`YYYY-MM-DD`. Our project is a strict superset (SDK suite + auth + MCP + plugin).

---

## TASK RUNNER: SINGLE ROOT JUSTFILE (strict)

- There is **ONE** justfile, at the repo root. **No per-directory justfiles.**
- **EVERY** dev and release command goes through `just <recipe>`: setup, build, test, lint,
  fmt, check, clean, codegen, spec-fetch, run/mcp, auth flows, release, publish, ci.
- Raw tools (cargo, npm/npx, openapi-generator, progenitor, cargo-dist/dist, jq, node) are
  invoked **ONLY inside recipes** — never documented or run directly. README, CLAUDE.md,
  CONTRIBUTING, and CI must show `just <recipe>`, never the underlying command.
- Recipes are **thin wrappers**. Group them (just's grouping/comments). Use justfile
  variables for the spec URL/version and paths. Default recipe = list recipes (`just`
  shows help).
- Representative recipes (fill in as needed, keep names stable):

  ```
  just setup            # install/verify toolchains + deps
  just spec-fetch       # re-fetch the versioned Oura openapi export to spec/openapi.json
  just spec-overlay     # apply codegen overlays (see below)
  just gen              # regenerate ALL generated SDK clients (never touches *-auth)
  just gen-rust / gen-ts / gen-py / gen-go
  just build            # workspace build
  just test             # unit
  just test-sandbox     # integration against Oura /v2/sandbox routes
  just fmt / lint / check
  just clean
  just mcp              # run the CLI (oura) as a STDIO MCP server
  just auth-setup / auth-login
  just release          # cargo-dist
  just publish          # npm + homebrew + crates
  just ci               # fmt-check + lint + test (what CI runs)
  ```

- **CI invokes `just ci` and nothing else.**

---

## SPEC IS THE SOURCE OF TRUTH

Vendor Oura's OpenAPI **v3.1.0** spec (title "Oura API Documentation", version **2.0**)
from the versioned static export and pin it at `spec/openapi.json`:

```
https://api.ouraring.com/v2/static/json/openapi-1.35.json
```

- Fetched via `just spec-fetch` (recipe wraps: `mkdir -p spec` then
  `curl -fsS <url> -o spec/openapi.json`).
- The spec drives **EVERYTHING** downstream: the Rust client, the MCP tools, and the
  breadth SDKs. `just spec-fetch` + `just gen` re-fetches and regenerates.
- **Drift watch (#29):** a SCHEDULED workflow (`.github/workflows/spec-drift.yml`, separate
  from `just ci`) runs `just spec-drift-check` weekly — it diffs the pinned URL against the
  committed spec and probes for a newer `openapi-<major>.<minor>`, opening/updating a
  `spec-drift` issue on change. Watch-only; adopting a change is the documented upgrade
  procedure (CONTRIBUTING). The detector's decision logic is guarded hermetically by
  `just spec-drift-selftest` in CI's gen-drift job.
- API base is `https://api.ouraring.com/v2`.
- **Pagination** is cursor-based via a `next_token` query param.
- **Sandbox** routes exist under `/v2/sandbox/usercollection/*` returning canned data —
  point `just test-sandbox`, MCP-tool development, and the auth smoke test at **sandbox**,
  not real data.

### Known spec issues — `just spec-overlay` applies these BEFORE any generation

1. `servers[0].url` is literally `"https://api.None.com"` (a leaked Python `None`).
   Rewrite to `"https://api.ouraring.com"`. Nothing resolves until this is fixed.
   **NON-NEGOTIABLE.**
2. Every multi-doc response is typed `anyOf:[TypedResponse, MultiDocumentResponseDict]` —
   the dict branch produces ugly union return types. **Strip the
   `MultiDocumentResponseDict` branch** so generated models stay clean.
3. For the generated **CLIENT only**, narrow per-op security to **BearerAuth** (see auth).
4. Every `start_date`/`end_date` query param is typed `anyOf:[date-time, date, null]`, which
   generates unusable-to-awkward param types (progenitor: always-failing flatten structs,
   PR #36; openapi-generator go: String/TimeTime union structs, #15). **Collapse to plain
   `date`** — moved from the Rust-only down-convert into the SHARED overlay 2026-07-03 once
   the Go client hit the same wart (Rust output stayed byte-identical).

Overlay files live in `codegen/` (no justfile there — recipes are in the root justfile).

---

## CLIENT / SDK GENERATION (do not hand-write transport)

- The Rust API client is **GENERATED** from the spec with **`progenitor`** (idiomatic Rust;
  far better than openapi-generator's generic Rust). This crate is `sdks/rust/oura-toolkit-api`.
- `sdks/rust/oura-toolkit-api` **IS** the shipped Rust SDK. Dual-use: the CLI depends on it directly
  (dogfooding = free integration test). Do **NOT** regenerate a second Rust copy elsewhere.
- **Breadth SDKs** (TypeScript, Python, Go, Java, C# — Java/C# added 2026-07-03) are
  generated with **openapi-generator** into `sdks/<lang>/api` (Python: the
  `oura_toolkit/api` subtree of the single dist). Pure data-plane: accept a token /
  configured client, stay **auth-agnostic**. Pinned toolchain (#15): generator jar
  **7.14.0** via `codegen/openapitools.json`, npm wrapper `@openapitools/
  openapi-generator-cli@2.39.1`, per-language configs `codegen/<lang>.yaml`. Gotchas: the
  wrapper does NOT honor config-file `globalProperties` — docs/test-stub suppression is the
  CLI flag in the justfile (`oag_skip_docs`); the python generator's root metadata mis-names
  the dist and doesn't build, so `sdks/python`'s pyproject.toml + namespace `__init__.py`
  are HAND-WRITTEN (Rust-Cargo.toml precedent) and `just gen-py` copies only the generated
  subtree (its version sync is guarded by `just version-check`, #59). The **C#** generator emits a single
  `netstandard2.0` (Newtonsoft-based) target and REJECTS `net10.0` outright, so `just
  gen-csharp` post-patches the csproj to multi-target `netstandard2.0;net8.0;net10.0` with a
  pinned `<LangVersion>13.0</LangVersion>` and strips the generator's bogus `System.Web`
  references (both post-patches are guarded — the recipe fails if either stops applying).
  The **typescript-fetch** packaging ships a Node-broken dual-ESM build (ESM-syntax `.js`
  in a CJS package, reachable only via the non-standard `module` field), no `exports`
  map, and placeholder repo metadata, so `just gen-ts` post-patches package.json to the
  auth companion's reviewed CJS-only + exports-map shape
  (`codegen/ts-package-postpatch.jq`) and drops `tsconfig.esm.json` (#57; guarded like
  the C# patches, and `just sdk-check-ts` loads the exports entry via BOTH `require` and
  a self-referencing ESM `import` smoke + asserts a dist-only `npm pack` surface). `just sdk-check` compile-checks all five (own CI job);
  `just test-sandbox-sdks` runs live sandbox smokes for all five (TS/Py/Go/Java/C#). The C#
  auth suite additionally runs under **Mono** against the net472-resolved `netstandard2.0`
  asset (`just sdk-test-csharp-netstandard`, its own `csharp-netstandard` CI job, #61) so the
  `#if NETSTANDARD2_0` store/transport branches and Polyfills execute on a runtime that loads
  them — not just compile (TESTING rule 6); a `BuildInfoTests` marker fails the leg if it ever
  loads the wrong asset. Speakeasy/Fern remain an option for
  companion codegen later.
- **DO NOT hand-write any transport/HTTP client in any language.** Generate it and depend
  on it.
- Every language ships the **SAME shape**: a generated client + a hand-written auth
  companion, side by side under `sdks/<lang>/`. Rust is not special; it just happens to
  also host the CLI.
- Mark generated crates/dirs **unmistakably** (`// @generated` header,
  `.gitattributes linguist-generated=true`, or a `GENERATED` marker) so nobody hand-edits
  them.
- **Verify current versions/usage** of `progenitor`, `rmcp`, `rmcp-openapi`, and
  `cargo-dist` from their docs before writing config — **do not assume**.

### Bootstrap discoveries (pinned toolchain realities — do NOT relitigate; these make the
### mandated progenitor path actually work)

- **Version matrix (bootstrap):** `cargo-progenitor` **0.14** / `progenitor-client` **0.14**
  need **reqwest ^0.13**; `reqwest-middleware` **0.5** / `reqwest-retry` **0.9** also target
  reqwest 0.13. The workspace pins **reqwest 0.13** (feature `rustls`, not the old
  `rustls-tls`; add `form`/`query`/`stream` where used).
- **progenitor reads OpenAPI 3.0 only.** The vendored spec is 3.1. A **Rust-only** jq
  down-convert (`codegen/progenitor-downconvert.jq`, run inside `just gen-rust` AFTER
  `spec-overlay`) relabels `3.1.0`→`3.0.3`, rewrites the `anyOf:[X,{type:null}]` nullable
  idiom to `nullable:true`, and prunes content-less `4xx` responses (progenitor asserts one
  error type per op). The **shared overlay stays 3.1** so the breadth SDKs (openapi-generator,
  3.1-native) keep full fidelity. (The date-param collapse originally lived here too; it moved
  to the shared overlay in #15 — see Known spec issues item 4.)
- **progenitor formats with unstable rustfmt opts** (`wrap_comments`,
  `normalize_doc_attributes`) → needs **nightly rustfmt**, and those opts corrupted a doc
  comment into invalid Rust. `just gen-rust` runs progenitor through `codegen/rustfmt-shim.sh`
  on a **pinned dated nightly** (the `nightly_rustfmt` justfile variable — single source for
  `setup`, `just install-nightly-rustfmt`, `gen-rust`, and CI's gen-drift job; #50), then
  re-formats with stable `cargo fmt` so CI's `fmt --check` passes. Pinning keeps codegen
  deterministic — a floating `nightly` shifting rustfmt behavior could otherwise flake an
  unrelated PR's `just gen-check` (bump procedure is documented at the variable). The committed
  generated crate builds on **stable**; only *regeneration* needs nightly + progenitor
  (installed by `just setup`).
- **The generated client takes a plain `reqwest::Client`** (`Client::new_with_client`);
  progenitor 0.14 does **not** accept a `reqwest_middleware::ClientWithMiddleware`. So the
  data-plane auth wiring (issues #9/#10) hands the generated client a `reqwest::Client`
  preconfigured with `oura-toolkit-auth`'s fresh Bearer (proactive refresh via `TokenManager`), and
  retries once on 401 via `TokenManager::force_refresh`. `oura-toolkit-auth` still ships the
  reqwest-middleware `AuthMiddleware`/`build_authenticated_client` for middleware consumers,
  but that is **not** the progenitor integration path.

---

## AUTH (the meaty hand-written part)

The spec generates the data plane; it does **NOT** generate the interactive OAuth flow.

### Confirmed from the spec's `components.securitySchemes`

- **BearerAuth**: `{ type: http, scheme: bearer }` — how data requests actually
  authenticate.
- **OAuth2**: `authorizationCode` flow, FULLY specified:
  - `authorizationUrl`: `https://cloud.ouraring.com/oauth/authorize`
  - `tokenUrl`: `https://api.ouraring.com/oauth/token`
  - scopes (8): `email`, `personal`, `daily`, `heartrate`, `workout`, `tag`, `session`,
    `spo2Daily`
- **ClientIdAuth / ClientSecretAuth**: apiKey headers `x-client-id` / `x-client-secret`,
  used **only** by webhook-subscription endpoints.

Per-op security is `BearerAuth OR OAuth2` with **EMPTY scope arrays** (no per-endpoint
scope mapping in the spec — that lives in prose).

### Implications

- The auth companion **READS** `authorizationUrl` / `tokenUrl` / the 8 scopes **FROM the
  vendored spec** (or codegen emits them as constants). **Do NOT hardcode** these
  URLs/scopes.
- No generator produces the interactive browser+loopback dance — **hand-write
  exchange/refresh for Rust** (progenitor gives only a bearer slot). Speakeasy/Fern CAN
  emit token refresh for the breadth SDKs from this flow block; evaluate them for
  TS/Python if you want thinner companions. Either way the **interactive consent is always
  hand-written**.

### Oura specifics (get these exactly right)

- Personal access tokens were **deprecated Dec 2025**. **OAuth2 Authorization Code flow
  only.**
- Oura is a **CONFIDENTIAL client**: the token endpoint requires **client_id AND
  client_secret**. There is **NO PKCE / public-client path**. Do NOT implement PKCE. Do NOT
  assume a public native-app flow.
- A client_secret can't be safely embedded in a distributed binary →
  **BRING-YOUR-OWN-CREDENTIALS**: each user registers their own Oura OAuth app and supplies
  their own `client_id`/`client_secret`. (Also sidesteps Oura's 10-users-per-app cap.)

### Structure

- **`sdks/rust/oura-toolkit-auth`**: reusable, **non-interactive** Rust auth companion — token
  store, refresh (with rotation), Bearer-injecting middleware, reads OAuth metadata from
  the spec. Depended on by **BOTH** the CLI's SDK calls **AND** the MCP tool calls (one
  auth layer, two consumers). Plugs into the generated client's bearer slot (progenitor:
  hand it a `reqwest::Client` with `reqwest-middleware` for auth + retry). Each other
  language gets its own sibling companion.
- **Interactive OAuth** (loopback listener, browser-open, `auth setup` / `auth login`)
  lives in **`cli/oura-toolkit-cli`**, NOT in any SDK/companion. SDK consumers bring their own
  token.

### Commands (all runnable via just recipes for local testing)

- **`oura auth setup`** — guided registration. Open
  `cloud.ouraring.com/oauth/applications` in the **USER'S OWN default browser**, print the
  exact values to paste (app name, redirect URI, scopes), then collect
  `client_id`/`client_secret` via **terminal prompts** (the secret with echo disabled,
  gh-style). No local HTTP form: the loopback listener exists only for the OAuth callback
  and is GET-only. **Secret never leaves the machine.** Chain into `auth login` on
  success. (Supersedes the earlier localhost paste-box design — decided 2026-07-02.)
- **`oura auth login`** — Authorization Code flow. Loopback HTTP listener on **FIXED
  default port 8788** (`redirect_uri http://localhost:8788/callback`, `--port` override).
  Oura requires the `redirect_uri` to be **pre-registered and match EXACTLY**. Open the
  authorize URL, catch the code on callback, exchange at the token endpoint with
  `client_id`+`client_secret`, persist.
- **Default scope request** for the toolkit: `personal daily heartrate workout tag session
  spo2Daily` (omit `email` unless needed).

### Token store (in oura-toolkit-auth)

- Fixed, invocation-independent path: `$XDG_CONFIG_HOME/oura-toolkit/`
  (→ `~/.config/oura-toolkit/`) on Unix/macOS (locked; deliberately NOT
  `~/Library/Application Support`), `%LOCALAPPDATA%\oura-toolkit\` on Windows (#24 —
  **Local, not Roaming**: roaming profiles sync `%APPDATA%` to file servers/backups, which
  would copy plaintext secrets off the machine). Perms **0600** + atomic writes on Unix; on
  Windows protection comes from `%LOCALAPPDATA%`'s user-private ACLs (the chmods are no-ops
  there). Empty or relative env values are ignored (a relative base
  would make secret placement cwd-dependent). MUST be identical whether invoked via `npx`,
  `bunx`, or a brew binary.
- **Keyring evaluated & deferred (#26, decided 2026-07-05):** migrating to the `keyring`
  crate (macOS Keychain / Windows Credential Manager / Linux Secret Service) was rejected —
  the **file store stays canonical**. Two load-bearing reasons: (1) the Linux Secret Service
  backend needs a live D-Bus session + unlocked keyring daemon, which headless servers /
  containers / SSH / WSL lack — and `oura mcp` runs exactly there, so a keyring backend would
  break the primary server deployment; (2) it wouldn't even retire the filesystem dependency —
  the #22 cross-process rotation lock (advisory `.lock`) and the atomic write-temp+rename that
  makes a refresh crash-safe still need the store dir, and a `keyring` set isn't atomic across
  the `credentials.json`+`tokens.json` pair, so keyring would add a SECOND secret store beside
  the file rather than replace it. The file is the only backend that is simultaneously
  invocation-independent, cross-process-lockable, and dependency-free on headless hosts. Incremental at-rest hardening that KEEPS the file
  (Windows DPAPI on the bytes; macOS Keychain as an opt-in layer) is tracked in **#78**.
- **Two records** (split 2026-07-02, #23): `credentials.json` (`client_id`/`client_secret` —
  exists from `auth setup` onward; refresh is a confidential-client call needing the secret)
  and `tokens.json` (`access_token`, `refresh_token`, `expiry`, scope). A failed login never
  loses the pasted secret. (No migration from the earlier combined shape — nothing shipped.)
- Oura **rotates the refresh_token** on refresh and **invalidates the old one** — you MUST
  persist the newly returned `refresh_token` or the next refresh 400s.
- **Cross-process safety** (#22): the CLI and the long-running MCP server share this store,
  so every refresh runs under an exclusive advisory `.lock` (std `File::lock`) and
  **re-reads the store first** — a rotation another process already performed is adopted
  instead of re-burned; a refresh 400 is retried once against freshly reloaded disk state.
  Lock acquisition happens on a blocking pool (never an executor thread) and the token
  endpoint has a hard 30s timeout, which bounds lock-hold time.
- **MSRV policy (decided 2026-07-02):** `rust-version` floats with recent stable — currently
  **1.89** for std file locking (`File::lock`), chosen over a locking dependency. Pre-1.0,
  no shipped consumers; revisit only if a real consumer needs older.

---

## TESTING & VERIFICATION (LOAD-BEARING — MUST ADHERE)

**The release gate:** a green `just ci` run on the full CI matrix IS the release decision —
**"if all tests pass, this tool is ready to release a new version."** No manual checklist
may stand between green CI and a release. Anything release confidence depends on MUST be an
automated check that fails CI when violated; if a property matters and CI can't see it, the
work isn't done.

Every change MUST satisfy all of the following (added 2026-07-02; enforced from PR #35 on):

1. **Guarantee = test.** Every documented behavior — `docs/cli-contract.md`, rustdoc
   promises, CLAUDE.md invariants (refresh-token rotation, 0600 perms, secret redaction,
   output sanitization, locked store paths) — has an enforcing test that fails when the
   guarantee breaks. A guarantee without a test is prose, not a guarantee.
2. **Hermetic `just test`.** No network, no real credentials, no shared/global state:
   wiremock for HTTP, tempdirs for stores, injected env lookups (never racy
   `env::set_var`). Live-sandbox integration lives ONLY behind `just test-sandbox` and
   never gates CI.
3. **Break-verified guard tests, and load-bearing assertions everywhere.** A test that
   guards a property must FAIL when the property is broken: when authoring one, neuter the
   code, watch the test fail with a message naming the contract, restore. More generally,
   every test's assertions must be load-bearing — pinned to the specific behavior claimed,
   not "it returned Ok" / "output is non-empty" / a substring so loose anything matches.
   Reviewing a test means asking "what wrong implementation would still pass this?" — if
   the answer is "plenty", the test manufactures release confidence and is worse than no
   test (see the self-masking unicode test caught in PR #34's review loop).
4. **Real concurrency for concurrency claims.** Locking/liveness/rotation guarantees are
   tested with genuinely concurrent tasks or lock-holders, never sequential approximations
   that would pass with a no-op lock (see PR #31's review loop).
5. **Security invariants each carry an attack test.** Redaction (`{:?}` leak tests),
   sanitization (escape/forgery payloads), permission modes — one enforcing test per
   invariant, exercising the hostile input, not just the happy path.
6. **Platform code runs on its platform.** `cfg`-gated branches ship with `cfg`-gated
   tests that execute on that OS's CI leg. The 3-OS matrix is the proof; "compiles locally"
   claims don't count.
7. **The user-facing contract is enforced at the binary level.** Exit codes, stream
   discipline, and error/hint shape are asserted by spawning the real `oura` binary
   (`cli/oura-toolkit-cli/tests/`), not only by unit-testing the classifier.
8. **Coverage floor (ratchet) — a tripwire, never a target.** `just coverage`
   (cargo-llvm-cov) enforces a minimum line-coverage floor on the hand-written crates (the
   generated client is excluded — it is exercised through its consumers and the drift
   check). CI runs it as its own job. Lowering the floor is a deliberate, reviewed
   decision; raising it after test additions is encouraged. The floor lives in the
   justfile (`coverage_floor`). **Coverage measures execution, not verification** — a line
   counts as covered even if no assertion would catch it misbehaving, so the floor only
   detects the *absence* of testing; rules 1 and 3 are what make the tests worth anything.
   Never add an assertion-free test to move the number: that converts a visible gap into
   invisible false confidence, which is strictly worse.

---

**Cross-language auth conformance (#58):** `codegen/conformance/auth-cases.json` is the
SINGLE SOURCE for the hostile token-endpoint responses, hostile store files, and canonical
store records every auth companion must survive (the two bug families found independently
by four companion review loops, plus the #54 store-fixture-drift check). ALL SIX companion
suites — the Rust reference included — iterate the fixture FROM THE FILE. A new hostile
case goes into the fixture, never into just one language's suite; a companion that fails a
case gets fixed — the fixture is never weakened to accommodate an implementation.

---

## DOCS STAY TRUE TO THE CODE (added 2026-07-03 — same weight as TESTING & VERIFICATION)

`README.md` is the front door, and `CONTRIBUTING.md` / `docs/cli-contract.md` /
`plugins/oura-toolkit/README.md` are shipped surface. A doc claim that contradicts the
code is a bug of the same severity as the code change that orphaned it.

1. **Same-PR rule.** Any change to a documented fact — user-visible behavior (CLI surface:
   commands, flags, help text, exit codes, output shape; auth flow; token-store location;
   MCP tools; plugin; install/release paths) or documented-but-non-behavioral facts (SDK
   status, supported targets / MSRV, package & binary names, the dev workflow's `just`
   recipes) — MUST update every doc claim it invalidates **in the same PR**. Doc sync is
   part of the definition of done, never a follow-up task.
2. **Review gate.** Every PR review (and every review-loop round) explicitly asks:
   *"which documented claim does this diff invalidate?"* — and verifies the touched docs
   against the code the way PR #44's accuracy lens did (commands, URLs, scopes, recipes,
   printed terminal text checked against source, not against memory).
3. **Catch-up commits are a process failure.** A docs-only PR whose purpose is re-syncing
   docs with reality (PR #44 was the last permitted one) means an earlier PR merged in
   violation of rule 1. Never plan one; if one becomes necessary, treat it as an incident:
   fix the docs AND name the PR that broke the rule. (Typo fixes and wording improvements
   that aren't drift catch-ups remain welcome — this rule targets only re-sync commits.)
4. **Mechanize what's mechanizable** (TESTING & VERIFICATION rule 1 applies to docs too):
   doc claims that are enumerable — the README's command list, scope string, redirect URI,
   store paths, recipe names, MCP tool names — MUST be pinned by tripwire tests (in the
   mold of mcp.rs's skill tool-name tripwire; #45). An enumerable doc claim without one is
   drift waiting to happen, and CI — not reviewer diligence — must catch it. Prose claims
   (tone, walkthrough accuracy) remain a review-gate responsibility — rules 1–3.

---

## MCP

- The CLI has an **`oura mcp` subcommand** that runs it as a **STDIO MCP server** using the
  official Rust SDK **`rmcp`** (run locally via `just mcp`). (A subcommand, not the earlier
  `--mcp` flag — decided 2026-07-02: modes and modifiers don't mix, and clap makes the
  nonsense states unrepresentable.) Expose ~8 well-described tools people actually want
  (daily sleep, daily readiness, daily activity, daily stress, heart rate, sessions,
  workouts, personal info), **hide the rest**, write good LLM-facing descriptions.
- **Tool generation (decided 2026-07-02; supersedes the earlier "use `rmcp-openapi`" plan;
  user-approved after a full trade-off review):** the tools are a **hybrid spec-codegen**
  over the CLI's own data plane, NOT `rmcp-openapi`. Verified against rmcp-openapi 0.31.2
  source: its auth is a static `HeaderMap` (or an HTTP-transport passthrough type from
  `rmcp-actix-web`) with **no seam** for per-call token rotation or silent
  401-refresh-retry — retrofitting ours would mean sniffing 401s out of its formatted tool
  output (a test pinning a third party's formatting), and it drags actix-web + rmcp 1.0
  into a stdio-only binary. Instead:
  - **Spec-derived at build time** (`cli/oura-toolkit-cli/build.rs`, same spec-read pattern
    as `oura-toolkit-auth`'s OAuth metadata): each tool description = hand-curated LLM lead
    + the operation's spec summary + the response document's field inventory with the
    spec's own field descriptions. A curated operation vanishing from the spec **fails the
    build**. Tool RESULTS are the progenitor-generated models serialized to JSON — as a text
    block AND (`#40`) a `structured_content` value: the same data typed for clients that read
    it, with a collection's array enveloped under `data` (MCP requires an object) and an
    object result passing through. Output schemas are not advertised yet (the progenitor
    models don't derive `JsonSchema`); the text block stays for client compat.
  - **Deliberately curated input schema** (one shared `DateRangeParams`): the cursor is
    hidden (tools auto-paginate), dates use the CLI's local-timezone semantics. The raw
    spec params are intentionally NOT the tool surface.
  - **Dispatch is hand-wired to `commands::fetch_*`** — the same functions the CLI
    subcommands render from: one auth layer AND one data plane, two presentations.
  - rmcp gotcha: `#[tool_handler]` defaults to a FRESH `Self::tool_router()` per request —
    it must be `#[tool_handler(router = self.tool_router)]` or descriptions injected at
    construction silently vanish (the `#[tool]` attr only takes literal descriptions, so
    the build-time ones are injected into the router's public `map` in `OuraMcp::new`).
  - Client disconnect (stdin EOF, even before the handshake) is a **clean exit 0**.
- MCP tool calls use the **SAME `oura-toolkit-auth` companion** (Bearer + refresh) as the CLI.
- **`oura mcp` auth behavior**: read tokens from the fixed path; refresh **silently on 401**,
  persist rotated tokens, retry. If tokens are genuinely **ABSENT**: do **NOT** prompt, do
  **NOT** open a browser, do **NOT** write to stdout (stdout is the JSON-RPC transport).
  Let the `initialize` handshake succeed, and on the **first tool call** return a
  **structured error** telling the user to run `oura auth login`.
- stdio MCP auth is **out-of-band** per the MCP spec — do **NOT** implement
  OAuth-over-the-wire for the server, and do **NOT** make the server remote/HTTP or a
  hosted OAuth broker.

---

## DISTRIBUTION

- **cargo-dist** ("dist", pinned **0.32.0** in `dist-workspace.toml` — landed 2026-07-02,
  #11) emits shell + powershell + npm + homebrew installers for the CLI across 5 targets.
- **Shell completions + man page in every archive (#75):** `dist-workspace.toml` `include`s
  the committed `cli/oura-toolkit-cli/dist-assets/` (generated from `oura completion`/`oura
  man`, drift-checked by `just gen-completions-check` in `release-config`). cargo-dist 0.32's
  `include` copies existing files, so they're committed (a build-time writer can't hit a
  stable path without breaking the read-only crates.io publish); the man page's `.TH` embeds
  the version, so `just set-version` needs a follow-up `just gen-completions`. **Known 0.32
  limit:** its Homebrew template has no completion/manpage wiring (and no config for it) — the
  files land in the formula's `pkgshare`, not auto-loaded. Archive INCLUSION is done; Homebrew
  AUTO-wiring is deferred to a cargo-dist upgrade (follow-up).
- **Releases are TAG-DRIVEN, not laptop-driven:** run `just set-version X.Y.Z` (#59 —
  the SINGLE WRITER: `codegen/version.sh` bumps the root `Cargo.toml` source (incl. the
  two internal-crate `[workspace.dependencies]` pins) plus every hand-written manifest
  carrying the literal — TS/Python/Java/C# companion manifests, plugin.json, .mcp.json's
  npx pin — self-verifying each rewrite; the recipe then refreshes Cargo.lock; never
  hand-edit those versions) → commit → tag `vX.Y.Z` → push. The justfile derives
  `version` from Cargo.toml, dist versions every installer from it, and mismatched tags
  fail at plan time. `just version-check` is the SINGLE drift guard (same script, `check`
  mode; replaced the per-file grep-guards that sprawled across gen-py/sdk-test-*/
  plugin-check) and it also round-trips the WRITER against a temp copy of the manifests,
  so a broken rewriter fails the `release-config` CI job on the PR that broke it. `.github/workflows/release.yml`
  (dist-generated, committed, drift-checked by `dist generate --check` inside
  `just dist-check`) builds all artifacts and runs the npm + homebrew publish jobs.
  `just release` is a LOCAL smoke build only; `just publish` covers only crates.io
  (dependency order: api → auth → cli).
- **One-time prerequisites before the first real release:** create `spxrogers/homebrew-tap`
  + a `HOMEBREW_TAP_TOKEN` secret with push access, and an `NPM_TOKEN` secret. Until then,
  tag pushes still build every installer artifact — only the publish jobs fail.
- **Before the breadth SDKs publish (later; NOT needed for the v0.1.0 CLI tag):** claim the
  npm `@oura-toolkit` scope, verify the `com.ouratoolkit` namespace on Maven Central (DNS
  TXT on ouratoolkit.com), and register the NuGet + PyPI names. The generated clients ship
  in-repo (compile-, drift- and smoke-checked) but unpublished until then.
- **The `release-config` CI job** runs `just dist-check` (plan + generate-drift + the
  NPX-first assertion on the real npm artifact: name `oura-toolkit`, bin `oura`) and
  `just publish-check` (the packaged crate builds in an out-of-repo temp dir — the
  crates.io context; guards the spec-bundle publish fix) on every PR.
- **Crates.io publishability:** the spec-reading build scripts (auth, CLI) read a
  crate-local `openapi.json` bundle (refreshed by `just spec-fetch`, sync-guarded by
  per-crate bundled-spec tests) because a published package has no repo root to walk to.
- Primary launch is **`npx -y oura-toolkit ...` (NPX-FIRST)**. `brew` / `bun i -g` are
  speed-path alternatives. **No bunx-first shell shim.** Known accepted upstream risk:
  cargo-dist 0.32's npm installer does not checksum-verify the binary it downloads
  (shell + homebrew do) — noted in `dist-workspace.toml`.

---

## PLUGIN

- **ONE** Claude plugin (not two) shipping **BOTH** the MCP server config **AND** skills.
  Skills call the MCP tools. The MCP server entry launches
  `npx -y oura-toolkit@<pinned-version> mcp`.
- **Landed 2026-07-02 (#12), schema-verified against Anthropic's docs:**
  `.claude-plugin/marketplace.json` lives at the **REPO ROOT** (the docs require it there —
  `/plugin marketplace add spxrogers/oura-toolkit` resolves it from the repo root and
  relative plugin sources resolve against the directory containing `.claude-plugin/`;
  supersedes the earlier `plugins/.claude-plugin/` placement, owner-approved). The plugin
  itself is `plugins/oura-toolkit/`: `.claude-plugin/plugin.json` (version = workspace
  version), `.mcp.json` (server `oura` → `npx -y oura-toolkit@<version> mcp`), README, and
  `skills/` (`morning-checkin`, `wellness-report` — auto-discovered; both handle the
  auth-required tool error by pointing at `oura auth login`).
- **Version pins are mechanically guarded**: plugin.json's `version` and .mcp.json's npx
  pin (including its exact arg position) must equal the workspace version — written by
  `just set-version` and guarded by `just version-check` like every other manifest (#59).
  Both manifests must additionally pass `claude plugin validate --strict` via
  `just plugin-check` (the CLI is installed in CI for exactly this); both recipes run in
  the `release-config` CI job.

---

## HARD "DO NOT" LIST

- Do **NOT** create more than one justfile, or place it anywhere but the repo root.
- Do **NOT** invoke or document any build/test/lint/fmt/clean/codegen/run/release/publish
  command outside `just`. Raw cargo/npm/dist/jq live only inside recipes.
- Do **NOT** hand-write a transport/HTTP client in any language. Generate it and depend on
  it. (Sole sanctioned exception: `oura api`, the arbitrary-path passthrough (#19), issues
  ONE raw `reqwest` request — a user-supplied path has no generated operation to call. It is
  a *caller* of the transport, still reusing the generated data plane's auth + 401-retry
  contract; it is NOT a second typed SDK. Do not generalize this to the typed commands.)
- Do **NOT** let codegen touch hand-written auth companions. `just gen` regenerates only
  the generated SDK clients (e.g. `sdks/rust/oura-toolkit-api`); it MUST NOT modify `sdks/*/…-auth`.
- Do **NOT** regenerate a second Rust SDK copy; `sdks/rust/oura-toolkit-api` is the one Rust SDK.
- Do **NOT** privilege Rust structurally; every language sits under `sdks/<lang>/` with the
  same client+companion shape. The CLI is an app, not an SDK, hence `cli/`.
- Do **NOT** use PKCE or assume a public client. Oura needs a client_secret.
- Do **NOT** embed any shared/default client_id or client_secret. BYO only.
- Do **NOT** hardcode the authorize/token URLs or scopes — read them from the spec.
- Do **NOT** put interactive OAuth (browser/loopback) in any SDK/companion — CLI only.
- Do **NOT** build any browser-automation / headless-Chromium / "claude -p" onboarding.
- Do **NOT** make the MCP server remote/HTTP or a hosted OAuth broker. STDIO only for v1.
- Do **NOT** split into two plugins.
- Do **NOT** put secrets in URL query strings, logs, or MCP stdout.
- Do **NOT** skip the `servers[0].url` overlay fix (api.None.com → api.ouraring.com).
- Do **NOT** merge a documented guarantee without its enforcing test, a guard test that
  hasn't been break-verified, or platform `cfg` code with no test on that CI leg — see
  TESTING & VERIFICATION (the release gate is "green CI == releasable").
- Do **NOT** merge a change to user-visible behavior without updating every doc claim it
  invalidates (README, CONTRIBUTING, cli-contract, plugin README) in the SAME PR —
  docs-only catch-up commits are a process failure, see DOCS STAY TRUE TO THE CODE.

---

## REPO LAYOUT

```
oura-toolkit/
├── CLAUDE.md                      # decisions/constraints (this file)
├── README.md                      # "register your Oura app" + setup; all commands as `just …`
├── justfile                       # THE single task runner for the whole repo
├── Cargo.toml                     # workspace; members point wherever crates live
├── spec/openapi.json              # vendored from openapi-1.35.json, pinned (via just spec-fetch)
├── sdks/
│   ├── rust/
│   │   ├── oura-toolkit-api/      # GENERATED (progenitor) = Rust SDK client. regen target; DO NOT hand-edit.
│   │   └── oura-toolkit-auth/     # hand-written Rust auth companion (store, refresh, middleware, spec-read metadata)
│   ├── typescript/                # api/ GENERATED (@oura-toolkit/api); auth/ hand-written (@oura-toolkit/auth)
│   ├── python/                    # oura_toolkit/api GENERATED; hand-written pyproject + ns __init__; oura_toolkit.auth companion
│   ├── go/                        # api/ GENERATED; hand-written go.mod (module …/sdks/go); auth/ companion
│   ├── java/                      # api/ GENERATED (com.ouratoolkit:api); auth/ companion (com.ouratoolkit:auth)
│   └── csharp/                    # api/ GENERATED (OuraToolkit.Api, multi-target netstandard2.0;net8.0;net10.0); auth/ companion (OuraToolkit.Auth)
├── cli/
│   └── oura-toolkit-cli/         # THE app (binary `oura`): auth setup|login (loopback OAuth), data cmds, mcp; depends on oura-toolkit-api + oura-toolkit-auth
├── .claude-plugin/marketplace.json  # at the REPO ROOT — required by the marketplace schema (see PLUGIN)
├── plugins/
│   └── oura-toolkit/             # single plugin (name matches dir): plugin.json + .mcp.json + skills/
├── codegen/                       # overlays + codegen/release scripts (NO justfile; recipes are in root justfile)
└── dist-workspace.toml            # cargo-dist config
```

**Cargo workspace members**: `["sdks/rust/oura-toolkit-api", "sdks/rust/oura-toolkit-auth", "cli/oura-toolkit-cli"]`.

---

## SESSION-1 PRIORITY ORDER

1. CLAUDE.md (this file)
2. git init + workspace skeleton
3. Root justfile with the core recipes
   (setup/spec-fetch/spec-overlay/gen/build/test/fmt/lint/clean/mcp/ci stubs)
4. `just spec-fetch` to vendor the spec
5. Write codegen overlays (fix servers url, strip dict union, narrow client security) wired
   to `just spec-overlay` / `just gen`
6. Generate `sdks/rust/oura-toolkit-api` with progenitor
7. Implement `sdks/rust/oura-toolkit-auth` (token store, refresh w/ rotation, Bearer middleware,
   spec-read metadata)
8. Implement `cli/oura-toolkit-cli` auth setup/login (loopback)
9. Stub `oura mcp` wiring rmcp + rmcp-openapi through oura-toolkit-auth
10. `dist-workspace.toml`
11. README setup section (commands shown as `just …`)

**Process note**: after CLAUDE.md, propose a concrete file-by-file plan and wait for
explicit confirmation before deep implementation.
