"""Cross-process locking with REAL concurrency (never sequential approximations that a
no-op lock would pass — CLAUDE.md TESTING & VERIFICATION rule 4):

- mutual exclusion against a raw ``flock`` holder in a live subprocess — this is
  byte-for-byte the primitive the Rust CLI's ``std::fs::File::lock`` uses on Unix, so
  it is also the interop proof;
- two PROCESSES refreshing the same store concurrently serialize to a single token
  endpoint call (the loser adopts the winner's persisted rotation).

Unix-gated: the flock interop guarantee is Unix-only by design (on Windows the
reload-adopt + 400-retry protocol, tested in test_manager.py, is the guarantee).
"""

from __future__ import annotations

import os
import subprocess
import sys
import textwrap
import time
from pathlib import Path

import pytest

from oura_toolkit.auth import TokenStore, Tokens

pytestmark = pytest.mark.skipif(
    os.name == "nt", reason="flock interop guarantee is Unix-only"
)

# Holds an exclusive flock on argv[1] exactly like Rust's File::lock, then waits for
# stdin EOF. Prints HELD only once the lock is actually acquired.
RAW_FLOCK_HOLDER = textwrap.dedent(
    """
    import fcntl, os, sys
    fd = os.open(sys.argv[1], os.O_RDWR | os.O_CREAT, 0o600)
    fcntl.flock(fd, fcntl.LOCK_EX)
    print("HELD", flush=True)
    sys.stdin.read()  # hold until the parent closes our stdin
    """
)

# A second PROCESS sharing the store: refreshes via its own TokenManager against the
# parent's mock endpoint and prints the access token it ends up with. Before attempting
# the refresh it drops a unique "started" sentinel in argv[3] so the parent's slow
# handler can block deterministically until BOTH processes are live and contending for
# the store lock (no fixed sleep window — see the test's slow_rotate).
CONCURRENT_REFRESHER = textwrap.dedent(
    """
    import os, sys
    from oura_toolkit.auth import ClientCredentials, TokenManager, TokenStore, Tokens

    store_dir, token_url, started_dir = sys.argv[1], sys.argv[2], sys.argv[3]
    # Announce liveness BEFORE touching the lock, so the sentinel appears whether this
    # process wins the lock or blocks waiting for it.
    open(os.path.join(started_dir, str(os.getpid())), "w").close()
    manager = TokenManager(
        TokenStore(store_dir),
        ClientCredentials(client_id="cid", client_secret="secret"),
        Tokens(access_token="stale-access-r1", refresh_token="r1", expires_at=0),
        token_url=token_url,
    )
    print(manager.access_token(), flush=True)
    """
)


class TestFlockMutualExclusion:
    def test_lock_is_excluded_by_a_raw_flock_holder_in_another_process(
        self, tmp_path: Path
    ) -> None:
        store = TokenStore(tmp_path)
        store._ensure_dir()
        holder = subprocess.Popen(
            [sys.executable, "-c", RAW_FLOCK_HOLDER, str(tmp_path / ".lock")],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True,
        )
        try:
            assert holder.stdout.readline().strip() == "HELD"
            # While the other process holds the raw flock, we must NOT acquire.
            assert store.try_lock_exclusive() is None, (
                "try_lock_exclusive acquired a lock another process holds via "
                "flock — the Rust CLI would not be excluded"
            )
        finally:
            holder.stdin.close()  # release: holder exits, dropping the flock
            assert holder.wait(timeout=10) == 0
        lock = store.try_lock_exclusive()
        assert lock is not None, "lock must be acquirable after the holder exits"
        lock.release()

    def test_held_lock_excludes_a_raw_flock_taker_in_another_process(
        self, tmp_path: Path
    ) -> None:
        # The other direction: OUR lock must block a raw flock (i.e. the Rust side).
        probe = textwrap.dedent(
            """
            import fcntl, os, sys
            fd = os.open(sys.argv[1], os.O_RDWR | os.O_CREAT, 0o600)
            try:
                fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
                print("ACQUIRED", flush=True)
            except OSError:
                print("BLOCKED", flush=True)
            """
        )
        store = TokenStore(tmp_path)
        with store.lock_exclusive():
            out = subprocess.run(
                [sys.executable, "-c", probe, str(tmp_path / ".lock")],
                capture_output=True,
                text=True,
                timeout=30,
            )
            assert out.stdout.strip() == "BLOCKED", (
                "a raw flock in another process acquired the lock we hold"
            )
        out = subprocess.run(
            [sys.executable, "-c", probe, str(tmp_path / ".lock")],
            capture_output=True,
            text=True,
            timeout=30,
        )
        assert out.stdout.strip() == "ACQUIRED", "lock must be free after release"


class TestTwoProcessRefresh:
    def test_concurrent_refreshes_across_processes_make_one_endpoint_call(
        self, token_endpoint, tmp_path: Path
    ) -> None:
        # The lock's reason to exist: the endpoint blocks the FIRST refresh until BOTH
        # child processes are live and contending, guaranteeing overlap DETERMINISTICALLY
        # (no fixed sleep). A no-op lock then lets both send r1 — the second replay is
        # visible in the endpoint's request count. With a real lock the loser blocks,
        # reloads, and adopts, so exactly one request ever reaches the endpoint.
        started_dir = tmp_path / "started"
        started_dir.mkdir()

        def slow_rotate(form):
            if form.get("refresh_token") != "r1":
                return (500, {"error": f"unexpected refresh_token {form!r}"})
            # Hold this (winning) refresh open until both children have announced they
            # are alive — i.e. the loser is now blocked on the store lock (real lock) or
            # racing us to the endpoint (no-op lock). Hard 10s escape so a hang fails the
            # test rather than wedging the suite.
            deadline = time.monotonic() + 10.0
            while len(list(started_dir.iterdir())) < 2:
                if time.monotonic() >= deadline:
                    break
                time.sleep(0.01)
            return (
                200,
                {
                    "access_token": "fresh-access",
                    "refresh_token": "r2",
                    "expires_in": 3600,
                },
            )

        token_endpoint.handler = slow_rotate
        store = TokenStore(tmp_path)
        store.save_tokens(
            Tokens(access_token="stale-access-r1", refresh_token="r1", expires_at=0)
        )

        children = [
            subprocess.Popen(
                [
                    sys.executable,
                    "-c",
                    CONCURRENT_REFRESHER,
                    str(tmp_path),
                    token_endpoint.url,
                    str(started_dir),
                ],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
            )
            for _ in range(2)
        ]
        results = [child.communicate(timeout=60) for child in children]
        for child, (stdout, stderr) in zip(children, results):
            assert child.returncode == 0, f"child refresh failed: {stderr}"
            assert stdout.strip() == "fresh-access"

        assert len(token_endpoint.requests) == 1, (
            "two concurrent cross-process refreshes must serialize to exactly one "
            "endpoint call (replayed r1 or burned r2 otherwise)"
        )
        assert store.load_tokens().refresh_token == "r2"
