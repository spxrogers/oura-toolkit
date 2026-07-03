package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.net.http.HttpTimeoutException;
import java.nio.file.Path;
import java.time.Duration;
import java.time.Instant;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Timeout;
import org.junit.jupiter.api.io.TempDir;

/**
 * The refresh protocol, mirrored from the Rust suite: proactive skew, rotation
 * persistence, reload-and-adopt, the 400-reload-retry-once arm, and the hard endpoint
 * timeout that bounds lock-hold time. All hermetic — the token endpoint is a loopback
 * {@link TokenEndpointStub}, stores live in tempdirs.
 */
class TokenManagerTest {

    @TempDir
    Path dir;

    private static ClientCredentials credentials() {
        return new ClientCredentials("cid", "secret");
    }

    private static Tokens expiredTokens(String refreshToken) {
        return new Tokens("stale-access-" + refreshToken, refreshToken, 0L, null, null);
    }

    private static Tokens freshTokens(String access, String refreshToken) {
        return new Tokens(
                access, refreshToken, Instant.now().getEpochSecond() + 3600, null, null);
    }

    private TokenManager manager(TokenEndpointStub stub, Tokens memory) {
        TokenManager m = new TokenManager(new TokenStore(dir), credentials(), memory);
        m.overrideTokenUrl(stub.url());
        return m;
    }

    @Test
    void accessTokenRequiresAuthentication() {
        TokenManager m = new TokenManager(new TokenStore(dir), credentials(), null);
        assertEquals(false, m.isAuthenticated());
        assertThrows(NotAuthenticatedException.class, m::getAccessToken);
    }

    @Test
    void refreshWithoutCredentialsReportsMissingCredentials() {
        TokenManager m = new TokenManager(new TokenStore(dir), null, expiredTokens("r1"));
        assertThrows(MissingClientCredentialsException.class, m::getAccessToken);
    }

