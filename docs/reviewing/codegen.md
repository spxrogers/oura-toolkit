# Reviewing the spec, codegen and the SDKs

The law is `CLAUDE.md` → *Spec is the source of truth*, *Client / SDK generation* and *Naming*;
`DECISIONS.md` → *Toolchain realities* says why each pin and post-patch exists.

## Never

- A hand edit to generated code: `sdks/rust/oura-toolkit-api`, `sdks/<lang>/api`, the CLI
  reference page. A fix to generated output is a fix to the generator's input, the overlay or a
  guarded post-patch.
- `just gen` changing a hand-written auth companion or hand-written distribution metadata.
- A second generated Rust client. `sdks/rust/oura-toolkit-api` is the Rust SDK and the CLI
  depends on it.
- Rust given a privileged place in the layout. Every language sits under `sdks/<lang>/` with a
  generated client beside a hand-written companion; the CLI is an app under `cli/`.
- A locked name "improved": no package or module called `sdk`, no `-cli` suffix on the binary,
  no per-function Python packages. The binary is `oura`; the crate is `oura-toolkit-cli`.
- The overlay's `servers[0].url` fix removed because upstream fixed it. It stays as a
  regression guard.

## Check

- **The overlay runs before any generation**, for all languages, and is OpenAPI 3.1. Its four
  jobs: pin the server URL, strip the `MultiDocumentResponseDict` branch, narrow the client's
  per-operation security to Bearer, collapse `start_date` and `end_date` to plain `date`.
- **The 3.1 to 3.0 down-convert is Rust-only** and runs after the shared overlay.
- **Generated output is in sync:** `just gen-check`. Generated crates and directories are
  marked (`// @generated`, `linguist-generated=true`).
- **A spec bump** goes through the documented upgrade procedure in `CONTRIBUTING.md`, and
  `just spec-fetch` plus `just gen` reproduce the tree.
- **A post-patch fails its recipe when it stops applying.** A patch that silently matches
  nothing is the defect to look for.
- **Tool versions come from the tool's docs,** not from memory: progenitor, rmcp, cargo-dist.

## Gotchas this repo has met

- **The pinned export stops exercising a fix.** Once upstream fixed the server URL, the pinned
  spec no longer proved the overlay did anything. `just spec-overlay-selftest` runs it against
  a synthetic bad spec. Ask the same of any overlay or post-patch rule: what proves it still
  works when the input no longer needs it?
- **Oura serves only its latest export.** A new version makes every older URL 404. The drift
  detector reports that as drift and still runs its probe; only a network or 5xx failure is an
  error. `just spec-drift-selftest` guards the logic.
- **progenitor formats with nightly rustfmt options** that once produced invalid Rust. The
  nightly is pinned by date in one justfile variable; a floating one can fail an unrelated PR.
- **progenitor takes a plain `reqwest::Client`,** not a middleware client, so the data plane
  retries once on 401 itself.
- **openapi-generator emits broken packaging** in Python, C# and TypeScript. Each fix is a
  guarded post-patch; `DECISIONS.md` lists them.
- **C# `netstandard2.0` branches do not run on a modern .NET host.** They run under Mono in
  their own CI job, and the runner's `-noappdomain` flag is load-bearing.
