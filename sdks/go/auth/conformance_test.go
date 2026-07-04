// Cross-language auth-companion conformance (#58) — the GO leg.
//
// Iterates codegen/conformance/auth-cases.json (the single source for the hostile
// token-endpoint responses, hostile store files, and canonical store records that every
// companion suite must exercise; cases are added THERE, never here):
//
//   - hostile-but-2xx token responses → typed *TokenEndpointError, store UNTOUCHED (the
//     rotated refresh token is never burned by persisting a blank/expired Bearer);
//   - hostile store files → typed *StoreFormatError, never a zero-valued record that
//     would make IsAuthenticated lie, and never a panic;
//   - canonical valid records → load with exactly the fixture's field values and
//     round-trip through this package's own persist path (the cross-language store
//     compatibility check — field names are the shared wire format, #54).
//
// Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs).
package auth

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"sync/atomic"
	"testing"
)

// conformanceFixture is the decoded shape of codegen/conformance/auth-cases.json.
type conformanceFixture struct {
	HostileTokenResponses []struct {
		Name string `json:"name"`
		// Body is kept raw: the server replays the fixture's JSON verbatim, so a
		// wrong-typed field (42, "soon") reaches the companion exactly as authored.
		Body    json.RawMessage `json:"body"`
		RawBody *string         `json:"raw_body"`
	} `json:"hostile_token_responses"`
	HostileStoreFiles []struct {
		Name    string `json:"name"`
		File    string `json:"file"`
		Content string `json:"content"`
	} `json:"hostile_store_files"`
	ValidRecords map[string]json.RawMessage `json:"valid_records"`
}

// loadConformanceFixture walks up from the package dir to the repo root (nearest
// ancestor holding the justfile + README — the same walk as the Rust leg) and decodes
// the shared fixture. Monorepo-only by design.
func loadConformanceFixture(t *testing.T) conformanceFixture {
	t.Helper()
	dir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	for {
		if fileExists(filepath.Join(dir, "justfile")) && fileExists(filepath.Join(dir, "README.md")) {
			break
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			t.Fatal("repo root (justfile + README.md) not found above the package dir")
		}
		dir = parent
	}
	data, err := os.ReadFile(filepath.Join(dir, "codegen", "conformance", "auth-cases.json"))
	if err != nil {
		t.Fatalf("reading the shared fixture: %v", err)
	}
	var f conformanceFixture
	if err := json.Unmarshal(data, &f); err != nil {
		t.Fatalf("fixture is not valid JSON: %v", err)
	}
	return f
}

func fileExists(path string) bool {
	info, err := os.Stat(path)
	return err == nil && !info.IsDir()
}

// Every hostile-but-2xx token response must fail the refresh with the typed
// *TokenEndpointError and leave the persisted record byte-identical — the rotated
// refresh token is never burned by a blank/expired Bearer. (A panic escaping to the
// caller fails the t.Run outright, so reaching the assertions proves "never a panic".)
func TestConformanceHostile2xxTokenResponsesFailTypedAndLeaveStoreUntouched(t *testing.T) {
	fixture := loadConformanceFixture(t)
	if n := len(fixture.HostileTokenResponses); n < 8 {
		t.Fatalf("fixture shrank? hostile_token_responses has %d cases, want >= 8", n)
	}

	for _, tc := range fixture.HostileTokenResponses {
		t.Run(tc.Name, func(t *testing.T) {
			var calls atomic.Int32
			srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				calls.Add(1)
				w.WriteHeader(http.StatusOK)
				if tc.RawBody != nil {
					// raw_body is replayed VERBATIM (deliberately not JSON).
					_, _ = w.Write([]byte(*tc.RawBody))
					return
				}
				w.Header().Set("Content-Type", "application/json")
				_, _ = w.Write(tc.Body)
			}))
			defer srv.Close()

			store := NewStoreAt(t.TempDir())
			if err := store.SaveCredentials(sampleCredentials()); err != nil {
				t.Fatal(err)
			}
			// Expired on purpose, so the refresh genuinely calls the endpoint.
			if err := store.SaveTokens(expiredTokens("r1")); err != nil {
				t.Fatal(err)
			}
			tokensBefore, err := os.ReadFile(store.TokensPath())
			if err != nil {
				t.Fatal(err)
			}
			credsBefore, err := os.ReadFile(store.CredentialsPath())
			if err != nil {
				t.Fatal(err)
			}

			m := testManager(t, srv.URL, store, expiredTokens("r1"))
			err = m.ForceRefresh(context.Background())
			if err == nil {
				t.Fatal("a hostile 2xx must not succeed")
			}
			// Typed: the companion's invalid-response error — never a raw
			// json.Unmarshal error escaping untyped, and never a mis-filed sentinel
			// that would trigger re-login remediation for a server-side fault.
			var te *TokenEndpointError
			if !errors.As(err, &te) {
				t.Fatalf("want the typed *TokenEndpointError, got %T: %v", err, err)
			}
			if te.Status != http.StatusOK {
				t.Fatalf("the 2xx status must be preserved (so the 400-retry arm never misfires), got %d", te.Status)
			}
			if n := calls.Load(); n != 1 {
				t.Fatalf("a hostile 2xx must not trigger the reload-retry arm: want 1 endpoint call, got %d", n)
			}

			tokensAfter, err := os.ReadFile(store.TokensPath())
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Equal(tokensBefore, tokensAfter) {
				t.Fatalf("tokens.json must be byte-identical (rotation not burned):\nbefore: %s\nafter:  %s", tokensBefore, tokensAfter)
			}
			credsAfter, err := os.ReadFile(store.CredentialsPath())
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Equal(credsBefore, credsAfter) {
				t.Fatal("credentials.json must be byte-identical after a failed refresh")
			}
		})
	}
}

