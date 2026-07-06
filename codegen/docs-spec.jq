# Docs-only spec transforms (applied AFTER the shared overlay, ONLY for the documentation site).
#
# Kept OUT of codegen/overlay.jq on purpose: that overlay feeds real code generation (Rust +
# breadth SDKs) and must stay a minimal, generation-focused set of fixes. Everything here is a
# docs-only presentation concern, run only by `just docs-spec` into codegen/build/openapi.docs.json.
#
# Two transforms:
#  1. Normalize `x-codeSamples` language labels. Oura's spec labels its per-operation request
#     examples with display names — "cURL", "Python", "JavaScript", "Java" — that are NOT Shiki
#     grammar ids, so starlight-openapi's highlighter falls back to unhighlighted plain text
#     (hundreds of build warnings). Map each to a real Shiki id.
#  2. Trim the `info.description` intro. Oura's description opens with a "# Getting Started"
#     section ("What is an API?", a Quick Start Guide, Common Questions) aimed at first-time API
#     users. The docs site is for developers and has its own Guides, so drop that 101-level block
#     (from the "# Getting Started" heading up to the next top-level "# " heading) while keeping
#     the genuine reference material that follows (Data Access, Authentication, Response Codes,
#     Rate Limits).

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
def normalize_code_samples:
  walk(
    if (type == "object") and (has("lang")) and (has("source"))
    then .lang |= (lang_map[.] // ascii_downcase)
    else .
    end
  );

# Drop the "# Getting Started" section from a Markdown string: skip lines from that heading up to
# (but not including) the next top-level "# " heading. `##`/`###` subheadings do not terminate it.
def trim_intro_section:
  (. / "\n") as $lines
  | reduce $lines[] as $line ({out: [], skipping: false};
      if ($line | test("^#\\s+Getting Started\\s*$")) then .skipping = true
      elif .skipping and ($line | test("^#\\s")) then .skipping = false | .out += [$line]
      elif .skipping then .
      else .out += [$line]
      end)
  | .out | join("\n");

normalize_code_samples
| if (.info.description | type) == "string"
  then .info.description |= trim_intro_section
  else .
  end
