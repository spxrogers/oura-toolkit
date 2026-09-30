// Cross-language auth-companion conformance (#58) — the TYPESCRIPT leg.
//
// Iterates codegen/conformance/auth-cases.json (the single source for the hostile
// token-endpoint responses, successful-refresh fallbacks, hostile store files, and
// canonical store records that every companion suite must exercise; new cases are added
// THERE, never here — and its `$comment` is the contract):
//
//  - the fixture's top-level tables are EXACTLY the seven below, so a table added to the
//    fixture can't be silently ignored by this leg;
//  - hostile_token_responses: a hostile-but-2xx token response (`body` JSON, `raw_body`
//    verbatim, or `raw_body_base64` decoded bytes — e.g. invalid UTF-8) -> typed
//    TokenEndpointError (never a bare SyntaxError/TypeError escaping), tokens.json
//    byte-identical afterwards (the rotated refresh token is never burned by persisting
//    a blank/expired Bearer) — incl. anything but whitespace after the one top-level
//    JSON value (trailing junk, a second object); a case with `must_not_echo` also
//    requires that string to appear nowhere in the error's text or any error it chains
//    (`cause`, recursively) — parser messages can quote the body;
//  - implementation_defined_token_responses: a 2xx where companions may legitimately
//    differ (leading BOM, duplicate keys, deep nesting, an integral float, an uppercase
//    key) -> EITHER success persisting access_token "at-refreshed", a refresh_token that
//    is "rt-refreshed" or the prior stored one (never empty), and expires_at = refresh
//    time + 3600, OR the typed TokenEndpointError (2xx status) with tokens.json
//    byte-identical — never an untyped throw, a wrong persisted token, or a half-written
//    store;
//  - refresh_success_cases: a successful refresh from the fixture's `prior` record
//    persists EXACTLY `expected` (access_token, refresh_token, scope, token_type) and
//    expires_at = refresh time + expected.expires_in — incl. the omitted/null/blank
//    scope and omitted/null/empty refresh_token/token_type fallbacks to `prior`;
//  - rejected_token_responses: a NON-2xx token response, with the stored refresh token
//    and the credentials' client_secret seeded from the table's `submitted` -> the typed
//    TokenEndpointError carrying the case's `status`, tokens.json byte-identical, the
//    body kept for diagnosis (`must_echo` in the error text), a submitted secret the
//    server echoed redacted (`must_not_echo` nowhere in the error or its chain), and no
//    string in the chain longer than `max_error_chars`;
//  - hostile_store_files (the file as `content` text or `content_base64` bytes — e.g.
//    invalid UTF-8, which must fail typed, never load with U+FFFD) -> the typed
//    StoreFormatError, never a default/null-filled record and never an untyped throw;
//    `must_not_echo` as above (the store holds secrets);
//  - every `must_not_echo` needle must actually occur in its case's payload bytes (else
//    the no-echo check passes vacuously), and at least 4 hostile token / 5 hostile store
//    cases carry one;
//  - implementation_defined_store_files: contents a parser may accept or reject (deep
//    nesting) -> EITHER load exactly `expected`, OR the typed StoreFormatError;
//  - valid_records -> load with exactly the fixture's field values and round-trip
//    through this companion's own persist path (the cross-language store compatibility
//    check — field names are the shared wire format, #54).
//
// Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs):
// same test structure, same fixture-shrink guards (>= 30 hostile token responses, >= 5
// implementation-defined token responses, >= 18 refresh success cases, >= 21 hostile
// store files, >= 4 rejected token responses, >= 1 implementation-defined store file).
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const { auth, tempStoreDir, startTokenEndpoint, credentials, expiredTokens } = require("./helpers.cjs");

/** Repo root: nearest ancestor holding the justfile + README (same walk as the Rust leg). */
function repoRoot() {
  let dir = __dirname;
  for (;;) {
    if (fs.existsSync(path.join(dir, "justfile")) && fs.existsSync(path.join(dir, "README.md"))) {
      return dir;
    }
    const parent = path.dirname(dir);
    assert.notEqual(parent, dir, "repo root not found above __dirname");
    dir = parent;
  }
}

