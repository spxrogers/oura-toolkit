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
  // The timeout is a fetch rejection (a DOMException, not an HTTP response). It must be
  // WRAPPED into a typed AuthError subclass — otherwise a caller doing `try { ... } catch
  // (e) { if (e instanceof AuthError) ... }` would miss a stalled token endpoint entirely.
  await assert.rejects(
    () => m.accessToken(),
    (e) => {
      assert.ok(e instanceof auth.AuthError, `a timeout must surface as an AuthError, got ${e}`);
      assert.ok(
        e instanceof auth.TokenEndpointTransportError,
        "a transport failure must be the TokenEndpointTransportError variant"
      );
      // The underlying cause is preserved and IS the abort/timeout (load-bearing: proves
      // the wrapper wrapped a real timeout, not some unrelated error).
      assert.ok(
        e.cause && (e.cause.name === "TimeoutError" || e.cause.name === "AbortError"),
        `the wrapped cause must be the abort/timeout, got ${e.cause && e.cause.name}`
      );
      // Secret hygiene: the wrapper message never carries request material.
      assert.ok(!e.message.includes("SECRET"), "transport error must not leak secrets");
      return true;
    }
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

// A raw 2xx endpoint that writes `body` verbatim (bypasses tokenJson's JSON.stringify so
// we can send non-JSON, empty, and hand-built hostile payloads).
async function raw2xxEndpoint(t, body, status = 200) {
  const endpoint = await startTokenEndpoint((_params, res) => {
    res.writeHead(status, { "content-type": "application/json" });
    res.end(body);
  });
  t.after(endpoint.close);
  return endpoint;
}

// TESTING & VERIFICATION rule 5 (attack test): a hostile or broken 2xx must fail as the
// typed TokenEndpointError, must NEVER yield or persist a token, and — the burn-prevention
// guarantee — must leave the store's still-valid refresh token UNTOUCHED. (A half-parsed
// 2xx overwriting r1 with garbage would 400 every future refresh, long after Oura already
// invalidated r1.) Mirrors go/auth/oauth_test.go and python test_manager's 2xx attack set.
for (const [name, body] of [
  ["non-JSON body", "not json at all"],
  ["empty body", ""],
  ["empty object", "{}"],
  ["missing access_token", JSON.stringify({ refresh_token: "r2-INJECTED", expires_in: 3600 })],
  ["empty access_token", JSON.stringify({ access_token: "", expires_in: 3600 })],
  ["missing expires_in", JSON.stringify({ access_token: "a2-INJECTED", refresh_token: "r2-INJECTED" })],
  ["string expires_in", JSON.stringify({ access_token: "a2-INJECTED", expires_in: "3600" })],
  ["zero expires_in", JSON.stringify({ access_token: "a2-INJECTED", expires_in: 0 })],
  ["negative expires_in", JSON.stringify({ access_token: "a2-INJECTED", expires_in: -100 })],
  ["array body", JSON.stringify([{ access_token: "a2-INJECTED", expires_in: 3600 }])],
]) {
  test(`hostile 2xx (${name}): typed error, no token, store left untouched`, async (t) => {
    const endpoint = await raw2xxEndpoint(t, body);
    const store = withTempStore(t);
    store.saveTokens(expiredTokens("r1"));

    const m = manager(endpoint, store, expiredTokens("r1"));
    let thrown;
    await assert.rejects(
      () => m.accessToken(),
      (e) => {
        thrown = e;
        assert.ok(
          e instanceof auth.TokenEndpointError,
          `a hostile 2xx must surface a typed TokenEndpointError, got ${e}`
        );
        // The 2xx status is preserved so the manager's 400-reload-retry arm never misfires.
        assert.equal(e.status, 200, "the 2xx status must be preserved, not rewritten to 400");
        // The fixed description never echoes response material (a partial 2xx may carry
        // token bytes). If the payload smuggled tokens, they must not reach the error body.
        assert.ok(
          !e.body.includes("INJECTED"),
          `the typed error body must not echo response material: ${e.body}`
        );
        return true;
      }
    );
    // accessToken must reject, never resolve to a blank/garbage Bearer.
    assert.ok(thrown, "expected a rejection, not a resolved token");

    // Burn-prevention: disk still holds the original, still-valid r1 record verbatim.
    const disk = store.loadTokens();
    assert.equal(disk.refreshToken(), "r1", "hostile 2xx must not overwrite the refresh token");
    assert.equal(disk.accessToken(), "stale-access-r1", "hostile 2xx must not touch the store");

    // A non-400 failure must NOT trigger the reload-retry arm (exactly one endpoint call).
    assert.equal(endpoint.requests.length, 1, "a hostile 2xx must not trigger the retry arm");
  });
}

test("a 2xx that OMITS refresh_token keeps the previous refresh token (rotation fallback)", async (t) => {
  // Oura always rotates, but the fallback must be exercised: the server may omit it, and
  // we must keep r1 rather than persist an empty refresh token (which would 400 next time).
  const endpoint = await startTokenEndpoint((_params, res) => {
    tokenJson(res, { access_token: "kept-refresh-access", expires_in: 3600 });
  });
  t.after(endpoint.close);
  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r1"));

  const m = manager(endpoint, store, expiredTokens("r1"));
  assert.equal(await m.accessToken(), "kept-refresh-access");

  const disk = store.loadTokens();
  assert.equal(disk.accessToken(), "kept-refresh-access", "the new access token is persisted");
  assert.equal(
    disk.refreshToken(),
    "r1",
    "an omitted refresh_token must fall back to the prior one, never blank it"
  );
});

test("confidential client REFUSES token-endpoint redirects (client_secret never re-POSTed)", async (t) => {
  // If the token endpoint 307s, Node's default fetch would re-POST the form body —
  // client_secret included — to the Location. `redirect:"error"` must reject instead, so
  // the redirect target is never contacted and the secret cannot leak there. 307 (not 302)
  // preserves the method+body, which is precisely the leak we are refusing.
  const target = await startTokenEndpoint((_params, res) => {
    // A successful-looking refresh: if we ever reach here, the client wrongly followed and
    // handed this server client_id/client_secret.
    tokenJson(res, { access_token: "leaked-access", refresh_token: "r2", expires_in: 3600 });
  });
  t.after(target.close);
  const redirector = await startTokenEndpoint((_params, res) => {
    res.writeHead(307, { location: target.url });
    res.end();
  });
  t.after(redirector.close);

  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r1"));
  const m = manager(redirector, store, expiredTokens("r1"));

  await assert.rejects(
    () => m.accessToken(),
    (e) => {
      assert.ok(
        e instanceof auth.AuthError,
        `a refused redirect must surface as an AuthError, got ${e}`
      );
      return true;
    }
  );
  // THE guarantee: the secret never reached the redirect target.
  assert.equal(
    target.requests.length,
    0,
    "confidential client must NOT re-POST client_secret to a redirect target"
  );
  // A refused redirect rotates nothing — disk keeps its original refresh token.
  assert.equal(store.loadTokens().refreshToken(), "r1", "a refused redirect leaves the store untouched");
});

test("a rejected refresh does not poison the mutex: a later accessToken() re-attempts", async (t) => {
  // The in-process mutex must swallow a rejection so the NEXT critical section still runs.
  // If the chain propagated the failure, the second call would reuse the rejected promise
  // and never retry — a single transient 400 would wedge the manager forever.
  let call = 0;
  const endpoint = await startTokenEndpoint((_params, res) => {
    call += 1;
    if (call === 1) {
      res.writeHead(400);
      res.end("invalid_grant");
    } else {
      tokenJson(res, { access_token: "recovered", refresh_token: "r2", expires_in: 3600 });
    }
  });
  t.after(endpoint.close);
  const store = withTempStore(t);
  store.saveTokens(expiredTokens("r1"));

  const m = manager(endpoint, store, expiredTokens("r1"));
  // First attempt: the endpoint 400s and disk has not moved past r1 => surfaces, no retry.
  await assert.rejects(() => m.accessToken(), auth.TokenEndpointError);
  // Second attempt: the mutex was not poisoned, so this RE-RUNS the refresh and succeeds.
  assert.equal(await m.accessToken(), "recovered", "a later call must re-attempt, not reuse the rejection");
  assert.equal(endpoint.requests.length, 2, "the second call made its own endpoint request");
  assert.equal(store.loadTokens().refreshToken(), "r2", "the recovered rotation is persisted");
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
