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
just sdk-check      # CI's compile check: every breadth client actually builds
just test-sandbox-sdks  # opt-in live sandbox smokes for the breadth clients (network)
```

Codegen touches **only** generated clients — never the hand-written auth companions,
`sdks/go/go.mod`, or `sdks/python`'s distribution metadata.

The C# recipes (`just sdk-check-csharp` / `just sdk-test-csharp` / `just gen-csharp`)
need a **.NET 10 SDK**: the C# client and auth companion multi-target
`netstandard2.0;net8.0;net10.0`, and an 8.x/9.x SDK cannot build the net10.0 leg.
`just setup` warns if it is missing.

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

## Releases (maintainers)

Tag-driven, never from a laptop: run `just set-version X.Y.Z` (the single writer — it
bumps the root `Cargo.toml` source plus every hand-written manifest that carries the
version, and refreshes `Cargo.lock`), commit, tag `vX.Y.Z`, push. CI builds every
installer and publishes npm + Homebrew. `just version-check` (the single drift guard),
`just dist-check`, `just publish-check` and `just plugin-check` guard the release
config on every PR; `just release` is a local smoke build and `just publish` covers
crates.io. Details in [CLAUDE.md → DISTRIBUTION](CLAUDE.md).

## Repo map

```
spec/                      vendored Oura OpenAPI spec (just spec-fetch)
codegen/                   overlays, generator configs + sandbox smoke scripts
sdks/rust/oura-toolkit-api   GENERATED Rust client — do not hand-edit
sdks/rust/oura-toolkit-auth  hand-written auth companion (token store, refresh)
sdks/{typescript,python,go,java,csharp}  GENERATED breadth clients + hand-written auth companions
cli/oura-toolkit-cli         the app: binary `oura` (CLI + MCP server)
plugins/oura-toolkit/        the Claude plugin (MCP entry + skills)
docs/cli-contract.md         the scripting contract (exit codes, streams, formats)
CLAUDE.md                    architecture decisions + the testing law
```
