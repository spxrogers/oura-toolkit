"""Secret-redaction attack tests (CLAUDE.md TESTING & VERIFICATION rule 5): every
object that holds a secret is rendered every way Python renders things — ``repr()``,
``str()``, f-string, and inside containers — and must never leak the secret."""

from __future__ import annotations

from pathlib import Path

from oura_toolkit.auth import ClientCredentials, TokenManager, TokenStore, Tokens

SECRETS = ("SECRET-CS-789", "SECRET-AT-123", "SECRET-RT-456")

CREDENTIALS = ClientCredentials(client_id="cid", client_secret="SECRET-CS-789")
TOKENS = Tokens(
    access_token="SECRET-AT-123",
    refresh_token="SECRET-RT-456",
    expires_at=4_102_444_800,
    scope="daily personal",
    token_type="Bearer",
)


def renderings(obj: object):
    """Every stringification path an accidental log line could take."""
    return (
        repr(obj),
        str(obj),
        f"{obj}",
        repr([obj]),  # container repr goes through __repr__ too
        repr({"key": obj}),
        "{}".format(obj),
    )


def assert_no_secret(obj: object) -> None:
    for text in renderings(obj):
        for secret in SECRETS:
            assert secret not in text, (
                f"secret {secret!r} leaked from {type(obj).__name__}: {text}"
            )


def test_client_credentials_redact_the_secret_but_keep_the_id() -> None:
    assert_no_secret(CREDENTIALS)
    assert "cid" in repr(CREDENTIALS), "client_id should remain visible for debugging"
    assert "[REDACTED]" in repr(CREDENTIALS)


def test_tokens_redact_both_token_fields_but_keep_metadata() -> None:
    assert_no_secret(TOKENS)
    text = repr(TOKENS)
    assert "[REDACTED]" in text
    assert "4102444800" in text, "expires_at should remain visible for debugging"
    assert "daily personal" in text, "scope is not a secret and aids debugging"


def test_token_manager_redacts_everything_it_holds(tmp_path: Path) -> None:
    manager = TokenManager(TokenStore(tmp_path), CREDENTIALS, TOKENS)
    assert_no_secret(manager)


def test_store_repr_carries_no_secrets(tmp_path: Path) -> None:
    store = TokenStore(tmp_path)
    store.save_credentials(CREDENTIALS)
    store.save_tokens(TOKENS)
    assert_no_secret(store)
