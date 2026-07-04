using Xunit;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// Direct execution coverage for <see cref="PosixInterop"/> — the libc leg that gives the
/// netstandard2.0 build its atomic-0600 create. A modern test host never loads the
/// netstandard2.0 asset, so without these tests the security-critical create path would ship
/// unexercised (see the DESIGN note on PosixInterop). Because the helper is always compiled,
/// it runs and is asserted here on net8.0 + net10.0.
/// </summary>
public class PosixInteropTests
{
    [Fact]
    public void CreateExclusive0600CreatesAnOwnerOnlyFileAtCreation()
    {
        if (OperatingSystem.IsWindows())
        {
            return; // libc path is Unix-only; Windows relies on %LOCALAPPDATA% ACLs.
        }

        var dir = Directory.CreateTempSubdirectory("oura-posix-mode-").FullName;
        try
        {
            var path = Path.Combine(dir, "secret");
            using (var stream = PosixInterop.CreateExclusive0600(path))
            {
                var payload = new byte[] { 1, 2, 3 };
                stream.Write(payload, 0, payload.Length);
                stream.Flush(flushToDisk: true);
            }

            // Exactly 0600 — no world/group bits ever visible, because open(2) set the mode AT
            // creation. Break-verify: make PosixInterop.Mode0600 = 0644 and this fails.
            Assert.Equal(
                UnixFileMode.UserRead | UnixFileMode.UserWrite,
                File.GetUnixFileMode(path));
            Assert.Equal(new byte[] { 1, 2, 3 }, File.ReadAllBytes(path));
        }
        finally
        {
            Directory.Delete(dir, recursive: true);
        }
    }

    [Fact]
    public void CreateExclusive0600IsExclusiveASecondCreateFails()
    {
        if (OperatingSystem.IsWindows())
        {
            return;
        }

        var dir = Directory.CreateTempSubdirectory("oura-posix-excl-").FullName;
        try
        {
            var path = Path.Combine(dir, "secret");
            using (PosixInterop.CreateExclusive0600(path))
            {
                // first create succeeds
            }

            // The O_EXCL flag makes a second exclusive create on the same path fail — proving
            // the create cannot follow a pre-planted symlink or clobber a racing temp file.
            // Break-verify: drop O_EXCL from the open(2) flags and this second create succeeds,
            // failing the test.
            Assert.Throws<IOException>(() => PosixInterop.CreateExclusive0600(path));
        }
        finally
        {
            Directory.Delete(dir, recursive: true);
        }
    }

    [Fact]
    public void RenameAtomicallyReplacesAnExistingDestination()
    {
        if (OperatingSystem.IsWindows())
        {
            return;
        }

        var dir = Directory.CreateTempSubdirectory("oura-posix-rename-").FullName;
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
