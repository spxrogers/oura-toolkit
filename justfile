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

# Pinned nightly rustfmt for `just gen-rust` (#50). progenitor formats through
# codegen/rustfmt-shim.sh with unstable rustfmt options that need nightly; a dated toolchain
# keeps codegen deterministic so an unrelated PR's `just gen-check` can't flake when the
# floating `nightly` shifts rustfmt behavior. SINGLE SOURCE: `setup`, `install-nightly-rustfmt`,
# `gen-rust`, and CI's gen-drift job all read this.
# BUMP PROCEDURE: pick a newer date whose nightly ships rustfmt, set it here, run
# `just install-nightly-rustfmt && just gen-check` — it must stay byte-identical (the final
# `cargo fmt` is stable, so a bump should never churn the committed client); commit the bump alone.
nightly_rustfmt := "nightly-2026-06-15"

# openapi-generator, doubly pinned: the npm wrapper by version here, the generator jar by
# codegen/openapitools.json (7.14.0). Breadth-SDK codegen only; Rust stays on progenitor.
oag := "npx -y @openapitools/openapi-generator-cli@2.39.1 --openapitools codegen/openapitools.json"
# Skip per-model markdown docs + test stubs in generated output (hundreds of noise files);
# the spec is the reference. NOTE: config-file globalProperties are NOT honored by the npm
# wrapper -- this CLI flag is the working mechanism (verified against 7.14.0).
oag_skip_docs := "--global-property=apiDocs=false,modelDocs=false,apiTests=false,modelTests=false"

# Every generated client dir (drift-checked by `gen-check`; flagged linguist-generated).
# The *-auth companions, sdks/go/go.mod, and sdks/python's dist metadata are hand-written
# and deliberately NOT listed.
generated_dirs := "sdks/rust/oura-toolkit-api sdks/typescript/api sdks/python/oura_toolkit/api sdks/go/api sdks/java/api sdks/csharp/api"

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
setup: install-nightly-rustfmt
    rustup component add rustfmt clippy llvm-tools-preview
    # Rust codegen: progenitor CLI (the nightly rustfmt it formats through is the dep above).
    cargo install cargo-progenitor --locked
    # Coverage floor enforcement (`just coverage`).
    command -v cargo-llvm-cov >/dev/null || cargo install cargo-llvm-cov --locked
    @command -v jq  >/dev/null || echo "!! install jq -- needed by 'just spec-overlay' / 'just gen-rust'"
    @command -v npx >/dev/null || echo "!! install node/npx -- needed by breadth-SDK codegen"
    # The C# SDKs multi-target net10.0, so their build/test recipes need a .NET 10 SDK (an
    # 8.x/9.x SDK cannot build the net10.0 leg). Only relevant to the C# breadth-SDK recipes.
    @dotnet --list-sdks 2>/dev/null | grep -q '^10\.' || echo "!! install the .NET 10 SDK -- needed by 'just sdk-check-csharp' / 'just sdk-test-csharp' (net10.0 target)"
    # The netstandard2.0/Mono leg (`just sdk-test-csharp-netstandard`, #61) runs the C# suite
    # under Mono against the net472-resolved netstandard2.0 asset; only that recipe needs it.
    @command -v mono >/dev/null || echo "!! install mono (apt: mono-devel) -- needed by 'just sdk-test-csharp-netstandard' (#61 netstandard2.0 leg)"
    # Release tooling (`just dist-check` / `just release`).
    command -v dist >/dev/null || cargo install cargo-dist --locked

# Install the pinned nightly rustfmt (`nightly_rustfmt`) that `just gen-rust` formats through.
# Its own recipe so CI's gen-drift job installs the SAME dated toolchain without duplicating the
# date in YAML (#50). Idempotent — rustup no-ops if it's already present.
[group('setup')]
install-nightly-rustfmt:
    rustup toolchain install {{nightly_rustfmt}} --profile minimal --component rustfmt

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
gen: gen-rust gen-ts gen-py gen-go gen-java gen-csharp

