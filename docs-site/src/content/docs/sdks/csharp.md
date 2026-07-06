---
title: C#
description: The C# SDK — an openapi-generator client (OuraToolkit.Api) plus the OuraToolkit.Auth companion.
---

The C# SDK is the generated client at `sdks/csharp/api` (reserved as `OuraToolkit.Api` on
NuGet) plus the `OuraToolkit.Auth` companion. The client multi-targets modern .NET and
`netstandard2.0`.

:::caution[Consume from source]
Not yet published to NuGet. Build it from the repository for now, and supply your own access
token until the companion is published.
:::

## Usage sketch

The client is auth-agnostic — configure the base URL and your Bearer token, then call the
generated operation for daily sleep:

```csharp
using OuraToolkit.Api;
using OuraToolkit.Client;

var config = new Configuration
{
    BasePath = "https://api.ouraring.com",
    AccessToken = Environment.GetEnvironmentVariable("OURA_ACCESS_TOKEN"),
};

var api = new DefaultApi(config);
var sleep = api.DailySleep("2026-06-01", "2026-06-07");
foreach (var day in sleep.Data) Console.WriteLine($"{day.Day} {day.Score}");
```

The exact class and method names come from the generated client — see the
[API reference](/api/). Generated code is marked generated; regenerate from the spec rather than
editing it.
