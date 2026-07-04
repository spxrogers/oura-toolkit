package com.ouratoolkit.auth;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Optional;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Timeout;
import org.junit.jupiter.api.io.TempDir;

/**
 * The locking guarantees, tested with REAL concurrency (CLAUDE.md → TESTING rule 4):
 * genuinely concurrent refreshers, a genuinely held lock, and a genuinely separate JVM —
 * never sequential approximations that a no-op lock would pass.
 */
class LockConcurrencyTest {

    @TempDir
    Path dir;

    private static ClientCredentials credentials() {
        return new ClientCredentials("cid", "secret");
    }

    private static Tokens expiredTokens(String refreshToken) {
        return new Tokens("stale-access-" + refreshToken, refreshToken, 0L, null, null);
    }

    /**
     * The lock's reason to exist: two managers refreshing CONCURRENTLY must serialize to
     * exactly one endpoint call — the loser adopts the winner's persisted rotation. With
     * a no-op lock both would send r1 (or one would burn r2) and the count assertion
     * fails.
     */
    @Test
    @Timeout(30)
    void concurrentRefreshesSerializeToASingleEndpointCall() throws Exception {
        try (TokenEndpointStub stub = new TokenEndpointStub(form -> {
            if ("r1".equals(form.get("refresh_token"))) {
                try {
                    // Widen the race window: the winner sits in the critical section
                    // while the loser is genuinely blocked on the lock.
                    Thread.sleep(200);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
                return TokenEndpointStub.ok("fresh-access", "r2", 3600);
            }
            return TokenEndpointStub.invalidGrant();
        })) {
            TokenStore store = new TokenStore(dir);
            store.saveTokens(expiredTokens("r1"));

            TokenManager a = new TokenManager(new TokenStore(dir), credentials(), expiredTokens("r1"));
            TokenManager b = new TokenManager(new TokenStore(dir), credentials(), expiredTokens("r1"));
            a.overrideTokenUrl(stub.url());
            b.overrideTokenUrl(stub.url());

            ExecutorService pool = Executors.newFixedThreadPool(2);
            try {
                CountDownLatch start = new CountDownLatch(1);
                Future<String> fa = pool.submit(() -> {
                    start.await();
                    return a.getAccessToken();
                });
                Future<String> fb = pool.submit(() -> {
                    start.await();
                    return b.getAccessToken();
                });
                start.countDown();

                assertEquals("fresh-access", fa.get(20, TimeUnit.SECONDS));
                assertEquals("fresh-access", fb.get(20, TimeUnit.SECONDS));
            } finally {
                pool.shutdownNow();
            }

            assertEquals(1, stub.requests.get(),
                    "concurrent refreshes must serialize under the store lock: exactly "
                            + "one endpoint call; the loser adopts the winner's rotation");
            assertEquals("r2", store.loadTokens().orElseThrow().getRefreshToken());
        }
    }

    /**
     * The lock is genuinely HELD, and a blocking acquire genuinely WAITS: while a
     * concurrent thread holds the lock, {@code tryLockExclusive} must fail and
     * {@code lockExclusive} must not return until the holder releases.
     */
    @Test
    @Timeout(30)
    void heldLockExcludesAndBlocksAConcurrentAcquirer() throws Exception {
        TokenStore store = new TokenStore(dir);
        CountDownLatch locked = new CountDownLatch(1);
        CountDownLatch release = new CountDownLatch(1);
        AtomicBoolean released = new AtomicBoolean(false);

        Thread holder = new Thread(() -> {
            try (TokenStore.StoreLock ignored = store.lockExclusive()) {
                locked.countDown();
                release.await();
                released.set(true);
            } catch (Exception e) {
                throw new RuntimeException(e);
            }
        });
        holder.start();
        assertTrue(locked.await(10, TimeUnit.SECONDS), "holder failed to lock");

        assertTrue(store.tryLockExclusive().isEmpty(),
                "tryLockExclusive must fail while another thread holds the lock");

        // A blocking acquire must WAIT for the release — with a no-op lock it would
        // return before `released` is set and the assertion inside would fail.
        ExecutorService pool = Executors.newSingleThreadExecutor();
        try {
            Future<Boolean> blocked = pool.submit(() -> {
                try (TokenStore.StoreLock ignored = store.lockExclusive()) {
                    return released.get();
                }
            });
            Thread.sleep(200); // give a broken (non-blocking) acquire room to sneak in
            release.countDown();
            assertTrue(blocked.get(10, TimeUnit.SECONDS),
                    "lockExclusive returned before the holder released — the lock is not exclusive");
        } finally {
            pool.shutdownNow();
        }
        holder.join(10_000);

        Optional<TokenStore.StoreLock> free = store.tryLockExclusive();
        assertTrue(free.isPresent(), "lock must be free after release");
        free.get().close();
    }

    /**
     * CROSS-PROCESS exclusion, with a real second JVM: a child process holds the store
     * lock; this process must fail to acquire it until the child releases. (This proves
     * Java↔Java coordination. Java↔Rust on Linux is deliberately NOT lock-coordinated —
     * POSIX record locks vs flock, see TokenStore — the reload+retry protocol covers it.)
     */
    @Test
    @Timeout(60)
    void childJvmHoldingTheLockExcludesThisProcess() throws Exception {
        TokenStore store = new TokenStore(dir);
        String javaBin = Paths.get(System.getProperty("java.home"), "bin", "java").toString();
        ProcessBuilder pb = new ProcessBuilder(
                javaBin,
                "-cp",
                System.getProperty("java.class.path"),
                LockHolderMain.class.getName(),
                dir.toString());
        pb.redirectErrorStream(true);
        Process child = pb.start();
        try {
            BufferedReader out = new BufferedReader(
                    new InputStreamReader(child.getInputStream(), StandardCharsets.UTF_8));
            String line;
            boolean lockedSeen = false;
            while ((line = out.readLine()) != null) {
                if (line.equals("LOCKED")) {
                    lockedSeen = true;
                    break;
                }
                // Skip JVM banner noise (e.g. "Picked up JAVA_TOOL_OPTIONS...").
            }
            assertTrue(lockedSeen, "child JVM never reported acquiring the lock");

            assertTrue(store.tryLockExclusive().isEmpty(),
                    "a lock held by ANOTHER PROCESS must exclude this one — this is the "
                            + "cross-process guarantee the .lock file exists for");

            child.getOutputStream().write('\n');
            child.getOutputStream().flush();
            assertTrue(child.waitFor(20, TimeUnit.SECONDS), "child JVM did not exit");
            assertEquals(0, child.exitValue());

            Optional<TokenStore.StoreLock> free = store.tryLockExclusive();
            assertTrue(free.isPresent(), "lock must be free after the child released it");
            free.get().close();
        } finally {
            child.destroyForcibly();
        }
    }

    /**
     * Same-JVM contention must be EXCLUSION, not an exception: java.nio file locks are
     * per-JVM, so a naive FileChannel-only implementation would throw
     * {@code OverlappingFileLockException} here instead of reporting "held".
     */
    @Test
    @Timeout(30)
    void sameJvmContentionReportsHeldNotOverlappingFileLock() throws Exception {
        TokenStore store = new TokenStore(dir);
        try (TokenStore.StoreLock ignored = store.lockExclusive()) {
            // A second TokenStore instance for the same dir (flock-style: NOT reentrant,
            // even from the holding thread).
            assertTrue(new TokenStore(dir).tryLockExclusive().isEmpty());
        }
        Optional<TokenStore.StoreLock> free = store.tryLockExclusive();
        assertTrue(free.isPresent(), "lock must be free after close");
        free.get().close();
    }
}
