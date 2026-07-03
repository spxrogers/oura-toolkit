//go:build unix

package auth

import (
	"os"

	"golang.org/x/sys/unix"
)

// lockFile takes a blocking exclusive advisory flock — the same primitive Rust std's
// File::lock uses on Unix/macOS, so this excludes the CLI/MCP server sharing the store.
func lockFile(f *os.File) error {
	for {
		err := unix.Flock(int(f.Fd()), unix.LOCK_EX)
		if err != unix.EINTR {
			return err
		}
	}
}

// tryLockFile is the non-blocking variant: (false, nil) when another handle holds it.
func tryLockFile(f *os.File) (bool, error) {
	err := unix.Flock(int(f.Fd()), unix.LOCK_EX|unix.LOCK_NB)
	switch err {
	case nil:
		return true, nil
	case unix.EWOULDBLOCK:
		return false, nil
	default:
		return false, err
	}
}

func unlockFile(f *os.File) error {
	return unix.Flock(int(f.Fd()), unix.LOCK_UN)
}

// setFilePrivate chmods a record to 0600 (owner-only).
func setFilePrivate(path string) error {
	return os.Chmod(path, 0o600)
}

// setDirPrivate chmods the store dir to 0700 (owner-only).
func setDirPrivate(dir string) error {
	return os.Chmod(dir, 0o700)
}
