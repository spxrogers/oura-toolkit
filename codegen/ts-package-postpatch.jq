# ts-package-postpatch.jq — post-patch for the GENERATED sdks/typescript/api/package.json (#57).
#
# ("post-patch", not "overlay": in this repo an overlay is the SPEC transform applied
# BEFORE generation — this runs AFTER, on the generator's output, like gen-csharp's.)
#
# typescript-fetch 7.14.0 emits npm packaging with three defects (the same ones the
# @oura-toolkit/auth companion fixed for itself in PR #51):
#   1. a dual-ESM build that is broken in Node: dist/esm/*.js carries ES-module syntax
#      inside a CJS package (no "type": "module", no dist/esm marker), so only bundlers
#      reading the non-standard "module" field can use it — Node errors out;
#   2. no "exports" map, so modern Node/TS resolution (node16/bundler) has no entry and
#      deep imports are unencapsulated;
#   3. placeholder metadata (author "OpenAPI-Generator", repository GIT_USER_ID/…) and no
#      files allowlist (npm pack would ship src/ and tsconfigs).
#
# This post-patch rewrites the manifest to the companion's reviewed CJS-only shape: one
# tsc build, an exports map with types+default conditions, real repository metadata, and
# a dist-only publish surface. Applied (and guarded) by `just gen-ts`; the injected
# name/version pass through untouched.

del(.author)
| .description = "Generated typescript-fetch client for the Oura Ring API v2 — part of oura-toolkit"
# Publish metadata (#96): the generator emits no license field, and npm shows the package as
# "Unlicensed" without one. MIT, same as the repo LICENSE (gen-ts copies that file into the
# package so `npm pack` ships it alongside this declaration).
| .license = "MIT"
| del(.module)
| del(.typings)
| .repository = {
    type: "git",
    url: "https://github.com/spxrogers/oura-toolkit.git",
    directory: "sdks/typescript/api"
  }
| .types = "./dist/index.d.ts"
| .exports = { ".": { types: "./dist/index.d.ts", default: "./dist/index.js" } }
| .engines = { node: ">=18" }
| .files = ["dist"]
# Single CJS build; clean dist/ first (zero-dep, cross-platform) so a publish from a
# dirty tree can never ship stale dist/esm artifacts.
| .scripts.build = "node -e \"require('fs').rmSync('dist',{recursive:true,force:true})\" && tsc"
# Parity with the companion's reviewed range (4.9+ for the `exports` types condition era).
| .devDependencies.typescript = "^4.9 || ^5.0"
