# Architecture — oura-toolkit

How the pieces fit together. This is the **map**; [CLAUDE.md](CLAUDE.md) is the **law**
(the hard constraints every change must satisfy) and [DECISIONS.md](DECISIONS.md) is the
**log** (why each locked choice was made, and the hard-won toolchain lessons behind it).
When this doc and CLAUDE.md disagree on a rule, CLAUDE.md wins.

## What it is

**oura-toolkit** is a Rust CLI for the [Oura Ring API v2](https://api.ouraring.com/v2),
plus a STDIO MCP server, a single Claude plugin, and auto-generated SDK clients in six
languages. It's a cargo workspace and a plugin marketplace in one monorepo.

One idea drives the whole layout: **Oura's OpenAPI spec is the source of truth, and every
language ships the same shape** — a generated data-plane client plus a hand-written auth
companion. Rust is not privileged; it just also happens to host the CLI app.

## The pipeline (spec → clients)

```
spec/openapi.json  (vendored, pinned: openapi-1.35.json, OpenAPI 3.1)
      │   just spec-fetch
      ▼
codegen/overlay.jq  ── shared overlay (3.1, all languages) ──┐
      │   just spec-overlay                                   │
      ▼                                                       ▼
progenitor down-convert (3.1→3.0, Rust only)      openapi-generator 7.14.0
      │   just gen-rust                              │   just gen-ts/py/go/java/csharp
      ▼                                              ▼
sdks/rust/oura-toolkit-api  (the Rust SDK)   sdks/{typescript,python,go,java,csharp}/api
```

- **Shared overlay** (`codegen/overlay.jq`) fixes the spec's known defects for *every*
  language: the leaked `api.None.com` server URL, the `MultiDocumentResponseDict` union
  branch, per-op security narrowed to Bearer, and the `start_date`/`end_date` param types
  collapsed to plain `date`. See [CLAUDE.md → SPEC / Known spec issues](CLAUDE.md).
- **Rust** goes one step further with a 3.1→3.0 down-convert, because progenitor only reads
  3.0. The breadth generators are 3.1-native and skip it. Details and rationale in
  [DECISIONS.md](DECISIONS.md).
- **Generated code is never hand-edited.** CI's `just gen-check` regenerates and diffs; a
  hand edit fails the build. Generated crates/dirs are marked (`// @generated`,
  `linguist-generated`).

## Rust workspace

Three crates (`Cargo.toml` workspace members):

| Crate | Dir | Role |
|---|---|---|
| `oura-toolkit-api` | `sdks/rust/oura-toolkit-api` | GENERATED progenitor client = the Rust SDK. Dual-use: the CLI depends on it directly (dogfooding = free integration test). |
| `oura-toolkit-auth` | `sdks/rust/oura-toolkit-auth` | Hand-written, non-interactive auth companion: token store, refresh-with-rotation, cross-process lock, Bearer middleware, OAuth metadata read from the spec. |
| `oura-toolkit-cli` | `cli/oura-toolkit-cli` | The app. Binary `oura`. Interactive OAuth, data commands, `oura api` passthrough, `oura mcp` server. Depends on both crates above. |

The auth companion is depended on by **both** the CLI's data commands **and** the MCP
tools — one auth layer, two consumers.

## Auth architecture

Oura is a **confidential OAuth2 client** (Authorization Code flow only; personal access
tokens were deprecated Dec 2025; no PKCE). Because a client_secret can't be safely embedded
in a distributed binary, the model is **bring-your-own-credentials**: each user registers
their own Oura OAuth app.

- **Interactive consent** (browser open + loopback callback listener) lives **only** in the
  CLI (`oura auth setup` / `oura auth login`). No SDK/companion contains it.
- **Non-interactive machinery** lives in `oura-toolkit-auth`, three pieces:
  - a **token store** at the locked config path — two records (`credentials.json` +
    `tokens.json`) so a failed login never loses the pasted secret;
  - **`TokenManager`**, the refresh engine — proactive refresh, one 401→refresh→retry on the
    data plane, and refresh-token **rotation persisted**;
  - a **cross-process lock** so the CLI and the long-running MCP server can share one store
    without clobbering each other's rotations.
- **Headless / CI**: `OURA_ACCESS_TOKEN` injects a raw Bearer token that bypasses the store
  entirely (`TokenManager::from_access_token`, never refreshes); `--no-browser` swaps the
  loopback catch for a paste-the-redirect-URL flow (still CSRF-checked via `state`).

This is the shape only. The exact store paths, file modes, default port, rotation and
locking protocol are the load-bearing invariants — stated once, authoritatively, in
[CLAUDE.md → AUTH](CLAUDE.md) (and tripwire-pinned there against the code); the reasoning is
in [DECISIONS.md](DECISIONS.md). This map deliberately does not restate the literals so there
is one source to keep true.

## CLI surface

Binary `oura`. Eight read commands (`sleep`, `readiness`, `activity`, `stress`,
`heartrate`, `sessions`, `workouts`, `personal-info`), each windowed with `--start`/`--end`
(or `--date` shorthand) and auto-paginating. Plus:

- `oura auth setup|login|status|token|refresh|logout` — account management.
- `oura api <path>` — authenticated raw passthrough (like `gh api`), the **one sanctioned**
  hand-written-transport carve-out (a user path has no generated operation). Bearer-only.
- `oura mcp` — the STDIO MCP server.
- `oura completion <shell>` / `oura man` — generators for the shipped completions + man page.

Output adapts to context: aligned tables on a TTY, TSV when piped, `--json` on request.
Exit codes are a documented contract (`0` ok, `1` runtime, `2` usage, `4` auth needed) —
see [docs/cli-contract.md](docs/cli-contract.md), enforced at the binary level.

## MCP architecture

`oura mcp` runs a **STDIO** server (official Rust SDK `rmcp`) exposing ~8 curated tools.
Tools are a **hybrid spec-codegen**, not `rmcp-openapi` (see [DECISIONS.md](DECISIONS.md)
for why):

- **Build-time** (`build.rs`): each tool description = curated LLM lead + the operation's
  spec summary + the response field inventory. A curated op vanishing from the spec fails
  the build.
- **Dispatch** is hand-wired to the same `commands::fetch_*` functions the CLI renders from
  — one auth layer, one data plane, two presentations.
- **Results** are the progenitor models serialized to JSON: a text block plus a
  `structured_content` value (arrays enveloped under `data`, objects pass through).
- **Auth**: reads the shared store, refreshes silently on 401. Absent tokens do **not**
  fail the handshake, never prompt, never write to stdout (it's the JSON-RPC transport) —
  the first tool call returns a structured "run `oura auth login`" error.

## Distribution & plugin

- **cargo-dist** (pinned 0.32.0) emits shell + powershell + npm + homebrew installers across
  5 targets. Release archives also ship the man page + shell completions (committed under
  `cli/oura-toolkit-cli/dist-assets/`, drift-checked).
- **Releases are tag-driven**: `just set-version X.Y.Z` (the single version writer) → commit
  → tag `vX.Y.Z` → push. The one tag fans out to every publish channel in CI: `release.yml`
  (installers + homebrew), `publish-crates.yml` (crates.io via OIDC), `publish-sdks.yml`
  (breadth SDKs — `@oura-toolkit/api` + `@oura-toolkit/auth` on npm, `oura-toolkit` on
  PyPI, and the `sdks/go/vX.Y.Z` sub-tag that versions the Go module, #96) and, chained
  off release.yml's completion, `publish-cli-npm.yml` (the CLI's `oura-toolkit` npm launcher,
  OIDC, from the hosted release tarball). Every npm/crates publish is Trusted Publishing — the
  only stored publish secret is the Homebrew tap token.
- **Launch is NPX-first**: `npx -y oura-toolkit ...`; brew / bun are speed-path alternatives.
- **One Claude plugin** (`plugins/oura-toolkit/`) ships both the MCP server config
  (`npx -y oura-toolkit@<version> mcp`) and skills (`morning-checkin`, `wellness-report`).
  The marketplace manifest lives at the repo root (`.claude-plugin/marketplace.json`).

## CI (the release gate)

Green CI **is** the release decision — "if all tests pass, this tool is ready to release."
CI invokes `just ci` (fmt-check + lint + test) plus dedicated jobs: coverage floor,
gen-drift (regenerate + diff), sdk-check (all six clients compile), the C# `netstandard`
Mono leg, release-config (`dist-check` / `version-check` / `plugin-check` /
`gen-completions-check`), across an ubuntu/macos/windows matrix. The testing law that makes
this trustworthy is [CLAUDE.md → TESTING & VERIFICATION](CLAUDE.md).

## Documentation site

The public docs website ([ouratoolkit.com](https://ouratoolkit.com)) is an
[Astro Starlight](https://starlight.astro.build) project under `docs-site/`, published to
**GitHub Pages** (apex domain pinned by `docs-site/public/CNAME`; deploy on push to `main` via
`.github/workflows/docs-deploy.yml`). It is wired into the same source-of-truth discipline as
the rest of the repo, so the docs can't drift from the code:

- **API reference** — generated at build time by `starlight-openapi` from the overlaid spec
  (`just docs-spec` → `codegen/build/openapi.docs.json`; `codegen/docs-spec.jq` applies docs-only
  transforms: `x-codeSamples` language labels normalized for highlighting, and the spec's
  101-level "Getting Started" intro trimmed from `info.description`). It *is* the spec, so a
  spec/overlay problem fails the build.
- **CLI reference** — generated from the `oura` binary's own `--help` by `just docs-gen-cli`,
  committed at `docs-site/src/content/docs/cli/reference.md` and drift-checked by
  `just docs-gen-cli-check` (same doctrine as the completions/man page).
- **Guides + SDK pages** — hand-written, with every enumerable claim (scopes, ports, store
  paths, MCP tool names, env overrides, rate-limit numbers, the SDK language set) pinned to
  source by the docs-site tripwires in `cli/oura-toolkit-cli/tests/docs_tripwire.rs`.

Everything goes through `just docs-*` recipes (a `[group('docs')]`); a PR build gate
(`just docs-check`) runs in CI. Built-in Pagefind search, dark mode, and versioned nav come
from Starlight. The page footer's "Last updated" line also carries the **deployed commit's
short SHA**, linked to the exact commit on GitHub — a "what's currently live" breadcrumb. The
SHA is resolved at build time in `docs-site/astro.config.mjs` (CI's `GITHUB_SHA`, falling back
to the local `git` HEAD) and rendered by the `LastUpdated` component override in
`docs-site/src/components/`; if neither resolves the hash is simply omitted.

## Repo layout

```
oura-toolkit/
├── CLAUDE.md              # the law: hard constraints for every change
├── ARCHITECTURE.md        # this file: how the pieces fit
├── DECISIONS.md           # the log: why the locked choices, + toolchain lessons
├── README.md              # front door (all commands shown as `just …`)
├── justfile               # THE single task runner for the whole repo
├── Cargo.toml             # workspace
├── spec/openapi.json      # vendored Oura spec, pinned (just spec-fetch)
├── sdks/
│   ├── rust/oura-toolkit-api    # GENERATED (progenitor) = Rust SDK
│   ├── rust/oura-toolkit-auth   # hand-written Rust auth companion
│   ├── typescript/  python/  go/  java/  csharp/   # api/ GENERATED + auth/ companion
├── cli/oura-toolkit-cli   # the app (binary `oura`)
├── .claude-plugin/marketplace.json   # at repo root (marketplace schema requires it)
├── plugins/oura-toolkit/  # single plugin: plugin.json + .mcp.json + skills/
├── docs-site/             # docs website (Astro Starlight → ouratoolkit.com)
├── codegen/               # overlays + generator configs + conformance fixtures
└── dist-workspace.toml    # cargo-dist config
```
