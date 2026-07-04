package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.nio.file.Path;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

/**
 * Attack test for the "no secrets in logs" invariant: rendering ANY companion type with
 * {@code toString()} — the thing every logger, debugger, and string-concat does — must
 * never emit the client secret, access token, or refresh token.
 */
class RedactionTest {

    private static final String CLIENT_SECRET = "SECRET-CS-789";
    private static final String ACCESS_TOKEN = "SECRET-AT-123";
    private static final String REFRESH_TOKEN = "SECRET-RT-456";

    @TempDir
    Path dir;

    @Test
    void clientCredentialsToStringRedactsTheSecret() {
        String rendered = new ClientCredentials("cid", CLIENT_SECRET).toString();
        assertFalse(rendered.contains(CLIENT_SECRET), "client secret leaked: " + rendered);
        assertTrue(rendered.contains("cid"), "client_id should remain visible for debugging");
        assertTrue(rendered.contains("[REDACTED]"));
    }

    @Test
    void tokensToStringRedactsBothTokens() {
        String rendered =
                new Tokens(ACCESS_TOKEN, REFRESH_TOKEN, 4_102_444_800L, "daily", "Bearer")
                        .toString();
        assertFalse(rendered.contains(ACCESS_TOKEN), "access token leaked: " + rendered);
        assertFalse(rendered.contains(REFRESH_TOKEN), "refresh token leaked: " + rendered);
        assertTrue(rendered.contains("[REDACTED]"));
        assertTrue(rendered.contains("4102444800"), "non-secret expiry stays visible");
    }

    @Test
    void tokenManagerToStringLeaksNoSecretEvenWithBothRecordsLoaded() {
        TokenManager manager = new TokenManager(
                new TokenStore(dir),
                new ClientCredentials("cid", CLIENT_SECRET),
                new Tokens(ACCESS_TOKEN, REFRESH_TOKEN, 4_102_444_800L, null, null));
        String rendered = manager.toString();
        assertFalse(rendered.contains(CLIENT_SECRET), "client secret leaked: " + rendered);
        assertFalse(rendered.contains(ACCESS_TOKEN), "access token leaked: " + rendered);
        assertFalse(rendered.contains(REFRESH_TOKEN), "refresh token leaked: " + rendered);
    }
}
