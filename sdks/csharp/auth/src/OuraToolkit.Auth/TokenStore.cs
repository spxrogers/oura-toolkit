using System.Text.Json;

namespace OuraToolkit.Auth;

/// <summary>
/// Persistent credential store at the fixed, invocation-independent per-platform path
/// shared with every other oura-toolkit consumer (the Rust CLI, the MCP server, the other
/// language companions): <c>$XDG_CONFIG_HOME/oura-toolkit/</c> (falling back to
/// <c>~/.config/oura-toolkit/</c>) on Unix/macOS, <c>%LOCALAPPDATA%\oura-toolkit\</c> on
/// Windows (Local, not Roaming — roaming profiles sync <c>%APPDATA%</c> off the machine).
/// Empty or RELATIVE environment values are ignored (a relative base would make secret
/// placement depend on the process cwd, breaking invocation independence).
///
/// Two records live in the store dir, with snake_case JSON matching the Rust companion:
/// <list type="bullet">
/// <item><c>credentials.json</c> — <see cref="ClientCredentials"/> (the user's own OAuth
/// app id + secret; exists from <c>oura auth setup</c> onward).</item>
/// <item><c>tokens.json</c> — <see cref="Tokens"/> (access/refresh/expiry/scope; rewritten
/// on every refresh rotation).</item>
/// </list>
///
/// File hygiene: on Unix, records are created 0600 (and the dir 0700) and every write is
/// atomic (uniquely named temp file + rename), so concurrent writers can never mix bytes —
/// last writer wins. On Windows the chmods are no-ops; protection relies on
/// <c>%LOCALAPPDATA%</c>'s user-private ACLs.
///
/// Cross-process coordination: <see cref="AcquireLockAsync"/> takes an exclusive lock on a
/// <c>.lock</c> file in the store dir; <see cref="TokenManager"/> holds it across its
/// reload → refresh → persist critical section. NOTE: whether this lock and the Rust CLI's
/// <c>flock</c> exclude each other is a runtime implementation detail (on Linux .NET
/// happens to map <c>FileShare.None</c> onto <c>flock</c>, but that is NOT a contract) —
/// the cross-runtime guarantee is the PROTOCOL, not the lock: every refresher re-reads the
/// store first, adopts an already-performed rotation instead of re-burning it, and retries
/// a refresh 400 once against freshly reloaded disk state (see <see cref="TokenManager"/>).
/// </summary>
public sealed class TokenStore
{
    /// <summary>
    /// The locked config-directory name (CLAUDE.md → NAMING), identical under every install
    /// path on every platform.
    /// </summary>
    public const string AppDirName = "oura-toolkit";

    /// <summary>Store at the default per-platform config location.</summary>
    public TokenStore()
        : this(ResolveConfigDir(Environment.GetEnvironmentVariable, !IsPosix))
    {
    }

    /// <summary>
    /// True when NOT running on Windows (i.e. a POSIX host whose file modes matter). Carried
    /// as a single guard so the two legs differ only in HOW they answer: net7+ uses
    /// <c>OperatingSystem.IsWindows()</c> — which the platform-compatibility analyzer also
    /// recognizes (via the <c>[UnsupportedOSPlatformGuard("windows")]</c> on this member)
    /// as a guard for the <c>[UnsupportedOSPlatform("windows")]</c> UnixFileMode APIs —
    /// while netstandard2.0 (no <c>OperatingSystem</c> type) uses <c>RuntimeInformation</c>.
    /// </summary>
#if !NETSTANDARD2_0
    [System.Runtime.Versioning.UnsupportedOSPlatformGuard("windows")]
#endif
    private static bool IsPosix =>
#if NETSTANDARD2_0
        !System.Runtime.InteropServices.RuntimeInformation.IsOSPlatform(
            System.Runtime.InteropServices.OSPlatform.Windows);
#else
        !OperatingSystem.IsWindows();
#endif

    /// <summary>Store rooted at an explicit directory (used by tests).</summary>
    public TokenStore(string directory) => Directory = directory;

    /// <summary>The store directory.</summary>
    public string Directory { get; }

