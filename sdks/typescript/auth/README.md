# @oura-toolkit/auth

Hand-written auth companion for [`@oura-toolkit/api`](../api), part of
[oura-toolkit](https://github.com/spxrogers/oura-toolkit). It keeps an Oura OAuth2 access
token fresh — token store, refresh with rotation, and the `Configuration.accessToken`
seam the generated client expects — so your code never touches the token endpoint by hand.

Non-interactive by design: the browser + loopback consent flow (`oura auth setup` /
`oura auth login`) lives in the `oura` CLI, never in this package. Run it once and every
consumer of the shared token store — the CLI, its MCP server, and this package — picks the
tokens up.

## Install

```sh
npm install @oura-toolkit/api @oura-toolkit/auth
```

CommonJS and ESM both resolve the package (`require` and `import` alike); Node ≥ 18.

## Usage

After `oura auth login`, wire `accessTokenProvider` into the generated client's
`Configuration`. Each request gets a proactively refreshed Bearer token:

```ts
import { Configuration, DailySleepRoutesApi } from "@oura-toolkit/api";
import { TokenManager, accessTokenProvider } from "@oura-toolkit/auth";

const manager = TokenManager.load(); // reads the shared token store
const config = new Configuration({ accessToken: accessTokenProvider(manager) });
const sleep = await new DailySleepRoutesApi(config).multipleDailySleepDocumentsV2UsercollectionDailySleepGet({});
```

Refresh is proactive (an expiry-skew window), so requests normally carry a valid token. If
a request still `401`s (e.g. the token was revoked server-side), call
`manager.forceRefresh()` and retry the request once.

## Token store & credentials

`TokenManager.load()` reads the fixed, invocation-independent store shared with the `oura`
CLI: `~/.config/oura-toolkit/` on Unix/macOS (`$XDG_CONFIG_HOME/oura-toolkit/` if set),
`%LOCALAPPDATA%\oura-toolkit\` on Windows. Records are `0600` (Unix).

Oura is a **confidential client** with **bring-your-own credentials**: you register your
own Oura OAuth application and supply its `client_id` / `client_secret` (via `oura auth
setup`). Refresh is a confidential-client call, so both live in the store's
`credentials.json`; tokens live in `tokens.json` and are rewritten on every rotation.

## Exports

`TokenManager`, `accessTokenProvider`, `TokenStore`, `ClientCredentials`, `Tokens`,
`configDirFrom`; the OAuth metadata constants `AUTHORIZE_URL`, `TOKEN_URL`, `ALL_SCOPES`,
`DEFAULT_SCOPES`; and the error types `AuthError` (base), `NotAuthenticatedError`,
`MissingClientCredentialsError`, `NoConfigDirError`, `StoreFormatError`,
`TokenEndpointError`, `TokenEndpointTransportError`.

See the [toolkit README](../../../README.md) for the Oura app registration walkthrough.
