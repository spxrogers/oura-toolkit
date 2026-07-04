package com.ouratoolkit.auth;

/**
 * No tokens are available (no login has ever succeeded). The message deliberately embeds
 * no remediation hint — callers own the UX.
 */
public class NotAuthenticatedException extends AuthException {
    private static final long serialVersionUID = 1L;

    public NotAuthenticatedException() {
        super("not authenticated (no tokens stored)");
    }
}
