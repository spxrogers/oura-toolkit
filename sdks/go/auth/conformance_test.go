// Cross-language auth-companion conformance (#58) — the GO leg.
//
// Iterates codegen/conformance/auth-cases.json (the single source for the hostile
// token-endpoint responses, hostile store files, successful-refresh expectations, and
// canonical store records that every companion suite must exercise; cases are added
// THERE, never here — its `$comment` is the contract):
//
//   - hostile-but-2xx token responses (`body`, verbatim `raw_body`, or decoded
//     `raw_body_base64` bytes) → typed *TokenEndpointError, store UNTOUCHED (the
//     rotated refresh token is never burned by persisting a blank/expired Bearer; a
//     non-string refresh_token/token_type fails typed; a lone surrogate in any of the
//     four fields read — access_token, refresh_token, token_type, scope — is never
//     persisted; a body with invalid UTF-8 ANYWHERE fails; an expires_in outside
//     1..=2147483647, fractional, or a numeric string fails);
//   - hostile store files → typed *StoreFormatError, never a zero-valued record that
//     would make IsAuthenticated lie, and never a panic;
//   - canonical valid records → load with exactly the fixture's field values and
//     round-trip through this package's own persist path (the cross-language store
//     compatibility check — field names are the shared wire format, #54);
//   - successful refreshes from the fixture's `prior` record → the refresh SUCCEEDS and
//     persists EXACTLY `expected` (access_token, refresh_token, scope, token_type, and
//     expires_at = refresh time + expires_in): an omitted/null/empty/whitespace (incl.
//     U+00A0) or non-string scope keeps the prior grant; an omitted/null/EMPTY
//     refresh_token or token_type keeps the prior value; expires_in at the cap succeeds;
//     an unknown field is never validated;
//   - the fixture's top-level tables are exactly the four above, so a renamed/added
//     table can't be silently skipped by this leg.
//
// Mirrors the Rust reference leg (sdks/rust/oura-toolkit-auth/tests/conformance.rs).
package auth

import (
	"bytes"
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"sort"
	"sync/atomic"
	"testing"
	"time"
)

// conformanceFixture is the decoded shape of codegen/conformance/auth-cases.json.
type conformanceFixture struct {
	HostileTokenResponses []struct {
		Name string `json:"name"`
		// Body is kept raw: the server replays the fixture's JSON verbatim, so a
		// wrong-typed field (42, "soon") reaches the companion exactly as authored.
		Body    json.RawMessage `json:"body"`
		RawBody *string         `json:"raw_body"`
		// RawBodyBase64 carries bytes JSON can't hold (e.g. invalid UTF-8); the
		// decoded bytes are served verbatim.
		RawBodyBase64 *string `json:"raw_body_base64"`
	} `json:"hostile_token_responses"`
	HostileStoreFiles []struct {
		Name    string `json:"name"`
		File    string `json:"file"`
		Content string `json:"content"`
	} `json:"hostile_store_files"`
	RefreshSuccessCases struct {
		Prior struct {
			AccessToken  string `json:"access_token"`
			RefreshToken string `json:"refresh_token"`
			Scope        string `json:"scope"`
			TokenType    string `json:"token_type"`
		} `json:"prior"`
		Cases []struct {
			Name string `json:"name"`
			// Raw, replayed verbatim: an omitted vs null vs blank field must reach the
			// companion exactly as authored.
			Body     json.RawMessage `json:"body"`
			RawBody  *string         `json:"raw_body"`
			Expected struct {
				AccessToken  string `json:"access_token"`
				RefreshToken string `json:"refresh_token"`
				Scope        string `json:"scope"`
				TokenType    string `json:"token_type"`
				ExpiresIn    int64  `json:"expires_in"`
			} `json:"expected"`
		} `json:"cases"`
	} `json:"refresh_success_cases"`
	ValidRecords map[string]json.RawMessage `json:"valid_records"`
}

