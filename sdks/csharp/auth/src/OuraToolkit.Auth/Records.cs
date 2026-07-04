using System.Text.Json.Serialization;

namespace OuraToolkit.Auth;

/// <summary>
/// The user's own Oura OAuth application credentials (bring-your-own confidential client;
/// there is no shared/default app and no PKCE/public-client path).
///
/// The on-disk JSON field names (snake_case) are the cross-language store contract shared
/// with the Rust companion's <c>credentials.json</c> — pinned by the store-schema test.
///
/// <see cref="ToString"/> REDACTS the secret, so stray logging/interpolation can never leak
/// it (the "no secrets in logs" rule).
/// </summary>
public sealed record ClientCredentials
{
    /// <summary>The OAuth application's client id.</summary>
    [JsonPropertyName("client_id")]
    public required string ClientId { get; init; }

    /// <summary>The OAuth application's client secret (never logged; see <see cref="ToString"/>).</summary>
    [JsonPropertyName("client_secret")]
    public required string ClientSecret { get; init; }

    /// <summary>Redacts the client secret.</summary>
    public override string ToString() =>
        $"ClientCredentials {{ ClientId = {ClientId}, ClientSecret = [REDACTED] }}";
}

/// <summary>
/// The persisted OAuth token set. Client credentials live in their own record
/// (<see cref="ClientCredentials"/>), not here.
///
/// The on-disk JSON field names (snake_case; <c>scope</c>/<c>token_type</c> omitted when
/// null) are the cross-language store contract shared with the Rust companion's
/// <c>tokens.json</c> — pinned by the store-schema test.
///
/// <see cref="ToString"/> REDACTS both token fields.
/// </summary>
public sealed record Tokens
{
    /// <summary>The Bearer token the data plane authenticates with.</summary>
    [JsonPropertyName("access_token")]
    public required string AccessToken { get; init; }

    /// <summary>
    /// Oura rotates this on every refresh and invalidates the previous value — always
    /// persist the newly returned one or the next refresh 400s.
    /// </summary>
    [JsonPropertyName("refresh_token")]
    public required string RefreshToken { get; init; }

    /// <summary>Absolute expiry as a Unix timestamp (seconds).</summary>
    [JsonPropertyName("expires_at")]
    public required long ExpiresAt { get; init; }

    /// <summary>Space-separated granted scopes, when the endpoint reported them.</summary>
    [JsonPropertyName("scope")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? Scope { get; init; }

    /// <summary>Token type as reported by the endpoint (normally "Bearer").</summary>
    [JsonPropertyName("token_type")]
    [JsonIgnore(Condition = JsonIgnoreCondition.WhenWritingNull)]
    public string? TokenType { get; init; }

    /// <summary>
    /// True if the access token is expired, or within <paramref name="skewSeconds"/> of
    /// expiring (proactive-refresh window).
    /// </summary>
    public bool IsExpired(long skewSeconds) =>
        DateTimeOffset.UtcNow.ToUnixTimeSeconds() + skewSeconds >= ExpiresAt;

    /// <summary>Redacts both tokens; the non-secret fields stay visible for diagnostics.</summary>
    public override string ToString() =>
        $"Tokens {{ AccessToken = [REDACTED], RefreshToken = [REDACTED], ExpiresAt = {ExpiresAt}, " +
        $"Scope = {Scope ?? "null"}, TokenType = {TokenType ?? "null"} }}";
}
