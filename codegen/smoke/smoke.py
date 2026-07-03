# Python-client live smoke against Oura's sandbox (network; run via
# `just test-sandbox-sdks`, never CI). Proves the generated client end-to-end: config,
# bearer injection, request, and pydantic response parsing. The sandbox accepts any bearer
# value; override with OURA_SANDBOX_TOKEN if that ever changes.
import datetime
import os
import sys

from oura_toolkit.api import ApiClient, Configuration
from oura_toolkit.api.api import SandboxRoutesApi

config = Configuration(access_token=os.environ.get("OURA_SANDBOX_TOKEN", "sandbox-smoke"))
with ApiClient(config) as client:
    res = SandboxRoutesApi(client).sandbox_multiple_daily_sleep_documents_v2_sandbox_usercollection_daily_sleep_get(
        start_date=datetime.date(2026, 6, 25), end_date=datetime.date(2026, 7, 1)
    )
    if not isinstance(res.data, list):
        sys.exit("py smoke FAILED: no data array")
    first = res.data[0].day if res.data else None
    print(f"py smoke OK: {len(res.data)} sandbox daily_sleep docs, first day {first}")
