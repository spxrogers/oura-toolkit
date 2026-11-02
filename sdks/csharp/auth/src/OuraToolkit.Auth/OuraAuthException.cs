namespace OuraToolkit.Auth;

/// <summary>Base type for every error raised by the auth companion.</summary>
public abstract class OuraAuthException : Exception
{
    /// <summary>Creates the exception with a caller-facing message.</summary>
    protected OuraAuthException(string message) : base(message) { }

    /// <summary>Creates the exception wrapping an underlying cause.</summary>
    protected OuraAuthException(string message, Exception inner) : base(message, inner) { }
}

/// <summary>
/// No tokens available. The library deliberately does not embed remediation hints in the
/// message — callers own the UX (the CLI maps this to "run <c>oura auth login</c>").
/// </summary>
public sealed class NotAuthenticatedException : OuraAuthException
{
    /// <summary>Creates the exception.</summary>
    public NotAuthenticatedException() : base("not authenticated (no tokens stored)") { }
}

/// <summary>
/// Tokens exist but the client credentials record is missing, so a refresh is impossible
/// (confidential client: the token endpoint requires client_id + client_secret).
/// </summary>
public sealed class MissingClientCredentialsException : OuraAuthException
{
    /// <summary>Creates the exception.</summary>
    public MissingClientCredentialsException() : base("no client credentials stored") { }
}

/// <summary>Could not resolve the config directory from the platform's environment.</summary>
public sealed class NoConfigDirException : OuraAuthException
{
    /// <summary>Creates the exception with the platform-appropriate variable names.</summary>
    public NoConfigDirException(bool isWindows)
        : base(isWindows
            ? "could not determine the config directory (%LOCALAPPDATA% unset or not an absolute path)"
            : "could not determine the config directory ($XDG_CONFIG_HOME / $HOME unset or not an absolute path)")
    {
    }
}

/// <summary>
/// The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh token).
/// </summary>
public sealed class TokenEndpointException : OuraAuthException
{
    /// <summary>Creates the exception from the endpoint's status and body.</summary>
    public TokenEndpointException(int statusCode, string body)
        : base($"token endpoint returned HTTP {statusCode}: {body}")
    {
        StatusCode = statusCode;
        Body = body;
    }

    /// <summary>HTTP status code from the token endpoint.</summary>
    public int StatusCode { get; }

    /// <summary>
    /// Response body from the token endpoint (server-supplied), for diagnosis only: on a
    /// non-2xx, every secret the request submitted is replaced with <c>[REDACTED]</c> and the
    /// text is capped at 1024 characters (<c>…</c> appended when cut); on a malformed 2xx it is
    /// a fixed, secret-free description instead.
    /// </summary>
    public string Body { get; }
}

/// <summary>
/// The token endpoint could not be reached or did not answer in time — a connection
/// failure, or the hard <see cref="TokenManager.TokenEndpointTimeout"/> elapsing. Distinct
/// from <see cref="TokenEndpointException"/>, which carries an actual HTTP status the server
/// returned; a <see cref="TransportException"/> means no usable response arrived at all.
/// </summary>
public sealed class TransportException : OuraAuthException
{
    /// <summary>Creates the exception wrapping the underlying transport failure.</summary>
    public TransportException(string message, Exception inner) : base(message, inner) { }
}

/// <summary>A stored record failed to parse (corrupt JSON, wrong shape).</summary>
public sealed class StoreFormatException : OuraAuthException
{
    /// <summary>Creates the exception naming the offending record file.</summary>
    public StoreFormatException(string path, Exception inner)
        : base($"token store format error in {path}: {inner.Message}", inner)
    {
    }

    /// <summary>
    /// Creates the exception naming the offending record file with a fixed reason and NO inner
    /// exception — for failures this library detects itself (not valid UTF-8, a JSON null
    /// record), where there is no underlying parser error to chain.
    /// </summary>
    internal StoreFormatException(string path, string reason)
        : base($"token store format error in {path}: {reason}")
    {
    }
}
