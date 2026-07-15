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
# Docs-site API-reference input: the overlaid spec with x-codeSamples language labels normalized
# to Shiki ids (docs-only presentation fix). Derived; produced by `just docs-spec`.
docs_spec := build_dir / "openapi.docs.json"

# Astro Starlight documentation site (docs-site/), published to ouratoolkit.com via GitHub
# Pages. Its API reference is generated at build time from the OVERLAID spec, its CLI reference
# from the `oura` binary; both are wired through the `docs-*` recipes below (never raw npm/astro).
docs_dir := "docs-site"

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

# Pinned cargo-progenitor — the Rust code GENERATOR (#50). Pinned alongside nightly_rustfmt so a
# floating `latest` can't change generated output and fail `just gen-check` for a reason the
# rustfmt pin can't prevent (matches the CLAUDE.md bootstrap version matrix). SINGLE SOURCE:
# `setup`, `install-progenitor`, and CI's gen-drift job read this. Bump: change here, re-run
# `just install-progenitor && just gen-check` (byte-identical), and bump the workflow cache key.
progenitor_version := "0.14.0"

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
setup: install-nightly-rustfmt install-progenitor
    rustup component add rustfmt clippy llvm-tools-preview
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
    # Release tooling (`just dist-check` / `just dist-build` / `just release`).
    command -v dist >/dev/null || cargo install cargo-dist --locked

# Install the pinned nightly rustfmt (`nightly_rustfmt`) that `just gen-rust` formats through.
# Its own recipe so CI's gen-drift job installs the SAME dated toolchain without duplicating the
# date in YAML (#50). Idempotent — rustup no-ops if it's already present.
[group('setup')]
install-nightly-rustfmt:
    rustup toolchain install {{nightly_rustfmt}} --profile minimal --component rustfmt

# Install the pinned progenitor CLI (`progenitor_version`). Its own recipe so CI's gen-drift job
# installs the SAME version without duplicating it in YAML (#50). `--force` so a bump re-pins even
# when an older progenitor is already on PATH (CI guards this call with `command -v` for caching).
[group('setup')]
install-progenitor:
    cargo install cargo-progenitor --version {{progenitor_version}} --locked --force

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

# Detect upstream Oura OpenAPI drift (#29): the pinned export re-published with changes, or a
# newer openapi-<major>.<minor> exists. WATCH-ONLY — never edits the spec. Hits the network, so
# it is NOT in `just ci`; the scheduled spec-drift workflow runs it and opens/updates an issue.
# Exit 0 = no drift, 1 = drift (markdown report on stdout), 2 = error.
[group('spec')]
spec-drift-check:
    codegen/spec-drift.sh {{spec_version}} {{spec_url}} {{spec_file}}

