using System.Net;
using Xunit;

namespace OuraToolkit.Auth.Tests;

public class TokenManagerTests
{
    private static TokenManager Manager(
        TempStore temp, MockTokenEndpoint endpoint, Tokens? tokens, ClientCredentials? credentials = null) =>
        new(temp.Store, credentials ?? Fixtures.Credentials, tokens,
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

    private static MockTokenEndpoint GrantOnly(string expectedRefreshToken, string accessToken, string rotatedTo) =>
        new(body => body.Contains($"refresh_token={expectedRefreshToken}")
            ? MockTokenEndpoint.TokenGrant(accessToken, rotatedTo)
            : MockTokenEndpoint.Json(HttpStatusCode.BadRequest, "\"unexpected refresh token\""));

    [Fact]
    public async Task GetAccessTokenRequiresAuthentication()
    {
        using var temp = new TempStore();
        var endpoint = new MockTokenEndpoint(_ => throw new InvalidOperationException("no calls expected"));
        using var manager = Manager(temp, endpoint, tokens: null);

        Assert.False(manager.IsAuthenticated);
        await Assert.ThrowsAsync<NotAuthenticatedException>(() => manager.GetAccessTokenAsync());
        Assert.Equal(0, endpoint.Calls);
    }

    [Fact]
    public async Task RefreshWithoutCredentialsReportsMissingCredentials()
    {
        using var temp = new TempStore();
        var endpoint = new MockTokenEndpoint(_ => throw new InvalidOperationException("no calls expected"));
        using var manager = new TokenManager(temp.Store, credentials: null, Fixtures.Expired("r1"),
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

        await Assert.ThrowsAsync<MissingClientCredentialsException>(() => manager.GetAccessTokenAsync());
    }

    [Fact]
    public async Task FreshTokenIsReturnedWithoutAnyEndpointCall()
    {
        using var temp = new TempStore();
        var endpoint = new MockTokenEndpoint(_ => throw new InvalidOperationException("no calls expected"));
        var fresh = Fixtures.Expired("r1") with
        {
            AccessToken = "still-good",
            ExpiresAt = DateTimeOffset.UtcNow.ToUnixTimeSeconds() + 3600,
        };
        using var manager = Manager(temp, endpoint, fresh);

        Assert.Equal("still-good", await manager.GetAccessTokenAsync());
        Assert.Equal(0, endpoint.Calls);
    }

    /// <summary>
    /// THE rotation-persistence guarantee: Oura invalidates the previous refresh token on
    /// every refresh, so the newly returned one must land on disk — and the refresh must be
    /// a confidential-client call carrying client_id + client_secret in the form body.
    /// </summary>
    [Fact]
    public async Task RefreshPersistsTheRotatedRefreshToken()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));
        var endpoint = GrantOnly("r1", accessToken: "fresh-access", rotatedTo: "r2");
        using var manager = Manager(temp, endpoint, Fixtures.Expired("r1"));

        Assert.Equal("fresh-access", await manager.GetAccessTokenAsync());

        var persisted = temp.Store.LoadTokens();
        Assert.NotNull(persisted);
        Assert.Equal("r2", persisted!.RefreshToken); // rotation persisted, old one gone
        Assert.Equal("fresh-access", persisted.AccessToken);
        Assert.True(persisted.ExpiresAt > DateTimeOffset.UtcNow.ToUnixTimeSeconds() + 3000);

