// Secret-redaction attack tests (TESTING & VERIFICATION rule 5): client_secret,
// access_token and refresh_token must never escape via JSON.stringify, util.inspect
// (console.log's path), toString/template literals, or nesting — including inspect
// invocations that deliberately try to defeat the custom renderer.
"use strict";

const assert = require("node:assert/strict");
const { test } = require("node:test");
const util = require("node:util");

const { auth } = require("./helpers.cjs");

const CS = "SECRET-CS-789";
const AT = "SECRET-AT-123";
const RT = "SECRET-RT-456";

function creds() {
  return new auth.ClientCredentials({ clientId: "cid", clientSecret: CS });
}

function tokens() {
  return new auth.Tokens({
    accessToken: AT,
    refreshToken: RT,
    expiresAt: 4102444800,
    scope: "daily personal",
    tokenType: "Bearer",
  });
}

/** Every rendering a stray log/serialize call could produce. */
function renderings(value) {
  return [
    ["JSON.stringify", JSON.stringify(value)],
    ["JSON.stringify nested", JSON.stringify({ nested: [value] })],
    ["util.inspect", util.inspect(value)],
    ["util.inspect deep+hidden", util.inspect({ nested: [value] }, { depth: 10, showHidden: true })],
    // Defeat attempts: disable the custom renderer and ask for getters/hidden props.
    [
      "util.inspect customInspect:false",
      util.inspect(value, { customInspect: false, showHidden: true, getters: true, depth: 10 }),
    ],
    ["toString / template literal", `${value}`],
    ["String()", String(value)],
  ];
}

test("ClientCredentials never leaks client_secret through any rendering", () => {
  for (const [name, rendered] of renderings(creds())) {
    assert.ok(!rendered.includes(CS), `client_secret leaked via ${name}: ${rendered}`);
  }
  // The redaction must not hide the non-secret fields (that would mask bugs elsewhere).
  assert.ok(util.inspect(creds()).includes("cid"), "client_id should remain visible");
  assert.ok(`${creds()}`.includes("[REDACTED]"));
});

test("Tokens never leaks access_token or refresh_token through any rendering", () => {
  for (const [name, rendered] of renderings(tokens())) {
    assert.ok(!rendered.includes(AT), `access_token leaked via ${name}: ${rendered}`);
    assert.ok(!rendered.includes(RT), `refresh_token leaked via ${name}: ${rendered}`);
  }
  const shown = util.inspect(tokens());
  assert.ok(shown.includes("4102444800"), "expires_at should remain visible");
  assert.ok(shown.includes("[REDACTED]"));
});

test("secrets are not own enumerable properties (spread/entries cannot leak them)", () => {
  const spreadCreds = JSON.stringify({ ...creds() });
  assert.ok(!spreadCreds.includes(CS), `client_secret leaked via spread: ${spreadCreds}`);
  const entries = JSON.stringify(Object.entries(tokens()));
  assert.ok(!entries.includes(AT) && !entries.includes(RT), `tokens leaked via entries: ${entries}`);
});

test("redaction does not break the real accessors (the store still gets real values)", () => {
  assert.equal(creds().clientSecret(), CS);
  assert.equal(tokens().accessToken(), AT);
  assert.equal(tokens().refreshToken(), RT);
});

test("TokenManager never leaks credentials or tokens through any rendering (Python repr parity)", () => {
  const dir = require("node:fs").mkdtempSync(
    require("node:path").join(require("node:os").tmpdir(), "oura-toolkit-auth-mgr-")
  );
  const manager = new auth.TokenManager({
    store: new auth.TokenStore(dir),
    credentials: creds(),
    tokens: tokens(),
  });
  // The Python companion pins `repr(TokenManager)` free of secrets; the TS analogue is
  // that a stray console.log / JSON.stringify of the manager never surfaces token material.
  for (const [name, rendered] of renderings(manager)) {
    for (const secret of [CS, AT, RT]) {
      assert.ok(!rendered.includes(secret), `TokenManager leaked a secret via ${name}: ${rendered}`);
    }
  }
});
