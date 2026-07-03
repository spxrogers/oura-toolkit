/**
 * Persistent credential store at the fixed, invocation-independent per-platform path:
 * `$XDG_CONFIG_HOME/oura-toolkit/` (→ `~/.config/oura-toolkit/`) on Unix/macOS,
 * `%LOCALAPPDATA%\oura-toolkit\` on Windows — the SAME directory and the SAME two JSON
 * records as the Rust companion (sdks/rust/oura-toolkit-auth/src/store.rs), so the CLI,
 * the MCP server, and this SDK companion all share one store.
 *
 * Two records live in the store dir:
 *
 * - **`credentials.json`** — {@link ClientCredentials} (the user's own OAuth app id +
 *   secret; BYO confidential client). Exists from `oura auth setup` onward.
 * - **`tokens.json`** — {@link Tokens} (access/refresh/expiry/scope). Exists only after
 *   a successful login and is rewritten on every refresh rotation.
 *
 * The on-disk field names are the Rust structs' serde output (`client_id`,
 * `client_secret`, `access_token`, `refresh_token`, `expires_at`, optional `scope` /
 * `token_type` OMITTED when absent) — pinned by a fixture test in `tests/store.test.cjs`.
 *
 * File hygiene: records are written atomically (uniquely named temp file in the same
 * directory + rename) and, on Unix, chmod'd `0600` with a `0700` parent dir. On Windows
 * those modes are no-ops — protection relies on `%LOCALAPPDATA%`'s user-private ACLs.
 *
 * Secret hygiene: {@link ClientCredentials} and {@link Tokens} REDACT their secret
 * fields from `JSON.stringify`, `util.inspect` (and therefore `console.log`), and
 * `toString`. The store's own serializer is the only path that writes the real values —
 * a stray log statement can never leak a secret, and `JSON.stringify(tokens)` is
 * deliberately NOT a way to persist tokens.
 */

import * as crypto from "node:crypto";
import * as fs from "node:fs";
import * as path from "node:path";

import { NoConfigDirError, StoreFormatError } from "./errors";

const INSPECT = Symbol.for("nodejs.util.inspect.custom");
const REDACTED = "[REDACTED]";

/**
 * The user's own Oura OAuth application credentials (BYO confidential client).
 *
 * The secret lives in a private field: it is invisible to `JSON.stringify`, spreads,
 * `Object.entries`, and `util.inspect` — read it via {@link clientSecret}. That accessor
 * is a METHOD, not a getter, deliberately: `util.inspect(x, { getters: true })` evaluates
 * accessor properties (and would print the secret), but never invokes plain methods.
 */
export class ClientCredentials {
  readonly clientId: string;
  readonly #clientSecret: string;

  constructor(args: { clientId: string; clientSecret: string }) {
    this.clientId = args.clientId;
    this.#clientSecret = args.clientSecret;
  }

  /** The client secret. Never appears in logs/serialization — see class docs. */
  clientSecret(): string {
    return this.#clientSecret;
  }

  /** Redacted: `JSON.stringify` must never emit the secret. Persist via {@link TokenStore}. */
  toJSON(): unknown {
    return { client_id: this.clientId, client_secret: REDACTED };
  }

  toString(): string {
    return `ClientCredentials { clientId: ${JSON.stringify(this.clientId)}, clientSecret: ${REDACTED} }`;
  }

  /** Redacted `util.inspect` / `console.log` rendering. */
  [INSPECT](): string {
    return this.toString();
  }
}

/**
 * The persisted OAuth token set. Client credentials live in their own record
 * ({@link ClientCredentials}), not here.
 *
 * The access and refresh tokens live in private fields and are redacted from
 * `JSON.stringify` / `util.inspect` / `toString` (see module docs). Their accessors are
 * METHODS, not getters — `util.inspect(x, { getters: true })` evaluates getters but
 * never invokes plain methods.
 */
export class Tokens {
  readonly #accessToken: string;
  readonly #refreshToken: string;
  /** Absolute expiry as a Unix timestamp (seconds). */
  readonly expiresAt: number;
  readonly scope?: string;
  readonly tokenType?: string;

  constructor(args: {
    accessToken: string;
    refreshToken: string;
    expiresAt: number;
    scope?: string;
    tokenType?: string;
  }) {
    this.#accessToken = args.accessToken;
    this.#refreshToken = args.refreshToken;
    this.expiresAt = args.expiresAt;
    this.scope = args.scope;
    this.tokenType = args.tokenType;
  }

