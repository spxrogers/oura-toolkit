//go:build unix

package auth

import (
	"os"
	"path/filepath"
	"testing"
)

// The 0600/0700 hygiene invariant (CLAUDE.md → token store), on the Unix CI leg where the
// modes are real (Windows relies on %LOCALAPPDATA% ACLs and the chmods are no-ops).
func TestRecordsDirAndLockAreOwnerOnly(t *testing.T) {
	dir := filepath.Join(t.TempDir(), "store")
	store := NewStoreAt(dir)
	if err := store.SaveCredentials(sampleCredentials()); err != nil {
		t.Fatal(err)
	}
	if err := store.SaveTokens(sampleTokens()); err != nil {
		t.Fatal(err)
	}
	lock, err := store.LockExclusive()
	if err != nil {
		t.Fatal(err)
	}
	defer lock.Unlock()

	for _, path := range []string{store.CredentialsPath(), store.TokensPath(), filepath.Join(dir, ".lock")} {
		info, err := os.Stat(path)
		if err != nil {
			t.Fatal(err)
		}
		if mode := info.Mode().Perm(); mode != 0o600 {
			t.Fatalf("%s must be 0600 (owner-only secrets), got %o", path, mode)
		}
	}
	info, err := os.Stat(dir)
	if err != nil {
		t.Fatal(err)
	}
	if mode := info.Mode().Perm(); mode != 0o700 {
		t.Fatalf("store dir must be 0700, got %o", mode)
	}
}

// Overwrites (the every-refresh rewrite path) must keep 0600 even through the temp-file +
// rename dance and a pre-existing looser file.
func TestRewriteRestoresOwnerOnlyPerms(t *testing.T) {
	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(sampleTokens()); err != nil {
		t.Fatal(err)
	}
	if err := os.Chmod(store.TokensPath(), 0o644); err != nil {
		t.Fatal(err)
	}
	if err := store.SaveTokens(sampleTokens()); err != nil {
		t.Fatal(err)
	}
	info, err := os.Stat(store.TokensPath())
	if err != nil {
		t.Fatal(err)
	}
	if mode := info.Mode().Perm(); mode != 0o600 {
		t.Fatalf("rewritten tokens.json must be 0600, got %o", mode)
	}
}
