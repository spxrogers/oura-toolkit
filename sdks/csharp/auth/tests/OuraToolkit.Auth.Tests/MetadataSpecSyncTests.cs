using System.Text.Json;
using Xunit;

namespace OuraToolkit.Auth.Tests;

/// <summary>
/// Pins <see cref="OAuthMetadata"/> to the vendored spec (CLAUDE.md: do NOT hardcode the
/// authorize/token URLs or scopes — read them from the spec). The C# companion carries the
/// values as constants; THIS test is what makes them spec-read rather than hardcoded: it
/// re-parses <c>spec/openapi.json</c>'s <c>components.securitySchemes</c> and fails on any
/// drift, so a spec refresh that moves an endpoint or renames a scope breaks CI, not users.
///
/// Monorepo-only by nature (walks up to the repo root, like the Rust crates' bundled-spec
/// tests); the shipped library has no test dependency on the repo layout.
/// </summary>
public class MetadataSpecSyncTests
{
    private static string RepoRoot()
    {
        var dir = new DirectoryInfo(AppContext.BaseDirectory);
        while (dir is not null)
        {
            if (File.Exists(Path.Combine(dir.FullName, "justfile"))
                && File.Exists(Path.Combine(dir.FullName, "spec", "openapi.json")))
            {
                return dir.FullName;
            }
            dir = dir.Parent!;
        }
        throw new InvalidOperationException(
            "repo root (justfile + spec/openapi.json) not found above the test binary");
    }

    private static JsonElement AuthorizationCodeFlow(JsonDocument spec) =>
        spec.RootElement
            .GetProperty("components")
            .GetProperty("securitySchemes")
            .GetProperty("OAuth2")
            .GetProperty("flows")
            .GetProperty("authorizationCode");

    [Fact]
    public void UrlsMatchTheVendoredSpec()
    {
        using var spec = JsonDocument.Parse(
            File.ReadAllText(Path.Combine(RepoRoot(), "spec", "openapi.json")));
        var flow = AuthorizationCodeFlow(spec);

        // The spec is the source of truth; equality is symmetric, and xunit's analyzer
        // wants the constant in the 'expected' slot.
        Assert.Equal(OAuthMetadata.AuthorizeUrl, flow.GetProperty("authorizationUrl").GetString());
        Assert.Equal(OAuthMetadata.TokenUrl, flow.GetProperty("tokenUrl").GetString());
    }

    [Fact]
    public void AllScopesMatchTheVendoredSpecExactly()
    {
        using var spec = JsonDocument.Parse(
            File.ReadAllText(Path.Combine(RepoRoot(), "spec", "openapi.json")));
        var specScopes = AuthorizationCodeFlow(spec)
            .GetProperty("scopes")
            .EnumerateObject()
            .Select(p => p.Name)
            .ToHashSet();

        Assert.Equal(8, specScopes.Count); // the shape CLAUDE.md documents
        Assert.Equal(specScopes, OAuthMetadata.AllScopes.ToHashSet());
    }

    [Fact]
    public void DefaultScopesAreExactlyAllScopesMinusEmail()
    {
        // The toolkit's consent policy: everything except email — a silent narrowing here
        // would quietly shrink what users are asked to grant (mirrors metadata.rs).
        var expected = OAuthMetadata.AllScopes.Where(s => s != "email").ToHashSet();
        Assert.Equal(expected, OAuthMetadata.DefaultScopes.ToHashSet());
        Assert.Equal(7, OAuthMetadata.DefaultScopes.Count);
        Assert.DoesNotContain("email", OAuthMetadata.DefaultScopes);
    }
}
