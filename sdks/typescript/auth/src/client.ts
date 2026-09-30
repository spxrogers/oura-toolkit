/**
 * The runtime auth layer: a {@link TokenManager} that owns the token state and keeps it
 * fresh (proactive refresh inside an expiry-skew window; rotation always persisted), and
 * {@link accessTokenProvider}, the seam into the generated `@oura-toolkit/api` client's
 * `Configuration.accessToken` slot.
 *
 * ## Cross-process protocol (the correctness core)
 *
 * Oura invalidates the previous refresh token on every rotation, and this store is
 * SHARED with the Rust CLI and the long-running MCP server. The cross-process guarantee
 * here is the **reload + retry protocol**, not a file lock:
 *
 * 1. Before refreshing, the store is RE-READ — if another process already rotated
 *    (disk holds a different, unexpired token set), it is ADOPTED without a refresh,
 *    so that rotation is never burned by a second endpoint call.
 * 2. A refresh that 400s is retried ONCE against freshly reloaded disk state — if disk
 *    has moved past the refresh token we sent, the retry uses the fresher one.
 *
 * The Rust companion additionally serializes rotations under an advisory `flock` on the
 * store's `.lock` file. Node has NO way to take that flock without a native or
 * third-party dependency (deliberately out of scope: this package has zero runtime
 * deps), so this companion does NOT interoperate with — or attempt to imitate — that
 * lock: an O_EXCL sentinel file would be a *different* protocol the flock-holding Rust
 * processes ignore, manufacturing a safety it cannot deliver. The reload-adopt +
 * 400-reload-retry protocol above is exactly the unlocked-writer path the Rust manager
 * already tolerates (its own 400-retry arm exists for us). Within one Node process,
 * refreshes ARE serialized by an in-process mutex, so concurrent API calls share a
 * single rotation.
 */

import {
  MissingClientCredentialsError,
  NotAuthenticatedError,
  TokenEndpointError,
  TokenEndpointTransportError,
} from "./errors";
import { TOKEN_URL } from "./metadata";
import { ClientCredentials, EnvLookup, Tokens, TokenStore } from "./store";

/**
 * Refresh this many seconds before the token's actual expiry — the same skew as the
 * Rust companion's `DEFAULT_SKEW_SECS` (client.rs).
 */
export const DEFAULT_SKEW_SECS = 60;

/**
 * Hard timeout on each token-endpoint call (same bound as the Rust companion's
 * `TOKEN_ENDPOINT_TIMEOUT`): a stalled endpoint must fail the call, never wedge the app.
 */
export const TOKEN_ENDPOINT_TIMEOUT_MS = 30_000;

export interface TokenManagerOptions {
  store: TokenStore;
  /** BYO confidential client credentials; absent ⇒ refresh throws {@link MissingClientCredentialsError}. */
  credentials?: ClientCredentials | null;
  /** In-memory token seed; absent ⇒ first use throws {@link NotAuthenticatedError}. */
  tokens?: Tokens | null;
  /** Test-only override of the spec-derived token endpoint (mock server). */
  tokenUrl?: string;
  /** Expiry skew in seconds (default {@link DEFAULT_SKEW_SECS}). */
  skewSecs?: number;
  /** Token-endpoint timeout in ms (default {@link TOKEN_ENDPOINT_TIMEOUT_MS}). */
  timeoutMs?: number;
}

/** Raw token-endpoint response (Oura returns a rotated `refresh_token` on every call). */
interface TokenResponse {
  access_token: string;
  refresh_token?: string;
  expires_in: number;
  scope?: string;
  token_type?: string;
}

/**
 * Owns the current tokens and the machinery to keep them fresh. Construct once and
 * share it across API instances; see {@link accessTokenProvider} for the wiring.
 */
export class TokenManager {
  readonly #store: TokenStore;
  readonly #credentials: ClientCredentials | null;
  #tokens: Tokens | null;
  readonly #tokenUrl: string;
  readonly #skewSecs: number;
  readonly #timeoutMs: number;
  /** In-process mutex: serializes refresh critical sections within this process. */
  #chain: Promise<void> = Promise.resolve();

