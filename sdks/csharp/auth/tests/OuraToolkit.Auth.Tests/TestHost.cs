using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// Small portability helpers so the ONE auth test suite compiles and RUNS on every target the
/// library ships — including issue #61's net472/Mono leg, which loads and executes the
/// library's netstandard2.0 asset. A handful of BCL APIs the tests reach for directly are
/// net7+-only (<c>UnixFileMode</c>/<c>File.GetUnixFileMode</c>,
/// <c>Directory.CreateTempSubdirectory</c>) or net5+-only (<c>OperatingSystem.IsWindows</c>);
/// these give the SAME answers on net472 via portable equivalents, so no assertion is weakened
/// — the modern legs stay byte-for-byte identical (the helpers just wrap the in-box APIs there).
/// </summary>
internal static class TestHost
{
    /// <summary>Portable OS check (<c>OperatingSystem.IsWindows</c> is net5+).</summary>
    public static bool IsWindows { get; } = RuntimeInformation.IsOSPlatform(OSPlatform.Windows);

    /// <summary>A fresh unique temp directory (<c>Directory.CreateTempSubdirectory</c> is net7+).</summary>
    public static string CreateTempDir(string prefix)
    {
        var dir = Path.Combine(Path.GetTempPath(), prefix + Path.GetRandomFileName());
        Directory.CreateDirectory(dir);
        return dir;
    }

    /// <summary>
    /// The Unix permission bits of <paramref name="path"/> as an octal-valued int (e.g. 0x180
    /// == 0o600), directly comparable to <see cref="PosixInterop.Mode0600"/> /
    /// <see cref="PosixInterop.Mode0700"/>. The modern legs read them in-box; the net472 leg is
    /// Unix-only by design (its whole reason to exist is running the netstandard2.0 libc path
    /// under Mono), and .NET Framework has no <c>File.GetUnixFileMode</c>, so it shells to
    /// coreutils <c>stat</c>. Fails loudly rather than returning a wrong mode.
    /// </summary>
    public static int UnixPermBits(string path)
    {
#if NET472
        var psi = new ProcessStartInfo("stat")
        {
            Arguments = $"-c %a \"{path}\"",
            RedirectStandardOutput = true,
            UseShellExecute = false,
        };
        using var p = Process.Start(psi)
            ?? throw new InvalidOperationException("could not start `stat`");
        var octal = p.StandardOutput.ReadToEnd().Trim();
        p.WaitForExit();
        if (p.ExitCode != 0 || octal.Length == 0)
        {
            throw new InvalidOperationException($"`stat -c %a` failed for '{path}' (exit {p.ExitCode})");
        }

        return Convert.ToInt32(octal, 8);
#else
        // File.GetUnixFileMode is [UnsupportedOSPlatform("windows")]; every caller guards with
        // TestHost.IsWindows first, but the analyzer can't see that guard across this helper —
        // suppress CA1416 here rather than annotate (the attribute is not portable to net472).
#pragma warning disable CA1416
        return (int)File.GetUnixFileMode(path);
#pragma warning restore CA1416
#endif
    }
}
