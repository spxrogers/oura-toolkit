# Reviewing oura-toolkit: start here

What a review of this repo checks, on top of whatever method runs it: a single pass, a
self-review before a push, or each lens of the `/review-loop` skill. `CLAUDE.md` is the law and
stays the source for every rule; `DECISIONS.md` says why. These guides say what a reviewer does
with them: what to look for, which check shows it, and what has gone wrong here before. A new
review angle goes in the guide for its area, not in the skill and not in `.reviewlooprc`.

## Which guides to read

Every review reads this file. Then, by what the diff touches:

| The diff touches | Read |
|---|---|
| `sdks/*/…-auth`, `cli/oura-toolkit-cli` auth or store code, anything that handles a token, secret or credential | [`auth.md`](auth.md) |
| `spec/`, `codegen/`, `sdks/*/api`, a `gen-*` recipe, a package or crate name | [`codegen.md`](codegen.md) |
| CLI commands, flags, output or exit codes; `oura mcp`; the plugin under `plugins/` | [`cli-mcp.md`](cli-mcp.md) |
| any test, or any behaviour a document promises | [`tests.md`](tests.md) |
| the justfile, `.github/workflows/`, version pins, `dist` config, `docs-site/` | [`release.md`](release.md) |

The test-rigor lens of `/review-loop` reads `tests.md` whatever the diff touches.

## The change is more than the diff

Each review, and each round of a review loop, asks: **which documented claim does this diff
invalidate?** (`CLAUDE.md` → *Docs stay true to the code*, rule 2.) Then it checks the docs the
change touched against the code: commands, URLs, scopes, recipe names and printed terminal text
are checked against source, never from memory.

The documents a change can make false:

- `README.md`, `CONTRIBUTING.md`, `docs/cli-contract.md`, `plugins/oura-toolkit/README.md`;
- the docs site's hand-written pages under `docs-site/`;
- `ARCHITECTURE.md` and `DECISIONS.md`, when a decision or a fact about the layout changes;
- `CLAUDE.md`, which is rendered: its source is `.agentsync/memory/AGENTS.md`;
- these guides and `.reviewlooprc`;
- code comments, rustdoc promises, and the PR body.

A doc claim that is enumerable (a command list, a scope string, a path, a recipe name, a tool
name) and has no tripwire test pinning it is a finding (`tests.md`).

## Never, in any change

Each of these is in `CLAUDE.md` → *Hard "do not" list*; a change that does one is not ready.

- A second justfile, or a build, test, lint, codegen or release command run or documented
  outside `just`.
- A hand-written transport or HTTP client in any language. `oura api` is the one sanctioned
  exception and is not to be generalised.
- Codegen that touches a hand-written auth companion.
- PKCE, a public-client path, or an embedded default `client_id` or `client_secret`.
- Authorize or token URLs, or scopes, hardcoded instead of read from the spec.
- Interactive OAuth in an SDK or companion.
- A remote or HTTP MCP server, or a second plugin.
- A secret in a URL query string, a log or MCP stdout.
- A documented guarantee without its enforcing test, or a behaviour change without the doc
  updates in the same PR.

A change that relitigates the language, transport, client generation, auth model, layout or
task runner is raised with the owner and waits for an answer; it is not reviewed as a design
choice.

## Running the checks

Every command is a `just` recipe. A reviewer runs recipes, never the raw tool.

- `just ci` — format check, lint and the hermetic test suite. The gate.
- `just coverage` — the line-coverage floor on the hand-written crates.
- `just gen-check`, `just gen-completions-check`, `just docs-gen-cli-check` — generated output
  is in sync with its source.
- `just sdk-check` and the `just sdk-test-<lang>` recipes — the breadth SDKs and their auth
  companions.
- `just version-check`, `just plugin-check`, `just dist-check`, `just publish-check` — version
  pins and release configuration.
- `just docs-check` — the docs site builds and its generated pages are current.
- `just spec-overlay-selftest`, `just spec-drift-selftest` — the overlay and the drift
  detector's own logic.

A read-only reviewer runs a recipe that rewrites files (`just fmt`, `just gen`, `just
set-version`, `just docs-gen-cli`) only in a throwaway clone outside the worktree.

## What a reviewer must not touch

- The real token store: `~/.config/oura-toolkit/` (or `$XDG_CONFIG_HOME/oura-toolkit/`), and
  `%LOCALAPPDATA%\oura-toolkit\` on Windows. Tests use tempdirs.
- A real Oura account. `just auth-*` recipes and `oura auth` are not for a review.
- The live sandbox: `just test-sandbox` and `just test-sandbox-sdks` reach the network and never
  gate CI.
- Anything that publishes or tags: `just release`, `just release-tag`, `just publish`, the
  `just publish-*` and `just sdk-publish-*` recipes, and `just spec-fetch`.

## Severity here

The review loop's rubric (`severity-rubric.md`, beside the skill) rates a finding by what goes
wrong if it merged. Two of its terms, as they apply here:

- **A commitment to users** is what the project says it will never do: embed a shared
  credential, leak a secret to a URL, a log, an error or MCP stdout, weaken the token store's
  permissions or move its path, or run OAuth anywhere but the user's own CLI. Breaking one is a
  `BLOCKER`.
- **Green CI is the release decision** (`CLAUDE.md` → *Testing & verification*). So a property
  release confidence depends on that CI cannot see is at least an `ISSUE`, even when the code
  is right today: the next release ships on that blind spot.

Typical `BLOCKER` shapes here: a rotated refresh token not persisted, so the next refresh
fails; a secret in an error message or `{:?}` output; a wrong `servers[0].url`; a write to
stdout from `oura mcp` outside the protocol; a release workflow that publishes the wrong
artifact.
