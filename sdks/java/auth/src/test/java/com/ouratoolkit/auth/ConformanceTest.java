package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.time.Instant;
import java.util.ArrayDeque;
import java.util.Base64;
import java.util.Collections;
import java.util.IdentityHashMap;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Optional;
import java.util.Set;
import java.util.TreeSet;
import java.util.concurrent.atomic.AtomicReference;
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
 *       Bearer would burn the still-valid rotated refresh token); a case's
 *       {@code must_not_echo} string appears nowhere in the exception's chain;</li>
 *   <li>implementation-defined 2xx token responses → EITHER a successful refresh that
 *       persists access_token {@code at-refreshed}, a refresh_token that is
 *       {@code rt-refreshed} or the prior one (never empty), and expires_at = refresh
 *       time + 3600, OR the typed {@link TransportException} with {@code tokens.json}
 *       byte-identical — never an unchecked throw, a wrong persisted record, or a
 *       half-written store;</li>
 *   <li>hostile store files → the typed {@link StoreException}, never a null-filled
 *       {@link Tokens} that would make {@code isAuthenticated} lie, never an unchecked
 *       crash; a case's {@code must_not_echo} string appears nowhere in the exception's
 *       chain;</li>
 *   <li>rejected (non-2xx) token responses → the typed {@link TokenEndpointException}
 *       carrying the case's status, the store byte-identical, the case's {@code must_echo}
 *       diagnosis kept, a {@code must_not_echo} submitted secret redacted everywhere in the
 *       chain, and no text in the chain longer than {@code max_error_chars};</li>
 *   <li>implementation-defined store files → EITHER exactly the case's {@code expected}
 *       record OR the typed {@link StoreException} — never an unchecked throw/Error;</li>
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
     * companion exactly as authored. A case must carry EXACTLY ONE of the three — two
     * would make which bytes are served depend on this helper's precedence (and differ
     * across legs), none would serve nothing — and fails naming the case otherwise.
     */
    private static byte[] caseBody(JsonNode testCase) throws IOException {
        String name = testCase.get("name").asText();
        int sources = 0;
        for (String key : new String[] {"body", "raw_body", "raw_body_base64"}) {
            if (testCase.has(key)) {
                sources++;
            }
        }
        assertEquals(1, sources, name + ": case must carry EXACTLY ONE of body / raw_body / "
                + "raw_body_base64 (has " + sources + ")");
        if (testCase.has("raw_body_base64")) {
            JsonNode b64 = testCase.get("raw_body_base64");
            assertTrue(b64.isTextual(), name + ": raw_body_base64 must be a string");
            return Base64.getDecoder().decode(b64.asText());
        }
        if (testCase.has("raw_body")) {
            JsonNode raw = testCase.get("raw_body");
            assertTrue(raw.isTextual(), name + ": raw_body must be a string");
            return raw.asText().getBytes(StandardCharsets.UTF_8);
        }
        return MAPPER.writeValueAsBytes(testCase.get("body"));
    }

    /**
     * A case's optional {@code must_not_echo} needle, validated: a non-empty string that
     * genuinely occurs in the case's served bytes / file content (else the no-echo
     * assertion would pass vacuously). Empty when the case carries none.
     */
    private static Optional<String> mustNotEcho(JsonNode testCase, byte[] payload) {
        String name = testCase.get("name").asText();
        JsonNode needle = testCase.get("must_not_echo");
        if (needle == null) {
            return Optional.empty();
        }
        assertTrue(needle.isTextual() && !needle.asText().isEmpty(),
                name + ": must_not_echo must be a non-empty string");
        assertTrue(containsBytes(payload, needle.asText().getBytes(StandardCharsets.UTF_8)),
                name + ": must_not_echo must occur in the case's payload, or the no-echo "
                        + "check is vacuous");
        return Optional.of(needle.asText());
    }

    /**
     * Byte-level substring search — the payload may be deliberately invalid UTF-8
     * ({@code raw_body_base64} / {@code content_base64}), so it is never decoded (a lossy
     * decode could mangle the bytes around the needle).
     */
    private static boolean containsBytes(byte[] haystack, byte[] needle) {
        outer:
        for (int i = 0; i + needle.length <= haystack.length; i++) {
            for (int j = 0; j < needle.length; j++) {
                if (haystack[i + j] != needle[j]) {
                    continue outer;
                }
            }
            return true;
        }
        return false;
    }

    /**
     * The no-echo floor: at least {@code min} cases of {@code table} carry
     * {@code must_not_echo}, so the secret-leak checks can't silently vanish from the
     * fixture (a shrink guard on the no-echo coverage itself).
     */
    private static void assertNoEchoFloor(String table, JsonNode cases, int min) {
        int carrying = 0;
        for (JsonNode c : cases) {
            if (c.has("must_not_echo")) {
                carrying++;
            }
        }
        assertTrue(carrying >= min, "fixture lost no-echo coverage? " + carrying + " "
                + table + " cases carry must_not_echo, want >= " + min);
    }

    /**
     * The exact bytes a store-file case writes: {@code content_base64} decoded (bytes JSON
     * can't hold, e.g. invalid UTF-8) or {@code content} as UTF-8 text. EXACTLY ONE must be
     * present — fails naming the case otherwise.
     */
    private static byte[] caseFileContent(JsonNode testCase) {
        String name = testCase.get("name").asText();
        boolean text = testCase.has("content");
        boolean b64 = testCase.has("content_base64");
        assertTrue(text ^ b64, name + ": case must carry EXACTLY ONE of content / "
                + "content_base64 (has " + ((text ? 1 : 0) + (b64 ? 1 : 0)) + ")");
        if (b64) {
            JsonNode v = testCase.get("content_base64");
            assertTrue(v.isTextual(), name + ": content_base64 must be a string");
            return Base64.getDecoder().decode(v.asText());
        }
        JsonNode v = testCase.get("content");
        assertTrue(v.isTextual(), name + ": content must be a string");
        return v.asText().getBytes(StandardCharsets.UTF_8);
    }

    /**
     * The {@code must_not_echo} contract: the needle appears in NEITHER the typed
     * exception's {@code toString()} NOR any throwable it chains — every {@code getCause()}
     * recursively and every suppressed exception (a logged stack trace prints them all).
     * Walks by identity so a cyclic chain terminates.
     */
    private static void assertChainDoesNotEcho(String name, Throwable top, String needle) {
        for (Map.Entry<String, String> text : chainTexts(top).entrySet()) {
            assertTrue(!text.getValue().contains(needle),
                    name + ": must_not_echo — " + text.getKey()
                            + " in the exception chain echoes the case's secret text");
        }
    }

    /**
     * Every text the exception chain exposes, keyed by where it came from: each
     * throwable's {@code toString()} and {@code getMessage()} (and a
     * {@link TokenEndpointException}'s {@code getBody()}), for {@code top}, every
     * {@code getCause()} recursively and every suppressed exception (a logged stack trace
     * prints them all). Walks by identity so a cyclic chain terminates.
     */
    private static Map<String, String> chainTexts(Throwable top) {
        Map<String, String> texts = new LinkedHashMap<>();
        Set<Throwable> seen = Collections.newSetFromMap(new IdentityHashMap<>());
        ArrayDeque<Throwable> todo = new ArrayDeque<>();
        todo.push(top);
        int i = 0;
        while (!todo.isEmpty()) {
            Throwable t = todo.pop();
            if (!seen.add(t)) {
                continue;
            }
            String where = "#" + i++ + " " + t.getClass().getName();
            texts.put(where + ".toString()", String.valueOf(t));
            texts.put(where + ".getMessage()", String.valueOf(t.getMessage()));
            if (t instanceof TokenEndpointException) {
                texts.put(where + ".getBody()",
                        String.valueOf(((TokenEndpointException) t).getBody()));
            }
            if (t.getCause() != null) {
                todo.push(t.getCause());
            }
            for (Throwable s : t.getSuppressed()) {
                todo.push(s);
            }
        }
        return texts;
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
        assertTrue(cases.size() >= 30,
                "fixture shrank? hostile_token_responses has " + cases.size()
                        + " cases, want >= 30");
        assertNoEchoFloor("hostile_token_responses", cases, 4);
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
            TransportException thrown = assertThrows(TransportException.class, m::forceRefresh,
                    name + ": a hostile 2xx must surface the typed TransportException");
            Optional<String> needle = mustNotEcho(testCase, body);
            if (needle.isPresent()) {
                assertChainDoesNotEcho(name, thrown, needle.get());
            }
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

    // --- 1b. implementation-defined 2xx token responses ----------------------------------

    /**
     * 2xx bodies where companions may legitimately differ (a leading UTF-8 BOM, duplicate
     * keys, nesting past a parser's depth limit, an integral float expires_in, an
     * upper-case key). Seeded exactly like the hostile harness; the refresh must EITHER
     * succeed and persist access_token {@code at-refreshed}, a refresh_token that is
     * {@code rt-refreshed} or the prior stored one (never empty), and expires_at =
     * (time of the refresh) + 3600, OR fail with the typed {@link TransportException}
     * leaving both records byte-identical. Anything else — an unchecked exception/error
     * (e.g. a StackOverflowError on deep nesting), a different persisted record, or a
     * changed store on failure — fails, naming the case.
     */
    @TestFactory
    Stream<DynamicTest> implementationDefinedTokenResponsesSucceedOrFailTypedCleanly()
            throws IOException {
        JsonNode cases = fixture().get("implementation_defined_token_responses");
        assertNotNull(cases, "fixture lost its implementation_defined_token_responses table");
        assertTrue(cases.size() >= 5,
                "fixture shrank? implementation_defined_token_responses has " + cases.size()
                        + " cases, want >= 5");
        return StreamSupport.stream(cases.spliterator(), false)
                .map(c -> DynamicTest.dynamicTest(
                        c.get("name").asText(),
                        () -> assertImplementationDefinedResponseHandledCleanly(c)));
    }

    private void assertImplementationDefinedResponseHandledCleanly(JsonNode testCase)
            throws Exception {
        String name = testCase.get("name").asText();
        byte[] body = caseBody(testCase);

        Path dir = caseDir("impl-defined-" + name);
        TokenStore store = new TokenStore(dir);
        store.saveCredentials(new ClientCredentials("cid", "secret"));
        store.saveTokens(expiredTokens("r1"));
        byte[] tokensBefore = Files.readAllBytes(store.tokensPath());
        byte[] credsBefore = Files.readAllBytes(store.credentialsPath());

        try (TokenEndpointStub stub = new TokenEndpointStub(
                form -> new TokenEndpointStub.Response(200, body))) {
            TokenManager m = new TokenManager(
                    store, new ClientCredentials("cid", "secret"), expiredTokens("r1"));
            m.overrideTokenUrl(stub.url());

            Throwable thrown = null;
            long t0 = Instant.now().getEpochSecond();
            try {
                m.forceRefresh();
            } catch (Throwable t) { // incl. Errors: a StackOverflowError must fail HERE, named
                thrown = t;
            }
            long t1 = Instant.now().getEpochSecond();
            assertEquals(1, stub.requests.get(),
                    name + ": a 2xx is not a 400 — the endpoint must be hit exactly once");
            if (thrown == null) {
                Tokens persisted = store.loadTokens().orElseThrow(() -> new AssertionError(
                        name + ": a successful refresh must persist tokens.json"));
                assertEquals("at-refreshed", persisted.getAccessToken(),
                        name + ": an ACCEPTED implementation-defined response must persist "
                                + "access_token = at-refreshed exactly");
                String rt = persisted.getRefreshToken();
                assertTrue("rt-refreshed".equals(rt) || "r1".equals(rt),
                        name + ": an ACCEPTED implementation-defined response must persist "
                                + "refresh_token rt-refreshed or the prior r1 (never empty / "
                                + "anything else), got " + (rt == null ? "null"
                                        : rt.isEmpty() ? "\"\"" : "another value"));
                long expiresAt = persisted.getExpiresAt();
                assertTrue(expiresAt >= t0 + 3600 && expiresAt <= t1 + 3600,
                        name + ": an ACCEPTED implementation-defined response must persist "
                                + "expires_at = refresh time + 3600, i.e. within ["
                                + (t0 + 3600) + ", " + (t1 + 3600) + "], got " + expiresAt);
            } else {
                if (!(thrown instanceof TransportException)) {
                    throw new AssertionError(name + ": an implementation-defined response "
                            + "must either succeed or fail with the typed TransportException, "
                            + "got " + thrown.getClass().getName(), thrown);
                }
                assertArrayEquals(tokensBefore, Files.readAllBytes(store.tokensPath()),
                        name + ": a REJECTED implementation-defined response must leave "
                                + "tokens.json byte-identical (no half-written store)");
                assertArrayEquals(credsBefore, Files.readAllBytes(store.credentialsPath()),
                        name + ": credentials.json must be byte-identical after a failed "
                                + "refresh");
            }
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
        assertTrue(cases.size() >= 21,
                "fixture shrank? hostile_store_files has " + cases.size()
                        + " cases, want >= 21");
        assertNoEchoFloor("hostile_store_files", cases, 5);
        return StreamSupport.stream(cases.spliterator(), false)
                .map(c -> DynamicTest.dynamicTest(
                        c.get("name").asText(), () -> assertHostileStoreFileRejected(c)));
    }

    private void assertHostileStoreFileRejected(JsonNode testCase) throws Exception {
        String name = testCase.get("name").asText();
        String file = testCase.get("file").asText();
        byte[] bytes = caseFileContent(testCase);

        Path dir = caseDir("store-" + name);
        TokenStore store = new TokenStore(dir);
        Files.write(dir.resolve(file), bytes);

        final StoreException thrown;
        switch (file) {
            case "tokens.json":
                thrown = assertThrows(StoreException.class, store::loadTokens,
                        name + ": a hostile tokens.json must surface the typed "
                                + "StoreException — never a null-filled Tokens, never an "
                                + "unchecked crash");
                break;
            case "credentials.json":
                thrown = assertThrows(StoreException.class, store::loadCredentials,
                        name + ": a hostile credentials.json must surface the typed "
                                + "StoreException — never a null-filled record, never an "
                                + "unchecked crash");
                break;
            default:
                throw new AssertionError(name + ": fixture names an unknown store file: "
                        + file);
        }
        Optional<String> needle = mustNotEcho(testCase, bytes);
        if (needle.isPresent()) {
            assertChainDoesNotEcho(name, thrown, needle.get());
        }
    }

    // --- 2c. rejected (non-2xx) token responses ---------------------------------------------

    /**
     * A NON-2xx token response, seeded so the refresh sends exactly the fixture's
     * {@code submitted} refresh_token + client_secret: the refresh must fail with the typed
     * {@link TokenEndpointException} carrying the case's status and leave both records
     * byte-identical; the body is kept for diagnosis ({@code must_echo} appears in the
     * exception text) but a submitted secret the server echoed is REDACTED
     * ({@code must_not_echo} appears nowhere in the chain), and no text in the chain
     * exceeds {@code max_error_chars} however large the body.
     */
    @TestFactory
    Stream<DynamicTest> rejectedTokenResponsesFailTypedRedactedAndCapped() throws IOException {
        JsonNode table = fixture().get("rejected_token_responses");
        assertNotNull(table, "fixture lost its rejected_token_responses table");
        JsonNode submitted = table.get("submitted");
        assertNotNull(submitted, "fixture's rejected_token_responses lost its submitted record");
        String refreshToken = requiredText(submitted, "refresh_token", "submitted");
        String clientSecret = requiredText(submitted, "client_secret", "submitted");
        assertTrue(!refreshToken.isEmpty() && !clientSecret.isEmpty(),
                "rejected_token_responses.submitted secrets must be non-empty");
        JsonNode maxNode = table.get("max_error_chars");
        assertTrue(maxNode != null && maxNode.isIntegralNumber() && maxNode.asInt() > 0,
                "rejected_token_responses.max_error_chars must be a positive integer");
        int maxErrorChars = maxNode.asInt();
        JsonNode cases = table.get("cases");
        assertNotNull(cases, "fixture's rejected_token_responses lost its cases");
        assertTrue(cases.size() >= 4,
                "fixture shrank? rejected_token_responses has " + cases.size()
                        + " cases, want >= 4");
        assertNoEchoFloor("rejected_token_responses", cases, 3);
        return StreamSupport.stream(cases.spliterator(), false)
                .map(c -> DynamicTest.dynamicTest(
                        c.get("name").asText(),
                        () -> assertRejectedTokenResponseHandled(
                                refreshToken, clientSecret, maxErrorChars, c)));
    }

    private void assertRejectedTokenResponseHandled(
            String refreshToken, String clientSecret, int maxErrorChars, JsonNode testCase)
            throws Exception {
        String name = testCase.get("name").asText();
        JsonNode statusNode = testCase.get("status");
        assertTrue(statusNode != null && statusNode.isIntegralNumber()
                        && (statusNode.asInt() < 200 || statusNode.asInt() >= 300),
                name + ": status must be a non-2xx integer");
        int status = statusNode.asInt();
        String rawBody = requiredText(testCase, "raw_body", name);
        byte[] body = rawBody.getBytes(StandardCharsets.UTF_8);
        String mustEcho = requiredText(testCase, "must_echo", name);
        assertTrue(!mustEcho.isEmpty() && rawBody.contains(mustEcho),
                name + ": must_echo must be a non-empty string that occurs in raw_body, or the "
                        + "echo check is vacuous");
        Optional<String> needle = mustNotEcho(testCase, body);

        Path dir = caseDir("rejected-" + name);
        TokenStore store = new TokenStore(dir);
        store.saveCredentials(new ClientCredentials("cid", clientSecret));
        // Expired on purpose, so the refresh genuinely calls the endpoint.
        store.saveTokens(expiredTokens(refreshToken));
        byte[] tokensBefore = Files.readAllBytes(store.tokensPath());
        byte[] credsBefore = Files.readAllBytes(store.credentialsPath());

        AtomicReference<String> sentRefresh = new AtomicReference<>();
        AtomicReference<String> sentSecret = new AtomicReference<>();
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            sentRefresh.set(form.get("refresh_token"));
            sentSecret.set(form.get("client_secret"));
            return new TokenEndpointStub.Response(status, body);
        })) {
            TokenManager m = new TokenManager(store, store.loadCredentials().orElseThrow(),
                    store.loadTokens().orElseThrow());
            m.overrideTokenUrl(stub.url());

            TokenEndpointException thrown = assertThrows(TokenEndpointException.class,
                    m::forceRefresh,
                    name + ": a non-2xx must surface the typed TokenEndpointException");
            assertEquals(status, thrown.getStatus(),
                    name + ": the TokenEndpointException must carry the response status");
            int calls = stub.requests.get();
            assertTrue(calls == 1 || (status == 400 && calls == 2),
                    name + ": the endpoint must be hit once (a 400 may add ONE reload-retry), "
                            + "got " + calls);
            assertEquals(refreshToken, sentRefresh.get(),
                    name + ": the refresh must SEND the submitted refresh_token");
            assertEquals(clientSecret, sentSecret.get(),
                    name + ": the refresh must SEND the submitted client_secret");

            assertTrue(String.valueOf(thrown).contains(mustEcho),
                    name + ": must_echo — the error body is kept for diagnosis, so the "
                            + "exception text must contain \"" + mustEcho + "\"");
            if (needle.isPresent()) {
                assertChainDoesNotEcho(name, thrown, needle.get());
            }
            for (Map.Entry<String, String> text : chainTexts(thrown).entrySet()) {
                assertTrue(text.getValue().length() <= maxErrorChars,
                        name + ": max_error_chars — " + text.getKey() + " is "
                                + text.getValue().length() + " chars, want <= " + maxErrorChars
                                + " (the error body must be capped)");
            }
            assertArrayEquals(tokensBefore, Files.readAllBytes(store.tokensPath()),
                    name + ": tokens.json must be byte-identical after a rejected refresh");
            assertArrayEquals(credsBefore, Files.readAllBytes(store.credentialsPath()),
                    name + ": credentials.json must be byte-identical after a rejected "
                            + "refresh");
        }
    }

    // --- 2b. implementation-defined store files -------------------------------------------

    /**
     * Store contents a parser may accept or reject (nesting past its depth limit inside
     * an unknown field). Loading must EITHER return exactly the case's {@code expected}
     * record OR fail with the typed {@link StoreException}. Anything else — an unchecked
     * exception or Error (e.g. a StackOverflowError), or a record differing from
     * {@code expected} — fails, naming the case.
     */
    @TestFactory
    Stream<DynamicTest> implementationDefinedStoreFilesLoadExactlyOrFailTyped()
            throws IOException {
        JsonNode cases = fixture().get("implementation_defined_store_files");
        assertNotNull(cases, "fixture lost its implementation_defined_store_files table");
        assertTrue(cases.size() >= 1,
                "fixture shrank? implementation_defined_store_files has " + cases.size()
                        + " cases, want >= 1");
        return StreamSupport.stream(cases.spliterator(), false)
                .map(c -> DynamicTest.dynamicTest(
                        c.get("name").asText(),
                        () -> assertImplementationDefinedStoreFileHandledCleanly(c)));
    }

    private static String optionalText(JsonNode record, String field, String what) {
        JsonNode value = record.get(field);
        if (value == null) {
            return null;
        }
        assertTrue(value.isTextual(), what + "." + field + " must be a string");
        return value.asText();
    }

    private void assertImplementationDefinedStoreFileHandledCleanly(JsonNode testCase)
            throws Exception {
        String name = testCase.get("name").asText();
        String file = testCase.get("file").asText();
        byte[] content = caseFileContent(testCase);
        JsonNode expected = testCase.get("expected");
        assertNotNull(expected, name + ": case lacks its expected record");
        String what = name + ".expected";

        // The record a successful load must equal EXACTLY (all fields — an absent optional
        // field in `expected` must load as absent, not as some default).
        final Object want;
        switch (file) {
            case "tokens.json": {
                JsonNode exp = expected.get("expires_at");
                assertTrue(exp != null && exp.canConvertToLong(),
                        what + ".expires_at must be an integer");
                want = new Tokens(
                        requiredText(expected, "access_token", what),
                        requiredText(expected, "refresh_token", what),
                        exp.asLong(),
                        optionalText(expected, "scope", what),
                        optionalText(expected, "token_type", what));
                break;
            }
            case "credentials.json":
                want = new ClientCredentials(
                        requiredText(expected, "client_id", what),
                        requiredText(expected, "client_secret", what));
                break;
            default:
                throw new AssertionError(name + ": fixture names an unknown store file: "
                        + file);
        }

        Path dir = caseDir("store-impl-defined-" + name);
        TokenStore store = new TokenStore(dir);
        Files.write(dir.resolve(file), content);

        Optional<?> loaded;
        try {
            loaded = "tokens.json".equals(file) ? store.loadTokens() : store.loadCredentials();
        } catch (StoreException e) {
            return; // the typed rejection arm
        } catch (Throwable t) { // incl. Errors: a StackOverflowError must fail HERE, named
            throw new AssertionError(name + ": an implementation-defined store file must "
                    + "either load exactly `expected` or fail with the typed "
                    + "StoreException, got " + t.getClass().getName(), t);
        }
        assertTrue(loaded.isPresent(),
                name + ": an ACCEPTED implementation-defined store file must load a record");
        assertEquals(want, loaded.get(),
                name + ": an ACCEPTED implementation-defined store file must load EXACTLY "
                        + "`expected`");
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
        assertTrue(cases.size() >= 18,
                "fixture shrank? refresh_success_cases has " + cases.size()
                        + " cases, want >= 18");
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

        AtomicReference<String> sentRefreshToken = new AtomicReference<>();
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            sentRefreshToken.set(form.get("refresh_token"));
            return new TokenEndpointStub.Response(200, body);
        })) {
            TokenManager m = new TokenManager(
                    store, new ClientCredentials("cid", "secret"), prior);
            m.overrideTokenUrl(stub.url());

            long t0 = Instant.now().getEpochSecond();
            m.forceRefresh();
            long t1 = Instant.now().getEpochSecond();

            assertEquals(1, stub.requests.get(),
                    name + ": the refresh must call the token endpoint exactly once");
            assertEquals("rt-prior", prior.getRefreshToken(),
                    "fixture's refresh_success_cases.prior.refresh_token drifted from rt-prior");
            assertEquals(prior.getRefreshToken(), sentRefreshToken.get(),
                    name + ": the refresh request must SEND the prior refresh_token");
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
     * The fixture's tables must be EXACTLY the seven this suite maps (plus {@code $comment}):
     * a NEW table fails here (this leg must be extended deliberately — beats ten
     * silently-unexercised cases), and so does a renamed/removed one.
     */
    @Test
    void everyFixtureTableIsMappedByThisSuite() throws IOException {
        Set<String> known = new TreeSet<>(Set.of(
                "$comment",
                "hostile_token_responses",
                "implementation_defined_token_responses",
                "hostile_store_files",
                "rejected_token_responses",
                "implementation_defined_store_files",
                "refresh_success_cases",
                "valid_records"));
        Set<String> actual = new TreeSet<>();
        fixture().fieldNames().forEachRemaining(actual::add);
        assertEquals(known, actual,
                "the shared fixture's tables must be exactly the ones this Java leg maps — "
                        + "extend ConformanceTest for a new table; a missing one means the "
                        + "fixture lost coverage");
    }
}