# Generate the Rust SDK client (progenitor) -> sdks/rust/oura-toolkit-api/src/lib.rs.
# progenitor reads OpenAPI 3.0 only, so the overlaid 3.1 spec is down-converted first; its
# formatter needs nightly rustfmt via the shim (see codegen/rustfmt-shim.sh). The committed
# output builds on stable — only regeneration needs nightly + progenitor (installed by `setup`).
[group('codegen')]
gen-rust: spec-overlay
    mkdir -p {{build_dir}}
    jq -f codegen/progenitor-downconvert.jq {{overlaid_spec}} > {{progenitor_spec}}
    rm -rf {{build_dir}}/oura-toolkit-api-gen
    RUSTFMT_TOOLCHAIN={{nightly_rustfmt}} RUSTFMT="{{justfile_directory()}}/codegen/rustfmt-shim.sh" cargo progenitor -i {{progenitor_spec}} -o {{build_dir}}/oura-toolkit-api-gen -n oura-toolkit-api -v {{version}}
    cat codegen/generated-header.rs {{build_dir}}/oura-toolkit-api-gen/src/lib.rs > sdks/rust/oura-toolkit-api/src/lib.rs
    cargo fmt -p oura-toolkit-api
    @echo "Generated sdks/rust/oura-toolkit-api/src/lib.rs"

# Verify the committed generated clients match the current spec + overlays (CI drift check:
# catches hand-edits to generated dirs and spec/codegen drift). Needs `just setup`.
# `status --porcelain` (not `diff`) so brand-new untracked generated files count as drift too.
[group('codegen')]
gen-check: gen
    git diff --exit-code -- {{generated_dirs}}
    @test -z "$(git status --porcelain -- {{generated_dirs}})" || { git status --porcelain -- {{generated_dirs}}; echo "gen-check: untracked generated files (see above) — commit them"; exit 1; }

# Generate the TypeScript SDK client (openapi-generator) -> sdks/typescript/api.
[group('codegen')]
gen-ts: spec-overlay
    rm -rf sdks/typescript/api
    {{oag}} generate {{oag_skip_docs}} -c codegen/typescript.yaml -i {{overlaid_spec}} -o sdks/typescript/api --additional-properties=npmVersion={{version}}
    rm -f sdks/typescript/api/git_push.sh
    # Packaging post-patch (#57), guarded like gen-csharp's: the generator emits a dual-ESM
    # build that is BROKEN in Node (ESM-syntax .js inside a CJS package, reachable only via
    # the non-standard `module` field), no exports map, and placeholder repo metadata.
    # Pre-guards pin the generator shape we patch — if an upgrade changes it, fail here so
    # the overlay gets revisited instead of silently mispatching.
    @jq -e '.module and .typings and (.scripts.build | contains("tsconfig.esm.json"))' sdks/typescript/api/package.json > /dev/null || { echo "gen-ts: generator package.json shape changed (module/typings/dual-build gone) — update codegen/ts-package-postpatch.jq and this recipe"; exit 1; }
    @test -f sdks/typescript/api/tsconfig.esm.json || { echo "gen-ts: generator no longer emits tsconfig.esm.json — update this recipe"; exit 1; }
    jq -f codegen/ts-package-postpatch.jq sdks/typescript/api/package.json > sdks/typescript/api/package.json.tmp && mv sdks/typescript/api/package.json.tmp sdks/typescript/api/package.json
    rm -f sdks/typescript/api/tsconfig.esm.json
    @jq -e '(.exports["."] == {"types": "./dist/index.d.ts", "default": "./dist/index.js"}) and (.module | not) and (.typings | not) and .files == ["dist"] and .types == "./dist/index.d.ts"' sdks/typescript/api/package.json > /dev/null || { echo "gen-ts: packaging post-patch did not apply — see codegen/ts-package-postpatch.jq"; exit 1; }
    @echo "Generated sdks/typescript/api (@oura-toolkit/api {{version}}; CJS + exports map)"

