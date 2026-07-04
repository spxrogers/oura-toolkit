using Xunit;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// Attack tests for the "no secrets in logs" invariant: the hostile path is any stray
/// <c>ToString()</c> — direct, via string interpolation, or via a collection dump — landing
/// in a log line. Records synthesize a ToString that prints EVERY member, so a removed
/// override silently reintroduces the leak; these tests are the tripwire.
/// </summary>
public class RedactionTests
{
    [Fact]
    public void ClientCredentialsToStringRedactsTheSecret()
    {
        var rendered = Fixtures.Credentials.ToString();
        Assert.DoesNotContain("SECRET-CS-789", rendered);
        Assert.Contains("cid", rendered); // the non-secret id stays visible for diagnostics
        Assert.Contains("[REDACTED]", rendered);
    }

    [Fact]
    public void TokensToStringRedactsBothTokens()
    {
        var rendered = Fixtures.SampleTokens.ToString();
        Assert.DoesNotContain("SECRET-AT-123", rendered);
        Assert.DoesNotContain("SECRET-RT-456", rendered);
        Assert.Contains("[REDACTED]", rendered);
        // Non-secret fields remain visible.
        Assert.Contains("4102444800", rendered);
        Assert.Contains("daily personal", rendered);
    }

    [Fact]
    public void StringInterpolationCannotLeakSecretsEither()
    {
        // Interpolation and Object.ToString are the same path only if no IFormattable
        // sneaks in — pin the interpolated output too.
        var log = $"credentials={Fixtures.Credentials} tokens={Fixtures.SampleTokens}";
        Assert.DoesNotContain("SECRET-CS-789", log);
        Assert.DoesNotContain("SECRET-AT-123", log);
        Assert.DoesNotContain("SECRET-RT-456", log);
    }
}
