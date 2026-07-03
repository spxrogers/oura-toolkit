// TokenManager guarantees: rotation persistence, the cross-process reload+retry
// protocol (adopt; 400-reload-retry-once), proactive expiry-skew refresh, the endpoint
// timeout bound, and in-process refresh serialization. All hermetic: the token endpoint
// is a scripted 127.0.0.1 node:http server, stores live in tempdirs.
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const { test } = require("node:test");

const {
  auth,
  tempStoreDir,
  startTokenEndpoint,
  tokenJson,
  credentials,
  expiredTokens,
} = require("./helpers.cjs");

/** A manager whose token-endpoint calls hit the mock server. */
function manager(endpoint, store, tokens, extra = {}) {
  return new auth.TokenManager({
    store,
    credentials: credentials(),
    tokens,
    tokenUrl: endpoint.url,
    ...extra,
  });
}

function withTempStore(t) {
  const dir = tempStoreDir();
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  return new auth.TokenStore(dir);
}

test("refresh sends a confidential-client POST and PERSISTS the rotated refresh_token", async (t) => {
  const endpoint = await startTokenEndpoint((params, res) => {
    tokenJson(res, {
      access_token: "fresh-access",
      refresh_token: "r2",
      expires_in: 3600,
      token_type: "Bearer",
    });
  });
  t.after(endpoint.close);
  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r1"));

  const m = manager(endpoint, store, expiredTokens("r1"));
  assert.equal(await m.accessToken(), "fresh-access");

  // Confidential client: id AND secret in the form body — and never in the URL.
  assert.equal(endpoint.requests.length, 1);
  const { params, url } = endpoint.requests[0];
  assert.equal(params.get("grant_type"), "refresh_token");
  assert.equal(params.get("refresh_token"), "r1");
  assert.equal(params.get("client_id"), "cid");
  assert.equal(params.get("client_secret"), "SECRET-CS-789");
  assert.ok(!url.includes("SECRET"), "secrets must not appear in the URL");

  // THE rotation contract: Oura invalidated r1 — r2 must be on disk or the next
  // refresh (from any process) 400s.
  const persisted = store.loadTokens();
  assert.equal(persisted.refreshToken(), "r2");
  assert.equal(persisted.accessToken(), "fresh-access");
  assert.ok(!persisted.isExpired(0), "persisted expiry must reflect expires_in");
});

test("reload-adopt: a rotation another process persisted is adopted, never re-burned", async (t) => {
  // ZERO endpoint calls allowed: replaying the invalidated r1 (or burning r2) is the
  // pre-#22 failure mode this protocol exists to prevent.
  const endpoint = await startTokenEndpoint((params, res) => {
    res.writeHead(500);
    res.end("the adopt path must not call the token endpoint");
  });
  t.after(endpoint.close);
  const store = withTempStore(t);

  // Another process (e.g. the Rust CLI) already rotated: disk holds fresh r2 tokens.
  store.saveTokens(
    new auth.Tokens({
      accessToken: "rotated-access",
      refreshToken: "r2",
      expiresAt: Math.floor(Date.now() / 1000) + 3600,
    })
  );

  // Our manager still has the stale, expired r1 in memory.
  const m = manager(endpoint, store, expiredTokens("r1"));
  assert.equal(await m.accessToken(), "rotated-access");
  assert.equal(endpoint.requests.length, 0, "adopt must not spend an endpoint call");

  // forceRefresh (the post-401 path) must also adopt a DIFFERENT fresh token.
  const m2 = manager(endpoint, store, expiredTokens("r1"));
  await m2.forceRefresh();
  assert.equal(await m2.accessToken(), "rotated-access");
  assert.equal(endpoint.requests.length, 0);
});

test("refresh 400 reloads from disk and retries ONCE with the fresher refresh_token", async (t) => {
  const store = withTempStore(t);
  const endpoint = await startTokenEndpoint((params, res) => {
    if (params.get("refresh_token") === "r1") {
      // Simulate an uncoordinated writer rotating to r2 while our r1 request is in
      // flight (expired on disk, so the retry must actually refresh), then reject r1.
      store.saveTokens(
        new auth.Tokens({ accessToken: "r2-access", refreshToken: "r2", expiresAt: 0 })
      );
      res.writeHead(400);
      res.end("invalid_grant");
    } else {
      assert.equal(params.get("refresh_token"), "r2", "retry must use the reloaded token");
      tokenJson(res, { access_token: "r3-access", refresh_token: "r3", expires_in: 3600 });
    }
  });
  t.after(endpoint.close);
  store.saveTokens(expiredTokens("r1"));

  const m = manager(endpoint, store, expiredTokens("r1"));
  assert.equal(await m.accessToken(), "r3-access");
  assert.equal(endpoint.requests.length, 2, "exactly one retry");
  assert.equal(store.loadTokens().refreshToken(), "r3", "the retry's rotation is persisted");
});