# Generate the Python SDK client (openapi-generator) -> sdks/python/oura_toolkit/api.
# Like gen-rust: generate into the build dir, copy ONLY the generated package subtree in.
# The distribution metadata (sdks/python/pyproject.toml, name `oura-toolkit`) and the
# oura_toolkit/__init__.py namespace root are HAND-WRITTEN — the single PyPI dist will also
# house the hand-written oura_toolkit.auth companion, and the generator's own root metadata
# mis-names the dist (`oura_toolkit.api`) and doesn't build.
[group('codegen')]
gen-py: spec-overlay
    rm -rf {{build_dir}}/python-gen sdks/python/oura_toolkit/api
    {{oag}} generate {{oag_skip_docs}} -c codegen/python.yaml -i {{overlaid_spec}} -o {{build_dir}}/python-gen --additional-properties=packageVersion={{version}}
    mkdir -p sdks/python/oura_toolkit
    cp -R {{build_dir}}/python-gen/oura_toolkit/api sdks/python/oura_toolkit/api
    # (The hand-written pyproject.toml's version sync is guarded by `just version-check`.)
    @echo "Generated sdks/python/oura_toolkit/api (oura-toolkit {{version}})"

# Generate the Go SDK client (openapi-generator) -> sdks/go/api. The module file
# (sdks/go/go.mod, module github.com/spxrogers/oura-toolkit/sdks/go) is hand-written and
# deliberately outside the wipe: withGoMod=false keeps the generator's hands off it.
[group('codegen')]
gen-go: spec-overlay
    rm -rf sdks/go/api
    {{oag}} generate {{oag_skip_docs}} -c codegen/go.yaml -i {{overlaid_spec}} -o sdks/go/api
    rm -f sdks/go/api/git_push.sh sdks/go/api/.travis.yml
    @echo "Generated sdks/go/api"

# Generate the Java SDK client (openapi-generator) -> sdks/java/api (com.ouratoolkit:api).
[group('codegen')]
gen-java: spec-overlay
    rm -rf sdks/java/api
    {{oag}} generate {{oag_skip_docs}} -c codegen/java.yaml -i {{overlaid_spec}} -o sdks/java/api --additional-properties=artifactVersion={{version}}
    rm -f sdks/java/api/git_push.sh sdks/java/api/.travis.yml sdks/java/api/gradlew sdks/java/api/gradlew.bat
    rm -rf sdks/java/api/gradle
    @echo "Generated sdks/java/api (com.ouratoolkit:api {{version}})"

