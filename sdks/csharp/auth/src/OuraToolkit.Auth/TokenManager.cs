using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace OuraToolkit.Auth;

/// <summary>
/// Owns the current tokens and the machinery to keep them fresh: proactive refresh inside a
/// skew window before expiry, refresh-token ROTATION persistence (Oura invalidates the
/// previous refresh token on every refresh), and the cross-process reload/adopt/retry
/// protocol shared with the Rust companion.
///
/// This library is auth-plumbing only — no browser, no loopback listener, no interactive
/// consent (that is CLI territory). Wire it into the generated client's bearer seam:
/// <code>
/// var manager = TokenManager.Load();
/// var config = new OuraToolkit.Api.Client.Configuration
/// {
///     AccessToken = await manager.GetAccessTokenAsync(), // fresh: refreshed + persisted if needed
/// };
/// var sleep = new OuraToolkit.Api.Api.DailySleepRoutesApi(config);
/// </code>
/// Re-call <see cref="GetAccessTokenAsync"/> (and re-assign <c>Configuration.AccessToken</c>)
/// before each request batch so long-lived processes pick up refreshes; on a 401 from the
/// data plane, call <see cref="ForceRefreshAsync"/> and retry the request once.
/// </summary>
public sealed class TokenManager : IDisposable
{
    /// <summary>Refresh this many seconds before the token's actual expiry.</summary>
    public const long DefaultSkewSeconds = 60;

    /// <summary>
    /// Hard timeout on each token-endpoint call. Load-bearing: the refresh runs under the
    /// store's exclusive lock, so this bounds how long one process's stalled refresh can
    /// wedge every other process waiting on the lock (worst case ~2×: the 400-retry arm can
    /// chain a second endpoint call under the same lock).
    /// </summary>
    public static readonly TimeSpan TokenEndpointTimeout = TimeSpan.FromSeconds(30);

    /// <summary>
    /// The largest <c>expires_in</c> (seconds) a token response may carry: <see cref="int.MaxValue"/>
    /// (2147483647, ~68 years). Shared across every companion by the conformance fixture. It
    /// keeps <c>now + expires_in</c> exact (no overflow in the <c>expires_at</c> sum) and the
    /// resulting <c>expires_at</c> readable by every companion's store, including Rust's i64 and
    /// languages whose JSON numbers are doubles. Anything larger is rejected typed.
    /// </summary>
    private const long MaxExpiresInSeconds = int.MaxValue;

    private static readonly UTF8Encoding StrictUtf8 = new(encoderShouldEmitUTF8Identifier: false, throwOnInvalidBytes: true);
    private static readonly UTF8Encoding LenientUtf8 = new(encoderShouldEmitUTF8Identifier: false, throwOnInvalidBytes: false);

    private readonly TokenStore _store;
    private readonly ClientCredentials? _credentials;
    private readonly HttpClient _http;
    private readonly string _tokenUrl;
    private readonly long _skewSeconds;
    private readonly SemaphoreSlim _mutex = new(1, 1);
    private Tokens? _tokens;

    /// <summary>
    /// Load from the default token store. Absent records are not an error here —
    /// <see cref="GetAccessTokenAsync"/> reports <see cref="NotAuthenticatedException"/> on
    /// first use, so callers can surface their own "run oura auth login" UX.
    /// </summary>
    public static TokenManager Load()
    {
        var store = new TokenStore();
        return new TokenManager(store, store.LoadCredentials(), store.LoadTokens());
    }

