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
spec_file     := "spec/openapi.json"
build_dir     := "codegen/build"
overlaid_spec := build_dir / "openapi.overlaid.json"

# Show available recipes (default recipe).
default:
    @just --list --unsorted

# ---------------------------------------------------------------------------------------------
# Setup
# ---------------------------------------------------------------------------------------------

# Install/verify toolchains + codegen deps.
[group('setup')]
setup:
    rustup component add rustfmt clippy
    @command -v jq  >/dev/null || echo "!! install jq -- needed by 'just spec-overlay'"
    @command -v npx >/dev/null || echo "!! install node/npx -- needed by breadth-SDK codegen"
    @echo "Codegen/release tools are installed by their own recipes: progenitor (#6), cargo-dist (#11)."

# ---------------------------------------------------------------------------------------------
# Spec (source of truth)
# ---------------------------------------------------------------------------------------------

# Re-fetch the pinned Oura OpenAPI export to spec/openapi.json (kept pristine).
[group('spec')]
spec-fetch:
    mkdir -p spec
    curl -fsS {{spec_url}} -o {{spec_file}}
    @echo "Vendored {{spec_version}} -> {{spec_file}}"

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

# Generate the Rust SDK client (progenitor) -> sdks/rust/oura-api/src/lib.rs.
[group('codegen')]
gen-rust: spec-overlay
    @echo "TODO(#6): run progenitor on {{overlaid_spec}} -> sdks/rust/oura-api/src/lib.rs (// @generated)"

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

# Integration tests against Oura /v2/sandbox routes (canned data).
[group('build')]
test-sandbox:
    @echo "TODO(#9): run sandbox integration tests against /v2/sandbox/usercollection/*"

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

# What CI runs — and nothing else: fmt-check + lint + test.
[group('quality')]
ci:
    cargo fmt --all --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo test --workspace

# ---------------------------------------------------------------------------------------------
# Run / auth (local)
# ---------------------------------------------------------------------------------------------

# Run oura-cli as a STDIO MCP server.
[group('run')]
mcp:
    cargo run -p oura-cli -- --mcp

# Guided Oura OAuth app registration (loopback paste box), then chain into login.
[group('run')]
auth-setup:
    cargo run -p oura-cli -- auth setup

# Authorization Code login (loopback listener on :8788).
[group('run')]
auth-login:
    cargo run -p oura-cli -- auth login

# ---------------------------------------------------------------------------------------------
# Release / publish
# ---------------------------------------------------------------------------------------------

# Build installers/artifacts with cargo-dist.
[group('release')]
release:
    @echo "TODO(#11): cargo-dist release (shell + powershell + npm + homebrew)"

# Publish to npm + homebrew + crates.
[group('release')]
publish:
    @echo "TODO(#11): publish npm + homebrew + crates"
