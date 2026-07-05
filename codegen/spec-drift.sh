#!/usr/bin/env bash
# spec-drift.sh — detect upstream Oura OpenAPI drift (#29). WATCH-ONLY: it never edits the
# vendored spec. It reports what changed so a human can run the upgrade procedure
# (spec-fetch -> spec-overlay -> gen -> review the generated diff — see CONTRIBUTING).
#
# Two kinds of drift:
#   1. CONTENT — the pinned export URL's bytes now differ from the committed spec/openapi.json
#      (Oura re-published the SAME version number with changes).
#   2. VERSION — a newer openapi-<major>.<minor> export exists upstream than the one we pin.
#
# Invoked via `just spec-drift-check` (real network; the scheduled workflow, NOT `just ci`) and
# exercised hermetically by `just spec-drift-selftest` through the env hooks below (no network).
#
# Usage: spec-drift.sh <spec_version> <spec_url> <spec_file>
#   e.g. spec-drift.sh openapi-1.35 https://api.ouraring.com/v2/static/json/openapi-1.35.json spec/openapi.json
#
# Test hooks (UNSET in production — the selftest sets them to avoid the network):
#   OURA_SPEC_DRIFT_UPSTREAM_FILE   use this local file as "upstream content" instead of curl
#   OURA_SPEC_DRIFT_KNOWN_MINORS    space-separated minor numbers to treat as the "existing"
#                                   upstream exports instead of probing (empty = none exist)
#   OURA_SPEC_DRIFT_PROBE_COUNT     how many minors past the pinned one to probe (default 20)
#
# Exit codes: 0 = no drift; 1 = drift detected (markdown report on stdout); 2 = hard error.

set -euo pipefail

spec_version="${1:?usage: spec-drift.sh <spec_version> <spec_url> <spec_file>}"
spec_url="${2:?usage: spec-drift.sh <spec_version> <spec_url> <spec_file>}"
spec_file="${3:?usage: spec-drift.sh <spec_version> <spec_url> <spec_file>}"

# openapi-<major>.<minor>  ->  major / minor
if [[ ! "$spec_version" =~ ^openapi-([0-9]+)\.([0-9]+)$ ]]; then
  echo "spec-drift: cannot parse version '$spec_version' (expected openapi-<major>.<minor>)" >&2
  exit 2
fi
major="${BASH_REMATCH[1]}"
minor="${BASH_REMATCH[2]}"
# ".../json/openapi-1.35.json" -> ".../json/openapi-1." so we can probe "<base><n>.json".
url_prefix="${spec_url%"${minor}.json"}"

report=""
drift=0

# --- 1. content drift ------------------------------------------------------------------------
tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
if [[ -n "${OURA_SPEC_DRIFT_UPSTREAM_FILE:-}" ]]; then
  cp "$OURA_SPEC_DRIFT_UPSTREAM_FILE" "$tmp"
elif ! curl -fsS "$spec_url" -o "$tmp"; then
  echo "spec-drift: failed to fetch the pinned export $spec_url" >&2
  exit 2
fi
if ! cmp -s "$tmp" "$spec_file"; then
  drift=1
  changed=$(diff "$spec_file" "$tmp" | grep -c '^[<>]' || true)
  report+="## Content drift on the pinned export\n\n"
  report+="The pinned \`$spec_version\` export no longer matches the committed \`$spec_file\` "
  report+="(~$changed changed lines) — Oura re-published the same version number with changes.\n\n"
fi

# --- 2. version drift ------------------------------------------------------------------------
newer=()
if [[ -n "${OURA_SPEC_DRIFT_KNOWN_MINORS+x}" ]]; then
  for m in ${OURA_SPEC_DRIFT_KNOWN_MINORS}; do
    ((m > minor)) && newer+=("$major.$m")
  done
else
  probe_count="${OURA_SPEC_DRIFT_PROBE_COUNT:-20}"
  for ((m = minor + 1; m <= minor + probe_count; m++)); do
    # No -f: a 404 must yield its status code quietly, not an error. `000` marks a hard
    # network failure (connection refused/DNS) — treated as "not present", never a false 200.
    code=$(curl -s -o /dev/null -w '%{http_code}' "${url_prefix}${m}.json" || echo 000)
    [[ "$code" == "200" ]] && newer+=("$major.$m")
  done
fi
if ((${#newer[@]} > 0)); then
  drift=1
  report+="## A newer export is available\n\n"
  report+="Pinned: \`$major.$minor\`. Found upstream: ${newer[*]}. "
  report+="Bump \`spec_version\` in the justfile to the newest before re-fetching.\n\n"
fi

# --- report / exit ---------------------------------------------------------------------------
if ((drift == 1)); then
  printf '%b' "# Oura OpenAPI spec drift\n\n${report}"
  printf '%b' "## Upgrade procedure\n\n"
  printf '%b' "\`\`\`sh\njust spec-fetch     # bump \`spec_version\` first if a newer export exists\n"
  printf '%b' "just spec-overlay\njust gen            # then run the double-\`gen\` zero-diff check\n\`\`\`\n"
  printf '%b' "Review the generated diff and land it through the usual review loop.\n"
  exit 1
fi

echo "spec-drift: no drift — pinned $spec_version matches upstream and no newer export exists."
exit 0
