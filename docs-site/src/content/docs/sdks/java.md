---
title: Java
description: The Java SDK — an openapi-generator client (com.ouratoolkit:api) plus the com.ouratoolkit:auth companion.
---

The Java SDK is the generated client published as
[`com.ouratoolkit:api`](https://central.sonatype.com/artifact/com.ouratoolkit/api) on Maven
Central plus the
[`com.ouratoolkit:auth`](https://central.sonatype.com/artifact/com.ouratoolkit/auth)
companion — versioned in lockstep with the toolkit and published (signed, with sources and
javadoc) on every release tag.

## Install

Replace `$PREFERRED_VERSION` with the [latest release](https://github.com/spxrogers/oura-toolkit/releases):

```xml
<dependency>
    <groupId>com.ouratoolkit</groupId>
    <artifactId>api</artifactId>
    <version>$PREFERRED_VERSION</version>
</dependency>
<dependency>
    <groupId>com.ouratoolkit</groupId>
    <artifactId>auth</artifactId>
    <version>$PREFERRED_VERSION</version>
</dependency>
```

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
