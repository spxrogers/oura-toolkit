package com.ouratoolkit.auth;

/**
 * Transport failure talking to the token endpoint (connect error, TLS failure, or the
 * hard request timeout that bounds how long the store lock can be held — see
 * {@link TokenManager}).
 */
public class TransportException extends AuthException {
    private static final long serialVersionUID = 1L;

    public TransportException(String message, Throwable cause) {
        super(message, cause);
    }
}
