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
    public TokenManager(
        TokenStore store,
        ClientCredentials? credentials,
        Tokens? tokens,
        HttpMessageHandler? handler = null,
        string? tokenUrl = null,
        long skewSeconds = DefaultSkewSeconds)
    {
        _store = store;
        _credentials = credentials;
        _tokens = tokens;
        _tokenUrl = tokenUrl ?? OAuthMetadata.TokenUrl;
        _skewSeconds = skewSeconds;
        _http = handler is null ? new HttpClient() : new HttpClient(handler);
        _http.Timeout = TokenEndpointTimeout;
    }

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
        using var response = await _http.PostAsync(_tokenUrl, content, cancellationToken).ConfigureAwait(false);
        var body = await response.Content.ReadAsStringAsync(cancellationToken).ConfigureAwait(false);
        if (!response.IsSuccessStatusCode)
        {
            throw new TokenEndpointException((int)response.StatusCode, body);
        }
        var parsed = JsonSerializer.Deserialize<TokenResponse>(body)
            ?? throw new TokenEndpointException((int)response.StatusCode, "empty token response");
        return new Tokens
        {
            AccessToken = parsed.AccessToken
                ?? throw new TokenEndpointException((int)response.StatusCode, "token response without access_token"),
            // Persist the ROTATED token; fall back to the old one only if the server omits it.
            RefreshToken = parsed.RefreshToken ?? current.RefreshToken,
            ExpiresAt = DateTimeOffset.UtcNow.ToUnixTimeSeconds() + parsed.ExpiresIn,
            Scope = parsed.Scope ?? current.Scope,
            TokenType = parsed.TokenType ?? current.TokenType,
        };
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

        [JsonPropertyName("expires_in")]
        public long ExpiresIn { get; init; }

        [JsonPropertyName("token_type")]
        public string? TokenType { get; init; }

        [JsonPropertyName("scope")]
        public string? Scope { get; init; }
    }
}