# Generate the C# SDK client (openapi-generator) -> sdks/csharp/api (OuraToolkit.Api).
[group('codegen')]
gen-csharp: spec-overlay
    rm -rf sdks/csharp/api
    {{oag}} generate {{oag_skip_docs}} -c codegen/csharp.yaml -i {{overlaid_spec}} -o sdks/csharp/api --additional-properties=packageVersion={{version}}
    # The generator mints a FRESH solution GUID every run — the .sln is the one
    # non-deterministic output file, and sdk-check builds the csproj directly. Drop it.
    rm -f sdks/csharp/api/git_push.sh sdks/csharp/api/.travis.yml sdks/csharp/api/appveyor.yml sdks/csharp/api/OuraToolkit.Api.sln
    # Multi-target post-patch: the generator emits a single netstandard2.0 target (and rejects
    # net10.0 outright), so widen it to the shipped TFM set. Deterministic (drift-safe): the
    # sed target is the generator's fixed line. netstandard2.0 = broad reach; net8.0/net10.0 =
    # modern assemblies. The Newtonsoft-based netstandard2.0 code compiles unchanged on all three.
    # Pin LangVersion to 13.0 (not `latest`): `latest` floats with the installed SDK, so codegen
    # output would drift under gen-check as CI's dotnet updates. A fixed version is deterministic
    # and still >= the C# 9 the netstandard2.0 <Nullable>annotations</Nullable> leg needs.
    sed -i 's|<TargetFramework>netstandard2.0</TargetFramework>|<TargetFrameworks>netstandard2.0;net8.0;net10.0</TargetFrameworks>\n    <LangVersion>13.0</LangVersion>|' sdks/csharp/api/src/OuraToolkit.Api/OuraToolkit.Api.csproj
    @grep -q '<TargetFrameworks>netstandard2.0;net8.0;net10.0</TargetFrameworks>' sdks/csharp/api/src/OuraToolkit.Api/OuraToolkit.Api.csproj || { echo "gen-csharp: multi-target post-patch did not apply — generator csproj shape changed?"; exit 1; }
    @grep -q '<LangVersion>13.0</LangVersion>' sdks/csharp/api/src/OuraToolkit.Api/OuraToolkit.Api.csproj || { echo "gen-csharp: LangVersion post-patch did not apply (needed for <Nullable>annotations</Nullable> on the netstandard2.0 leg, whose default is C# 7.3)"; exit 1; }
    # Strip the generator's bogus System.Web ItemGroups (a .NET-Framework-only assembly the code
    # never uses) — wrapper and all, so no empty ItemGroups linger. It emits MSB3245 "could not
    # resolve" on every modern TFM otherwise.
    perl -0pi -e 's{\s*<ItemGroup>\s*<(?:None Remove|Reference Include)="System\.Web"\s*/>\s*</ItemGroup>}{}g' sdks/csharp/api/src/OuraToolkit.Api/OuraToolkit.Api.csproj
    @! grep -q 'System.Web' sdks/csharp/api/src/OuraToolkit.Api/OuraToolkit.Api.csproj || { echo "gen-csharp: System.Web strip did not apply"; exit 1; }
    @echo "Generated sdks/csharp/api (OuraToolkit.Api {{version}}; netstandard2.0;net8.0;net10.0)"

# Compile/import-check every committed breadth client (the generated code must actually
# build — release gate: a check CI can't see doesn't exist; CI runs this as its own job).
[group('codegen')]
sdk-check: sdk-check-ts sdk-check-py sdk-check-go sdk-check-java sdk-check-csharp

# In-tree npm build: node_modules/ + dist/ are covered by the generated .gitignore, and
# --no-package-lock keeps the tree clean for gen-check.
[group('codegen')]
sdk-check-ts:
    cd sdks/typescript/api && npm install --no-package-lock --no-fund --no-audit && npm run build
    # Packaging guards (#57): the exports-map entry must actually LOAD as CJS (a broken
    # dist path or ESM-syntax entry throws here), the stale dual-ESM tree must be gone,
    # and the publish surface stays dist-only (npm pack must never ship src/tsconfigs).
    cd sdks/typescript/api && node -e "const e = require('./package.json').exports['.']; const m = require(e.default); if (typeof m.Configuration !== 'function') throw new Error('exports[.].default did not load the client (Configuration missing)'); require('fs').accessSync(e.types);"
    # The headline #57 guarantee is the ESM consumer story, so exercise it for real:
    # a self-referencing `import` resolves through the exports map (self-reference REQUIRES
    # exports) and named exports must survive Node's CJS interop (cjs-module-lexer) — a
    # regression here leaves require() green and breaks `import {…}` consumers.
    cd sdks/typescript/api && node --input-type=module -e "const m = await import('@oura-toolkit/api'); if (typeof m.Configuration !== 'function') throw new Error('ESM named import via the exports map broke (interop/lexer regression?)');"
    @test ! -d sdks/typescript/api/dist/esm || { echo "sdk-check-ts: dist/esm reappeared — the CJS-only packaging post-patch (#57) regressed"; exit 1; }
    cd sdks/typescript/api && npm pack --dry-run --json | jq -e '.[0].files | map(.path) | all(. == "package.json" or . == "README.md" or . == "LICENSE" or startswith("dist/")) and any(startswith("dist/"))' > /dev/null || { echo "sdk-check-ts: npm pack would ship files outside dist/ (src, tsconfigs?) — files allowlist broken"; exit 1; }

