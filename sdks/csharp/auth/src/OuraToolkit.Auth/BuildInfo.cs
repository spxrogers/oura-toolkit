namespace OuraToolkit.Auth;

/// <summary>
/// Per-target-framework build marker. Used ONLY by the test suite to assert which library
/// asset a given test host actually loaded at runtime — the modern legs load their own
/// net8.0/net10.0 asset, and the Mono/.NET Framework leg (issue #61) exists solely to load
/// and execute the <c>netstandard2.0</c> asset (the one carrying the <c>#if NETSTANDARD2_0</c>
/// store/transport branches and Polyfills). A misconfigured leg that silently resolved the
/// wrong asset would defeat its own purpose; <c>BuildInfoTests</c> pins this so it cannot.
/// Internal — never part of the public surface (exposed to tests via InternalsVisibleTo).
/// </summary>
internal static class BuildInfo
{
    /// <summary>The target framework this assembly asset was compiled for.</summary>
    public const string TargetFramework =
#if NETSTANDARD2_0
        "netstandard2.0";
#elif NET8_0
        "net8.0";
#elif NET10_0
        "net10.0";
#else
        "unknown";
#endif
}
