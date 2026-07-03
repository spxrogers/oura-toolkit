// Token-store guarantees: the on-disk schema is the Rust companion's serde output
// (shared-store interop), records are 0600/0700 on Unix, and empty/relative env values
// never resolve to a cwd-dependent secret location.
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const { auth, tempStoreDir } = require("./helpers.cjs");

const isWindows = process.platform === "win32";

test("store schema: on-disk JSON matches the Rust structs' serde output exactly", (t) => {
  const dir = tempStoreDir();
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  const store = new auth.TokenStore(dir);

  store.saveCredentials(
    new auth.ClientCredentials({ clientId: "cid", clientSecret: "SECRET-CS-789" })
  );
  store.saveTokens(
    new auth.Tokens({
      accessToken: "SECRET-AT-123",
      refreshToken: "SECRET-RT-456",
      expiresAt: 4102444800,
      scope: "daily personal",
      tokenType: "Bearer",
    })
  );

  // Fixture literals transcribed from sdks/rust/oura-toolkit-auth/src/store.rs:
  // ClientCredentials { client_id, client_secret } and Tokens { access_token,
  // refresh_token, expires_at, scope, token_type }. A renamed/missing/extra field is a
  // shared-store interop break with the Rust CLI/MCP server.
  assert.deepEqual(JSON.parse(fs.readFileSync(path.join(dir, "credentials.json"), "utf8")), {
    client_id: "cid",
    client_secret: "SECRET-CS-789",
  });
  assert.deepEqual(JSON.parse(fs.readFileSync(path.join(dir, "tokens.json"), "utf8")), {
    access_token: "SECRET-AT-123",
    refresh_token: "SECRET-RT-456",
    expires_at: 4102444800,
    scope: "daily personal",
    token_type: "Bearer",
  });

  // Round-trip through load preserves every field.
  const creds = store.loadCredentials();
  assert.equal(creds.clientId, "cid");
  assert.equal(creds.clientSecret(), "SECRET-CS-789");
  const tokens = store.loadTokens();
  assert.equal(tokens.accessToken(), "SECRET-AT-123");
  assert.equal(tokens.refreshToken(), "SECRET-RT-456");
  assert.equal(tokens.expiresAt, 4102444800);
  assert.equal(tokens.scope, "daily personal");
  assert.equal(tokens.tokenType, "Bearer");
});

test("store schema: absent optional fields are OMITTED, not null (serde skip_serializing_if)", (t) => {
  const dir = tempStoreDir();
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  const store = new auth.TokenStore(dir);

  store.saveTokens(
    new auth.Tokens({ accessToken: "a", refreshToken: "r", expiresAt: 123 })
  );
  const raw = JSON.parse(fs.readFileSync(path.join(dir, "tokens.json"), "utf8"));
  assert.deepEqual(raw, { access_token: "a", refresh_token: "r", expires_at: 123 });
  assert.ok(!("scope" in raw), "absent scope must be omitted, not serialized as null");
  assert.ok(!("token_type" in raw), "absent token_type must be omitted");
});

test("empty store loads as null; corrupt records throw a typed StoreFormatError", (t) => {
  const dir = tempStoreDir();
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  const store = new auth.TokenStore(dir);

  assert.equal(store.loadCredentials(), null);
  assert.equal(store.loadTokens(), null);

  fs.writeFileSync(path.join(dir, "tokens.json"), "{not json");
  assert.throws(() => store.loadTokens(), auth.StoreFormatError);
  fs.writeFileSync(path.join(dir, "credentials.json"), JSON.stringify({ client_id: "cid" }));
  assert.throws(() => store.loadCredentials(), auth.StoreFormatError);
});

