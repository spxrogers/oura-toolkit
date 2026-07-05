package com.ouratoolkit.auth;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.nio.file.Files;
import java.nio.file.NoSuchFileException;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;
import java.nio.file.StandardOpenOption;
import java.nio.file.attribute.FileAttribute;
import java.nio.file.attribute.PosixFilePermission;
import java.nio.file.attribute.PosixFilePermissions;
import java.util.Locale;
import java.util.Optional;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.Semaphore;
import java.util.function.Function;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.cfg.CoercionAction;
import com.fasterxml.jackson.databind.cfg.CoercionInputShape;
import com.fasterxml.jackson.databind.type.LogicalType;

/**
 * Persistent credential store at the fixed, invocation-independent per-platform path:
 * {@code $XDG_CONFIG_HOME/oura-toolkit/} (falling back to
 * {@code $HOME/.config/oura-toolkit/}) on Unix/macOS, {@code %LOCALAPPDATA%\oura-toolkit\}
 * on Windows. This is the SAME directory the Rust CLI/MCP server uses — the two runtimes
 * share one store, so schemas and paths are locked (CLAUDE.md → AUTH).
 *
 * <p>Two records live in the store dir:
 * <ul>
 *   <li><b>{@code credentials.json}</b> — {@link ClientCredentials} (the user's own OAuth
 *       app id + secret). Exists from {@code oura auth setup} onward.</li>
 *   <li><b>{@code tokens.json}</b> — {@link Tokens} (access/refresh/expiry/scope). Exists
 *       only after a successful login; rewritten on every refresh rotation.</li>
 * </ul>
 *
 * <p><b>File hygiene:</b> on POSIX filesystems the records are written {@code 0600} and
 * the directory {@code 0700}, via a uniquely named temp file + atomic move (two
 * concurrent writers can never truncate each other's in-flight temp file; the atomic
 * rename makes the outcome last-writer-wins, never a corrupt mix). On Windows those
 * permission calls are skipped — protection relies on {@code %LOCALAPPDATA%}'s inherited
 * user-private ACLs (Local, not Roaming, deliberately: roaming profiles sync
 * {@code %APPDATA%} off the machine).
 *
 * <p><b>Cross-process coordination:</b> {@link #lockExclusive()} takes an exclusive lock
 * on a {@code .lock} file in the store dir; {@link TokenManager} holds it across its
 * reload → refresh → persist critical section.
 *
 * <p><b>IMPORTANT cross-runtime caveat:</b> on Linux, {@code FileChannel.lock} acquires a
 * POSIX record lock ({@code fcntl}), which does NOT interact with the {@code flock} the
 * Rust CLI/MCP server takes on the same {@code .lock} file — a Java process and a Rust
 * process can both believe they hold "the" lock. The lock still serializes Java processes
 * (and, via a JVM-wide per-path mutex, threads within one JVM — plain
 * {@code FileChannel.lock} would throw {@code OverlappingFileLockException} instead of
 * blocking for same-JVM contention). Across runtimes, the guarantee is the
 * <b>reload-and-adopt + 400-reload-retry protocol</b> in {@link TokenManager}, which is
 * exactly the arm the Rust implementation provides for uncoordinated writers.
 */
public final class TokenStore {

    /** The locked config-directory name (CLAUDE.md → NAMING). */
    public static final String APP_DIR_NAME = "oura-toolkit";

    private static final Set<PosixFilePermission> OWNER_RW =
            PosixFilePermissions.fromString("rw-------");
    private static final Set<PosixFilePermission> OWNER_RWX =
            PosixFilePermissions.fromString("rwx------");

    /**
     * JVM-wide per-store mutexes backing {@link #lockExclusive()}: java.nio file locks
     * are held on behalf of the whole JVM, so same-JVM contention must be serialized
     * here (see the class doc). Deliberately a NON-reentrant {@code Semaphore(1)}, not a
     * {@code ReentrantLock}: flock-style store locks are not reentrant (a thread
     * re-acquiring would then hit {@code OverlappingFileLockException} at the channel),
     * so a second acquire from the SAME thread reports "held" / blocks like any other
     * contender.
     */
    private static final ConcurrentHashMap<String, Semaphore> JVM_LOCKS =
            new ConcurrentHashMap<>();

