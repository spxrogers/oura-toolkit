# oura-toolkit — THE single task runner for the whole repo.
#
# There is exactly ONE justfile, at the repo root. Every dev/release command goes through
# `just <recipe>`; raw tools (cargo, curl, jq, npm, progenitor, dist) are invoked ONLY inside
# these recipes and are never documented or run directly. Recipes are thin wrappers.
#
# Prerequisite: install `just` itself (it can't install itself), then everything is `just …`.
# See CLAUDE.md for the architecture and the hard "do not" list.

# ---------------------------------------------------------------------------------------------
# Variables (spec URL/version + paths)
# ---------------------------------------------------------------------------------------------

# Pinned Oura OpenAPI export (v3.1.0, title "Oura API Documentation", version 2.0).
spec_version  := "openapi-1.35"
spec_url      := "https://api.ouraring.com/v2/static/json/" + spec_version + ".json"

# Pristine vendored spec (committed) and the derived overlay output (gitignored).
spec_file       := "spec/openapi.json"
build_dir       := "codegen/build"
overlaid_spec   := build_dir / "openapi.overlaid.json"
# Rust-only down-convert (3.1 -> 3.0.3) fed to progenitor.
progenitor_spec := build_dir / "openapi.progenitor.json"

# Workspace version, derived from Cargo.toml [workspace.package].version (single source).
version := `sed -nE 's/^version = "([^"]+)"$/\1/p' Cargo.toml | head -n1`

# Release-gate line-coverage floor (percent) for the HAND-WRITTEN crates; the generated
# client is excluded (exercised via consumers + the drift check). Lowering this is a
# deliberate reviewed decision; raise it after test additions (ratchet). See CLAUDE.md
# TESTING & VERIFICATION.
coverage_floor := "85"

# Show available recipes (default recipe).
default:
    @just --list --unsorted

# ---------------------------------------------------------------------------------------------
# Setup
# ---------------------------------------------------------------------------------------------

# Install/verify toolchains + codegen deps.
[group('setup')]
setup:
    rustup component add rustfmt clippy llvm-tools-preview
    # Rust codegen: progenitor CLI + a nightly rustfmt (progenitor formats with unstable opts).
    rustup toolchain install nightly --profile minimal --component rustfmt
    cargo install cargo-progenitor --locked
    # Coverage floor enforcement (`just coverage`).
    command -v cargo-llvm-cov >/dev/null || cargo install cargo-llvm-cov --locked
    @command -v jq  >/dev/null || echo "!! install jq -- needed by 'just spec-overlay' / 'just gen-rust'"
    @command -v npx >/dev/null || echo "!! install node/npx -- needed by breadth-SDK codegen"
    # Release tooling (`just dist-check` / `just release`).
    command -v dist >/dev/null || cargo install cargo-dist --locked

# ---------------------------------------------------------------------------------------------
# Spec (source of truth)
# ---------------------------------------------------------------------------------------------

# Re-fetch the pinned Oura OpenAPI export to spec/openapi.json (kept pristine).
[group('spec')]
spec-fetch:
    mkdir -p spec
    curl -fsS {{spec_url}} -o {{spec_file}}
    # Crate-local bundles: the spec-reading build scripts (oura-toolkit-auth, the CLI)
    # need the spec INSIDE the packaged crate on crates.io (no repo root to walk to
    # there). Sync is enforced by each crate's bundled-spec test (#11).
    cp {{spec_file}} sdks/rust/oura-toolkit-auth/openapi.json
    cp {{spec_file}} cli/oura-toolkit-cli/openapi.json
    @echo "Vendored {{spec_version}} -> {{spec_file}} (+ crate-local bundles)"

# Apply codegen overlays (fix servers url, strip dict union, narrow client security).
# Reads the pristine spec, writes the derived generation-input spec. Never edits the source.
[group('spec')]
spec-overlay:
    mkdir -p {{build_dir}}
    jq -f codegen/overlay.jq {{spec_file}} > {{overlaid_spec}}
    @echo "Overlaid spec -> {{overlaid_spec}}"

# ---------------------------------------------------------------------------------------------
# Codegen (generated SDK clients only — NEVER touches the *-auth companions)
# ---------------------------------------------------------------------------------------------

# Regenerate ALL generated SDK clients from the overlaid spec.
[group('codegen')]
gen: gen-rust gen-ts gen-py gen-go

# Generate the Rust SDK client (progenitor) -> sdks/rust/oura-toolkit-api/src/lib.rs.
# progenitor reads OpenAPI 3.0 only, so the overlaid 3.1 spec is down-converted first; its
# formatter needs nightly rustfmt via the shim (see codegen/rustfmt-shim.sh). The committed
# output builds on stable — only regeneration needs nightly + progenitor (installed by `setup`).
[group('codegen')]
gen-rust: spec-overlay
    mkdir -p {{build_dir}}
    jq -f codegen/progenitor-downconvert.jq {{overlaid_spec}} > {{progenitor_spec}}
    rm -rf {{build_dir}}/oura-toolkit-api-gen
    RUSTFMT="{{justfile_directory()}}/codegen/rustfmt-shim.sh" cargo progenitor -i {{progenitor_spec}} -o {{build_dir}}/oura-toolkit-api-gen -n oura-toolkit-api -v {{version}}
    cat codegen/generated-header.rs {{build_dir}}/oura-toolkit-api-gen/src/lib.rs > sdks/rust/oura-toolkit-api/src/lib.rs
    cargo fmt -p oura-toolkit-api
    @echo "Generated sdks/rust/oura-toolkit-api/src/lib.rs"