test("a genuinely invalid refresh token surfaces the 400 with NO blind retry", async (t) => {
  const endpoint = await startTokenEndpoint((params, res) => {
    res.writeHead(400);
    res.end("invalid_grant");
  });
  t.after(endpoint.close);
  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r-dead"));

  const m = manager(endpoint, store, expiredTokens("r-dead"));
  await assert.rejects(
    () => m.accessToken(),
    (e) => e instanceof auth.TokenEndpointError && e.status === 400
  );
  assert.equal(
    endpoint.requests.length,
    1,
    "the reload-retry fires only when disk moved past what we sent"
  );
});

test("proactive refresh uses the 60s expiry skew (same constant as the Rust companion)", async (t) => {
  assert.equal(auth.DEFAULT_SKEW_SECS, 60, "skew must match client.rs DEFAULT_SKEW_SECS");

  const endpoint = await startTokenEndpoint((params, res) => {
    tokenJson(res, { access_token: "refreshed", refresh_token: "r2", expires_in: 3600 });
  });
  t.after(endpoint.close);

  const now = Math.floor(Date.now() / 1000);
  const soon = (secs, rt) =>
    new auth.Tokens({ accessToken: `live-${rt}`, refreshToken: rt, expiresAt: now + secs });

  // 30s from expiry: inside the 60s skew window => refresh even though not yet expired.
  const storeA = withTempStore(t);
  storeA.saveTokens(soon(30, "rA"));
  const a = manager(endpoint, storeA, soon(30, "rA"));
  assert.equal(await a.accessToken(), "refreshed");
  assert.equal(endpoint.requests.length, 1);

  // 3600s from expiry: outside the window => the stored token is returned untouched.
  const storeB = withTempStore(t);
  const b = manager(endpoint, storeB, soon(3600, "rB"));
  assert.equal(await b.accessToken(), "live-rB");
  assert.equal(endpoint.requests.length, 1, "a fresh token must not trigger a refresh");
});

test("concurrent accessToken() calls in one process serialize to a SINGLE refresh", async (t) => {
  const endpoint = await startTokenEndpoint((params, res, callIndex) => {
    assert.equal(params.get("refresh_token"), "r1", "a second call would burn r2");
    // Delay the response so both callers are genuinely in flight together.
    setTimeout(
      () => tokenJson(res, { access_token: "fresh", refresh_token: "r2", expires_in: 3600 }),
      50
    );
  });
  t.after(endpoint.close);
  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r1"));

  const m = manager(endpoint, store, expiredTokens("r1"));
  const [x, y] = await Promise.all([m.accessToken(), m.accessToken()]);
  assert.equal(x, "fresh");
  assert.equal(y, "fresh");
  assert.equal(endpoint.requests.length, 1, "one rotation, shared by both callers");
  assert.equal(store.loadTokens().refreshToken(), "r2");
});

test("token-endpoint calls have a hard timeout (default pinned to 30s)", async (t) => {
  assert.equal(
    auth.TOKEN_ENDPOINT_TIMEOUT_MS,
    30000,
    "default timeout must match client.rs TOKEN_ENDPOINT_TIMEOUT (30s)"
  );

  // A hanging endpoint must abort at the configured bound, not wedge forever.
  const endpoint = await startTokenEndpoint(() => {
    /* never respond */
  });
  t.after(endpoint.close);
  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r1"));

  const m = manager(endpoint, store, expiredTokens("r1"), { timeoutMs: 200 });
  const started = Date.now();
  await assert.rejects(
    () => m.accessToken(),
    (e) => e.name === "TimeoutError" || e.name === "AbortError"
  );
  assert.ok(Date.now() - started < 5000, "the timeout must bound the stall");
});

test("absent records produce the typed not-authenticated / missing-credentials errors", async (t) => {
  const store = withTempStore(t);
  const noTokens = new auth.TokenManager({ store, credentials: credentials() });
  assert.equal(noTokens.isAuthenticated(), false);
  await assert.rejects(() => noTokens.accessToken(), auth.NotAuthenticatedError);

  const noCreds = new auth.TokenManager({ store, tokens: expiredTokens("r1") });
  assert.equal(noCreds.isAuthenticated(), true);
  await assert.rejects(() => noCreds.accessToken(), auth.MissingClientCredentialsError);
});

test("accessTokenProvider is a Configuration-compatible async accessToken function", async (t) => {
  const endpoint = await startTokenEndpoint((params, res) => {
    tokenJson(res, { access_token: "fresh", refresh_token: "r2", expires_in: 3600 });
  });
  t.after(endpoint.close);
  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r1"));

  const provider = auth.accessTokenProvider(manager(endpoint, store, expiredTokens("r1")));
  // The generated runtime calls it as (name, scopes) and awaits a string.
  assert.equal(await provider("OAuth2", []), "fresh");
  assert.equal(await provider(), "fresh", "second call reuses the fresh token");
  assert.equal(endpoint.requests.length, 1);
});
