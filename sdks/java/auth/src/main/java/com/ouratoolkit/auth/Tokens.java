package com.ouratoolkit.auth;

import java.time.Instant;
import java.util.Objects;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * The persisted OAuth token set. Client credentials live in their own record
 * ({@link ClientCredentials}), not here.
 *
 * <p>JSON schema is pinned to the Rust companion's serde output: {@code access_token},
 * {@code refresh_token}, {@code expires_at} (absolute Unix seconds), and OPTIONAL
 * {@code scope} / {@code token_type} which are OMITTED when null (serde's
 * {@code skip_serializing_if}) — enforced by {@code StoreSchemaTest}.
 *
 * <p>Oura ROTATES the refresh token on every refresh and invalidates the previous value —
 * always persist the newly returned one or the next refresh 400s.
 *
 * <p>{@link #toString()} REDACTS both token fields.
 */
@JsonInclude(JsonInclude.Include.NON_NULL)
public final class Tokens {

    private final String accessToken;
    private final String refreshToken;
    private final long expiresAt;
    private final String scope;
    private final String tokenType;

    @JsonCreator
    public Tokens(
            @JsonProperty(value = "access_token", required = true) String accessToken,
            @JsonProperty(value = "refresh_token", required = true) String refreshToken,
            @JsonProperty(value = "expires_at", required = true) long expiresAt,
            @JsonProperty("scope") String scope,
            @JsonProperty("token_type") String tokenType) {
        this.accessToken = Objects.requireNonNull(accessToken, "accessToken");
        this.refreshToken = Objects.requireNonNull(refreshToken, "refreshToken");
        this.expiresAt = expiresAt;
        this.scope = scope;
        this.tokenType = tokenType;
    }

    @JsonProperty("access_token")
    public String getAccessToken() {
        return accessToken;
    }

    /** The rotated-on-every-refresh refresh token; persist the newest one, always. */
    @JsonProperty("refresh_token")
    public String getRefreshToken() {
        return refreshToken;
    }

    /** Absolute expiry as a Unix timestamp (seconds). */
    @JsonProperty("expires_at")
    public long getExpiresAt() {
        return expiresAt;
    }

    /** Space-separated granted scopes, or null if the endpoint omitted them. */
    @JsonProperty("scope")
    public String getScope() {
        return scope;
    }

    @JsonProperty("token_type")
    public String getTokenType() {
        return tokenType;
    }

    /** True if the access token is expired (or within {@code skewSecs} of expiring). */
    public boolean isExpired(long skewSecs) {
        return Instant.now().getEpochSecond() + skewSecs >= expiresAt;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (!(o instanceof Tokens)) {
            return false;
        }
        Tokens that = (Tokens) o;
        return expiresAt == that.expiresAt
                && accessToken.equals(that.accessToken)
                && refreshToken.equals(that.refreshToken)
                && Objects.equals(scope, that.scope)
                && Objects.equals(tokenType, that.tokenType);
    }

    @Override
    public int hashCode() {
        return Objects.hash(accessToken, refreshToken, expiresAt, scope, tokenType);
    }

    /** Redacts both token fields. */
    @Override
    public String toString() {
        return "Tokens{access_token=[REDACTED], refresh_token=[REDACTED], expires_at="
                + expiresAt + ", scope=" + scope + ", token_type=" + tokenType + "}";
    }
}
