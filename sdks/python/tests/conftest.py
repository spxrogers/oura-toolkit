"""Shared fixtures: a hermetic stdlib mock token endpoint (no external test deps).

Everything here is loopback + tempdir: no network beyond 127.0.0.1, no real
credentials, no environment mutation (the store takes injected env lookups).
"""

from __future__ import annotations

import json
import threading
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from typing import Callable, Dict, List, Tuple
from urllib.parse import parse_qs

import pytest

# (status, json-able body or raw string)
Response = Tuple[int, object]
Handler = Callable[[Dict[str, str]], Response]


class MockTokenEndpoint:
    """A scriptable token endpoint. ``handler`` maps the POSTed form (flattened
    single-value dict) to a ``(status, body)`` response; every request form is
    recorded in ``requests`` for load-bearing assertions (call counts, exact
    refresh_token sent, client_secret present)."""

    def __init__(self) -> None:
        self.requests: List[Dict[str, str]] = []
        self._requests_mutex = threading.Lock()
        self.handler: Handler = lambda form: (500, {"error": "no handler installed"})

        endpoint = self

        class _RequestHandler(BaseHTTPRequestHandler):
            def do_POST(self) -> None:  # noqa: N802 (http.server API)
                length = int(self.headers.get("Content-Length", "0"))
                raw = self.rfile.read(length).decode("utf-8")
                form = {k: v[0] for k, v in parse_qs(raw).items()}
                with endpoint._requests_mutex:
                    endpoint.requests.append(form)
                status, body = endpoint.handler(form)
                payload = (
                    body.encode("utf-8")
                    if isinstance(body, str)
                    else json.dumps(body).encode("utf-8")
                )
                self.send_response(status)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(payload)))
                self.end_headers()
                self.wfile.write(payload)

            def log_message(self, *args: object) -> None:  # silence stderr noise
                pass

        # Bind an ephemeral port (0) so parallel test processes never collide, and let
        # in-flight handler threads be daemonic so a slow/blocking handler can never wedge
        # shutdown. Block until serve_forever has actually started before handing the
        # fixture out, so the first request can never race the server loop's startup.
        self._server = ThreadingHTTPServer(("127.0.0.1", 0), _RequestHandler)
        self._server.daemon_threads = True
        self._started = threading.Event()

        def _serve() -> None:
            self._started.set()
            self._server.serve_forever(poll_interval=0.02)

        self._thread = threading.Thread(target=_serve, daemon=True)
        self._thread.start()
        assert self._started.wait(timeout=5), "mock token endpoint failed to start"

    @property
    def url(self) -> str:
        host, port = self._server.server_address[:2]
        return f"http://{host}:{port}/oauth/token"

    def requests_with(self, refresh_token: str) -> List[Dict[str, str]]:
        with self._requests_mutex:
            return [r for r in self.requests if r.get("refresh_token") == refresh_token]

    def shutdown(self) -> None:
        self._server.shutdown()
        self._server.server_close()
        self._thread.join(timeout=5)


@pytest.fixture()
def token_endpoint():
    endpoint = MockTokenEndpoint()
    yield endpoint
    endpoint.shutdown()
