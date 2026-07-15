---
title: Rust
description: The Rust SDK — a progenitor-generated client plus the oura-toolkit-auth companion, the same stack the CLI runs on.
---

The Rust SDK is two crates: `oura-toolkit-api` (the progenitor-generated data-plane client) and
`oura-toolkit-auth` (the hand-written, non-interactive auth companion). The `oura` CLI depends
on both directly, so this is the most exercised SDK in the toolkit.

Both crates are published on [crates.io](https://crates.io/crates/oura-toolkit-api). Add them with `cargo add`:

```sh
cargo add oura-toolkit-api oura-toolkit-auth
```

…or add them to your `Cargo.toml` directly:

```toml
[dependencies]
oura-toolkit-api = "0.1"
oura-toolkit-auth = "0.1"
```

## Usage sketch

The client is auth-agnostic — attach a Bearer token (via the auth companion's middleware, or a
raw token for a one-shot). Fetch daily sleep for a date range:

```rust
// Pseudocode shape: a generated `Client` with one method per Oura operation,
// returning typed models. The auth companion supplies the Bearer middleware.
let client = oura_toolkit_api::Client::new_with_client(base_url, http_with_bearer);
let sleep = client.daily_sleep().start_date(start).end_date(end).send().await?;
for day in sleep.into_inner().data {
    println!("{} {:?}", day.day, day.score);
}
```

The generated client is marked `// @generated` — never hand-edit it; regenerate from the spec
instead. See the [API reference](/api/) for every operation and model, and
[Authentication](/guides/authentication/) for how the companion manages tokens.
