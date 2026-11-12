# Reviewing tests

The law is `CLAUDE.md` → *Testing & verification*. A green `just ci` on the full CI matrix is
the release decision, so a test that cannot fail is worse than no test: it manufactures release
confidence.

## The question to ask of every test

**What wrong implementation would still pass this?** If the answer is "plenty", the test is a
finding. Assertions are pinned to the behaviour claimed: not "it returned Ok", not "the output
is non-empty", not a substring so loose that anything matches.

## Check

1. **A guarantee has a test.** Every documented behaviour (`docs/cli-contract.md`, a rustdoc
   promise, an invariant in `CLAUDE.md`) has a test that fails when it breaks. Name the
   guarantee the change adds or touches, then find its test.
2. **`just test` is hermetic.** No network, no real credentials, no shared state: wiremock for
   HTTP, tempdirs for stores, injected environment lookups. `env::set_var` in a test is a race.
   Anything that needs the live sandbox is behind `just test-sandbox`.
3. **A guard test was break-verified.** The author neutered the code, saw the test fail with a
   message naming the contract, and restored it. A reviewer who doubts it describes the
   mutation in the report, or measures it in a throwaway clone.
4. **Concurrency claims use real concurrency.** A locking, liveness or rotation guarantee is
   tested with genuinely concurrent tasks or lock holders. A sequential test that would pass
   with a no-op lock proves nothing.
5. **Each security invariant has an attack test.** Redaction is tested with a `{:?}` leak
   test, sanitization with escape and forgery payloads, permissions with the mode itself. The
   hostile input, not only the happy path.
6. **Platform code runs on its platform.** A `cfg`-gated branch has a `cfg`-gated test that
   runs on that OS's CI leg. "Compiles locally" does not count.
7. **The user-facing contract is asserted on the binary** (`cli-mcp.md`).
8. **Coverage is a tripwire, not a target.** `just coverage` enforces a floor on the
   hand-written crates. A lowered floor is a deliberate, reviewed decision, and a test with no
   assertion added to move the number is a finding.

## Docs are tested too

- An enumerable doc claim is pinned by a tripwire test
  (`cli/oura-toolkit-cli/tests/docs_tripwire.rs`): the README's command list, the scope string,
  the redirect URI, store paths, recipe names, MCP tool names, and the docs site's enumerable
  claims.
- A change that adds such a claim adds its tripwire. A change that adds an environment variable
  the CLI reads documents it where the tripwire looks.
- Generated pages are drift-checked, not hand-edited.

## Cross-language conformance

`codegen/conformance/auth-cases.json` is the single source for the hostile cases every auth
companion must survive, and all six suites, the Rust reference included, iterate it from the
file. Look for a case added to one language's suite only, and for a fixture changed to let an
implementation pass.

## Gotchas this repo has met

- **A mock that answers anything.** The User-Agent tests use a mock that answers only when the
  exact header is present. A mock that accepts every request cannot show a missing header.
- **A test that loads the wrong build.** The C# `netstandard2.0` leg carries a marker test that
  fails if the suite loaded another target's asset.
- **A detector nobody tests.** The spec-drift detector is watch-only, and its decision logic
  rotted once. Logic that only reports still needs a self-test.
