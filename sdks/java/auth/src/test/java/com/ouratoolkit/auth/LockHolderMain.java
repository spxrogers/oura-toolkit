package com.ouratoolkit.auth;

import java.nio.file.Paths;

/**
 * Child-JVM helper for the cross-process lock test: acquires the store's exclusive lock,
 * announces it on stdout, and holds it until the parent writes a byte to stdin. Run in a
 * SEPARATE JVM so the parent's {@code tryLockExclusive} contends through the OS file
 * lock, not the JVM-internal mutex.
 */
public final class LockHolderMain {

    private LockHolderMain() {}

    public static void main(String[] args) throws Exception {
        TokenStore store = new TokenStore(Paths.get(args[0]));
        try (TokenStore.StoreLock ignored = store.lockExclusive()) {
            System.out.println("LOCKED");
            System.out.flush();
            System.in.read(); // parent signals release
        }
        System.out.println("RELEASED");
        System.out.flush();
    }
}