    /// <summary>Path of the client-credentials record.</summary>
    public string CredentialsPath => Path.Combine(Directory, "credentials.json");

    /// <summary>Path of the token record.</summary>
    public string TokensPath => Path.Combine(Directory, "tokens.json");

    private string LockPath => Path.Combine(Directory, ".lock");

    private static readonly JsonSerializerOptions JsonOptions = new() { WriteIndented = true };

    /// <summary>Load the client credentials, or null if <c>auth setup</c> has never run.</summary>
    public ClientCredentials? LoadCredentials() => LoadRecord<ClientCredentials>(CredentialsPath);

    /// <summary>Persist the client credentials (0600 on Unix, atomic).</summary>
    public void SaveCredentials(ClientCredentials credentials)
    {
        EnsureDirectory();
        WriteSecure(CredentialsPath, JsonSerializer.SerializeToUtf8Bytes(credentials, JsonOptions));
    }

    /// <summary>Load the tokens, or null if no login has succeeded yet.</summary>
    public Tokens? LoadTokens() => LoadRecord<Tokens>(TokensPath);

    /// <summary>
    /// Persist the tokens (0600 on Unix, atomic). Callers refreshing MUST persist the
    /// rotated refresh token (Oura invalidates the previous one), and MUST do so under
    /// <see cref="AcquireLockAsync"/> as <see cref="TokenManager"/> does — the atomic write
    /// prevents corruption, not rotation burns.
    /// </summary>
    public void SaveTokens(Tokens tokens)
    {
        EnsureDirectory();
        WriteSecure(TokensPath, JsonSerializer.SerializeToUtf8Bytes(tokens, JsonOptions));
    }

    /// <summary>
    /// Take an exclusive lock on the store, waiting (polling) until the current holder
    /// releases it. Hold the returned handle across a reload → refresh → persist critical
    /// section; dispose to release. The hold duration is bounded by the token endpoint's
    /// hard timeout (<see cref="TokenManager.TokenEndpointTimeout"/>).
    /// </summary>
    public async Task<StoreLock> AcquireLockAsync(CancellationToken cancellationToken = default)
    {
        while (true)
        {
            if (TryAcquireLock() is { } held)
            {
                return held;
            }
            await Task.Delay(TimeSpan.FromMilliseconds(25), cancellationToken).ConfigureAwait(false);
        }
    }

    /// <summary>
    /// Non-blocking variant of <see cref="AcquireLockAsync"/>: null if another holder
    /// currently has the lock.
    /// </summary>
    public StoreLock? TryAcquireLock()
    {
        EnsureDirectory();
        try
        {
            return new StoreLock(OpenExclusiveLock(LockPath));
        }
        catch (IOException)
        {
            // Held by someone else (sharing violation). Genuine I/O failures (permissions,
            // missing dir) also land here in rare cases, but the retry loop's caller keeps
            // a bounded critical section, and EnsureDirectory above rules out the dir.
            return null;
        }
    }

    /// <summary>
    /// Open the <c>.lock</c> file with exclusive sharing (<see cref="FileShare.None"/>), the
    /// primitive the advisory lock is built on. The lock file is secret-free, so its 0600 on
    /// Unix is hygiene, not a security guarantee. net7+: <c>FileStreamOptions.UnixCreateMode</c>
    /// sets 0600 at creation. netstandard2.0: the classic constructor (no mode support), then a
    /// best-effort <c>chmod</c> — a brief non-0600 window is acceptable for a contentless lock.
    /// </summary>
    private static FileStream OpenExclusiveLock(string path)
    {
#if NETSTANDARD2_0
        var stream = new FileStream(path, FileMode.OpenOrCreate, FileAccess.ReadWrite, FileShare.None);
        if (IsPosix)
        {
            PosixInterop.TryChmod(path, PosixInterop.Mode0600);
        }
        return stream;
#else
        var options = new FileStreamOptions
        {
            Mode = FileMode.OpenOrCreate,
            Access = FileAccess.ReadWrite,
            // Exclusivity: no other handle (in this or any other process honoring the
            // sharing mode) may hold the file while we do.
            Share = FileShare.None,
        };
        if (IsPosix)
        {
            options.UnixCreateMode = UnixFileMode.UserRead | UnixFileMode.UserWrite;
        }
        return new FileStream(path, options);
#endif
    }

