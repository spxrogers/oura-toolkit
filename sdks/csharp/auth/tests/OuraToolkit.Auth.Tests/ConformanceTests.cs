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
/// in 1..=2147483647, a body that is not valid UTF-8 anywhere, and trailing data after the
/// one top-level value) → the typed <see cref="TokenEndpointException"/>
/// with the 2xx status (what the PR #56 guards throw — never a raw
/// JsonException/NullReferenceException escaping), exactly ONE endpoint call (a hostile 2xx
/// is not a 400 — the reload-retry arm must not misfire), and <c>tokens.json</c> /
/// <c>credentials.json</c> byte-identical afterwards (persisting a blank/expired Bearer
/// would burn the still-valid rotated refresh token);</item>
/// <item>implementation-defined token responses (BOM, duplicate keys, deep nesting, an
/// integral-float expires_in, an upper-case key) → EITHER a successful refresh persisting
/// access_token = at-refreshed, refresh_token = rt-refreshed or the prior one (never empty)
/// and expires_at = refresh time + 3600, OR the typed error with the store byte-identical —
/// never an untyped exception or a half-written store;</item>
/// <item>hostile store files (given as <c>content</c> text or <c>content_base64</c> bytes,
/// e.g. invalid UTF-8 — a store file must be valid UTF-8) → the typed
/// <see cref="StoreFormatException"/>, never a default-filled record that makes
/// is-authenticated lie, never a U+FFFD-substituted load, and never an untyped crash;</item>
/// <item>rejected (non-2xx) token responses, with the store and credentials seeded with the
/// table's <c>submitted</c> secrets → the typed <see cref="TokenEndpointException"/> carrying
/// the case's status, the store byte-identical, <c>must_echo</c> kept in the error for
/// diagnosis, <c>must_not_echo</c> (a submitted secret the server echoed back) appearing nowhere
/// in the chain, and no text in the chain longer than <c>max_error_chars</c>;</item>
/// <item>implementation-defined store files (nesting past a parser's depth limit in an
/// unknown field) → EITHER exactly the fixture's <c>expected</c> record OR the typed
/// <see cref="StoreFormatException"/> — never an untyped crash;</item>
/// <item>every case carrying <c>must_not_echo</c> (token responses and store files) → that
/// string appears nowhere in the typed error's text or in any exception it chains
/// (InnerException, recursively, incl. every AggregateException inner): parser messages can
/// quote content, and the content is token material. Each needle must actually occur in its
/// case's payload (else the check is vacuous), and at least 4 hostile token / 5 hostile store
/// cases must carry one;</item>
/// <item>canonical valid records → load with exactly the fixture's field values and
/// round-trip through this companion's own persist path (the cross-language store
/// compatibility check — field names are the shared wire format, #54);</item>
/// <item>refresh success cases → a SUCCESSFUL refresh from the stored <c>prior</c> record
/// SENDS the prior refresh_token and persists exactly <c>expected</c> (access_token,
/// refresh_token, scope, token_type) with
/// expires_at = refresh time + <c>expected.expires_in</c>: an omitted, null, empty,
/// whitespace-only (incl. U+00A0), or non-string <c>scope</c> keeps the prior grant (and a
/// non-string one must not fail the refresh); an omitted, null or empty refresh_token /
/// token_type keeps the prior value; expires_in at both bounds (1 and the 2147483647 cap)
/// succeeds; a lone
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
    /// >= 30 hostile_token_responses, >= 5 implementation_defined_token_responses,
    /// >= 21 hostile_store_files, >= 1 implementation_defined_store_files,
    /// >= 12 rejected_token_responses, >= 18 refresh_success_cases.
    /// </summary>
    [Fact]
    public void FixtureTablesHaveNotShrunk()
    {
        var fixture = Fixture();
        Assert.True(fixture.TryGetProperty("hostile_token_responses", out var responses),
            "fixture lost its hostile_token_responses table");
        Assert.True(fixture.TryGetProperty("hostile_store_files", out var storeFiles),
            "fixture lost its hostile_store_files table");
        Assert.True(fixture.TryGetProperty("implementation_defined_token_responses", out var implDefined),
            "fixture lost its implementation_defined_token_responses table");
        Assert.True(fixture.TryGetProperty("implementation_defined_store_files", out var implStore),
            "fixture lost its implementation_defined_store_files table");
        Assert.True(responses.GetArrayLength() >= 30,
            $"fixture shrank? hostile_token_responses has {responses.GetArrayLength()} cases, want >= 30");
        Assert.True(implStore.GetArrayLength() >= 1,
            $"fixture shrank? implementation_defined_store_files has {implStore.GetArrayLength()} cases, want >= 1");
        Assert.True(implDefined.GetArrayLength() >= 5,
            $"fixture shrank? implementation_defined_token_responses has {implDefined.GetArrayLength()} cases, want >= 5");
        Assert.True(storeFiles.GetArrayLength() >= 21,
            $"fixture shrank? hostile_store_files has {storeFiles.GetArrayLength()} cases, want >= 21");
        Assert.True(fixture.TryGetProperty("rejected_token_responses", out var rejectedTable),
            "fixture lost its rejected_token_responses table");
        var rejectedCases = rejectedTable.GetProperty("cases").GetArrayLength();
        Assert.True(rejectedCases >= 12,
            $"fixture shrank? rejected_token_responses has {rejectedCases} cases, want >= 12");
        Assert.True(fixture.TryGetProperty("refresh_success_cases", out var successTable),
            "fixture lost its refresh_success_cases table");
        var successCases = successTable.GetProperty("cases").GetArrayLength();
        Assert.True(successCases >= 18,
            $"fixture shrank? refresh_success_cases has {successCases} cases, want >= 18");
    }

    /// <summary>
    /// If the fixture grows a NEW table, this leg must be extended deliberately — an unknown
    /// top-level key failing here beats ten silently-unexercised cases (mirrors the Java leg).
    /// Every mapped table must also be PRESENT: a renamed table (e.g. the old
    /// refresh_scope_cases) fails here rather than leaving its theory iterating nothing.
    /// Together the two checks pin the set to EXACTLY the seven tables (plus <c>$comment</c>).
    /// </summary>
    [Fact]
    public void EveryFixtureTableIsMappedByThisSuite()
    {
        string[] known =
        [
            "$comment", "hostile_token_responses", "implementation_defined_token_responses",
            "hostile_store_files", "rejected_token_responses", "implementation_defined_store_files",
            "refresh_success_cases", "valid_records",
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
    /// theory row stays a plain string. A case must give EXACTLY ONE of the three: two would
    /// make the served body depend on this helper's precedence (not the fixture author's
    /// intent), none would silently serve nothing — either fails loading, naming the case.
    /// </summary>
    private static string BodyBase64(JsonElement c)
    {
        var name = c.GetProperty("name").GetString();
        string[] keys = ["body", "raw_body", "raw_body_base64"];
        var given = keys.Where(k => c.TryGetProperty(k, out _)).ToList();
        if (given.Count != 1)
        {
            throw new InvalidOperationException(
                $"fixture case {name}: must give exactly one of body/raw_body/raw_body_base64, "
                + $"got [{string.Join(", ", given)}]");
        }
        switch (given[0])
        {
            case "raw_body_base64":
                return c.GetProperty("raw_body_base64").GetString()!;
            case "raw_body":
                return Convert.ToBase64String(Encoding.UTF8.GetBytes(c.GetProperty("raw_body").GetString()!));
            default:
                return Convert.ToBase64String(Encoding.UTF8.GetBytes(c.GetProperty("body").GetRawText()));
        }
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

    /// <summary>A case's optional <c>must_not_echo</c> string (null when the case has none).</summary>
    private static string? MustNotEcho(JsonElement c) =>
        c.TryGetProperty("must_not_echo", out var v) ? v.GetString() : null;

    /// <summary>
    /// <paramref name="e"/> and every exception it chains — InnerException, recursively, and
    /// every AggregateException inner — each visited once (a cycle cannot loop forever).
    /// </summary>
    private static IEnumerable<Exception> ExceptionChain(Exception e)
    {
        // A reference-identity list, not a HashSet: ReferenceEqualityComparer is .NET 5+ and
        // this suite also builds for net472 (the netstandard2.0 Mono leg, #61).
        var seen = new List<Exception>();
        var pending = new Stack<Exception>();
        pending.Push(e);
        while (pending.Count > 0)
        {
            var next = pending.Pop();
            if (seen.Any(s => ReferenceEquals(s, next)))
            {
                continue;
            }
            seen.Add(next);
            yield return next;
            if (next is AggregateException aggregate)
            {
                foreach (var inner in aggregate.InnerExceptions)
                {
                    pending.Push(inner);
                }
            }
            if (next.InnerException is { } innerException)
            {
                pending.Push(innerException);
            }
        }
    }

    /// <summary>
    /// The fixture's <c>must_not_echo</c> contract: <paramref name="secret"/> appears in NO
    /// exception of <paramref name="e"/>'s chain — neither its ToString() (message + type +
    /// stack) nor its Message, nor a TokenEndpointException's public <c>Body</c>. A parser
    /// message quoting the content it choked on would leak token material into logs.
    /// </summary>
    private static void AssertDoesNotEcho(string name, Exception e, string? secret)
    {
        if (secret is null)
        {
            return;
        }
        Assert.False(string.IsNullOrEmpty(secret), $"fixture case {name}: must_not_echo must be non-empty");
        foreach (var link in ExceptionChain(e))
        {
            var texts = new List<(string What, string Text)>
            {
                ("ToString()", link.ToString()),
                ("Message", link.Message),
            };
            if (link is TokenEndpointException endpointError)
            {
                texts.Add(("Body", endpointError.Body));
            }
            foreach (var (what, text) in texts)
            {
                Assert.False(text.Contains(secret),
                    $"case {name}: the typed error chain must never echo \"{secret}\" (must_not_echo), "
                    + $"but {link.GetType().Name}.{what} does: {text}");
            }
        }
    }

    /// <summary>
    /// Why a case's <c>must_not_echo</c> needle would make its no-echo check vacuous, or null
    /// when it is sound: the needle must be non-empty and its UTF-8 bytes must actually occur
    /// in the case's payload (the decoded body / file bytes) — a needle the payload doesn't
    /// carry can never be echoed, so the check would pass against any implementation.
    /// </summary>
    internal static string? NeedleProblem(string name, byte[] payload, string? needle)
    {
        if (needle is null)
        {
            return null;
        }
        if (needle.Length == 0)
        {
            return $"fixture case {name}: must_not_echo must be non-empty";
        }
        var n = Encoding.UTF8.GetBytes(needle);
        for (var i = 0; i + n.Length <= payload.Length; i++)
        {
            var match = true;
            for (var j = 0; j < n.Length && match; j++)
            {
                match = payload[i + j] == n[j];
            }
            if (match)
            {
                return null;
            }
        }
        return $"fixture case {name}: must_not_echo \"{needle}\" does not occur in the case's "
            + "payload, so its no-echo check is vacuous (it could never be echoed)";
    }

    /// <summary>Fails naming the case when its <c>must_not_echo</c> check would be vacuous (<see cref="NeedleProblem"/>).</summary>
    private static void AssertNeedleInPayload(string name, byte[] payload, string? needle)
    {
        var problem = NeedleProblem(name, payload, needle);
        Assert.True(problem is null, problem);
    }

    /// <summary>
    /// The vacuity guard itself is load-bearing: a needle absent from its payload (or empty) is
    /// rejected naming the case, a present one (incl. inside invalid-UTF-8 bytes) accepted.
    /// </summary>
    [Fact]
    public void NeedleGuardRejectsAVacuousMustNotEcho()
    {
        var payload = Encoding.UTF8.GetBytes("{\"refresh_token\":\"rtSEC000\"}");
        Assert.Null(NeedleProblem("present", payload, "rtSEC000"));
        Assert.Null(NeedleProblem("no-needle", payload, null));
        var absent = NeedleProblem("absent", payload, "rtSEC999");
        Assert.NotNull(absent);
        Assert.Contains("fixture case absent", absent);
        Assert.Contains("vacuous", absent);
        Assert.Contains("non-empty", NeedleProblem("empty", payload, ""));
        byte[] invalidUtf8 = [.. Encoding.UTF8.GetBytes("\"rtSEC135"), 0xFF, (byte)'"'];
        Assert.Null(NeedleProblem("invalid-utf8", invalidUtf8, "rtSEC135"));
    }

    /// <summary>
    /// Every <c>must_not_echo</c> needle in the hostile tables actually occurs in its case's
    /// payload, and the no-echo coverage cannot silently erode: at least 4 hostile token cases
    /// and 5 hostile store cases carry one (the fixture's floors).
    /// </summary>
    [Fact]
    public void MustNotEchoNeedlesAreSoundAndMeetTheFloors()
    {
        var fixture = Fixture();
        var tables = new (string Table, Func<JsonElement, string> PayloadBase64, int Floor)[]
        {
            ("hostile_token_responses", BodyBase64, 4),
            ("hostile_store_files", ContentBase64, 5),
        };
        foreach (var (table, payloadBase64, floor) in tables)
        {
            var carrying = 0;
            foreach (var c in fixture.GetProperty(table).EnumerateArray())
            {
                var needle = MustNotEcho(c);
                if (needle is null)
                {
                    continue;
                }
                carrying++;
                AssertNeedleInPayload(c.GetProperty("name").GetString()!,
                    Convert.FromBase64String(payloadBase64(c)), needle);
            }
            Assert.True(carrying >= floor,
                $"fixture shrank? only {carrying} {table} cases carry must_not_echo, want >= {floor}");
        }
    }

    // --- 1. hostile-but-2xx token responses --------------------------------------------------

    /// <summary>
    /// One (name, base64 of the exact 200 body bytes (<see cref="BodyBase64"/>), optional
    /// must_not_echo) triple per fixture case.
    /// </summary>
    public static TheoryData<string, string, string?> HostileTokenResponses()
    {
        var data = new TheoryData<string, string, string?>();
        foreach (var c in Fixture().GetProperty("hostile_token_responses").EnumerateArray())
        {
            data.Add(c.GetProperty("name").GetString()!, BodyBase64(c), MustNotEcho(c));
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
    /// A case's <c>must_not_echo</c> string must appear nowhere in the error chain.
    /// </summary>
    [Theory]
    [MemberData(nameof(HostileTokenResponses))]
    public async Task HostileTokenResponseFailsTypedAndLeavesTheStoreUntouched(string name, string bodyBase64, string? mustNotEcho)
    {
        AssertNeedleInPayload(name, Convert.FromBase64String(bodyBase64), mustNotEcho);
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
        AssertDoesNotEcho(name, e, mustNotEcho);

        Assert.Equal(1, endpoint.Calls); // a hostile 2xx must NOT trigger the reload-retry arm
        Assert.True(
            tokensBefore.SequenceEqual(File.ReadAllBytes(temp.Store.TokensPath)),
            $"case {name}: tokens.json must be byte-identical (persisting a blank/expired "
            + "Bearer would burn the still-valid rotation)");
        Assert.True(
            credsBefore.SequenceEqual(File.ReadAllBytes(temp.Store.CredentialsPath)),
            $"case {name}: credentials.json must be byte-identical after a failed refresh");
    }

    // --- 1a. rejected (non-2xx) token responses -----------------------------------------------

    /// <summary>The rejected_token_responses table (its <c>submitted</c> secrets, cap and cases).</summary>
    private static JsonElement RejectedTable() => Fixture().GetProperty("rejected_token_responses");

    /// <summary>
    /// The secrets a rejected case submits: the case's own <c>submitted</c> when it carries one
    /// (e.g. one secret nested in another), else the table's.
    /// </summary>
    private static (string RefreshToken, string ClientSecret) RejectedSubmitted(JsonElement table, JsonElement c)
    {
        var submitted = c.TryGetProperty("submitted", out var own) ? own : table.GetProperty("submitted");
        return (submitted.GetProperty("refresh_token").GetString()!,
            submitted.GetProperty("client_secret").GetString()!);
    }

    /// <summary>A case's optional <c>expected_body</c> (null when the case has none).</summary>
    private static string? ExpectedBody(JsonElement c) =>
        c.TryGetProperty("expected_body", out var v) ? v.GetString() : null;

    /// <summary>A case's optional <c>cut_well_formed</c> flag (false when absent).</summary>
    private static bool CutWellFormed(JsonElement c) =>
        c.TryGetProperty("cut_well_formed", out var v) && v.GetBoolean();

    /// <summary>
    /// One row per fixture case: (name, status, raw body, must_echo, optional must_not_echo,
    /// submitted refresh_token, submitted client_secret, optional expected_body, cut_well_formed).
    /// </summary>
    public static TheoryData<string, int, string, string, string?, string, string, string?, bool> RejectedTokenResponses()
    {
        var data = new TheoryData<string, int, string, string, string?, string, string, string?, bool>();
        var table = RejectedTable();
        foreach (var c in table.GetProperty("cases").EnumerateArray())
        {
            var (refreshToken, clientSecret) = RejectedSubmitted(table, c);
            data.Add(
                c.GetProperty("name").GetString()!,
                c.GetProperty("status").GetInt32(),
                c.GetProperty("raw_body").GetString()!,
                c.GetProperty("must_echo").GetString()!,
                MustNotEcho(c),
                refreshToken,
                clientSecret,
                ExpectedBody(c),
                CutWellFormed(c));
        }
        return data;
    }

    /// <summary>
    /// The table's guards: its secrets (and any case's own <c>submitted</c>) are non-empty,
    /// every case's status is a non-2xx, and every <c>must_echo</c> / <c>must_not_echo</c>
    /// needle actually occurs in the case's raw_body — a needle the body doesn't carry makes
    /// its check vacuous (it could never be echoed, or never be missing). An
    /// <c>expected_body</c> must be an over-cap body's first 1024 chars plus "…", and a
    /// <c>cut_well_formed</c> body must really put a surrogate pair across the cut in UTF-16
    /// (a high surrogate at index 1023) — otherwise either check would be vacuous here.
    /// </summary>
    [Fact]
    public void RejectedTokenResponseNeedlesAreSound()
    {
        var table = RejectedTable();
        var submitted = table.GetProperty("submitted");
        Assert.False(string.IsNullOrEmpty(submitted.GetProperty("refresh_token").GetString()),
            "rejected_token_responses.submitted.refresh_token must be non-empty");
        Assert.False(string.IsNullOrEmpty(submitted.GetProperty("client_secret").GetString()),
            "rejected_token_responses.submitted.client_secret must be non-empty");
        Assert.True(table.GetProperty("max_error_chars").GetInt32() > 0,
            "rejected_token_responses.max_error_chars must be positive");
        foreach (var c in table.GetProperty("cases").EnumerateArray())
        {
            var name = c.GetProperty("name").GetString()!;
            var status = c.GetProperty("status").GetInt32();
            Assert.False(status is >= 200 and < 300, $"fixture case {name}: status {status} is not a non-2xx");
            var body = Encoding.UTF8.GetBytes(c.GetProperty("raw_body").GetString()!);
            var mustEcho = c.GetProperty("must_echo").GetString();
            Assert.False(string.IsNullOrEmpty(mustEcho), $"fixture case {name}: must_echo must be non-empty");
            Assert.True(NeedleProblem(name, body, mustEcho) is null,
                $"fixture case {name}: must_echo \"{mustEcho}\" does not occur in its raw_body");
            AssertNeedleInPayload(name, body, MustNotEcho(c));
            if (c.TryGetProperty("submitted", out _))
            {
                var (refreshToken, clientSecret) = RejectedSubmitted(table, c);
                Assert.False(string.IsNullOrEmpty(refreshToken),
                    $"fixture case {name}: submitted.refresh_token must be non-empty");
                Assert.False(string.IsNullOrEmpty(clientSecret),
                    $"fixture case {name}: submitted.client_secret must be non-empty");
            }
            var rawBody = c.GetProperty("raw_body").GetString()!;
            var expectedBody = ExpectedBody(c);
            if (expectedBody is not null)
            {
                Assert.True(rawBody.Length > TokenManager.MaxDiagnosticBodyChars
                        && expectedBody == rawBody.Substring(0, TokenManager.MaxDiagnosticBodyChars) + "\u2026",
                    $"fixture case {name}: expected_body must be the over-cap raw_body's first "
                    + $"{TokenManager.MaxDiagnosticBodyChars} chars plus \"\u2026\"");
            }
            if (CutWellFormed(c))
            {
                Assert.True(rawBody.Length > TokenManager.MaxDiagnosticBodyChars
                        && char.IsHighSurrogate(rawBody[TokenManager.MaxDiagnosticBodyChars - 1]),
                    $"fixture case {name}: cut_well_formed needs a surrogate pair straddling the "
                    + $"{TokenManager.MaxDiagnosticBodyChars}-char cut, or the check is vacuous in UTF-16");
            }
        }
    }

    /// <summary>
    /// A NON-2xx token response, with the stored refresh_token and the credentials'
    /// client_secret seeded from <c>submitted</c> — the case's own, else the table's (so those
    /// exact values are what the refresh sends — asserted on the captured request). The mock answers
    /// <paramref name="status"/> with <paramref name="rawBody"/> on EVERY request (a 400 may
    /// trigger the one reload-retry). The refresh must fail with the typed
    /// <see cref="TokenEndpointException"/> carrying <paramref name="status"/>, leave both
    /// records byte-identical, keep <paramref name="mustEcho"/> in the error for diagnosis,
    /// never carry <paramref name="mustNotEcho"/> anywhere in the chain (ToString, Message,
    /// Body, inner exceptions), and hold no text in the chain longer than
    /// <c>max_error_chars</c> however large the body. With <paramref name="expectedBody"/> the
    /// Body must EQUAL it; with <paramref name="cutWellFormed"/> the Body must be well-formed
    /// (no unpaired surrogate), start with the raw body's first 1023 chars and end with "…".
    /// </summary>
    [Theory]
    [MemberData(nameof(RejectedTokenResponses))]
    public async Task RejectedTokenResponseFailsTypedRedactedAndCapped(
        string name, int status, string rawBody, string mustEcho, string? mustNotEcho,
        string refreshToken, string clientSecret, string? expectedBody, bool cutWellFormed)
    {
        var table = RejectedTable();
        var maxErrorChars = table.GetProperty("max_error_chars").GetInt32();
        AssertNeedleInPayload(name, Encoding.UTF8.GetBytes(rawBody), mustNotEcho);

        var credentials = new ClientCredentials { ClientId = "cid", ClientSecret = clientSecret };
        var tokens = new Tokens { AccessToken = "at-original", RefreshToken = refreshToken, ExpiresAt = 0 };
        using var temp = new TempStore();
        temp.Store.SaveCredentials(credentials);
        temp.Store.SaveTokens(tokens);
        var tokensBefore = File.ReadAllBytes(temp.Store.TokensPath);
        var credsBefore = File.ReadAllBytes(temp.Store.CredentialsPath);

        var endpoint = new MockTokenEndpoint(_ =>
        {
            var content = new ByteArrayContent(Encoding.UTF8.GetBytes(rawBody));
            content.Headers.ContentType = new MediaTypeHeaderValue("application/json");
            return new HttpResponseMessage((HttpStatusCode)status) { Content = content };
        });
        using var manager = new TokenManager(temp.Store, credentials, tokens,
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

        var thrown = await Record.ExceptionAsync(() => manager.ForceRefreshAsync());
        Assert.True(thrown is TokenEndpointException,
            $"case {name}: a rejected ({status}) token response must fail with the typed "
            + "TokenEndpointException, got "
            + (thrown is null ? "a SUCCESSFUL refresh" : $"untyped {thrown.GetType().Name}"));
        var e = (TokenEndpointException)thrown!;
        Assert.True(e.StatusCode == status,
            $"case {name}: the typed error must carry the endpoint's status {status}, got {e.StatusCode}");

        // The harness is not vacuous: the refresh really SENT the submitted secrets.
        Assert.True(endpoint.Calls >= 1, $"case {name}: the token endpoint was never called");
        var sent = endpoint.Bodies[0];
        Assert.Contains("refresh_token=" + Uri.EscapeDataString(refreshToken), sent);
        Assert.Contains("client_secret=" + Uri.EscapeDataString(clientSecret), sent);

        Assert.True(e.Message.Contains(mustEcho),
            $"case {name}: the typed error must keep the server's diagnostic \"{mustEcho}\" "
            + $"(must_echo), got Message: {e.Message}");
        Assert.True(e.Body.Contains(mustEcho),
            $"case {name}: TokenEndpointException.Body must keep \"{mustEcho}\" (must_echo), got: {e.Body}");
        AssertDoesNotEcho(name, e, mustNotEcho);
        if (expectedBody is not null)
        {
            Assert.True(e.Body == expectedBody,
                $"case {name}: TokenEndpointException.Body must be exactly the first "
                + $"{TokenManager.MaxDiagnosticBodyChars} chars plus \"\u2026\" (expected_body); got "
                + $"{e.Body.Length} chars ending \"{e.Body.Substring(Math.Max(0, e.Body.Length - 8))}\"");
        }
        if (cutWellFormed)
        {
            var keep = TokenManager.MaxDiagnosticBodyChars - 1;
            Assert.True(IsWellFormedUtf16(e.Body),
                $"case {name}: the capped Body split a character (an unpaired surrogate) — "
                + "cut_well_formed requires well-formed Unicode");
            Assert.True(e.Body.StartsWith(rawBody.Substring(0, keep), StringComparison.Ordinal),
                $"case {name}: the capped Body must start with the raw body's first {keep} chars (cut_well_formed)");
            Assert.True(e.Body.EndsWith("\u2026", StringComparison.Ordinal),
                $"case {name}: the capped Body must end with \"\u2026\" (cut_well_formed)");
        }
        foreach (var link in ExceptionChain(e))
        {
            var texts = new List<(string What, string Text)>
            {
                ("ToString()", link.ToString()),
                ("Message", link.Message),
            };
            if (link is TokenEndpointException endpointError)
            {
                texts.Add(("Body", endpointError.Body));
            }
            foreach (var (what, text) in texts)
            {
                Assert.True(text.Length <= maxErrorChars,
                    $"case {name}: {link.GetType().Name}.{what} is {text.Length} chars, over the "
                    + $"fixture's max_error_chars {maxErrorChars} (the body must be capped)");
            }
        }

        Assert.True(
            tokensBefore.SequenceEqual(File.ReadAllBytes(temp.Store.TokensPath)),
            $"case {name}: tokens.json must be byte-identical after a rejected refresh");
        Assert.True(
            credsBefore.SequenceEqual(File.ReadAllBytes(temp.Store.CredentialsPath)),
            $"case {name}: credentials.json must be byte-identical after a rejected refresh");
    }

    /// <summary>True when <paramref name="s"/> holds no unpaired surrogate (every high surrogate
    /// is followed by a low one, and every low one preceded by a high one).</summary>
    private static bool IsWellFormedUtf16(string s)
    {
        for (var i = 0; i < s.Length; i++)
        {
            if (char.IsHighSurrogate(s[i]))
            {
                if (i + 1 >= s.Length || !char.IsLowSurrogate(s[i + 1]))
                {
                    return false;
                }
                i++;
            }
            else if (char.IsLowSurrogate(s[i]))
            {
                return false;
            }
        }
        return true;
    }

    // --- 1b. implementation-defined token responses ------------------------------------------

    /// <summary>One (name, base64 of the exact 200 body bytes) pair per fixture case (<see cref="BodyBase64"/>).</summary>
    public static TheoryData<string, string> ImplementationDefinedTokenResponses()
    {
        var data = new TheoryData<string, string>();
        foreach (var c in Fixture().GetProperty("implementation_defined_token_responses").EnumerateArray())
        {
            data.Add(c.GetProperty("name").GetString()!, BodyBase64(c));
        }
        return data;
    }

    /// <summary>
    /// 2xx bodies where companions may legitimately differ (a leading UTF-8 BOM, duplicate
    /// keys, nesting past the parser's depth limit, an integral float expires_in, an
    /// upper-case key). Seeded exactly like the hostile harness, the refresh must land in ONE
    /// of the two sound outcomes: SUCCEED and persist a whole, loadable record with
    /// access_token = at-refreshed, refresh_token = rt-refreshed or the prior stored one (never
    /// empty — a duplicate key's "" must not win), and expires_at within
    /// [t0 + 3600, t1 + 3600]; or FAIL with the typed <see cref="TokenEndpointException"/>
    /// carrying the 2xx status and leave tokens.json / credentials.json byte-identical. An
    /// untyped exception, a wrong persisted access token, or a half-written store fails,
    /// naming the case. Either way it is exactly one endpoint call (a 2xx is not a 400).
    /// </summary>
    [Theory]
    [MemberData(nameof(ImplementationDefinedTokenResponses))]
    public async Task ImplementationDefinedTokenResponseSucceedsOrFailsTypedCleanly(string name, string bodyBase64)
    {
        using var temp = new TempStore();
        temp.Store.SaveCredentials(Credentials());
        temp.Store.SaveTokens(OriginalTokens());
        var tokensBefore = File.ReadAllBytes(temp.Store.TokensPath);
        var credsBefore = File.ReadAllBytes(temp.Store.CredentialsPath);

        var endpoint = new MockTokenEndpoint(_ => OkBytes(bodyBase64));
        using var manager = new TokenManager(temp.Store, Credentials(), OriginalTokens(),
            handler: endpoint, tokenUrl: "http://token.invalid/oauth/token");

        var t0 = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        var thrown = await Record.ExceptionAsync(() => manager.ForceRefreshAsync());
        var t1 = DateTimeOffset.UtcNow.ToUnixTimeSeconds();
        Assert.True(endpoint.Calls == 1, $"case {name}: want exactly one endpoint call, got {endpoint.Calls}");
        Assert.True(
            credsBefore.SequenceEqual(File.ReadAllBytes(temp.Store.CredentialsPath)),
            $"case {name}: credentials.json must be byte-identical after a refresh");

        if (thrown is null)
        {
            // Accepted: the persisted record must be whole and carry the refreshed Bearer.
            Tokens? persisted;
            try
            {
                persisted = temp.Store.LoadTokens();
            }
            catch (StoreFormatException e)
            {
                throw new Xunit.Sdk.XunitException(
                    $"case {name}: an accepted refresh left a half-written tokens.json: {e.Message}");
            }
            Assert.True(persisted is not null, $"case {name}: an accepted refresh must persist a token record");
            Assert.True(persisted!.AccessToken == "at-refreshed",
                $"case {name}: an accepted refresh must persist access_token \"at-refreshed\", got \"{persisted.AccessToken}\"");
            var priorRefresh = OriginalTokens().RefreshToken;
            Assert.True(persisted.RefreshToken == "rt-refreshed" || persisted.RefreshToken == priorRefresh,
                $"case {name}: an accepted refresh must persist refresh_token \"rt-refreshed\" or the "
                + $"prior \"{priorRefresh}\" (never empty), got \"{persisted.RefreshToken}\"");
            Assert.True(persisted.ExpiresAt >= t0 + 3600 && persisted.ExpiresAt <= t1 + 3600,
                $"case {name}: an accepted refresh must persist expires_at = refresh time + 3600 "
                + $"(within [{t0 + 3600}, {t1 + 3600}]), got {persisted.ExpiresAt}");
            return;
        }

        // Rejected: only the typed error is sound, and the store must be untouched.
        Assert.True(thrown is TokenEndpointException,
            $"case {name}: an implementation-defined 2xx must either succeed or fail with the typed "
            + $"TokenEndpointException, got untyped {thrown.GetType().Name}: {thrown.Message}");
        Assert.True(((TokenEndpointException)thrown).StatusCode == 200,
            $"case {name}: the typed error must carry the 2xx status, got {((TokenEndpointException)thrown).StatusCode}");
        Assert.True(
            tokensBefore.SequenceEqual(File.ReadAllBytes(temp.Store.TokensPath)),
            $"case {name}: a rejected refresh must leave tokens.json byte-identical");
    }

    // --- 2. hostile store files ---------------------------------------------------------------

    /// <summary>
    /// A hostile store case's file as the exact bytes to write, base64-encoded (so the theory
    /// row stays a plain string): <c>content_base64</c> decoded (bytes JSON can't hold, e.g.
    /// invalid UTF-8) or <c>content</c> as UTF-8 text. EXACTLY ONE of the two, or loading the
    /// fixture fails naming the case (mirrors <see cref="BodyBase64"/>).
    /// </summary>
    private static string ContentBase64(JsonElement c)
    {
        var name = c.GetProperty("name").GetString();
        string[] keys = ["content", "content_base64"];
        var given = keys.Where(k => c.TryGetProperty(k, out _)).ToList();
        if (given.Count != 1)
        {
            throw new InvalidOperationException(
                $"fixture case {name}: must give exactly one of content/content_base64, "
                + $"got [{string.Join(", ", given)}]");
        }
        return given[0] == "content_base64"
            ? c.GetProperty("content_base64").GetString()!
            : Convert.ToBase64String(Encoding.UTF8.GetBytes(c.GetProperty("content").GetString()!));
    }

    /// <summary>One (name, record file, base64 of the exact file bytes, optional must_not_echo) row per fixture case.</summary>
    public static TheoryData<string, string, string, string?> HostileStoreFiles()
    {
        var data = new TheoryData<string, string, string, string?>();
        foreach (var c in Fixture().GetProperty("hostile_store_files").EnumerateArray())
        {
            data.Add(
                c.GetProperty("name").GetString()!,
                c.GetProperty("file").GetString()!,
                ContentBase64(c),
                MustNotEcho(c));
        }
        return data;
    }

    /// <summary>Loads <paramref name="file"/> from <paramref name="store"/>, returning the record (or null).</summary>
    private static object? LoadStoreFile(string name, TokenStore store, string file) => file switch
    {
        "tokens.json" => store.LoadTokens(),
        "credentials.json" => store.LoadCredentials(),
        _ => throw new InvalidOperationException($"case {name}: fixture names an unknown store file {file}"),
    };

    /// <summary>
    /// Every hostile store file must fail its load with the typed
    /// <see cref="StoreFormatException"/> — never a default-filled record that makes
    /// is-authenticated lie (System.Text.Json is strict here: `required` members reject
    /// partial records, and wrong-typed fields like a NUMBER client_id are never coerced),
    /// never an untyped crash. A case's <c>must_not_echo</c> string (the store holds secrets)
    /// must appear nowhere in the error chain.
    /// </summary>
    [Theory]
    [MemberData(nameof(HostileStoreFiles))]
    public void HostileStoreFileFailsTyped(string name, string file, string contentBase64, string? mustNotEcho)
    {
        var content = Convert.FromBase64String(contentBase64);
        AssertNeedleInPayload(name, content, mustNotEcho);
        using var temp = new TempStore();
        // WriteAllBytes, not WriteAllText: a content_base64 case's invalid UTF-8 must reach
        // the loader untouched (a string round-trip would have replaced it with U+FFFD).
        File.WriteAllBytes(Path.Combine(temp.Dir, file), content);

        object? loaded = null;
        var thrown = Record.Exception(() => loaded = LoadStoreFile(name, temp.Store, file));
        Assert.True(thrown is StoreFormatException,
            $"case {name}: a hostile {file} must fail with the typed StoreFormatException, got "
            + (thrown is null ? $"a loaded record ({loaded ?? "null"})" : $"untyped {thrown.GetType().Name}"));
        Assert.True(thrown!.Message.Contains(file),
            $"case {name}: the typed error must name the offending record {file}");
        AssertDoesNotEcho(name, thrown, mustNotEcho);
    }

    // --- 2b. implementation-defined store files -----------------------------------------------

    /// <summary>One (name, record file, exact file content, expected record JSON) row per fixture case.</summary>
    public static TheoryData<string, string, string, string> ImplementationDefinedStoreFiles()
    {
        var data = new TheoryData<string, string, string, string>();
        foreach (var c in Fixture().GetProperty("implementation_defined_store_files").EnumerateArray())
        {
            data.Add(
                c.GetProperty("name").GetString()!,
                c.GetProperty("file").GetString()!,
                c.GetProperty("content").GetString()!,
                c.GetProperty("expected").GetRawText());
        }
        return data;
    }

    /// <summary>
    /// Store contents a parser may accept or reject (nesting past its depth limit inside an
    /// unknown field). Loading must land in ONE of the two sound outcomes: return a record
    /// whose fields are EXACTLY the fixture's <c>expected</c> values (every field it names),
    /// or fail with the typed <see cref="StoreFormatException"/> naming the record — never an
    /// untyped crash (a raw JsonException/InvalidOperationException escaping the loader).
    /// </summary>
    [Theory]
    [MemberData(nameof(ImplementationDefinedStoreFiles))]
    public void ImplementationDefinedStoreFileLoadsExactlyOrFailsTyped(string name, string file, string content, string expectedJson)
    {
        using var expectedDoc = JsonDocument.Parse(expectedJson);
        var expected = expectedDoc.RootElement;
        using var temp = new TempStore();
        File.WriteAllText(Path.Combine(temp.Dir, file), content);

        object? loaded = null;
        var thrown = Record.Exception(() => loaded = LoadStoreFile(name, temp.Store, file));
        if (thrown is not null)
        {
            Assert.True(thrown is StoreFormatException,
                $"case {name}: an implementation-defined {file} must either load exactly or fail with "
                + $"the typed StoreFormatException, got untyped {thrown.GetType().Name}: {thrown.Message}");
            Assert.True(thrown.Message.Contains(file),
                $"case {name}: the typed error must name the offending record {file}");
            return;
        }

        Assert.True(loaded is not null, $"case {name}: an accepted {file} must load a record, got null");
        // Round-trip the loaded record through the shared wire format, then compare EVERY
        // field the fixture's `expected` names — no field left to chance.
        using var actualDoc = JsonDocument.Parse(JsonSerializer.Serialize(loaded, loaded!.GetType()));
        var fields = expected.EnumerateObject().ToList();
        Assert.True(fields.Count > 0, $"fixture case {name}: expected names no fields");
        foreach (var field in fields)
        {
            Assert.True(actualDoc.RootElement.TryGetProperty(field.Name, out var actual),
                $"case {name}: the loaded record has no {field.Name}");
            Assert.True(actual.GetRawText() == field.Value.GetRawText(),
                $"case {name}: loaded {field.Name} must be exactly {field.Value.GetRawText()}, got {actual.GetRawText()}");
        }
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
        // The refresh must have SENT the prior refresh token (read from the fixture): a
        // companion refreshing from anything else would be burning some other rotation.
        var sent = FormField(endpoint.Bodies.Single(), "refresh_token");
        Assert.True(sent == prior.RefreshToken,
            $"case {name}: the refresh request must send the prior refresh_token \"{prior.RefreshToken}\", sent \"{sent}\"");
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

    /// <summary>
    /// The single value of <paramref name="field"/> in an x-www-form-urlencoded body (null
    /// when absent); a repeated field fails, so the assertion cannot pick an arbitrary copy.
    /// </summary>
    private static string? FormField(string form, string field)
    {
        var values = form.Split('&')
            .Select(pair => pair.Split(['='], 2))
            .Where(kv => WebUtility.UrlDecode(kv[0]) == field)
            .Select(kv => kv.Length == 2 ? WebUtility.UrlDecode(kv[1]) : "")
            .ToList();
        Assert.True(values.Count <= 1, $"form field {field} was sent {values.Count} times");
        return values.SingleOrDefault();
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
