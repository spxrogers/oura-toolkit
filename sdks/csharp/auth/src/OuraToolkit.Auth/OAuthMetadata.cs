namespace OuraToolkit.Auth;

/// <summary>
/// OAuth2 metadata for the Oura API. Nothing here is invented: every value is transcribed
/// from the vendored spec's <c>components.securitySchemes.OAuth2.flows.authorizationCode</c>
/// (<c>spec/openapi.json</c>) and MECHANICALLY pinned to it by
/// <c>MetadataSpecSyncTests</c>, which re-reads the spec and fails on any drift — the C#
/// equivalent of the Rust companion's build-time spec read. Do not edit these by hand
/// without a spec refresh; the sync test is the arbiter.
/// </summary>
public static class OAuthMetadata
{
    /// <summary>The spec's <c>authorizationUrl</c> (interactive consent — CLI territory, not this library's).</summary>
    public const string AuthorizeUrl = "https://cloud.ouraring.com/oauth/authorize";

    /// <summary>The spec's <c>tokenUrl</c> (code exchange and refresh).</summary>
    public const string TokenUrl = "https://api.ouraring.com/oauth/token";

    /// <summary>Every scope the spec advertises (8).</summary>
    public static readonly IReadOnlyList<string> AllScopes = new[]
    {
        "email",
        "personal",
        "daily",
        "heartrate",
        "workout",
        "tag",
        "session",
        "spo2Daily",
    };

    /// <summary>
    /// Scopes the toolkit requests by default: everything except <c>email</c>. This is the
    /// toolkit's policy (mirroring the Rust companion's <c>default_scopes()</c>), not spec
    /// metadata — the spec-advertised set is <see cref="AllScopes"/>. The sync test asserts
    /// this stays exactly AllScopes minus email.
    /// </summary>
    public static readonly IReadOnlyList<string> DefaultScopes = new[]
    {
        "personal",
        "daily",
        "heartrate",
        "workout",
        "tag",
        "session",
        "spo2Daily",
    };
}