    private T? LoadRecord<T>(string path) where T : class
    {
        byte[] bytes;
        try
        {
            bytes = File.ReadAllBytes(path);
        }
        catch (FileNotFoundException)
        {
            return null;
        }
        catch (DirectoryNotFoundException)
        {
            return null;
        }
        catch (UnauthorizedAccessException e)
        {
            // The record path is not a readable file — most commonly it is itself a directory
            // (File.ReadAllBytes on a directory throws this on Unix). Surface it as a typed
            // store-format error, not a raw IO exception leaking to callers.
            throw new StoreFormatException(path, e);
        }
        try
        {
            return JsonSerializer.Deserialize<T>(bytes)
                ?? throw new StoreFormatException(path, new JsonException("record is JSON null"));
        }
        catch (JsonException e)
        {
            throw new StoreFormatException(path, e);
        }
    }

    private void EnsureDirectory()
    {
        System.IO.Directory.CreateDirectory(Directory);
        if (IsPosix)
        {
            // 0700, every time (CreateDirectory does not re-mode an existing dir).
#if NETSTANDARD2_0
            PosixInterop.TryChmod(Directory, PosixInterop.Mode0700);
#else
            File.SetUnixFileMode(
                Directory,
                UnixFileMode.UserRead | UnixFileMode.UserWrite | UnixFileMode.UserExecute);
#endif
        }
    }

    /// <summary>
    /// Atomic write with owner-only perms: a UNIQUELY named temp file in the same directory
    /// (created 0600 on Unix), flushed to disk, then renamed into place. The unique name
    /// means two concurrent writers can never truncate each other's in-flight temp file;
    /// the rename makes the outcome last-writer-wins, never a corrupt mix.
    /// </summary>
    private static void WriteSecure(string path, byte[] data)
    {
        var dir = Path.GetDirectoryName(path)
            ?? throw new InvalidOperationException("store paths always have a parent dir");
        var temp = Path.Combine(dir, $".tmp-{Guid.NewGuid():N}");
        try
        {
            using (var stream = CreateExclusiveOwnerOnly(temp))
            {
                // Write(byte[], int, int) — not the Write(ReadOnlySpan<byte>) overload, which
                // is absent on netstandard2.0; this form is identical on every TFM.
                stream.Write(data, 0, data.Length);
                stream.Flush(flushToDisk: true);
            }
            ReplaceAtomically(temp, path);
        }
        catch
        {
            try
            {
                File.Delete(temp);
            }
            catch (IOException)
            {
                // Best-effort cleanup; the original error is what matters.
            }
            throw;
        }
    }

    /// <summary>
    /// Create a brand-new temp file for writing, owner-only (0600) on Unix, failing if it
    /// somehow already exists. net7+: <c>FileStreamOptions.UnixCreateMode</c> pins the mode at
    /// creation. netstandard2.0: <see cref="PosixInterop.CreateExclusive0600"/> does the same
    /// via <c>open(O_CREAT|O_EXCL, 0600)</c> — NO wider window than the modern leg. On
    /// Windows/.NET Framework, <c>FileMode.CreateNew</c> and the user-private ACLs of
    /// <c>%LOCALAPPDATA%</c> provide the protection.
    /// </summary>
    private static FileStream CreateExclusiveOwnerOnly(string temp)
    {
#if NETSTANDARD2_0
        if (IsPosix)
        {
            return PosixInterop.CreateExclusive0600(temp);
        }
        return new FileStream(temp, FileMode.CreateNew, FileAccess.Write, FileShare.None);
#else
        var options = new FileStreamOptions
        {
            Mode = FileMode.CreateNew,
            Access = FileAccess.Write,
            Share = FileShare.None,
        };
        if (IsPosix)
        {
            options.UnixCreateMode = UnixFileMode.UserRead | UnixFileMode.UserWrite;
        }
        return new FileStream(temp, options);
#endif
    }