const FIXTURE_PATH = path.join(repoRoot(), "codegen", "conformance", "auth-cases.json");
/**
 * The fixture, loaded so a `body` column re-encodes to the SAME JSON text the fixture
 * holds: a number whose source text isn't its canonical JS form (`3600.0` — the
 * expires_in_integral_float case) is kept as `JSON.rawJSON(source)`, so JSON.stringify
 * sends `3600.0` rather than silently collapsing it to `3600` (which would turn the
 * implementation-defined case into a plain valid one). Mirrors the Rust leg, whose
 * serde_json f64 re-serializes as `3600.0`. Needs the reviver `context.source` +
 * JSON.rawJSON (Node >= 22, the CI version); refuses to run without them rather than
 * silently sending different bytes.
 */
function loadFixture() {
  assert.equal(
    typeof JSON.rawJSON,
    "function",
    "conformance harness needs JSON.rawJSON + reviver source access (Node >= 22) to send the fixture's bodies faithfully"
  );
  let sawSource = false;
  const parsed = JSON.parse(fs.readFileSync(FIXTURE_PATH, "utf8"), function (_key, value, context) {
    if (typeof value === "number") {
      assert.ok(context && typeof context.source === "string", "JSON.parse reviver source access unavailable");
      sawSource = true;
      if (context.source !== String(value)) return JSON.rawJSON(context.source);
    }
    return value;
  });
  assert.ok(sawSource, "fixture holds no numbers? reviver source access never exercised");
  return parsed;
}
const fixture = loadFixture();

function withTempStore(t) {
  const dir = tempStoreDir();
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  return new auth.TokenStore(dir);
}

/**
 * The exact bytes a case's token endpoint sends: `raw_body_base64` decoded (bytes JSON
 * can't hold, e.g. invalid UTF-8), else `raw_body` verbatim, else the JSON-encoded
 * `body` — same rule as the Rust leg's ResponseTemplate selection. A case must carry
 * exactly one of the three, so a typo'd column can't silently send `undefined`.
 */
function casePayload(c) {
  const columns = ["body", "raw_body", "raw_body_base64"].filter((k) => k in c);
  assert.equal(columns.length, 1, `case ${c.name}: exactly one of body/raw_body/raw_body_base64, got ${columns}`);
  if (typeof c.raw_body_base64 === "string") return Buffer.from(c.raw_body_base64, "base64");
  if (typeof c.raw_body === "string") return Buffer.from(c.raw_body, "utf8");
  assert.ok("body" in c, `case ${c.name}: payload column has the wrong type`);
  return Buffer.from(JSON.stringify(c.body), "utf8");
}

/**
 * The exact bytes a store case writes to disk: `content_base64` decoded (bytes JSON can't
 * hold, e.g. invalid UTF-8), else `content` as UTF-8 text. Exactly one of the two, so a
 * typo'd column can't silently write `undefined`.
 */
function storeContent(c) {
  const columns = ["content", "content_base64"].filter((k) => k in c);
  assert.equal(columns.length, 1, `case ${c.name}: exactly one of content/content_base64, got ${columns}`);
  if (typeof c.content_base64 === "string") return Buffer.from(c.content_base64, "base64");
  assert.equal(typeof c.content, "string", `case ${c.name}: content column has the wrong type`);
  return Buffer.from(c.content, "utf8");
}

/**
 * Vacuity guard: a case's `must_not_echo` needle must actually occur in the bytes the case
 * sends/writes — otherwise the no-echo check passes no matter what the error says. A
 * no-op for a case without `must_not_echo`.
 */
function assertNeedleInPayload(c, payload) {
  if (c.must_not_echo === undefined) return;
  assert.equal(typeof c.must_not_echo, "string", `case ${c.name}: must_not_echo must be a string`);
  assert.ok(
    payload.includes(Buffer.from(c.must_not_echo, "utf8")),
    `case ${c.name}: must_not_echo ${JSON.stringify(c.must_not_echo)} does not occur in the case's payload — the no-echo check would pass vacuously`
  );
}