# Hermetic self-test of the drift detector's decision logic (#29) — no network (the upstream
# content and the "existing versions" are injected), so it runs in CI (the gen-drift job).
# Break-verify: neuter a branch in codegen/spec-drift.sh and a case below fails.
[group('spec')]
spec-drift-selftest:
    #!/usr/bin/env bash
    set -euo pipefail
    sv="{{spec_version}}"; su="{{spec_url}}"; sf="{{spec_file}}"
    pd="$(mktemp -d)"; mod="$(mktemp)"; trap 'rm -rf "$pd" "$mod"' EXIT
    # spec_version is openapi-1.35, so 1.36 / 1.40 / 2.0 are candidates the probe loop reaches.
    export OURA_SPEC_DRIFT_PROBE_COUNT=6
    run() { OURA_SPEC_DRIFT_UPSTREAM_FILE="$1" OURA_SPEC_DRIFT_PROBE_DIR="$pd" codegen/spec-drift.sh "$sv" "$su" "$sf"; }
    # 1) no drift: upstream == committed, empty probe dir -> exit 0.
    run "$sf" >/dev/null || { echo "spec-drift-selftest: the clean case must exit 0"; exit 1; }
    # 2) content drift: a byte-changed upstream -> exit 1 and a Content-drift report.
    cp "$sf" "$mod"; printf '\n' >> "$mod"
    out="$(run "$mod")" && { echo "spec-drift-selftest: content drift must exit 1"; exit 1; }
    grep -q 'Content drift' <<<"$out" || { echo "spec-drift-selftest: content report missing"; exit 1; }
    # 3) minor version drift: a REAL export appears as a newer minor -> reported.
    cp "$sf" "$pd/openapi-1.36.json"
    out="$(run "$sf")" && { echo "spec-drift-selftest: minor version drift must exit 1"; exit 1; }
    grep -q 'newer export' <<<"$out" && grep -q '1.36' <<<"$out" \
      || { echo "spec-drift-selftest: minor version report missing"; exit 1; }
    # 4) soft-404: a 200 that ISN'T an OpenAPI doc must be IGNORED (no spam). The run still exits
    #    1 (the valid 1.36 above is real drift), so `|| true` just lets us inspect the report.
    printf '<html>not found</html>' > "$pd/openapi-1.40.json"
    out="$(run "$sf" || true)"
    grep -q '1.40' <<<"$out" && { echo "spec-drift-selftest: soft-404 must not be reported"; exit 1; }
    grep -q '1.36' <<<"$out" || { echo "spec-drift-selftest: valid version lost in the soft-404 case"; exit 1; }
    # 5) MAJOR bump: the next major's .0 export must be caught (content drift never would).
    cp "$sf" "$pd/openapi-2.0.json"
    out="$(run "$sf")" && { echo "spec-drift-selftest: major bump must exit 1"; exit 1; }
    grep -q '2.0' <<<"$out" || { echo "spec-drift-selftest: major-bump report missing"; exit 1; }
    # 6) hard error: an unparseable version must exit 2 (the workflow branches on it). Capture
    #    with `|| c=$?` so `set -e` doesn't abort on the expected non-zero exit.
    c=0
    OURA_SPEC_DRIFT_UPSTREAM_FILE="$sf" codegen/spec-drift.sh "openapi-BAD" "$su" "$sf" >/dev/null 2>&1 || c=$?
    [[ $c -eq 2 ]] || { echo "spec-drift-selftest: a bad version must exit 2 (got $c)"; exit 1; }
    echo "spec-drift-selftest: all cases pass"

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

# Shell completions + man page shipped in every release archive (#75). Generated from the
# CLI's own `oura completion`/`oura man` (shipped #27), committed and drift-checked like the
# generated clients, and `include`d by dist-workspace.toml so `dist build` packages them into
# each tarball. Regenerate after any change to the CLI surface OR the version (the man page's
# `.TH` embeds it) — the drift check below fails until you do.
[group('codegen')]
gen-completions:
    cargo build --quiet -p oura-toolkit-cli
    mkdir -p cli/oura-toolkit-cli/dist-assets
    ./target/debug/oura completion bash > cli/oura-toolkit-cli/dist-assets/oura.bash
    ./target/debug/oura completion zsh  > cli/oura-toolkit-cli/dist-assets/oura.zsh
    ./target/debug/oura completion fish > cli/oura-toolkit-cli/dist-assets/oura.fish
    ./target/debug/oura man             > cli/oura-toolkit-cli/dist-assets/oura.1
    @echo "Generated cli/oura-toolkit-cli/dist-assets/oura.{1,bash,zsh,fish}"

# Drift guard: the committed completions/man match the current CLI surface + version. Same
# doctrine as `gen-check`; runs in the release-config CI job.
[group('codegen')]
gen-completions-check: gen-completions
    git diff --exit-code -- cli/oura-toolkit-cli/dist-assets
    @test -z "$(git status --porcelain -- cli/oura-toolkit-cli/dist-assets)" || { git status --porcelain -- cli/oura-toolkit-cli/dist-assets; echo "gen-completions-check: dist-assets stale — run 'just gen-completions' and commit"; exit 1; }

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