test(
  "unix file hygiene: records are 0600, the store dir is 0700",
  { skip: isWindows && "unix permission modes are no-ops on Windows (ACLs protect)" },
  (t) => {
    const base = tempStoreDir();
    t.after(() => fs.rmSync(base, { recursive: true, force: true }));
    const dir = path.join(base, "store"); // created by the store itself
    const store = new auth.TokenStore(dir);

    store.saveCredentials(new auth.ClientCredentials({ clientId: "c", clientSecret: "s" }));
    store.saveTokens(new auth.Tokens({ accessToken: "a", refreshToken: "r", expiresAt: 1 }));

    assert.equal(
      fs.statSync(dir).mode & 0o777,
      0o700,
      "store dir must be owner-only (0700)"
    );
    for (const record of ["credentials.json", "tokens.json"]) {
      assert.equal(
        fs.statSync(path.join(dir, record)).mode & 0o777,
        0o600,
        `${record} must be owner-only (0600)`
      );
    }
  }
);

test("atomic write: a save never leaves a temp file behind", (t) => {
  const dir = tempStoreDir();
  t.after(() => fs.rmSync(dir, { recursive: true, force: true }));
  const store = new auth.TokenStore(dir);
  store.saveTokens(new auth.Tokens({ accessToken: "a", refreshToken: "r", expiresAt: 1 }));
  store.saveTokens(new auth.Tokens({ accessToken: "b", refreshToken: "r2", expiresAt: 2 }));
  assert.deepEqual(fs.readdirSync(dir).sort(), ["tokens.json"]);
  assert.equal(store.loadTokens().accessToken(), "b");
});

// --- config-dir resolution (injected env lookups; process.env is never mutated) -------

function env(pairs) {
  return (key) => pairs[key];
}

test("unix config dir: prefers $XDG_CONFIG_HOME, falls back to $HOME/.config", () => {
  assert.equal(
    auth.configDirFrom(env({ XDG_CONFIG_HOME: "/xdg", HOME: "/home/u" }), "linux"),
    "/xdg/oura-toolkit"
  );
  assert.equal(
    auth.configDirFrom(env({ HOME: "/home/u" }), "darwin"),
    "/home/u/.config/oura-toolkit"
  );
});

test("unix config dir: EMPTY or RELATIVE XDG_CONFIG_HOME is ignored (falls back to HOME)", () => {
  for (const bad of ["", "relative/config"]) {
    assert.equal(
      auth.configDirFrom(env({ XDG_CONFIG_HOME: bad, HOME: "/home/u" }), "linux"),
      "/home/u/.config/oura-toolkit",
      `XDG_CONFIG_HOME=${JSON.stringify(bad)} must be ignored — a relative base would ` +
        "make secret placement cwd-dependent"
    );
  }
});

test("unix config dir: EMPTY or RELATIVE HOME errors instead of resolving", () => {
  for (const bad of ["", "relative/home"]) {
    assert.throws(
      () => auth.configDirFrom(env({ HOME: bad }), "linux"),
      auth.NoConfigDirError,
      `HOME=${JSON.stringify(bad)} must not resolve`
    );
  }
  assert.throws(() => auth.configDirFrom(env({}), "linux"), auth.NoConfigDirError);
});

test("windows config dir: %LOCALAPPDATA% (Local, never Roaming); empty/relative errors", () => {
  assert.equal(
    auth.configDirFrom(
      env({
        LOCALAPPDATA: "C:\\Users\\u\\AppData\\Local",
        APPDATA: "C:\\Users\\u\\AppData\\Roaming",
      }),
      "win32"
    ),
    "C:\\Users\\u\\AppData\\Local\\oura-toolkit",
    "must use machine-local %LOCALAPPDATA%, never the roaming profile"
  );
  for (const bad of ["", "relative\\path"]) {
    assert.throws(
      () => auth.configDirFrom(env({ LOCALAPPDATA: bad }), "win32"),
      auth.NoConfigDirError,
      `LOCALAPPDATA=${JSON.stringify(bad)} must not resolve`
    );
  }
  // Windows users must not be told about Unix env vars.
  assert.throws(() => auth.configDirFrom(env({}), "win32"), /%LOCALAPPDATA%/);
});

test("config dir name is the locked oura-toolkit (identical to the Rust store)", () => {
  assert.equal(auth.APP_DIR_NAME, "oura-toolkit");
});
