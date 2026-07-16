---
title: Go
description: The Go SDK — an openapi-generator client under sdks/go with a companion auth package.
---

The Go SDK is the generated client under `sdks/go` (module
`github.com/spxrogers/oura-toolkit/sdks/go`) plus the companion auth package at `sdks/go/auth`
— versioned in lockstep with the toolkit: every release tags `sdks/go/vX.Y.Z`, so the module
resolves real versions (not pseudo-versions).

## Install

```sh
go get github.com/spxrogers/oura-toolkit/sdks/go
```

## Usage sketch

The client is auth-agnostic — build a configuration with your Bearer token and call the
generated operation for daily sleep:

```go
import oura "github.com/spxrogers/oura-toolkit/sdks/go"

cfg := oura.NewConfiguration()
cfg.Servers = oura.ServerConfigurations{{URL: "https://api.ouraring.com"}}
client := oura.NewAPIClient(cfg)

ctx := context.WithValue(context.Background(), oura.ContextAccessToken, token)
sleep, _, err := client.DefaultAPI.DailySleep(ctx).
    StartDate("2026-06-01").EndDate("2026-06-07").Execute()
```

The exact type and method names come from the generated client — see the
[API reference](/api/). Generated code is marked generated; regenerate from the spec rather than
editing it.