// loadConformanceFixture walks up from the package dir to the repo root (nearest
// ancestor holding the justfile + README — the same walk as the Rust leg) and decodes
// the shared fixture. Monorepo-only by design.
func loadConformanceFixture(t *testing.T) conformanceFixture {
	t.Helper()
	var f conformanceFixture
	if err := json.Unmarshal(readConformanceFixture(t), &f); err != nil {
		t.Fatalf("fixture is not valid JSON: %v", err)
	}
	return f
}

// readConformanceFixture returns the shared fixture's raw bytes.
func readConformanceFixture(t *testing.T) []byte {
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
	return data
}

// The fixture's top-level tables must be EXACTLY the four this leg iterates: a table
// renamed (as refresh_scope_cases → refresh_success_cases was) or added upstream would
// otherwise decode to a zero value / be ignored, silently skipping its cases here.
func TestConformanceFixtureTopLevelTablesAreExactlyTheKnownFour(t *testing.T) {
	var top map[string]json.RawMessage
	if err := json.Unmarshal(readConformanceFixture(t), &top); err != nil {
		t.Fatalf("fixture is not a JSON object: %v", err)
	}
	var got []string
	for k := range top {
		if k != "$comment" {
			got = append(got, k)
		}
	}
	sort.Strings(got)
	want := []string{"hostile_store_files", "hostile_token_responses", "refresh_success_cases", "valid_records"}
	if len(got) != len(want) {
		t.Fatalf("fixture top-level tables = %v, want exactly %v (update this leg's harnesses for any added/renamed table)", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("fixture top-level tables = %v, want exactly %v (update this leg's harnesses for any added/renamed table)", got, want)
		}
	}
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
	if n := len(fixture.HostileTokenResponses); n < 24 {
		t.Fatalf("fixture shrank? hostile_token_responses has %d cases, want >= 24", n)
	}

	for _, tc := range fixture.HostileTokenResponses {
		t.Run(tc.Name, func(t *testing.T) {
			var rawBytes []byte
			if tc.RawBodyBase64 != nil {
				decoded, err := base64.StdEncoding.DecodeString(*tc.RawBodyBase64)
				if err != nil {
					t.Fatalf("case %s: raw_body_base64 is not valid base64: %v", tc.Name, err)
				}
				rawBytes = decoded
			} else if tc.RawBody != nil {
				rawBytes = []byte(*tc.RawBody)
			} else if len(tc.Body) == 0 {
				t.Fatalf("case %s has none of body / raw_body / raw_body_base64", tc.Name)
			}
			var calls atomic.Int32
			srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				calls.Add(1)
				if rawBytes != nil {
					// raw_body / raw_body_base64 bytes are replayed VERBATIM (possibly
					// not JSON, so no content-type claim either).
					w.WriteHeader(http.StatusOK)
					_, _ = w.Write(rawBytes)
					return
				}
				w.Header().Set("Content-Type", "application/json")
				w.WriteHeader(http.StatusOK)
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
				t.Fatalf("case %s: a hostile 2xx must not succeed", tc.Name)
			}
			// Typed: the companion's invalid-response error — never a raw
			// json.Unmarshal error escaping untyped, and never a mis-filed sentinel
			// that would trigger re-login remediation for a server-side fault.
			var te *TokenEndpointError
			if !errors.As(err, &te) {
				t.Fatalf("case %s: want the typed *TokenEndpointError, got %T: %v", tc.Name, err, err)
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

// A SUCCESSFUL refresh starting from the fixture's stored `prior` record must persist
// EXACTLY `expected`: an omitted, null, empty, whitespace-only (incl. U+00A0), or
// non-string scope keeps the prior grant (persisting a blank would erase what the
// re-consent check reads, #116) and must not fail the refresh (that would burn the
// rotated refresh token); an omitted/null/EMPTY refresh_token or token_type keeps the
// prior value (persisting "" would make the next refresh 400); expires_in at the cap
// succeeds; an unknown field is never validated. expires_at must be the refresh time
// plus expected.expires_in.
func TestConformanceRefreshSuccessCasesPersistExpectedRecord(t *testing.T) {
	fixture := loadConformanceFixture(t)
	table := fixture.RefreshSuccessCases
	p := table.Prior
	if p.AccessToken == "" || p.RefreshToken == "" || p.Scope == "" || p.TokenType == "" {
		t.Fatalf("fixture refresh_success_cases.prior is incomplete: every field must be non-empty")
	}
	if n := len(table.Cases); n < 17 {
		t.Fatalf("fixture shrank? refresh_success_cases has %d cases, want >= 17", n)
	}

	for _, tc := range table.Cases {
		t.Run(tc.Name, func(t *testing.T) {
			var body []byte
			switch {
			case tc.RawBody != nil:
				body = []byte(*tc.RawBody)
			case len(tc.Body) != 0:
				body = tc.Body
			default:
				t.Fatalf("case %s has neither body nor raw_body", tc.Name)
			}
			if tc.Expected.ExpiresIn <= 0 {
				t.Fatalf("case %s: expected.expires_in missing from the fixture", tc.Name)
			}

			var calls atomic.Int32
			srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				calls.Add(1)
				w.Header().Set("Content-Type", "application/json")
				w.WriteHeader(http.StatusOK)
				_, _ = w.Write(body)
			}))
			defer srv.Close()

			store := NewStoreAt(t.TempDir())
			if err := store.SaveCredentials(sampleCredentials()); err != nil {
				t.Fatal(err)
			}
			// Already expired, so the refresh genuinely calls the endpoint.
			prior := &Tokens{
				AccessToken:  p.AccessToken,
				RefreshToken: p.RefreshToken,
				ExpiresAt:    0,
				Scope:        p.Scope,
				TokenType:    p.TokenType,
			}
			if err := store.SaveTokens(prior); err != nil {
				t.Fatal(err)
			}
			seed := *prior

			m := testManager(t, srv.URL, store, &seed)
			t0 := time.Now().Unix()
			if err := m.ForceRefresh(context.Background()); err != nil {
				t.Fatalf("case %s: a valid 2xx refresh must succeed: %v", tc.Name, err)
			}
			t1 := time.Now().Unix()
			if n := calls.Load(); n != 1 {
				t.Fatalf("case %s: want exactly 1 token-endpoint call, got %d", tc.Name, n)
			}

			persisted, err := store.LoadTokens()
			if err != nil {
				t.Fatal(err)
			}
			if persisted == nil {
				t.Fatalf("case %s: the refreshed record must be persisted", tc.Name)
			}
			want := tc.Expected
			if persisted.AccessToken != want.AccessToken {
				t.Fatalf("case %s: persisted access_token = %q, want %q", tc.Name, persisted.AccessToken, want.AccessToken)
			}
			if persisted.RefreshToken != want.RefreshToken {
				t.Fatalf("case %s: persisted refresh_token = %q, want %q (an omitted/null/empty one keeps the prior %q; a returned one is the rotation)",
					tc.Name, persisted.RefreshToken, want.RefreshToken, p.RefreshToken)
			}
			if persisted.Scope != want.Scope {
				t.Fatalf("case %s: persisted scope = %q, want %q (a blank/whitespace/non-string scope keeps the prior grant %q; a real one replaces it)",
					tc.Name, persisted.Scope, want.Scope, p.Scope)
			}
			if persisted.TokenType != want.TokenType {
				t.Fatalf("case %s: persisted token_type = %q, want %q (an omitted/null/empty one keeps the prior %q)",
					tc.Name, persisted.TokenType, want.TokenType, p.TokenType)
			}
			lo, hi := t0+want.ExpiresIn, t1+want.ExpiresIn
			if persisted.ExpiresAt < lo || persisted.ExpiresAt > hi {
				t.Fatalf("case %s: persisted expires_at = %d, want within [%d, %d] (refresh time + expires_in %d)",
					tc.Name, persisted.ExpiresAt, lo, hi, want.ExpiresIn)
			}
		})
	}
}
