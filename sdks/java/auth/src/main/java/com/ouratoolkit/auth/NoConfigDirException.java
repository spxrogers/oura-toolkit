package com.ouratoolkit.auth;

/**
 * The fixed per-platform config directory could not be resolved from the environment.
 * The message names the env vars for the platform branch that failed (never Unix vars on
 * Windows or vice versa).
 */
public class NoConfigDirException extends AuthException {
    private static final long serialVersionUID = 1L;

    public NoConfigDirException(String message) {
        super(message);
    }
}
