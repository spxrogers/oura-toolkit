package auth

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync/atomic"
	"testing"
)

// A hostile or broken 2xx from the token endpoint must fail as a typed *TokenEndpointError,
// AccessToken must error (never return ("", nil)), and — the burn-prevention guarantee —
// the on-disk refresh token must be UNCHANGED (a half-parsed 2xx must not overwrite a still
// valid refresh token with garbage, which would 400 every future refresh). The typed
// error's Body is a fixed, secret-free description that never echoes response material.
func TestRefreshRejectsHostile2xxTypedAndLeavesStoreUntouched(t *testing.T) {
	cases := []struct {
		name string
		body string
	}{
		{"empty object", `{}`},
		{"empty body", ``},
		{"non-JSON", `not json at all`},
		{"missing access_token only", `{"refresh_token":"r2-INJECTED","expires_in":3600}`},
		{"missing expires_in only", `{"access_token":"a2-INJECTED","refresh_token":"r2-INJECTED"}`},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			var calls atomic.Int32
			srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				calls.Add(1)
				w.Header().Set("Content-Type", "application/json")
				w.WriteHeader(http.StatusOK)
				io.WriteString(w, tc.body)
			}))
			defer srv.Close()

			store := NewStoreAt(t.TempDir())
			if err := store.SaveTokens(expiredTokens("r1")); err != nil {
				t.Fatal(err)
			}
			m := testManager(t, srv.URL, store, expiredTokens("r1"))

			got, err := m.AccessToken(context.Background())
			if got != "" {
				t.Fatalf("a hostile 2xx must not yield an access token, got %q", got)
			}
			var te *TokenEndpointError
			if !errors.As(err, &te) {
				t.Fatalf("hostile 2xx must surface a typed *TokenEndpointError, got %T: %v", err, err)
			}
			if te.Status != http.StatusOK {
				t.Fatalf("the 2xx status must be preserved (so the manager's 400-retry arm never misfires), got %d", te.Status)
			}
			for _, leak := range []string{"a2-INJECTED", "r2-INJECTED"} {
				if strings.Contains(te.Body, leak) {
					t.Fatalf("typed error body must not echo response material (%q leaked): %q", leak, te.Body)
				}
			}
			// Burn-prevention: the store still holds the original, still-valid refresh token.
			disk, derr := store.LoadTokens()
			if derr != nil {
				t.Fatal(derr)
			}
			if disk == nil || disk.RefreshToken != "r1" || disk.AccessToken != "stale-access-r1" {
				t.Fatalf("hostile 2xx must leave the store untouched, disk=%v", disk)
			}
			if n := calls.Load(); n != 1 {
				t.Fatalf("a hostile 2xx (non-400) must not trigger the reload-retry arm: want 1 call, got %d", n)
			}
		})
	}
}

// A confidential client must never re-POST its client_secret to a redirect target. The
// token-endpoint HTTP client refuses to follow redirects: a 3xx from the endpoint surfaces
// as an error and the redirect target is never contacted (so the secret cannot leak there).
func TestTokenEndpointClientRefusesRedirects(t *testing.T) {
	var targetHits atomic.Int32
	target := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		targetHits.Add(1)
		// If the client wrongly followed, it would deliver client_id/client_secret here
		// and this would look like a successful refresh.
		w.Header().Set("Content-Type", "application/json")
		_ = json.NewEncoder(w).Encode(map[string]any{
			"access_token": "leaked-access", "refresh_token": "r2", "expires_in": 3600,
		})
	}))
	defer target.Close()

	redirector := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, target.URL, http.StatusFound)
	}))
	defer redirector.Close()

	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(expiredTokens("r1")); err != nil {
		t.Fatal(err)
	}
	m := testManager(t, redirector.URL, store, expiredTokens("r1"))

	got, err := m.AccessToken(context.Background())
	if err == nil {
		t.Fatalf("a redirect from the token endpoint must surface an error, not be followed (got token %q)", got)
	}
	if n := targetHits.Load(); n != 0 {
		t.Fatalf("confidential client must NOT re-POST client_secret to a redirect target: target contacted %d time(s)", n)
	}
	// The store keeps its original refresh token — the redirect did not rotate anything.
	disk, _ := store.LoadTokens()
	if disk == nil || disk.RefreshToken != "r1" {
		t.Fatalf("a refused redirect must leave the store untouched, disk=%v", disk)
	}
}