// Every hostile store file must load as the typed *StoreFormatError — never a
// zero-valued record (which would make IsAuthenticated lie), never a panic.
func TestConformanceHostileStoreFilesFailTyped(t *testing.T) {
	fixture := loadConformanceFixture(t)
	if n := len(fixture.HostileStoreFiles); n < 8 {
		t.Fatalf("fixture shrank? hostile_store_files has %d cases, want >= 8", n)
	}

	for _, tc := range fixture.HostileStoreFiles {
		t.Run(tc.Name, func(t *testing.T) {
			store := NewStoreAt(t.TempDir())
			if err := os.WriteFile(filepath.Join(store.Dir(), tc.File), []byte(tc.Content), 0o600); err != nil {
				t.Fatal(err)
			}

			var loadErr error
			switch tc.File {
			case "tokens.json":
				tokens, err := store.LoadTokens()
				if tokens != nil {
					t.Fatalf("a hostile tokens.json must never yield a record, got %v", tokens)
				}
				loadErr = err
			case "credentials.json":
				creds, err := store.LoadCredentials()
				if creds != nil {
					t.Fatalf("a hostile credentials.json must never yield a record, got %v", creds)
				}
				loadErr = err
			default:
				t.Fatalf("fixture names an unknown store file %q", tc.File)
			}

			if loadErr == nil {
				t.Fatal("a hostile store file must not load")
			}
			var sfe *StoreFormatError
			if !errors.As(loadErr, &sfe) {
				t.Fatalf("want the typed *StoreFormatError, got %T: %v", loadErr, loadErr)
			}
		})
	}
}

// The canonical records load with exactly the fixture's values and survive a round-trip
// through this package's own persist path — the shared wire format every language reads
// (#54). The literal expectations double as a fixture-drift tripwire, mirroring the Rust
// reference leg.
func TestConformanceCanonicalValidRecordsLoadAndRoundTrip(t *testing.T) {
	fixture := loadConformanceFixture(t)
	credsRaw, ok := fixture.ValidRecords["credentials.json"]
	if !ok {
		t.Fatal("fixture is missing valid_records[credentials.json]")
	}
	tokensRaw, ok := fixture.ValidRecords["tokens.json"]
	if !ok {
		t.Fatal("fixture is missing valid_records[tokens.json]")
	}

	store := NewStoreAt(t.TempDir())
	if err := os.WriteFile(store.CredentialsPath(), credsRaw, 0o600); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(store.TokensPath(), tokensRaw, 0o600); err != nil {
		t.Fatal(err)
	}

	creds, err := store.LoadCredentials()
	if err != nil {
		t.Fatal(err)
	}
	if creds == nil {
		t.Fatal("canonical credentials.json must load")
	}
	if creds.ClientID != "cid-conformance" {
		t.Fatalf("client_id = %q, want cid-conformance", creds.ClientID)
	}
	if creds.ClientSecret != "cs-conformance" {
		t.Fatal("client_secret does not match the canonical record")
	}

	tokens, err := store.LoadTokens()
	if err != nil {
		t.Fatal(err)
	}
	if tokens == nil {
		t.Fatal("canonical tokens.json must load")
	}
	if tokens.AccessToken != "at-conformance" {
		t.Fatalf("access_token = %q, want at-conformance", tokens.AccessToken)
	}
	if tokens.RefreshToken != "rt-conformance" {
		t.Fatalf("refresh_token = %q, want rt-conformance", tokens.RefreshToken)
	}
	if tokens.ExpiresAt != 4_102_444_800 {
		t.Fatalf("expires_at = %d, want 4102444800", tokens.ExpiresAt)
	}
	if tokens.Scope != "personal daily" {
		t.Fatalf("scope = %q, want \"personal daily\"", tokens.Scope)
	}
	if tokens.TokenType != "Bearer" {
		t.Fatalf("token_type = %q, want Bearer", tokens.TokenType)
	}

	// Round-trip: this package's persist path must re-emit records the loader (and, by
	// the shared fixture, every other language) still reads identically.
	if err := store.SaveCredentials(creds); err != nil {
		t.Fatal(err)
	}
	if err := store.SaveTokens(tokens); err != nil {
		t.Fatal(err)
	}
	credsAgain, err := store.LoadCredentials()
	if err != nil {
		t.Fatal(err)
	}
	if credsAgain == nil || *credsAgain != *creds {
		t.Fatalf("credentials must round-trip through the persist path unchanged, got %v", credsAgain)
	}
	tokensAgain, err := store.LoadTokens()
	if err != nil {
		t.Fatal(err)
	}
	if tokensAgain == nil || *tokensAgain != *tokens {
		t.Fatalf("tokens must round-trip through the persist path unchanged, got %v", tokensAgain)
	}
}
