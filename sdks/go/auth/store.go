package auth

import (
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

// appDirName is the locked config-directory name (CLAUDE.md → NAMING), identical under
// npx/bunx/brew and identical to the Rust companion's store.
const appDirName = "oura-toolkit"

// ClientCredentials is the user's own Oura OAuth application id + secret (BYO
// confidential client). JSON field names are pinned to the Rust companion's serde output
// (credentials.json is shared with the CLI) — see the fixture test.
//
// String/GoString REDACT the secret so a stray %v/%+v/%#v can never leak it into logs.
type ClientCredentials struct {
	ClientID     string `json:"client_id"`
	ClientSecret string `json:"client_secret"`
}

func (c ClientCredentials) String() string {
	return fmt.Sprintf("ClientCredentials{ClientID:%q ClientSecret:[REDACTED]}", c.ClientID)
}

// GoString redacts under %#v (fmt uses GoStringer there, not Stringer).
func (c ClientCredentials) GoString() string { return c.String() }

// Tokens is the persisted OAuth token set. Client credentials live in their own record
// (ClientCredentials), not here. JSON field names are pinned to the Rust companion's
// serde output (tokens.json is shared with the CLI) — see the fixture test.
//
// String/GoString REDACT the token fields so a stray %v/%+v/%#v can never leak them.
type Tokens struct {
	AccessToken string `json:"access_token"`
	// RefreshToken is rotated by Oura on every refresh, invalidating the previous value —
	// always persist the newly returned one or the next refresh 400s.
	RefreshToken string `json:"refresh_token"`
	// ExpiresAt is the absolute expiry as a Unix timestamp (seconds).
	ExpiresAt int64  `json:"expires_at"`
	Scope     string `json:"scope,omitempty"`
	TokenType string `json:"token_type,omitempty"`
}

func (t Tokens) String() string {
	return fmt.Sprintf(
		"Tokens{AccessToken:[REDACTED] RefreshToken:[REDACTED] ExpiresAt:%d Scope:%q TokenType:%q}",
		t.ExpiresAt, t.Scope, t.TokenType,
	)
}

// GoString redacts under %#v (fmt uses GoStringer there, not Stringer).
func (t Tokens) GoString() string { return t.String() }

// expired reports whether the access token is expired, or within skew of expiring.
func (t *Tokens) expired(skew time.Duration) bool {
	return time.Now().Add(skew).Unix() >= t.ExpiresAt
}

// Store is a handle to the on-disk store directory. Two records live in it:
//
//   - credentials.json — ClientCredentials; exists from `oura auth setup` onward.
//   - tokens.json — Tokens; exists after a successful login, rewritten on every rotation.
//
// Separating the two means a failed or abandoned login never loses the pasted secret.
// Everything is written via an atomic, uniquely named temp-file + rename with 0600 perms
// on Unix (Windows relies on %LOCALAPPDATA%'s user-private ACLs).
type Store struct {
	dir string
}

// NewStore opens the store at the default per-platform config location
// ($XDG_CONFIG_HOME/oura-toolkit → ~/.config/oura-toolkit on Unix/macOS,
// %LOCALAPPDATA%\oura-toolkit on Windows).
func NewStore() (*Store, error) {
	dir, err := configDirFrom(runtime.GOOS, os.LookupEnv)
	if err != nil {
		return nil, err
	}
	return &Store{dir: dir}, nil
}

// NewStoreAt opens a store rooted at an explicit directory (used by tests).
func NewStoreAt(dir string) *Store {
	return &Store{dir: dir}
}

// Dir returns the store directory.
func (s *Store) Dir() string { return s.dir }

// CredentialsPath is the client-credentials record path.
func (s *Store) CredentialsPath() string { return filepath.Join(s.dir, "credentials.json") }

// TokensPath is the token record path.
func (s *Store) TokensPath() string { return filepath.Join(s.dir, "tokens.json") }

// LoadCredentials returns the client credentials, or (nil, nil) if `oura auth setup` has
// never run.
func (s *Store) LoadCredentials() (*ClientCredentials, error) {
	var c ClientCredentials
	ok, err := s.loadJSON(s.CredentialsPath(), &c)
	if err != nil || !ok {
		return nil, err
	}
	return &c, nil
}

// SaveCredentials persists the client credentials (0600, atomic).
func (s *Store) SaveCredentials(c *ClientCredentials) error {
	return s.saveJSON(s.CredentialsPath(), c)
}

// LoadTokens returns the tokens, or (nil, nil) if no login has succeeded yet.
func (s *Store) LoadTokens() (*Tokens, error) {
	var t Tokens
	ok, err := s.loadJSON(s.TokensPath(), &t)
	if err != nil || !ok {
		return nil, err
	}
	return &t, nil
}

// SaveTokens persists the tokens (0600, atomic). Callers refreshing MUST persist the
// rotated refresh token (Oura invalidates the previous one), and anything that refreshes
// and rewrites tokens must do so under LockExclusive — as Manager does — or it can burn a
// rotation another process just persisted.
func (s *Store) SaveTokens(t *Tokens) error {
	return s.saveJSON(s.TokensPath(), t)
}

func (s *Store) loadJSON(path string, into any) (bool, error) {
	data, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return false, nil
	}
	if err != nil {
		return false, fmt.Errorf("token store i/o error: %w", err)
	}
	if err := json.Unmarshal(data, into); err != nil {
		return false, fmt.Errorf("token store format error: %w", err)
	}
	return true, nil
}

