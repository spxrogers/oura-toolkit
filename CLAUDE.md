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

Availability verified against npm, crates.io, and PyPI. **Apply these names; do not "improve"
them.** Rationale is included only where it prevents a well-meaning refactor.

| Layer | Name | Notes |
|---|---|---|
| Project / repo / brand | `oura-toolkit` | umbrella name everywhere |
| CLI command (binary) | `oura` | what the user types — NOT `oura-toolkit`, NOT `oura-cli` |
| npm scope | `@oura-toolkit` | verified free |
| npm packages | `@oura-toolkit/api`, `@oura-toolkit/auth` | function-named leaves under the scope |
| Rust crates | `oura-toolkit-*` | `oura-toolkit-api`, `oura-toolkit-auth`, `oura-toolkit-cli` |
| Python distribution | `oura-toolkit` | single dist; submodules below |
| Python modules | `oura_toolkit.api`, `oura_toolkit.auth` | NOT separate PyPI packages |
| Claude plugin | `oura-toolkit` | |

**Per-ecosystem namespacing** (do NOT mirror the npm layout everywhere):
- **npm** — scoped `@oura-toolkit/api`, `@oura-toolkit/auth`. Keep `api`; never rename to `sdk`
  (the whole scope IS the SDK). Model: `@octokit/core` + `@octokit/auth-token`.
- **Rust** — no namespaces → flat prefixed crates `oura-toolkit-{api,auth,cli}`. Hyphens in the
  crate name, underscores on import (`use oura_toolkit_api::…`). The short `oura-api` is taken on
  crates.io anyway.
- **Python** — single distribution `oura-toolkit` with submodules `oura_toolkit.api` /
  `oura_toolkit.auth`. No per-function micro-packages.

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

Overlay files live in `codegen/` (no justfile there — recipes are in the root justfile).

---

## CLIENT / SDK GENERATION (do not hand-write transport)

- The Rust API client is **GENERATED** from the spec with **`progenitor`** (idiomatic Rust;
  far better than openapi-generator's generic Rust). This crate is `sdks/rust/oura-toolkit-api`.
- `sdks/rust/oura-toolkit-api` **IS** the shipped Rust SDK. Dual-use: the CLI depends on it directly
  (dogfooding = free integration test). Do **NOT** regenerate a second Rust copy elsewhere.
- **Breadth SDKs** (TypeScript, Python, Go) are generated with **openapi-generator** (or
  evaluate Speakeasy/Fern — see auth note) into `sdks/<lang>/`. Pure data-plane: accept a
  token / configured client, stay **auth-agnostic**.
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
  3.1-native) keep full fidelity.
- **progenitor formats with unstable rustfmt opts** (`wrap_comments`,
  `normalize_doc_attributes`) → needs **nightly rustfmt**, and those opts corrupted a doc
  comment into invalid Rust. `just gen-rust` runs progenitor through `codegen/rustfmt-shim.sh`
  (plain nightly rustfmt), then re-formats with stable `cargo fmt` so CI's `fmt --check`
  passes. The committed generated crate builds on **stable**; only *regeneration* needs
  nightly + progenitor (installed by `just setup`).
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

- Fixed, invocation-independent XDG path: `$XDG_CONFIG_HOME/oura-toolkit/`
  (→ `~/.config/oura-toolkit/`), perms **0600**, atomic writes. MUST be identical whether
  invoked via `npx`, `bunx`, or a brew binary.
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

## MCP

- The CLI has an **`oura mcp` subcommand** that runs it as a **STDIO MCP server** using the
  official Rust SDK **`rmcp`** (run locally via `just mcp`). (A subcommand, not the earlier
  `--mcp` flag — decided 2026-07-02: modes and modifiers don't mix, and clap makes the
  nonsense states unrepresentable.) Generate tools from the spec via
  **`rmcp-openapi`**, then add a thin **curation layer**: expose ~8 well-described tools
  people actually want (daily sleep, daily readiness, daily activity, daily stress, heart
  rate, sessions, workouts, personal info), **hide the rest**, write good LLM-facing
  descriptions.
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

- **cargo-dist** ("dist") emits shell + powershell + npm + homebrew installers from one
  `dist-workspace.toml`, driven by `just release` / `just publish`. **Confirm current
  cargo-dist version/config format** from its docs.
- Primary launch is **`npx -y oura-toolkit ...` (NPX-FIRST)**. `brew` / `bun i -g` are
  speed-path alternatives. **No bunx-first shell shim.**

---

## PLUGIN

- **ONE** Claude plugin (not two) shipping **BOTH** the MCP server config **AND** skills.
  Skills call the MCP tools. Standard marketplace layout
  (`.claude-plugin/marketplace.json` + the plugin's `plugin.json`). The MCP server entry
  launches `npx -y oura-toolkit@<pinned-version> mcp`. **Check the current Claude
  plugin/marketplace manifest schema** against Anthropic's docs before writing manifests.

---

## HARD "DO NOT" LIST

- Do **NOT** create more than one justfile, or place it anywhere but the repo root.
- Do **NOT** invoke or document any build/test/lint/fmt/clean/codegen/run/release/publish
  command outside `just`. Raw cargo/npm/dist/jq live only inside recipes.
- Do **NOT** hand-write a transport/HTTP client in any language. Generate it and depend on
  it.
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
│   ├── typescript/                # generated client + auth companion (later)
│   ├── python/
│   └── go/
├── cli/
│   └── oura-toolkit-cli/         # THE app (binary `oura`): auth setup|login (loopback OAuth), data cmds, mcp; depends on oura-toolkit-api + oura-toolkit-auth
├── plugins/
│   ├── .claude-plugin/marketplace.json
│   └── oura-toolkit/             # single plugin (name matches dir): MCP server entry + skills/
├── codegen/                       # overlays + codegen scripts (NO justfile; recipes are in root justfile)
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
