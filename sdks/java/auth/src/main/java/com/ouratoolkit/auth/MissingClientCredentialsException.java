package com.ouratoolkit.auth;

/**
 * Tokens exist but the client-credentials record is missing, so a refresh is impossible:
 * Oura is a confidential client and the token endpoint requires {@code client_id} +
 * {@code client_secret}. Callers own the remediation hint.
 */
public class MissingClientCredentialsException extends AuthException {
    private static final long serialVersionUID = 1L;

    public MissingClientCredentialsException() {
        super("no client credentials stored");
    }
}