# Regenerate the dist-managed CI (release.yml) after changing the [dist] config in
# dist-workspace.toml. The counterpart to `dist-check`'s `dist generate --check` drift guard
# (gen ↔ gen-check doctrine): edit the config, run this, commit the regenerated release.yml.
[group('release')]
dist-generate:
    dist generate

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
    # #75: the man page + completions the archives ship must ACTUALLY be packaged — assert
    # they landed in the built artifact, so a typo'd/renamed `include` path in
    # dist-workspace.toml fails a PR here, not silently at release-tag time (guarantee = test).
    for f in oura.1 oura.bash oura.zsh oura.fish; do tar -tzf target/distrib/oura-toolkit-cli-npm-package.tar.gz | grep -qx "package/$f" || { echo "release artifact is missing $f -- the dist include list (dist-workspace.toml) is broken"; exit 1; }; done
    @echo "dist config valid; npm artifact is oura-toolkit with bin oura; completions + man page packaged"

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
    # The man page's `.TH` embeds the version (#75), so it's a version-carrying artifact:
    # regenerate it here (the single writer, #59) rather than leave a stale oura.1 that would
    # fail `just gen-completions-check` on the release PR.
    just gen-completions
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

# Cut a release in ONE command from your laptop: `just release 0.1.0` (a leading `v` is fine
# too). Local-only preconditions, then the shared `release-tag` choreography (gate → bump →
# commit → tag → push). Fully TAG-DRIVEN (#59, #91): the pushed vX.Y.Z tag drives EVERY publish
# channel in CI — release.yml (installers + npm/Homebrew) AND publish-crates.yml (crates.io via
# OIDC Trusted Publishing). Nothing is published from your laptop. IRREVERSIBLE past the tag
# push. Needs the release toolchain — run `just setup`. (Prefer a button? The Cut-release GitHub
# Action runs the same `release-tag` server-side.)
[group('release')]
release new_version:
    #!/usr/bin/env bash
    set -euo pipefail
    ver="{{ trim_start_match(new_version, 'v') }}"
    tag="v${ver}"

    # Local-only preconditions — the gate + tag-existence checks live in `release-tag`. Nothing
    # is mutated until every one of these passes. (No crates.io-auth check: publishing is now
    # CI-driven via OIDC, #91 — the laptop never needs a registry token.)
    printf '%s' "$ver" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+([-.+][0-9A-Za-z.-]+)?$' \
      || { echo "release: '{{new_version}}' is not a semver version (expected X.Y.Z)"; exit 1; }
    branch="$(git rev-parse --abbrev-ref HEAD)"
    [ "$branch" = "main" ] || { echo "release: must be on 'main' (currently on '$branch')"; exit 1; }
    [ -z "$(git status --porcelain)" ] || { echo "release: working tree is dirty — commit or stash first"; exit 1; }
    git fetch --quiet origin main
    [ "$(git rev-parse HEAD)" = "$(git rev-parse origin/main)" ] \
      || { echo "release: local main has diverged from origin/main — sync first"; exit 1; }

    # Shared choreography: gate → set-version → guards → commit → tag → push (triggers CI).
    just release-tag "${ver}"

    echo ""
    echo "Released ${tag}: CI is publishing installers + npm/Homebrew (release.yml) and crates.io (publish-crates.yml). Watch the Actions tab."

