using Xunit;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// The leg-integrity guard for issue #61: every test host must load the library asset that
/// matches its own runtime family. The whole point of the Mono/.NET Framework leg (net472,
/// run by <c>just sdk-test-csharp-netstandard</c>) is to execute the <c>netstandard2.0</c>
/// asset — its <c>#if NETSTANDARD2_0</c> store/transport branches and the records/init/required
/// Polyfills — which no modern .NET test host can load. If that leg ever resolved a modern
/// asset instead (a broken TFM set, a stray ref), it would "pass" while proving nothing. This
/// asserts the loaded asset, so the leg cannot silently test the wrong code.
/// </summary>
public class BuildInfoTests
{
    [Fact]
    public void TestHostLoadsTheAssetMatchingItsRuntime()
    {
        // The expected asset is a compile-time fact of THIS test assembly's TFM, cross-checked
        // against the const the LIBRARY asset actually baked in. net472 resolves the library's
        // netstandard2.0 build (net472 ⊇ netstandard2.0, and the library has no net4x target).
#if NET472
        Assert.Equal("netstandard2.0", BuildInfo.TargetFramework);
#elif NET8_0
        Assert.Equal("net8.0", BuildInfo.TargetFramework);
#elif NET10_0
        Assert.Equal("net10.0", BuildInfo.TargetFramework);
#else
        Assert.Fail(
            "test assembly compiled for an unexpected TFM — teach BuildInfoTests (and the "
                + "library's BuildInfo) about it before adding a target");
#endif
    }
}
