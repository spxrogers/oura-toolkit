# Docs-only spec transform (applied AFTER the shared overlay, only for the documentation site).
#
# Oura's spec ships per-operation request examples under `x-codeSamples`, labelled with display
# names — "cURL", "Python", "JavaScript", "Java" — that are NOT Shiki grammar ids. starlight-openapi
# renders these samples through Expressive Code (Shiki), which then can't find the grammar and
# falls back to unhighlighted plain text (hundreds of build warnings). Map each label to a real
# Shiki id so the API-reference request samples are syntax-highlighted.
#
# Kept OUT of codegen/overlay.jq on purpose: that overlay feeds real code generation (Rust +
# breadth SDKs) and must stay a minimal, generation-focused set of fixes. This one is a docs-only
# presentation concern, run only by `just docs-spec`.

def lang_map:
  {
    "cURL": "bash",
    "curl": "bash",
    "Shell": "bash",
    "Python": "python",
    "JavaScript": "javascript",
    "TypeScript": "typescript",
    "Java": "java",
    "Go": "go",
    "C#": "csharp",
    "PHP": "php",
    "Ruby": "ruby"
  };

# Rewrite `.lang` only on x-codeSamples-shaped objects (both `lang` and `source` present), so no
# unrelated `lang` key elsewhere in the spec is touched. Unknown labels fall back to lowercase.
walk(
  if (type == "object") and (has("lang")) and (has("source"))
  then .lang |= (lang_map[.] // ascii_downcase)
  else .
  end
)
