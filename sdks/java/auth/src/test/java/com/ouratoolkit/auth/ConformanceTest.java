package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assertions.fail;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Instant;
import java.util.ArrayList;
import java.util.Base64;
import java.util.List;
import java.util.stream.Stream;
import java.util.stream.StreamSupport;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.DynamicTest;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.TestFactory;
import org.junit.jupiter.api.io.TempDir;

/**
 * Cross-language auth-companion conformance (#58) — the JAVA leg.
 *
 * <p>Iterates {@code codegen/conformance/auth-cases.json} (the SINGLE SOURCE for the
 * hostile token-endpoint responses, hostile store files, and canonical store records
 * every companion suite must exercise; new cases are added THERE, never here):
 *
 * <ul>
 *   <li>hostile-but-2xx token responses → the typed {@link TransportException} (an
 *       {@link AuthException} subclass — never a raw {@code NullPointerException} /
 *       {@code ClassCastException} escaping), exactly ONE endpoint call (a hostile 2xx is
 *       not a 400 — the reload-retry arm must not misfire), and {@code tokens.json} /
 *       {@code credentials.json} byte-identical afterwards (persisting a blank/expired
 *       Bearer would burn the still-valid rotated refresh token);</li>
 *   <li>hostile store files → the typed {@link StoreException}, never a null-filled
 *       {@link Tokens} that would make {@code isAuthenticated} lie, never an unchecked
 *       crash;</li>
 *   <li>refresh success cases → a successful refresh from the stored {@code prior} record
 *       persists EXACTLY {@code expected}'s access_token, refresh_token, scope and
 *       token_type, with expires_at = (time of the refresh) + {@code expected.expires_in}
 *       (an omitted/null/non-string/empty/whitespace-only scope keeps the prior grant; an
 *       omitted/null/empty refresh_token or token_type keeps the prior value);</li>
 *   <li>canonical valid records → load with exactly the fixture's field values and
 *       round-trip through this module's own persist path (the cross-language store
 *       compatibility check — field names are the shared wire format, #54).</li>
 * </ul>
 *
 * <p>Mirrors the Rust reference leg ({@code sdks/rust/oura-toolkit-auth/tests/conformance.rs})
 * and the Go leg ({@code sdks/go/auth/conformance_test.go}). Monorepo-only by nature: the
 * fixture is resolved by walking up to the repo root (nearest ancestor holding the
 * justfile + README.md — the same walk as every other leg).
 */
class ConformanceTest {

    private static final ObjectMapper MAPPER = new ObjectMapper();

    @TempDir
    Path baseDir;

    // --- fixture loading -----------------------------------------------------------------

    /** Repo root: the nearest ancestor holding both the justfile and README.md. */
    private static Path repoRoot() {
        Path dir = Paths.get("").toAbsolutePath();
        while (dir != null) {
            if (Files.isRegularFile(dir.resolve("justfile"))
                    && Files.isRegularFile(dir.resolve("README.md"))) {
                return dir;
            }
            dir = dir.getParent();
        }
        throw new AssertionError("repo root (justfile + README.md) not found above cwd");
    }

    /** The decoded shared fixture; cases are always iterated FROM THE FILE. */
    private static JsonNode fixture() throws IOException {
        return MAPPER.readTree(Files.readAllBytes(
                repoRoot().resolve("codegen").resolve("conformance").resolve("auth-cases.json")));
    }

    private Path caseDir(String name) throws IOException {
        Path dir = baseDir.resolve(name);
        Files.createDirectories(dir);
        return dir;
    }

    private static Tokens expiredTokens(String refreshToken) {
        return new Tokens("stale-access", refreshToken, 0L, null, null);
    }

    /**
     * The exact bytes a case's token endpoint serves: {@code raw_body_base64} decoded
     * (bytes JSON can't hold, e.g. invalid UTF-8), {@code raw_body} VERBATIM (deliberately
     * not JSON, or escapes a re-serialization would normalize), else the structured
     * {@code body} re-serialized, so a wrong-typed field (42, "soon") reaches the
     * companion exactly as authored.
     */
    private static byte[] caseBody(JsonNode testCase) throws IOException {
        if (testCase.hasNonNull("raw_body_base64")) {
            return Base64.getDecoder().decode(testCase.get("raw_body_base64").asText());
        }
        if (testCase.hasNonNull("raw_body")) {
            return testCase.get("raw_body").asText().getBytes(StandardCharsets.UTF_8);
        }
        JsonNode body = testCase.get("body");
        assertNotNull(body, testCase.get("name").asText()
                + ": case has none of body / raw_body / raw_body_base64");
        return MAPPER.writeValueAsBytes(body);
    }