# The shared release choreography behind BOTH `just release` (laptop) and the Cut-release GitHub
# Action (.github/workflows/cut-release.yml). Runs the full gate ('green == releasable'), bumps
# every manifest, commits, and pushes the commit + vX.Y.Z tag — the tag push triggers
# .github/workflows/release.yml (installers + npm/homebrew). Pushes with whatever git
# credentials are configured (your local remote, or the workflow's checkout PAT). Does NOT
# publish crates.io. `dry_run=true` runs the gate + bump but pushes nothing (a preview).
[group('release')]
release-tag new_version dry_run="false":
    #!/usr/bin/env bash
    set -euo pipefail
    ver="{{ trim_start_match(new_version, 'v') }}"
    tag="v${ver}"

    printf '%s' "$ver" | grep -Eq '^[0-9]+\.[0-9]+\.[0-9]+([-.+][0-9A-Za-z.-]+)?$' \
      || { echo "release-tag: '{{new_version}}' is not a semver version (expected X.Y.Z)"; exit 1; }
    git rev-parse -q --verify "refs/tags/${tag}" >/dev/null 2>&1 \
      && { echo "release-tag: tag ${tag} already exists locally"; exit 1; } || true
    git ls-remote --exit-code origin "refs/tags/${tag}" >/dev/null 2>&1 \
      && { echo "release-tag: tag ${tag} already exists on origin"; exit 1; } || true

    # Code gate FIRST, on the still-pristine tree — a failure here mutates nothing.
    echo "==> release gate: just ci"
    just ci
    # Bump every manifest (single writer, #59; also refreshes completions + Cargo.lock).
    echo "==> set-version ${ver}"
    just set-version "${ver}"
    # The release-config guards CI runs — MINUS gen-completions-check. `set-version` above
    # already regenerated the completions + man page for ${ver} (they'll ride the bump commit
    # below), and gen-completions-check is `gen-completions` + `git diff --exit-code`: on this
    # mid-bump tree that diff flags the legitimate, not-yet-committed version bump as "drift" and
    # fails (it assumes a clean tree, as on a PR). The check is redundant here — the assets are
    # freshly generated by construction — and CI's release-config job still enforces it on every
    # PR, so a stale-asset change can never reach a release tag. The other guards below tolerate
    # the dirty tree (no git-diff-against-HEAD; publish-check already runs cargo package
    # --allow-dirty).
    echo "==> release gate: release-config guards"
    just version-check
    just dist-check
    just publish-check
    just plugin-check

    if [ "{{dry_run}}" = "true" ]; then
      echo "DRY RUN — gate passed and manifests bumped to ${ver} in the working tree; nothing committed, tagged, or pushed."
      git --no-pager diff --stat
      exit 0
    fi

    # The bump is a no-op when the manifests are already at ${ver} (e.g. the FIRST release, or
    # re-cutting the current version) — `git commit` errors on a clean tree, so commit only when
    # there's something to commit and tag the current HEAD otherwise. The tag-existence guards
    # above already ensure we're not re-tagging a released version.
    committed=0
    if [ -n "$(git status --porcelain)" ]; then
      echo "==> commit + tag ${tag}"
      git commit -aqm "Release ${tag}"
      committed=1
    else
      echo "==> tag ${tag} (manifests already at ${ver} — nothing to commit)"
    fi
    git tag -a "${tag}" -m "Release ${tag}"

    # Push the bump commit (only if we made one), then the tag — the tag push triggers
    # .github/workflows/release.yml (npm + Homebrew + GitHub Release).
    echo "==> push ${tag}"
    if [ "$committed" = 1 ]; then git push --quiet origin HEAD:main; fi
    git push --quiet origin "${tag}"
    echo "Pushed ${tag}: CI is building installers + publishing npm/Homebrew (watch the Actions tab)."

# Build installers/artifacts locally (smoke test — publishes nothing). REAL releases go through
# `just release X.Y.Z`: pushing vX.Y.Z runs .github/workflows/release.yml (generated by dist
# init), which builds every platform and runs the npm/homebrew publish jobs.
[group('release')]
dist-build:
    dist build

# Publish the Rust crates to crates.io in dependency order. On a release this runs in CI via
# .github/workflows/publish-crates.yml (Trusted Publishing / OIDC, #91) — this recipe is the
# MANUAL fallback (needs `cargo login`), e.g. to finish a crates.io publish CI couldn't. npm +
# homebrew publishing is CI-driven by release.yml on tag push — not from a laptop.
[group('release')]
publish:
    cargo publish -p oura-toolkit-api
    cargo publish -p oura-toolkit-auth
    cargo publish -p oura-toolkit-cli

# ---------------------------------------------------------------------------------------------
# Docs site (docs-site/ — Astro Starlight, published to ouratoolkit.com via GitHub Pages)
# ---------------------------------------------------------------------------------------------
#
# Two source-of-truth wirings keep the site from drifting (CLAUDE.md → DOCS STAY TRUE TO THE
# CODE): the API reference is generated at build time from the OVERLAID spec (so it IS the
# spec), and the CLI reference is generated from the `oura` binary and drift-checked. Raw
# npm/astro live ONLY in these recipes.

# Install the pinned docs toolchain (reproducible; committed package-lock.json). `npm ci`
# mirrors the lockfile exactly, like the breadth-SDK npm trees.
[group('docs')]
docs-install:
    cd {{docs_dir}} && npm ci

