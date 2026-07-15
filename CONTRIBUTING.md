# Contributing to oura-toolkit

Thanks for helping! Two rules carry most of this document:

1. **Everything goes through `just`.** There is exactly one [justfile](justfile), at the
   repo root. Building, testing, linting, codegen, releasing — all of it is a recipe.
   If you find yourself typing a raw tool command from the docs, that's a bug in the
   docs.
2. **Generated code is never hand-edited.** `sdks/rust/oura-toolkit-api/` is produced
   from the vendored OpenAPI spec; CI's drift check rejects hand edits. Change the
   generator inputs (`spec/`, `codegen/`) and regenerate instead.

## Getting started

With a [Rust toolchain](https://rustup.rs) on hand, install
[`just`](https://github.com/casey/just) (the one bootstrap step it can't do for itself),
then:

```sh
just setup   # extra toolchain components + codegen/release tooling (assumes rustup)
just ci      # fmt-check + clippy + full test suite — exactly what CI runs
```

## The dev loop

```sh
just build          # workspace build
just test           # hermetic tests (wiremock HTTP, tempdir stores — no network)
just fmt            # format
just lint           # clippy, warnings are errors
just coverage       # line-coverage floor on the hand-written crates
just ci             # the merge gate: fmt-check + lint + test
```

Trying your changes live:

```sh
just auth-setup     # guided Oura app registration + login
just auth-login     # login only
just mcp            # run the CLI as a STDIO MCP server
just test-sandbox   # opt-in tests against Oura's live sandbox (network, no credentials)
```

## Codegen (spec is the source of truth)

```sh
just spec-fetch     # re-vendor the pinned Oura OpenAPI export (+ crate-local bundles)
just gen            # regenerate ALL generated SDK clients (Rust, TS, Python, Go, Java, C#)
just gen-check      # CI's drift check: committed generated code matches the spec
just gen-completions # shell completions + man page for the release archives (#75)
just sdk-check      # CI's compile check: every breadth client actually builds
just test-sandbox-sdks  # opt-in live sandbox smokes for the breadth clients (network)
```

Codegen touches **only** generated clients — never the hand-written auth companions,
`sdks/go/go.mod`, or `sdks/python`'s distribution metadata.

`just gen-completions` regenerates the committed `cli/oura-toolkit-cli/dist-assets/`
(shell completions + `oura.1`) that `dist-workspace.toml` `include`s into every release
archive. It's drift-checked by `just gen-completions-check` (the `release-config` CI job); run it
after any change to the CLI surface. The man page's `.TH` embeds the version, so
`just set-version` regenerates it for you (it runs `gen-completions` as part of #59's single
version writer). For the same reason `set-version` also runs `just gen`: the generated breadth
clients stamp the version at codegen time (openapi-generator's `npmVersion`/`packageVersion`/
`artifactVersion`), so a bump would drift them until the next `just gen` otherwise.

### Upgrading the vendored spec

The spec is pinned to a specific export (`spec_version` in the justfile). A scheduled
workflow (`.github/workflows/spec-drift.yml`, `just spec-drift-check`) watches upstream and
opens a `spec-drift` issue when the pinned export re-publishes or a newer
`openapi-<major>.<minor>` appears. To adopt a change:

```sh
just spec-drift-check   # what drifted (also runs weekly; needs network, not in `just ci`)
# bump `spec_version` in the justfile first if a newer export exists, then:
just spec-fetch         # re-vendor to spec/openapi.json (+ crate-local bundles)
just spec-overlay
just gen                # regenerate every client; run twice for the zero-diff check
```

Review the generated diff (the overlays + down-convert absorb most upstream churn) and land
it through the usual review loop. `just spec-drift-selftest` (hermetic) guards the detector.

The C# recipes (`just sdk-check-csharp` / `just sdk-test-csharp` / `just gen-csharp`)
need a **.NET 10 SDK**: the C# client and auth companion multi-target
`netstandard2.0;net8.0;net10.0`, and an 8.x/9.x SDK cannot build the net10.0 leg.
`just setup` warns if it is missing.

`just sdk-test-csharp` runs the auth suite on net8.0 + net10.0 only — a modern .NET host
cannot load the `netstandard2.0` asset, so its `#if NETSTANDARD2_0` branches never execute
there. `just sdk-test-csharp-netstandard` closes that gap: it builds the same suite for
net472 (which resolves the `netstandard2.0` asset) and runs it under **Mono** (`apt:
mono-devel`; `just setup` warns if missing). CI runs it as its own `csharp-netstandard`
job.

## Docs site (docs-site/)

The documentation website ([ouratoolkit.com](https://ouratoolkit.com)) is an
[Astro Starlight](https://starlight.astro.build) project under `docs-site/`, driven by the
same sources as the code so it can't drift.

```sh
just docs-dev       # local dev server (overlaid spec + fresh CLI reference first)
just docs-build     # production build to docs-site/dist/ (Pagefind search index)
just docs-gen-cli   # regenerate the committed CLI reference from the `oura` binary
just docs-check     # the docs CI gate: CLI-reference drift check + full build
```

Two things are generated, never hand-edited: the **API reference** is built from the overlaid
spec at build time (so it *is* the spec), and the **CLI reference**
(`docs-site/src/content/docs/cli/reference.md`) is generated from the `oura` binary by
`just docs-gen-cli` and drift-checked by `just docs-gen-cli-check` — regenerate and commit it
after any CLI-surface change. The hand-written guide/SDK pages' enumerable claims (scopes,
ports, store paths, tool names, env vars, rate-limit numbers, the SDK language set) are pinned
to source by the docs-site tripwires in `cli/oura-toolkit-cli/tests/docs_tripwire.rs`, so a code
change that orphans the site fails `just ci`. Deploy is automatic: pushing to `main` runs
`.github/workflows/docs-deploy.yml` (GitHub Pages).

Each page's footer shows the **deployed commit's short SHA**, linked to the commit on GitHub —
a "what's currently live" breadcrumb. It comes from CI's `GITHUB_SHA` (or your local `git` HEAD
when you run `just docs-build`), resolved in `docs-site/astro.config.mjs` and rendered by the
`LastUpdated` override in `docs-site/src/components/`.

## Testing bar (the release gate)

This repo treats green CI as the release decision, which puts real weight on test
quality — the full law lives in [CLAUDE.md → TESTING & VERIFICATION](CLAUDE.md). The
short version reviewers will hold you to:

- **Every documented guarantee has an enforcing test.** A promise without a test is
  prose.
- **Break-verify your guard tests**: sabotage the code, watch the test fail with a
  message naming the contract, restore. A guard that can't fail is false confidence.
- **Assertions must be load-bearing.** Ask "what wrong implementation would still pass
  this?" — if the answer is "plenty", tighten it.
- `just test` stays **hermetic**: wiremock for HTTP, tempdirs for state, no real
  credentials, no network.
- Platform-specific code ships with tests that run on that platform's CI leg
  (ubuntu / macos / windows all run `just ci`).
- Auth companions (all six languages) iterate the shared conformance fixture
  `codegen/conformance/auth-cases.json` — hostile token responses, hostile store files,
  canonical store records. Add new hostile cases there, never to one language's suite.

## Releases (maintainers)

Tag-driven, never from a laptop: `just release X.Y.Z` does the whole thing in one
command — runs the full local gate, then bumps every manifest (`just set-version`, the
single writer — it rewrites the root `Cargo.toml` source plus every hand-written manifest
that carries the version, refreshes `Cargo.lock`, and regenerates the completions + the
version-stamped SDK clients via `just gen`), commits, and pushes the `vX.Y.Z`
tag. That one tag drives **every** publish channel in CI: `release.yml` builds every
installer and publishes npm + Homebrew, and `publish-crates.yml` publishes the crates to
crates.io via Trusted Publishing (OIDC, no token — #91). Nothing publishes from your
laptop; guards refuse a dirty tree, a non-`main` branch, drift from origin, or an existing
tag. (The manual path is the same steps by hand: `just set-version X.Y.Z`, commit, tag,
push. `just publish` remains the manual crates.io fallback, needing `cargo login`.)
Prefer a button? The **Cut release** GitHub Action (`workflow_dispatch`) runs the same
choreography server-side via the shared `just release-tag` recipe — enter a version, and it
gates, bumps, commits, and pushes the tag. It needs a `RELEASE_TOKEN` PAT secret so the
pushed tag can trigger the release build.
`just version-check` (the single drift guard), `just dist-check`, `just publish-check` and
`just plugin-check` guard the release config on every PR; `just dist-build` is a local smoke
build and `just publish` is the manual crates.io fallback (CI publishes crates via
`publish-crates.yml`). Details in [CLAUDE.md → DISTRIBUTION](CLAUDE.md).

## Repo map

```
spec/                      vendored Oura OpenAPI spec (just spec-fetch)
codegen/                   overlays, generator configs + sandbox smoke scripts
sdks/rust/oura-toolkit-api   GENERATED Rust client — do not hand-edit
sdks/rust/oura-toolkit-auth  hand-written auth companion (token store, refresh)
sdks/{typescript,python,go,java,csharp}  GENERATED breadth clients + hand-written auth companions
cli/oura-toolkit-cli         the app: binary `oura` (CLI + MCP server)
plugins/oura-toolkit/        the Claude plugin (MCP entry + skills)
docs-site/                   the docs website (Astro Starlight -> ouratoolkit.com)
docs/cli-contract.md         the scripting contract (exit codes, streams, formats)
CLAUDE.md                    the law: hard constraints every change must satisfy
ARCHITECTURE.md              the map: how the pieces fit together
DECISIONS.md                 the log: why each locked choice was made
```