  constructor(options: TokenManagerOptions) {
    this.#store = options.store;
    this.#credentials = options.credentials ?? null;
    this.#tokens = options.tokens ?? null;
    this.#tokenUrl = options.tokenUrl ?? TOKEN_URL;
    this.#skewSecs = options.skewSecs ?? DEFAULT_SKEW_SECS;
    this.#timeoutMs = options.timeoutMs ?? TOKEN_ENDPOINT_TIMEOUT_MS;
  }

  /**
   * Load from the default token store (no error if records are absent —
   * {@link accessToken} throws {@link NotAuthenticatedError} on first use instead).
   */
  static load(env?: EnvLookup): TokenManager {
    const store = env === undefined ? TokenStore.default() : TokenStore.default(env);
    return new TokenManager({
      store,
      credentials: store.loadCredentials(),
      tokens: store.loadTokens(),
    });
  }

  /**
   * Whether tokens are loaded (does not validate them, and does not imply a refresh is
   * possible — refresh additionally needs the client-credentials record).
   */
  isAuthenticated(): boolean {
    return this.#tokens !== null;
  }

  /** Return a valid access token, refreshing (and persisting the rotation) if needed. */
  accessToken(): Promise<string> {
    return this.#withMutex(async () => {
      const tokens = this.#tokens;
      if (tokens === null) throw new NotAuthenticatedError();
      if (tokens.isExpired(this.#skewSecs)) await this.#refreshCriticalSection();
      // Present after a successful refresh/adopt.
      return (this.#tokens as Tokens).accessToken();
    });
  }

  /**
   * Force a refresh regardless of expiry (call this on a data-plane 401), persisting
   * the rotation. If another process already rotated, its fresher tokens are adopted
   * instead of burning that rotation with a second refresh.
   */
  forceRefresh(): Promise<void> {
    return this.#withMutex(() => this.#refreshCriticalSection());
  }

  /** Run `fn` with the in-process refresh mutex held (FIFO; rejections don't poison it). */
  #withMutex<T>(fn: () => Promise<T>): Promise<T> {
    const result = this.#chain.then(fn);
    this.#chain = result.then(
      () => undefined,
      () => undefined
    );
    return result;
  }

  /**
   * The reload → (adopt | refresh) → persist critical section — see the module docs for
   * the cross-process protocol this implements.
   *
   * The adopt rule covers both entry points: if disk holds tokens that differ from
   * memory and aren't expired, another process already rotated — adopt them. (On the
   * proactive path memory is expired, so anything fresher is strictly better; on the
   * `forceRefresh` path memory just 401'd, so a *different* fresh token is the fix and
   * an *identical* one means we must rotate.)
   */
  async #refreshCriticalSection(): Promise<void> {
    const credentials = this.#credentials;
    if (credentials === null) throw new MissingClientCredentialsError();

    const disk = this.#store.loadTokens();
    if (disk !== null) {
      const memAccess = this.#tokens?.accessToken();
      const differs = memAccess !== disk.accessToken();
      if (differs && !disk.isExpired(this.#skewSecs)) {
        this.#tokens = disk;
        return;
      }
      // Refresh from the freshest persisted rotation, never from stale memory.
      this.#tokens = disk;
    }
    const current = this.#tokens;
    if (current === null) throw new NotAuthenticatedError();

    let refreshed: Tokens;
    try {
      refreshed = await this.#refreshAtEndpoint(credentials, current);
    } catch (e) {
      // A 400 usually means the refresh token we sent is no longer valid. If disk has
      // moved past what we sent (a rotation by another process — e.g. the flock-holding
      // Rust CLI), retry ONCE with the fresher token before surfacing "re-login".
      if (e instanceof TokenEndpointError && e.status === 400) {
        const reloaded = this.#store.loadTokens();
        if (reloaded !== null && reloaded.refreshToken() !== current.refreshToken()) {
          refreshed = await this.#refreshAtEndpoint(credentials, reloaded);
        } else {
          throw e;
        }
      } else {
        throw e;
      }
    }
    // ALWAYS persist the rotation: Oura invalidated the refresh token we just used.
    this.#store.saveTokens(refreshed);
    this.#tokens = refreshed;
  }

  /** One confidential-client refresh call (client_id + client_secret in the form body). */
  async #refreshAtEndpoint(credentials: ClientCredentials, current: Tokens): Promise<Tokens> {
    // Secrets go in the POST body, never the URL (they must stay out of server logs).
    const body = new URLSearchParams({
      grant_type: "refresh_token",
      refresh_token: current.refreshToken(),
      client_id: credentials.clientId,
      client_secret: credentials.clientSecret(),
    });
    let response: Response;
    try {
      response = await fetch(this.#tokenUrl, {
        method: "POST",
        headers: { "content-type": "application/x-www-form-urlencoded" },
        body: body.toString(),
        // Confidential client: NEVER follow a redirect. Node's default (`follow`) would
        // re-POST this body — including client_secret — to a 307/308 `Location`, leaking
        // it to whatever that URL is. `error` makes fetch reject on any 3xx, so the
        // redirect target is never contacted. Mirrors the Go client's `CheckRedirect ->
        // http.ErrUseLastResponse`.
        redirect: "error",
        signal: AbortSignal.timeout(this.#timeoutMs),
      });
    } catch (e) {
      // No HTTP response was produced: a network failure, an aborted/timed-out call, or
      // the refused redirect above. Surface it as a typed AuthError subclass (mirrors the
      // Rust companion's AuthError::Http) so callers catching AuthError don't miss it, and
      // so it never reaches the 400-reload-retry arm (that keys on TokenEndpointError.400).
      throw new TokenEndpointTransportError(describeFetchError(e, this.#timeoutMs), e);
    }
    const status = response.status;
    if (!response.ok) {
      // The non-2xx body is kept for diagnosis (e.g. `invalid_grant`), but it is
      // server-chosen text: a server that echoes back what we submitted would put the
      // refresh token / client_secret into an error callers log. So EVERY occurrence of
      // each submitted secret is redacted first, then the body is capped so a huge body
      // can't balloon the error. Conformance: auth-cases.json rejected_token_responses.
      const text = await response.text().catch(() => "");
      throw new TokenEndpointError(
        status,
        sanitizeErrorBody(text, [current.refreshToken(), credentials.clientSecret()])
      );
    }
    // A hostile or broken 2xx body must fail as the typed TokenEndpointError, never a raw
    // decode error detonating downstream and never a half-populated token persisted to the
    // store: an empty access_token would be a blank Bearer, and a zero/negative expiry
    // would only resurface as a baffling 400 on the NEXT refresh (long after Oura already
    // invalidated the refresh token we just spent — a burn). The status is the (2xx)
    // response status so the caller's 400-retry arm never misfires, and the body is a
    // FIXED, secret-free description (a partial 2xx payload may carry token material).
    // Mirrors go/auth/oauth.go and python .../auth/manager.py.
    //
    // The body must be valid UTF-8 ANYWHERE — unknown fields included — before it is even
    // considered JSON (RFC 8259 §8.1). `response.json()` / `response.text()` decode
    // leniently, silently substituting U+FFFD for an invalid byte, which would turn a
    // malformed response into a "valid" one carrying corrupted token material. So the raw
    // bytes are read and decoded with a FATAL decoder first.
    // Conformance: auth-cases.json body_invalid_utf8_* hostile cases.
    let bytes: ArrayBuffer;
    try {
      bytes = await response.arrayBuffer();
    } catch {
      throw new TokenEndpointError(status, "token-endpoint 2xx response body could not be read");
    }
    let text: string;
    try {
      text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
    } catch {
      throw new TokenEndpointError(status, "token-endpoint 2xx response was not valid UTF-8");
    }
    let json: unknown;
    try {
      json = JSON.parse(text);
    } catch {
      throw new TokenEndpointError(status, "token-endpoint 2xx response was not valid JSON");
    }
    if (typeof json !== "object" || json === null || Array.isArray(json)) {
      throw new TokenEndpointError(status, "token-endpoint 2xx response was not a JSON object");
    }
    // Validate exactly the four fields this companion reads (access_token, refresh_token,
    // token_type, scope) — unknown fields are NOT validated, and the error message only
    // ever names one of these known fields (never a server-chosen key). A string that
    // isn't valid Unicode (a lone-surrogate `\ud800`/`\udc00` escape, which JSON.parse
    // accepts) makes the RESPONSE malformed: persisting it would write a grant no other
    // language's companion can read back, so it fails typed before anything is
    // persisted. A regex rather than String.prototype.isWellFormed because the engine
    // floor is Node 18. A non-string scope stays lenient (keeps the prior grant, see
    // refresh_success_cases); a non-string refresh_token/token_type fails typed below.
    // Conformance: auth-cases.json *_lone_surrogate / wrong_type_* hostile cases.
    const fields = json as Record<string, unknown>;
    for (const field of READ_FIELDS) {
      const value = fields[field];
      if (typeof value === "string" && LONE_SURROGATE.test(value)) {
        throw new TokenEndpointError(
          status,
          `token-endpoint 2xx response field ${field} is not valid Unicode`
        );
      }
    }
    const resp = json as Partial<TokenResponse>;
    if (typeof resp.access_token !== "string" || resp.access_token === "") {
      throw new TokenEndpointError(status, "token-endpoint 2xx response missing access_token");
    }
    // expires_in must be an INTEGER in 1..=MAX_EXPIRES_IN_SECS. A fractional value, a
    // numeric string ("3600"), anything above the cap, and a literal that overflowed to
    // Infinity (`1e400`) are all malformed: there is no trustworthy expiry to persist.
    // (Number.isInteger rejects non-numbers, fractions, NaN and ±Infinity.)
    // Conformance: auth-cases.json *_expires_in / expires_in_* hostile cases.
    if (
      !Number.isInteger(resp.expires_in) ||
      (resp.expires_in as number) < 1 ||
      (resp.expires_in as number) > MAX_EXPIRES_IN_SECS
    ) {
      throw new TokenEndpointError(
        status,
        "token-endpoint 2xx response missing or invalid expires_in"
      );
    }
    // An omitted/null/EMPTY refresh_token or token_type falls back to the stored value
    // (the server didn't rotate it; persisting "" would make the next refresh 400), but
    // a present non-string is a malformed response: silently keeping the old refresh
    // token would persist one Oura has already invalidated.
    // Conformance: wrong_type_refresh_token / wrong_type_token_type (hostile) and
    // refresh_token_* / token_type_* (refresh_success_cases).
    for (const field of ["refresh_token", "token_type"] as const) {
      const value = resp[field] as unknown;
      if (value !== undefined && value !== null && typeof value !== "string") {
        throw new TokenEndpointError(
          status,
          `token-endpoint 2xx response field ${field} is not a string`
        );
      }
    }
    const expiresIn = resp.expires_in as number;
    return new Tokens({
      accessToken: resp.access_token,
      // Persist the rotated token; fall back to the old one only if the server omits it
      // (or sends null / an empty string).
      refreshToken: nonEmptyString(resp.refresh_token) ?? current.refreshToken(),
      expiresAt: Math.floor(Date.now() / 1000) + expiresIn,
      // An omitted/null/blank/non-string scope keeps the prior grant (RFC 6749 §5.1 lets
      // the server omit an unchanged scope, and scope is informational, so a non-string
      // one is not worth burning the rotated refresh token over; persisting a blank would
      // erase the grant the CLI's re-consent check reads). JS `trim()` covers ASCII
      // space/tab and U+00A0. Conformance: auth-cases.json refresh_success_cases scope_*.
      scope:
        typeof resp.scope === "string" && resp.scope.trim() !== "" ? resp.scope : current.scope,
      tokenType: nonEmptyString(resp.token_type) ?? current.tokenType,
    });
  }
}

