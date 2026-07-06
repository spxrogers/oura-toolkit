---
title: Spec as source of truth
description: How this documentation stays in sync with the code — a generated API reference, a generated CLI reference, and tripwire-tested claims.
sidebar:
  order: 2
---

The hardest problem for any documentation site is drift: docs that quietly stop matching the
code. `oura-toolkit` treats a doc claim that contradicts the code as a **bug of the same
severity** as the code change that orphaned it — and mechanizes the sync so CI catches drift
instead of relying on a reviewer noticing.

This site is wired into that discipline three ways.

## 1. The API reference is generated from the spec

Oura's OpenAPI spec is vendored in the repository and drives everything downstream — the Rust
client, the MCP tools, and the SDKs. This site's [API reference](/api/) is built from that same
spec at build time (after a small overlay that fixes known upstream defects, such as a leaked
server URL). Because the reference is generated on every build, it *is* the spec — it can't
fall out of sync. When Oura publishes a spec change, a scheduled drift check flags it.

## 2. The CLI reference is generated from the binary

The [CLI reference](/cli/reference/) is generated directly from the `oura` binary's own command
definitions — the same source that produces its `--help`, shell completions, and man page. It
is committed and drift-checked: regenerating it and finding a difference fails CI, so the
documented flags, defaults, and help text always match the binary.

## 3. Everything else is pinned by tripwire tests

The hand-written guides restate as few hard facts as possible, and every enumerable fact they
*do* state is pinned to its source by a test that fails CI on drift:

- the command and `oura auth` subcommand lists ⟷ the binary
- the OAuth scope string ⟷ the spec-derived default
- the redirect URI and default port ⟷ the CLI's own default
- the token-store paths ⟷ the auth crate's locked directory name
- the MCP tool names ⟷ the server's registered tools
- the environment overrides ⟷ the variables the code reads
- the rate-limit numbers ⟷ the constants the code enforces
- the SDK language set ⟷ the languages the build actually generates

If any of these changes in the code without the docs following, the build goes red. Prose tone
and accuracy remain a review responsibility — but the facts are the machine's job.