    // --- 1. hostile-but-2xx token responses ----------------------------------------------

    /**
     * Every hostile-but-2xx token response must fail the refresh with the typed
     * {@link TransportException} and leave BOTH persisted records byte-identical — the
     * rotated refresh token is never burned by a blank/expired Bearer. (An unchecked
     * NPE/ClassCastException escaping instead fails the {@code assertThrows} outright, so
     * passing proves "never an untyped crash".)
     */
    @TestFactory
    Stream<DynamicTest> hostile2xxTokenResponsesFailTypedAndLeaveStoreUntouched()
            throws IOException {
        JsonNode cases = fixture().get("hostile_token_responses");
        assertNotNull(cases, "fixture lost its hostile_token_responses table");
        assertTrue(cases.size() >= 24,
                "fixture shrank? hostile_token_responses has " + cases.size()
                        + " cases, want >= 24");
        return StreamSupport.stream(cases.spliterator(), false)
                .map(c -> DynamicTest.dynamicTest(
                        c.get("name").asText(), () -> assertHostileTokenResponseRejected(c)));
    }

    private void assertHostileTokenResponseRejected(JsonNode testCase) throws Exception {
        String name = testCase.get("name").asText();
        byte[] body = caseBody(testCase);

        Path dir = caseDir("hostile2xx-" + name);
        TokenStore store = new TokenStore(dir);
        store.saveCredentials(new ClientCredentials("cid", "secret"));
        // Expired on purpose, so the refresh genuinely calls the endpoint.
        store.saveTokens(expiredTokens("r1"));
        byte[] tokensBefore = Files.readAllBytes(store.tokensPath());
        byte[] credsBefore = Files.readAllBytes(store.credentialsPath());

        try (TokenEndpointStub stub = new TokenEndpointStub(
                form -> new TokenEndpointStub.Response(200, body))) {
            TokenManager m = new TokenManager(
                    store, new ClientCredentials("cid", "secret"), expiredTokens("r1"));
            m.overrideTokenUrl(stub.url());

            // Typed: the companion's invalid-response error — an AuthException subclass,
            // never a raw NPE/ClassCastException, and never TokenEndpointException (which
            // would mis-file a server-side 2xx fault as a re-login problem).
            assertThrows(TransportException.class, m::forceRefresh,
                    name + ": a hostile 2xx must surface the typed TransportException");
            assertEquals(1, stub.requests.get(),
                    name + ": a hostile 2xx is not a 400 — the reload-retry arm must NOT "
                            + "fire (endpoint hit exactly once)");
            assertArrayEquals(tokensBefore, Files.readAllBytes(store.tokensPath()),
                    name + ": tokens.json must be byte-identical (persisting a blank/"
                            + "expired Bearer would burn the still-valid rotation)");
            assertArrayEquals(credsBefore, Files.readAllBytes(store.credentialsPath()),
                    name + ": credentials.json must be byte-identical after a failed "
                            + "refresh");
        }
    }

    // --- 2. hostile store files -----------------------------------------------------------

    /**
     * Every hostile store file must fail its load with the typed {@link StoreException} —
     * never a null-filled record that makes {@code isAuthenticated} lie, never an
     * unchecked NPE escaping {@code Optional.of(null)} or a silently coerced value.
     */
    @TestFactory
    Stream<DynamicTest> hostileStoreFilesFailTyped() throws IOException {
        JsonNode cases = fixture().get("hostile_store_files");
        assertNotNull(cases, "fixture lost its hostile_store_files table");
        assertTrue(cases.size() >= 8,
                "fixture shrank? hostile_store_files has " + cases.size()
                        + " cases, want >= 8");
        return StreamSupport.stream(cases.spliterator(), false)
                .map(c -> DynamicTest.dynamicTest(
                        c.get("name").asText(), () -> assertHostileStoreFileRejected(c)));
    }

    private void assertHostileStoreFileRejected(JsonNode testCase) throws Exception {
        String name = testCase.get("name").asText();
        String file = testCase.get("file").asText();
        String content = testCase.get("content").asText();

        Path dir = caseDir("store-" + name);
        TokenStore store = new TokenStore(dir);
        Files.write(dir.resolve(file), content.getBytes(StandardCharsets.UTF_8));

        switch (file) {
            case "tokens.json":
                assertThrows(StoreException.class, store::loadTokens,
                        name + ": a hostile tokens.json must surface the typed "
                                + "StoreException — never a null-filled Tokens, never an "
                                + "unchecked crash");
                break;
            case "credentials.json":
                assertThrows(StoreException.class, store::loadCredentials,
                        name + ": a hostile credentials.json must surface the typed "
                                + "StoreException — never a null-filled record, never an "
                                + "unchecked crash");
                break;
            default:
                fail("fixture names an unknown store file: " + file);
        }
    }

