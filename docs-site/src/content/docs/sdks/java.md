---
title: Java
description: The Java SDK — an openapi-generator client (com.ouratoolkit:api) plus the com.ouratoolkit:auth companion.
---

The Java SDK is the generated client at `sdks/java/api` (reserved as `com.ouratoolkit:api` on
Maven Central) plus the `com.ouratoolkit:auth` companion.

:::caution[Consume from source]
Not yet published to Maven Central. Build it from the repository for now, and supply your own
access token until the companion is published.
:::

## Usage sketch

The client is auth-agnostic — point it at the API base URL, attach your Bearer token, and call
the generated operation for daily sleep:

```java
import com.ouratoolkit.api.ApiClient;
import com.ouratoolkit.api.DefaultApi;

ApiClient client = new ApiClient();
client.setBasePath("https://api.ouraring.com");
client.setBearerToken(System.getenv("OURA_ACCESS_TOKEN"));

DefaultApi api = new DefaultApi(client);
var sleep = api.dailySleep("2026-06-01", "2026-06-07");
sleep.getData().forEach(day -> System.out.println(day.getDay() + " " + day.getScore()));
```

The exact class and method names come from the generated client — see the
[API reference](/api/). Generated code is marked generated; regenerate from the spec rather than
editing it.
