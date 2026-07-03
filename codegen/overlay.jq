# codegen/overlay.jq — codegen overlays applied by `just spec-overlay` BEFORE any generation.
#
# Reads the pristine vendored spec (spec/openapi.json) on stdin and emits the derived
# generation-input spec on stdout. The pristine spec is never edited in place. See issue #5
# and CLAUDE.md ("Known spec issues"). The three fixes:
#
#   1. servers[0].url is the leaked Python `None` ("https://api.None.com") — rewrite it.
#      NON-NEGOTIABLE: nothing resolves without this.
#   2. Every multi-doc response is `anyOf: [TypedResponse, MultiDocumentResponseDict]`; the
#      dict branch produces ugly union return types. Strip it, collapse the now-single-element
#      anyOf into the typed response, and drop the unused schema.
#   3. For the generated CLIENT, narrow the data-plane `[BearerAuth, OAuth2]` alternation to
#      just `[BearerAuth]`. Webhook ops (`[ClientIdAuth + ClientSecretAuth]`) are left intact.

# ---- (1) Fix the server URL -------------------------------------------------------------------
(.servers[0].url) |= "https://api.ouraring.com"

# ---- (2) Strip the MultiDocumentResponseDict branch from multi-doc responses ------------------
| walk(
    if (type == "object")
       and has("anyOf")
       and (.anyOf | map(select(has("$ref") and (."$ref" | test("MultiDocumentResponseDict")))) | length > 0)
    then
      # Drop the dict branch...
      (.anyOf |= map(select((has("$ref") | not) or (."$ref" | test("MultiDocumentResponseDict") | not))))
      # ...and if exactly one branch remains, collapse to it (dropping the wrapper title) so the
      # generated return type is the clean typed response rather than a one-variant union.
      | if (.anyOf | length) == 1
        then (.anyOf[0]) + (del(.anyOf, .title))
        else .
        end
    else .
    end
  )

# Remove the now-unreferenced schema definition so generators don't emit a dead type.
| del(.components.schemas.MultiDocumentResponseDict)

# ---- (3) Narrow client security to BearerAuth (data plane only) -------------------------------
| walk(
    if (type == "array")
       and (map(type == "object" and has("BearerAuth")) | any)
       and (map(type == "object" and has("OAuth2")) | any)
    then [ { "BearerAuth": [] } ]
    else .
    end
  )

# ---- (4) Collapse anyOf:[date-time,date] query params to plain `date` --------------------------
# The spec types every start_date/end_date query param `anyOf: [date-time, date]` (60 hits, all
# in parameters — none in response models). That union generates unusable-to-awkward param types
# in MULTIPLE generators: progenitor rendered it as two #[serde(flatten)] Options (always-failing
# "builder error", caught in PR #36), and openapi-generator's go emits a String/TimeTime union
# struct for what callers should pass as a YYYY-MM-DD string (caught in #15). Oura accepts plain
# dates here and the toolkit only ever sends dates — collapse ONCE for every generator.
| .paths |= map_values(
    map_values(
      if (type == "object") and has("parameters") then
        .parameters |= map(
          if ((.schema.anyOf? | type) == "array")
             and (.schema.anyOf | all(
                    ((.type? == "string") and ((.format? == "date") or (.format? == "date-time")))
                    or (.type? == "null")))
             and (.schema.anyOf | any(.format? == "date"))
          then .schema |= (del(.anyOf) + {type: "string", format: "date"})
          else .
          end
        )
      else .
      end
    )
  )
