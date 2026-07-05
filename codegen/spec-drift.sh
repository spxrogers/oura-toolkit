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
#   OURA_SPEC_DRIFT_PROBE_DIR       probe local fixture files (openapi-<maj>.<min>.json) in this
#                                   dir instead of the network — runs the real probe loop + the
#                                   soft-404 gate hermetically
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
# ".../json/openapi-1.35.json" -> ".../json/openapi-" so we can probe "<root><maj>.<min>.json".
url_root="${spec_url%"${major}.${minor}.json"}"

report=""
drift=0

# --- 1. content drift ------------------------------------------------------------------------
tmp="$(mktemp)"
tmp2="$(mktemp)"
trap 'rm -f "$tmp" "$tmp2"' EXIT
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

# Is the body in $tmp2 REALLY an OpenAPI export? Guards against a soft-404 (a 200 serving an
# HTML/error page): without this, a CDN error page would spam the report with bogus versions.
is_openapi_doc() { jq -e 'has("openapi") and (.info.title | type == "string")' "$1" >/dev/null 2>&1; }

# Probe one candidate "<maj>.<min>"; append to `newer` only if the export really exists.
probe_version() {
  local v="$1"
  local url="${url_root}${v}.json"
  local code
  if [[ -n "${OURA_SPEC_DRIFT_PROBE_DIR:-}" ]]; then
    # Hermetic (selftest): a local fixture stands in for a live export; absent = 404. Runs the
    # SAME loop + soft-404 gate as production, just without the network.
    if [[ -f "${OURA_SPEC_DRIFT_PROBE_DIR}/openapi-${v}.json" ]]; then
      cp "${OURA_SPEC_DRIFT_PROBE_DIR}/openapi-${v}.json" "$tmp2"
      code=200
    else
      code=404
    fi
  else
    # No -f: a 404 must yield its status code quietly. `000` = hard network failure.
    code=$(curl -s -o "$tmp2" -w '%{http_code}' "$url" || echo 000)
  fi
  case "$code" in
    200)
      if is_openapi_doc "$tmp2"; then
        newer+=("$v")
      else
        echo "spec-drift: $url returned 200 but is not an OpenAPI doc (soft-404?) — ignoring" >&2
      fi
      ;;
    404 | 000) : ;; # absent / unreachable
    *) echo "spec-drift: $url returned HTTP $code — treating as absent (transient?)" >&2 ;;
  esac
}

# Newer minors of the current major, PLUS the next major's .0 — a major bump keeps the pinned
# URL serving the old bytes, so content drift alone would never catch it.
probe_count="${OURA_SPEC_DRIFT_PROBE_COUNT:-20}"
for ((m = minor + 1; m <= minor + probe_count; m++)); do
  probe_version "$major.$m"
done
probe_version "$((major + 1)).0"
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
