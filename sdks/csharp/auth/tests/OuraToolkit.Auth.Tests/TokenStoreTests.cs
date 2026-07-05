using System.Text.Json;
using Xunit;

namespace OuraToolkit.Auth.Tests;

public class TokenStoreTests
{
    // -- On-disk schema: the cross-language store contract ------------------------------------

    /// <summary>
    /// Fixtures transcribed VERBATIM from the Rust companion's serde output (store.rs's
    /// sample_credentials / sample_tokens serialized with serde_json::to_vec_pretty). The
    /// C# companion must parse what the Rust CLI wrote — this is the interop contract.
    /// </summary>
    private const string RustCredentialsJson = """
        {
          "client_id": "cid",
          "client_secret": "SECRET-CS-789"
        }
        """;

    private const string RustTokensJson = """
        {
          "access_token": "SECRET-AT-123",
          "refresh_token": "SECRET-RT-456",
          "expires_at": 4102444800,
          "scope": "daily personal",
          "token_type": "Bearer"
        }
        """;

    [Fact]
    public void ReadsRecordsWrittenByTheRustCompanion()
    {
        using var temp = new TempStore();
        File.WriteAllText(temp.Store.CredentialsPath, RustCredentialsJson);
        File.WriteAllText(temp.Store.TokensPath, RustTokensJson);

        Assert.Equal(Fixtures.Credentials, temp.Store.LoadCredentials());
        Assert.Equal(Fixtures.SampleTokens, temp.Store.LoadTokens());
    }

    [Fact]
    public void WritesRecordsTheRustCompanionCanRead()
    {
        using var temp = new TempStore();
        temp.Store.SaveCredentials(Fixtures.Credentials);
        temp.Store.SaveTokens(Fixtures.SampleTokens);

        // Exact snake_case field names + values, checked on the RAW JSON (not via our own
        // deserializer, which would mask a renamed property by mis-writing AND mis-reading
        // it symmetrically).
        using var credentials = JsonDocument.Parse(File.ReadAllText(temp.Store.CredentialsPath));
        Assert.Equal(
            new[] { "client_id", "client_secret" },
            credentials.RootElement.EnumerateObject().Select(p => p.Name).ToArray());
        Assert.Equal("cid", credentials.RootElement.GetProperty("client_id").GetString());
        Assert.Equal("SECRET-CS-789", credentials.RootElement.GetProperty("client_secret").GetString());

        using var tokens = JsonDocument.Parse(File.ReadAllText(temp.Store.TokensPath));
        Assert.Equal(
            new[] { "access_token", "refresh_token", "expires_at", "scope", "token_type" },
            tokens.RootElement.EnumerateObject().Select(p => p.Name).ToArray());
        Assert.Equal("SECRET-AT-123", tokens.RootElement.GetProperty("access_token").GetString());
        Assert.Equal("SECRET-RT-456", tokens.RootElement.GetProperty("refresh_token").GetString());
        Assert.Equal(4_102_444_800, tokens.RootElement.GetProperty("expires_at").GetInt64());
        Assert.Equal("daily personal", tokens.RootElement.GetProperty("scope").GetString());
        Assert.Equal("Bearer", tokens.RootElement.GetProperty("token_type").GetString());
    }

