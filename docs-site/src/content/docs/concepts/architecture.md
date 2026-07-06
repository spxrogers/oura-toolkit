---
title: Architecture
description: How the pieces of oura-toolkit fit together — the spec-driven pipeline, the shared auth layer, and the CLI/MCP/SDK surfaces.
sidebar:
  order: 1
---

`oura-toolkit` is a Rust CLI for the Oura Ring API v2, plus a STDIO MCP server, a single Claude
plugin, and generated SDK clients in six languages — a cargo workspace and a plugin marketplace
in one monorepo. One idea drives the whole layout: **Oura's OpenAPI spec is the source of
truth, and every language ships the same shape** — a generated data-plane client plus a
hand-written auth companion. Rust is not privileged; it just also happens to host the CLI app.

For the authoritative, always-current map, see
[ARCHITECTURE.md](https://github.com/spxrogers/oura-toolkit/blob/main/ARCHITECTURE.md) in the
repository; this page is the orientation.

## The shape

- **Generated data plane.** The Rust client is generated with progenitor; the TypeScript,
  Python, Go, Java and C# clients with openapi-generator. Every one is pure data-plane and
  auth-agnostic — bring a Bearer token. See the [SDKs](/sdks/).
- **Hand-written auth companion.** The spec generates the data plane but not the interactive
  OAuth flow. A non-interactive auth companion (token store, refresh-with-rotation, a
  cross-process lock, Bearer middleware, OAuth metadata read from the spec) is shared by both
  the CLI's data calls and the MCP tool calls — one auth layer, two consumers. Interactive
  consent (browser + loopback) lives only in the CLI.
- **Three surfaces, one data plane.** The [CLI](/guides/cli-usage/), the
  [MCP server](/guides/mcp-server/), and the [API reference](/api/) all resolve to the same
  spec-derived operations.

## Auth model

Oura is a **confidential OAuth2 client** (Authorization Code flow only; no PKCE), so credentials
are **bring-your-own**: each user registers their own Oura app. The load-bearing invariants —
store paths, file modes, the default port, rotation and locking — are stated once,
authoritatively, in the repository and pinned by tests. See
[Authentication](/guides/authentication/) for the user-facing view.

## Why the docs can't drift

The API reference on this site is generated at build time from the same overlaid spec that
generates the clients, and the [CLI reference](/cli/reference/) is generated from the `oura`
binary. Everything else is pinned by tests that fail CI when a documented claim contradicts the
code. That is the subject of the next page:
[Spec as source of truth](/concepts/spec-source-of-truth/).
