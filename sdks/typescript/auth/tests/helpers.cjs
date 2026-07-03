// Shared hermetic test helpers: a local node:http mock of the OAuth token endpoint and
// tempdir stores. No network beyond 127.0.0.1, no real credentials (TESTING &
// VERIFICATION rule 2). Tests exercise the BUILT package (../dist), so they also verify
// the artifact consumers install.
"use strict";

const fs = require("node:fs");
const http = require("node:http");
const os = require("node:os");
const path = require("node:path");

const auth = require("../dist/index.js");

/** A throwaway store dir; callers rm it in after-hooks. */
function tempStoreDir() {
  return fs.mkdtempSync(path.join(os.tmpdir(), "oura-toolkit-auth-test-"));
}

/**
 * Start a scripted token endpoint on 127.0.0.1. `respond(params, res, callIndex)`
 * receives the parsed x-www-form-urlencoded body; every request is recorded in
 * `requests` so tests can assert exact call counts and payloads.
 */
function startTokenEndpoint(respond) {
  const requests = [];
  const server = http.createServer((req, res) => {
    let body = "";
    req.on("data", (chunk) => (body += chunk));
    req.on("end", () => {
      const params = new URLSearchParams(body);
      requests.push({ method: req.method, params, url: req.url });
      respond(params, res, requests.length);
    });
  });
  return new Promise((resolve) => {
    server.listen(0, "127.0.0.1", () => {
      resolve({
        url: `http://127.0.0.1:${server.address().port}/oauth/token`,
        requests,
        close: () => new Promise((done) => server.close(done)),
      });
    });
  });
}

/** Respond 200 with a token-endpoint JSON body. */
function tokenJson(res, body) {
  res.writeHead(200, { "content-type": "application/json" });
  res.end(JSON.stringify(body));
}

function credentials() {
  return new auth.ClientCredentials({ clientId: "cid", clientSecret: "SECRET-CS-789" });
}

/** An already-expired token set carrying the given refresh token. */
function expiredTokens(refreshToken) {
  return new auth.Tokens({
    accessToken: `stale-access-${refreshToken}`,
    refreshToken,
    expiresAt: 0,
  });
}

module.exports = { auth, tempStoreDir, startTokenEndpoint, tokenJson, credentials, expiredTokens };
