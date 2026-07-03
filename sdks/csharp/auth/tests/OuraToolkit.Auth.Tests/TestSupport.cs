using System.Net;

namespace OuraToolkit.Auth.Tests;

/// <summary>A tempdir-backed store, deleted on dispose.</summary>
public sealed class TempStore : IDisposable
{
    public TempStore()
    {
        Dir = System.IO.Directory.CreateTempSubdirectory("oura-auth-test-").FullName;
        Store = new TokenStore(Dir);
    }

    public string Dir { get; }

    public TokenStore Store { get; }

    public void Dispose() => System.IO.Directory.Delete(Dir, recursive: true);
}

/// <summary>
/// Injected token-endpoint transport (hermetic — no sockets). The responder receives the
/// decoded form body; the call counter is thread-safe so concurrency tests can assert
/// exact endpoint call counts.
/// </summary>
public sealed class MockTokenEndpoint : HttpMessageHandler
{
    private readonly Func<string, HttpResponseMessage> _respond;
    private int _calls;

    public MockTokenEndpoint(Func<string, HttpResponseMessage> respond) => _respond = respond;

    public int Calls => Volatile.Read(ref _calls);

    /// <summary>Every request body seen, in order (thread-safe).</summary>
    public List<string> Bodies { get; } = new();

    protected override async Task<HttpResponseMessage> SendAsync(
        HttpRequestMessage request, CancellationToken cancellationToken)
    {
        Interlocked.Increment(ref _calls);
        var body = request.Content is null
            ? ""
            : await request.Content.ReadAsStringAsync(cancellationToken);
        lock (Bodies)
        {
            Bodies.Add(body);
        }
        return _respond(body);
    }

    public static HttpResponseMessage Json(HttpStatusCode status, string json) => new(status)
    {
        Content = new StringContent(json, System.Text.Encoding.UTF8, "application/json"),
    };

    public static HttpResponseMessage TokenGrant(string accessToken, string refreshToken, long expiresIn = 3600) =>
        Json(HttpStatusCode.OK,
            $"{{\"access_token\":\"{accessToken}\",\"refresh_token\":\"{refreshToken}\",\"expires_in\":{expiresIn}}}");
}

public static class Fixtures
{
    public static ClientCredentials Credentials { get; } = new()
    {
        ClientId = "cid",
        ClientSecret = "SECRET-CS-789",
    };

    public static Tokens SampleTokens { get; } = new()
    {
        AccessToken = "SECRET-AT-123",
        RefreshToken = "SECRET-RT-456",
        ExpiresAt = 4_102_444_800, // 2100-01-01
        Scope = "daily personal",
        TokenType = "Bearer",
    };

    /// <summary>An expired token set whose access token is derived from the refresh token.</summary>
    public static Tokens Expired(string refreshToken) => new()
    {
        AccessToken = $"stale-access-{refreshToken}",
        RefreshToken = refreshToken,
        ExpiresAt = 0,
    };
}
