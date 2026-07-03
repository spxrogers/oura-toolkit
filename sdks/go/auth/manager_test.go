package auth

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"net/http/httptest"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

func expiredTokens(refreshToken string) *Tokens {
	return &Tokens{
		AccessToken:  "stale-access-" + refreshToken,
		RefreshToken: refreshToken,
		ExpiresAt:    0,
	}
}

func testManager(t *testing.T, endpoint string, store *Store, tokens *Tokens) *Manager {
	t.Helper()
	m := NewManager(store, sampleCredentials(), tokens)
	m.tokenURL = endpoint
	return m
}

// tokenEndpoint is a hermetic stand-in for Oura's token endpoint: it accepts exactly the
// refresh tokens in `grants` (each once, confidential-client form required), rotating to
// the mapped next token, and 400s anything else — replaying a burned token or exceeding
// the expected call budget fails the calling test, mirroring Oura's
// rotate-and-invalidate behavior.
type tokenEndpoint struct {
	t      *testing.T
	server *httptest.Server
	calls  atomic.Int32
	mu     sync.Mutex
	grants map[string]tokenGrant
}

type tokenGrant struct {
	access  string
	refresh string
	// beforeRespond runs while the request is in flight (e.g. simulate an uncoordinated
	// rotation landing on disk); the response is 400 if fail400 is set.
	beforeRespond func()
	fail400       bool
}

func newTokenEndpoint(t *testing.T) *tokenEndpoint {
	e := &tokenEndpoint{t: t, grants: map[string]tokenGrant{}}
	e.server = httptest.NewServer(http.HandlerFunc(e.handle))
	t.Cleanup(e.server.Close)
	return e
}

func (e *tokenEndpoint) url() string { return e.server.URL }

func (e *tokenEndpoint) grant(refreshIn string, g tokenGrant) {
	e.mu.Lock()
	defer e.mu.Unlock()
	e.grants[refreshIn] = g
}

func (e *tokenEndpoint) handle(w http.ResponseWriter, r *http.Request) {
	e.calls.Add(1)
	if err := r.ParseForm(); err != nil {
		e.t.Errorf("token endpoint: bad form: %v", err)
	}
	// The confidential-client contract: EVERY token call carries grant_type,
	// client_id AND client_secret (no PKCE / public-client path).
	if got := r.PostFormValue("grant_type"); got != "refresh_token" {
		e.t.Errorf("token endpoint: grant_type = %q, want refresh_token", got)
	}
	if r.PostFormValue("client_id") != "cid" || r.PostFormValue("client_secret") != "SECRET-CS-789" {
		e.t.Error("token endpoint: request missing client_id/client_secret — confidential-client contract broken")
	}

	sent := r.PostFormValue("refresh_token")
	e.mu.Lock()
	g, ok := e.grants[sent]
	if ok {
		delete(e.grants, sent) // each refresh token is single-use (Oura invalidates it)
	}
	e.mu.Unlock()

	if g.beforeRespond != nil {
		g.beforeRespond()
	}
	if !ok || g.fail400 {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprint(w, "invalid_grant")
		return
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]any{
		"access_token":  g.access,
		"refresh_token": g.refresh,
		"expires_in":    3600,
		"token_type":    "Bearer",
	})
}

func TestAccessTokenRequiresAuthentication(t *testing.T) {
	m := NewManager(NewStoreAt(t.TempDir()), sampleCredentials(), nil)
	if m.IsAuthenticated() {
		t.Fatal("no tokens loaded but IsAuthenticated is true")
	}
	if _, err := m.AccessToken(context.Background()); !errors.Is(err, ErrNotAuthenticated) {
		t.Fatalf("want ErrNotAuthenticated, got %v", err)
	}
}

func TestRefreshWithoutCredentialsReportsMissingCredentials(t *testing.T) {
	m := NewManager(NewStoreAt(t.TempDir()), nil, expiredTokens("r1"))
	if _, err := m.AccessToken(context.Background()); !errors.Is(err, ErrMissingClientCredentials) {
		t.Fatalf("want ErrMissingClientCredentials, got %v", err)
	}
}

