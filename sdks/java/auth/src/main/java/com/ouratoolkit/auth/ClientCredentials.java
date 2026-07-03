package com.ouratoolkit.auth;

import java.util.Objects;

import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * The user's own Oura OAuth application credentials (bring-your-own confidential client).
 *
 * <p>JSON field names ({@code client_id} / {@code client_secret}) are pinned to the Rust
 * companion's serde output — both companions share the SAME on-disk
 * {@code credentials.json}; {@code StoreSchemaTest} enforces the schema with a fixture
 * transcribed from {@code sdks/rust/oura-toolkit-auth/src/store.rs}.
 *
 * <p>{@link #toString()} REDACTS the secret, so stray logging can never leak it (the "no
 * secrets in logs" rule).
 */
public final class ClientCredentials {

    private final String clientId;
    private final String clientSecret;

    @JsonCreator
    public ClientCredentials(
            @JsonProperty(value = "client_id", required = true) String clientId,
            @JsonProperty(value = "client_secret", required = true) String clientSecret) {
        this.clientId = Objects.requireNonNull(clientId, "clientId");
        this.clientSecret = Objects.requireNonNull(clientSecret, "clientSecret");
    }

    @JsonProperty("client_id")
    public String getClientId() {
        return clientId;
    }

    @JsonProperty("client_secret")
    public String getClientSecret() {
        return clientSecret;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) {
            return true;
        }
        if (!(o instanceof ClientCredentials)) {
            return false;
        }
        ClientCredentials that = (ClientCredentials) o;
        return clientId.equals(that.clientId) && clientSecret.equals(that.clientSecret);
    }

    @Override
    public int hashCode() {
        return Objects.hash(clientId, clientSecret);
    }

    /** Redacts the client secret. */
    @Override
    public String toString() {
        return "ClientCredentials{client_id=" + clientId + ", client_secret=[REDACTED]}";
    }
}