/**
 * Upper bound (inclusive) on a token response's `expires_in`, in seconds: i32::MAX
 * (~68 years). Every companion enforces the same cap so `now + expires_in` stays an
 * exact integer in every language (well inside JS's 2^53 safe range) and the persisted
 * `expires_at` is readable by every companion's store — including Rust's `i64` field and
 * the narrower integer types other languages decode into. Anything larger is not a real
 * expiry, so the response is rejected rather than clamped.
 * Module-internal (not exported), like the other five companions' caps: nothing
 * consumes it, and exporting it would widen the public API for no caller.
 * Conformance: auth-cases.json expires_in_at_cap / expires_in_above_cap.
 */
const MAX_EXPIRES_IN_SECS = 2_147_483_647;

/** `value` if it is a non-empty string, else `undefined` (omitted/null/"" all fall back). */
function nonEmptyString(value: unknown): string | undefined {
  return typeof value === "string" && value !== "" ? value : undefined;
}

/** Cap (in UTF-16 code units, after redaction) on a non-2xx body kept in TokenEndpointError. */
const MAX_ERROR_BODY_CHARS = 1024;

/** What a submitted secret is replaced with in a non-2xx body. */
const REDACTED = "[REDACTED]";

/**
 * Prepare a NON-2xx token-endpoint body for {@link TokenEndpointError}: (a) replace EVERY
 * occurrence of each secret the request submitted — longest first, so an overlapping
 * shorter secret can't leave a fragment — with `[REDACTED]`, then (b) cap the result at {@link MAX_ERROR_BODY_CHARS}
 * characters, appending "…" when cut (never splitting a surrogate pair). Everything else
 * in the body is kept for diagnosis. Empty secrets are skipped (replacing "" would
 * interleave the marker between every character).
 */
