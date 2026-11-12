# Reviewing oura-toolkit

How to review a change to this repo: a single pass, a self-review before a push, or each lens of
the `/review-loop` skill.

**This file states no rule.** `CLAUDE.md` is the law and `DECISIONS.md` is the log of why, and
each rule lives in exactly one of them. This file says where a reviewer looks for a given diff,
which recipe shows the answer, and what a reviewer must not touch. If a line here restates a
rule, that is a defect in this file: replace it with a pointer.

## Where to look, by what the diff touches

Every review reads `CLAUDE.md` → *Hard "do not" list* and *Docs stay true to the code*. Then:

| The diff touches | The law (`CLAUDE.md`) | The log (`DECISIONS.md`) | Recipes that show it |
|---|---|---|---|
| Auth, the token store, anything that handles a token, secret or credential | *Auth*; *Testing & verification* → the conformance paragraph | *Auth*; both *Token store* entries; the lessons on conformance (#58), the static token (#20), User-Agent and scope changes (#116) | `just ci`, the `just sdk-test-<lang>` recipes |
| `spec/`, `codegen/`, a generated client, a `gen-*` recipe, a package or crate name | *Spec is the source of truth*; *Client / SDK generation*; *Naming* | *Client generation*; *Naming*; all of *Toolchain realities*; the lesson on spec drift (#29) | `just gen-check`, `just sdk-check`, `just spec-overlay-selftest`, `just spec-drift-selftest` |
| CLI commands, flags, output or exit codes | *Testing & verification* rule 7; `docs/cli-contract.md` | the lesson on `oura api` (#19) | `just ci`, `just docs-gen-cli-check`, `just gen-completions-check` |
| `oura mcp`, the plugin under `plugins/` | *MCP*; *Plugin* | both *MCP* entries; the rmcp router gotcha; the lesson on `serde_json` (#40) | `just ci`, `just plugin-check`, `just version-check` |
| Any test, or any behaviour a document promises | *Testing & verification*, all eight rules | *Lessons from the review loops* | `just ci`, `just coverage` |
| The justfile, `.github/workflows/`, version pins, `dist` config | *Task runner*; *Distribution* | *Single root justfile*; *Version centralization*; *Breadth-SDK publishing*; *CLI npm launcher* | `just version-check`, `just dist-check`, `just publish-check` |
| `docs-site/`, or any shipped doc | *Docs stay true to the code*; *Docs site* | *Documentation site* | `just docs-check` |

The test-rigor lens of `/review-loop` reads *Testing & verification* whatever the diff touches.

## The change is more than the diff

The review gate, and the documents a change can make false, are `CLAUDE.md` → *Docs stay true to
the code*. Four more that list does not name, because they are not shipped surface:

- `CLAUDE.md` itself. It is rendered: the source is `.agentsync/memory/AGENTS.md`.
- This file and `.reviewlooprc`.
- Code comments and rustdoc.
- The PR body.

## Running the checks

A reviewer runs recipes, never the raw tool; the justfile's default recipe lists them. A
read-only reviewer runs a recipe that rewrites files (`just fmt`, `just gen`, `just
set-version`, `just docs-gen-cli`) only in a throwaway clone outside the worktree.

## What a reviewer must not touch

- The real token store. Its paths are in `CLAUDE.md` → *Token store*.
- A real Oura account: the `just auth-*` recipes and `oura auth`.
- The live sandbox: `just test-sandbox` and `just test-sandbox-sdks`.
- Anything that fetches, tags or publishes: `just spec-fetch`, `just release`,
  `just release-tag`, `just publish`, and the `just publish-*` and `just sdk-publish-*` recipes.

## Severity here

The review loop's rubric (`severity-rubric.md`, beside the skill) rates a finding by what goes
wrong if it merged. Two of its terms, as they apply here:

- **A commitment to users** is an item of the *Hard "do not" list* that protects the user: their
  credentials, their secrets, their token store, where OAuth runs. Breaking one is a `BLOCKER`.
- **A test gap.** Green CI is the release decision here, so a property that release confidence
  depends on and CI cannot see is at least an `ISSUE`, even when the code is right today.

## Adding to this file

A review that finds a class of defect this repo is prone to records the rule in `CLAUDE.md` or
the lesson in `DECISIONS.md`, and adds the pointer here. An angle that is only about how to
review, and belongs in neither, goes here in full.