/** Floor: at least `min` cases of a hostile table carry `must_not_echo`. */
function assertNoEchoFloor(cases, min, table) {
  const n = cases.filter((c) => c.must_not_echo !== undefined).length;
  assert.ok(n >= min, `${table}: only ${n} cases carry must_not_echo (floor ${min}) — fixture shrank?`);
}

/**
 * `must_not_echo` (fixture contract): the secret must appear NOWHERE in the typed error's
 * text or in any error it chains. Walks the error and its `cause` chain (plus an
 * AggregateError's `errors`), checking every textual surface a caller could log:
 * String(e), message, stack, and every own string property (e.g. TokenEndpointError's
 * `body`). A no-op for a case without `must_not_echo`.
 *
 * Stricter than the fixture's full-string rule, deliberately: V8's JSON.parse message
 * quotes only a ~10-char EXCERPT of the input (`..."h_token": rtSECRETst"...`), so a
 * full-string check alone misses a truncated-but-real secret leak. Any 8-char window of
 * the secret counts as an echo too.
 */
function assertNoEcho(err, secret, name) {
  if (secret === undefined) return;
  assert.equal(typeof secret, "string", `case ${name}: must_not_echo must be a string`);
  assert.ok(secret.length > 0, `case ${name}: must_not_echo must be non-empty`);
  const WINDOW = Math.min(8, secret.length);
  const echoes = (text) => {
    for (let i = 0; i + WINDOW <= secret.length; i++) {
      if (text.includes(secret.slice(i, i + WINDOW))) return true;
    }
    return false;
  };
  const seen = new Set();
  const walk = (e, where) => {
    if (e === null || e === undefined) return;
    if (typeof e !== "object" && typeof e !== "function") {
      assert.ok(!echoes(String(e)), `case ${name}: ${where} echoes the must_not_echo secret`);
      return;
    }
    if (seen.has(e)) return;
    seen.add(e);
    const surfaces = { "String()": String(e), message: e.message, stack: e.stack };
    for (const key of Object.getOwnPropertyNames(e)) {
      if (typeof e[key] === "string") surfaces[key] = e[key];
    }
    for (const [surface, text] of Object.entries(surfaces)) {
      if (typeof text !== "string") continue;
      assert.ok(
        !echoes(text),
        `case ${name}: ${where}.${surface} echoes the must_not_echo secret: ${JSON.stringify(text)}`
      );
    }
    walk(e.cause, `${where}.cause`);
    if (Array.isArray(e.errors)) e.errors.forEach((inner, i) => walk(inner, `${where}.errors[${i}]`));
  };
  walk(err, "error");
}

test("conformance: the fixture's top-level tables are exactly the ones this leg iterates", () => {
  // A table added to (or renamed in) the fixture must fail here until this leg iterates
  // it — otherwise new cases would be silently ignored (e.g. the refresh_scope_cases ->
  // refresh_success_cases rename).
  const tables = Object.keys(fixture)
    .filter((k) => k !== "$comment")
    .sort();
  assert.deepEqual(
    tables,
    [
      "hostile_store_files",
      "hostile_token_responses",
      "implementation_defined_store_files",
      "implementation_defined_token_responses",
      "refresh_success_cases",
      "rejected_token_responses",
      "valid_records",
    ],
    "auth-cases.json top-level tables changed: iterate the new table in this suite"
  );
});

