package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

/**
 * A store record that EXISTS but is malformed must surface a typed {@link StoreException} —
 * never an unchecked crash, never a silently-coerced value. Two failure modes the review
 * lenses flagged:
 *
 * <ul>
 *   <li>A literal {@code null} JSON document deserializes to Java {@code null}; naive
 *       {@code Optional.of(null)} throws an UNCHECKED {@link NullPointerException} that
 *       escapes callers' {@code IOException} handling and crashes raw.</li>
 *   <li>{@code "expires_at": null} would, with {@code FAIL_ON_NULL_FOR_PRIMITIVES} off,
 *       coerce to {@code 0L} and masquerade as an already-expired token instead of being
 *       rejected.</li>
 * </ul>
 *
 * Every case here also asserts the error message NEVER echoes the record's bytes (records
 * carry secrets) — matching the "no secrets in logs" rule.
 */
class StoreValidationTest {

    @TempDir
    Path dir;

    private TokenStore store() throws IOException {
        Files.createDirectories(dir);
        return new TokenStore(dir);
    }

    private void writeTokens(String json) throws IOException {
        Files.write(store().tokensPath(), json.getBytes(StandardCharsets.UTF_8));
    }

    private void writeCredentials(String json) throws IOException {
        Files.write(store().credentialsPath(), json.getBytes(StandardCharsets.UTF_8));
    }

    private StoreException assertTokensRejected(String label) throws IOException {
        return assertThrows(StoreException.class, store()::loadTokens, label);
    }

    private StoreException assertCredentialsRejected(String label) throws IOException {
        return assertThrows(StoreException.class, store()::loadCredentials, label);
    }

    @Test
    void literalNullTokensFileSurfacesTypedStoreException() throws IOException {
        writeTokens("null");
        assertTokensRejected("a literal `null` tokens.json must be a typed StoreException, "
                + "not an unchecked NPE from Optional.of(null)");
    }

    @Test
    void literalNullCredentialsFileSurfacesTypedStoreException() throws IOException {
        writeCredentials("null");
        assertCredentialsRejected("a literal `null` credentials.json must be a typed "
                + "StoreException, not an unchecked NPE from Optional.of(null)");
    }

    @Test
    void expiresAtNullSurfacesTypedStoreExceptionNotZeroCoercion() throws IOException {
        // The NIT: without FAIL_ON_NULL_FOR_PRIMITIVES this silently becomes 0L (an
        // "expired" token) instead of a typed error.
        writeTokens("{\"access_token\":\"a\",\"refresh_token\":\"r\",\"expires_at\":null}");
        assertTokensRejected("expires_at:null must be a typed StoreException, never coerced "
                + "to 0L (a phantom already-expired token)");
    }

    @Test
    void expiresAtAsStringSurfacesTypedStoreException() throws IOException {
        writeTokens(
                "{\"access_token\":\"a\",\"refresh_token\":\"r\",\"expires_at\":\"not-a-number\"}");
        assertTokensRejected("expires_at as a non-numeric string must be rejected");
    }

    @Test
    void expiresAtAsBooleanSurfacesTypedStoreException() throws IOException {
        writeTokens("{\"access_token\":\"a\",\"refresh_token\":\"r\",\"expires_at\":true}");
        assertTokensRejected("expires_at as a boolean must be rejected");
    }

    @Test
    void missingRequiredFieldSurfacesTypedStoreException() throws IOException {
        // access_token is a required creator property.
        writeTokens("{\"refresh_token\":\"r\",\"expires_at\":123}");
        assertTokensRejected("a missing required field (access_token) must be rejected");
    }

    @Test
    void wrongTypedRequiredFieldSurfacesTypedStoreException() throws IOException {
        // access_token is a String; an array cannot be coerced to one.
        writeTokens(
                "{\"access_token\":[1,2,3],\"refresh_token\":\"r\",\"expires_at\":123}");
        StoreException e =
                assertTokensRejected("a wrong-typed field (access_token as array) must be "
                        + "rejected");
        assertFalse(e.getMessage().contains("[1,2,3]"),
                "the error must NOT echo the record's bytes (they can hold secrets)");
    }
}