# Hand-written TS auth companion (#15): build + the hermetic node:test suite (mock token
# endpoint on 127.0.0.1, tempdir stores — no network, no real credentials). (package.json's
# version sync is guarded by `just version-check`.)
[group('codegen')]
sdk-test-ts:
    cd sdks/typescript/auth && npm install --no-package-lock --no-fund --no-audit && npm run build && npm test

[group('codegen')]
sdk-check-py:
    rm -rf {{build_dir}}/py-venv
    python3 -m venv {{build_dir}}/py-venv
    {{build_dir}}/py-venv/bin/pip install --quiet ./sdks/python
    {{build_dir}}/py-venv/bin/python -c "from oura_toolkit.api import ApiClient, Configuration"

# Hermetic tests for the hand-written oura_toolkit.auth companion (loopback mock token
# endpoint + tempdir stores; pytest is a dev-only tool in this venv, never a runtime dep
# of the dist). Own venv so it can't fight sdk-check-py's; tests run against the
# INSTALLED copy, so packaging regressions fail here too.
[group('codegen')]
sdk-test-py:
    rm -rf {{build_dir}}/py-test-venv
    python3 -m venv {{build_dir}}/py-test-venv
    {{build_dir}}/py-test-venv/bin/pip install --quiet ./sdks/python pytest
    {{build_dir}}/py-test-venv/bin/python -m pytest -q sdks/python/tests

[group('codegen')]
sdk-check-go:
    cd sdks/go && go build ./...

# Hand-written Go auth companion (sdks/go/auth, #15): vet + hermetic tests under the race
# detector (httptest token endpoint, tempdir stores, a real second process for the lock
# test, genuinely concurrent refreshes), then a GOOS=windows vet so the windows lock/store
# branches at least compile on the Linux leg.
[group('codegen')]
sdk-test-go:
    cd sdks/go && go vet ./... && go test -race ./auth/...
    cd sdks/go && GOOS=windows go vet ./auth/...

[group('codegen')]
sdk-check-java:
    cd sdks/java/api && mvn --quiet -DskipTests compile

# Java auth-companion unit tests (sdks/java/auth — hermetic: loopback token endpoint via
# jdk.httpserver, @TempDir stores, injected env lookups). (The hand-written pom's version
# sync is guarded by `just version-check`.)
[group('codegen')]
sdk-test-java:
    cd sdks/java/auth && mvn --quiet test

# Requires the dotnet SDK (absent locally is a real failure, not a skip — CI has it; a
# silent skip here would let a broken C# client ship). Builds BOTH halves: the generated api
# client AND the hand-written auth companion, whose multi-target csproj
# (netstandard2.0;net8.0;net10.0) makes a single `dotnet build` compile every TFM — so a
# netstandard2.0-only break (missing UnixFileMode/SocketsHttpHandler polyfill, etc.) fails here.
[group('codegen')]
sdk-check-csharp:
    dotnet build --nologo -v quiet sdks/csharp/api/src/OuraToolkit.Api
    dotnet build --nologo -v quiet sdks/csharp/auth/src/OuraToolkit.Auth

# HAND-WRITTEN C# auth companion tests (hermetic: mock HttpMessageHandler + temp-dir
# stores; run by the sdk-compile CI job). The multi-target test project (net8.0;net10.0)
# means one `dotnet test` runs the whole suite on BOTH modern runtimes. A modern host cannot
# LOAD the library's netstandard2.0 asset, so its `#if NETSTANDARD2_0` branches never execute
# here — that gap is closed by `just sdk-test-csharp-netstandard` (#61), which runs the SAME
# suite on Mono. (The csproj's version sync is guarded by `just version-check`.)
[group('codegen')]
sdk-test-csharp:
    dotnet test --nologo -v quiet sdks/csharp/auth/tests/OuraToolkit.Auth.Tests

