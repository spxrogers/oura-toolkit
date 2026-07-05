using Xunit;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// Direct execution coverage for <see cref="PosixInterop"/> — the libc leg that gives the
/// netstandard2.0 build its atomic-0600 create-and-write. A modern test host never loads the
/// netstandard2.0 asset, so without these tests the security-critical create path would ship
/// unexercised (see the DESIGN note on PosixInterop). Because the helper is always compiled,
/// it runs and is asserted here on net8.0 + net10.0; the netstandard2.0/Mono leg
/// (`just sdk-test-csharp-netstandard`, #61) runs the SAME tests where this asset is loaded.
/// </summary>
public class PosixInteropTests
{
    [Fact]
    public void WriteNewExclusive0600WritesAnOwnerOnlyFileAtCreation()
    {
        if (TestHost.IsWindows)
        {
            return; // libc path is Unix-only; Windows relies on %LOCALAPPDATA% ACLs.
        }

        var dir = TestHost.CreateTempDir("oura-posix-mode-");
        try
        {
            var path = Path.Combine(dir, "secret");
            var payload = new byte[] { 1, 2, 3 };
            PosixInterop.WriteNewExclusive0600(path, payload);

            // Exactly 0600 — no world/group bits ever visible, because open(2) set the mode AT
            // creation. The expected value is an INDEPENDENT octal literal, NOT
            // PosixInterop.Mode0600 (asserting against the same constant the create uses would be
            // a tautology). Break-verify: set PosixInterop.Mode0600 = 0644 and this fails.
            Assert.Equal(Convert.ToInt32("600", 8), TestHost.UnixPermBits(path));
            // The bytes went through the raw fd (write(2)), not a FileStream.
            Assert.Equal(payload, File.ReadAllBytes(path));
        }
        finally
        {
            Directory.Delete(dir, recursive: true);
        }
    }

    [Fact]
    public void WriteNewExclusive0600IsExclusiveASecondCreateFails()
    {
        if (TestHost.IsWindows)
        {
            return;
        }

        var dir = TestHost.CreateTempDir("oura-posix-excl-");
        try
        {
            var path = Path.Combine(dir, "secret");
            PosixInterop.WriteNewExclusive0600(path, new byte[] { 9 });

            // The O_EXCL flag makes a second exclusive create on the same path fail — proving
            // the create cannot follow a pre-planted symlink or clobber a racing temp file.
            // Break-verify: drop O_EXCL from the open(2) flags and this second create succeeds,
            // failing the test.
            Assert.Throws<IOException>(
                () => PosixInterop.WriteNewExclusive0600(path, new byte[] { 9 }));
        }
        finally
        {
            Directory.Delete(dir, recursive: true);
        }
    }

    [Fact]
    public void RenameAtomicallyReplacesAnExistingDestination()
    {
        if (TestHost.IsWindows)
        {
            return;
        }

        var dir = TestHost.CreateTempDir("oura-posix-rename-");
        try
        {
            var source = Path.Combine(dir, "src");
            var dest = Path.Combine(dir, "dest");
            File.WriteAllText(dest, "old");
            File.WriteAllText(source, "new");

            // rename(2) atomically overwrites the destination (the netstandard2.0 stand-in for
            // File.Move(overwrite: true)): dest ends up with the new bytes, source is gone.
            PosixInterop.Rename(source, dest);

            Assert.Equal("new", File.ReadAllText(dest));
            Assert.False(File.Exists(source));
        }
        finally
        {
            Directory.Delete(dir, recursive: true);
        }
    }
}
