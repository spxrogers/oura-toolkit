using System.Diagnostics;
using System.Net;
using System.Text;
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

    // -- Hostile 2xx: a malformed "success" must fail typed and NEVER persist a half-token ----

    /// <summary>
    /// A 2xx whose body is non-JSON / empty / <c>null</c> / <c>{}</c> / missing-or-empty
    /// access_token / missing/zero/negative/non-numeric expires_in must surface the typed
    /// <see cref="TokenEndpointException"/> — not a raw JsonException, not a persisted blank
    /// Bearer or zero-expiry token (which would only resurface as a baffling 400 on the NEXT
    /// refresh). It is NOT a 400, so the reload-retry arm must not fire: exactly one call, and
    /// the store is left exactly as it was.
    /// </summary>
    [Theory]
    [InlineData("this is not json")]
    [InlineData("")]
    [InlineData("null")]
    [InlineData("{}")]
    [InlineData("{\"expires_in\":3600}")] // missing access_token
    [InlineData("{\"access_token\":\"\",\"expires_in\":3600}")] // empty access_token
    [InlineData("{\"access_token\":\"a\"}")] // missing expires_in
    [InlineData("{\"access_token\":\"a\",\"expires_in\":0}")] // zero expires_in
    [InlineData("{\"access_token\":\"a\",\"expires_in\":-5}")] // negative expires_in
    [InlineData("{\"access_token\":\"a\",\"expires_in\":\"abc\"}")] // non-numeric expires_in
    public async Task Hostile2xxIsRejectedTypedLeavingTheStoreUntouched(string body)
    {
        using var temp = new TempStore();
        var original = Fixtures.Expired("r1");
        temp.Store.SaveTokens(original);
        var endpoint = new MockTokenEndpoint(_ => MockTokenEndpoint.Json(HttpStatusCode.OK, body));
        using var manager = Manager(temp, endpoint, Fixtures.Expired("r1"));

        var e = await Assert.ThrowsAsync<TokenEndpointException>(() => manager.GetAccessTokenAsync());
        Assert.Equal(200, e.StatusCode); // a 2xx, so the 400-retry arm cannot claim it
        // The fixed diagnostic must never echo token material.
        Assert.DoesNotContain("access_token\":\"a", e.Body);

        Assert.Equal(1, endpoint.Calls); // a hostile 2xx must NOT trigger the reload-retry arm

        var disk = temp.Store.LoadTokens();
        Assert.NotNull(disk);
        Assert.Equal("r1", disk!.RefreshToken); // rotation NOT burned
        Assert.Equal(original.AccessToken, disk.AccessToken); // no blank Bearer persisted
    }

    /// <summary>
    /// An EMPTY refresh_token in an otherwise-valid 2xx must be treated like a missing one:
    /// keep the current, still-valid refresh token rather than clobbering it with a blank
    /// (which would 400 every future refresh). Break-verify: reverting the fallback to
    /// <c>parsed.RefreshToken ?? current.RefreshToken</c> persists "" here and fails this test.
    /// </summary>
    [Fact]
    public async Task EmptyRefreshTokenInGrantKeepsTheCurrentRefreshToken()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));
        var endpoint = new MockTokenEndpoint(_ => MockTokenEndpoint.Json(HttpStatusCode.OK,
            "{\"access_token\":\"fresh-access\",\"refresh_token\":\"\",\"expires_in\":3600}"));
        using var manager = Manager(temp, endpoint, Fixtures.Expired("r1"));

        Assert.Equal("fresh-access", await manager.GetAccessTokenAsync());

        var disk = temp.Store.LoadTokens();
        Assert.NotNull(disk);
        Assert.Equal("r1", disk!.RefreshToken); // kept, NOT blanked
        Assert.Equal("fresh-access", disk.AccessToken);
        Assert.Equal(1, endpoint.Calls);
    }

    /// <summary>
    /// A missing (omitted) refresh_token likewise keeps the current one — the server chose
    /// not to rotate.
    /// </summary>
    [Fact]
    public async Task MissingRefreshTokenInGrantKeepsTheCurrentRefreshToken()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));
        var endpoint = new MockTokenEndpoint(_ => MockTokenEndpoint.Json(HttpStatusCode.OK,
            "{\"access_token\":\"fresh-access\",\"expires_in\":3600}"));
        using var manager = Manager(temp, endpoint, Fixtures.Expired("r1"));

        Assert.Equal("fresh-access", await manager.GetAccessTokenAsync());
        Assert.Equal("r1", temp.Store.LoadTokens()!.RefreshToken);
    }

    // -- Redirect refusal: the confidential form must never reach a 3xx Location host ---------

    /// <summary>
    /// SOCKET-LEVEL guard for the redirect leak (a mock HttpMessageHandler cannot catch this —
    /// it short-circuits SendAsync before any redirect logic). Two real loopback endpoints: the
    /// token endpoint returns <c>307 Location: &lt;target&gt;</c>; the target would gladly hand
    /// back a valid token, so IF the client followed the 307 it would re-POST client_id +
    /// client_secret + refresh_token there and the leak would masquerade as a successful
    /// refresh. The production transport (<c>SocketsHttpHandler</c>, AllowAutoRedirect = false)
    /// must refuse: the 307 surfaces as a typed error and the target is NEVER contacted.
    /// Break-verify: flip to AllowAutoRedirect = true (and drop the 3xx guard) and this fails —
    /// the token becomes "leaked-access", the target is hit, and r2 lands on disk.
    /// </summary>
    [Fact]
    public async Task DefaultTokenClientRefusesRedirectsAndNeverResendsTheSecret()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));

        using var target = new LoopbackHttpServer(async ctx =>
        {
            ctx.Response.StatusCode = 200;
            ctx.Response.ContentType = "application/json";
            var bytes = Encoding.UTF8.GetBytes(
                "{\"access_token\":\"leaked-access\",\"refresh_token\":\"r2\",\"expires_in\":3600}");
            // 3-arg WriteAsync is available on every TFM (the ReadOnlyMemory overload is not on net472).
            await ctx.Response.OutputStream.WriteAsync(bytes, 0, bytes.Length);
        });
        using var redirector = new LoopbackHttpServer(ctx =>
        {
            ctx.Response.StatusCode = 307; // preserves method + body if (wrongly) followed
            ctx.Response.Headers["Location"] = target.Url;
            return Task.CompletedTask;
        });

        // NO injected handler → exercises the REAL default SocketsHttpHandler transport.
        using var manager = new TokenManager(temp.Store, Fixtures.Credentials, Fixtures.Expired("r1"),
            tokenUrl: redirector.Url);

        var e = await Assert.ThrowsAsync<TokenEndpointException>(() => manager.GetAccessTokenAsync());
        Assert.Equal(307, e.StatusCode);
        Assert.Equal(0, target.Hits); // the confidential form NEVER reached the redirect target

        var disk = temp.Store.LoadTokens();
        Assert.NotNull(disk);
        Assert.Equal("r1", disk!.RefreshToken); // a refused redirect rotates nothing
    }

    // -- Timeout: a hung endpoint must fail fast and typed, not wedge the lock for 30s --------

    /// <summary>
    /// A hung token endpoint must surface a typed <see cref="TransportException"/> on the
    /// (injected, short) per-call timeout — never a bare TaskCanceledException, and never the
    /// full 30s default wait (which would wedge every process queued on the store lock).
    /// </summary>
    [Fact]
    public async Task HungTokenEndpointTimesOutTypedAndFast()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));

        using var release = new SemaphoreSlim(0, 1);
        using var hung = new LoopbackHttpServer(async ctx =>
        {
            await release.WaitAsync(); // never respond until the test lets it unwind
            ctx.Response.StatusCode = 200;
        });

        using var manager = new TokenManager(temp.Store, Fixtures.Credentials, Fixtures.Expired("r1"),
            tokenUrl: hung.Url, httpTimeout: TimeSpan.FromMilliseconds(200));

        var sw = Stopwatch.StartNew();
        await Assert.ThrowsAsync<TransportException>(() => manager.GetAccessTokenAsync());
        sw.Stop();
        Assert.True(sw.Elapsed < TimeSpan.FromSeconds(10),
            $"timeout must fire on the 200ms injected timeout, not the 30s default (took {sw.Elapsed})");

        release.Release(); // let the hung handler unwind before teardown
        // The store is untouched — a timed-out refresh persists nothing.
        Assert.Equal("r1", temp.Store.LoadTokens()!.RefreshToken);
    }
}
