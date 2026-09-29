using System.Net;
using System.Net.Http.Headers;
using System.Text;
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
/// <item>hostile-but-2xx token responses (incl. a wrong-typed or lone-surrogate string in any
/// of access_token / refresh_token / token_type / scope, an expires_in that is not an integer
/// in 1..=2147483647, and a body that is not valid UTF-8 anywhere) → the typed <see cref="TokenEndpointException"/>
/// with the 2xx status (what the PR #56 guards throw — never a raw
/// JsonException/NullReferenceException escaping), exactly ONE endpoint call (a hostile 2xx
/// is not a 400 — the reload-retry arm must not misfire), and <c>tokens.json</c> /
/// <c>credentials.json</c> byte-identical afterwards (persisting a blank/expired Bearer
/// would burn the still-valid rotated refresh token);</item>
/// <item>hostile store files → the typed <see cref="StoreFormatException"/>, never a
/// default-filled record that makes is-authenticated lie, and never an untyped crash;</item>
/// <item>canonical valid records → load with exactly the fixture's field values and
/// round-trip through this companion's own persist path (the cross-language store
/// compatibility check — field names are the shared wire format, #54);</item>
/// <item>refresh success cases → a SUCCESSFUL refresh from the stored <c>prior</c> record
/// persists exactly <c>expected</c> (access_token, refresh_token, scope, token_type) with
/// expires_at = refresh time + <c>expected.expires_in</c>: an omitted, null, empty,
/// whitespace-only (incl. U+00A0), or non-string <c>scope</c> keeps the prior grant (and a
/// non-string one must not fail the refresh); an omitted, null or empty refresh_token /
/// token_type keeps the prior value; expires_in at the 2147483647 cap succeeds; a lone
/// surrogate in an UNKNOWN field is not validated.</item>
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
    /// fixture shrank, so the table sizes are pinned here at the fixture's current sizes:
    /// >= 24 hostile_token_responses, >= 8 hostile_store_files, >= 17 refresh_success_cases.
    /// </summary>
    [Fact]
    public void FixtureTablesHaveNotShrunk()
    {
        var fixture = Fixture();
        Assert.True(fixture.TryGetProperty("hostile_token_responses", out var responses),
            "fixture lost its hostile_token_responses table");
        Assert.True(fixture.TryGetProperty("hostile_store_files", out var storeFiles),
            "fixture lost its hostile_store_files table");
        Assert.True(responses.GetArrayLength() >= 24,
            $"fixture shrank? hostile_token_responses has {responses.GetArrayLength()} cases, want >= 24");
        Assert.True(storeFiles.GetArrayLength() >= 8,
            $"fixture shrank? hostile_store_files has {storeFiles.GetArrayLength()} cases, want >= 8");
        Assert.True(fixture.TryGetProperty("refresh_success_cases", out var successTable),
            "fixture lost its refresh_success_cases table");
        var successCases = successTable.GetProperty("cases").GetArrayLength();
        Assert.True(successCases >= 17,
            $"fixture shrank? refresh_success_cases has {successCases} cases, want >= 17");
    }

    /// <summary>
    /// If the fixture grows a NEW table, this leg must be extended deliberately — an unknown
    /// top-level key failing here beats ten silently-unexercised cases (mirrors the Java leg).
    /// Every mapped table must also be PRESENT: a renamed table (e.g. the old
    /// refresh_scope_cases) fails here rather than leaving its theory iterating nothing.
    /// </summary>
    [Fact]
    public void EveryFixtureTableIsMappedByThisSuite()
    {
        string[] known =
        [
            "$comment", "hostile_token_responses", "hostile_store_files", "refresh_success_cases",
            "valid_records",
        ];
        var present = Fixture().EnumerateObject().Select(p => p.Name).ToList();
        var unknown = present.Where(name => !known.Contains(name)).ToList();
        Assert.True(unknown.Count == 0,
            "the shared fixture grew tables this C# leg does not exercise — extend "
            + $"ConformanceTests to map them: {string.Join(", ", unknown)}");
        var missing = known.Where(name => !present.Contains(name)).ToList();
        Assert.True(missing.Count == 0,
            "the shared fixture lost (or renamed) tables this C# leg maps — update "
            + $"ConformanceTests: {string.Join(", ", missing)}");
    }

    /// <summary>
    /// A case's 200 body as the exact bytes to serve: <c>raw_body_base64</c> decoded (bytes
    /// JSON can't hold, e.g. invalid UTF-8), <c>raw_body</c> VERBATIM (deliberately not JSON),
    /// otherwise <c>body</c> re-emitted as its exact JSON text, so a wrong-typed field (42,
    /// "soon") reaches the companion exactly as authored. Returned base64-encoded so the
    /// theory row stays a plain string.
    /// </summary>
    private static string BodyBase64(JsonElement c)
    {
        if (c.TryGetProperty("raw_body_base64", out var b64))
        {
            return b64.GetString()!;
        }
        var text = c.TryGetProperty("raw_body", out var raw) ? raw.GetString()! : c.GetProperty("body").GetRawText();
        return Convert.ToBase64String(Encoding.UTF8.GetBytes(text));
    }

    /// <summary>
    /// A 200 serving exactly <paramref name="bodyBase64"/>'s bytes — a ByteArrayContent, so
    /// invalid UTF-8 reaches the companion untouched (a StringContent would re-encode it).
    /// </summary>
    private static HttpResponseMessage OkBytes(string bodyBase64)
    {
        var content = new ByteArrayContent(Convert.FromBase64String(bodyBase64));
        content.Headers.ContentType = new MediaTypeHeaderValue("application/json");
        return new HttpResponseMessage(HttpStatusCode.OK) { Content = content };
    }

    // --- 1. hostile-but-2xx token responses --------------------------------------------------

    /// <summary>One (name, base64 of the exact 200 body bytes) pair per fixture case (<see cref="BodyBase64"/>).</summary>
    public static TheoryData<string, string> HostileTokenResponses()
    {
        var data = new TheoryData<string, string>();
        foreach (var c in Fixture().GetProperty("hostile_token_responses").EnumerateArray())
        {
            data.Add(c.GetProperty("name").GetString()!, BodyBase64(c));
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
    public async Task HostileTokenResponseFailsTypedAndLeavesTheStoreUntouched(string name, string bodyBase64)
    {
        using var temp = new TempStore();
        temp.Store.SaveCredentials(Credentials());
        temp.Store.SaveTokens(OriginalTokens());
        var tokensBefore = File.ReadAllBytes(temp.Store.TokensPath);
        var credsBefore = File.ReadAllBytes(temp.Store.CredentialsPath);

        var endpoint = new MockTokenEndpoint(_ => OkBytes(bodyBase64));
        using var manager = new TokenManager(temp.Store, Credentials(), OriginalTokens(),
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

        var thrown = await Record.ExceptionAsync(() => manager.ForceRefreshAsync());
        Assert.True(thrown is TokenEndpointException,
            $"case {name}: a hostile 2xx must fail with the typed TokenEndpointException, got "
            + (thrown is null ? "a SUCCESSFUL refresh" : thrown.GetType().Name));
        var e = (TokenEndpointException)thrown!;
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

    // --- 3. refresh success cases -------------------------------------------------------------

    /// <summary>
    /// One (name, base64 of the exact 200 body bytes, expected record JSON) triple per case;
    /// <c>expected</c> travels as raw JSON text so the theory row stays plain strings.
    /// </summary>
    public static TheoryData<string, string, string> RefreshSuccessCases()
    {
        var data = new TheoryData<string, string, string>();
        foreach (var c in Fixture().GetProperty("refresh_success_cases").GetProperty("cases").EnumerateArray())
        {
            data.Add(c.GetProperty("name").GetString()!, BodyBase64(c), c.GetProperty("expected").GetRawText());
        }
        return data;
    }

    /// <summary>
    /// The table's shared starting record, read FROM THE FILE, with an already-expired
    /// expires_at so the refresh genuinely calls the endpoint.
    /// </summary>
    private static Tokens PriorTokens()
    {
        var prior = Fixture().GetProperty("refresh_success_cases").GetProperty("prior");
        return new Tokens
        {
            AccessToken = prior.GetProperty("access_token").GetString()!,
            RefreshToken = prior.GetProperty("refresh_token").GetString()!,
            Scope = prior.GetProperty("scope").GetString(),
            TokenType = prior.GetProperty("token_type").GetString(),
            ExpiresAt = 0,
        };
    }

    /// <summary>
    /// A SUCCESSFUL refresh starting from the stored <c>prior</c> record must persist EXACTLY
    /// <c>expected</c>: access_token, refresh_token, scope and token_type, and expires_at within
    /// [t0 + expires_in, t1 + expires_in] where t0/t1 are unix seconds just before/after the
    /// refresh. Fallbacks under test: an omitted/null/empty/whitespace/non-string scope keeps
    /// the prior grant (RFC 6749 §5.1; a blank would erase the grant #116's re-consent reads);
    /// an omitted/null/EMPTY refresh_token or token_type keeps the prior value (persisting ""
    /// would 400 the next refresh); expires_in at the 2147483647 cap succeeds; a lone surrogate
    /// in an unknown field is not validated. The PERSISTED record is asserted, not just the
    /// returned value.
    /// </summary>
    [Theory]
    [MemberData(nameof(RefreshSuccessCases))]
    public async Task RefreshSuccessCasesPersistExpectedRecord(string name, string bodyBase64, string expectedJson)
    {
        using var expectedDoc = JsonDocument.Parse(expectedJson);
        var expected = expectedDoc.RootElement;
        var prior = PriorTokens();
        using var temp = new TempStore();
        temp.Store.SaveCredentials(Credentials());
        temp.Store.SaveTokens(prior);

        var endpoint = new MockTokenEndpoint(_ => OkBytes(bodyBase64));
        using var manager = new TokenManager(temp.Store, Credentials(), prior,
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

        var t0 = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        var thrown = await Record.ExceptionAsync(() => manager.ForceRefreshAsync());
        var t1 = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        Assert.True(thrown is null,
            $"case {name}: a valid 2xx must refresh SUCCESSFULLY, got {thrown?.GetType().Name}: {thrown?.Message}");

        Assert.True(endpoint.Calls == 1, $"case {name}: want exactly one endpoint call, got {endpoint.Calls}");
        var persisted = temp.Store.LoadTokens();
        Assert.True(persisted is not null, $"case {name}: the refresh must persist a token record");
        foreach (var (field, actual) in new[]
        {
            ("access_token", persisted!.AccessToken),
            ("refresh_token", persisted.RefreshToken),
            ("scope", persisted.Scope),
            ("token_type", persisted.TokenType),
        })
        {
            var want = expected.GetProperty(field).GetString();
            Assert.True(actual == want,
                $"case {name}: persisted {field} must be exactly \"{want}\" (prior "
                + $"{prior}; omitted/null/empty keeps the prior value), got \"{actual}\"");
        }
        var expiresIn = expected.GetProperty("expires_in").GetInt64();
        Assert.True(persisted.ExpiresAt >= t0 + expiresIn && persisted.ExpiresAt <= t1 + expiresIn,
            $"case {name}: persisted expires_at must be refresh time + {expiresIn} "
            + $"(within [{t0 + expiresIn}, {t1 + expiresIn}]), got {persisted.ExpiresAt}");
    }

    // --- 4. canonical valid records -------------------------------------------------------------

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
