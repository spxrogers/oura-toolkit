//go:build windows

package auth

import (
	"errors"
	"os"

	"golang.org/x/sys/windows"
)

// The whole-file byte range (offset 0, length u32max/u32max), matching what Rust std's
// File::lock passes to LockFileEx so the two implementations contend on the same range.
const lockLenLow, lockLenHigh = ^uint32(0), ^uint32(0)

// lockFile takes a blocking exclusive LockFileEx lock — the same primitive Rust std's
// File::lock uses on Windows, so this excludes the CLI/MCP server sharing the store.
func lockFile(f *os.File) error {
	return windows.LockFileEx(
		windows.Handle(f.Fd()), windows.LOCKFILE_EXCLUSIVE_LOCK,
		0, lockLenLow, lockLenHigh, new(windows.Overlapped),
	)
}

// tryLockFile is the non-blocking variant: (false, nil) when another handle holds it.
func tryLockFile(f *os.File) (bool, error) {
	err := windows.LockFileEx(
		windows.Handle(f.Fd()), windows.LOCKFILE_EXCLUSIVE_LOCK|windows.LOCKFILE_FAIL_IMMEDIATELY,
		0, lockLenLow, lockLenHigh, new(windows.Overlapped),
	)
	if errors.Is(err, windows.ERROR_LOCK_VIOLATION) {
		return false, nil
	}
	if err != nil {
		return false, err
	}
	return true, nil
}

func unlockFile(f *os.File) error {
	return windows.UnlockFileEx(windows.Handle(f.Fd()), 0, lockLenLow, lockLenHigh, new(windows.Overlapped))
}

// setFilePrivate is a no-op on Windows: protection comes from %LOCALAPPDATA%'s inherited
// user-private ACLs (the Unix chmods are meaningless here). OS-keyring was evaluated and
// deferred (#26); optional DPAPI at-rest encryption of the file bytes is tracked in #78.
func setFilePrivate(string) error { return nil }

// setDirPrivate is a no-op on Windows (see setFilePrivate).
func setDirPrivate(string) error { return nil }
