package com.ouratoolkit.auth;

/**
 * Unchecked wrapper for {@link AuthException}, thrown from contexts that cannot declare
 * checked exceptions — the generated {@code ApiClient}'s request interceptor is a
 * {@code java.util.function.Consumer}. Unwrap via {@link #getCause()}.
 */
public class UncheckedAuthException extends RuntimeException {
    private static final long serialVersionUID = 1L;

    public UncheckedAuthException(AuthException cause) {
        super(cause.getMessage(), cause);
    }

    @Override
    public synchronized AuthException getCause() {
        return (AuthException) super.getCause();
    }
}
