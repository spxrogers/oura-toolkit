# Reviewing auth, the token store and secrets

The law is `CLAUDE.md` → *Auth*; the reasons are in `DECISIONS.md` (*Auth*, *Token store*).
This is the hand-written part of the repo and the one where a quiet mistake costs a user their
login or their secret.

## Never

- PKCE or a public-client path: Oura is a confidential client and the token endpoint needs
  `client_id` and `client_secret`.
- A shared or default credential. Each user brings their own.
- An authorize URL, token URL or scope written in code. The companion reads them from the
  vendored spec.
- Browser or loopback OAuth outside `cli/oura-toolkit-cli`. `sdks/rust/oura-toolkit-auth` and
  every other companion are non-interactive.
- A local HTTP form for credentials. The loopback listener is the OAuth callback only, and it
  answers GET only.
- A secret in a URL query string, a log line, an error message, `Debug` output or MCP stdout.
- OAuth over the wire for the MCP server, or any prompt or browser launch from `oura mcp`.

## Check

- **Rotation.** Oura rotates the refresh token on every refresh and invalidates the old one.
  The newly returned token is persisted, or the next refresh fails with a 400.
- **Two processes, one store.** The CLI and a long-running MCP server share it. A refresh takes
  the exclusive advisory lock, re-reads the store first so it adopts a rotation another process
  made, and retries a 400 once against freshly reloaded state. Lock acquisition is on a
  blocking pool, and the token endpoint has a hard 30-second timeout.
- **Store paths.** `$XDG_CONFIG_HOME/oura-toolkit/` (`~/.config/oura-toolkit/`) on Unix and
  macOS, `%LOCALAPPDATA%\oura-toolkit\` on Windows: Local, not Roaming, and not
  `~/Library/Application Support`. An empty or relative environment value is ignored. The path
  is the same under npx, bunx and brew.
- **Permissions and writes.** 0600 and atomic writes on Unix.
- **Two records.** `credentials.json` and `tokens.json` are separate, so a failed login never
  loses the secret the user pasted.
- **The login flow.** The redirect URI is `http://localhost:8788/callback` by default, and Oura
  requires an exact pre-registered match. `--no-browser` still checks `state`.
- **Default scopes.** `email` is left out unless it is needed.
- **Headless.** `OURA_ACCESS_TOKEN` bypasses the store, with no login and no refresh. A forced
  refresh on that path stops at `AuthError::StaticTokenRejected` before the credentials check.
- **Scope changes.** The check is data-driven: the recorded grant against the spec-derived
  `default_scopes()`. It prompts once per change of the scope set on a terminal, and a failed
  login never costs the command. `scope-notice.json` holds no secrets.
- **User-Agent.** Every request carries one. The Rust data plane, `oura-toolkit-auth` and the C#
  companion set it explicitly, each pinned by a test.

## Gotchas this repo has met

- **The same bug in every language.** Separate reviews of the companions found the same two bug
  families. A hostile token-endpoint response, a hostile store file, a rejected response or a
  refresh fallback goes in `codegen/conformance/auth-cases.json`, never in one language's
  suite. A companion that fails a case is fixed; the fixture is never weakened.
- **An error that quotes its input.** A store-file or token-response error must not echo the
  secret-bearing input (`must_not_echo` in the fixture). Check the error path, not only the
  happy one.
- **A helper in the wrong place.** Adding public API to one companion breaks the rule that all
  six have the same shape. CLI policy built on what the companions already expose lives in the
  CLI (`reconsent::missing_default_scopes` is the precedent).
