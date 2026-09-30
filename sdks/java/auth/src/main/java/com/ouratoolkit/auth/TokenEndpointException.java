package com.ouratoolkit.auth;

/**
 * The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh token).
 * Carries the HTTP status and the response body for diagnosis — with every secret the
 * request submitted (refresh token, client secret) replaced by {@code [REDACTED]} in case
 * the server echoed it, and capped at 1024 characters ("…" appended when cut).
 */
public class TokenEndpointException extends AuthException {
    private static final long serialVersionUID = 1L;

    private final int status;
    private final String body;

    public TokenEndpointException(int status, String body) {
        super("token endpoint returned HTTP " + status + ": " + body);
        this.status = status;
        this.body = body;
    }

    public int getStatus() {
        return status;
    }

    public String getBody() {
        return body;
    }
}
