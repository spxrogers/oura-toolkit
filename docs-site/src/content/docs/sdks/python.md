---
title: Python
description: The Python SDK — a single oura-toolkit distribution with oura_toolkit.api and oura_toolkit.auth modules.
---

Python ships as a single distribution (reserved as `oura-toolkit` on PyPI) with two modules:
`oura_toolkit.api` (the generated data-plane client) and `oura_toolkit.auth` (the auth
companion) — no per-function micro-packages.

:::caution[Consume from source]
Not yet published to PyPI. Install from the repository for now, and supply your own access token
until the companion is published.
:::

## Usage sketch

The client is auth-agnostic — configure it with a base URL and your Bearer token, then call the
generated operation for daily sleep:

```python
from oura_toolkit.api import ApiClient, Configuration, DefaultApi

config = Configuration(host="https://api.ouraring.com")
config.access_token = os.environ["OURA_ACCESS_TOKEN"]

with ApiClient(config) as client:
    api = DefaultApi(client)
    sleep = api.daily_sleep(start_date="2026-06-01", end_date="2026-06-07")
    for day in sleep.data:
        print(day.day, day.score)
```

The exact class and method names come from the generated client — see the
[API reference](/api/). Generated modules are marked generated; regenerate from the spec rather
than editing them.
