package auth

import (
	"context"
	"errors"
	"net/http"
	"sync"
	"time"

	"golang.org/x/oauth2"
)

// defaultSkew: refresh this long before the token's actual expiry (proactive refresh),
// mirroring the Rust companion's 60s.
const defaultSkew = 60 * time.Second

// tokenEndpointTimeout is the HARD timeout on each token-endpoint call. It bounds how
// long the store's exclusive lock can be held (the refresh runs under it) — without it,
// one process's stalled refresh would wedge every other process waiting on the lock.
// Worst case is ~2× this value: the 400-retry arm can chain a second endpoint call under
// the same lock.
const tokenEndpointTimeout = 30 * time.Second

// Manager owns the current tokens and the machinery to keep them fresh. It implements
// oauth2.TokenSource, which is the seam into the generated client (api.ContextOAuth2) —
// see the package example. Safe for concurrent use.
type Manager struct {
	store *Store
	creds *ClientCredentials

	mu     sync.Mutex
	tokens *Tokens

	// http is a plain client (no auth) for token-endpoint calls. Its timeout is
	// load-bearing: the call runs under the store's exclusive lock, so an unbounded hang
	// would block other processes too.
	http *http.Client
	skew time.Duration
	// tokenURL is the spec-derived token endpoint; overridden only by tests.
	tokenURL string
}

// LoadManager loads from the default token store. Absent records are not an error —
// AccessToken reports ErrNotAuthenticated on first use.
func LoadManager() (*Manager, error) {
	store, err := NewStore()
	if err != nil {
		return nil, err
	}
	creds, err := store.LoadCredentials()
	if err != nil {
		return nil, err
	}
	tokens, err := store.LoadTokens()
	if err != nil {
		return nil, err
	}
	return NewManager(store, creds, tokens), nil
}

// NewManager constructs from an explicit store + optional in-memory records.
//
// Both records are independently optional because both partial states are legitimate:
// credentials-without-tokens is `oura auth setup` completed but no login yet
// (refresh-able once tokens arrive), and tokens-without-credentials is a caller-supplied
// token that can be used until expiry but not refreshed (ErrMissingClientCredentials).
func NewManager(store *Store, creds *ClientCredentials, tokens *Tokens) *Manager {
	return &Manager{
		store:  store,
		creds:  creds,
		tokens: tokens,
		http: &http.Client{
			Timeout: tokenEndpointTimeout,
			// Refuse to follow redirects: a confidential client must never re-POST its
			// client_secret to a redirect target. Returning ErrUseLastResponse hands the
			// 3xx back so refreshTokens surfaces it as a TokenEndpointError instead.
			CheckRedirect: func(*http.Request, []*http.Request) error {
				return http.ErrUseLastResponse
			},
		},
		skew:     defaultSkew,
		tokenURL: TokenURL,
	}
}

// IsAuthenticated reports whether tokens are loaded (it does not validate them, and does
// not imply a refresh is possible — refresh additionally needs the credentials record).
func (m *Manager) IsAuthenticated() bool {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.tokens != nil
}

// AccessToken returns a valid access token, refreshing proactively (and persisting the
// rotation) if it is expired or within the skew window.
func (m *Manager) AccessToken(ctx context.Context) (string, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if m.tokens == nil {
		return "", ErrNotAuthenticated
	}
	if m.tokens.expired(m.skew) {
		if err := m.refreshCriticalSection(ctx); err != nil {
			return "", err
		}
	}
	return m.tokens.AccessToken, nil
}

// ForceRefresh refreshes regardless of expiry (for callers reacting to a 401),
// persisting the rotation. If another process already rotated, its fresher tokens are
// adopted instead of burning that rotation with a second refresh.
func (m *Manager) ForceRefresh(ctx context.Context) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.refreshCriticalSection(ctx)
}

// Token implements oauth2.TokenSource: it returns a fresh *oauth2.Token, refreshing and
// persisting as needed. Wire it into the generated client with api.ContextOAuth2 (see
// the package example). The refresh token is deliberately NOT copied into the returned
// oauth2.Token — the data plane only needs the Bearer value, and rotation stays this
// package's job.
//
// A refresh triggered here uses a background context — it is NOT cancellable by the
// caller's request context; it is bounded only by the token-endpoint timeout (the hard 30s
// that also bounds how long the store lock is held).
func (m *Manager) Token() (*oauth2.Token, error) {
	if _, err := m.AccessToken(context.Background()); err != nil {
		return nil, err
	}
	m.mu.Lock()
	defer m.mu.Unlock()
	tokenType := m.tokens.TokenType
	if tokenType == "" {
		tokenType = "Bearer"
	}
	return &oauth2.Token{
		AccessToken: m.tokens.AccessToken,
		TokenType:   tokenType,
		Expiry:      time.Unix(m.tokens.ExpiresAt, 0),
	}, nil
}

// refreshCriticalSection is the reload → refresh → persist critical section, run under
// the store's exclusive advisory lock so only one PROCESS rotates at a time (m.mu is
// already held, serializing goroutines within this one).
//
// The adopt rule covers both entry points: if disk holds tokens that differ from memory
// and aren't expired, another process already rotated — adopt them. (On the proactive
// path memory is expired, so anything fresher is strictly better; on the force path
// memory just 401'd, so a *different* fresh token is the fix and an *identical* one means
// we must rotate.)
func (m *Manager) refreshCriticalSection(ctx context.Context) error {
	if m.creds == nil {
		return ErrMissingClientCredentials
	}

	lock, err := m.store.LockExclusive()
	if err != nil {
		return err
	}
	defer lock.Unlock()

	if disk, err := m.store.LoadTokens(); err != nil {
		return err
	} else if disk != nil {
		differs := m.tokens == nil || m.tokens.AccessToken != disk.AccessToken
		if differs && !disk.expired(m.skew) {
			m.tokens = disk
			return nil
		}
		// Refresh from the freshest persisted rotation, never from stale memory.
		m.tokens = disk
	}
	if m.tokens == nil {
		return ErrNotAuthenticated
	}

	refreshed, err := refreshTokens(ctx, m.http, m.tokenURL, m.creds, m.tokens)
	if err != nil {
		// A 400 usually means the refresh token we sent is no longer valid. If disk has
		// moved past what we sent (a rotation by a process not using the lock), retry
		// ONCE with the fresher token before surfacing "re-login".
		var te *TokenEndpointError
		if !errors.As(err, &te) || te.Status != http.StatusBadRequest {
			return err
		}
		disk, lerr := m.store.LoadTokens()
		if lerr != nil {
			return lerr
		}
		if disk == nil || disk.RefreshToken == m.tokens.RefreshToken {
			return err // genuinely invalid — no blind retry
		}
		refreshed, err = refreshTokens(ctx, m.http, m.tokenURL, m.creds, disk)
		if err != nil {
			return err
		}
	}

	if err := m.store.SaveTokens(refreshed); err != nil {
		return err
	}
	m.tokens = refreshed
	return nil
}