    /// <summary>
    /// Construct from an explicit store + optional in-memory records. Both records are
    /// independently optional: credentials-without-tokens is "setup done, no login yet";
    /// tokens-without-credentials is a caller-supplied token, usable until expiry but not
    /// refreshable (<see cref="MissingClientCredentialsException"/>).
    /// </summary>
    /// <param name="store">The on-disk store refreshes reload from and persist to.</param>
    /// <param name="credentials">The user's own OAuth app credentials (null: refresh impossible).</param>
    /// <param name="tokens">The starting token set (null: not authenticated yet).</param>
    /// <param name="handler">
    /// Token-endpoint transport override (hermetic tests inject a mock
    /// <see cref="HttpMessageHandler"/>). Default: a plain handler — deliberately NOT an
    /// authenticated client, to avoid refresh recursion.
    /// </param>
    /// <param name="tokenUrl">
    /// Token-endpoint override (tests point at a mock). Default:
    /// <see cref="OAuthMetadata.TokenUrl"/>, the spec-pinned endpoint.
    /// </param>
    /// <param name="skewSeconds">Refresh this many seconds before actual expiry.</param>
    /// <param name="httpTimeout">
    /// Token-endpoint per-call timeout override (tests inject a short one to exercise the
    /// timeout path without a 30s wait). Default: <see cref="TokenEndpointTimeout"/>.
    /// </param>
    public TokenManager(
        TokenStore store,
        ClientCredentials? credentials,
        Tokens? tokens,
        HttpMessageHandler? handler = null,
        string? tokenUrl = null,
        long skewSeconds = DefaultSkewSeconds,
        TimeSpan? httpTimeout = null)
    {
        _store = store;
        _credentials = credentials;
        _tokens = tokens;
        _tokenUrl = tokenUrl ?? OAuthMetadata.TokenUrl;
        _skewSeconds = skewSeconds;
        // The default transport REFUSES redirects (AllowAutoRedirect = false): a confidential
        // client must never re-POST the token form (client_id + client_secret + refresh_token)
        // to a 3xx Location host — .NET's HttpClient default WOULD follow it and leak the
        // secret. A test may inject its own handler for hermetic mock responses.
        _http = handler is null
            ? new HttpClient(CreateRedirectRefusingHandler())
            : new HttpClient(handler);
        _http.Timeout = httpTimeout ?? TokenEndpointTimeout;
    }

    /// <summary>
    /// The default token-endpoint transport, refusing to auto-follow redirects (see the
    /// constructor comment). <c>SocketsHttpHandler</c> is netstandard2.1+, so the
    /// netstandard2.0 leg uses <c>HttpClientHandler</c>; both set
    /// <c>AllowAutoRedirect = false</c>. This is defense-in-depth — the framework-agnostic 3xx
    /// rejection in <see cref="RefreshAtAsync"/> is the actual backstop and catches a leak
    /// even if a handler is misconfigured.
    /// </summary>
    private static HttpMessageHandler CreateRedirectRefusingHandler() =>
#if NETSTANDARD2_0
        new HttpClientHandler { AllowAutoRedirect = false };
#else
        new SocketsHttpHandler { AllowAutoRedirect = false };
#endif

    /// <summary>
    /// Whether tokens are loaded (does not validate them, and does not imply a refresh is
    /// possible — refresh additionally needs the client-credentials record).
    /// </summary>
    public bool IsAuthenticated
    {
        get
        {
            _mutex.Wait();
            try
            {
                return _tokens is not null;
            }
            finally
            {
                _mutex.Release();
            }
        }
    }

    /// <summary>
    /// Return a valid access token, refreshing (and persisting the rotation) if it is
    /// expired or within the skew window.
    /// </summary>
    public async Task<string> GetAccessTokenAsync(CancellationToken cancellationToken = default)
    {
        await _mutex.WaitAsync(cancellationToken).ConfigureAwait(false);
        try
        {
            var current = _tokens ?? throw new NotAuthenticatedException();
            if (current.IsExpired(_skewSeconds))
            {
                await RefreshCriticalSectionAsync(cancellationToken).ConfigureAwait(false);
            }
            return _tokens!.AccessToken;
        }
        finally
        {
            _mutex.Release();
        }
    }

    /// <summary>
    /// Force a refresh regardless of expiry (call this when the data plane returns 401),
    /// persisting the rotation. If another process already rotated, its fresher tokens are
    /// adopted instead of burning that rotation with a second endpoint call.
    /// </summary>
    public async Task ForceRefreshAsync(CancellationToken cancellationToken = default)
    {
        await _mutex.WaitAsync(cancellationToken).ConfigureAwait(false);
        try
        {
            await RefreshCriticalSectionAsync(cancellationToken).ConfigureAwait(false);
        }
        finally
        {
            _mutex.Release();
        }
    }

