package com.ouratoolkit.auth;

/**
 * Filesystem or format failure reading/writing the token store, wrapped so
 * {@link TokenManager} callers deal with a single checked exception family.
 */
public class StoreException extends AuthException {
    private static final long serialVersionUID = 1L;

    public StoreException(String message, Throwable cause) {
        super(message, cause);
    }
}