    private static final ObjectMapper MAPPER = storeMapper();

    private static ObjectMapper storeMapper() {
        ObjectMapper mapper = new ObjectMapper()
                // Match serde's default: unknown fields are ignored, not fatal — a newer
                // writer adding a field must not brick older readers of the shared store.
                .configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false)
                // A JSON `null` for a primitive field (e.g. "expires_at": null) must be a typed
                // store-format error, NOT silently coerced to 0L (which would masquerade as an
                // already-expired token). serde rejects this; match it.
                .configure(DeserializationFeature.FAIL_ON_NULL_FOR_PRIMITIVES, true);
        // A JSON number/boolean where a string field is expected (e.g. "client_id": 7)
        // must be a typed store-format error, NOT silently coerced to "7" — serde rejects
        // wrong-typed fields, and the shared store's wire format is serde's (conformance
        // fixture #58, hostile_store_files/credentials_wrong_type_client_id).
        mapper.coercionConfigFor(LogicalType.Textual)
                .setCoercion(CoercionInputShape.Integer, CoercionAction.Fail)
                .setCoercion(CoercionInputShape.Float, CoercionAction.Fail)
                .setCoercion(CoercionInputShape.Boolean, CoercionAction.Fail);
        return mapper;
    }

    private final Path dir;

    /** Store rooted at an explicit directory (tests, or embedded consumers). */
    public TokenStore(Path dir) {
        this.dir = dir;
    }

    /** Store at the default per-platform config location. */
    public static TokenStore openDefault() throws NoConfigDirException {
        return new TokenStore(configDir(System::getenv, isWindows()));
    }

    /** The store directory. */
    public Path getDir() {
        return dir;
    }

    /** Path of the client-credentials record. */
    public Path credentialsPath() {
        return dir.resolve("credentials.json");
    }

    /** Path of the token record. */
    public Path tokensPath() {
        return dir.resolve("tokens.json");
    }

    /**
     * Load the client credentials, or empty if {@code auth setup} has never run. A record
     * that exists but is malformed (bad JSON, wrong-typed/missing field, or a literal
     * {@code null}) surfaces as {@link StoreException}, never a crash.
     */
    public Optional<ClientCredentials> loadCredentials() throws IOException, StoreException {
        return load(credentialsPath(), ClientCredentials.class);
    }

    /** Persist the client credentials ({@code 0600}, atomic). */
    public void saveCredentials(ClientCredentials credentials) throws IOException {
        save(credentialsPath(), credentials);
    }

    /**
     * Load the tokens, or empty if no login has succeeded yet. A record that exists but is
     * malformed (bad JSON, wrong-typed/missing field, or a literal {@code null}) surfaces
     * as {@link StoreException}, never a crash.
     */
    public Optional<Tokens> loadTokens() throws IOException, StoreException {
        return load(tokensPath(), Tokens.class);
    }

    /**
     * Persist the tokens ({@code 0600}, atomic). Callers refreshing MUST persist the
     * rotated refresh token (Oura invalidates the previous one), and MUST do so under
     * {@link #lockExclusive()} as {@link TokenManager} does, or they can burn a rotation
     * another process just persisted.
     */
    public void saveTokens(Tokens tokens) throws IOException {
        save(tokensPath(), tokens);
    }

    /**
     * Take a BLOCKING exclusive lock on the store; hold the returned guard across a
     * reload → refresh → persist critical section (see {@link TokenManager}). Excludes
     * other threads in this JVM (per-path mutex) and other Java processes (file lock);
     * for the Rust runtime see the class-doc caveat — the reload+retry protocol, not this
     * lock, is the cross-runtime guarantee.
     */
    public StoreLock lockExclusive() throws IOException {
        ensureDir();
        Semaphore jvmLock = jvmLockFor();
        jvmLock.acquireUninterruptibly();
        try {
            FileChannel channel = openLockChannel();
            try {
                FileLock fileLock = channel.lock();
                return new StoreLock(jvmLock, channel, fileLock);
            } catch (IOException | RuntimeException e) {
                channel.close();
                throw e;
            }
        } catch (IOException | RuntimeException e) {
            jvmLock.release();
            throw e;
        }
    }

    /**
     * Non-blocking variant of {@link #lockExclusive()}: empty if another holder (thread
     * or process) currently has it.
     */
    public Optional<StoreLock> tryLockExclusive() throws IOException {
        ensureDir();
        Semaphore jvmLock = jvmLockFor();
        if (!jvmLock.tryAcquire()) {
            return Optional.empty();
        }
        try {
            FileChannel channel = openLockChannel();
            try {
                FileLock fileLock = channel.tryLock();
                if (fileLock == null) {
                    channel.close();
                    jvmLock.release();
                    return Optional.empty();
                }
                return Optional.of(new StoreLock(jvmLock, channel, fileLock));
            } catch (IOException | RuntimeException e) {
                channel.close();
                throw e;
            }
        } catch (IOException | RuntimeException e) {
            jvmLock.release();
            throw e;
        }
    }

    // --- internals ---------------------------------------------------------------------

    private Semaphore jvmLockFor() {
        return JVM_LOCKS.computeIfAbsent(
                dir.toAbsolutePath().normalize().toString(), k -> new Semaphore(1));
    }

    private FileChannel openLockChannel() throws IOException {
        Path lockPath = dir.resolve(".lock");
        if (posixSupported()) {
            return FileChannel.open(
                    lockPath,
                    Set.of(StandardOpenOption.CREATE, StandardOpenOption.WRITE),
                    PosixFilePermissions.asFileAttribute(OWNER_RW));
        }
        return FileChannel.open(lockPath, StandardOpenOption.CREATE, StandardOpenOption.WRITE);
    }

    private <T> Optional<T> load(Path path, Class<T> type)
            throws IOException, StoreException {
        final byte[] bytes;
        try {
            bytes = Files.readAllBytes(path);
        } catch (NoSuchFileException e) {
            return Optional.empty();
        }
        final T value;
        try {
            value = MAPPER.readValue(bytes, type);
        } catch (JsonProcessingException e) {
            // Malformed JSON, a wrong-typed or missing required field, or a JSON null for a
            // primitive (FAIL_ON_NULL_FOR_PRIMITIVES): surface ONE typed store-format error,
            // mirroring the Rust store's typed Serde error. The message names the file but
            // NEVER echoes its bytes (the record holds secrets).
            throw new StoreException(path.getFileName() + " is not a valid store record", e);
        }
        if (value == null) {
            // A literal `null` JSON document deserializes to Java null. Left as
            // Optional.of(null) it would throw an UNCHECKED NullPointerException that
            // escapes callers' IOException handling and crashes raw — surface it typed.
            throw new StoreException(
                    path.getFileName() + " is not a valid store record (JSON null)", null);
        }
        return Optional.of(value);
    }

    private void save(Path path, Object record) throws IOException {
        ensureDir();
        byte[] data = MAPPER.writerWithDefaultPrettyPrinter()
                .writeValueAsBytes(record);
        writeSecure(path, data);
    }

    /**
     * Atomic write with owner-only perms: a UNIQUELY named temp file in the same
     * directory (created {@code 0600} on POSIX), fsync, atomic move into place.
     */
    private void writeSecure(Path path, byte[] data) throws IOException {
        boolean posix = posixSupported();
        Path tmp = posix
                ? Files.createTempFile(dir, ".tmp", null,
                        PosixFilePermissions.asFileAttribute(OWNER_RW))
                : Files.createTempFile(dir, ".tmp", null);
        try {
            try (FileChannel ch = FileChannel.open(tmp, StandardOpenOption.WRITE)) {
                ByteBuffer buf = ByteBuffer.wrap(data);
                while (buf.hasRemaining()) {
                    ch.write(buf);
                }
                ch.force(true);
            }
            Files.move(tmp, path, StandardCopyOption.ATOMIC_MOVE,
                    StandardCopyOption.REPLACE_EXISTING);
            if (posix) {
                Files.setPosixFilePermissions(path, OWNER_RW);
            }
        } catch (IOException | RuntimeException e) {
            Files.deleteIfExists(tmp);
            throw e;
        }
    }

    private void ensureDir() throws IOException {
        Files.createDirectories(dir);
        if (posixSupported()) {
            Files.setPosixFilePermissions(dir, OWNER_RWX);
        }
    }

    private boolean posixSupported() {
        return dir.getFileSystem().supportedFileAttributeViews().contains("posix");
    }

    static boolean isWindows() {
        return System.getProperty("os.name", "").toLowerCase(Locale.ROOT).contains("win");
    }

    /**
     * Resolve the fixed, invocation-independent config dir from an injected env lookup
     * (testable on any OS without mutating global env — the store.rs
     * {@code config_dir_from} pattern):
     *
     * <ul>
     *   <li><b>Unix (incl. macOS):</b> {@code $XDG_CONFIG_HOME/oura-toolkit}, falling
     *       back to {@code $HOME/.config/oura-toolkit}. Locked decision — identical under
     *       every launcher, deliberately NOT {@code ~/Library/Application Support}.</li>
     *   <li><b>Windows:</b> {@code %LOCALAPPDATA%\oura-toolkit}. Local, NOT roaming.</li>
     * </ul>
     *
     * <p>EMPTY and RELATIVE env values are treated as absent (XDG spec; a relative base
     * would make where secrets land depend on the process cwd). Absoluteness is checked
     * per the TARGET platform's syntax (a drive-letter check on the Windows branch), so
     * both branches are unit-testable on every CI OS.
     */
    static Path configDir(Function<String, String> env, boolean windows)
            throws NoConfigDirException {
        if (windows) {
            String local = usable(env.apply("LOCALAPPDATA"), true);
            if (local == null) {
                throw new NoConfigDirException(
                        "could not determine the config directory "
                                + "(%LOCALAPPDATA% unset or not an absolute path)");
            }
            return Paths.get(local, APP_DIR_NAME);
        }
        String xdg = usable(env.apply("XDG_CONFIG_HOME"), false);
        if (xdg != null) {
            return Paths.get(xdg, APP_DIR_NAME);
        }
        String home = usable(env.apply("HOME"), false);
        if (home != null) {
            return Paths.get(home, ".config", APP_DIR_NAME);
        }
        throw new NoConfigDirException(
                "could not determine the config directory "
                        + "($XDG_CONFIG_HOME / $HOME unset or not an absolute path)");
    }

    /** Null unless {@code value} is non-empty and absolute for the target platform. */
    private static String usable(String value, boolean windows) {
        if (value == null || value.isEmpty()) {
            return null;
        }
        boolean absolute = windows
                ? value.matches("^[A-Za-z]:[\\\\/].*") || value.startsWith("\\\\")
                : value.startsWith("/");
        return absolute ? value : null;
    }

    /**
     * An exclusive lock on the store, released on {@link #close()}. Always use
     * try-with-resources; the lock releases the moment this closes.
     */
    public static final class StoreLock implements AutoCloseable {
        private final Semaphore jvmLock;
        private final FileChannel channel;
        private final FileLock fileLock;
        private boolean closed;

        private StoreLock(Semaphore jvmLock, FileChannel channel, FileLock fileLock) {
            this.jvmLock = jvmLock;
            this.channel = channel;
            this.fileLock = fileLock;
        }

        @Override
        public void close() {
            if (closed) {
                return;
            }
            closed = true;
            try {
                fileLock.release();
            } catch (IOException ignored) {
                // Channel close below still drops the OS lock.
            } finally {
                try {
                    channel.close();
                } catch (IOException ignored) {
                    // Nothing actionable; the process-exit fallback releases anyway.
                } finally {
                    jvmLock.release();
                }
            }
        }
    }
}
