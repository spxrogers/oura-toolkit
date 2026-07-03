// Spec-sync tripwire (mold: cli/oura-toolkit-cli/tests/docs_tripwire.rs): the metadata
// constants are transcriptions of the vendored spec's securitySchemes — this test walks
// up to the repo root, re-reads spec/openapi.json, and fails on any drift, so a spec
// refresh can never silently orphan the constants. Hermetic: filesystem only.
"use strict";

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { test } = require("node:test");

const { auth } = require("./helpers.cjs");

function repoRoot() {
  let dir = __dirname;
  for (;;) {
    if (fs.existsSync(path.join(dir, "justfile")) && fs.existsSync(path.join(dir, "README.md"))) {
      return dir;
    }
    const parent = path.dirname(dir);
    assert.notEqual(parent, dir, "repo root (justfile + README.md) not found above tests/");
    dir = parent;
  }
}

function specOAuthFlow() {
  const spec = JSON.parse(
    fs.readFileSync(path.join(repoRoot(), "spec", "openapi.json"), "utf8")
  );
  const flow = spec.components.securitySchemes.OAuth2.flows.authorizationCode;
  assert.ok(flow, "spec lost its OAuth2 authorizationCode flow");
  return flow;
}

test("AUTHORIZE_URL and TOKEN_URL match the vendored spec's securitySchemes", () => {
  const flow = specOAuthFlow();
  assert.equal(
    auth.AUTHORIZE_URL,
    flow.authorizationUrl,
    "AUTHORIZE_URL drifted from spec authorizationUrl — re-sync src/metadata.ts"
  );
  assert.equal(
    auth.TOKEN_URL,
    flow.tokenUrl,
    "TOKEN_URL drifted from spec tokenUrl — re-sync src/metadata.ts"
  );
});

test("ALL_SCOPES is exactly the spec's advertised scope set", () => {
  const flow = specOAuthFlow();
  assert.deepEqual(
    [...auth.ALL_SCOPES].sort(),
    Object.keys(flow.scopes).sort(),
    "ALL_SCOPES drifted from the spec's OAuth2 scopes — re-sync src/metadata.ts"
  );
});

test("DEFAULT_SCOPES is the spec scope set minus email (the toolkit's consent policy)", () => {
  const flow = specOAuthFlow();
  assert.deepEqual(
    [...auth.DEFAULT_SCOPES].sort(),
    Object.keys(flow.scopes)
      .filter((s) => s !== "email")
      .sort(),
    "DEFAULT_SCOPES must be every spec scope except email — a mismatch silently " +
      "narrows (or over-asks) the consent the toolkit requests"
  );
  assert.ok(!auth.DEFAULT_SCOPES.includes("email"), "email is opt-in, never default");
});