test("conformance: hostile 2xx token responses fail typed and leave the store untouched", async (t) => {
  const cases = fixture.hostile_token_responses;
  assert.ok(Array.isArray(cases), "hostile_token_responses table");
  assert.ok(cases.length >= 30, `fixture shrank? ${cases.length} cases`);
  assertNoEchoFloor(cases, 4, "hostile_token_responses");

  for (const c of cases) {
    const name = c.name;
    const payload = casePayload(c);
    assertNeedleInPayload(c, payload);

    const endpoint = await startTokenEndpoint((_params, res) => {
      res.writeHead(200, { "content-type": "application/json" });
      res.end(payload);
    });
    t.after(endpoint.close);

    const store = withTempStore(t);
    store.saveCredentials(credentials());
    store.saveTokens(expiredTokens("rt-original")); // expired, so the refresh genuinely calls the endpoint
    const bytesBefore = fs.readFileSync(store.tokensPath());

    const manager = new auth.TokenManager({
      store,
      credentials: credentials(),
      tokens: expiredTokens("rt-original"),
      tokenUrl: endpoint.url,
    });

    let thrown;
    await assert.rejects(
      () => manager.forceRefresh(),
      (e) => {
        thrown = e;
        return true;
      },
      `case ${name}: a hostile 2xx must not succeed`
    );
    // Typed: the companion's own error classes — never a raw SyntaxError/TypeError from
    // the decode detonating downstream, and never a mis-filed variant that would trigger
    // remediation hints (or the 400-reload-retry arm) for a server-side fault.
    assert.ok(
      thrown instanceof auth.AuthError,
      `case ${name}: expected a typed AuthError subclass, got ${thrown && thrown.constructor.name}: ${thrown}`
    );
    assert.ok(
      thrown instanceof auth.TokenEndpointError,
      `case ${name}: expected the TokenEndpointError variant, got ${thrown && thrown.constructor.name}`
    );
    assertNoEcho(thrown, c.must_not_echo, name);
    assert.equal(
      endpoint.requests.length,
      1,
      `case ${name}: a hostile 2xx must not trigger the 400-reload-retry arm`
    );
    // Burn-prevention: the on-disk record is byte-identical — the still-valid rotated
    // refresh token was never overwritten by a blank/expired Bearer.
    const bytesAfter = fs.readFileSync(store.tokensPath());
    assert.ok(
      bytesBefore.equals(bytesAfter),
      `case ${name}: tokens.json must be byte-identical (store UNTOUCHED, rotation not burned)`
    );
  }
});

test("conformance: implementation-defined 2xx token responses succeed cleanly or fail typed", async (t) => {
  const cases = fixture.implementation_defined_token_responses;
  assert.ok(Array.isArray(cases), "implementation_defined_token_responses table");
  assert.ok(cases.length >= 5, `fixture shrank? ${cases.length} cases`);

  for (const c of cases) {
    const name = c.name;
    const payload = casePayload(c);

    const endpoint = await startTokenEndpoint((_params, res) => {
      res.writeHead(200, { "content-type": "application/json" });
      res.end(payload);
    });
    t.after(endpoint.close);

    // Seeded exactly as the hostile harness seeds it.
    const store = withTempStore(t);
    store.saveCredentials(credentials());
    store.saveTokens(expiredTokens("rt-original"));
    const bytesBefore = fs.readFileSync(store.tokensPath());

    const manager = new auth.TokenManager({
      store,
      credentials: credentials(),
      tokens: expiredTokens("rt-original"),
      tokenUrl: endpoint.url,
    });

    let thrown = null;
    const t0 = Math.floor(Date.now() / 1000);
    try {
      await manager.forceRefresh();
    } catch (e) {
      thrown = e;
    }
    const t1 = Math.floor(Date.now() / 1000);
    assert.equal(endpoint.requests.length, 1, `case ${name}: the refresh must call the endpoint exactly once`);
    t.diagnostic(`${name}: ${thrown === null ? "succeeded" : `failed typed (${thrown && thrown.body})`}`);

    if (thrown === null) {
      // Outcome A: success — the PERSISTED record (a fresh load from disk) carries the
      // response's access token, never a stale/blank/mangled one.
      const persisted = store.loadTokens();
      assert.notEqual(persisted, null, `case ${name}: a successful refresh must persist tokens`);
      assert.equal(
        persisted.accessToken(),
        "at-refreshed",
        `case ${name}: a successful refresh must persist access_token "at-refreshed"`
      );
      // The refresh token is the response's rotated one or the prior stored one — never
      // empty (a "" would make the next refresh 400) and never anything else.
      assert.ok(
        ["rt-refreshed", "rt-original"].includes(persisted.refreshToken()),
        `case ${name}: a successful refresh must persist refresh_token "rt-refreshed" or the prior ` +
          `"rt-original", got ${JSON.stringify(persisted.refreshToken())}`
      );
      assert.ok(
        persisted.expiresAt >= t0 + 3600 && persisted.expiresAt <= t1 + 3600,
        `case ${name}: a successful refresh must persist expires_at = refresh time + 3600 ` +
          `(within [${t0 + 3600}, ${t1 + 3600}]), got ${persisted.expiresAt}`
      );
    } else {
      // Outcome B: the TYPED failure — TokenEndpointError carrying the 2xx status (never
      // an untyped SyntaxError/RangeError/TypeError, never a mis-filed variant) — with the
      // store UNTOUCHED (no half-written record).
      assert.ok(
        thrown instanceof auth.TokenEndpointError,
        `case ${name}: must succeed or fail with the typed TokenEndpointError, got ` +
          `${thrown && thrown.constructor.name}: ${thrown}`
      );
      assert.ok(
        thrown.status >= 200 && thrown.status <= 299,
        `case ${name}: the typed failure must carry the 2xx status, got ${thrown.status}`
      );
      const bytesAfter = fs.readFileSync(store.tokensPath());
      assert.ok(
        bytesBefore.equals(bytesAfter),
        `case ${name}: a typed failure must leave tokens.json byte-identical (no half-written store)`
      );
    }
  }
});

