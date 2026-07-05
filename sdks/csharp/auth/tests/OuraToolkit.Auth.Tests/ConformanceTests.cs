using System.Net;
using System.Text.Json;
using Xunit;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// Cross-language auth-companion conformance (#58) — the C# leg.
///
/// Iterates <c>codegen/conformance/auth-cases.json</c> (the SINGLE SOURCE for the hostile
/// token-endpoint responses, hostile store files, and canonical store records that every
/// companion suite must exercise — new cases are added THERE, never here):
///
/// <list type="bullet">
/// <item>hostile-but-2xx token responses → the typed <see cref="TokenEndpointException"/>
/// with the 2xx status (what the PR #56 guards throw — never a raw
/// JsonException/NullReferenceException escaping), exactly ONE endpoint call (a hostile 2xx
/// is not a 400 — the reload-retry arm must not misfire), and <c>tokens.json</c> /
/// <c>credentials.json</c> byte-identical afterwards (persisting a blank/expired Bearer
/// would burn the still-valid rotated refresh token);</item>
/// <item>hostile store files → the typed <see cref="StoreFormatException"/>, never a
/// default-filled record that makes is-authenticated lie, and never an untyped crash;</item>
/// <item>canonical valid records → load with exactly the fixture's field values and
/// round-trip through this companion's own persist path (the cross-language store
/// compatibility check — field names are the shared wire format, #54).</item>
/// </list>
///
/// Mirrors the Rust reference leg (<c>sdks/rust/oura-toolkit-auth/tests/conformance.rs</c>)
/// and the Java leg (<c>sdks/java/auth/.../ConformanceTest.java</c>). Monorepo-only by
/// nature: the fixture is resolved by walking up to the repo root (nearest ancestor holding
/// the justfile + README.md — the same walk as every other leg); the shipped library has no
/// dependency on the repo layout.
/// </summary>
public class ConformanceTests
{
    // --- fixture loading ------------------------------------------------------------------

