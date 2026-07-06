# CLAUDE.md — oura-toolkit

This file is the **law**: the hard constraints every change must satisfy. Two companion docs
carry the rest so this one stays lean and load-bearing:

- **[ARCHITECTURE.md](ARCHITECTURE.md)** — the map: how the pieces fit (pipeline, workspace,
  auth, CLI, MCP, distribution, repo layout).
- **[DECISIONS.md](DECISIONS.md)** — the log: *why* each locked choice was made, plus the
  hard-won toolchain realities and review-loop lessons behind them.

The architecture is **MOSTLY FINAL**. Do not relitigate the language, transport,
client-generation, auth model, layout, or task-runner convention unless you have **high
conviction** on a specific item; in that case, present the issue + suggestion(s) and wait for
an explicit answer before acting. Rationale for the settled questions lives in DECISIONS.md —
read it before proposing a change to something that looks arbitrary.

---

## PROJECT

**oura-toolkit**: a Rust CLI for the Oura Ring API v2, plus a single Claude plugin and
auto-generated multi-language SDKs. Monorepo (cargo workspace + plugin marketplace).

The CLI is written in Rust, but **Rust is NOT privileged** — it's one language among the
SDKs. The CLI is an *app*, not an SDK (hence `cli/`). Every language sits under
`sdks/<lang>/` with the same client + companion shape. API base:
`https://api.ouraring.com/v2`. See [ARCHITECTURE.md](ARCHITECTURE.md) for the full picture.

---

## NAMING (LOCKED)

Availability verified against npm, crates.io, PyPI, Maven Central and NuGet. **Apply these
names; do not "improve" them.** (Rationale in [DECISIONS.md](DECISIONS.md).)

| Layer | Name | Notes |
|---|---|---|
| Project / repo / brand | `oura-toolkit` | umbrella name everywhere |
| CLI command (binary) | `oura` | what the user types — NOT `oura-toolkit`, NOT `oura-cli` |
| npm scope / packages | `@oura-toolkit` → `@oura-toolkit/api`, `@oura-toolkit/auth` | function-named leaves |
| Rust crates | `oura-toolkit-{api,auth,cli}` | hyphens in name, underscores on import |
| Python | dist `oura-toolkit`; modules `oura_toolkit.api` / `oura_toolkit.auth` | single dist, no micro-packages |
| Java (Maven Central) | `com.ouratoolkit:api`, `com.ouratoolkit:auth` | owner owns ouratoolkit.com |
| C# (NuGet) | `OuraToolkit.Api`, `OuraToolkit.Auth` | namespaces match package ids |
| Claude plugin | `oura-toolkit` | |

- **Binary ≠ crate name (the one Cargo gotcha):** the CLI crate is `oura-toolkit-cli` but
  `[[bin]] name = "oura"` installs a binary named `oura`. Each crate's directory matches its
  crate name (no diverging `[package] name`).
- **Per-ecosystem namespacing** — do NOT mirror the npm layout everywhere: Rust flat-prefixed
  crates, single Python dist, `com.ouratoolkit` Maven group, `OuraToolkit.*` NuGet.
- **Do NOT**: name any package/module `sdk`; suffix the binary `-cli`; publish Python
  sub-packages per function; try to claim the taken names `oura`/`oura-api`/`oura-cli`.

---

## TASK RUNNER: SINGLE ROOT JUSTFILE (strict)

- There is **ONE** justfile, at the repo root. **No per-directory justfiles.**
- **EVERY** dev/release command goes through `just <recipe>`. Raw tools (cargo, npm,
  openapi-generator, progenitor, dist, jq, node) are invoked **ONLY inside recipes** — never
  documented or run directly. README, CLAUDE.md, CONTRIBUTING and CI show `just <recipe>`,
  never the underlying command.
- Recipes are **thin wrappers**; default recipe lists recipes (`just` shows help).
- **CI invokes `just ci` and nothing else** (fmt-check + lint + test). Other CI jobs each call
  a single named recipe.