func (s *Store) saveJSON(path string, v any) error {
	data, err := json.MarshalIndent(v, "", "  ")
	if err != nil {
		return fmt.Errorf("token store format error: %w", err)
	}
	if err := s.ensureDir(); err != nil {
		return err
	}
	return writeSecure(path, data)
}

// StoreLock is an exclusive advisory lock on the store, released by Unlock (or process
// exit). Hold it across a reload → refresh → persist critical section (see Manager).
type StoreLock struct {
	f *os.File
}

// Unlock releases the lock and closes the underlying file.
func (l *StoreLock) Unlock() error {
	err := unlockFile(l.f)
	if cerr := l.f.Close(); err == nil {
		err = cerr
	}
	return err
}

// LockExclusive takes a BLOCKING exclusive lock on the store's .lock file — flock on
// Unix, LockFileEx on Windows: the same primitives as Rust std's File::lock, so this
// coordinates with the CLI and MCP server sharing the store.
//
// The lock is cooperative (it excludes only other holders of this protocol, not
// arbitrary writers), and mutual exclusion rests on the .lock file's identity — deleting
// it while held lets the next locker acquire a fresh one and defeats coordination.
func (s *Store) LockExclusive() (*StoreLock, error) {
	f, err := s.openLockFile()
	if err != nil {
		return nil, err
	}
	if err := lockFile(f); err != nil {
		f.Close()
		return nil, fmt.Errorf("token store lock error: %w", err)
	}
	return &StoreLock{f: f}, nil
}

// TryLockExclusive is the non-blocking variant: (nil, nil) means another process
// currently holds the lock.
func (s *Store) TryLockExclusive() (*StoreLock, error) {
	f, err := s.openLockFile()
	if err != nil {
		return nil, err
	}
	ok, err := tryLockFile(f)
	if err != nil {
		f.Close()
		return nil, fmt.Errorf("token store lock error: %w", err)
	}
	if !ok {
		f.Close()
		return nil, nil
	}
	return &StoreLock{f: f}, nil
}

func (s *Store) openLockFile() (*os.File, error) {
	if err := s.ensureDir(); err != nil {
		return nil, err
	}
	f, err := os.OpenFile(filepath.Join(s.dir, ".lock"), os.O_CREATE|os.O_RDWR, 0o600)
	if err != nil {
		return nil, fmt.Errorf("token store i/o error: %w", err)
	}
	return f, nil
}

