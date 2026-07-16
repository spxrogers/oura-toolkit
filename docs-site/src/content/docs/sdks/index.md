---
title: SDKs
description: The same shape in six languages — a generated data-plane client plus a hand-written auth companion.
sidebar:
  order: 1
  label: Overview
---

Every language gets the **same shape**: a data-plane client generated from Oura's OpenAPI spec
(auth-agnostic — bring a Bearer token) plus a hand-written auth companion (token store +
refresh). Rust also hosts the CLI, which runs on its client — so the Rust SDK is dogfooded end
to end.

| Language | Generated client | Auth companion | Package |
|---|---|---|---|
| [Rust](/sdks/rust/) | `sdks/rust/oura-toolkit-api` | `oura-toolkit-auth` | `oura-toolkit-api` / `oura-toolkit-auth` — published ([crates.io](https://crates.io/crates/oura-toolkit-api)) |
| [TypeScript](/sdks/typescript/) | `sdks/typescript/api` | `@oura-toolkit/auth` | `@oura-toolkit/api` / `@oura-toolkit/auth` — published ([npm](https://www.npmjs.com/package/@oura-toolkit/api)) |
| [Python](/sdks/python/) | `sdks/python` (`oura_toolkit.api`) | `oura_toolkit.auth` | `oura-toolkit` — published ([PyPI](https://pypi.org/project/oura-toolkit/)) |
| [Go](/sdks/go/) | `sdks/go` | `sdks/go/auth` | `github.com/spxrogers/oura-toolkit/sdks/go` — published (release-tagged) |
| [Java](/sdks/java/) | `sdks/java/api` | `com.ouratoolkit:auth` | `com.ouratoolkit:api` (Maven Central, reserved) |
| [C#](/sdks/csharp/) | `sdks/csharp/api` | `OuraToolkit.Auth` | `OuraToolkit.Api` / `OuraToolkit.Auth` — published ([NuGet](https://www.nuget.org/packages/OuraToolkit.Api)) |

:::caution[Publishing in progress]
Every breadth client is compile-checked and drift-checked in CI, and smoke-tested against
Oura's live sandbox. Rust (crates.io), TypeScript (npm), Python (PyPI), Go (release-tagged
for `go get`) and C# (NuGet) are published; the Java client is **not yet published** to
Maven Central. Consume it from source for now, and supply your own access token.
:::

## What each client does — and doesn't

- **Does**: type every Oura API v2 operation and model, handle serialization, and issue
  authenticated requests when you attach a Bearer token.
- **Doesn't**: run the interactive OAuth flow (that lives in the CLI) or write to your account
  (the toolkit is read-only). The non-interactive auth companion handles the token store and
  refresh-with-rotation.

Pick your language from the sidebar for a usage sketch. For the endpoints themselves, see the
generated [API reference](/api/).
