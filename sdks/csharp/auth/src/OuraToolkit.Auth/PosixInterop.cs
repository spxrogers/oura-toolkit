using System;
using System.IO;
using System.Runtime.InteropServices;

namespace OuraToolkit.Auth;

/// <summary>
/// Minimal libc P/Invoke used by the <b>netstandard2.0</b> leg of <see cref="TokenStore"/> to
/// reproduce the file-hygiene guarantees the netstandard2.0 BCL cannot express: an ATOMIC
/// owner-only (0600) create-and-write with no world-readable TOCTOU window, and an atomic
/// overwrite-rename. On net7+ the store instead uses in-box
/// <c>UnixFileMode</c>/<c>FileStreamOptions.UnixCreateMode</c>/<c>File.Move(overwrite)</c> and
/// never touches this class in production.
///
/// The create path writes through the raw <c>open(2)</c> file descriptor via <c>write(2)</c> +
/// <c>fsync(2)</c> and NEVER constructs a managed <c>FileStream</c> from that fd, nor reopens
/// the file by path: Mono/Xamarin (a real netstandard2.0 target) cannot wrap a foreign libc fd
/// in a <c>FileStream</c> (it validates handles against its own IO table and throws), and
/// reopening by path would introduce a TOCTOU the fd-only design specifically avoids. The fd
/// stays in this class's hands from create to close.
///
/// DESIGN — this helper is <b>ALWAYS COMPILED on every target framework</b> (no
/// <c>#if TFM</c> fence): only <see cref="TokenStore"/>'s <c>#if NETSTANDARD2_0</c> SELECTS
/// between it and the BCL API. That is deliberate and load-bearing for testability: a modern
/// (net8/net10) test host can never load the netstandard2.0 asset, so a naive
/// <c>#if NETSTANDARD2_0</c> around this code would hide the security-critical create path
/// from ALL tests. Because it compiles and runs everywhere, it is unit-tested directly on the
/// modern Unix legs (written file is exactly 0600, and a second exclusive create fails —
/// proving O_EXCL); the netstandard2.0/Mono leg (`just sdk-test-csharp-netstandard`, #61) runs
/// the SAME tests on a runtime that actually loads this asset.
/// </summary>
internal static class PosixInterop
{
    /// <summary>Owner read+write (octal 0600).</summary>
    public const int Mode0600 = 0x180;

    /// <summary>Owner read+write+execute (octal 0700) — the store directory.</summary>
    public const int Mode0700 = 0x1C0;

    // open(2) flags. O_WRONLY is 0x1 on Linux and the BSDs/macOS alike, but O_CREAT/O_EXCL
    // differ between Linux (0x40/0x80) and the BSD lineage — macOS AND FreeBSD/NetBSD/OpenBSD
    // all share 0x200/0x800 — so resolve those at runtime. We treat "BSD-family" as the macOS
    // branch and everything else as Linux; the only non-macOS BSD that could reach here is a
    // Mono host on FreeBSD, which shares macOS's values, so IsBsd (not just IsMac) picks the
    // 0x200/0x800 pair. (This leg only executes on legacy/Mono runtimes; modern .NET selects
    // the net8/net10 asset and never calls here.)
    private const int O_WRONLY = 0x1;

    private static bool IsBsd =>
        RuntimeInformation.IsOSPlatform(OSPlatform.OSX)
        || RuntimeInformation.IsOSPlatform(OSPlatform.Create("FREEBSD"))
        || RuntimeInformation.IsOSPlatform(OSPlatform.Create("NETBSD"))
        || RuntimeInformation.IsOSPlatform(OSPlatform.Create("OPENBSD"));

    private static int O_CREAT => IsBsd ? 0x200 : 0x40;

    private static int O_EXCL => IsBsd ? 0x800 : 0x80;

    // Interrupted syscall — errno 4 on Linux and the whole BSD/macOS lineage alike. write(2) and
    // fsync(2) interrupted by a signal BEFORE any progress return -1/EINTR and must be retried
    // (the in-box FileStream path retries this internally on Unix). close(2) is NOT retried on
    // EINTR: POSIX leaves the fd state unspecified afterward, and on Linux the fd is already
    // closed, so a retry could close an unrelated reused descriptor.
    private const int EINTR = 4;

