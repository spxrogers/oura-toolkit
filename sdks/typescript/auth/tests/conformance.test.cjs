// Cross-language auth-companion conformance (#58) — the TYPESCRIPT leg.
//
// Iterates codegen/conformance/auth-cases.json (the single source for the hostile
// token-endpoint responses, hostile store files, and canonical store records that every
// companion suite must exercise; new cases are added THERE, never here):
//
//  - hostile-but-2xx token responses -> typed AuthError subclass (never a bare
//    SyntaxError/TypeError escaping), tokens.json byte-identical afterwards (the rotated
//    refresh token is never burned by persisting a blank/expired Bearer);
//  - hostile store files -> the typed StoreFormatError, never a default/null-filled
//    record and never an untyped throw;
//  - canonical valid records -> load with exactly the fixture's field values and
//    round-trip through this companion's own persist path (the cross-language store
//    compatibility check — field names are the shared wire format, #54).
//
// Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs):
// same three-test structure, same fixture-shrink guards (>= 8 cases per hostile table).
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

test("conformance: hostile 2xx token responses fail typed and leave the store untouched", async (t) => {
  const cases = fixture.hostile_token_responses;
  assert.ok(Array.isArray(cases), "hostile_token_responses table");
  assert.ok(cases.length >= 8, `fixture shrank? ${cases.length} cases`);

  for (const c of cases) {
    const name = c.name;
    // raw_body verbatim when present, else the JSON-encoded body — same rule as the
    // Rust leg's ResponseTemplate selection.
    const payload = typeof c.raw_body === "string" ? c.raw_body : JSON.stringify(c.body);

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