    // --- 3. canonical valid records --------------------------------------------------------

    /**
     * The canonical records load with exactly the fixture's values and survive a
     * round-trip through this module's own persist path — the shared wire format every
     * language reads (#54). The literal expectations double as a fixture-drift tripwire,
     * mirroring the Rust reference leg.
     */
    @Test
    void canonicalValidRecordsLoadExactlyAndRoundTrip() throws Exception {
        JsonNode valid = fixture().get("valid_records");
        assertNotNull(valid, "fixture lost its valid_records table");
        JsonNode credsRecord = valid.get("credentials.json");
        JsonNode tokensRecord = valid.get("tokens.json");
        assertNotNull(credsRecord, "fixture is missing valid_records[credentials.json]");
        assertNotNull(tokensRecord, "fixture is missing valid_records[tokens.json]");

        Path dir = caseDir("valid-records");
        TokenStore store = new TokenStore(dir);
        Files.write(store.credentialsPath(),
                MAPPER.writeValueAsBytes(credsRecord));
        Files.write(store.tokensPath(),
                MAPPER.writeValueAsBytes(tokensRecord));

        ClientCredentials creds = store.loadCredentials().orElseThrow(
                () -> new AssertionError("canonical credentials.json must load"));
        assertEquals("cid-conformance", creds.getClientId(),
                "client_id must match the canonical record exactly");
        assertEquals("cs-conformance", creds.getClientSecret(),
                "client_secret must match the canonical record exactly");

        Tokens tokens = store.loadTokens().orElseThrow(
                () -> new AssertionError("canonical tokens.json must load"));
        assertEquals("at-conformance", tokens.getAccessToken(),
                "access_token must match the canonical record exactly");
        assertEquals("rt-conformance", tokens.getRefreshToken(),
                "refresh_token must match the canonical record exactly");
        assertEquals(4_102_444_800L, tokens.getExpiresAt(),
                "expires_at must match the canonical record exactly");
        assertEquals("personal daily", tokens.getScope(),
                "scope must match the canonical record exactly");
        assertEquals("Bearer", tokens.getTokenType(),
                "token_type must match the canonical record exactly");

        // Round-trip: this module's persist path must re-emit records the loader (and,
        // by the shared fixture, every other language) still reads identically.
        store.saveCredentials(creds);
        store.saveTokens(tokens);
        ClientCredentials credsAgain = store.loadCredentials().orElseThrow(
                () -> new AssertionError("credentials must reload after the round-trip"));
        assertEquals(creds, credsAgain,
                "credentials must round-trip through the persist path unchanged");
        Tokens tokensAgain = store.loadTokens().orElseThrow(
                () -> new AssertionError("tokens must reload after the round-trip"));
        assertEquals(tokens, tokensAgain,
                "tokens (all five fields incl. scope + token_type) must round-trip "
                        + "through the persist path unchanged");
    }

    // --- 4. successful refresh: exact persisted record ----------------------------------

    /**
     * A SUCCESSFUL refresh starting from the fixture's stored {@code prior} record must
     * persist EXACTLY each case's {@code expected} access_token, refresh_token, scope and
     * token_type, with expires_at = (time of the refresh) + {@code expected.expires_in}.
     * Fallbacks under test: an omitted/null/empty/whitespace-only (incl. U+00A0) or
     * NON-STRING {@code scope} keeps the prior grant; an omitted/null/EMPTY refresh_token or
     * token_type keeps the prior value; expires_in at the cap (2^31 - 1) succeeds; a lone
     * surrogate in an UNKNOWN field is not validated. (Malformed read fields are hostile
     * responses instead — see hostile_token_responses.)
     */
    @TestFactory
    Stream<DynamicTest> refreshSuccessCasesPersistExpectedRecord() throws IOException {
        JsonNode table = fixture().get("refresh_success_cases");
        assertNotNull(table, "fixture lost its refresh_success_cases table");
        JsonNode prior = table.get("prior");
        assertNotNull(prior, "fixture's refresh_success_cases lost its prior record");
        JsonNode cases = table.get("cases");
        assertNotNull(cases, "fixture's refresh_success_cases lost its cases");
        assertTrue(cases.size() >= 17,
                "fixture shrank? refresh_success_cases has " + cases.size()
                        + " cases, want >= 17");
        return StreamSupport.stream(cases.spliterator(), false)
                .map(c -> DynamicTest.dynamicTest(
                        c.get("name").asText(),
                        () -> assertRefreshPersistsExpectedRecord(prior, c)));
    }

