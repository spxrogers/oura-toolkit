/**
 * OAuth2 metadata for the Oura API. Nothing here is invented: every value below is
 * transcribed from the vendored spec's `components.securitySchemes.OAuth2` block
 * (`spec/openapi.json` at the repo root) and PINNED by a hermetic sync test
 * (`tests/metadata.test.cjs`) that re-reads the spec and fails CI on any drift — the
 * TypeScript analogue of the Rust companion's build-time spec read
 * (sdks/rust/oura-toolkit-auth/src/metadata.rs).
 *
 * Do not edit these constants by hand except to re-sync them with a refreshed spec;
 * the sync test will name the exact mismatch.
 */

/** The spec's `authorizationCode.authorizationUrl` (browser consent page). */
export const AUTHORIZE_URL = "https://cloud.ouraring.com/oauth/authorize";

/** The spec's `authorizationCode.tokenUrl` (code exchange + refresh endpoint). */
export const TOKEN_URL = "https://api.ouraring.com/oauth/token";

/** Every scope the spec advertises (`authorizationCode.scopes`), in spec order. */
export const ALL_SCOPES: readonly string[] = [
  "email",
  "personal",
  "daily",
  "heartrate",
  "workout",
  "tag",
  "session",
  "spo2Daily",
];

/**
 * Scopes the toolkit requests by default: everything except `email` — the same policy
 * as the Rust companion's `default_scopes()`. This is toolkit *policy*, not spec
 * metadata; the sync test asserts it equals {@link ALL_SCOPES} minus `email`, so a
 * spec refresh that renames a scope fails CI instead of silently narrowing consent.
 */
export const DEFAULT_SCOPES: readonly string[] = [
  "personal",
  "daily",
  "heartrate",
  "workout",
  "tag",
  "session",
  "spo2Daily",
];
