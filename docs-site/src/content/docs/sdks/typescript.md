---
title: TypeScript
description: The TypeScript SDK — an openapi-generator client plus the @oura-toolkit/auth companion.
---

The TypeScript SDK is the generated client published as
[`@oura-toolkit/api`](https://www.npmjs.com/package/@oura-toolkit/api) plus the hand-written
[`@oura-toolkit/auth`](https://www.npmjs.com/package/@oura-toolkit/auth) companion — versioned
in lockstep with the toolkit and published on every release tag.

## Install

```sh
npm install @oura-toolkit/api @oura-toolkit/auth
```

## Usage sketch

The client is auth-agnostic — construct it with a base URL and attach your Bearer token, then
call the generated operation for daily sleep:

```ts
import { Configuration, DefaultApi } from "@oura-toolkit/api";

const api = new DefaultApi(
  new Configuration({
    basePath: "https://api.ouraring.com",
    accessToken: process.env.OURA_ACCESS_TOKEN,
  }),
);

const sleep = await api.dailySleep({ startDate: "2026-06-01", endDate: "2026-06-07" });
for (const day of sleep.data) console.log(day.day, day.score);
```

Exact class and method names come from the generated client — browse the
[API reference](/api/) for the operations and models. The generated code is marked generated;
regenerate it from the spec rather than editing by hand.