test("conformance: refresh_success_cases persist exactly `expected`", async (t) => {
  const table = fixture.refresh_success_cases;
  assert.ok(table && typeof table === "object", "refresh_success_cases table");
  const prior = table.prior;
  assert.ok(prior && typeof prior === "object", "refresh_success_cases.prior");
  for (const field of ["access_token", "refresh_token", "scope", "token_type"]) {
    assert.equal(typeof prior[field], "string", `refresh_success_cases.prior.${field}`);
    assert.notEqual(prior[field].trim(), "", `refresh_success_cases.prior.${field} must be non-blank`);
  }
  const cases = table.cases;
  assert.ok(Array.isArray(cases), "refresh_success_cases.cases");
  assert.ok(cases.length >= 18, `fixture shrank? ${cases.length} cases`);

  for (const c of cases) {
    const name = c.name;
    const expected = c.expected;
    assert.ok(expected && typeof expected === "object", `case ${name}: expected`);
    const payload = casePayload(c);
    const endpoint = await startTokenEndpoint((_params, res) => {
      res.writeHead(200, { "content-type": "application/json" });
      res.end(payload);
    });
    t.after(endpoint.close);

    // Seeded with the fixture's `prior` and an already-expired expiry, so the refresh
    // genuinely calls the endpoint.
    const startTokens = () =>
      new auth.Tokens({
        accessToken: prior.access_token,
        refreshToken: prior.refresh_token,
        expiresAt: 0,
        scope: prior.scope,
        tokenType: prior.token_type,
      });
    const store = withTempStore(t);
    store.saveCredentials(credentials());
    store.saveTokens(startTokens());

    const manager = new auth.TokenManager({
      store,
      credentials: credentials(),
      tokens: startTokens(),
      tokenUrl: endpoint.url,
    });

    const t0 = Math.floor(Date.now() / 1000);
    try {
      await manager.forceRefresh();
    } catch (e) {
      assert.fail(`case ${name}: the refresh must SUCCEED, got ${e && e.constructor.name}: ${e}`);
    }
    const t1 = Math.floor(Date.now() / 1000);
    assert.equal(endpoint.requests.length, 1, `case ${name}: the refresh must call the endpoint once`);
    assert.equal(
      endpoint.requests[0].params.get("refresh_token"),
      prior.refresh_token,
      `case ${name}: the refresh must send the prior refresh token`
    );

    // Assert against the PERSISTED record (a fresh load from disk), not in-memory state.
    const persisted = store.loadTokens();
    assert.notEqual(persisted, null, `case ${name}: tokens must be persisted`);
    assert.equal(persisted.accessToken(), expected.access_token, `case ${name}: persisted access_token`);
    assert.equal(
      persisted.refreshToken(),
      expected.refresh_token,
      `case ${name}: persisted refresh_token (omitted/null/empty keeps prior ${JSON.stringify(prior.refresh_token)})`
    );
    assert.equal(
      persisted.scope,
      expected.scope,
      `case ${name}: persisted scope (omitted/null/blank/non-string keeps prior ${JSON.stringify(prior.scope)})`
    );
    assert.equal(
      persisted.tokenType,
      expected.token_type,
      `case ${name}: persisted token_type (omitted/null/empty keeps prior ${JSON.stringify(prior.token_type)})`
    );
    assert.equal(typeof expected.expires_in, "number", `case ${name}: expected.expires_in`);
    assert.ok(
      persisted.expiresAt >= t0 + expected.expires_in && persisted.expiresAt <= t1 + expected.expires_in,
      `case ${name}: persisted expires_at ${persisted.expiresAt} must be refresh time + ` +
        `${expected.expires_in} (within [${t0 + expected.expires_in}, ${t1 + expected.expires_in}])`
    );
  }
});