# Verify the committed generated client matches the current spec + overlays (CI drift check:
# catches hand-edits to the generated crate and spec/codegen drift). Needs `just setup`.
[group('codegen')]
gen-check: gen-rust
    git diff --exit-code -- sdks/rust/oura-toolkit-api

# Generate the TypeScript SDK client (openapi-generator) -> sdks/typescript/.
[group('codegen')]
gen-ts: spec-overlay
    @echo "TODO(#15): generate TypeScript client from {{overlaid_spec}} -> sdks/typescript/"

# Generate the Python SDK client (openapi-generator) -> sdks/python/.
[group('codegen')]
gen-py: spec-overlay
    @echo "TODO(#15): generate Python client from {{overlaid_spec}} -> sdks/python/"

# Generate the Go SDK client (openapi-generator) -> sdks/go/.
[group('codegen')]
gen-go: spec-overlay
    @echo "TODO(#15): generate Go client from {{overlaid_spec}} -> sdks/go/"

# ---------------------------------------------------------------------------------------------
# Build / test / quality
# ---------------------------------------------------------------------------------------------

# Build the workspace.
[group('build')]
build:
    cargo build --workspace

# Run unit tests.
[group('build')]
test:
    cargo test --workspace

# Integration tests against the LIVE Oura /v2/sandbox routes (canned data; needs network,
# no credentials — the sandbox accepts any bearer). #[ignore]d so `just test`/`just ci`
# stay hermetic; this recipe opts in.
[group('build')]
test-sandbox:
    cargo test -p oura-toolkit-cli --test sandbox -- --ignored

# Format sources.
[group('quality')]
fmt:
    cargo fmt --all

# Lint (clippy, warnings are errors).
[group('quality')]
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Type-check the workspace.
[group('quality')]
check:
    cargo check --workspace

# Remove build artifacts and generated codegen output.
[group('quality')]
clean:
    cargo clean
    rm -rf {{build_dir}} node_modules dist

# Enforce the release-gate coverage floor on hand-written crates (cargo-llvm-cov;
# generated oura-toolkit-api excluded). CI runs this as its own job.
[group('quality')]
coverage:
    cargo llvm-cov --workspace --all-targets \
        --ignore-filename-regex 'oura-toolkit-api/src/' \
        --fail-under-lines {{coverage_floor}} --summary-only

# What CI runs — and nothing else: fmt-check + lint + test.
#
# The env prefixes are build-speed config, not behavior: the release gate never uses
# debuginfo (failures print messages; nobody attaches a debugger to CI), and stripping it
# roughly halves compile+link time and cache size — the dominant cost of the Windows leg
# (MSVC PDB generation). Incremental artifacts are dead weight on throwaway runners.
# Scoped INSIDE the recipe so local `just ci` behaves identically to CI; `just test`/
# `just build` keep full debuginfo for real debugging.
[group('quality')]
ci:
    cargo fmt --all --check
    CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo clippy --workspace --all-targets -- -D warnings
    CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0 cargo test --workspace

# ---------------------------------------------------------------------------------------------
# Run / auth (local)
# ---------------------------------------------------------------------------------------------

# Run the CLI (oura) as a STDIO MCP server.
[group('run')]
mcp:
    cargo run -p oura-toolkit-cli -- mcp

# Guided Oura OAuth app registration (terminal prompts), then chain into login.
[group('run')]
auth-setup:
    cargo run -p oura-toolkit-cli -- auth setup

# Authorization Code login (loopback listener on :8788).
[group('run')]
auth-login:
    cargo run -p oura-toolkit-cli -- auth login

# ---------------------------------------------------------------------------------------------
# Release / publish
# ---------------------------------------------------------------------------------------------

# Validate the dist config and compute the release plan (CI runs this as its own job:
# a broken dist-workspace.toml must fail PRs, not the release tag).
[group('release')]
dist-check:
    dist plan

# Prove the crates still build from their PACKAGED form (no repo root — the crates.io
# publish context). Guards the #11 publish blocker: the spec-reading build scripts must
# find the crate-local openapi.json bundle. The CLI can only be verified this way once
# its path-deps are published, so the bundled-spec sync tests cover it meanwhile.
[group('release')]
publish-check:
    cargo package -p oura-toolkit-auth --locked --allow-dirty

# Build installers/artifacts locally (smoke test). REAL releases are tag-driven: pushing
# vX.Y.Z runs .github/workflows/release.yml (generated by dist init), which builds every
# platform and runs the npm/homebrew publish jobs.
[group('release')]
release:
    dist build

# Publish the Rust crates to crates.io in dependency order. npm + homebrew publishing is
# CI-driven by the release workflow on tag push — not from a laptop.
[group('release')]
publish:
    cargo publish -p oura-toolkit-api
    cargo publish -p oura-toolkit-auth
    cargo publish -p oura-toolkit-cli