    /// <summary>Repo root: the nearest ancestor holding both the justfile and README.md.</summary>
    private static string FixturePath()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "justfile"))
                && File.Exists(Path.Combine(dir.FullName, "README.md")))
            {
                return Path.Combine(dir.FullName, "codegen", "conformance", "auth-cases.json");
            }
            dir = dir.Parent!;
        }
        throw new InvalidOperationException(
            "repo root (justfile + README.md) not found above the test binary");
    }

    /// <summary>The decoded shared fixture; cases are always iterated FROM THE FILE.</summary>
    private static JsonElement Fixture()
    {
        using var doc = JsonDocument.Parse(File.ReadAllText(FixturePath()));
        return doc.RootElement.Clone();
    }

    /// <summary>Expired on purpose, so a refresh genuinely calls the endpoint.</summary>
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

    // --- fixture-shape tripwires ------------------------------------------------------------

    /// <summary>
    /// The fixture-shrink guard: iterating theories would silently run fewer cases if the
    /// fixture shrank, so the table sizes are pinned here (>= 8 each, matching the other legs).
    /// </summary>
    [Fact]
    public void FixtureTablesHaveNotShrunk()
    {
        var fixture = Fixture();
        Assert.True(fixture.TryGetProperty("hostile_token_responses", out var responses),
            "fixture lost its hostile_token_responses table");
        Assert.True(fixture.TryGetProperty("hostile_store_files", out var storeFiles),
            "fixture lost its hostile_store_files table");
        Assert.True(responses.GetArrayLength() >= 8,
            $"fixture shrank? hostile_token_responses has {responses.GetArrayLength()} cases, want >= 8");
        Assert.True(storeFiles.GetArrayLength() >= 8,
            $"fixture shrank? hostile_store_files has {storeFiles.GetArrayLength()} cases, want >= 8");
    }

    /// <summary>
    /// If the fixture grows a NEW table, this leg must be extended deliberately — an unknown
    /// top-level key failing here beats ten silently-unexercised cases (mirrors the Java leg).
    /// </summary>
    [Fact]
    public void EveryFixtureTableIsMappedByThisSuite()
    {
        string[] known = ["$comment", "hostile_token_responses", "hostile_store_files", "valid_records"];
        var unknown = Fixture().EnumerateObject()
            .Select(p => p.Name)
            .Where(name => !known.Contains(name))
            .ToList();
        Assert.True(unknown.Count == 0,
            "the shared fixture grew tables this C# leg does not exercise — extend "
            + $"ConformanceTests to map them: {string.Join(", ", unknown)}");
    }

    // --- 1. hostile-but-2xx token responses --------------------------------------------------

    /// <summary>
    /// One (name, verbatim 200 body) pair per fixture case: <c>raw_body</c> is replayed
    /// VERBATIM (deliberately not JSON) when present; otherwise <c>body</c> is re-emitted as
    /// its exact JSON text, so a wrong-typed field (42, "soon") reaches the companion exactly
    /// as authored.
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
    /// <see cref="TokenEndpointException"/> carrying the 2xx status — the ThrowsAsync is the
    /// typed-error assertion: a raw JsonException or NullReferenceException escaping the PR #56
    /// guards fails the test naming it. A 200 is not a 400, so the reload-retry arm must not
    /// fire either (exactly one endpoint call), and BOTH persisted records stay byte-identical:
    /// persisting a blank/expired Bearer would burn the still-valid rotated refresh token.
    /// </summary>
    [Theory]
    [MemberData(nameof(HostileTokenResponses))]
    public async Task HostileTokenResponseFailsTypedAndLeavesTheStoreUntouched(string name, string body)
    {
        using var temp = new TempStore();
        temp.Store.SaveCredentials(Credentials());
        temp.Store.SaveTokens(OriginalTokens());
        var tokensBefore = File.ReadAllBytes(temp.Store.TokensPath);
        var credsBefore = File.ReadAllBytes(temp.Store.CredentialsPath);

        var endpoint = new MockTokenEndpoint(_ => MockTokenEndpoint.Json(HttpStatusCode.OK, body));
        using var manager = new TokenManager(temp.Store, Credentials(), OriginalTokens(),
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

        var e = await Assert.ThrowsAsync<TokenEndpointException>(() => manager.ForceRefreshAsync());
        Assert.Equal(200, e.StatusCode); // a 2xx, so the 400-retry arm cannot claim it
        // The typed error's diagnostic is FIXED and secret-free — a partial 2xx payload may
        // carry token material, so the raw body is never echoed (PR #56).
        Assert.DoesNotContain("rt-hostile-new", e.Message);

        Assert.Equal(1, endpoint.Calls); // a hostile 2xx must NOT trigger the reload-retry arm
        Assert.True(
            tokensBefore.SequenceEqual(File.ReadAllBytes(temp.Store.TokensPath)),
            $"case {name}: tokens.json must be byte-identical (persisting a blank/expired "
            + "Bearer would burn the still-valid rotation)");
        Assert.True(
            credsBefore.SequenceEqual(File.ReadAllBytes(temp.Store.CredentialsPath)),
            $"case {name}: credentials.json must be byte-identical after a failed refresh");
    }

    // --- 2. hostile store files ---------------------------------------------------------------

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
    /// Every hostile store file must fail its load with the typed
    /// <see cref="StoreFormatException"/> — never a default-filled record that makes
    /// is-authenticated lie (System.Text.Json is strict here: `required` members reject
    /// partial records, and wrong-typed fields like a NUMBER client_id are never coerced),
    /// never an untyped crash.
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

    // --- 3. canonical valid records -------------------------------------------------------------

    /// <summary>
    /// The canonical records load with exactly the fixture's values and survive a round-trip
    /// through this companion's own persist path — the shared wire format every language
    /// reads (field-name source of truth: oura-toolkit-auth's store.rs; #54). The literal
    /// expectations double as a fixture-drift tripwire, mirroring the Rust reference leg.
    /// </summary>
    [Fact]
    public void CanonicalValidRecordsLoadExactlyAndRoundTrip()
    {
        var fixture = Fixture();
        Assert.True(fixture.TryGetProperty("valid_records", out var valid),
            "fixture lost its valid_records table");
        Assert.True(valid.TryGetProperty("credentials.json", out var credsRecord),
            "fixture is missing valid_records[credentials.json]");
        Assert.True(valid.TryGetProperty("tokens.json", out var tokensRecord),
            "fixture is missing valid_records[tokens.json]");

        using var temp = new TempStore();
        File.WriteAllText(temp.Store.CredentialsPath, credsRecord.GetRawText());
        File.WriteAllText(temp.Store.TokensPath, tokensRecord.GetRawText());

        var creds = temp.Store.LoadCredentials();
        Assert.NotNull(creds);
        Assert.Equal("cid-conformance", creds!.ClientId);
        Assert.Equal("cs-conformance", creds.ClientSecret);

        var tokens = temp.Store.LoadTokens();
        Assert.NotNull(tokens);
        Assert.Equal("at-conformance", tokens!.AccessToken);
        Assert.Equal("rt-conformance", tokens.RefreshToken);
        Assert.Equal(4_102_444_800L, tokens.ExpiresAt);
        Assert.Equal("personal daily", tokens.Scope);
        Assert.Equal("Bearer", tokens.TokenType);

        // Round-trip: this companion's persist path must re-emit records the loader (and, by
        // the shared fixture, every other language) still reads identically — all five token
        // fields including scope + token_type.
        temp.Store.SaveCredentials(creds);
        temp.Store.SaveTokens(tokens);
        Assert.Equal(creds, temp.Store.LoadCredentials());
        Assert.Equal(tokens, temp.Store.LoadTokens());
    }
}