function sanitizeErrorBody(body: string, secrets: readonly string[]): string {
  const needles = new Set<string>();
  for (const secret of secrets) {
    if (secret === "") continue;
    needles.add(secret);
  }
  let redacted = body;
  for (const needle of [...needles].sort((a, b) => b.length - a.length)) {
    redacted = redacted.split(needle).join(REDACTED);
  }
  if (redacted.length <= MAX_ERROR_BODY_CHARS) return redacted;
  let end = MAX_ERROR_BODY_CHARS;
  const last = redacted.charCodeAt(end - 1);
  if (last >= 0xd800 && last <= 0xdbff) end -= 1; // don't split a surrogate pair
  return `${redacted.slice(0, end)}…`;
}

/** The token-response fields this companion reads — the only ones it validates. */
const READ_FIELDS = ["access_token", "refresh_token", "token_type", "scope"] as const;

/** Matches an unpaired UTF-16 surrogate (a string that is not well-formed Unicode). */
const LONE_SURROGATE = /[\uD800-\uDBFF](?![\uDC00-\uDFFF])|(?<![\uD800-\uDBFF])[\uDC00-\uDFFF]/;

/**
 * A secret-free description of a `fetch` rejection for {@link TokenEndpointTransportError}.
 * `fetch` errors describe the transport (timeout, network, redirect), never the request
 * body, so this can never echo `client_secret`.
 */