    [Fact]
    public void NullScopeAndTokenTypeAreOmittedNotWrittenAsNull()
    {
        // Rust: #[serde(skip_serializing_if = "Option::is_none")] — a literal JSON null
        // would round-trip through serde as None but is not what the Rust side writes;
        // keep the bytes convention identical in both directions.
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));

        using var doc = JsonDocument.Parse(File.ReadAllText(temp.Store.TokensPath));
        Assert.Equal(
            new[] { "access_token", "refresh_token", "expires_at" },
            doc.RootElement.EnumerateObject().Select(p => p.Name).ToArray());
    }

    [Fact]
    public void MissingRecordsLoadAsNull()
    {
        using var temp = new TempStore();
        Assert.Null(temp.Store.LoadCredentials());
        Assert.Null(temp.Store.LoadTokens());
    }

    [Fact]
    public void CorruptRecordThrowsATypedFormatError()
    {
        using var temp = new TempStore();
        File.WriteAllText(temp.Store.TokensPath, "{not json");
        var e = Assert.Throws<StoreFormatException>(() => temp.Store.LoadTokens());
        Assert.Contains("tokens.json", e.Message);
    }

    [Fact]
    public void LiteralJsonNullRecordThrowsATypedFormatErrorForBothRecords()
    {
        using var temp = new TempStore();
        File.WriteAllText(temp.Store.TokensPath, "null");
        var tokensError = Assert.Throws<StoreFormatException>(() => temp.Store.LoadTokens());
        Assert.Contains("tokens.json", tokensError.Message);

        File.WriteAllText(temp.Store.CredentialsPath, "null");
        var credsError = Assert.Throws<StoreFormatException>(() => temp.Store.LoadCredentials());
        Assert.Contains("credentials.json", credsError.Message);
    }

    [Fact]
    public void EmptyObjectMissingRequiredFieldsThrowsATypedFormatError()
    {
        using var temp = new TempStore();
        // `{}` omits the required access_token/refresh_token/expires_at — System.Text.Json's
        // `required` enforcement must surface as a typed store error, not a raw JsonException.
        File.WriteAllText(temp.Store.TokensPath, "{}");
        var e = Assert.Throws<StoreFormatException>(() => temp.Store.LoadTokens());
        Assert.Contains("tokens.json", e.Message);
    }

    [Fact]
    public void WrongTypedExpiresAtThrowsATypedFormatError()
    {
        using var temp = new TempStore();
        // expires_at is a number in the contract; a string must not silently coerce or throw raw.
        File.WriteAllText(temp.Store.TokensPath,
            "{\"access_token\":\"a\",\"refresh_token\":\"b\",\"expires_at\":\"not-a-number\"}");
        var e = Assert.Throws<StoreFormatException>(() => temp.Store.LoadTokens());
        Assert.Contains("tokens.json", e.Message);
    }

    [Fact]
    public void RecordPathThatIsADirectoryThrowsATypedFormatError()
    {
        using var temp = new TempStore();
        // A botched store where the record path is a directory: File.ReadAllBytes on a directory
        // throws UnauthorizedAccessException on Unix — it must map to a typed store error, not
        // escape as a raw IO exception.
        System.IO.Directory.CreateDirectory(temp.Store.TokensPath);
        var e = Assert.Throws<StoreFormatException>(() => temp.Store.LoadTokens());
        Assert.Contains("tokens.json", e.Message);
    }

    // -- File hygiene (Unix leg; Windows relies on %LOCALAPPDATA% ACLs) -----------------------

    [Fact]
    public void RecordsAndDirAreOwnerOnlyOnUnix()
    {
        if (TestHost.IsWindows)
        {
            return; // chmod is a no-op there by design; see TokenStore docs.
        }
        using var temp = new TempStore();
        temp.Store.SaveCredentials(Fixtures.Credentials);
        temp.Store.SaveTokens(Fixtures.SampleTokens);

        // Independent octal literals, NOT PosixInterop.Mode0600/Mode0700 — asserting against the
        // same constants the store uses to create these would be a tautology (a Mode0600 = 0644
        // regression would move both sides together and pass). Break-verify: bump either constant.
        Assert.Equal(Convert.ToInt32("600", 8), TestHost.UnixPermBits(temp.Store.CredentialsPath));
        Assert.Equal(Convert.ToInt32("600", 8), TestHost.UnixPermBits(temp.Store.TokensPath));
        Assert.Equal(Convert.ToInt32("700", 8), TestHost.UnixPermBits(temp.Store.Directory));
    }

    // The store must work under a NON-ASCII directory (#72): a store dir whose on-disk name is
    // non-ASCII exercises PosixInterop's libc path marshalling (open/rename/chmod) against the
    // .NET BCL's own path APIs (Directory.CreateDirectory, File.ReadAllBytes). The libc DllImports
    // use CharSet.Ansi, which marshals with the platform-narrow encoding — the SAME one the BCL
    // uses — so the two agree and the record rename(2) writes is exactly the one LoadTokens reads
    // back. (Verified to hold on CoreCLR, Mono, AND Mono under MONO_EXTERNAL_ENCODINGS=ISO-8859-1;
    // forcing UTF-8 in the interop would instead RISK diverging from Mono's own BCL path encoding.)
    // Break-verify: make PosixInterop encode a different path than the BCL (e.g. append a byte in
    // its path helper) and the round-trip below fails — LoadTokens returns null / the wrong record.
    [Fact]
    public void StoreRoundTripsUnderANonAsciiDirectory()
    {
        if (TestHost.IsWindows)
        {
            return; // The concern is Unix libc path marshalling; Windows uses the BCL throughout.
        }

        var dir = TestHost.CreateTempDir("oura-café-日本-");
        try
        {
            var store = new TokenStore(dir);
            store.SaveCredentials(Fixtures.Credentials);
            store.SaveTokens(Fixtures.SampleTokens);

            var creds = store.LoadCredentials();
            var tokens = store.LoadTokens();
            Assert.NotNull(creds);
            Assert.NotNull(tokens);
            // Load returned the SAME record Save wrote — so the libc write and the BCL read
            // resolved the identical non-ASCII path.
            Assert.Equal(Fixtures.Credentials.ClientId, creds!.ClientId);
            Assert.Equal(Fixtures.SampleTokens.AccessToken, tokens!.AccessToken);
            Assert.Equal(Fixtures.SampleTokens.RefreshToken, tokens.RefreshToken);
            // The record really lives under the non-ASCII directory and is still exactly 0600.
            Assert.StartsWith(dir, store.TokensPath);
            Assert.Equal(Convert.ToInt32("600", 8), TestHost.UnixPermBits(store.TokensPath));
        }
        finally
        {
            Directory.Delete(dir, recursive: true);
        }
    }

    [Fact]
    public void SaveReplacesAtomicallyViaAUniquelyNamedTempFile()
    {
        using var temp = new TempStore();
        temp.Store.SaveTokens(Fixtures.Expired("r1"));
        temp.Store.SaveTokens(Fixtures.Expired("r2"));

        Assert.Equal("r2", temp.Store.LoadTokens()!.RefreshToken);
        // No orphaned temp files after successful writes.
        Assert.Empty(System.IO.Directory.GetFiles(temp.Dir, ".tmp-*"));
    }

    // -- Config-dir resolution (injected env; both platform branches run everywhere) ----------

    private static Func<string, string?> Env(params (string Key, string Value)[] pairs) =>
        key => pairs.Where(p => p.Key == key).Select(p => p.Value).FirstOrDefault();

    [Fact]
    public void UnixPrefersXdgConfigHome()
    {
        var dir = TokenStore.ResolveConfigDir(
            Env(("XDG_CONFIG_HOME", "/xdg"), ("HOME", "/home/u")), isWindows: false);
        Assert.Equal("/xdg/oura-toolkit", dir);
    }

    [Fact]
    public void UnixFallsBackToHomeDotConfig()
    {
        var dir = TokenStore.ResolveConfigDir(Env(("HOME", "/home/u")), isWindows: false);
        Assert.Equal("/home/u/.config/oura-toolkit", dir);
    }

    [Theory]
    [InlineData("")]
    [InlineData("relative/config")]
    public void UnixEmptyOrRelativeXdgFallsBackToHome(string bad)
    {
        var dir = TokenStore.ResolveConfigDir(
            Env(("XDG_CONFIG_HOME", bad), ("HOME", "/home/u")), isWindows: false);
        Assert.Equal("/home/u/.config/oura-toolkit", dir);
    }

    [Theory]
    [InlineData("")]
    [InlineData("relative/home")]
    public void UnixEmptyOrRelativeHomeThrows(string bad)
    {
        var e = Assert.Throws<NoConfigDirException>(
            () => TokenStore.ResolveConfigDir(Env(("HOME", bad)), isWindows: false));
        Assert.Contains("$XDG_CONFIG_HOME", e.Message);
    }

    [Fact]
    public void UnixThrowsWhenNeitherIsSet()
    {
        Assert.Throws<NoConfigDirException>(
            () => TokenStore.ResolveConfigDir(Env(), isWindows: false));
    }

    [Fact]
    public void WindowsUsesLocalAppDataNeverRoaming()
    {
        var dir = TokenStore.ResolveConfigDir(
            Env(("LOCALAPPDATA", @"C:\Users\u\AppData\Local"),
                ("APPDATA", @"C:\Users\u\AppData\Roaming")),
            isWindows: true);
        // Local, not Roaming: roaming profiles sync %APPDATA% off the machine.
        Assert.Equal(@"C:\Users\u\AppData\Local\oura-toolkit", dir);
    }

    [Theory]
    [InlineData("")]
    [InlineData(@"relative\path")]
    [InlineData(@"C:relative-to-drive-cwd")]
    public void WindowsEmptyOrRelativeLocalAppDataThrows(string bad)
    {
        Assert.Throws<NoConfigDirException>(
            () => TokenStore.ResolveConfigDir(Env(("LOCALAPPDATA", bad)), isWindows: true));
    }

    [Fact]
    public void WindowsErrorNamesTheWindowsVariable()
    {
        var e = Assert.Throws<NoConfigDirException>(
            () => TokenStore.ResolveConfigDir(Env(), isWindows: true));
        // Windows users must not be told about Unix env vars.
        Assert.Contains("%LOCALAPPDATA%", e.Message);
        Assert.DoesNotContain("XDG", e.Message);
    }

    // -- Locking -------------------------------------------------------------------------------

    [Fact]
    public void LockIsExclusiveAndReleasedOnDispose()
    {
        using var temp = new TempStore();
        var held = temp.Store.TryAcquireLock();
        Assert.NotNull(held);
        Assert.Null(temp.Store.TryAcquireLock()); // second holder must fail while held
        held!.Dispose();
        using var reacquired = temp.Store.TryAcquireLock();
        Assert.NotNull(reacquired); // free after release
    }

    /// <summary>
    /// Real concurrency for the liveness half: a blocking acquire must WAIT for the
    /// current holder (not fail, not sneak in early) and proceed once released. The
    /// release flag is set before Dispose, so observing it true is only possible if B
    /// genuinely waited out A.
    /// </summary>
    [Fact]
    public async Task BlockingAcquireWaitsForTheCurrentHolder()
    {
        using var temp = new TempStore();
        var released = false;

        var holder = temp.Store.TryAcquireLock();
        Assert.NotNull(holder);

        var waiter = Task.Run(async () =>
        {
            using var acquired = await temp.Store.AcquireLockAsync();
            // The acquire must not complete while the holder still has the lock.
            Assert.True(Volatile.Read(ref released),
                "AcquireLockAsync completed while another holder still had the lock");
        });

        await Task.Delay(200); // give the waiter time to (wrongly) sneak in if it can
        Assert.False(waiter.IsCompleted, "waiter finished while the lock was held");

        Volatile.Write(ref released, true);
        holder!.Dispose();
        await waiter; // rethrows the inner assert if the waiter cheated
    }
}
