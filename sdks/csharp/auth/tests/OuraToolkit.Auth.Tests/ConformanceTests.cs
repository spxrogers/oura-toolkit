using System.Net;
using System.Text.Json;
using Xunit;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// Cross-language auth-companion conformance (#58) — the C# leg.
///
/// Iterates <c>codegen/conformance/auth-cases.json</c> (the single source for the hostile
/// token-endpoint responses, hostile store files, and canonical store records that every
/// companion suite must exercise — new cases are added THERE, never here):
///
/// <list type="bullet">
/// <item>hostile-but-2xx token responses → the typed <see cref="TokenEndpointException"/>
/// (what the PR #56 guards throw — never a raw JsonException/NullReferenceException), and
/// <c>tokens.json</c> byte-identical (the rotated refresh token is never burned by
/// persisting a blank/expired Bearer);</item>
/// <item>hostile store files → the typed <see cref="StoreFormatException"/>, never a
/// default-filled record that makes is-authenticated lie, and never an untyped crash;</item>
/// <item>canonical valid records → load with exactly the fixture's field values and
/// round-trip through this companion's own persist path (the cross-language store
/// compatibility check — field names are the shared wire format, #54).</item>
/// </list>
///
/// Monorepo-only by nature (walks up to the repo root for the fixture, like
/// <see cref="MetadataSpecSyncTests"/>); the shipped library has no dependency on the
/// repo layout.
/// </summary>
public class ConformanceTests
{
    private static string FixturePath()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "justfile")))
            {
                return Path.Combine(dir.FullName, "codegen", "conformance", "auth-cases.json");
            }
            dir = dir.Parent!;
        }
        throw new InvalidOperationException("repo root (justfile) not found above the test binary");
    }

    private static JsonElement Fixture()
    {
        using var doc = JsonDocument.Parse(File.ReadAllText(FixturePath()));
        return doc.RootElement.Clone();
    }

    /// <summary>Expired, so a refresh genuinely calls the endpoint (mirrors the Rust leg).</summary>
    private static Tokens OriginalTokens() => new()
    {
        AccessToken = "at-original",
        RefreshToken = "rt-original",
        ExpiresAt = 0,
    };

    private static ClientCredentials Credentials() => new()
    {
        ClientId = "cid",
        ClientSecret = "cs",
    };

    /// <summary>
    /// The fixture-shrink guard: iterating THEORIES silently run fewer cases if the fixture
    /// shrinks, so the table sizes are pinned here (>= 8 each, matching the Rust leg).
    /// </summary>
    [Fact]
    public void FixtureTablesHaveNotShrunk()
    {
        var fixture = Fixture();
        var hostileResponses = fixture.GetProperty("hostile_token_responses").GetArrayLength();
        var hostileStoreFiles = fixture.GetProperty("hostile_store_files").GetArrayLength();
        Assert.True(hostileResponses >= 8, $"fixture shrank? {hostileResponses} hostile_token_responses cases");
        Assert.True(hostileStoreFiles >= 8, $"fixture shrank? {hostileStoreFiles} hostile_store_files cases");
    }

    // -- hostile_token_responses ---------------------------------------------------------------

    /// <summary>
    /// One (name, verbatim 200 body) pair per fixture case: <c>raw_body</c> is used verbatim
    /// when present, otherwise <c>body</c> is re-emitted as its exact JSON text.
    /// </summary>
    public static TheoryData<string, string> HostileTokenResponses()
    {
        var data = new TheoryData<string, string>();
        foreach (var c in Fixture().GetProperty("hostile_token_responses").EnumerateArray())
        {
            var name = c.GetProperty("name").GetString()!;
            var body = c.TryGetProperty("raw_body", out var raw)
                ? raw.GetString()!
                : c.GetProperty("body").GetRawText();
            data.Add(name, body);
        }
        return data;
    }

    /// <summary>
    /// Every hostile-but-2xx token response must fail the refresh with the typed
    /// <see cref="TokenEndpointException"/> and leave <c>tokens.json</c> byte-identical —
    /// the rotated refresh token is never burned by a blank/expired Bearer. A 200 is not a
    /// 400, so the reload-retry arm must not fire either: exactly one endpoint call.
    /// </summary>
    [Theory]
    [MemberData(nameof(HostileTokenResponses))]
    public async Task HostileTokenResponseFailsTypedAndLeavesTheStoreUntouched(string name, string body)
    {
        using var temp = new TempStore();
        temp.Store.SaveCredentials(Credentials());
        temp.Store.SaveTokens(OriginalTokens());
        var bytesBefore = File.ReadAllBytes(temp.Store.TokensPath);

        var endpoint = new MockTokenEndpoint(_ => MockTokenEndpoint.Json(HttpStatusCode.OK, body));
        using var manager = new TokenManager(temp.Store, Credentials(), OriginalTokens(),
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

        // ThrowsAsync is the typed-error assertion: a raw JsonException or
        // NullReferenceException escaping the guards fails the test naming it.
        var e = await Assert.ThrowsAsync<TokenEndpointException>(() => manager.ForceRefreshAsync());
        Assert.Equal(200, e.StatusCode);

        Assert.Equal(1, endpoint.Calls); // a hostile 2xx must NOT trigger the 400-retry arm
        Assert.True(
            bytesBefore.SequenceEqual(File.ReadAllBytes(temp.Store.TokensPath)),
            $"case {name}: the store must be UNTOUCHED (rotation not burned)");
    }

    // -- hostile_store_files ---------------------------------------------------------------------

    /// <summary>One (name, record file, exact file content) triple per fixture case.</summary>
    public static TheoryData<string, string, string> HostileStoreFiles()
    {
        var data = new TheoryData<string, string, string>();
        foreach (var c in Fixture().GetProperty("hostile_store_files").EnumerateArray())
        {
            data.Add(
                c.GetProperty("name").GetString()!,
                c.GetProperty("file").GetString()!,
                c.GetProperty("content").GetString()!);
        }
        return data;
    }

    /// <summary>
    /// Every hostile store file must load as the typed <see cref="StoreFormatException"/> —
    /// never a default-filled record, never an untyped crash.
    /// </summary>
    [Theory]
    [MemberData(nameof(HostileStoreFiles))]
    public void HostileStoreFileFailsTyped(string name, string file, string content)
    {
        using var temp = new TempStore();
        File.WriteAllText(Path.Combine(temp.Dir, file), content);

        var e = file switch
        {
            "tokens.json" => Assert.Throws<StoreFormatException>(() => temp.Store.LoadTokens()),
            "credentials.json" => Assert.Throws<StoreFormatException>(() => temp.Store.LoadCredentials()),
            _ => throw new InvalidOperationException($"case {name}: fixture names an unknown store file {file}"),
        };
        Assert.Contains(file, e.Message); // the typed error names the offending record
    }

    // -- valid_records ---------------------------------------------------------------------------

    /// <summary>
    /// The canonical records load with exactly the fixture's values and survive a round-trip
    /// through this companion's own persist path — the shared wire format every language
    /// reads (field-name source of truth: oura-toolkit-auth's store.rs).
    /// </summary>
    [Fact]
    public void CanonicalValidRecordsLoadExactlyAndRoundTrip()
    {
        var valid = Fixture().GetProperty("valid_records");

        using var temp = new TempStore();
        File.WriteAllText(temp.Store.CredentialsPath, valid.GetProperty("credentials.json").GetRawText());
        File.WriteAllText(temp.Store.TokensPath, valid.GetProperty("tokens.json").GetRawText());

        var creds = temp.Store.LoadCredentials();
        Assert.NotNull(creds);
        Assert.Equal("cid-conformance", creds!.ClientId);
        Assert.Equal("cs-conformance", creds.ClientSecret);

        var tokens = temp.Store.LoadTokens();
        Assert.NotNull(tokens);
        Assert.Equal("at-conformance", tokens!.AccessToken);
        Assert.Equal("rt-conformance", tokens.RefreshToken);
        Assert.Equal(4_102_444_800, tokens.ExpiresAt);
        Assert.Equal("personal daily", tokens.Scope);
        Assert.Equal("Bearer", tokens.TokenType);

        // Round-trip: this companion's persist path must re-emit records the loader (and, by
        // the shared fixture, every other language) still reads identically.
        temp.Store.SaveCredentials(creds);
        temp.Store.SaveTokens(tokens);
        Assert.Equal(creds, temp.Store.LoadCredentials());
        Assert.Equal(tokens, temp.Store.LoadTokens());
    }
}