# Regenerate the committed CLI reference (docs-site/src/content/docs/cli/reference.md) from the
# `oura` binary's own `--help` — the SAME source as its completions/man page, so the documented
# flags/defaults can't drift from the binary. Committed + drift-checked (`docs-gen-cli-check`),
# exactly like `gen-completions`. Subcommands are enumerated FROM the binary (not a hand-kept
# list) so a new command is captured automatically; a completeness tripwire in docs_tripwire.rs
# fails CI if the enumeration ever misses one. stdout is a pipe here, so clap wraps at its fixed
# default width — the output is deterministic regardless of the terminal.
[group('docs')]
docs-gen-cli:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --quiet -p oura-toolkit-cli
    bin=./target/debug/oura
    dir={{docs_dir}}/src/content/docs/cli
    out="$dir/reference.md"
    export NO_COLOR=1
    cp "$dir/_reference.header.md" "$out"
    # One section per command; `oura auth` subcommands nest under it as level-3 headings.
    emit() { local level="$1"; shift; printf '\n%s `oura %s`\n\n```text\n' "$level" "$*" >> "$out"; "$bin" "$@" --help >> "$out"; printf '```\n' >> "$out"; }
    # Subcommand names under a clap `Commands:` section: exactly-two-space-indented lines (first
    # token), skipping deeper-indented wrapped descriptions and clap's own `help`.
    subcommands() { "$bin" "$@" --help | awk '/^Commands:/{f=1;next} f{ if($0 !~ /^  /) exit; if($0 ~ /^   /) next; print $1 }' | grep -vx help; }
    printf '\n## `oura`\n\n```text\n' >> "$out"; "$bin" --help >> "$out"; printf '```\n' >> "$out"
    for cmd in $(subcommands); do
      emit '##' "$cmd"
      if [ "$cmd" = auth ]; then for sub in $(subcommands auth); do emit '###' auth "$sub"; done; fi
    done
    echo "Generated $out"

# Drift guard: the committed CLI reference matches the current binary. Same doctrine as
# `gen-completions-check`; run by the docs CI job (via `docs-check`).
[group('docs')]
docs-gen-cli-check: docs-gen-cli
    git diff --exit-code -- {{docs_dir}}/src/content/docs/cli/reference.md
    @test -z "$(git status --porcelain -- {{docs_dir}}/src/content/docs/cli/reference.md)" || { git status --porcelain -- {{docs_dir}}/src/content/docs/cli/reference.md; echo "docs-gen-cli-check: cli/reference.md is stale — run 'just docs-gen-cli' and commit"; exit 1; }

# Build the docs-site API-reference input from the overlaid spec (docs-only transforms; see
# codegen/docs-spec.jq): normalize x-codeSamples language labels to Shiki ids, and trim the
# spec's 101-level "Getting Started" intro from info.description. starlight-openapi reads the
# result (codegen/build/openapi.docs.json).
[group('docs')]
docs-spec: spec-overlay
    jq -f codegen/docs-spec.jq {{overlaid_spec}} > {{docs_spec}}
    @echo "Docs API-reference spec -> {{docs_spec}}"

# Local Astro dev server (docs spec + fresh CLI reference generated first). Ctrl-C to stop.
[group('docs')]
docs-dev: docs-spec docs-gen-cli
    cd {{docs_dir}} && npm run dev

# Production build to docs-site/dist/ (Pagefind search index runs). Depends on the docs spec
# (the API-reference source) and the freshly generated CLI reference. The GitHub Pages deploy
# workflow runs exactly this recipe.
[group('docs')]
docs-build: docs-spec docs-gen-cli docs-install
    cd {{docs_dir}} && npm run build

# Serve the built site locally.
[group('docs')]
docs-preview: docs-build
    cd {{docs_dir}} && npm run preview

# The docs CI gate (one recipe, like every other CI job): the CLI-reference drift check, then a
# full production build — a broken Astro/API build or a stale CLI reference fails here.
[group('docs')]
docs-check: docs-gen-cli-check docs-build