/**
 * Every string surface of an error and the errors it chains (`cause`, recursively, plus an
 * AggregateError's `errors`): String(e), message, stack, and every own string property.
 * The same surfaces {@link assertNoEcho} walks.
 */
function chainStrings(err) {
  const out = [];
  const seen = new Set();
  const walk = (e, where) => {
    if (e === null || e === undefined) return;
    if (typeof e !== "object" && typeof e !== "function") {
      out.push({ where, text: String(e) });
      return;
    }
    if (seen.has(e)) return;
    seen.add(e);
    const surfaces = { "String()": String(e), message: e.message, stack: e.stack };
    for (const key of Object.getOwnPropertyNames(e)) {
      if (typeof e[key] === "string") surfaces[key] = e[key];
    }
    for (const [surface, text] of Object.entries(surfaces)) {
      if (typeof text === "string") out.push({ where: `${where}.${surface}`, text });
    }
    walk(e.cause, `${where}.cause`);
    if (Array.isArray(e.errors)) e.errors.forEach((inner, i) => walk(inner, `${where}.errors[${i}]`));
  };
  walk(err, "error");
  return out;
}

test("conformance: rejected (non-2xx) token responses fail typed, keep the body, redact secrets, cap size", async (t) => {
  const table = fixture.rejected_token_responses;
  assert.ok(table && typeof table === "object", "rejected_token_responses table");
  const submitted = table.submitted;
  assert.ok(submitted && typeof submitted === "object", "rejected_token_responses.submitted");
  for (const field of ["refresh_token", "client_secret"]) {
    assert.equal(typeof submitted[field], "string", `rejected_token_responses.submitted.${field}`);
    assert.notEqual(submitted[field], "", `rejected_token_responses.submitted.${field} must be non-empty`);
  }
  const maxChars = table.max_error_chars;
  assert.ok(Number.isInteger(maxChars) && maxChars > 0, "rejected_token_responses.max_error_chars");
  const cases = table.cases;
  assert.ok(Array.isArray(cases), "rejected_token_responses.cases");
  assert.ok(cases.length >= 4, `fixture shrank? ${cases.length} cases`);
  // Vacuity guard for the size cap: some case's body must exceed it.
  assert.ok(
    cases.some((c) => typeof c.raw_body === "string" && c.raw_body.length > maxChars),
    `rejected_token_responses: no case body exceeds max_error_chars ${maxChars} — the cap check would pass vacuously`
  );

  // One subtest per case, so a broken implementation reports EVERY failing case by name.
  for (const c of cases) {
    await t.test(`rejected_token_responses: ${c.name}`, async (st) => {
      const { name, status } = c;
      assert.equal(typeof c.raw_body, "string", `case ${name}: raw_body`);
      assert.ok(Number.isInteger(status) && (status < 200 || status > 299), `case ${name}: status must be non-2xx`);
      assert.equal(typeof c.must_echo, "string", `case ${name}: must_echo`);
      assert.ok(c.raw_body.includes(c.must_echo), `case ${name}: must_echo must occur in raw_body (vacuous otherwise)`);
      const payload = Buffer.from(c.raw_body, "utf8");
      assertNeedleInPayload(c, payload);

      // Answer EVERY request the same way: a 400 may trigger the one reload-retry.
      const endpoint = await startTokenEndpoint((_params, res) => {
        res.writeHead(status, { "content-type": "application/json" });
        res.end(payload);
      });
      st.after(endpoint.close);

      const creds = () => new auth.ClientCredentials({ clientId: "cid", clientSecret: submitted.client_secret });
      const store = withTempStore(st);
      store.saveCredentials(creds());
      store.saveTokens(expiredTokens(submitted.refresh_token));
      const bytesBefore = fs.readFileSync(store.tokensPath());

      const manager = new auth.TokenManager({
        store,
        credentials: creds(),
        tokens: expiredTokens(submitted.refresh_token),
        tokenUrl: endpoint.url,
      });

      let thrown;
      await assert.rejects(
        () => manager.forceRefresh(),
        (e) => {
          thrown = e;
          return true;
        },
        `case ${name}: a non-2xx must not succeed`
      );
      assert.ok(
        thrown instanceof auth.TokenEndpointError,
        `case ${name}: expected the typed TokenEndpointError, got ${thrown && thrown.constructor.name}: ${thrown}`
      );
      assert.equal(thrown.status, status, `case ${name}: TokenEndpointError.status`);
      // The refresh really sent the seeded secrets (else redacting them proves nothing).
      assert.ok(endpoint.requests.length >= 1, `case ${name}: the refresh must call the endpoint`);
      for (const req of endpoint.requests) {
        assert.equal(req.params.get("refresh_token"), submitted.refresh_token, `case ${name}: submitted refresh_token`);
        assert.equal(req.params.get("client_secret"), submitted.client_secret, `case ${name}: submitted client_secret`);
      }
      // The body is kept for diagnosis.
      assert.ok(
        thrown.message.includes(c.must_echo),
        `case ${name}: the error must keep the body for diagnosis (${JSON.stringify(c.must_echo)} missing): ${thrown.message.slice(0, 200)}`
      );
      // A submitted secret the server echoed back is redacted everywhere in the chain.
      assertNoEcho(thrown, c.must_not_echo, name);
      // Bounded: no string in the chain exceeds max_error_chars, however large the body.
      for (const { where, text } of chainStrings(thrown)) {
        assert.ok(
          text.length <= maxChars,
          `case ${name}: ${where} is ${text.length} chars, exceeds max_error_chars ${maxChars}`
        );
      }
      const bytesAfter = fs.readFileSync(store.tokensPath());
      assert.ok(bytesBefore.equals(bytesAfter), `case ${name}: tokens.json must be byte-identical (store UNTOUCHED)`);
    });
  }
});