---

## SPEC IS THE SOURCE OF TRUTH

Vendor Oura's OpenAPI **v3.1** spec (title "Oura API Documentation", version **2.0**) and pin
it at `spec/openapi.json` from the versioned export:

```
https://api.ouraring.com/v2/static/json/openapi-1.35.json
```

- Fetched via `just spec-fetch`. The spec drives **EVERYTHING** downstream — the Rust client,
  the MCP tools, and the breadth SDKs. `just spec-fetch` + `just gen` re-fetches and
  regenerates.
- API base `https://api.ouraring.com/v2`; **pagination** is cursor-based via `next_token`;
  **sandbox** routes under `/v2/sandbox/usercollection/*` return canned data — point
  `just test-sandbox`, MCP-tool development, and the auth smoke test at sandbox, not real data.
- **Drift watch (#29):** a scheduled workflow (`.github/workflows/spec-drift.yml`) runs
  `just spec-drift-check` weekly and opens/updates a `spec-drift` issue on change. Watch-only;
  the detector's logic is guarded by `just spec-drift-selftest` in CI. Adopting a change is
  the documented upgrade procedure (CONTRIBUTING).

### Known spec issues — `just spec-overlay` applies these BEFORE any generation

The shared overlay (`codegen/overlay.jq`, 3.1, all languages) is **non-negotiable**:

1. `servers[0].url` is literally `"https://api.None.com"` (a leaked Python `None`) — rewrite
   to `"https://api.ouraring.com"`. Nothing resolves until this is fixed. **NON-NEGOTIABLE.**
2. Strip the `MultiDocumentResponseDict` branch from every `anyOf` multi-doc response so
   generated models stay clean.
3. For the generated **CLIENT only**, narrow per-op security to **BearerAuth**.
4. Collapse every `start_date`/`end_date` param (`anyOf:[date-time, date, null]`) to plain
   `date` — the union generates unusable/awkward param types.

The Rust-only 3.1→3.0 down-convert (progenitor reads 3.0 only) runs *after* this shared
overlay; see [DECISIONS.md](DECISIONS.md). Overlay files live in `codegen/`.

---

## CLIENT / SDK GENERATION (do not hand-write transport)

- **DO NOT hand-write any transport/HTTP client in any language.** Generate it and depend on
  it. (Sole sanctioned exception: `oura api`, the arbitrary-path passthrough — see HARD DO NOT.)
- The Rust client is GENERATED with **progenitor**; `sdks/rust/oura-toolkit-api` **IS** the
  shipped Rust SDK and the CLI depends on it directly (dogfooding). Do **NOT** regenerate a
  second Rust copy elsewhere.
- **Breadth SDKs** (TS, Python, Go, Java, C#) are generated with **openapi-generator** (jar
  pinned 7.14.0) into `sdks/<lang>/api`. Pure data-plane, auth-agnostic.
- Every language ships the **SAME shape**: generated client + hand-written auth companion,
  side by side under `sdks/<lang>/`. Rust is not special; it just also hosts the CLI.
- `just gen` regenerates ONLY the generated clients — it MUST NOT modify any `sdks/*/…-auth`
  companion or hand-written distribution metadata.
- Mark generated crates/dirs **unmistakably** (`// @generated`, `linguist-generated=true`) so
  nobody hand-edits them. Per-language codegen post-patches and the pinned-toolchain realities
  that make progenitor + openapi-generator work are documented in [DECISIONS.md](DECISIONS.md)
  — do not relitigate them.
- **Verify current versions/usage** of `progenitor`, `rmcp`, `cargo-dist` from their docs
  before writing config — do not assume.

---

## AUTH (the meaty hand-written part)

The spec generates the data plane; it does **NOT** generate the interactive OAuth flow. From
the spec's `components.securitySchemes`: **BearerAuth** (`http`/`bearer`) is how data requests
authenticate; **OAuth2** is `authorizationCode` with `authorizationUrl`
`https://cloud.ouraring.com/oauth/authorize`, `tokenUrl` `https://api.ouraring.com/oauth/token`
and 8 scopes (`email personal daily heartrate workout tag session spo2Daily`);
**ClientIdAuth/ClientSecretAuth** apiKey headers are used only by webhook endpoints.

### Invariants (get these exactly right)

- **OAuth2 Authorization Code flow only** — personal access tokens were deprecated Dec 2025.
- Oura is a **CONFIDENTIAL client**: the token endpoint requires client_id AND client_secret.
  **NO PKCE / public-client path.** Do NOT implement PKCE.
- **BRING-YOUR-OWN-CREDENTIALS**: each user registers their own Oura OAuth app and supplies
  their own client_id/secret. Do **NOT** embed any shared/default credential.
- The auth companion **READS** authorizeUrl/tokenUrl/scopes **FROM the vendored spec** — do
  **NOT** hardcode them.
- **Interactive OAuth** (browser/loopback) lives in `cli/oura-toolkit-cli` ONLY, never in any
  SDK/companion. `sdks/rust/oura-toolkit-auth` is the reusable, **non-interactive** companion
  (store, refresh-with-rotation, Bearer middleware, spec-read metadata) shared by BOTH the
  CLI's data calls AND the MCP tool calls — one auth layer, two consumers.

### Commands

- **`oura auth setup`** — guided registration: open Oura's app page in the user's own default
  browser, print the exact values to paste, collect client_id/secret via terminal prompts
  (secret echo-disabled). No local HTTP form; the loopback listener is the OAuth callback only
  (GET-only). Chains into `auth login`.
- **`oura auth login`** — Authorization Code flow. Loopback listener on **fixed default port
  8788** (`redirect_uri http://localhost:8788/callback`, `--port` override; Oura requires an
  EXACT pre-registered match). `--no-browser` swaps the loopback catch for a
  paste-the-redirect-URL flow (still `state` CSRF-checked).
- **Default scope request**: `personal daily heartrate workout tag session spo2Daily` (omit
  `email` unless needed).
- **Headless**: `OURA_ACCESS_TOKEN` injects a raw Bearer that bypasses the store (no login, no
  refresh); `OURA_API_BASE_URL` points the client at an alternate host/proxy/mock.

### Token store (in oura-toolkit-auth)

- **Fixed, invocation-independent path**: `$XDG_CONFIG_HOME/oura-toolkit/`
  (→ `~/.config/oura-toolkit/`) on Unix/macOS (locked; deliberately NOT
  `~/Library/Application Support`), `%LOCALAPPDATA%\oura-toolkit\` on Windows (**Local, not
  Roaming**). Perms **0600** + atomic writes on Unix; Windows relies on `%LOCALAPPDATA%`'s
  private ACLs. Empty/relative env values are ignored. MUST be identical under npx/bunx/brew.
- **Two records** (#23): `credentials.json` (client_id/secret) and `tokens.json`
  (access/refresh/expiry/scope) — a failed login never loses the pasted secret.
- **Rotation**: Oura rotates the refresh_token on refresh and invalidates the old one — you
  MUST persist the newly returned refresh_token or the next refresh 400s.
- **Cross-process safety** (#22): CLI + long-running MCP server share the store, so every
  refresh runs under an exclusive advisory `.lock`, **re-reads the store first** (adopt a
  rotation another process performed), and retries a 400 once against freshly reloaded state.
  Lock acquisition is on a blocking pool; the token endpoint has a hard 30s timeout.
- The file store is **canonical**; keyring was evaluated and deferred (#26, reasoning in
  [DECISIONS.md](DECISIONS.md)). At-rest hardening that keeps the file is tracked in #78.

---

## TESTING & VERIFICATION (LOAD-BEARING — MUST ADHERE)

**The release gate:** a green `just ci` run on the full CI matrix IS the release decision —
**"if all tests pass, this tool is ready to release a new version."** No manual checklist may
stand between green CI and a release. Anything release confidence depends on MUST be an
automated check that fails CI when violated; if a property matters and CI can't see it, the
work isn't done.

Every change MUST satisfy all of the following:

1. **Guarantee = test.** Every documented behavior — `docs/cli-contract.md`, rustdoc
   promises, CLAUDE.md invariants (refresh-token rotation, 0600 perms, secret redaction,
   output sanitization, locked store paths) — has an enforcing test that fails when the
   guarantee breaks. A guarantee without a test is prose, not a guarantee.
2. **Hermetic `just test`.** No network, no real credentials, no shared/global state:
   wiremock for HTTP, tempdirs for stores, injected env lookups (never racy
   `env::set_var`). Live-sandbox integration lives ONLY behind `just test-sandbox` and never
   gates CI.
3. **Break-verified guard tests, and load-bearing assertions everywhere.** A test that guards
   a property must FAIL when the property is broken: when authoring one, neuter the code,
   watch the test fail with a message naming the contract, restore. More generally, every
   test's assertions must be load-bearing — pinned to the specific behavior claimed, not "it
   returned Ok" / "output is non-empty" / a substring so loose anything matches. Reviewing a
   test means asking "what wrong implementation would still pass this?" — if the answer is
   "plenty", the test manufactures release confidence and is worse than no test.
4. **Real concurrency for concurrency claims.** Locking/liveness/rotation guarantees are
   tested with genuinely concurrent tasks or lock-holders, never sequential approximations
   that would pass with a no-op lock.
5. **Security invariants each carry an attack test.** Redaction (`{:?}` leak tests),
   sanitization (escape/forgery payloads), permission modes — one enforcing test per
   invariant, exercising the hostile input, not just the happy path.
6. **Platform code runs on its platform.** `cfg`-gated branches ship with `cfg`-gated tests
   that execute on that OS's CI leg. The 3-OS matrix is the proof; "compiles locally" claims
   don't count.
7. **The user-facing contract is enforced at the binary level.** Exit codes, stream
   discipline, and error/hint shape are asserted by spawning the real `oura` binary
   (`cli/oura-toolkit-cli/tests/`), not only by unit-testing the classifier.
8. **Coverage floor (ratchet) — a tripwire, never a target.** `just coverage`
   (cargo-llvm-cov) enforces a minimum line-coverage floor on the hand-written crates (the
   generated client is excluded). CI runs it as its own job. Lowering the floor is a
   deliberate, reviewed decision. **Coverage measures execution, not verification** — the
   floor only detects the *absence* of testing; rules 1 and 3 are what make the tests worth
   anything. Never add an assertion-free test to move the number.

**Cross-language auth conformance (#58):** `codegen/conformance/auth-cases.json` is the SINGLE
SOURCE for the hostile token-endpoint responses, hostile store files, and canonical store
records every auth companion must survive. ALL SIX companion suites — the Rust reference
included — iterate the fixture FROM THE FILE. A new hostile case goes into the fixture, never
into just one language's suite; a companion that fails a case gets fixed — the fixture is never
weakened to accommodate an implementation.

---

## DOCS STAY TRUE TO THE CODE (same weight as TESTING & VERIFICATION)

`README.md` is the front door, and `CONTRIBUTING.md` / `docs/cli-contract.md` /
`plugins/oura-toolkit/README.md` are shipped surface. A doc claim that contradicts the code is
a bug of the same severity as the code change that orphaned it.

1. **Same-PR rule.** Any change to a documented fact — user-visible behavior (CLI surface,
   auth flow, token-store location, MCP tools, plugin, install/release paths) or
   documented-but-non-behavioral facts (SDK status, supported targets / MSRV, package & binary
   names, the `just` recipes) — MUST update every doc claim it invalidates **in the same PR**.
   Doc sync is part of the definition of done, never a follow-up task. **This includes
   ARCHITECTURE.md and DECISIONS.md** — a change that moots a stated decision or map fact
   updates them in the same PR.
2. **Review gate.** Every PR review (and every review-loop round) explicitly asks: *"which
   documented claim does this diff invalidate?"* — and verifies the touched docs against the
   code (commands, URLs, scopes, recipes, printed terminal text checked against source, not
   memory).
3. **Catch-up commits are a process failure.** A docs-only PR whose purpose is re-syncing docs
   with reality means an earlier PR merged in violation of rule 1. Never plan one; if one
   becomes necessary, treat it as an incident. (Typo/wording fixes remain welcome.)
4. **Mechanize what's mechanizable.** Doc claims that are enumerable — the README's command
   list, scope string, redirect URI, store paths, recipe names, MCP tool names — MUST be
   pinned by tripwire tests (in the mold of mcp.rs's tool-name tripwire; #45). CI, not
   reviewer diligence, must catch drift. Prose claims (tone, walkthrough accuracy) remain a
   review-gate responsibility (rules 1–3).

---

## MCP

- The CLI has an **`oura mcp` subcommand** (not a flag) that runs it as a **STDIO** MCP server
  using the official Rust SDK **`rmcp`** (`just mcp`). Expose ~8 well-described tools people
  actually want (daily sleep/readiness/activity/stress, heart rate, sessions, workouts,
  personal info), hide the rest, write good LLM-facing descriptions.
- **Tool generation is a hybrid spec-codegen** over the CLI's own data plane, NOT
  `rmcp-openapi` (decided 2026-07-02; trade-off review in [DECISIONS.md](DECISIONS.md)):
  spec-derived descriptions at build time (`build.rs` — a curated op vanishing from the spec
  fails the build), a curated input schema (one shared `DateRangeParams`, cursor hidden,
  local-timezone dates), and dispatch hand-wired to the same `commands::fetch_*` functions the
  CLI renders from. Results are the progenitor models serialized to JSON: a text block AND a
  `structured_content` value (a collection's array enveloped under `data`, an object passing
  through).
- MCP tool calls use the **SAME `oura-toolkit-auth`** companion (Bearer + refresh) as the CLI.
- **`oura mcp` auth behavior**: read tokens from the fixed path; refresh **silently on 401**,
  persist rotated tokens, retry. If tokens are **ABSENT**: do **NOT** prompt, open a browser,
  or write to stdout (stdout is the JSON-RPC transport). Let `initialize` succeed; on the
  **first tool call** return a structured error telling the user to run `oura auth login`.
- stdio MCP auth is **out-of-band** per the MCP spec — do **NOT** implement OAuth-over-the-wire
  for the server, and do **NOT** make it remote/HTTP or a hosted OAuth broker. STDIO only for
  v1.

---

## DISTRIBUTION

- **cargo-dist** (pinned **0.32.0**) emits shell + powershell + npm + homebrew installers for
  the CLI across 5 targets. Release archives also ship the man page + shell completions
  (committed under `cli/oura-toolkit-cli/dist-assets/`, drift-checked by
  `just gen-completions-check`). Known cargo-dist 0.32 limit: its Homebrew template has no
  completion/manpage wiring — archive INCLUSION is done; Homebrew AUTO-wiring is deferred to a
  cargo-dist upgrade (#75, see [DECISIONS.md](DECISIONS.md)).
- **Releases are TAG-DRIVEN, not laptop-driven:** `just set-version X.Y.Z` (#59 — the SINGLE
  WRITER of the version across every manifest, self-verifying; also regenerates completions)
  → commit → tag `vX.Y.Z` → push. `just version-check` is the SINGLE drift guard (and
  round-trips the writer, so a broken rewriter fails the `release-config` CI job).
  `.github/workflows/release.yml` (dist-generated, committed, drift-checked by `just
  dist-check`) builds artifacts and runs the npm + homebrew publish jobs. `just release` is a
  LOCAL smoke build; `just publish` covers only crates.io (order: api → auth → cli).
- **Crates.io publishability:** the spec-reading build scripts read a crate-local
  `openapi.json` bundle (a published package has no repo root to walk to); sync-guarded by
  per-crate bundled-spec tests. `release-config` CI runs `just dist-check` + `just
  publish-check`.
- Primary launch is **`npx -y oura-toolkit ...` (NPX-FIRST)**; `brew` / `bun i -g` are
  speed-path alternatives. Known accepted risk: cargo-dist 0.32's npm installer does not
  checksum-verify the binary it downloads (shell + homebrew do).
- **One-time prerequisites before the first real release:** `spxrogers/homebrew-tap` +
  `HOMEBREW_TAP_TOKEN`, and `NPM_TOKEN`. Until then tag pushes still build every artifact —
  only the publish jobs fail. Before the breadth SDKs publish (later): claim the npm
  `@oura-toolkit` scope, verify `com.ouratoolkit` on Maven Central, register the NuGet + PyPI
  names.

---

## PLUGIN

- **ONE** Claude plugin (not two) shipping **BOTH** the MCP server config **AND** skills.
  Skills call the MCP tools. The MCP server entry launches
  `npx -y oura-toolkit@<pinned-version> mcp`.
- `.claude-plugin/marketplace.json` lives at the **REPO ROOT** (the marketplace schema
  requires it — `/plugin marketplace add spxrogers/oura-toolkit` resolves it there). The plugin
  itself is `plugins/oura-toolkit/`: `.claude-plugin/plugin.json` (version = workspace
  version), `.mcp.json` (server `oura` → `npx -y oura-toolkit@<version> mcp`), README, and
  `skills/` (`morning-checkin`, `wellness-report` — both handle the auth-required tool error by
  pointing at `oura auth login`).
- **Version pins are mechanically guarded**: plugin.json's `version` and .mcp.json's npx pin
  (including its exact arg position) must equal the workspace version — written by `just
  set-version`, guarded by `just version-check`. Both manifests must pass `claude plugin
  validate --strict` via `just plugin-check`. Both recipes run in the `release-config` CI job.

---

## HARD "DO NOT" LIST

- Do **NOT** create more than one justfile, or place it anywhere but the repo root.
- Do **NOT** invoke or document any build/test/lint/fmt/clean/codegen/run/release/publish
  command outside `just`. Raw cargo/npm/dist/jq live only inside recipes.
- Do **NOT** hand-write a transport/HTTP client in any language. Generate it and depend on it.
  (Sole sanctioned exception: `oura api`, the arbitrary-path passthrough (#19), issues ONE raw
  `reqwest` request — a user-supplied path has no generated operation to call. It is a
  *caller* of the transport, reusing the generated data plane's auth + 401-retry contract; it
  is NOT a second typed SDK. Do not generalize this to the typed commands.)
- Do **NOT** let codegen touch hand-written auth companions. `just gen` regenerates only the
  generated SDK clients; it MUST NOT modify `sdks/*/…-auth`.
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
- Do **NOT** merge a documented guarantee without its enforcing test, a guard test that hasn't
  been break-verified, or platform `cfg` code with no test on that CI leg — see TESTING &
  VERIFICATION (the release gate is "green CI == releasable").
- Do **NOT** merge a change to user-visible behavior without updating every doc claim it
  invalidates (README, CONTRIBUTING, cli-contract, plugin README, and — when a decision or map
  fact changes — ARCHITECTURE.md / DECISIONS.md) in the SAME PR. Docs-only catch-up commits are
  a process failure — see DOCS STAY TRUE TO THE CODE.

---

*Repo layout, the codegen pipeline diagram, and the full system map live in
[ARCHITECTURE.md](ARCHITECTURE.md). The rationale for every locked decision and the toolchain
lessons behind them live in [DECISIONS.md](DECISIONS.md).*