  accessToken(): string {
    return this.#accessToken;
  }

  /**
   * Oura ROTATES this on every refresh and invalidates the previous value — whoever
   * refreshes must persist the newly returned token or the next refresh 400s.
   */
  refreshToken(): string {
    return this.#refreshToken;
  }

  /** True if the access token is expired (or within `skewSecs` of expiring). */
  isExpired(skewSecs: number): boolean {
    return Math.floor(Date.now() / 1000) + skewSecs >= this.expiresAt;
  }

  /** Redacted: `JSON.stringify` must never emit tokens. Persist via {@link TokenStore}. */
  toJSON(): unknown {
    return {
      access_token: REDACTED,
      refresh_token: REDACTED,
      expires_at: this.expiresAt,
      scope: this.scope,
      token_type: this.tokenType,
    };
  }

  toString(): string {
    return (
      `Tokens { accessToken: ${REDACTED}, refreshToken: ${REDACTED}, ` +
      `expiresAt: ${this.expiresAt}, scope: ${JSON.stringify(this.scope)}, ` +
      `tokenType: ${JSON.stringify(this.tokenType)} }`
    );
  }

  /** Redacted `util.inspect` / `console.log` rendering. */
  [INSPECT](): string {
    return this.toString();
  }
}

/** The locked config-directory name (CLAUDE.md → NAMING), identical on every platform. */
export const APP_DIR_NAME = "oura-toolkit";

/** Injectable environment lookup (tests never mutate `process.env`). */
export type EnvLookup = (key: string) => string | undefined;

/**
 * Resolve the fixed, invocation-independent config dir from an injected env lookup:
 *
 * - **Unix (incl. macOS):** `$XDG_CONFIG_HOME/oura-toolkit`, falling back to
 *   `$HOME/.config/oura-toolkit` (locked decision — deliberately NOT
 *   `~/Library/Application Support`).
 * - **Windows:** `%LOCALAPPDATA%\oura-toolkit` — Local, NOT Roaming: roaming profiles
 *   sync `%APPDATA%` off the machine, which would copy plaintext secrets to file
 *   servers/backups.
 *
 * EMPTY and RELATIVE env values are treated as absent (XDG spec): a relative base would
 * make where secrets land depend on the process cwd, breaking invocation independence
 * (the path MUST be identical under `npx`, `bunx`, and a brew binary).
 */
export function configDirFrom(
  env: EnvLookup,
  platform: NodeJS.Platform = process.platform
): string {
  const p = platform === "win32" ? path.win32 : path.posix;
  const usable = (key: string): string | undefined => {
    const value = env(key);
    return value && p.isAbsolute(value) ? value : undefined;
  };

  if (platform === "win32") {
    const base = usable("LOCALAPPDATA");
    if (base !== undefined) return p.join(base, APP_DIR_NAME);
    throw new NoConfigDirError("%LOCALAPPDATA% unset or not an absolute path");
  }

  const xdg = usable("XDG_CONFIG_HOME");
  if (xdg !== undefined) return p.join(xdg, APP_DIR_NAME);
  const home = usable("HOME");
  if (home !== undefined) return p.join(home, ".config", APP_DIR_NAME);
  throw new NoConfigDirError("$XDG_CONFIG_HOME / $HOME unset or not an absolute path");
}

/** Handle to the on-disk store directory. */
export class TokenStore {
  readonly dir: string;

  /** Store rooted at an explicit directory (used by tests). */
  constructor(dir: string) {
    this.dir = dir;
  }

  /** Store at the default per-platform config location (see {@link configDirFrom}). */
  static default(env: EnvLookup = (key) => process.env[key]): TokenStore {
    return new TokenStore(configDirFrom(env));
  }

  /** Path of the client-credentials record. */
  credentialsPath(): string {
    return path.join(this.dir, "credentials.json");
  }

  /** Path of the token record. */
  tokensPath(): string {
    return path.join(this.dir, "tokens.json");
  }

  /** Load the client credentials, or `null` if `auth setup` has never run. */
  loadCredentials(): ClientCredentials | null {
    const record = readRecord(this.credentialsPath());
    if (record === null) return null;
    return new ClientCredentials({
      clientId: requireString(record, "client_id"),
      clientSecret: requireString(record, "client_secret"),
    });
  }