        var body = Assert.Single(endpoint.Bodies);
        Assert.Contains("grant_type=refresh_token", body);
        Assert.Contains("client_id=cid", body);
        Assert.Contains("client_secret=SECRET-CS-789", body);
    }

    /// <summary>
    /// Skew half 1: a token that is still VALID but inside the skew window must refresh
    /// proactively (requests should never go out with a token about to die mid-flight).
    /// </summary>
    [Fact]
    public async Task TokenInsideTheSkewWindowRefreshesProactively()
    {
        using var temp = new TempStore();
        var expiring = Fixtures.Expired("r1") with
        {
            ExpiresAt = DateTimeOffset.UtcNow.ToUnixTimeSeconds() + 30, // < 60s default skew
        };
        temp.Store.SaveTokens(expiring);
        var endpoint = GrantOnly("r1", accessToken: "fresh-access", rotatedTo: "r2");
        using var manager = Manager(temp, endpoint, expiring);

        Assert.Equal("fresh-access", await manager.GetAccessTokenAsync());
        Assert.Equal(1, endpoint.Calls);
    }

    /// <summary>Skew half 2: with no skew, the same 30s-out token is NOT refreshed.</summary>
    [Fact]
    public async Task TokenOutsideTheSkewWindowIsNotRefreshed()
    {
        using var temp = new TempStore();
        var endpoint = new MockTokenEndpoint(_ => throw new InvalidOperationException("no calls expected"));
        var expiring = Fixtures.Expired("r1") with
        {
            AccessToken = "still-good",
            ExpiresAt = DateTimeOffset.UtcNow.ToUnixTimeSeconds() + 30,
        };
        using var manager = new TokenManager(temp.Store, Fixtures.Credentials, expiring,
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token", skewSeconds: 0);

        Assert.Equal("still-good", await manager.GetAccessTokenAsync());
        Assert.Equal(0, endpoint.Calls);
    }

    /// <summary>
    /// The reload/adopt rule (cross-process rotation safety): B, holding the same stale
    /// state A started from, must ADOPT A's persisted rotation from disk instead of
    /// replaying the invalidated r1 or burning r2 — exactly ONE endpoint call total.
    /// </summary>
    [Fact]
    public async Task SecondManagerAdoptsRotationFromDiskWithoutCallingTheEndpoint()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));
        var endpoint = GrantOnly("r1", accessToken: "fresh-access", rotatedTo: "r2");

        using var a = Manager(temp, endpoint, Fixtures.Expired("r1"));
        using var b = Manager(temp, endpoint, Fixtures.Expired("r1"));

        Assert.Equal("fresh-access", await a.GetAccessTokenAsync()); // burns r1, persists r2
        Assert.Equal("fresh-access", await b.GetAccessTokenAsync()); // adopts, no 2nd call

        Assert.Equal(1, endpoint.Calls);
        Assert.Equal("r2", temp.Store.LoadTokens()!.RefreshToken);
    }

    /// <summary>Same adopt rule on the force path (data-plane 401 handler).</summary>
    [Fact]
    public async Task ForceRefreshAdoptsFresherDiskState()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));
        var endpoint = GrantOnly("r1", accessToken: "fresh-access", rotatedTo: "r2");

        using var a = Manager(temp, endpoint, Fixtures.Expired("r1"));
        using var b = Manager(temp, endpoint, Fixtures.Expired("r1"));

        Assert.Equal("fresh-access", await a.GetAccessTokenAsync());

        await b.ForceRefreshAsync(); // B's request 401'd; must adopt, not burn r2
        Assert.Equal("fresh-access", await b.GetAccessTokenAsync());
        Assert.Equal(1, endpoint.Calls);
    }

    /// <summary>
    /// The lock's reason to exist, with REAL concurrency: two managers refreshing at the
    /// same time must serialize to exactly one endpoint call — the loser adopts the
    /// winner's persisted rotation. (A sequential version would pass with a no-op lock.)
    /// </summary>
    [Fact]
    public async Task ConcurrentRefreshesSerializeToASingleEndpointCall()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));
        var endpoint = GrantOnly("r1", accessToken: "fresh-access", rotatedTo: "r2");

        using var a = Manager(temp, endpoint, Fixtures.Expired("r1"));
        using var b = Manager(temp, endpoint, Fixtures.Expired("r1"));

        var results = await Task.WhenAll(
            Task.Run(() => a.GetAccessTokenAsync()),
            Task.Run(() => b.GetAccessTokenAsync()));

        Assert.All(results, r => Assert.Equal("fresh-access", r));
        Assert.Equal(1, endpoint.Calls); // a double-refresh would replay r1 or burn r2
        Assert.Equal("r2", temp.Store.LoadTokens()!.RefreshToken);
    }

    /// <summary>
    /// The 400-retry arm: an UNCOORDINATED writer (e.g. a different runtime whose lock does
    /// not interop with ours — the documented cross-runtime reality) rotates to r2 while
    /// our r1 request is in flight. The endpoint 400s r1; the manager must reload, see the
    /// fresher refresh token, and retry exactly once with it.
    /// </summary>
    [Fact]
    public async Task Refresh400ReloadsAndRetriesOnceAgainstFresherDiskState()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));

        var endpoint = new MockTokenEndpoint(body =>
        {
            if (body.Contains("refresh_token=r1"))
            {
                // Simulate the uncoordinated rotation landing mid-flight, then reject r1.
                temp.Store.SaveTokens(Fixtures.Expired("r2"));
                return MockTokenEndpoint.Json(HttpStatusCode.BadRequest, "\"invalid_grant\"");
            }
            if (body.Contains("refresh_token=r2"))
            {
                return MockTokenEndpoint.TokenGrant("r3-access", "r3");
            }
            return MockTokenEndpoint.Json(HttpStatusCode.BadRequest, "\"unexpected refresh token\"");
        });
        using var manager = Manager(temp, endpoint, Fixtures.Expired("r1"));

        Assert.Equal("r3-access", await manager.GetAccessTokenAsync());
        Assert.Equal(2, endpoint.Calls); // r1 (400) + r2 (retry), nothing more
        Assert.Equal("r3", temp.Store.LoadTokens()!.RefreshToken);
    }

    /// <summary>
    /// A genuinely dead refresh token must surface the 400 — no blind retry with the SAME
    /// token (disk has not moved), so callers can prompt for re-login.
    /// </summary>
    [Fact]
    public async Task GenuinelyInvalidRefreshTokenSurfacesThe400WithoutBlindRetry()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r-dead"));
        var endpoint = new MockTokenEndpoint(
            _ => MockTokenEndpoint.Json(HttpStatusCode.BadRequest, "\"invalid_grant\""));
        using var manager = Manager(temp, endpoint, Fixtures.Expired("r-dead"));

        var e = await Assert.ThrowsAsync<TokenEndpointException>(() => manager.GetAccessTokenAsync());
        Assert.Equal(400, e.StatusCode);
        Assert.Contains("invalid_grant", e.Body);
        Assert.Equal(1, endpoint.Calls);
    }

    /// <summary>
    /// Refresh must start from the freshest persisted rotation, not stale memory — even
    /// when disk is also expired (so the adopt short-circuit does not apply).
    /// </summary>
    [Fact]
    public async Task RefreshUsesDiskStateNotStaleMemory()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r2")); // a newer (but expired) rotation on disk
        var endpoint = GrantOnly("r2", accessToken: "fresh-access", rotatedTo: "r3");
        using var manager = Manager(temp, endpoint, Fixtures.Expired("r1")); // stale memory

        Assert.Equal("fresh-access", await manager.GetAccessTokenAsync());
        Assert.Equal(1, endpoint.Calls); // sent r2, never the stale r1
        Assert.Equal("r3", temp.Store.LoadTokens()!.RefreshToken);
    }
}