# The C# auth suite on a runtime that ACTUALLY LOADS the netstandard2.0 asset (#61). The modern
# `just sdk-test-csharp` host only loads net8.0/net10.0, leaving the library's `#if NETSTANDARD2_0`
# code (Polyfills, the libc open(2)+write(2)+fsync(2) store path selection, the HttpClientHandler
# transport) compiled but never run. This builds the same xunit suite for net472 — whose project
# reference resolves the library's netstandard2.0 asset — and runs it under Mono via the xunit
# console runner. BuildInfoTests fails the leg if it ever loads the wrong asset. Needs `mono`
# (apt: mono-devel); CI installs it. Opt-in TFM (-p:NetFxTest=true) so sdk-test-csharp is untouched.
[group('codegen')]
sdk-test-csharp-netstandard:
    command -v mono >/dev/null || { echo "!! install mono (apt: mono-devel) -- needed by 'just sdk-test-csharp-netstandard' to run the net472/netstandard2.0 leg"; exit 1; }
    dotnet build --nologo -v quiet -c Release -f net472 -p:NetFxTest=true sdks/csharp/auth/tests/OuraToolkit.Auth.Tests
    mono sdks/csharp/auth/tests/OuraToolkit.Auth.Tests/bin/Release/net472/xunit-runner/xunit.console.exe sdks/csharp/auth/tests/OuraToolkit.Auth.Tests/bin/Release/net472/OuraToolkit.Auth.Tests.dll

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

# Breadth-SDK live smokes against the sandbox (network; opt-in, never CI): each generated
# client makes a real request and parses the typed response — TS/Python/Go/Java/C# today.
# Builds via the sdk-check recipes first (Java needs `install`, a superset of
# sdk-check-java's compile, so the smoke's pom can resolve com.ouratoolkit:api from the
# local repo).
[group('build')]
test-sandbox-sdks: sdk-check-ts sdk-check-py sdk-check-go sdk-check-csharp
    node codegen/smoke/smoke.cjs
    {{build_dir}}/py-venv/bin/python codegen/smoke/smoke.py
    cd codegen/smoke/go-smoke && go run .
    cd sdks/java/api && mvn --quiet -DskipTests -Dmaven.javadoc.skip=true -Dmaven.source.skip=true install
    cd codegen/smoke/java-smoke && mvn --quiet -Dapi.version={{version}} compile exec:java
    dotnet run --project codegen/smoke/csharp-smoke

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

# Show stored auth state (exit 4 when unauthenticated).
[group('run')]
auth-status:
    cargo run -p oura-toolkit-cli -- auth status

# Remove stored tokens (`oura auth logout --all` also removes the client credentials).
[group('run')]
auth-logout:
    cargo run -p oura-toolkit-cli -- auth logout

# Force a token refresh now (persists the rotated refresh token).
[group('run')]
auth-refresh:
    cargo run -p oura-toolkit-cli -- auth refresh

# Print a valid access token to stdout (the scripting workhorse).
[group('run')]
auth-token:
    cargo run -p oura-toolkit-cli -- auth token

# ---------------------------------------------------------------------------------------------
# Release / publish
# ---------------------------------------------------------------------------------------------

# Validate the release config (CI runs this as its own job: a broken dist-workspace.toml
# must fail PRs, not the release tag). Three guards:
#  1. `dist plan` — config parses and the release resolves.
#  2. `dist generate --check` — the committed release.yml matches the config (same drift
#     doctrine as `gen-check` for the progenitor client).
#  3. The NPX-FIRST invariant (CLAUDE.md → DISTRIBUTION): `npx -y oura-toolkit` must run
#     `oura` — asserted on the REAL npm artifact's package.json, because a typo'd
#     npm-package sails through `dist plan` green.
[group('release')]
dist-check:
    dist plan
    dist generate --check
    dist build --artifacts=global
    tar -xzOf target/distrib/oura-toolkit-cli-npm-package.tar.gz package/package.json | jq -e '.name == "oura-toolkit" and (.bin | keys == ["oura"])' > /dev/null
    @echo "dist config valid; npm artifact is oura-toolkit with bin oura"

