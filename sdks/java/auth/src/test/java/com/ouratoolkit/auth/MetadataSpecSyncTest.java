package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

/**
 * "Do NOT hardcode the authorize/token URLs or scopes — read them from the spec"
 * (CLAUDE.md): a Maven module has no build-time spec-read codegen like the Rust
 * companion's build.rs, so {@link OuraOAuthMetadata}'s constants are transcriptions and
 * THIS test is the mechanism that keeps them true to the vendored spec. A spec refresh
 * that moves an endpoint or renames a scope fails here, in CI.
 *
 * <p>Monorepo-only by nature (walks up to the repo root), like the Rust crates'
 * bundled-spec tests.
 */
class MetadataSpecSyncTest {

    /** Repo root: the nearest ancestor holding both the justfile and spec/openapi.json. */
    private static Path repoRoot() {
        Path dir = Paths.get("").toAbsolutePath();
        while (dir != null) {
            if (Files.isRegularFile(dir.resolve("justfile"))
                    && Files.isRegularFile(dir.resolve("spec/openapi.json"))) {
                return dir;
            }
            dir = dir.getParent();
        }
        throw new AssertionError("repo root (justfile + spec/openapi.json) not found above cwd");
    }

    private static JsonNode authorizationCodeFlow() throws IOException {
        JsonNode spec = new ObjectMapper().readTree(
                Files.readAllBytes(repoRoot().resolve("spec/openapi.json")));
        JsonNode flow = spec.at(
                "/components/securitySchemes/OAuth2/flows/authorizationCode");
        assertFalse(flow.isMissingNode(),
                "spec lost its OAuth2 authorizationCode flow — the auth model changed");
        return flow;
    }

    @Test
    void urlsMatchTheSpec() throws IOException {
        JsonNode flow = authorizationCodeFlow();
        assertEquals(flow.get("authorizationUrl").asText(), OuraOAuthMetadata.AUTHORIZE_URL,
                "AUTHORIZE_URL drifted from the vendored spec — update OuraOAuthMetadata");
        assertEquals(flow.get("tokenUrl").asText(), OuraOAuthMetadata.TOKEN_URL,
                "TOKEN_URL drifted from the vendored spec — update OuraOAuthMetadata");
    }

    @Test
    void allScopesMatchTheSpecInSpecOrder() throws IOException {
        JsonNode scopes = authorizationCodeFlow().get("scopes");
        assertNotNull(scopes, "spec flow lost its scopes map");
        List<String> specScopes = new ArrayList<>();
        for (Iterator<String> it = scopes.fieldNames(); it.hasNext(); ) {
            specScopes.add(it.next());
        }
        assertEquals(specScopes, OuraOAuthMetadata.ALL_SCOPES,
                "ALL_SCOPES drifted from the spec's OAuth2 scopes — update OuraOAuthMetadata");
    }

    @Test
    void defaultScopesAreAllScopesMinusEmail() {
        assertFalse(OuraOAuthMetadata.DEFAULT_SCOPES.contains("email"),
                "toolkit policy: never request email by default");
        List<String> expected = new ArrayList<>(OuraOAuthMetadata.ALL_SCOPES);
        expected.remove("email");
        assertEquals(expected, OuraOAuthMetadata.DEFAULT_SCOPES,
                "DEFAULT_SCOPES must be exactly the spec scopes minus email "
                        + "(a silent narrowing would shrink the consent we request)");
        assertTrue(OuraOAuthMetadata.ALL_SCOPES.containsAll(OuraOAuthMetadata.DEFAULT_SCOPES),
                "every default scope must be spec-advertised");
    }
}