    private static String requiredText(JsonNode record, String field, String what) {
        JsonNode value = record.get(field);
        assertNotNull(value, what + " lacks " + field);
        assertTrue(value.isTextual(), what + "." + field + " must be a string");
        return value.asText();
    }

    private void assertRefreshPersistsExpectedRecord(JsonNode priorRecord, JsonNode testCase)
            throws Exception {
        String name = testCase.get("name").asText();
        byte[] body = caseBody(testCase);
        JsonNode expected = testCase.get("expected");
        assertNotNull(expected, name + ": case lacks its expected record");
        JsonNode expiresInNode = expected.get("expires_in");
        assertTrue(expiresInNode != null && expiresInNode.canConvertToLong(),
                name + ": expected.expires_in must be an integer");
        long expectedExpiresIn = expiresInNode.asLong();

        Path dir = caseDir("refresh-success-" + name);
        TokenStore store = new TokenStore(dir);
        store.saveCredentials(new ClientCredentials("cid", "secret"));
        // Expired on purpose (expires_at 0), so the refresh genuinely calls the endpoint.
        Tokens prior = new Tokens(
                requiredText(priorRecord, "access_token", "prior"),
                requiredText(priorRecord, "refresh_token", "prior"),
                0L,
                requiredText(priorRecord, "scope", "prior"),
                requiredText(priorRecord, "token_type", "prior"));
        store.saveTokens(prior);

        try (TokenEndpointStub stub = new TokenEndpointStub(
                form -> new TokenEndpointStub.Response(200, body))) {
            TokenManager m = new TokenManager(
                    store, new ClientCredentials("cid", "secret"), prior);
            m.overrideTokenUrl(stub.url());

            long t0 = Instant.now().getEpochSecond();
            m.forceRefresh();
            long t1 = Instant.now().getEpochSecond();

            assertEquals(1, stub.requests.get(),
                    name + ": the refresh must call the token endpoint exactly once");
            Tokens persisted = store.loadTokens().orElseThrow(
                    () -> new AssertionError(name + ": a successful refresh must persist"));
            assertEquals(requiredText(expected, "access_token", name + ".expected"),
                    persisted.getAccessToken(),
                    name + ": persisted access_token must equal expected exactly");
            assertEquals(requiredText(expected, "refresh_token", name + ".expected"),
                    persisted.getRefreshToken(),
                    name + ": persisted refresh_token must equal expected exactly (an "
                            + "omitted/null/empty one keeps the prior value)");
            assertEquals(requiredText(expected, "scope", name + ".expected"),
                    persisted.getScope(),
                    name + ": persisted scope must equal expected exactly (a blank/absent/"
                            + "non-string scope keeps the prior grant)");
            assertEquals(requiredText(expected, "token_type", name + ".expected"),
                    persisted.getTokenType(),
                    name + ": persisted token_type must equal expected exactly (an "
                            + "omitted/null/empty one keeps the prior value)");
            long expiresAt = persisted.getExpiresAt();
            assertTrue(expiresAt >= t0 + expectedExpiresIn && expiresAt <= t1 + expectedExpiresIn,
                    name + ": persisted expires_at " + expiresAt + " must be the refresh time + "
                            + expectedExpiresIn + ", i.e. within [" + (t0 + expectedExpiresIn)
                            + ", " + (t1 + expectedExpiresIn) + "]");
        }
    }

    // --- sanity: the fixture's tables are the ones this suite knows how to map ------------

    /**
     * If the fixture grows a NEW table, this leg must be extended deliberately — an
     * unknown top-level key failing here beats ten silently-unexercised cases.
     */
    @Test
    void everyFixtureTableIsMappedByThisSuite() throws IOException {
        List<String> known = List.of(
                "$comment", "hostile_token_responses", "hostile_store_files", "valid_records",
                "refresh_success_cases");
        List<String> unknown = new ArrayList<>();
        fixture().fieldNames().forEachRemaining(f -> {
            if (!known.contains(f)) {
                unknown.add(f);
            }
        });
        assertEquals(List.of(), unknown,
                "the shared fixture grew tables this Java leg does not exercise — extend "
                        + "ConformanceTest to map them");
    }
}
