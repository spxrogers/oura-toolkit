#!/usr/bin/env bash
# version.sh — the single WRITER and single GUARD for the workspace version (#59).
#
# The root Cargo.toml [workspace.package].version is the one source of truth. A handful of
# hand-written manifests must carry the same literal (the generated clients get it injected
# at codegen time; dist versions the installers from Cargo.toml). This script is the ONLY
# thing that writes those literals, and the only thing that checks them — one enumerator,
# so the writer and the guard cannot drift from each other:
#
#   codegen/version.sh check          # every manifest equals the workspace version
#   codegen/version.sh set X.Y.Z      # rewrite the source + every manifest, then re-check
#
# `set` self-verifies by running the check extractors against the new version afterwards:
# if a file's structure changes and a rewrite pattern stops matching, the set FAILS loudly
# instead of silently leaving a manifest behind.
#
# Invoked only via `just set-version` / `just version-check` (single-justfile rule).

set -euo pipefail

usage() {
    echo "usage: codegen/version.sh <check | set NEW_VERSION>" >&2
    exit 2
}

mode="${1-}"
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"

# --- The enumeration: every file that carries the version literal ---------------------------

CARGO="Cargo.toml"                                                   # THE source
TS_PKG="sdks/typescript/auth/package.json"                           # npm @oura-toolkit/auth
PYPROJECT="sdks/python/pyproject.toml"                               # PyPI oura-toolkit dist
POM="sdks/java/auth/pom.xml"                                         # com.ouratoolkit:auth
CSPROJ="sdks/csharp/auth/src/OuraToolkit.Auth/OuraToolkit.Auth.csproj" # OuraToolkit.Auth
PLUGIN_JSON="plugins/oura-toolkit/.claude-plugin/plugin.json"        # plugin version
MCP_JSON="plugins/oura-toolkit/.mcp.json"                            # npx pin oura-toolkit@X

# --- Extractors (used by check, and by set's self-verification) ------------------------------

get_cargo()   { sed -nE 's/^version = "([^"]+)"$/\1/p' "$CARGO" | head -n1; }
# The internal crates' [workspace.dependencies] pins (version + path) must track the
# workspace version too, or `cargo update` fails right after a bump.
get_cargo_dep() { sed -nE "s/^$1 = \\{ version = \"([^\"]+)\".*/\\1/p" "$CARGO" | head -n1; }
get_toml()    { sed -nE 's/^version = "([^"]+)"$/\1/p' "$1" | head -n1; }
# The top-level "version" member (2-space indent — deeper-nested versions never match).
get_json()    { sed -nE 's/^  "version": "([^"]+)",?$/\1/p' "$1" | head -n1; }
# The project <version> is the line directly after <artifactId>auth</artifactId>;
# dependency <version>s live in other elements and are never adjacent to it.
get_pom() {
    awk '{
        if (prev ~ /<artifactId>auth<\/artifactId>/ && match($0, /<version>[^<]+<\/version>/)) {
            print substr($0, RSTART + 9, RLENGTH - 19); exit
        }
        prev = $0
    }' "$POM"
}
get_csproj()  { sed -nE 's/.*<Version>([^<]+)<\/Version>.*/\1/p' "$CSPROJ" | head -n1; }
# The npx pin, with its position asserted: args must be exactly [-y, oura-toolkit@X, mcp]
# (a mispositioned pin would make npx execute the wrong package).
get_mcp() {
    jq -re '.mcpServers.oura.args
            | if length == 3 and .[0] == "-y" and .[2] == "mcp"
                 and (.[1] | startswith("oura-toolkit@"))
              then .[1][13:] else empty end' "$MCP_JSON"
}

# --- check -----------------------------------------------------------------------------------