// The rotation-persistence guarantee: a refresh must persist the NEWLY returned refresh
// token to disk (Oura invalidates the old one — an unpersisted rotation 400s next time).
func TestRefreshRotatesAndPersistsTheNewRefreshToken(t *testing.T) {
	endpoint := newTokenEndpoint(t)
	endpoint.grant("r1", tokenGrant{access: "fresh-access", refresh: "r2"})

	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(expiredTokens("r1")); err != nil {
		t.Fatal(err)
	}
	m := testManager(t, endpoint.url(), store, expiredTokens("r1"))

	got, err := m.AccessToken(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	if got != "fresh-access" {
		t.Fatalf("want the refreshed access token, got %q", got)
	}
	disk, err := store.LoadTokens()
	if err != nil {
		t.Fatal(err)
	}
	if disk.RefreshToken != "r2" {
		t.Fatalf("rotated refresh token was not persisted: disk has %q, want r2", disk.RefreshToken)
	}
	if disk.AccessToken != "fresh-access" {
		t.Fatalf("refreshed access token was not persisted: disk has %q", disk.AccessToken)
	}
}

// The reload-adopt rule: a manager whose memory is stale must re-read the store under the
// lock and ADOPT a rotation another process already performed — never call the endpoint
// with the burned token (the single-use grant would 400 a second call).
func TestSecondManagerAdoptsRotationFromDiskWithoutCallingEndpoint(t *testing.T) {
	endpoint := newTokenEndpoint(t)
	endpoint.grant("r1", tokenGrant{access: "fresh-access", refresh: "r2"})

	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(expiredTokens("r1")); err != nil {
		t.Fatal(err)
	}
	// Both managers start from the same stale state (r1, expired) — the pre-#22 failure
	// mode: B's refresh would replay the invalidated r1 and 400.
	a := testManager(t, endpoint.url(), store, expiredTokens("r1"))
	b := testManager(t, endpoint.url(), store, expiredTokens("r1"))

	if got, err := a.AccessToken(context.Background()); err != nil || got != "fresh-access" {
		t.Fatalf("manager A: got (%q, %v)", got, err)
	}
	if got, err := b.AccessToken(context.Background()); err != nil || got != "fresh-access" {
		t.Fatalf("manager B must adopt A's rotation from disk: got (%q, %v)", got, err)
	}
	if n := endpoint.calls.Load(); n != 1 {
		t.Fatalf("exactly one endpoint call allowed (the adopt path makes none), got %d", n)
	}
	disk, _ := store.LoadTokens()
	if disk.RefreshToken != "r2" {
		t.Fatalf("rotation persisted exactly once, want r2 on disk, got %q", disk.RefreshToken)
	}
}

func TestForceRefreshAdoptsFresherDiskState(t *testing.T) {
	endpoint := newTokenEndpoint(t)
	endpoint.grant("r1", tokenGrant{access: "fresh-access", refresh: "r2"})

	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(expiredTokens("r1")); err != nil {
		t.Fatal(err)
	}
	a := testManager(t, endpoint.url(), store, expiredTokens("r1"))
	b := testManager(t, endpoint.url(), store, expiredTokens("r1"))

	if got, err := a.AccessToken(context.Background()); err != nil || got != "fresh-access" {
		t.Fatalf("manager A: got (%q, %v)", got, err)
	}
	// B's request 401'd (stale memory) and it force-refreshes: it must adopt the disk
	// rotation rather than burn r2 with another endpoint call.
	if err := b.ForceRefresh(context.Background()); err != nil {
		t.Fatal(err)
	}
	if got, err := b.AccessToken(context.Background()); err != nil || got != "fresh-access" {
		t.Fatalf("manager B after ForceRefresh: got (%q, %v)", got, err)
	}
	if n := endpoint.calls.Load(); n != 1 {
		t.Fatalf("ForceRefresh must adopt, not re-burn: want 1 endpoint call, got %d", n)
	}
}

// The lock's reason to exist: managers refreshing CONCURRENTLY (real goroutines, separate
// lock fds — the in-process equivalent of CLI + MCP server) must serialize to exactly one
// endpoint call; the losers adopt the winner's persisted rotation. The endpoint's 50ms
// in-flight delay widens the race window so a no-op lock fails deterministically (both
// would send the single-use r1; the second gets 400).
func TestConcurrentRefreshesSerializeToASingleEndpointCall(t *testing.T) {
	endpoint := newTokenEndpoint(t)
	endpoint.grant("r1", tokenGrant{
		access: "fresh-access", refresh: "r2",
		beforeRespond: func() { time.Sleep(50 * time.Millisecond) },
	})

	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(expiredTokens("r1")); err != nil {
		t.Fatal(err)
	}

	const workers = 4
	managers := make([]*Manager, workers)
	for i := range managers {
		managers[i] = testManager(t, endpoint.url(), store, expiredTokens("r1"))
	}

	var wg sync.WaitGroup
	results := make([]string, workers)
	errs := make([]error, workers)
	for i, m := range managers {
		wg.Add(1)
		go func() {
			defer wg.Done()
			results[i], errs[i] = m.AccessToken(context.Background())
		}()
	}
	wg.Wait()

	for i := range managers {
		if errs[i] != nil {
			t.Fatalf("manager %d: %v (a concurrent double-refresh burned the rotation?)", i, errs[i])
		}
		if results[i] != "fresh-access" {
			t.Fatalf("manager %d: got %q", i, results[i])
		}
	}
	if n := endpoint.calls.Load(); n != 1 {
		t.Fatalf("concurrent refreshes must serialize to ONE endpoint call, got %d", n)
	}
	disk, _ := store.LoadTokens()
	if disk.RefreshToken != "r2" {
		t.Fatalf("want r2 persisted, got %q", disk.RefreshToken)
	}
}

// The 400-retry arm: a rotation performed by a writer NOT holding the lock lands on disk
// after our reload but before our request is answered. The endpoint 400s the stale token;
// the manager must reload, see the fresher refresh token, and retry ONCE with it.
func TestRefresh400ReloadsAndRetriesOnceAgainstFresherDiskState(t *testing.T) {
	endpoint := newTokenEndpoint(t)
	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(expiredTokens("r1")); err != nil {
		t.Fatal(err)
	}
	endpoint.grant("r1", tokenGrant{
		fail400: true,
		beforeRespond: func() {
			// Simulate an uncoordinated writer rotating to r2 (still expired, so the
			// retry must actually refresh) while our r1 request is in flight.
			if err := store.SaveTokens(&Tokens{
				AccessToken: "r2-access", RefreshToken: "r2", ExpiresAt: 0,
			}); err != nil {
				t.Error(err)
			}
		},
	})
	endpoint.grant("r2", tokenGrant{access: "r3-access", refresh: "r3"})

	m := testManager(t, endpoint.url(), store, expiredTokens("r1"))
	got, err := m.AccessToken(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	if got != "r3-access" {
		t.Fatalf("the 400 must trigger exactly one reload-retry: got %q, want r3-access", got)
	}
	if n := endpoint.calls.Load(); n != 2 {
		t.Fatalf("want exactly 2 endpoint calls (r1 → 400, r2 → ok), got %d", n)
	}
	disk, _ := store.LoadTokens()
	if disk.RefreshToken != "r3" {
		t.Fatalf("retry's rotation must be persisted: got %q, want r3", disk.RefreshToken)
	}
}

func TestGenuinelyInvalidRefreshTokenSurfaces400WithoutBlindRetry(t *testing.T) {
	endpoint := newTokenEndpoint(t) // no grants: everything 400s
	store := NewStoreAt(t.TempDir())
	if err := store.SaveTokens(expiredTokens("r-dead")); err != nil {
		t.Fatal(err)
	}
	m := testManager(t, endpoint.url(), store, expiredTokens("r-dead"))

	_, err := m.AccessToken(context.Background())
	var te *TokenEndpointError
	if !errors.As(err, &te) || te.Status != 400 {
		t.Fatalf("want TokenEndpointError 400, got %v", err)
	}
	if n := endpoint.calls.Load(); n != 1 {
		t.Fatalf("the reload-retry fires only when disk moved past what we sent: want 1 call, got %d", n)
	}
}

// Proactive refresh honors the expiry skew: a token 30s from expiry is refreshed under
// the default 60s skew, but served as-is when the skew window doesn't reach it.
func TestProactiveRefreshUsesExpirySkew(t *testing.T) {
	endpoint := newTokenEndpoint(t)
	endpoint.grant("r1", tokenGrant{access: "fresh-access", refresh: "r2"})

	soon := &Tokens{AccessToken: "still-good", RefreshToken: "r1", ExpiresAt: time.Now().Unix() + 30}

	// 30s out, default 60s skew: inside the window — must refresh.
	m := testManager(t, endpoint.url(), NewStoreAt(t.TempDir()), soon)
	if got, err := m.AccessToken(context.Background()); err != nil || got != "fresh-access" {
		t.Fatalf("30s-out token must be refreshed under the 60s skew: got (%q, %v)", got, err)
	}
	if n := endpoint.calls.Load(); n != 1 {
		t.Fatalf("want 1 skew-triggered refresh, got %d", n)
	}

	// Same expiry, no skew: outside the window — must serve the current token untouched.
	noSkew := testManager(t, endpoint.url(), NewStoreAt(t.TempDir()), soon)
	noSkew.skew = 0
	if got, err := noSkew.AccessToken(context.Background()); err != nil || got != "still-good" {
		t.Fatalf("30s-out token with no skew must be served as-is: got (%q, %v)", got, err)
	}
	if n := endpoint.calls.Load(); n != 1 {
		t.Fatalf("no-skew manager must not refresh: total calls %d", n)
	}
}

// The oauth2.TokenSource seam into the generated client (api.ContextOAuth2): Token()
// refreshes when needed and returns a Bearer token the client can inject — without ever
// exposing the refresh token to the data plane.
func TestTokenSourceRefreshesAndOmitsTheRefreshToken(t *testing.T) {
	endpoint := newTokenEndpoint(t)
	endpoint.grant("r1", tokenGrant{access: "fresh-access", refresh: "r2"})

	store := NewStoreAt(t.TempDir())
	m := testManager(t, endpoint.url(), store, expiredTokens("r1"))

	tok, err := m.Token()
	if err != nil {
		t.Fatal(err)
	}
	if tok.AccessToken != "fresh-access" {
		t.Fatalf("TokenSource must serve the refreshed token, got %q", tok.AccessToken)
	}
	if tok.TokenType != "Bearer" {
		t.Fatalf("want Bearer token type, got %q", tok.TokenType)
	}
	if !tok.Valid() {
		t.Fatal("returned oauth2.Token must be valid (future expiry set)")
	}
	if tok.RefreshToken != "" {
		t.Fatal("the refresh token must NOT leak into the data-plane oauth2.Token")
	}
	// Rotation persisted through the seam too.
	disk, _ := store.LoadTokens()
	if disk == nil || disk.RefreshToken != "r2" {
		t.Fatalf("Token() must persist the rotation, disk: %v", disk)
	}

	// Absent tokens: a structured, sentinel error (the "run oura auth login" signal).
	empty := NewManager(NewStoreAt(t.TempDir()), sampleCredentials(), nil)
	if _, err := empty.Token(); !errors.Is(err, ErrNotAuthenticated) {
		t.Fatalf("want ErrNotAuthenticated from the TokenSource, got %v", err)
	}
}

// The token-endpoint timeout bounds lock-hold time (CLAUDE.md: hard 30s) — a manager
// without it would let one stalled refresh wedge every process waiting on the store lock.
func TestTokenEndpointClientHasTheHard30sTimeout(t *testing.T) {
	m := NewManager(NewStoreAt(t.TempDir()), sampleCredentials(), nil)
	if m.http.Timeout != 30*time.Second {
		t.Fatalf("token-endpoint client timeout must be the documented hard 30s, got %v", m.http.Timeout)
	}
}
