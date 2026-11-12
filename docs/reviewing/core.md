# Reviewing oura-toolkit

How to review a change to this repo: a single pass, a self-review before a push, or each lens of
the `/review-loop` skill.

**This file states no rule about the product.** `CLAUDE.md` is the law and `DECISIONS.md` is the
log of why, and each rule lives in exactly one of them. This file says where a reviewer looks
for a given diff, which recipe shows the answer, and what a reviewer must not touch. If a line
here restates a rule, that is a defect in this file: replace it with a pointer. What it does
hold of its own is about reviewing only: how to run a check without changing the tree, and how
to rate a finding.

## Where to look, by what the diff touches

Every review reads `CLAUDE.md` → *Hard "do not" list* and `CLAUDE.md` → *Docs stay true to the
code*. Then:

| The diff touches | The law (`CLAUDE.md`) | The log (`DECISIONS.md`) | Recipes that show it |
|---|---|---|---|
| Auth, the token store, anything that handles a token, secret or credential | *Auth*; *MCP* (its auth behaviour); *Testing & verification* → the conformance paragraph | *Auth*; *`auth setup`*; both *Token store* entries; the lessons on conformance (#58), the static token (#20), User-Agent and scope changes (#116) | `just ci`, the `just sdk-test-<lang>` recipes |
| `spec/`, `codegen/`, a generated client, a `gen-*` recipe, a package or crate name | *Spec is the source of truth*; *Client / SDK generation*; *Naming* | *Client generation*; *Naming*; all of *Toolchain realities*; the lesson on spec drift (#29) | `just gen-check`, `just sdk-check`, `just spec-overlay-selftest`, `just spec-drift-selftest` |
| CLI commands, flags, output or exit codes | *Testing & verification* rule 7; *Auth* (its commands); `docs/cli-contract.md` | the lesson on `oura api` (#19) | `just ci`, `just docs-gen-cli-check`, `just gen-completions-check` |
| `oura mcp`, the plugin under `plugins/` | *MCP*; *Plugin* | both *MCP* entries; *rmcp* (the router gotcha); the lesson on `serde_json` (#40) | `just ci`, `just plugin-check`, `just version-check` |
| Any test, or any behaviour a document promises | *Testing & verification*, all eight rules | *Lessons from the review loops* | `just ci`, `just coverage` |
| The justfile, `.github/workflows/`, version pins, `dist` config | *Task runner*; *Distribution* | *Single root justfile*; *Version centralization*; *Breadth-SDK publishing*; *CLI npm launcher*; *cargo-dist 0.32 Homebrew limit* | `just version-check`, `just dist-check`, `just publish-check` |
| `docs-site/`, or any shipped doc | *Docs stay true to the code*; *Docs site* | *Documentation site* | `just docs-check` |

The test-rigor lens of `/review-loop` reads `CLAUDE.md` → *Testing & verification* whatever the
diff touches.

## The change is more than the diff

The review gate, and the documents a change can make false, are `CLAUDE.md` → *Docs stay true to
the code*. Four more that list does not name, because they are not shipped surface:

- `CLAUDE.md` itself. It is rendered: the source is `.agentsync/memory/AGENTS.md`.
- This file and `.reviewlooprc`.
- Code comments and rustdoc.
- The PR body.

## Running the checks

Recipes only (`CLAUDE.md` → *Task runner*); the justfile's default recipe lists them.

**A read-only reviewer runs these only in a throwaway clone outside the worktree**, because
they write to the tree:

- the recipes that rewrite or delete files: `just fmt`, `just gen` and each `just gen-*`
  recipe, `just set-version`, `just docs-gen-cli`, `just dist-generate`, `just clean`;
- **the drift checks, which regenerate in place and then compare:** `just gen-check`,
  `just gen-completions-check`, `just docs-gen-cli-check`, and `just docs-check`, which runs the
  last of those. Run in the worktree, they dirty it exactly when there is drift to report.

When unsure whether a recipe writes to the tree, read it in the justfile first, or use the
clone.

## What a reviewer must not touch

- The real token store. Its paths are in `CLAUDE.md` → *Token store*.
- A real Oura account: the `just auth-*` recipes and `oura auth`.
- The live sandbox: `just test-sandbox` and `just test-sandbox-sdks`.
- Anything that tags or publishes: `just release`, `just release-tag`, `just publish`,
  `just publish-cli-npm`, and the `just sdk-publish-*` recipes. `just publish-check` is a check
  and publishes nothing.
- Anything that changes what is vendored or installed: `just spec-fetch`, `just setup`, the
  `just install-*` recipes, `just docs-install`. `just spec-drift-check` only reads, but it
  reaches the network.

## Severity here

The review loop's rubric (`severity-rubric.md`, beside the skill) rates a finding by what goes
wrong if it merged. Two of its terms, as they apply here:

- **A commitment to users** is an item of `CLAUDE.md` → *Hard "do not" list* that protects the
  user: their credentials, their secrets, their token store, where OAuth runs. Breaking one is a
  `BLOCKER`.
- **A test gap.** By the release gate (`CLAUDE.md` → *Testing & verification*), a property that
  release confidence depends on and CI cannot see is at least an `ISSUE`, even when the code is
  right today.

## Adding to this file

The pointers above are pinned by a tripwire test
(`review_guide_pointers_resolve` in `cli/oura-toolkit-cli/tests/docs_tripwire.rs`), so they keep
one form: in the table, an italic span in the second column is the start of a `CLAUDE.md`
heading, and in the third an italic span or an issue number in parentheses is the start of, or
appears in, a `DECISIONS.md` heading; in prose, a file name in backticks, an arrow, then the
heading in italics, with one arrow for each heading. Recipes are written in backticks.

A review that finds a class of defect this repo is prone to records the rule in `CLAUDE.md` or
the lesson in `DECISIONS.md` → *Lessons from the review loops*, and adds the pointer here. An angle that is only about how to
review, and belongs in neither, goes here in full.
