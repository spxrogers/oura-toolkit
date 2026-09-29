package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.PosixFilePermissions;
import java.time.Instant;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.condition.EnabledOnOs;
import org.junit.jupiter.api.condition.OS;
import org.junit.jupiter.api.io.TempDir;

/** Store round-trip, file hygiene (0600/0700, POSIX-gated), and corrupt-record behavior. */
class TokenStoreTest {

    @TempDir
    Path dir;

    private static ClientCredentials sampleCredentials() {
        return new ClientCredentials("cid", "SECRET-CS-789");
    }

    private static Tokens sampleTokens() {
        return new Tokens(
                "SECRET-AT-123", "SECRET-RT-456", 4_102_444_800L, "daily personal", "Bearer");
    }

    @Test
    void bothRecordsRoundTrip() throws Exception {
        TokenStore store = new TokenStore(dir);
        assertTrue(store.loadCredentials().isEmpty(), "no setup yet: credentials absent");
        assertTrue(store.loadTokens().isEmpty(), "no login yet: tokens absent");

        store.saveCredentials(sampleCredentials());
        store.saveTokens(sampleTokens());

        assertEquals(sampleCredentials(), store.loadCredentials().orElseThrow());
        assertEquals(sampleTokens(), store.loadTokens().orElseThrow());
    }

    @Test
    @EnabledOnOs({OS.LINUX, OS.MAC})
    void recordsAreOwnerOnlyAndDirIsPrivate() throws IOException {
        TokenStore store = new TokenStore(dir);
        store.saveCredentials(sampleCredentials());
        store.saveTokens(sampleTokens());

        for (Path path : new Path[] {store.credentialsPath(), store.tokensPath()}) {
            assertEquals(
                    PosixFilePermissions.fromString("rw-------"),
                    Files.getPosixFilePermissions(path),
                    path + " must be 0600 (plaintext secrets)");
        }
        assertEquals(
                PosixFilePermissions.fromString("rwx------"),
                Files.getPosixFilePermissions(dir),
                "store dir must be 0700");
    }

    @Test
    @EnabledOnOs({OS.LINUX, OS.MAC})
    void lockFileIsOwnerOnly() throws IOException {
        TokenStore store = new TokenStore(dir);
        try (TokenStore.StoreLock ignored = store.lockExclusive()) {
            assertEquals(
                    PosixFilePermissions.fromString("rw-------"),
                    Files.getPosixFilePermissions(dir.resolve(".lock")));
        }
    }

    @Test
    void corruptRecordSurfacesTypedStoreError() throws IOException {
        TokenStore store = new TokenStore(dir);
        Files.createDirectories(dir);
        Files.write(store.tokensPath(), "{not json".getBytes(StandardCharsets.UTF_8));

        StoreException e = assertThrows(
                StoreException.class,
                store::loadTokens,
                "corrupt tokens.json must surface a typed StoreException, not a crash or empty");
        assertFalse(
                e.getMessage().contains("not json"),
                "the error must NOT echo the record's bytes (they can hold secrets)");
    }

    @Test
    void expiryUsesSkew() {
        Tokens t = new Tokens("at", "rt", Instant.now().getEpochSecond() + 30, null, null);
        assertFalse(t.isExpired(0), "30s out, no skew => not expired");
        assertTrue(t.isExpired(60), "30s out, 60s skew => treated as expired");
    }

    @Test
    void writesAreAtomicIntoPlace() throws Exception {
        TokenStore store = new TokenStore(dir);
        store.saveTokens(sampleTokens());
        // Overwrite with different content; a non-atomic writer could leave a truncated
        // mix. After the write, the record parses and no temp files linger.
        store.saveTokens(new Tokens("at2", "rt2", 1L, null, null));
        assertEquals("rt2", store.loadTokens().orElseThrow().getRefreshToken());
        try (var listing = Files.list(dir)) {
            assertTrue(
                    listing.noneMatch(p -> p.getFileName().toString().startsWith(".tmp")),
                    "no orphaned temp files after a successful atomic write");
        }
    }
}