    @Test
    void refreshSendsConfidentialClientFormAndPersistsTheRotation() throws Exception {
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            assertEquals("refresh_token", form.get("grant_type"));
            assertEquals("r1", form.get("refresh_token"));
            assertEquals("cid", form.get("client_id"));
            assertEquals("secret", form.get("client_secret"));
            return TokenEndpointStub.ok("fresh-access", "r2", 3600);
        })) {
            TokenStore store = new TokenStore(dir);
            store.saveTokens(expiredTokens("r1"));
            TokenManager m = manager(stub, expiredTokens("r1"));

            assertEquals("fresh-access", m.getAccessToken());
            assertEquals(1, stub.requests.get());
            assertEquals(
                    "r2",
                    store.loadTokens().orElseThrow().getRefreshToken(),
                    "the ROTATED refresh token must be persisted — Oura invalidates the "
                            + "old one, so losing r2 bricks the next refresh");
        }
    }

    @Test
    void refreshKeepsOldRefreshTokenIfServerOmitsRotation() throws Exception {
        try (TokenEndpointStub stub = new TokenEndpointStub(
                form -> TokenEndpointStub.ok("fresh-access", null, 3600))) {
            TokenStore store = new TokenStore(dir);
            store.saveTokens(expiredTokens("r1"));
            TokenManager m = manager(stub, expiredTokens("r1"));

            assertEquals("fresh-access", m.getAccessToken());
            assertEquals("r1", store.loadTokens().orElseThrow().getRefreshToken());
        }
    }

    @Test
    void secondManagerAdoptsRotationFromDiskWithoutCallingEndpoint() throws Exception {
        // Exactly ONE refresh is allowed, and only with r1: a second call would either
        // replay the invalidated r1 or burn the rotated r2.
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            if ("r1".equals(form.get("refresh_token"))) {
                return TokenEndpointStub.ok("fresh-access", "r2", 3600);
            }
            return TokenEndpointStub.invalidGrant();
        })) {
            TokenStore store = new TokenStore(dir);
            store.saveTokens(expiredTokens("r1"));

            // Both managers start from the same stale state — the pre-#22 failure mode:
            // B's refresh would replay the invalidated r1 and 400.
            TokenManager a = manager(stub, expiredTokens("r1"));
            TokenManager b = manager(stub, expiredTokens("r1"));

            assertEquals("fresh-access", a.getAccessToken()); // burns r1, persists r2
            assertEquals("fresh-access", b.getAccessToken()); // must ADOPT, not call
            assertEquals(1, stub.requests.get(),
                    "B reloaded under the lock and adopted disk state — no second call");
            assertEquals("r2", store.loadTokens().orElseThrow().getRefreshToken(),
                    "rotation persisted exactly once");
        }
    }

    @Test
    void forceRefreshAdoptsFresherDiskStateButRotatesOnIdenticalState() throws Exception {
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            if ("r2".equals(form.get("refresh_token"))) {
                return TokenEndpointStub.ok("r3-access", "r3", 3600);
            }
            return TokenEndpointStub.invalidGrant();
        })) {
            TokenStore store = new TokenStore(dir);
            store.saveTokens(freshTokens("fresh-access", "r2"));

            // B's request 401'd on a stale token: force must adopt the disk rotation
            // rather than burn r2 with an endpoint call.
            TokenManager b = manager(stub, expiredTokens("r1"));
            b.forceRefresh();
            assertEquals("fresh-access", b.getAccessToken());
            assertEquals(0, stub.requests.get(), "adoption must not touch the endpoint");

            // But when memory ALREADY matches disk (the fresh token itself 401'd — e.g.
            // revoked), an identical record is no fix: force must actually rotate.
            b.forceRefresh();
            assertEquals("r3-access", b.getAccessToken());
            assertEquals(1, stub.requests.get());
            assertEquals("r3", store.loadTokens().orElseThrow().getRefreshToken());
        }
    }

    @Test
    void refresh400ReloadsAndRetriesExactlyOnceAgainstFresherDiskState() throws Exception {
        TokenStore store = new TokenStore(dir);
        store.saveTokens(expiredTokens("r1"));

        // The 400-retry arm: an uncoordinated writer (on Linux, the Rust runtime — its
        // flock and our POSIX record lock don't interact) rotates to r2 while our r1
        // request is in flight. The endpoint 400s the stale r1; the manager must reload,
        // see r2, and retry once — successfully. Disk r2 is EXPIRED so the retry must
        // actually refresh, not adopt.
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            if ("r1".equals(form.get("refresh_token"))) {
                try {
                    store.saveTokens(expiredTokens("r2"));
                } catch (IOException e) {
                    throw new RuntimeException(e);
                }
                return TokenEndpointStub.invalidGrant();
            }
            if ("r2".equals(form.get("refresh_token"))) {
                return TokenEndpointStub.ok("r3-access", "r3", 3600);
            }
            return new TokenEndpointStub.Response(500, "unexpected token");
        })) {
            TokenManager m = manager(stub, expiredTokens("r1"));
            assertEquals("r3-access", m.getAccessToken());
            assertEquals(2, stub.requests.get(), "exactly one retry after the 400");
            assertEquals("r3", store.loadTokens().orElseThrow().getRefreshToken());
        }
    }

    @Test
    void genuinelyInvalidRefreshTokenSurfacesThe400WithoutBlindRetry() throws Exception {
        try (TokenEndpointStub stub = new TokenEndpointStub(
                form -> TokenEndpointStub.invalidGrant())) {
            TokenStore store = new TokenStore(dir);
            store.saveTokens(expiredTokens("r-dead"));
            TokenManager m = manager(stub, expiredTokens("r-dead"));

            TokenEndpointException e =
                    assertThrows(TokenEndpointException.class, m::getAccessToken);
            assertEquals(400, e.getStatus());
            assertTrue(e.getBody().contains("invalid_grant"));
            assertEquals(1, stub.requests.get(),
                    "the reload-retry fires only when disk moved past what we sent — "
                            + "a blind replay would double-burn a dead token");
        }
    }

    @Test
    void proactiveRefreshHonorsTheSkewWindow() throws Exception {
        try (TokenEndpointStub stub = new TokenEndpointStub(
                form -> TokenEndpointStub.ok("fresh-access", "r2", 3600))) {
            // 30s from expiry, skew 0: still valid — the endpoint must not be called.
            Tokens soon = new Tokens(
                    "soon-access", "r1", Instant.now().getEpochSecond() + 30, null, null);
            TokenStore store = new TokenStore(dir);
            store.saveTokens(soon);

            TokenManager noSkew = manager(stub, soon);
            noSkew.overrideSkewSecs(0);
            assertEquals("soon-access", noSkew.getAccessToken());
            assertEquals(0, stub.requests.get(), "30s out with no skew must not refresh");

            // Same token, skew 60: inside the window — must refresh proactively.
            TokenManager skewed = manager(stub, soon);
            skewed.overrideSkewSecs(60);
            assertEquals("fresh-access", skewed.getAccessToken());
            assertEquals(1, stub.requests.get(), "30s out with 60s skew must refresh");
        }
    }

    @Test
    void defaultTimingConstantsMatchTheDocumentedGuarantees() {
        assertEquals(Duration.ofSeconds(30), TokenManager.DEFAULT_ENDPOINT_TIMEOUT,
                "the documented hard 30s token-endpoint timeout (bounds lock-hold time)");
        assertEquals(60, TokenManager.DEFAULT_SKEW_SECS,
                "the documented 60s proactive-refresh skew");
    }

    @Test
    @Timeout(15)
    void hungEndpointTimesOutAndReleasesTheLock() throws Exception {
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            try {
                Thread.sleep(5_000);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
            return TokenEndpointStub.ok("late", "r2", 3600);
        })) {
            TokenStore store = new TokenStore(dir);
            store.saveTokens(expiredTokens("r1"));
            TokenManager m = manager(stub, expiredTokens("r1"));
            m.overrideEndpointTimeout(Duration.ofMillis(500));

            long started = System.nanoTime();
            TransportException e = assertThrows(TransportException.class, m::getAccessToken);
            assertInstanceOf(HttpTimeoutException.class, e.getCause(),
                    "the hard request timeout must be what fired");
            assertTrue(
                    Duration.ofNanos(System.nanoTime() - started).getSeconds() < 8,
                    "the timeout must bound the stall (and therefore the lock hold)");
            java.util.Optional<TokenStore.StoreLock> free = store.tryLockExclusive();
            assertTrue(free.isPresent(),
                    "the store lock must be RELEASED after a timed-out refresh — a wedged "
                            + "lock would starve every other process");
            free.get().close();
        }
    }
}