func (s *Store) ensureDir() error {
	if err := os.MkdirAll(s.dir, 0o700); err != nil {
		return fmt.Errorf("token store i/o error: %w", err)
	}
	return setDirPrivate(s.dir)
}

// configDirFrom resolves the fixed, invocation-independent config dir from an injected
// env lookup (testable on every OS without racy os.Setenv):
//
//   - Unix (incl. macOS): $XDG_CONFIG_HOME/oura-toolkit, falling back to
//     $HOME/.config/oura-toolkit. The XDG path is a locked decision (CLAUDE.md) —
//     deliberately NOT macOS's ~/Library/Application Support.
//   - Windows: %LOCALAPPDATA%\oura-toolkit. Local, NOT roaming: roaming profiles sync
//     %APPDATA% to file servers/backups at logoff, which would copy plaintext secrets off
//     the machine.
//
// Empty and RELATIVE values are treated as absent (XDG spec: relative values should be
// ignored) — a relative base would make where secrets land depend on the process cwd,
// breaking the invocation-independence invariant.
func configDirFrom(goos string, lookup func(string) (string, bool)) (string, error) {
	usable := func(key string) (string, bool) {
		v, ok := lookup(key)
		if !ok || v == "" || !isAbs(goos, v) {
			return "", false
		}
		return v, true
	}

	if goos == "windows" {
		if base, ok := usable("LOCALAPPDATA"); ok {
			return filepath.Join(base, appDirName), nil
		}
		return "", fmt.Errorf("%w (%%LOCALAPPDATA%% unset or not an absolute path)", ErrNoConfigDir)
	}

	if xdg, ok := usable("XDG_CONFIG_HOME"); ok {
		return filepath.Join(xdg, appDirName), nil
	}
	if home, ok := usable("HOME"); ok {
		return filepath.Join(home, ".config", appDirName), nil
	}
	return "", fmt.Errorf("%w ($XDG_CONFIG_HOME / $HOME unset or not an absolute path)", ErrNoConfigDir)
}

// isAbs is goos-parameterized (unlike filepath.IsAbs, which is compile-time) so both
// platform branches of configDirFrom are unit-tested on every CI leg.
func isAbs(goos, path string) bool {
	if goos == "windows" {
		// Drive-rooted (C:\ or C:/) or UNC (\\server\share).
		if len(path) >= 3 && path[1] == ':' && (path[2] == '\\' || path[2] == '/') {
			return true
		}
		return strings.HasPrefix(path, `\\`)
	}
	return strings.HasPrefix(path, "/")
}

// writeSecure writes atomically with owner-only perms: a UNIQUELY named temp file in the
// same directory (os.CreateTemp creates 0600), fsync, rename into place. The unique name
// means two concurrent writers can never truncate each other's in-flight temp file; the
// atomic rename makes the outcome last-writer-wins, never a corrupt mix.
func writeSecure(path string, data []byte) error {
	dir := filepath.Dir(path)
	tmp, err := os.CreateTemp(dir, ".tmp-*")
	if err != nil {
		return fmt.Errorf("token store i/o error: %w", err)
	}
	tmpName := tmp.Name()
	cleanup := func(err error) error {
		tmp.Close()
		os.Remove(tmpName)
		return fmt.Errorf("token store i/o error: %w", err)
	}
	if _, err := tmp.Write(data); err != nil {
		return cleanup(err)
	}
	if err := tmp.Sync(); err != nil {
		return cleanup(err)
	}
	if err := tmp.Close(); err != nil {
		os.Remove(tmpName)
		return fmt.Errorf("token store i/o error: %w", err)
	}
	if err := os.Rename(tmpName, path); err != nil {
		os.Remove(tmpName)
		return fmt.Errorf("token store i/o error: %w", err)
	}
	return setFilePrivate(path)
}
