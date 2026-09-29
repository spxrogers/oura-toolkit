// Cross-language auth-companion conformance (#58) — the TYPESCRIPT leg.
//
// Iterates codegen/conformance/auth-cases.json (the single source for the hostile
// token-endpoint responses, successful-refresh fallbacks, hostile store files, and
// canonical store records that every companion suite must exercise; new cases are added
// THERE, never here — and its `$comment` is the contract):
//
//  - the fixture's top-level tables are EXACTLY the four below, so a table added to the
//    fixture can't be silently ignored by this leg;
//  - hostile_token_responses: a hostile-but-2xx token response (`body` JSON, `raw_body`
//    verbatim, or `raw_body_base64` decoded bytes — e.g. invalid UTF-8) -> typed
//    TokenEndpointError (never a bare SyntaxError/TypeError escaping), tokens.json
//    byte-identical afterwards (the rotated refresh token is never burned by persisting
//    a blank/expired Bearer);
//  - refresh_success_cases: a successful refresh from the fixture's `prior` record
//    persists EXACTLY `expected` (access_token, refresh_token, scope, token_type) and
//    expires_at = refresh time + expected.expires_in — incl. the omitted/null/blank
//    scope and omitted/null/empty refresh_token/token_type fallbacks to `prior`;
//  - hostile_store_files -> the typed StoreFormatError, never a default/null-filled
//    record and never an untyped throw;
//  - valid_records -> load with exactly the fixture's field values and round-trip
//    through this companion's own persist path (the cross-language store compatibility
//    check — field names are the shared wire format, #54).
//
// Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs):
// same test structure, same fixture-shrink guards (>= 24 hostile token responses,
// >= 17 refresh success cases, >= 8 hostile store files).
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
const fixture = JSON.parse(fs.readFileSync(FIXTURE_PATH, "utf8"));

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

test("conformance: the fixture's top-level tables are exactly the ones this leg iterates", () => {
  // A table added to (or renamed in) the fixture must fail here until this leg iterates
  // it — otherwise new cases would be silently ignored (e.g. the refresh_scope_cases ->
  // refresh_success_cases rename).
  const tables = Object.keys(fixture)
    .filter((k) => k !== "$comment")
    .sort();
  assert.deepEqual(
    tables,
    ["hostile_store_files", "hostile_token_responses", "refresh_success_cases", "valid_records"],
    "auth-cases.json top-level tables changed: iterate the new table in this suite"
  );
});

test("conformance: hostile 2xx token responses fail typed and leave the store untouched", async (t) => {
  const cases = fixture.hostile_token_responses;
  assert.ok(Array.isArray(cases), "hostile_token_responses table");
  assert.ok(cases.length >= 24, `fixture shrank? ${cases.length} cases`);

  for (const c of cases) {
    const name = c.name;
    const payload = casePayload(c);

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
  assert.ok(cases.length >= 17, `fixture shrank? ${cases.length} cases`);

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

test("conformance: hostile store files fail with the typed StoreFormatError", (t) => {
  const cases = fixture.hostile_store_files;
  assert.ok(Array.isArray(cases), "hostile_store_files table");
  assert.ok(cases.length >= 8, `fixture shrank? ${cases.length} cases`);

  for (const c of cases) {
    const { name, file, content } = c;
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
        return true;
      },
      `case ${name}: hostile ${file} must not load`
    );
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