check_against() {
    want="$1"
    fail=0
    assert() { # file, got
        if [ "$2" != "$want" ]; then
            echo "version drift: $1 carries '${2:-<pattern not found>}' but the workspace version is '$want' — run \`just set-version\`" >&2
            fail=1
        fi
    }
    assert "$CARGO"       "$(get_cargo)"
    assert "$CARGO (oura-toolkit-api dep pin)"  "$(get_cargo_dep oura-toolkit-api)"
    assert "$CARGO (oura-toolkit-auth dep pin)" "$(get_cargo_dep oura-toolkit-auth)"
    assert "$TS_PKG"      "$(get_json "$TS_PKG")"
    assert "$PYPROJECT"   "$(get_toml "$PYPROJECT")"
    assert "$POM"         "$(get_pom)"
    assert "$CSPROJ"      "$(get_csproj)"
    assert "$PLUGIN_JSON" "$(get_json "$PLUGIN_JSON")"
    assert "$MCP_JSON"    "$(get_mcp || true)"
    return "$fail"
}

# --- set (line-targeted awk rewrites: minimal diffs, no whole-file reformatting) --------------

rewrite() { # file, awk-program (NEW is bound to the new version)
    tmp="$(mktemp)"
    awk -v NEW="$2" "$3" "$1" > "$tmp"
    mv "$tmp" "$1"
}

set_all() {
    new="$1"
    first_toml_version='!done && /^version = "/ { sub(/^version = "[^"]+"/, "version = \"" NEW "\""); done = 1 } { print }'
    top_json_version='!done && /^  "version": "/ { sub(/"version": "[^"]+"/, "\"version\": \"" NEW "\""); done = 1 } { print }'
    # Cargo.toml carries the version THREE times: [workspace.package].version (the source)
    # plus the two internal-crate pins in [workspace.dependencies].
    rewrite "$CARGO"       "$new" '
        !done && /^version = "/ { sub(/^version = "[^"]+"/, "version = \"" NEW "\""); done = 1 }
        /^oura-toolkit-(api|auth) = \{ version = "/ { sub(/version = "[^"]+"/, "version = \"" NEW "\"") }
        { print }'
    rewrite "$TS_PKG"      "$new" "$top_json_version"
    rewrite "$PYPROJECT"   "$new" "$first_toml_version"
    rewrite "$POM"         "$new" '{ if (prev ~ /<artifactId>auth<\/artifactId>/ && !done && /<version>/) { sub(/<version>[^<]+<\/version>/, "<version>" NEW "</version>"); done = 1 } print; prev = $0 }'
    rewrite "$CSPROJ"      "$new" '!done && /<Version>/ { sub(/<Version>[^<]+<\/Version>/, "<Version>" NEW "</Version>"); done = 1 } { print }'
    rewrite "$PLUGIN_JSON" "$new" "$top_json_version"
    rewrite "$MCP_JSON"    "$new" '{ gsub(/oura-toolkit@[^"]+/, "oura-toolkit@" NEW); print }'
}

# --- entry -----------------------------------------------------------------------------------

case "$mode" in
check)
    [ $# -eq 1 ] || usage
    want="$(get_cargo)"
    [ -n "$want" ] || { echo "could not read the workspace version from $CARGO" >&2; exit 1; }
    check_against "$want"
    echo "version-check ok: everything is $want"
    ;;
set)
    [ $# -eq 2 ] || usage
    new="$2"
    echo "$new" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+([-+][0-9A-Za-z.-]+)?$' \
        || { echo "'$new' is not a version (want X.Y.Z with optional -pre/+build)" >&2; exit 2; }
    set_all "$new"
    # Self-verify: every extractor must now see the new version — a rewrite pattern that
    # no longer matches its file surfaces HERE, not at the next release.
    check_against "$new" || { echo "set-version rewrote the files but verification failed — a manifest's structure changed; update codegen/version.sh" >&2; exit 1; }
    echo "set $new in: $CARGO $TS_PKG $PYPROJECT $POM $CSPROJ $PLUGIN_JSON $MCP_JSON"
    ;;
*)
    usage
    ;;
esac