test("conformance: hostile store files fail with the typed StoreFormatError", (t) => {
  const cases = fixture.hostile_store_files;
  assert.ok(Array.isArray(cases), "hostile_store_files table");
  assert.ok(cases.length >= 21, `fixture shrank? ${cases.length} cases`);
  assertNoEchoFloor(cases, 5, "hostile_store_files");

  for (const c of cases) {
    const { name, file } = c;
    const content = storeContent(c);
    assertNeedleInPayload(c, content);
    const store = withTempStore(t);
    fs.writeFileSync(path.join(store.dir, file), content);

    let load;
    if (file === "tokens.json") {
      load = () => store.loadTokens();
    } else if (file === "credentials.json") {
      load = () => store.loadCredentials();
    } else {
      assert.fail(`fixture names an unknown store file ${JSON.stringify(file)}`);
    }

    // Must throw — never return a default/null-filled record that makes
    // isAuthenticated lie — and the throw must be the TYPED store-format error, never
    // an untyped SyntaxError/TypeError escaping JSON.parse or a field access.
    assert.throws(
      load,
      (e) => {
        assert.ok(
          e instanceof auth.StoreFormatError,
          `case ${name}: expected the typed StoreFormatError, got ${e && e.constructor.name}: ${e}`
        );
        assertNoEcho(e, c.must_not_echo, name);
        return true;
      },
      `case ${name}: hostile ${file} must not load`
    );
  }
});

