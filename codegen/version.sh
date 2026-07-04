#!/usr/bin/env bash
# version.sh — the single WRITER and single GUARD for the workspace version (#59).
#
# The root Cargo.toml [workspace.package].version is the one source of truth. A handful of
# hand-written manifests must carry the same literal (the generated clients get it injected
# at codegen time; dist versions the installers from Cargo.toml; Go's version is the git
# tag — no file). This script is the ONLY thing that writes those literals, and the only
# thing that checks them — one enumerator, so the writer and the guard cannot drift from
# each other:
#
#   codegen/version.sh check          # every manifest equals the workspace version,
#                                     # then round-trips the WRITER against a temp copy
#   codegen/version.sh set X.Y.Z      # rewrite the source + every manifest, then re-check
#
# Safety properties, each of which fails LOUDLY rather than silently skipping a file:
#  - every extractor demands EXACTLY ONE match per file — a manifest that grows a second
#    version-shaped line is ambiguous and fails check AND set (a first-match extractor
#    would let the writer and the guard lock onto the same wrong line);
#  - `set` self-verifies by running the check extractors against the new version;
#  - `check` additionally exercises the full writer against a temp copy of the manifests,
#    so a rewrite pattern broken by a manifest edit turns CI red on that PR, not at the
#    next release.
#
# Adding a NEW version-bearing manifest? Add it to the enumeration below (extractor +
# rewriter + the two check_against/set_all lines) — nothing discovers it for you.
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

# All file access goes through $BASE so check mode can point the whole enumeration at a
# temp copy for the writer round-trip.
BASE="$root"

# --- The enumeration: every file that carries the version literal ---------------------------

CARGO="Cargo.toml"                                                   # THE source (3 literals)
TS_PKG="sdks/typescript/auth/package.json"                           # npm @oura-toolkit/auth
PYPROJECT="sdks/python/pyproject.toml"                               # PyPI oura-toolkit dist
POM="sdks/java/auth/pom.xml"                                         # com.ouratoolkit:auth
CSPROJ="sdks/csharp/auth/src/OuraToolkit.Auth/OuraToolkit.Auth.csproj" # OuraToolkit.Auth
PLUGIN_JSON="plugins/oura-toolkit/.claude-plugin/plugin.json"        # plugin version
MCP_JSON="plugins/oura-toolkit/.mcp.json"                            # npx pin oura-toolkit@X

ALL_FILES() { printf '%s\n' "$CARGO" "$TS_PKG" "$PYPROJECT" "$POM" "$CSPROJ" "$PLUGIN_JSON" "$MCP_JSON"; }

# --- Extractors (used by check, and by set's self-verification) ------------------------------
# Each demands exactly one match; zero or many yield a non-version marker so the caller's
# comparison fails with a message naming the problem.

exactly_one() {
    local matches n
    matches="$(cat)"
    n="$(printf '%s' "$matches" | grep -c . || true)"
    if [ "$n" -eq 1 ]; then printf '%s' "$matches"; else printf '<%s matches>' "$n"; fi
}

get_cargo()   { sed -nE 's/^version = "([^"]+)"$/\1/p' "$BASE/$CARGO" | exactly_one; }
# The internal crates' [workspace.dependencies] pins (version + path) must track the
# workspace version too, or `cargo update` fails right after a bump.
get_cargo_dep() { sed -nE "s/^$1 = \\{ version = \"([^\"]+)\".*/\\1/p" "$BASE/$CARGO" | exactly_one; }
get_toml()    { sed -nE 's/^version = "([^"]+)"$/\1/p' "$BASE/$1" | exactly_one; }
# The top-level "version" member (2-space indent — deeper-nested versions never match).
get_json()    { sed -nE 's/^  "version": "([^"]+)",?$/\1/p' "$BASE/$1" | exactly_one; }
# The project <version> is the line directly after <artifactId>auth</artifactId>;
# dependency <version>s live in other elements and are never adjacent to it.
get_pom() {
    awk '{
        if (prev ~ /<artifactId>auth<\/artifactId>/ && match($0, /<version>[^<]+<\/version>/)) {
            print substr($0, RSTART + 9, RLENGTH - 19)
        }
        prev = $0
    }' "$BASE/$POM" | exactly_one
}
get_csproj()  { sed -nE 's/.*<Version>([^<]+)<\/Version>.*/\1/p' "$BASE/$CSPROJ" | exactly_one; }
# The npx pin, with its position asserted: args must be exactly [-y, oura-toolkit@X, mcp]
# (a mispositioned pin would make npx execute the wrong package).
get_mcp() {
    jq -re '.mcpServers.oura.args
            | if length == 3 and .[0] == "-y" and .[2] == "mcp"
                 and (.[1] | startswith("oura-toolkit@"))
              then .[1][13:] else empty end' "$BASE/$MCP_JSON"
}

# --- check -----------------------------------------------------------------------------------

check_against() {
    local want fail
    want="$1"
    fail=0
    assert() { # file, got
        if [ "$2" != "$want" ]; then
            echo "version drift: $1 carries '${2:-<pattern not found>}' but the workspace version is '$want' — run \`just set-version $want\`" >&2
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

rewrite() { # file, new-version, awk-program
    local tmp
    tmp="$(mktemp)"
    awk -v NEW="$2" "$3" "$BASE/$1" > "$tmp"
    # cat-into (not mv) keeps the destination inode, preserving the file's permissions.
    cat "$tmp" > "$BASE/$1"
    rm -f "$tmp"
}

set_all() {
    local new first_toml_version top_json_version
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

# The writer round-trip (run by check): copy the enumerated manifests into a temp tree,
# run the full writer at a sentinel version, and verify with the check extractors. Proves
# on every PR that each rewrite pattern still matches its manifest — a broken rewriter
# otherwise stays invisible until the next release bump.
writer_selftest() {
    local tmp
    tmp="$(mktemp -d)"
    ALL_FILES | while IFS= read -r f; do
        mkdir -p "$tmp/$(dirname "$f")"
        cp "$root/$f" "$tmp/$f"
    done
    BASE="$tmp"
    set_all "9.9.9"
    if ! check_against "9.9.9" 2>/dev/null; then
        BASE="$root"
        echo "writer self-test FAILED: \`set\` no longer rewrites every manifest (a manifest's structure changed?) — update codegen/version.sh" >&2
        rm -rf "$tmp"
        return 1
    fi
    BASE="$root"
    rm -rf "$tmp"
}

# --- entry -----------------------------------------------------------------------------------

case "$mode" in
check)
    [ $# -eq 1 ] || usage
    want="$(get_cargo)"
    case "$want" in
    '' | '<'*) echo "could not read exactly one workspace version from $CARGO (got '${want:-nothing}')" >&2; exit 1 ;;
    esac
    check_against "$want"
    writer_selftest
    echo "version-check ok: everything is $want (writer self-test passed)"
    ;;
set)
    [ $# -eq 2 ] || usage
    new="$2"
    echo "$new" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$' \
        || { echo "'$new' is not a version (want X.Y.Z with optional -pre and/or +build)" >&2; exit 2; }
    set_all "$new"
    # Self-verify: every extractor must now see EXACTLY the new version — a rewrite pattern
    # that no longer matches (or a file with an ambiguous second version line) surfaces
    # HERE, not at the next release.
    check_against "$new" || { echo "set-version rewrote the files but verification failed — a manifest's structure changed; update codegen/version.sh" >&2; exit 1; }
    echo "set $new in: $(ALL_FILES | tr '\n' ' ')"
    ;;
*)
    usage
    ;;
esac
