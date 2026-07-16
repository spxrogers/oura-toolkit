# OuraToolkit.Auth

Hand-written auth companion for [`OuraToolkit.Api`](https://www.nuget.org/packages/OuraToolkit.Api),
part of [oura-toolkit](https://github.com/spxrogers/oura-toolkit). It keeps an Oura OAuth2
access token fresh — token store, refresh with rotation, spec-pinned OAuth metadata — so your
code never touches the token endpoint by hand.

Non-interactive by design: the browser + loopback consent flow (`oura auth setup` /
`oura auth login`) lives in the `oura` CLI, never in this package. Run it once and every
consumer of the shared token store — the CLI, its MCP server, and this package — picks the
tokens up. It shares the CLI's on-disk token store (`~/.config/oura-toolkit/` on Unix/macOS,
`%LOCALAPPDATA%\oura-toolkit\` on Windows) and its cross-process refresh protocol, so all of
them can rotate the same tokens safely.

## Install

```sh
dotnet add package OuraToolkit.Api
dotnet add package OuraToolkit.Auth
```

## Usage

After `oura auth login`, let the manager keep the token fresh and hand it to the generated
client's configuration:

```csharp
using OuraToolkit.Api.Api;
using OuraToolkit.Api.Client;
using OuraToolkit.Auth;

var manager = TokenManager.Load(); // reads the shared token store

var config = new Configuration();
config.AccessToken = await manager.GetAccessTokenAsync(); // refreshed + rotated as needed

var api = new DailySleepRoutesApi(config);
```

Targets `netstandard2.0`, `net8.0` and `net10.0`. Bring-your-own-credentials: Oura is a
confidential OAuth client, so you register your own Oura OAuth app (the CLI's
`oura auth setup` walks you through it). MIT licensed.