    /// <summary>
    /// The reload → refresh → persist critical section, run under the store's exclusive
    /// lock so only one coordinated process rotates at a time. Caller holds
    /// <see cref="_mutex"/>.
    ///
    /// The adopt rule covers both entry points: if disk holds tokens that differ from
    /// memory and are not expired, another process already rotated — adopt them instead of
    /// re-burning the rotation. A refresh 400 (usually "our refresh token is stale") is
    /// retried ONCE against freshly reloaded disk state, which absorbs rotations by writers
    /// not honoring our lock — this protocol, not the lock's cross-runtime semantics, is
    /// the interop guarantee with the Rust CLI (see <see cref="TokenStore"/>).
    /// </summary>
    private async Task RefreshCriticalSectionAsync(CancellationToken cancellationToken)
    {
        var credentials = _credentials ?? throw new MissingClientCredentialsException();

        using var storeLock = await _store.AcquireLockAsync(cancellationToken).ConfigureAwait(false);

        if (_store.LoadTokens() is { } disk)
        {
            var differs = _tokens?.AccessToken != disk.AccessToken;
            if (differs && !disk.IsExpired(_skewSeconds))
            {
                _tokens = disk;
                return;
            }
            // Refresh from the freshest persisted rotation, never from stale memory.
            _tokens = disk;
        }
        var current = _tokens ?? throw new NotAuthenticatedException();

        Tokens refreshed;
        try
        {
            refreshed = await RefreshAtAsync(credentials, current, cancellationToken).ConfigureAwait(false);
        }
        catch (TokenEndpointException e) when (e.StatusCode == 400)
        {
            // If disk moved past what we sent (a rotation by an uncoordinated writer),
            // retry once with the fresher token before surfacing "re-login".
            if (_store.LoadTokens() is { } fresher && fresher.RefreshToken != current.RefreshToken)
            {
                refreshed = await RefreshAtAsync(credentials, fresher, cancellationToken).ConfigureAwait(false);
            }
            else
            {
                throw;
            }
        }
        _store.SaveTokens(refreshed);
        _tokens = refreshed;
    }

