package com.ouratoolkit.auth;

/**
 * The token endpoint returned a non-2xx response (e.g. a rotated/expired refresh token).
 * Carries the HTTP status and the response body (OAuth error bodies describe the failure;
 * they never echo tokens or the client secret).
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
