package com.ouratoolkit.auth;

import java.io.IOException;
import java.net.URI;
import java.net.URLEncoder;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.nio.charset.StandardCharsets;
import java.time.Duration;
import java.time.Instant;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Optional;
import java.util.StringJoiner;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * The runtime auth layer: owns the current tokens, refreshes them proactively (expiry
 * skew), and persists every rotation. Mirrors the Rust companion's {@code TokenManager}
 * behavior so the two runtimes can safely share one on-disk store.
 *
 * <p><b>Refresh strategy:</b> proactive — {@link #getAccessToken()} refreshes when the
 * access token is expired or within the skew window, so requests carry a valid token.
 * Reactive refresh-on-401-then-retry is done by callers via {@link #forceRefresh()}.
 *
 * <p><b>Cross-process safety:</b> Oura invalidates the previous refresh token on every
 * rotation, and multiple processes (this SDK, the Rust CLI, the MCP server) share one
 * store. Every refresh therefore runs under the store's exclusive lock and RE-READS the
 * store first — if another process already rotated, its fresher tokens are ADOPTED
 * instead of burning (and thereby invalidating) that rotation with a second refresh. A
 * refresh that still 400s is retried ONCE against freshly reloaded disk state before
 * surfacing "re-login". Because Java's file lock does not interact with the Rust
 * runtime's {@code flock} on Linux (see {@link TokenStore}), this reload+retry protocol —
 * not the lock — is the cross-runtime guarantee.
 *
 * <p>The token endpoint has a hard {@link #DEFAULT_ENDPOINT_TIMEOUT 30s} timeout, which
 * bounds how long the store lock can be held (worst case ~2×: the 400-retry arm can chain
 * a second endpoint call under the same lock).
 */
public final class TokenManager {

    /** Refresh this many seconds before the token's actual expiry. */
    public static final long DEFAULT_SKEW_SECS = 60;

    /**
     * Hard timeout on each token-endpoint call — load-bearing: the call runs under the
     * store's exclusive lock, so an unbounded hang would wedge every other process
     * waiting on it.
     */
    public static final Duration DEFAULT_ENDPOINT_TIMEOUT = Duration.ofSeconds(30);

    private static final ObjectMapper MAPPER = new ObjectMapper()
            .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);

    private final TokenStore store;
    private final ClientCredentials credentials; // nullable: tokens-only use is legal
    private final HttpClient http;
    private Tokens tokens; // guarded by `this`

    /** The spec-derived token endpoint; overridden only by tests (mock server). */
    private String tokenUrl = OuraOAuthMetadata.TOKEN_URL;

    private long skewSecs = DEFAULT_SKEW_SECS;
    private Duration endpointTimeout = DEFAULT_ENDPOINT_TIMEOUT;

    /**
     * Construct from an explicit store + optional in-memory records. Both records are
     * independently optional: credentials-without-tokens is "setup done, no login yet";
     * tokens-without-credentials is a caller-supplied token that can be used until expiry
     * but not refreshed ({@link MissingClientCredentialsException}).
     */
    public TokenManager(TokenStore store, ClientCredentials credentials, Tokens tokens) {
        this.store = store;
        this.credentials = credentials;
        this.tokens = tokens;
        this.http = HttpClient.newBuilder()
                .connectTimeout(Duration.ofSeconds(10))
                // Pin the JDK default: NEVER follow redirects from the token endpoint. A
                // 3xx from a hostile/misconfigured endpoint must NOT silently re-POST the
                // confidential-client form (client_id + client_secret + refresh_token) to
                // the redirect target — it surfaces as a typed non-2xx error instead.
                .followRedirects(HttpClient.Redirect.NEVER)
                .build();
    }

    /**
     * Load from the default per-platform store. Absent records are not an error —
     * {@link #getAccessToken()} reports {@link NotAuthenticatedException} on first use.
     */
    public static TokenManager load() throws AuthException {
        TokenStore store = TokenStore.openDefault();
        try {
            return new TokenManager(
                    store,
                    store.loadCredentials().orElse(null),
                    store.loadTokens().orElse(null));
        } catch (IOException e) {
            throw new StoreException("reading the token store failed", e);
        }
    }

    /** Whether tokens are loaded (does not validate them or imply refresh is possible). */
    public synchronized boolean isAuthenticated() {
        return tokens != null;
    }

    /**
     * Return a valid access token, refreshing (and persisting the rotation) if it is
     * expired or within the skew window.
     */
    public synchronized String getAccessToken() throws AuthException {
        if (tokens == null) {
            throw new NotAuthenticatedException();
        }
        if (tokens.isExpired(skewSecs)) {
            refreshCriticalSection();
        }
        return tokens.getAccessToken();
    }

    /**
     * Force a refresh regardless of expiry (used by callers on a 401), persisting the
     * rotation. If another process already rotated, its fresher tokens are adopted
     * instead of burning that rotation with a second refresh.
     */
    public synchronized void forceRefresh() throws AuthException {
        refreshCriticalSection();
    }

    /**
     * The reload → refresh → persist critical section, run under the store's exclusive
     * lock. The adopt rule covers both entry points: if disk holds tokens that differ
     * from memory and aren't expired, another process already rotated — adopt them. (On
     * the proactive path memory is expired, so anything fresher is strictly better; on
     * the force path memory just 401'd, so a DIFFERENT fresh token is the fix and an
     * identical one means we must rotate.)
     */
    private void refreshCriticalSection() throws AuthException {
        if (credentials == null) {
            throw new MissingClientCredentialsException();
        }
        try (TokenStore.StoreLock ignored = lockStore()) {
            Optional<Tokens> diskOpt = loadTokensFromDisk();
            if (diskOpt.isPresent()) {
                Tokens disk = diskOpt.get();
                boolean differs = tokens == null
                        || !tokens.getAccessToken().equals(disk.getAccessToken());
                if (differs && !disk.isExpired(skewSecs)) {
                    tokens = disk; // adopt the rotation another process performed
                    return;
                }
                // Refresh from the freshest persisted rotation, never from stale memory.
                tokens = disk;
            }
            if (tokens == null) {
                throw new NotAuthenticatedException();
            }

            Tokens refreshed;
            try {
                refreshed = callTokenEndpoint(tokens);
            } catch (TokenEndpointException e) {
                // A 400 usually means the refresh token we sent is no longer valid. If
                // disk moved past what we sent (a rotation by a writer not coordinated
                // through this lock — e.g. the Rust runtime on Linux), retry ONCE with
                // the fresher token before surfacing "re-login".
                if (e.getStatus() != 400) {
                    throw e;
                }
                Optional<Tokens> reloaded = loadTokensFromDisk();
                if (reloaded.isPresent()
                        && !reloaded.get().getRefreshToken().equals(tokens.getRefreshToken())) {
                    tokens = reloaded.get();
                    refreshed = callTokenEndpoint(tokens);
                } else {
                    throw e;
                }
            }

            try {
                store.saveTokens(refreshed);
            } catch (IOException e) {
                throw new StoreException("persisting the rotated tokens failed", e);
            }
            tokens = refreshed;
        }
    }

    private TokenStore.StoreLock lockStore() throws StoreException {
        try {
            return store.lockExclusive();
        } catch (IOException e) {
            throw new StoreException("locking the token store failed", e);
        }
    }

    private Optional<Tokens> loadTokensFromDisk() throws StoreException {
        try {
            return store.loadTokens();
        } catch (IOException e) {
            throw new StoreException("reading the token store failed", e);
        }
    }

    /**
     * POST the refresh grant (confidential client: {@code client_id} + {@code
     * client_secret} in the form body — never in a URL query string). Oura ROTATES the
     * refresh token: the returned {@link Tokens} carry the new one, falling back to the
     * current one only if the server omits it.
     */
    private Tokens callTokenEndpoint(Tokens current) throws AuthException {
        Map<String, String> form = new LinkedHashMap<>();
        form.put("grant_type", "refresh_token");
        form.put("refresh_token", current.getRefreshToken());
        form.put("client_id", credentials.getClientId());
        form.put("client_secret", credentials.getClientSecret());

        HttpRequest request = HttpRequest.newBuilder(URI.create(tokenUrl))
                .timeout(endpointTimeout)
                .header("Content-Type", "application/x-www-form-urlencoded")
                .POST(HttpRequest.BodyPublishers.ofString(encodeForm(form)))
                .build();

        final HttpResponse<String> response;
        try {
            response = http.send(request, HttpResponse.BodyHandlers.ofString());
        } catch (IOException e) {
            // Includes HttpTimeoutException: the hard timeout that bounds lock-hold time.
            throw new TransportException("token endpoint request failed", e);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new TransportException("interrupted during token refresh", e);
        }

        if (response.statusCode() < 200 || response.statusCode() >= 300) {
            throw new TokenEndpointException(response.statusCode(), response.body());
        }

        final JsonNode node;
        try {
            node = MAPPER.readTree(response.body());
        } catch (IOException e) {
            throw new TransportException("token endpoint returned unparseable JSON", e);
        }
        // A hostile or broken 2xx body must fail as a typed error, never a half-populated
        // token persisted to the store (an empty access_token or a non-positive expiry
        // would only resurface as a baffling 400 on the NEXT refresh, long after the cause).
        // Mirrors go/auth/oauth.go:74-79. Messages carry NO token/secret material — the raw
        // body is never echoed, since a partial 2xx payload may contain token material.
        JsonNode accessToken = node == null ? null : node.get("access_token");
        JsonNode expiresIn = node == null ? null : node.get("expires_in");
        if (accessToken == null || !accessToken.isTextual() || accessToken.asText().isEmpty()) {
            throw new TransportException(
                    "token endpoint 2xx response missing or empty access_token",
                    null);
        }
        // Reject 0, negative, AND non-numeric (e.g. "expires_in":"abc", where a bare
        // asLong() would silently coerce to 0 and be treated as immediately expired).
        if (expiresIn == null || !expiresIn.canConvertToLong() || expiresIn.asLong() <= 0) {
            throw new TransportException(
                    "token endpoint 2xx response missing or invalid expires_in",
                    null);
        }
        String rotatedRefresh = node.hasNonNull("refresh_token")
                ? node.get("refresh_token").asText()
                : current.getRefreshToken(); // server omitted rotation; keep the old one
        String scope = node.hasNonNull("scope") ? node.get("scope").asText() : current.getScope();
        String tokenType = node.hasNonNull("token_type")
                ? node.get("token_type").asText()
                : current.getTokenType();
        return new Tokens(
                accessToken.asText(),
                rotatedRefresh,
                Instant.now().getEpochSecond() + expiresIn.asLong(),
                scope,
                tokenType);
    }

    private static String encodeForm(Map<String, String> form) {
        StringJoiner joiner = new StringJoiner("&");
        for (Map.Entry<String, String> e : form.entrySet()) {
            joiner.add(URLEncoder.encode(e.getKey(), StandardCharsets.UTF_8)
                    + "="
                    + URLEncoder.encode(e.getValue(), StandardCharsets.UTF_8));
        }
        return joiner.toString();
    }

    // --- test seams (package-private; hermetic tests point at a loopback endpoint) ------

    void overrideTokenUrl(String url) {
        this.tokenUrl = url;
    }

    void overrideSkewSecs(long skewSecs) {
        this.skewSecs = skewSecs;
    }

    void overrideEndpointTimeout(Duration timeout) {
        this.endpointTimeout = timeout;
    }

    /** Never leaks tokens or the client secret. */
    @Override
    public String toString() {
        return "TokenManager{store=" + store.getDir()
                + ", credentials=" + credentials
                + ", tokens=" + tokens + "}";
    }
}
