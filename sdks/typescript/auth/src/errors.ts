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
 * or a malformed/hostile 2xx success body. For a broken 2xx the {@link status} is the
 * (2xx) response status and {@link body} is a FIXED, secret-free description — the raw
 * response is never echoed, since a partial 2xx payload may carry token material.
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

/**
 * A transport-level failure talking to the token endpoint: the request never produced an
 * HTTP response — a network error, an aborted/timed-out call, or a REFUSED redirect (a
 * confidential client must never re-POST its `client_secret` to a 3xx `Location`). The
 * TypeScript mirror of the Rust companion's `AuthError::Http` variant, so callers catching
 * {@link AuthError} never miss a stalled or hijacked token-endpoint call. The message
 * carries no secret material (fetch's own errors describe the transport, not the body).
 */
export class TokenEndpointTransportError extends AuthError {
  /** The underlying `fetch` rejection (e.g. a `TimeoutError`/`AbortError` DOMException). */
  readonly cause?: unknown;

  constructor(detail: string, cause?: unknown) {
    super(`token endpoint transport error: ${detail}`);
    this.name = "TokenEndpointTransportError";
    this.cause = cause;
  }
}