    /// <summary>
    /// Atomically move <paramref name="temp"/> onto <paramref name="path"/>, replacing any
    /// existing record last-writer-wins. net5+: <c>File.Move(overwrite: true)</c>.
    /// netstandard2.0 on Unix: <c>rename(2)</c> (atomic replace). netstandard2.0 on
    /// Windows/.NET Framework: the 2-arg <c>File.Move</c> refuses an existing destination, so
    /// delete-then-move — a brief window covered by <c>%LOCALAPPDATA%</c>'s ACLs, the same
    /// place that leg relies on for confidentiality anyway.
    /// </summary>
    private static void ReplaceAtomically(string temp, string path)
    {
#if NETSTANDARD2_0
        if (IsPosix)
        {
            PosixInterop.Rename(temp, path);
            return;
        }
        if (File.Exists(path))
        {
            File.Delete(path);
        }
        File.Move(temp, path);
#else
        File.Move(temp, path, overwrite: true);
#endif
    }

    /// <summary>
    /// Testable core of the default-path resolution (injected env lookup — mirrors the Rust
    /// companion's <c>config_dir_from</c>; no racy global env mutation in tests, and both
    /// platform branches are exercisable everywhere via <paramref name="isWindows"/>).
    ///
    /// Empty and RELATIVE values are treated as absent (XDG spec: relative values should be
    /// ignored) — a relative base would make where secrets land depend on the process cwd.
    /// </summary>
    internal static string ResolveConfigDir(Func<string, string?> env, bool isWindows)
    {
        string? Usable(string key)
        {
            var value = env(key);
            // value is non-null in the true branch (guarded by !IsNullOrEmpty); the `!` is for
            // the netstandard2.0 BCL, whose IsNullOrEmpty lacks [NotNullWhen(false)].
            return !string.IsNullOrEmpty(value) && IsAbsolute(value!, isWindows) ? value : null;
        }

        var separator = isWindows ? '\\' : '/';
        if (isWindows)
        {
            // Local, NOT roaming (%APPDATA%): roaming profiles sync to file servers and
            // profile backups at logoff, which would copy plaintext OAuth secrets off the
            // machine.
            return Usable("LOCALAPPDATA") is { } local
                ? $"{local.TrimEnd(separator)}{separator}{AppDirName}"
                : throw new NoConfigDirException(isWindows: true);
        }
        if (Usable("XDG_CONFIG_HOME") is { } xdg)
        {
            return $"{xdg.TrimEnd(separator)}{separator}{AppDirName}";
        }
        return Usable("HOME") is { } home
            ? $"{home.TrimEnd(separator)}{separator}.config{separator}{AppDirName}"
            : throw new NoConfigDirException(isWindows: false);
    }

    /// <summary>
    /// Platform-aware absolute-path check, independent of the RUNNING platform so both
    /// branches are unit-testable on any CI leg (Path.IsPathRooted answers only for the
    /// current OS). Windows: drive-rooted (<c>C:\</c> or <c>C:/</c>) or UNC (<c>\\</c>);
    /// drive-relative forms like <c>C:foo</c> or bare <c>\foo</c> do not qualify — they
    /// still depend on per-process state. Unix: leading <c>/</c>.
    /// </summary>
    private static bool IsAbsolute(string path, bool isWindows)
    {
        if (!isWindows)
        {
            return path.Length > 0 && path[0] == '/';
        }
        if (path.StartsWith(@"\\", StringComparison.Ordinal))
        {
            return true; // UNC
        }
        return path.Length >= 3
            && IsAsciiLetter(path[0])
            && path[1] == ':'
            && (path[2] == '\\' || path[2] == '/');
    }

    // char.IsAsciiLetter is net7+; this ASCII-only check is TFM-agnostic and equivalent here
    // (drive letters are ASCII).
    private static bool IsAsciiLetter(char c) =>
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z');
}

/// <summary>
/// An exclusive lock on the store, released on dispose (or process exit). Bind it for the
/// whole critical section — releasing early re-opens the rotation race.
/// </summary>
public sealed class StoreLock : IDisposable
{
    private readonly FileStream _file;

    internal StoreLock(FileStream file) => _file = file;

    /// <summary>Releases the lock.</summary>
    public void Dispose() => _file.Dispose();
}
