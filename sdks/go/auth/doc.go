// Package auth is the hand-written Go auth companion for the oura-toolkit Go SDK.
//
// It mirrors the Rust companion (sdks/rust/oura-toolkit-auth) behavior for behavior:
//
//   - Token store: two records at the fixed, invocation-independent per-platform config
//     dir — credentials.json (the user's own OAuth app id + secret; BYO confidential
//     client) and tokens.json (access/refresh/expiry/scope). $XDG_CONFIG_HOME/oura-toolkit
//     (fallback $HOME/.config/oura-toolkit) on Unix/macOS, %LOCALAPPDATA%\oura-toolkit on
//     Windows. 0600 files + 0700 dir + atomic temp-and-rename writes on Unix; on Windows
//     protection relies on %LOCALAPPDATA%'s user-private ACLs.
//   - Refresh with rotation: Oura invalidates the previous refresh token on every refresh —
//     the newly returned one is always persisted.
//   - Cross-process safety: every refresh runs under an exclusive advisory lock on the
//     store's .lock file (flock on Unix, LockFileEx on Windows — the same primitives as the
//     Rust CLI's std File::lock, so the two coordinate) and re-reads the store first,
//     adopting a rotation another process already performed instead of re-burning it; a
//     refresh HTTP 400 is retried once against freshly reloaded disk state.
//   - Client seam: Manager implements golang.org/x/oauth2's TokenSource, which the
//     generated client accepts via api.ContextOAuth2.
//
// The package is deliberately non-interactive: no browser, no loopback listener, no
// authorization-code exchange. Interactive OAuth lives in the CLI (`oura auth setup` /
// `oura auth login`), which writes the same store this package reads.
package auth