    private async Task<Tokens> RefreshAtAsync(
        ClientCredentials credentials,
        Tokens current,
        CancellationToken cancellationToken)
    {
        // Confidential client: the token endpoint requires client_id AND client_secret in
        // the form body (never in the URL — no secrets in query strings).
        using var content = new FormUrlEncodedContent(new Dictionary<string, string>
        {
            ["grant_type"] = "refresh_token",
            ["refresh_token"] = current.RefreshToken,
            ["client_id"] = credentials.ClientId,
            ["client_secret"] = credentials.ClientSecret,
        });

        HttpResponseMessage response;
        try
        {
            response = await _http.PostAsync(_tokenUrl, content, cancellationToken).ConfigureAwait(false);
        }
        catch (TaskCanceledException e) when (!cancellationToken.IsCancellationRequested)
        {
            // The hard token-endpoint timeout elapsed (it bounds lock-hold time). Surface a
            // typed transport error, not a bare TaskCanceledException. A caller-requested
            // cancellation is excluded by the filter and propagates as OperationCanceledException.
            throw new TransportException("token endpoint request timed out", e);
        }
        catch (HttpRequestException e)
        {
            throw new TransportException("token endpoint request failed", e);
        }

        using (response)
        {
            var status = (int)response.StatusCode;
            // Read the RAW BYTES, not ReadAsStringAsync: that decodes leniently, silently
            // substituting U+FFFD for invalid UTF-8, which would let a malformed 2xx body pass as
            // JSON (see the strict decode below). The CancellationToken overload is net5+;
            // netstandard2.0 has only the parameterless form. Bounded either way by _http.Timeout.
#if NETSTANDARD2_0
            var bytes = await response.Content.ReadAsByteArrayAsync().ConfigureAwait(false);
#else
            var bytes = await response.Content.ReadAsByteArrayAsync(cancellationToken).ConfigureAwait(false);
#endif

            // Defense-in-depth against a redirect leaking the confidential form: the default
            // transport already refuses to follow redirects, so a 3xx from the token endpoint
            // surfaces here rather than being followed. Reject it explicitly — a 2xx is the ONLY
            // success — so even an injected/misconfigured handler that leaves a bare 3xx in place
            // cannot slip past as a silent no-op.
            if (status is >= 300 and < 400)
            {
                throw new TokenEndpointException(status, LenientUtf8.GetString(bytes));
            }
            if (!response.IsSuccessStatusCode)
            {
                // A non-2xx body is diagnostics only (never parsed or persisted), so it is decoded
                // leniently: a stray invalid byte there must not mask the real HTTP error.
                throw new TokenEndpointException(status, LenientUtf8.GetString(bytes));
            }

            // A hostile or broken 2xx body must fail as the typed TokenEndpointException, never a
            // raw JsonException detonating downstream and never a half-populated token persisted
            // (an empty access_token or a non-positive expiry would only resurface as a baffling
            // 400 on the NEXT refresh, long after the cause). These throws all run BEFORE the
            // store is written, so a hostile 2xx never burns the stored rotation. Messages are
            // FIXED and secret-free: the raw body is never echoed, since a partial 2xx payload may
            // carry token material. Mirrors the Go companion's refreshTokens (sdks/go/auth/oauth.go).
            string body;
            try
            {
                // STRICT UTF-8 over the WHOLE body, unknown fields included: bytes that are not
                // UTF-8 are not JSON text at all (RFC 8259 §8.1). This must run on the raw bytes —
                // System.Text.Json does not validate the contents of properties it skips, so an
                // invalid byte inside an unknown field would otherwise slip through (shared
                // fixture cases body_invalid_utf8_in_scope / body_invalid_utf8_in_unknown_field).
                // A leading UTF-8 BOM is skipped (RFC 8259 §8.1 lets parsers ignore it), matching
                // what the lenient ReadAsStringAsync used to do.
                var start = bytes.Length >= 3 && bytes[0] == 0xEF && bytes[1] == 0xBB && bytes[2] == 0xBF ? 3 : 0;
                body = StrictUtf8.GetString(bytes, start, bytes.Length - start);
            }
            catch (DecoderFallbackException)
            {
                throw new TokenEndpointException(status, "token-endpoint 2xx response was not valid UTF-8");
            }

            TokenResponse? parsed;
            try
            {
                parsed = JsonSerializer.Deserialize<TokenResponse>(body);
            }
            catch (JsonException)
            {
                // Covers both a body that is not JSON at all AND a well-formed body whose
                // strictly-typed string field (access_token, refresh_token, token_type) holds a
                // non-string value or a lone-surrogate escape — System.Text.Json reports both as
                // a JsonException (shared fixture cases body_not_json, body_json_array,
                // wrong_type_*, *_lone_surrogate). Unknown fields are never materialized. The
                // message stays fixed: the server's keys/values are never echoed.
                throw new TokenEndpointException(status, "token-endpoint 2xx response was not a well-formed token response");
            }
            if (parsed is null)
            {
                // The body was the literal JSON null.
                throw new TokenEndpointException(status, "token-endpoint 2xx response was empty");
            }
            if (string.IsNullOrEmpty(parsed.AccessToken))
            {
                throw new TokenEndpointException(status, "token-endpoint 2xx response missing access_token");
            }
            if (parsed.ValidExpiresIn() is not { } expiresIn)
            {
                throw new TokenEndpointException(status, "token-endpoint 2xx response missing or invalid expires_in");
            }
            // A scope STRING that is not valid Unicode (a lone-surrogate escape like "\ud800")
            // makes the whole response malformed: fail typed with the store untouched (shared
            // fixture case scope_lone_surrogate). Distinct from a well-formed but wrong-typed
            // scope, which ScopeString reads as absent so the prior grant is kept.
            string? scope;
            try
            {
                scope = parsed.ScopeString();
            }
            catch (InvalidOperationException)
            {
                throw new TokenEndpointException(status, "token-endpoint 2xx response scope is not valid Unicode");
            }

            return new Tokens
            {
                // Non-null: guarded by the IsNullOrEmpty check above. The `!` is for the
                // netstandard2.0 BCL, whose string.IsNullOrEmpty lacks the [NotNullWhen(false)]
                // flow annotation that net8/net10 carry (there it is simply redundant).
                AccessToken = parsed.AccessToken!,
                // Persist the ROTATED refresh token; treat null OR empty like a missing one and
                // keep the current (still-valid) token. An empty refresh_token would clobber the
                // good one and 400 every future refresh.
                RefreshToken = string.IsNullOrEmpty(parsed.RefreshToken) ? current.RefreshToken : parsed.RefreshToken!,
                // Cannot overflow: expiresIn <= MaxExpiresInSeconds (see ValidExpiresIn).
                ExpiresAt = DateTimeOffset.UtcNow.ToUnixTimeSeconds() + expiresIn,
                // An omitted, null, empty, whitespace-only (incl. U+00A0), or non-string scope
                // keeps the prior grant (RFC 6749 §5.1 lets the server omit an unchanged scope);
                // persisting a blank would erase the grant the CLI's re-consent check reads, and
                // failing on a wrong-typed scope would burn the rotated refresh token over
                // informational junk. Pinned by the shared fixture's refresh_success_cases
                // (ConformanceTests).
                Scope = scope ?? current.Scope,
                // Same rule as refresh_token: an omitted, null or EMPTY token_type keeps the
                // current value (shared fixture cases token_type_null / token_type_empty).
                TokenType = string.IsNullOrEmpty(parsed.TokenType) ? current.TokenType : parsed.TokenType,
            };
        }
    }