# Prove the publishable crate builds from its PACKAGED form with NO repo root: the
# tarball is extracted to a temp dir OUTSIDE the repo, where build.rs's walk-up fallback
# has nothing to find — the crates.io context for real. (Inside the repo, `cargo package`'s
# own verify build would be rescued by the fallback and prove nothing.) The CLI gets the
# same treatment once its path-deps are published; its bundled-spec sync test covers it
# meanwhile. Shares the workspace target dir so only the crate itself compiles.
[group('release')]
publish-check:
    cargo package -p oura-toolkit-auth --locked --allow-dirty
    # (.crate files carry trailing bytes after the gzip stream; decompress via pipe.)
    # DEDICATED target dir, not the workspace one: sharing it lets cargo reuse the
    # workspace build's cached build-script output (its watched repo files are unchanged)
    # and skip re-running build.rs in the isolated context — silently proving nothing.
    # In target/publish-check, any cached fingerprint watches the PREVIOUS run's dead
    # tmp path, so build.rs re-runs every time while dependency builds stay cached.
    # LOAD-BEARING: the extract path MUST stay a fresh mktemp per run — a fixed dir would
    # let run 2 cache-hit the build script and silently prove nothing again.
    tmp=$(mktemp -d) && gzip -dc target/package/oura-toolkit-auth-{{version}}.crate 2>/dev/null | tar x -C "$tmp" && CARGO_TARGET_DIR={{justfile_directory()}}/target/publish-check cargo build --manifest-path "$tmp/oura-toolkit-auth-{{version}}/Cargo.toml" && rm -rf "$tmp"

# Bump the workspace version EVERYWHERE, in one command (#59): codegen/version.sh is the
# single writer over the root Cargo.toml (the source, incl. the internal-crate dep pins)
# plus every hand-written manifest that carries the literal (TS/Python/Java/C# companions,
# plugin.json, .mcp.json's npx pin), and it self-verifies each rewrite. Then refresh
# Cargo.lock. Release = this, commit, tag vX.Y.Z.
[group('release')]
set-version new_version:
    codegen/version.sh set {{new_version}}
    cargo update --workspace --quiet
    @echo "Now commit, then tag v{{new_version}} and push to release (CLAUDE.md → DISTRIBUTION)."

# THE single version-drift guard (#59): every hand-written manifest equals the workspace
# version. Replaces the per-file grep-guards that used to sprawl across gen-py/sdk-test-*/
# plugin-check; run by the release-config CI job on every PR.
[group('release')]
version-check:
    codegen/version.sh check

# Plugin release-config guards (#12): both manifests validate under --strict, and the
# marketplace↔plugin source linkage resolves. (The version pins — plugin.json's version and
# .mcp.json's npx pin — are guarded by `just version-check` like every other manifest, #59.)
[group('release')]
plugin-check:
    # `claude plugin validate .` does NOT resolve marketplace source paths (break-verified:
    # a nonexistent source passes strict) — assert the linkage ourselves. The length
    # precheck keeps a zero-plugin marketplace (which the loop would skip) from passing.
    jq -e '.plugins | length >= 1' .claude-plugin/marketplace.json > /dev/null || { echo "marketplace lists no plugins"; exit 1; }
    # (No pipefail: just's sh is dash. A jq crash here can't be masked in practice — the
    # precheck above already parsed this exact file, and a missing/malformed .plugins
    # fails there first.)
    jq -r '.plugins[].source' .claude-plugin/marketplace.json | while read -r src; do test -f "$src/.claude-plugin/plugin.json" || { echo "marketplace source $src has no plugin manifest — broken marketplace->plugin linkage"; exit 1; }; done
    claude plugin validate plugins/oura-toolkit --strict
    claude plugin validate . --strict

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
