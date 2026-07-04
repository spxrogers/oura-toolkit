using System;
using System.IO;
using System.Runtime.InteropServices;
using Microsoft.Win32.SafeHandles;

namespace OuraToolkit.Auth;

/// <summary>
/// Minimal libc P/Invoke used by the <b>netstandard2.0</b> leg of <see cref="TokenStore"/> to
/// reproduce the file-hygiene guarantees the netstandard2.0 BCL cannot express: an ATOMIC
/// owner-only (0600) create with no world-readable TOCTOU window, and an atomic
/// overwrite-rename. On net7+ the store instead uses in-box
/// <c>UnixFileMode</c>/<c>FileStreamOptions.UnixCreateMode</c>/<c>File.Move(overwrite)</c> and
/// never touches this class in production.
///
/// DESIGN — this helper is <b>ALWAYS COMPILED on every target framework</b> (no
/// <c>#if TFM</c> fence): only <see cref="TokenStore"/>'s <c>#if NETSTANDARD2_0</c> SELECTS
/// between it and the BCL API. That is deliberate and load-bearing for testability: a modern
/// (net8/net10) test host can never load the netstandard2.0 asset, so a naive
/// <c>#if NETSTANDARD2_0</c> around this code would hide the security-critical create path
/// from ALL tests. Because it compiles and runs everywhere, it is unit-tested directly on the
/// modern Unix legs (created file is exactly 0600, and a second exclusive create fails —
/// proving O_EXCL).
/// </summary>
internal static class PosixInterop
{
    /// <summary>Owner read+write (octal 0600).</summary>
    public const int Mode0600 = 0x180;

    /// <summary>Owner read+write+execute (octal 0700) — the store directory.</summary>
    public const int Mode0700 = 0x1C0;

    // open(2) flags. O_WRONLY is 0x1 on Linux and the BSDs/macOS alike, but O_CREAT/O_EXCL
    // differ between Linux (0x40/0x80) and BSD-derived macOS (0x200/0x800), so resolve those
    // at runtime. (This leg only executes on legacy/Mono runtimes; modern .NET selects the
    // net8/net10 asset and never calls here.)
    private const int O_WRONLY = 0x1;

    private static bool IsMac => RuntimeInformation.IsOSPlatform(OSPlatform.OSX);

    private static int O_CREAT => IsMac ? 0x200 : 0x40;

    private static int O_EXCL => IsMac ? 0x800 : 0x80;

    [DllImport("libc", SetLastError = true, CharSet = CharSet.Ansi, BestFitMapping = false)]
    private static extern int open(string path, int flags, int mode);

    [DllImport("libc", SetLastError = true, CharSet = CharSet.Ansi, BestFitMapping = false)]
    private static extern int rename(string oldpath, string newpath);

    [DllImport("libc", SetLastError = true, CharSet = CharSet.Ansi, BestFitMapping = false)]
    private static extern int chmod(string path, int mode);

    /// <summary>
    /// Atomically create <paramref name="path"/> for writing with mode 0600, failing if it
    /// already exists. The mode is applied AT creation by <c>open(2)</c> (the umask can only
    /// clear bits, and 0600 has none to clear), so there is never a window in which the file
    /// is more permissive than 0600 — the security equivalent of net7's
    /// <c>FileStreamOptions.UnixCreateMode</c>. <c>O_EXCL</c> guarantees exclusive creation:
    /// no following a pre-planted symlink, no clobbering a racing writer's uniquely-named
    /// temp file.
    /// </summary>
    /// <exception cref="IOException">The file exists, or <c>open(2)</c> otherwise failed.</exception>
    public static FileStream CreateExclusive0600(string path)
    {
        var fd = open(path, O_WRONLY | O_CREAT | O_EXCL, Mode0600);
        if (fd < 0)
        {
            var errno = Marshal.GetLastWin32Error();
            throw new IOException($"exclusive 0600 create of '{path}' failed (errno {errno})");
        }

        // The SafeFileHandle owns the fd and closes it on FileStream disposal.
        var handle = new SafeFileHandle((IntPtr)fd, ownsHandle: true);
        return new FileStream(handle, FileAccess.Write);
    }

    /// <summary>
    /// Atomically rename <paramref name="source"/> onto <paramref name="destination"/> via
    /// <c>rename(2)</c>, which POSIX guarantees is an atomic replace on the same filesystem —
    /// the netstandard2.0 stand-in for <c>File.Move(src, dest, overwrite: true)</c>.
    /// </summary>
    /// <exception cref="IOException"><c>rename(2)</c> failed.</exception>
    public static void Rename(string source, string destination)
    {
        if (rename(source, destination) != 0)
        {
            var errno = Marshal.GetLastWin32Error();
            throw new IOException(
                $"atomic rename '{source}' -> '{destination}' failed (errno {errno})");
        }
    }

    /// <summary>
    /// Best-effort <c>chmod(2)</c> for artifacts whose mode cannot be pinned at open time —
    /// the exclusive <c>.lock</c> file (secret-free) and re-moding an already-existing store
    /// directory to 0700.
    /// </summary>
    public static void TryChmod(string path, int mode) => chmod(path, mode);
}
