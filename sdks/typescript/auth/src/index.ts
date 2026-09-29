/**
 * `@oura-toolkit/auth` — the hand-written auth companion for `@oura-toolkit/api`.
 *
 * Non-interactive by design: token store, refresh with rotation, and the
 * `Configuration.accessToken` seam. The interactive OAuth flow (browser + loopback
 * listener, `oura auth setup` / `oura auth login`) lives in the CLI, never in an SDK
 * companion — run `oura auth login` once and every consumer of the shared store
 * (this package included) picks the tokens up.
 */

export {
  AuthError,
  MissingClientCredentialsError,
  NoConfigDirError,
  NotAuthenticatedError,
  StoreFormatError,
  TokenEndpointError,
  TokenEndpointTransportError,
} from "./errors";
export { ALL_SCOPES, AUTHORIZE_URL, DEFAULT_SCOPES, TOKEN_URL } from "./metadata";
export {
  APP_DIR_NAME,
  ClientCredentials,
  configDirFrom,
  Tokens,
  TokenStore,
} from "./store";
export type { EnvLookup } from "./store";
export {
  accessTokenProvider,
  DEFAULT_SKEW_SECS,
  TOKEN_ENDPOINT_TIMEOUT_MS,
  TokenManager,
} from "./client";
export type { TokenManagerOptions } from "./client";