    /// <summary>Releases the token-endpoint HTTP client and the internal mutex.</summary>
    public void Dispose()
    {
        _http.Dispose();
        _mutex.Dispose();
    }

    /// <summary>Raw token-endpoint response shape.</summary>
    private sealed record TokenResponse
    {
        [JsonPropertyName("access_token")]
        public string? AccessToken { get; init; }

        [JsonPropertyName("refresh_token")]
        public string? RefreshToken { get; init; }

        /// <summary>
        /// Deliberately untyped, so the integer/range rule lives in ONE explicit place
        /// (<see cref="ValidExpiresIn"/>) rather than in the serializer's number converter.
        /// </summary>
        [JsonPropertyName("expires_in")]
        public JsonElement? ExpiresIn { get; init; }

        [JsonPropertyName("token_type")]
        public string? TokenType { get; init; }

        /// <summary>
        /// Deliberately untyped: <c>scope</c> is informational, so a non-string value (e.g. a
        /// number) must NOT fail deserialization and burn the rotated refresh token. Every
        /// other field keeps its strict type. Read it only via <see cref="ScopeString"/>.
        /// </summary>
        [JsonPropertyName("scope")]
        public JsonElement? Scope { get; init; }

        /// <summary>
        /// <c>expires_in</c> when it is a JSON integer in 1..=<see cref="MaxExpiresInSeconds"/>;
        /// otherwise null (missing, null, zero/negative, fractional like <c>3600.5</c>, a numeric
        /// string like <c>"3600"</c>, above the cap, or beyond any machine integer like
        /// <c>1e400</c>) — the caller fails the refresh typed. <see cref="JsonElement.TryGetInt64"/>
        /// rejects any non-integral or out-of-range number.
        /// </summary>
        public long? ValidExpiresIn()
        {
            if (ExpiresIn is { ValueKind: JsonValueKind.Number } element
                && element.TryGetInt64(out var seconds)
                && seconds is >= 1 and <= MaxExpiresInSeconds)
            {
                return seconds;
            }
            return null;
        }

        /// <summary>
        /// The scope when it is a JSON string with non-whitespace content (U+00A0 counts as
        /// whitespace); otherwise null, meaning "keep the prior grant" (this covers a
        /// well-formed but non-string scope). Throws <see cref="InvalidOperationException"/>
        /// (from <see cref="JsonElement.GetString"/>) when the string is not valid UTF-16 (a
        /// lone surrogate escape like <c>"\ud800"</c>): that makes the response malformed, and
        /// the refresh path maps it to the typed <see cref="TokenEndpointException"/> before
        /// anything is persisted (shared fixture case <c>scope_lone_surrogate</c>).
        /// </summary>
        public string? ScopeString()
        {
            if (Scope is not { ValueKind: JsonValueKind.String } element)
            {
                return null;
            }
            var value = element.GetString();
            return string.IsNullOrWhiteSpace(value) ? null : value;
        }

        /// <summary><see cref="ScopeString"/> for diagnostics only: never throws.</summary>
        private string DescribeScope()
        {
            try
            {
                return ScopeString() ?? "null";
            }
            catch (InvalidOperationException)
            {
                return "[invalid Unicode]";
            }
        }

        /// <summary>
        /// Redacts both token fields (parity with <see cref="Tokens"/> / <see cref="ClientCredentials"/>):
        /// the synthesized record ToString would otherwise print the raw access/refresh tokens if
        /// this value ever reached a log line.
        /// </summary>
        public override string ToString() =>
            "TokenResponse { access_token = [REDACTED], refresh_token = [REDACTED], " +
            $"expires_in = {ValidExpiresIn()?.ToString() ?? "invalid"}, token_type = {TokenType ?? "null"}, scope = {DescribeScope()} }}";
    }
}
