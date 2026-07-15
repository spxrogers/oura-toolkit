# Decisions & lessons — oura-toolkit

The **why** behind the locked choices, and the hard-won toolchain realities that make the
mandated paths actually work. [CLAUDE.md](CLAUDE.md) states the rules; this file records the
reasoning so nobody relitigates a settled question or re-learns a lesson the hard way.

**These are decided.** Reopen one only with high conviction on a specific item — present the
issue + suggestion and wait for an explicit answer, per CLAUDE.md's process rule.

---

## Architecture decisions

### Client generation: progenitor (Rust) + openapi-generator (breadth)
The Rust client is generated with **progenitor** — idiomatic Rust, far better than
openapi-generator's generic Rust output. `sdks/rust/oura-toolkit-api` **is** the shipped Rust
SDK; the CLI depends on it directly, so dogfooding doubles as a free integration test. Never
regenerate a second Rust copy. Breadth SDKs (TS/Python/Go/Java/C#) use **openapi-generator**
(jar 7.14.0, pinned). Transport is never hand-written in any language — generate and depend.

### Naming (locked, availability-verified)
Verified free against npm, crates.io, PyPI, Maven Central and NuGet. Umbrella brand
`oura-toolkit`; binary is `oura` (not `oura-cli`); per-ecosystem namespacing rather than
mirroring npm everywhere (Rust flat-prefixed crates, single Python dist, `com.ouratoolkit`
Maven group, `OuraToolkit.*` NuGet). The one Cargo gotcha: CLI crate `oura-toolkit-cli`
installs a binary named `oura` via `[[bin]] name = "oura"`. Full table in
[CLAUDE.md → NAMING](CLAUDE.md). The names are verified; do not "improve" them.

### Auth: OAuth-only, confidential, bring-your-own-credentials
Oura deprecated personal access tokens (Dec 2025) → Authorization Code flow only. Oura is a
**confidential client** (token endpoint needs client_id **and** secret) → **no PKCE, no
public-client path**. A secret can't ship in a distributed binary → each user registers
their own Oura app (which also sidesteps Oura's 10-users-per-app cap). Interactive consent is
always hand-written and CLI-only; SDK consumers bring their own token.

### `auth setup` collects the secret via terminal prompt (not a localhost form)
Decided 2026-07-02. `oura auth setup` opens Oura's app-registration page, prints the exact
values to paste, then collects client_id/secret via terminal prompts (secret with echo
disabled, gh-style). The loopback listener exists **only** for the OAuth callback and is
GET-only — the secret never leaves the machine via an HTTP form.

### Token store: file-backed, keyring rejected (#26)
The file store at the locked config path is canonical. Migrating to the `keyring` crate was
**rejected** (2026-07-05) for two load-bearing reasons: (1) the Linux Secret Service backend
needs a live D-Bus session + unlocked keyring daemon, which headless servers / containers /
SSH / WSL lack — and `oura mcp` runs exactly there, so keyring would break the primary server
deployment; (2) it wouldn't even retire the filesystem dependency — the cross-process
rotation lock (#22) and the atomic write-temp+rename still need the store dir, and a keyring
set isn't atomic across the `credentials.json`+`tokens.json` pair, so it would add a *second*
secret store rather than replace the file. The file is the only backend that is
simultaneously invocation-independent, cross-process-lockable, and dependency-free on
headless hosts. Incremental at-rest hardening that KEEPS the file (Windows DPAPI, opt-in
macOS Keychain) is tracked in **#78**.

### Token store: split records, Local not Roaming, rotation persisted
- **Two records** (#23): `credentials.json` and `tokens.json`, split so a failed login never
  loses the pasted secret.
- **Windows Local, not Roaming** (#24): roaming profiles sync `%APPDATA%` to file
  servers/backups, which would copy plaintext secrets off the machine.
- **Rotation** — Oura rotates the refresh_token on every refresh and invalidates the old one;
  you MUST persist the newly returned token or the next refresh 400s.
- **Cross-process lock** (#22): CLI + MCP server share the store, so refresh runs under an
  exclusive advisory `.lock`, re-reads first, retries a 400 once against fresh state. Lock
  acquisition is on a blocking pool; the token endpoint has a hard 30s timeout to bound
  lock-hold time.

### MCP tools: hybrid spec-codegen, not `rmcp-openapi`
Decided 2026-07-02, user-approved after a full trade-off review. Verified against
rmcp-openapi 0.31.2 source: its auth is a static `HeaderMap` with **no seam** for per-call
token rotation or silent 401-refresh-retry — retrofitting ours would mean sniffing 401s out
of its formatted tool output (a test pinning a third party's formatting), and it drags
actix-web + rmcp 1.0 into a stdio-only binary. Instead the tools are a hybrid: spec-derived
descriptions at build time, curated input schema (one shared `DateRangeParams`, cursor
hidden), dispatch hand-wired to the CLI's own `commands::fetch_*`. See
[CLAUDE.md → MCP](CLAUDE.md) and [ARCHITECTURE.md](ARCHITECTURE.md).

### MCP: STDIO only, subcommand not flag
STDIO transport only for v1 — not remote/HTTP, not a hosted OAuth broker (stdio MCP auth is
out-of-band per the MCP spec). It's a subcommand (`oura mcp`), not a `--mcp` flag (decided
2026-07-02: modes and modifiers don't mix, and clap makes the nonsense states
unrepresentable).

### Single root justfile
There is exactly ONE justfile, at the repo root. Every dev/release command is a `just`
recipe; raw tools (cargo/npm/openapi-generator/dist/jq) are invoked **only inside recipes**,
never documented or run directly. Recipes are thin wrappers. CI runs `just ci` and nothing
else. Keeps the dev surface uniform and the docs honest.

### Version centralization: one writer, one guard (#59)
`just set-version X.Y.Z` (`codegen/version.sh`) is the **single writer** of the version
across the root Cargo.toml, every hand-written companion manifest, plugin.json, and the
.mcp.json npx pin — self-verifying each rewrite. `just version-check` is the **single drift
guard** (same script, check mode) and round-trips the writer against a temp copy, so a broken
rewriter fails CI on the PR that broke it. Replaced a sprawl of per-file grep guards.

The **generated** breadth clients also carry the version, but the writer for those is
codegen, not `version.sh`: openapi-generator stamps `npmVersion`/`packageVersion`/
`artifactVersion` at generation time. So `just set-version` also runs `just gen` (alongside
`gen-completions`), keeping the generated clients in lockstep — otherwise a bump drifts them
until the next `just gen`, which is exactly what the v0.2.0 release shipped and `gen-check`
then caught on the next PR. Consequence: the release toolchain (and the Cut-release Action)
carries the codegen deps (nightly rustfmt + progenitor + openapi-generator), since a release
now regenerates.

### MSRV floats with recent stable
`rust-version` currently **1.89** (for `std::fs::File::lock`, chosen over a locking
dependency). Pre-1.0, no shipped consumers; revisit only when a real consumer needs older.

### Documentation site: Astro Starlight on GitHub Pages (`docs-site/`)
The docs website ([ouratoolkit.com](https://ouratoolkit.com)) is **Astro Starlight** — modern,
fast, built-in Pagefind search + dark mode, MD/MDX authoring — published to **GitHub Pages**
(self-contained, CI-driven, no external service; apex domain via `public/CNAME`). Chosen over
Docusaurus (heavier) and VitePress (weaker OpenAPI story). Wired to the source of truth so the
docs can't drift, matching the repo's law:
- **API reference** via the `starlight-openapi` plugin, fed the **overlaid** spec (so the
  server URL is `api.ouraring.com`, not the leaked `api.None.com`). A docs-only jq step
  (`codegen/docs-spec.jq`, run by `just docs-spec`) applies two presentation fixes — normalizing
  the spec's `x-codeSamples` language labels ("cURL"/"Python"/…) to Shiki grammar ids for
  highlighting, and trimming the spec's 101-level "Getting Started" intro ("What is an API?", a
  Quick Start Guide) from `info.description` since the docs site targets developers and has its
  own Guides — kept OUT of the shared `codegen/overlay.jq`, which feeds real codegen and must
  stay minimal. Starlight 3.1 support was verified against the pinned plugin (no down-convert
  needed).
- **CLI reference** generated from the `oura` binary's `--help` by `just docs-gen-cli`,
  committed + drift-checked (`docs-gen-cli-check`). **Rejected** adding a `clap-markdown` hidden
  subcommand: `clap_complete` still emits hidden subcommands into the shell completions, leaking
  an internal `oura markdown` command (with its docs-y description) into `oura <TAB>` — polluting
  the tightly-curated CLI surface. Capturing `--help` verbatim needs no new dep, no new
  subcommand, and no lib refactor, and mirrors how `gen-completions` already shells out to the
  binary. A completeness tripwire guards the enumeration.
- **Guides + SDK pages** hand-written; their enumerable claims are pinned by docs-site tripwires
  in `docs_tripwire.rs` (the #45 pattern extended to `docs-site/`). A PR build gate
  (`just docs-check`) is a CI job, so "green CI is releasable" covers the docs too.
Everything is a `just docs-*` recipe (`[group('docs')]`); raw npm/astro/jq stay inside them.

---

## Toolchain realities (do not relitigate — these make the mandated paths work)

### progenitor reads OpenAPI 3.0 only → Rust-only down-convert
The vendored spec is 3.1; progenitor reads 3.0. `codegen/progenitor-downconvert.jq` (run in
`just gen-rust` *after* the shared overlay) relabels `3.1.0`→`3.0.3`, rewrites the
`anyOf:[X,{type:null}]` nullable idiom to `nullable:true`, and prunes content-less 4xx
responses (progenitor asserts one error type per op). The **shared** overlay stays 3.1 so the
3.1-native breadth generators keep full fidelity.

### progenitor needs a pinned nightly rustfmt
progenitor formats with unstable rustfmt opts (`wrap_comments`, `normalize_doc_attributes`) →
needs **nightly rustfmt**, and those opts once corrupted a doc comment into invalid Rust.
`just gen-rust` runs progenitor through `codegen/rustfmt-shim.sh` on a **pinned dated
nightly** (the `nightly_rustfmt` justfile variable — single source for setup, gen-rust, and
CI), then re-formats with stable `cargo fmt`. Pinning keeps codegen deterministic — a
floating nightly could flake an unrelated PR's gen-check. The committed generated crate builds
on **stable**; only *regeneration* needs nightly + progenitor.

### progenitor takes a plain `reqwest::Client` (not middleware)
progenitor 0.14 does not accept a `reqwest_middleware::ClientWithMiddleware`. So the data
plane hands the generated client a plain `reqwest::Client` preconfigured with a fresh Bearer,
and retries once on 401 via `TokenManager::force_refresh`. The companion still ships the
reqwest-middleware `AuthMiddleware` for middleware consumers, but that is not the progenitor
path. Version matrix: cargo-progenitor/progenitor-client 0.14 need reqwest ^0.13 (feature
`rustls`, not `rustls-tls`); the workspace pins reqwest 0.13.

### rmcp `#[tool_handler]` router gotcha
`#[tool_handler]` defaults to a FRESH `Self::tool_router()` per request — it must be
`#[tool_handler(router = self.tool_router)]` or the build-time descriptions injected at
construction silently vanish (the `#[tool]` attr only takes literal descriptions, so the
build-time ones are injected into the router's public `map` in `OuraMcp::new`).

### openapi-generator per-language post-patches (all guarded)
The generator is pinned **two ways** for reproducibility: the npm wrapper
`@openapitools/openapi-generator-cli@2.39.1` (the `oag` justfile var), which in turn resolves
the generator jar to **7.14.0** via `codegen/openapitools.json`. Doc/test emission is
suppressed with a verified CLI flag set (`oag_skip_docs` —
`--global-property=apiDocs=false,…`) rather than config, which 7.14.0 ignores. The generator
emits broken packaging in several languages; each fix is a guarded post-patch that fails the
recipe if it stops applying:
- **Python**: root metadata mis-names the dist and won't build → `sdks/python`'s pyproject +
  namespace `__init__.py` are hand-written; `just gen-py` copies only the generated subtree.
- **C#**: emits a single netstandard2.0 target and rejects net10.0 → `just gen-csharp`
  post-patches the csproj to multi-target `netstandard2.0;net8.0;net10.0` (LangVersion 13.0)
  and strips bogus `System.Web` refs.
- **TypeScript**: typescript-fetch ships a Node-broken dual-ESM build → `just gen-ts`
  post-patches package.json to the auth companion's CJS-only + exports-map shape and drops
  `tsconfig.esm.json` (#57).

### C# netstandard branches run under Mono (#61)
A modern .NET host can't load the netstandard2.0 asset, so its `#if NETSTANDARD2_0` store/
transport branches never execute there. `just sdk-test-csharp-netstandard` builds the suite
for net472 and runs it under **Mono** (its own `csharp-netstandard` CI job), so those
branches actually run — TESTING rule 6 ("platform code runs on its platform"). A
`BuildInfoTests` marker fails the leg if it ever loads the wrong asset.

---

## Lessons from the review loops (bugs the process caught)

### Cross-language auth conformance is one fixture (#58)
Four independent companion review loops found the **same two bug families**.
`codegen/conformance/auth-cases.json` is now the SINGLE SOURCE for hostile token-endpoint
responses, hostile store files, and canonical store records — all six companion suites
(Rust reference included) iterate it from the file. A new hostile case goes in the fixture,
never one language's suite; a failing companion gets fixed, the fixture is never weakened.

### serde_json alphabetizes — don't route the text block through `to_value` (#40)
`serde_json::Value` is a `BTreeMap` (no `preserve_order` feature), so round-tripping the MCP
text block through `to_value` alphabetized the keys. Fix: serialize the text block from the
typed `&data` directly; use `to_value` **only** for the structured envelope.

### `oura api` is Bearer-only by construction (#19)
The passthrough attaches only the Bearer token, so it reaches the read (GET) API. Oura's
write endpoints (webhook subscriptions) use a separate client-id/secret scheme it does not
send — the original docs' `-X POST /v2/webhook/subscription` examples could never have
worked. `oura api` is a *caller* of the generated data plane's auth+retry contract, **not** a
second typed SDK; do not generalize the carve-out to typed commands.

### Static-token path short-circuits refresh (#20)
`OURA_ACCESS_TOKEN` builds a `TokenManager::from_access_token` with `expires_at = i64::MAX`,
no credentials, and an `env_token` flag; a forced refresh short-circuits to
`AuthError::StaticTokenRejected` *before* the credentials check, because an injected token
has nothing to refresh with.

### Spec-drift detection was a silent-bug risk → self-tested (#29)
The weekly drift detector's decision logic is guarded hermetically by
`just spec-drift-selftest` in CI — the detector is watch-only, but its logic can't be allowed
to rot silently.

### cargo-dist 0.32 Homebrew limit (#75, still open)
cargo-dist 0.32's `include` ships the man page + completions into every archive (verified
against a real host tarball), but its Homebrew formula template has **no** completion/manpage
wiring and no config for it — the files land in the formula's `pkgshare`, not auto-loaded.
Archive inclusion is done; Homebrew auto-wiring is deferred to a cargo-dist upgrade. #75 stays
open for exactly that half.