test("conformance: implementation-defined store files load exactly `expected` or fail typed", (t) => {
  const cases = fixture.implementation_defined_store_files;
  assert.ok(Array.isArray(cases), "implementation_defined_store_files table");
  assert.ok(cases.length >= 1, `fixture shrank? ${cases.length} cases`);

  for (const c of cases) {
    const { name, file, content, expected } = c;
    assert.equal(file, "tokens.json", `case ${name}: only tokens.json cases are harnessed (got ${file})`);
    assert.ok(expected && typeof expected === "object", `case ${name}: expected`);
    const store = withTempStore(t);
    fs.writeFileSync(path.join(store.dir, file), content);

    let loaded;
    let thrown = null;
    try {
      loaded = store.loadTokens();
    } catch (e) {
      thrown = e;
    }
    t.diagnostic(`${name}: ${thrown === null ? "loaded" : `failed (${thrown && thrown.constructor.name})`}`);
    if (thrown === null) {
      // Outcome A: accepted — exactly the fixture's fields, never a null/partial record.
      assert.notEqual(loaded, null, `case ${name}: an accepted load must return a record`);
      assert.equal(loaded.accessToken(), expected.access_token, `case ${name}: loaded access_token`);
      assert.equal(loaded.refreshToken(), expected.refresh_token, `case ${name}: loaded refresh_token`);
      assert.equal(loaded.expiresAt, expected.expires_at, `case ${name}: loaded expires_at`);
    } else {
      // Outcome B: the TYPED store-format error — never a RangeError/SyntaxError escaping.
      assert.ok(
        thrown instanceof auth.StoreFormatError,
        `case ${name}: must load exactly \`expected\` or fail with the typed StoreFormatError, got ` +
          `${thrown && thrown.constructor.name}: ${thrown}`
      );
    }
  }
});

test("conformance: canonical valid records load exactly and round-trip via the persist path", (t) => {
  const valid = fixture.valid_records;
  const store = withTempStore(t);
  // JSON.stringify of the fixture objects — the canonical on-disk wire format shared by
  // every language (source of truth: oura-toolkit-auth's store.rs; #54).
  fs.writeFileSync(store.credentialsPath(), JSON.stringify(valid["credentials.json"], null, 2));
  fs.writeFileSync(store.tokensPath(), JSON.stringify(valid["tokens.json"], null, 2));

  const creds = store.loadCredentials();
  assert.notEqual(creds, null, "credentials must load");
  assert.equal(creds.clientId, "cid-conformance");
  assert.equal(creds.clientSecret(), "cs-conformance");

  const tokens = store.loadTokens();
  assert.notEqual(tokens, null, "tokens must load");
  assert.equal(tokens.accessToken(), "at-conformance");
  assert.equal(tokens.refreshToken(), "rt-conformance");
  assert.equal(tokens.expiresAt, 4102444800);
  assert.equal(tokens.scope, "personal daily");
  assert.equal(tokens.tokenType, "Bearer");

  // Round-trip: this companion's persist path must re-emit records the loader (and, by
  // the shared fixture, every other language) still reads identically.
  store.saveCredentials(creds);
  store.saveTokens(tokens);

  const creds2 = store.loadCredentials();
  assert.equal(creds2.clientId, "cid-conformance");
  assert.equal(creds2.clientSecret(), "cs-conformance");

  const tokens2 = store.loadTokens();
  assert.equal(tokens2.accessToken(), "at-conformance");
  assert.equal(tokens2.refreshToken(), "rt-conformance");
  assert.equal(tokens2.expiresAt, 4102444800);
  assert.equal(tokens2.scope, "personal daily");
  assert.equal(tokens2.tokenType, "Bearer");
});