function describeFetchError(e: unknown, timeoutMs: number): string {
  if (e !== null && typeof e === "object" && "name" in e) {
    const name = String((e as { name: unknown }).name);
    if (name === "TimeoutError") return `request timed out after ${timeoutMs}ms`;
    if (name === "AbortError") return "request aborted";
    const message = "message" in e ? String((e as { message: unknown }).message) : "";
    return message ? `${name}: ${message}` : name;
  }
  return "request failed";
}

/**
 * A `Configuration`-compatible async `accessToken` function: fresh token per call,
 * refreshing proactively (and persisting rotations) as needed.
 *
 * Wire it into the generated `@oura-toolkit/api` client:
 *
 * ```ts
 * import { Configuration, DailySleepRoutesApi } from "@oura-toolkit/api";
 * import { TokenManager, accessTokenProvider } from "@oura-toolkit/auth";
 *
 * const manager = TokenManager.load(); // reads ~/.config/oura-toolkit/
 * const config = new Configuration({ accessToken: accessTokenProvider(manager) });
 * const sleep = await new DailySleepRoutesApi(config).multipleDailySleepDocumentsV2UsercollectionDailySleepGet({});
 * ```
 *
 * Refresh is proactive (expiry skew), so requests normally carry a valid token. If a
 * request still 401s (e.g. the token was revoked), call `manager.forceRefresh()` and
 * retry the request once — the same reactive pattern the Rust CLI uses.
 *
 * (Typed structurally so this package needs no dependency on `@oura-toolkit/api`; the
 * signature matches its `ConfigurationParameters.accessToken` function form.)
 */
export function accessTokenProvider(
  manager: TokenManager
): (name?: string, scopes?: string[]) => Promise<string> {
  return () => manager.accessToken();
}
