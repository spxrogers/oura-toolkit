package com.ouratoolkit.auth;

/**
 * Base checked exception for the auth companion. Subclasses mirror the Rust reference
 * implementation's {@code AuthError} variants; callers own the remediation UX (e.g. a CLI
 * maps {@link NotAuthenticatedException} to "run {@code oura auth login}").
 */
public class AuthException extends Exception {
    private static final long serialVersionUID = 1L;

    public AuthException(String message) {
        super(message);
    }

    public AuthException(String message, Throwable cause) {
        super(message, cause);
    }
}
