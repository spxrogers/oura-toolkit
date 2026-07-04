package com.ouratoolkit.auth;

import java.util.List;

/**
 * OAuth2 metadata from the vendored Oura OpenAPI spec — NOT invented here.
 *
 * <p>Every value below is transcribed from
 * {@code spec/openapi.json → components.securitySchemes.OAuth2.flows.authorizationCode}
 * (the repo's single source of truth). {@code MetadataSpecSyncTest} walks up to the repo
 * root and asserts each constant against the spec, so a spec refresh that moves an
 * endpoint or renames a scope fails CI here instead of silently drifting. (The Rust
 * companion reads the same block at build time; a Maven module has no equivalent
 * spec-read codegen step, so the sync test IS the mechanism.)
 */
public final class OuraOAuthMetadata {

    private OuraOAuthMetadata() {}

    /** {@code flows.authorizationCode.authorizationUrl} from the spec. */
    public static final String AUTHORIZE_URL = "https://cloud.ouraring.com/oauth/authorize";

    /** {@code flows.authorizationCode.tokenUrl} from the spec. */
    public static final String TOKEN_URL = "https://api.ouraring.com/oauth/token";

    /** All 8 scopes the spec advertises, in spec order. */
    public static final List<String> ALL_SCOPES = List.of(
            "email",
            "personal",
            "daily",
            "heartrate",
            "workout",
            "tag",
            "session",
            "spo2Daily");

    /**
     * Scopes the toolkit requests by default: everything except {@code email} (CLAUDE.md
     * policy, matching the Rust companion's {@code default_scopes()}). This is toolkit
     * POLICY, not spec metadata — the spec-advertised set is {@link #ALL_SCOPES}.
     */
    public static final List<String> DEFAULT_SCOPES = List.of(
            "personal",
            "daily",
            "heartrate",
            "workout",
            "tag",
            "session",
            "spo2Daily");
}
