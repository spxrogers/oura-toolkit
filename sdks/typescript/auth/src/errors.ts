/**
 * Error types for the auth companion — the TypeScript mirror of the Rust companion's
 * `AuthError` variants (sdks/rust/oura-toolkit-auth/src/error.rs).
 *
 * The library deliberately does not embed remediation hints in messages — callers own
 * the UX (e.g. an app maps {@link NotAuthenticatedError} to "run `oura auth login`").
 */

/** Base class for every error thrown by `@oura-toolkit/auth`. */
export class AuthError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "AuthError";
  }
}

/** No tokens available (no login has been persisted to the store). */
export class NotAuthenticatedError extends AuthError {
  constructor() {
    super("not authenticated (no tokens stored)");
    this.name = "NotAuthenticatedError";
  }
}

/**
 * Tokens exist but the client-credentials record is missing, so a refresh is impossible
 * (Oura is a confidential client: the token endpoint requires `client_id` + `client_secret`).
 */
export class MissingClientCredentialsError extends AuthError {
  constructor() {
    super("no client credentials stored");
    this.name = "MissingClientCredentialsError";
  }
}

/** Could not resolve the config directory from the platform's environment. */
export class NoConfigDirError extends AuthError {
  constructor(detail: string) {
    super(`could not determine the config directory (${detail})`);
    this.name = "NoConfigDirError";
  }
}

/** A store record exists but is not valid JSON of the expected shape. */
export class StoreFormatError extends AuthError {
  constructor(detail: string) {
    super(`token store format error: ${detail}`);
    this.name = "StoreFormatError";
  }
}

/**
 * The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh token)
 * or a malformed success body.
 */
export class TokenEndpointError extends AuthError {
  readonly status: number;
  readonly body: string;

  constructor(status: number, body: string) {
    super(`token endpoint returned HTTP ${status}: ${body}`);
    this.name = "TokenEndpointError";
    this.status = status;
    this.body = body;
  }
}