  /** Persist the client credentials (`0600`, atomic). */
  saveCredentials(credentials: ClientCredentials): void {
    // Explicit plain object — deliberately NOT JSON.stringify(credentials), whose
    // toJSON() redacts. Field names are the Rust structs' serde output (fixture-pinned).
    const record = {
      client_id: credentials.clientId,
      client_secret: credentials.clientSecret(),
    };
    this.#writeRecord(this.credentialsPath(), record);
  }

  /** Load the tokens, or `null` if no login has succeeded yet. */
  loadTokens(): Tokens | null {
    const record = readRecord(this.tokensPath());
    if (record === null) return null;
    return new Tokens({
      accessToken: requireString(record, "access_token"),
      refreshToken: requireString(record, "refresh_token"),
      expiresAt: requireNumber(record, "expires_at"),
      scope: optionalString(record, "scope"),
      tokenType: optionalString(record, "token_type"),
    });
  }

  /**
   * Persist the tokens (`0600`, atomic). Callers refreshing MUST persist the rotated
   * refresh token (Oura invalidates the previous one) — {@link TokenManager} does.
   */
  saveTokens(tokens: Tokens): void {
    // Optionals are OMITTED when absent, matching serde's skip_serializing_if.
    const record: Record<string, string | number> = {
      access_token: tokens.accessToken(),
      refresh_token: tokens.refreshToken(),
      expires_at: tokens.expiresAt,
    };
    if (tokens.scope !== undefined) record.scope = tokens.scope;
    if (tokens.tokenType !== undefined) record.token_type = tokens.tokenType;
    this.#writeRecord(this.tokensPath(), record);
  }

  #writeRecord(filePath: string, record: object): void {
    this.#ensureDir();
    writeSecure(filePath, JSON.stringify(record, null, 2));
  }

  #ensureDir(): void {
    fs.mkdirSync(this.dir, { recursive: true, mode: 0o700 });
    if (process.platform !== "win32") {
      // mkdirSync's mode only applies on creation; enforce 0700 on the existing dir too.
      fs.chmodSync(this.dir, 0o700);
    }
  }
}

function readRecord(filePath: string): Record<string, unknown> | null {
  let raw: string;
  try {
    raw = fs.readFileSync(filePath, "utf8");
  } catch (e) {
    const code = (e as NodeJS.ErrnoException).code;
    if (code === "ENOENT") return null;
    // A directory (or other non-file) where a record should be is a malformed store, not
    // an opaque OS error — surface it as the typed StoreFormatError callers already handle.
    if (code === "EISDIR") {
      throw new StoreFormatError(`${filePath} is a directory, not a token-store file`);
    }
    throw e;
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(raw);
  } catch (e) {
    throw new StoreFormatError(`${filePath} is not valid JSON: ${(e as Error).message}`);
  }
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
    throw new StoreFormatError(`${filePath} must contain a JSON object`);
  }
  return parsed as Record<string, unknown>;
}

function requireString(record: Record<string, unknown>, field: string): string {
  const value = record[field];
  if (typeof value !== "string") {
    throw new StoreFormatError(`missing or non-string field "${field}"`);
  }
  return value;
}

function requireNumber(record: Record<string, unknown>, field: string): number {
  const value = record[field];
  if (typeof value !== "number") {
    throw new StoreFormatError(`missing or non-number field "${field}"`);
  }
  return value;
}

function optionalString(record: Record<string, unknown>, field: string): string | undefined {
  const value = record[field];
  if (value === undefined || value === null) return undefined;
  if (typeof value !== "string") {
    throw new StoreFormatError(`non-string field "${field}"`);
  }
  return value;
}

/**
 * Atomic write with owner-only perms: write a UNIQUELY named temp file in the same
 * directory, fsync, rename into place. The unique name means two concurrent writers can
 * never truncate each other's in-flight temp file; the atomic rename makes the outcome
 * last-writer-wins, never a corrupt mix. On Windows the chmod is a no-op (ACLs protect).
 */
function writeSecure(filePath: string, data: string): void {
  const dir = path.dirname(filePath);
  const tmp = path.join(dir, `.tmp-${crypto.randomBytes(8).toString("hex")}`);
  const fd = fs.openSync(tmp, "wx", 0o600);
  try {
    fs.writeSync(fd, data);
    fs.fsyncSync(fd);
  } finally {
    fs.closeSync(fd);
  }
  try {
    fs.renameSync(tmp, filePath);
  } catch (e) {
    fs.rmSync(tmp, { force: true });
    throw e;
  }
  if (process.platform !== "win32") {
    // openSync's 0600 is umask-masked at creation; pin the final mode exactly.
    fs.chmodSync(filePath, 0o600);
  }
}
