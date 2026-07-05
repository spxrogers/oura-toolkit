using System.Net;
using System.Net.Sockets;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// A real loopback HTTP endpoint on 127.0.0.1 (a genuine socket, NOT a mock
/// <see cref="HttpMessageHandler"/>) so tests can exercise the production default transport —
/// the ONLY place redirect-following and connect/read timeouts actually happen. That transport
/// is <c>SocketsHttpHandler</c> on the modern legs and <c>HttpClientHandler</c> on the
/// netstandard2.0/net472 (Mono) leg; both refuse redirects, and this server lets the redirect
/// tests prove it on whichever one the runtime selected. Every received request is counted
/// (<see cref="Hits"/>); the handler decides the response.
/// </summary>
public sealed class LoopbackHttpServer : IDisposable
{
    private readonly HttpListener _listener = new();
    private int _hits;

    public LoopbackHttpServer(Func<HttpListenerContext, Task> handle)
    {
        Url = $"http://127.0.0.1:{FreePort()}/";
        _listener.Prefixes.Add(Url);
        _listener.Start();
        _ = Task.Run(async () =>
        {
            while (_listener.IsListening)
            {
                HttpListenerContext ctx;
                try
                {
                    ctx = await _listener.GetContextAsync();
                }
                catch
                {
                    break; // listener stopped/disposed
                }
                Interlocked.Increment(ref _hits);
                try
                {
                    await handle(ctx);
                }
                catch
                {
                    // A handler fault must not tear down the accept loop or the test process.
                }
                try
                {
                    ctx.Response.Close();
                }
                catch
                {
                    // Response may already be closed.
                }
            }
        });
    }

    /// <summary>The absolute base URL clients POST to (e.g. <c>http://127.0.0.1:PORT/</c>).</summary>
    public string Url { get; }

    /// <summary>Total requests this server has received (thread-safe).</summary>
    public int Hits => Volatile.Read(ref _hits);

    private static int FreePort()
    {
        var probe = new TcpListener(IPAddress.Loopback, 0);
        probe.Start();
        var port = ((IPEndPoint)probe.LocalEndpoint).Port;
        probe.Stop();
        return port;
    }

    public void Dispose()
    {
        try
        {
            _listener.Stop();
            _listener.Close();
        }
        catch
        {
            // Best-effort teardown.
        }
    }
}

/// <summary>A tempdir-backed store, deleted on dispose.</summary>
public sealed class TempStore : IDisposable
{
    public TempStore()
    {
        Dir = TestHost.CreateTempDir("oura-auth-test-");
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
#if NET472
            // The CancellationToken overload is net5+; net472 has only the parameterless read.
            : await request.Content.ReadAsStringAsync();
#else
            : await request.Content.ReadAsStringAsync(cancellationToken);
#endif
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
