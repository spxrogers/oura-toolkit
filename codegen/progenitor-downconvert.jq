# codegen/progenitor-downconvert.jq — Rust-codegen-only down-convert (OpenAPI 3.1 -> 3.0.3).
#
# progenitor (via the openapiv3 crate) parses OpenAPI 3.0 only; the vendored Oura spec is 3.1.
# openapi-generator (the breadth SDKs) reads 3.1 natively, so this transform is applied ONLY on
# the gen-rust path, AFTER `just spec-overlay`. It does not touch the shared overlaid spec.
#
# The Oura spec's only 3.1-isms are: the version string, and the JSON-Schema nullable idiom
# `{"anyOf"|"oneOf": [ <schema>, {"type":"null"} ]}`. This rewrites those to 3.0's
# `nullable: true`, collapsing a lone remaining branch (wrapping a bare `$ref` in `allOf` so the
# `nullable` sibling is honored). See issue #6.

def denull:
  reduce ("anyOf", "oneOf") as $k (.;
    if ((.[$k]? | type) == "array") and (.[$k] | any(.type? == "null")) then
      (.[$k] | map(select(.type? != "null"))) as $rest
      | .nullable = true
      | del(.[$k])
      | if ($rest | length) == 1 then
          ($rest[0]) as $only
          | if ($only | has("$ref")) then .allOf = ((.allOf // []) + [$only])
            else . * $only
            end
        elif ($rest | length) > 1 then
          .[$k] = $rest
        else .
        end
    else .
    end);

(.openapi = "3.0.3")
| walk(if type == "object" then denull else . end)

# progenitor asserts a single response type per category (success/error). Oura ops pair a typed
# `422` (HTTPValidationError) with several content-less 4xx responses (type "none"), which yields
# two distinct error types and panics the generator. Drop the content-less 4xx/5xx entries on the
# Rust path only — those statuses still surface as `Error::UnexpectedResponse` at runtime, and the
# breadth SDKs (openapi-generator) keep the full-fidelity 3.1 spec. See issue #6.
| .paths |= map_values(
    map_values(
      if (type == "object") and has("responses") then
        .responses |= with_entries(
          select((.key | test("^[45]") | not) or (.value | has("content")))
        )
      else .
      end
    )
  )
