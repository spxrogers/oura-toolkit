package auth

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"os/exec"
	"testing"
	"time"
)

// TestMain doubles as the cross-process lock holder: when re-exec'd with
// OURA_AUTH_TEST_HOLD_LOCK set, the test binary takes the store lock, announces it on
// stdout, and holds it until its stdin closes. This gives the mutual-exclusion test a
// REAL second process contending on the real flock/LockFileEx — not a goroutine
// approximation that a no-op lock would satisfy.
func TestMain(m *testing.M) {
	if dir := os.Getenv("OURA_AUTH_TEST_HOLD_LOCK"); dir != "" {
		holdLockUntilStdinCloses(dir)
		return
	}
	os.Exit(m.Run())
}

func holdLockUntilStdinCloses(dir string) {
	lock, err := NewStoreAt(dir).LockExclusive()
	if err != nil {
		fmt.Fprintln(os.Stderr, "helper: lock failed:", err)
		os.Exit(1)
	}
	fmt.Println("LOCKED")
	_, _ = io.Copy(io.Discard, os.Stdin) // hold until the parent closes stdin
	if err := lock.Unlock(); err != nil {
		fmt.Fprintln(os.Stderr, "helper: unlock failed:", err)
		os.Exit(1)
	}
}

// The cross-process protocol's foundation: while ANOTHER PROCESS holds the store lock,
// TryLockExclusive reports it held and LockExclusive genuinely blocks; when the holder
// releases, the blocked acquire completes. This is what lets the CLI and the MCP server
// share one store without burning each other's refresh-token rotations.
func TestLockExcludesAnotherProcessUntilReleased(t *testing.T) {
	dir := t.TempDir()
	exe, err := os.Executable()
	if err != nil {
		t.Fatal(err)
	}
	holder := exec.Command(exe, "-test.run=^$")
	holder.Env = append(os.Environ(), "OURA_AUTH_TEST_HOLD_LOCK="+dir)
	stdin, err := holder.StdinPipe()
	if err != nil {
		t.Fatal(err)
	}
	stdout, err := holder.StdoutPipe()
	if err != nil {
		t.Fatal(err)
	}
	holder.Stderr = os.Stderr
	if err := holder.Start(); err != nil {
		t.Fatal(err)
	}
	defer func() {
		stdin.Close()
		holder.Wait()
	}()

	// Wait until the helper HOLDS the lock.
	line, err := bufio.NewReader(stdout).ReadString('\n')
	if err != nil || line != "LOCKED\n" {
		t.Fatalf("helper did not confirm the lock (line %q, err %v)", line, err)
	}

	store := NewStoreAt(dir)

	// 1. Non-blocking acquire must observe the other process's lock.
	if held, err := store.TryLockExclusive(); err != nil {
		t.Fatal(err)
	} else if held != nil {
		held.Unlock()
		t.Fatal("TryLockExclusive succeeded while another PROCESS held the lock — cross-process exclusion is broken")
	}

	// 2. Blocking acquire must genuinely block while the other process holds it.
	type result struct {
		lock *StoreLock
		err  error
	}
	acquired := make(chan result, 1)
	go func() {
		l, err := store.LockExclusive()
		acquired <- result{l, err}
	}()
	select {
	case r := <-acquired:
		if r.err == nil {
			r.lock.Unlock()
		}
		t.Fatal("LockExclusive returned while another process held the lock — it does not block")
	case <-time.After(300 * time.Millisecond):
		// Still blocked: correct.
	}

	// 3. Releasing the holder (stdin EOF) must let the blocked acquire complete.
	stdin.Close()
	select {
	case r := <-acquired:
		if r.err != nil {
			t.Fatal(r.err)
		}
		r.lock.Unlock()
	case <-time.After(10 * time.Second):
		t.Fatal("LockExclusive still blocked after the holding process released the lock")
	}
	if err := holder.Wait(); err != nil {
		t.Fatalf("helper process failed: %v", err)
	}
}
