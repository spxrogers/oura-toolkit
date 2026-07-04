package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Iterator;
import java.util.LinkedHashSet;
import java.util.Set;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

/**
 * Pins the on-disk record schema to the Rust companion's serde output: the CLI, the MCP
 * server, and this SDK all share ONE store, so a field-name drift here silently locks the
 * Java SDK out of (or corrupts) the shared records. Fixtures are transcribed from
 * {@code sdks/rust/oura-toolkit-auth/src/store.rs} ({@code sample_credentials} /
 * {@code sample_tokens} serialized with {@code serde_json::to_vec_pretty}).
 */
class StoreSchemaTest {

    /** store.rs `sample_credentials()` as serde_json::to_vec_pretty writes it. */
    private static final String RUST_CREDENTIALS_JSON =
            "{\n"
                    + "  \"client_id\": \"cid\",\n"
                    + "  \"client_secret\": \"SECRET-CS-789\"\n"
                    + "}";

    /** store.rs `sample_tokens()` as serde_json::to_vec_pretty writes it. */
    private static final String RUST_TOKENS_JSON =
            "{\n"
                    + "  \"access_token\": \"SECRET-AT-123\",\n"
                    + "  \"refresh_token\": \"SECRET-RT-456\",\n"
                    + "  \"expires_at\": 4102444800,\n"
                    + "  \"scope\": \"daily personal\",\n"
                    + "  \"token_type\": \"Bearer\"\n"
                    + "}";

    private static final ObjectMapper MAPPER = new ObjectMapper();

    @TempDir
    Path dir;

    @Test
    void readsRustWrittenCredentials() throws Exception {
        TokenStore store = new TokenStore(dir);
        Files.createDirectories(dir);
        Files.write(store.credentialsPath(), RUST_CREDENTIALS_JSON.getBytes(StandardCharsets.UTF_8));

        ClientCredentials creds = store.loadCredentials().orElseThrow();
        assertEquals("cid", creds.getClientId());
        assertEquals("SECRET-CS-789", creds.getClientSecret());
    }

    @Test
    void readsRustWrittenTokens() throws Exception {
        TokenStore store = new TokenStore(dir);
        Files.createDirectories(dir);
        Files.write(store.tokensPath(), RUST_TOKENS_JSON.getBytes(StandardCharsets.UTF_8));

        Tokens tokens = store.loadTokens().orElseThrow();
        assertEquals("SECRET-AT-123", tokens.getAccessToken());
        assertEquals("SECRET-RT-456", tokens.getRefreshToken());
        assertEquals(4_102_444_800L, tokens.getExpiresAt());
        assertEquals("daily personal", tokens.getScope());
        assertEquals("Bearer", tokens.getTokenType());
    }

    @Test
    void writesExactlyTheRustFieldNames() throws IOException {
        TokenStore store = new TokenStore(dir);
        store.saveCredentials(new ClientCredentials("cid", "SECRET-CS-789"));
        store.saveTokens(new Tokens(
                "SECRET-AT-123", "SECRET-RT-456", 4_102_444_800L, "daily personal", "Bearer"));

        JsonNode creds = MAPPER.readTree(Files.readAllBytes(store.credentialsPath()));
        assertEquals(
                Set.of("client_id", "client_secret"),
                fieldNames(creds),
                "credentials.json field names must match the Rust serde schema exactly");
        assertEquals("cid", creds.get("client_id").asText());
        assertEquals("SECRET-CS-789", creds.get("client_secret").asText());

        JsonNode tokens = MAPPER.readTree(Files.readAllBytes(store.tokensPath()));
        assertEquals(
                Set.of("access_token", "refresh_token", "expires_at", "scope", "token_type"),
                fieldNames(tokens),
                "tokens.json field names must match the Rust serde schema exactly");
        assertTrue(
                tokens.get("expires_at").isIntegralNumber(),
                "expires_at must be a JSON number (Unix seconds), not a string");
        assertEquals(4_102_444_800L, tokens.get("expires_at").asLong());
    }

    @Test
    void omitsNullScopeAndTokenTypeLikeSerdeSkipSerializingIf() throws IOException {
        TokenStore store = new TokenStore(dir);
        store.saveTokens(new Tokens("at", "rt", 4_102_444_800L, null, null));

        JsonNode tokens = MAPPER.readTree(Files.readAllBytes(store.tokensPath()));
        assertEquals(
                Set.of("access_token", "refresh_token", "expires_at"),
                fieldNames(tokens),
                "null scope/token_type must be OMITTED (serde skip_serializing_if), "
                        + "never written as JSON null");
    }

    @Test
    void toleratesUnknownFieldsLikeSerdeDefault() throws Exception {
        TokenStore store = new TokenStore(dir);
        Files.createDirectories(dir);
        String withExtra = RUST_TOKENS_JSON.replaceFirst("\\{",
                "{\n  \"future_field_from_newer_writer\": true,");
        Files.write(store.tokensPath(), withExtra.getBytes(StandardCharsets.UTF_8));

        assertEquals("SECRET-AT-123", store.loadTokens().orElseThrow().getAccessToken(),
                "a newer writer's extra field must not brick this reader (serde ignores unknowns)");
    }

    private static Set<String> fieldNames(JsonNode node) {
        Set<String> names = new LinkedHashSet<>();
        for (Iterator<String> it = node.fieldNames(); it.hasNext(); ) {
            names.add(it.next());
        }
        return names;
    }
}