    // Path marshalling (#72): CharSet.Ansi marshals the managed string with the platform's
    // NARROW encoding — the SAME one the .NET BCL uses for its own path APIs
    // (Directory.CreateDirectory, File.ReadAllBytes, File.Move). On CoreCLR/Unix that is always
    // UTF-8; on Mono it follows the process encoding (default UTF-8, or MONO_EXTERNAL_ENCODINGS).
    // Because BOTH this interop and the BCL resolve a given managed path string to the same bytes,
    // a record rename(2) writes here is exactly the one a later File.ReadAllBytes finds — even
    // under a non-ASCII directory. Do NOT "fix" this to a forced UTF-8 byte[]: on Mono under a
    // non-UTF-8 process encoding that would make the interop DISAGREE with the BCL that created
    // the path (a real mismatch), trading a non-bug for one. Guarded by
    // TokenStoreTests.StoreRoundTripsUnderANonAsciiDirectory (runs on the net472/Mono leg too).
    [DllImport("libc", SetLastError = true, CharSet = CharSet.Ansi, BestFitMapping = false)]
    private static extern int open(string path, int flags, int mode);

    [DllImport("libc", SetLastError = true, CharSet = CharSet.Ansi, BestFitMapping = false)]
    private static extern int rename(string oldpath, string newpath);

    [DllImport("libc", SetLastError = true, CharSet = CharSet.Ansi, BestFitMapping = false)]
    private static extern int chmod(string path, int mode);

    [DllImport("libc", SetLastError = true)]
    private static extern int close(int fd);

    // write(2)/fsync(2): the buffer is passed as a pinned pointer so we can advance it across a
    // partial write. IntPtr count/return so the 64-bit ssize_t/size_t are not truncated.
    [DllImport("libc", SetLastError = true)]
    private static extern IntPtr write(int fd, IntPtr buf, IntPtr count);

    [DllImport("libc", SetLastError = true)]
    private static extern int fsync(int fd);

    /// <summary>
    /// Atomically create <paramref name="path"/> with mode 0600 (failing if it already exists),
    /// write <paramref name="data"/> to it, flush it to disk, and close it — all through the raw
    /// <c>open(2)</c> descriptor. The mode is applied AT creation by <c>open(2)</c> (the umask
    /// can only clear bits, and 0600 has none to clear), so there is never a window in which the
    /// file is more permissive than 0600 — the security equivalent of net7's
    /// <c>FileStreamOptions.UnixCreateMode</c>. <c>O_EXCL</c> guarantees exclusive creation: no
    /// following a pre-planted symlink, no clobbering a racing writer's uniquely-named temp file.
    /// The fd is never wrapped in a managed <c>FileStream</c> nor reopened by path (see the class
    /// remarks), so there is no fd-adoption or reopen-TOCTOU surface.
    /// </summary>
    /// <exception cref="IOException">
    /// The file exists, or <c>open(2)</c>/<c>write(2)</c>/<c>fsync(2)</c> otherwise failed.
    /// </exception>
    public static void WriteNewExclusive0600(string path, byte[] data)
    {
        var fd = open(path, O_WRONLY | O_CREAT | O_EXCL, Mode0600);
        if (fd < 0)
        {
            var errno = Marshal.GetLastWin32Error();
            throw new IOException($"exclusive 0600 create of '{path}' failed (errno {errno})");
        }

        // Pin the buffer so write(2) sees a stable address we can advance across partial writes.
        var pin = GCHandle.Alloc(data, GCHandleType.Pinned);
        try
        {
            var basePtr = pin.AddrOfPinnedObject();
            var offset = 0;
            while (offset < data.Length)
            {
                // Retry a signal-interrupted write; any other error is fatal (errno captured
                // immediately after the failing call, before any other P/Invoke can clobber it).
                long n;
                int errno;
                do
                {
                    n = (long)write(fd, IntPtr.Add(basePtr, offset), (IntPtr)(data.Length - offset));
                    errno = n < 0 ? Marshal.GetLastWin32Error() : 0;
                }
                while (n < 0 && errno == EINTR);

                if (n < 0)
                {
                    throw new IOException($"write to '{path}' failed (errno {errno})");
                }

                if (n == 0)
                {
                    // write(2) returns 0 only for a zero-length request; with bytes remaining it
                    // means no progress — bail rather than spin.
                    throw new IOException($"write to '{path}' made no progress");
                }

                offset += (int)n;
            }

            // Durability parity with the modern leg's Flush(flushToDisk: true); retry EINTR.
            int frc, ferrno;
            do
            {
                frc = fsync(fd);
                ferrno = frc != 0 ? Marshal.GetLastWin32Error() : 0;
            }
            while (frc != 0 && ferrno == EINTR);

            if (frc != 0)
            {
                throw new IOException($"fsync of '{path}' failed (errno {ferrno})");
            }
        }
        finally
        {
            pin.Free();
            close(fd);
        }
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
