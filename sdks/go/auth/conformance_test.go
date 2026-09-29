// Cross-language auth-companion conformance (#58) — the GO leg.
//
// Iterates codegen/conformance/auth-cases.json (the single source for the hostile
// token-endpoint responses, hostile store files, successful-refresh expectations, and
// canonical store records that every companion suite must exercise; cases are added
// THERE, never here — its `$comment` is the contract):
//
//   - hostile-but-2xx token responses (EXACTLY ONE of `body`, verbatim `raw_body`, or
//     decoded `raw_body_base64` bytes — one shared helper for all three token-endpoint
//     tables) → typed *TokenEndpointError, store UNTOUCHED (the rotated refresh token
//     is never burned by persisting a blank/expired Bearer; a non-string
//     refresh_token/token_type fails typed; a lone surrogate in any of the four fields
//     read — access_token, refresh_token, token_type, scope — is never persisted; a body with invalid UTF-8 ANYWHERE fails; an expires_in outside
//     1..=2147483647, fractional, or a numeric string fails; trailing data after the
//     one top-level value fails); a case's `must_not_echo` string appears nowhere in
//     the error's text or any error it chains;
//   - implementation-defined 2xx token responses (BOM, duplicate keys, deep nesting,
//     3600.0, an upper-case key) → EITHER success persisting access_token
//     "at-refreshed", refresh_token "rt-refreshed" or the prior one (never empty) and
//     expires_at = refresh time + 3600, OR typed *TokenEndpointError with the store
//     byte-identical;
//   - hostile store files → typed *StoreFormatError, never a zero-valued record that
//     would make IsAuthenticated lie, and never a panic; a case's `must_not_echo`
//     string appears nowhere in the error chain;
//   - implementation-defined store files (deep nesting in an unknown field) → EITHER
//     exactly the fixture's `expected` record OR the typed *StoreFormatError;
//   - canonical valid records → load with exactly the fixture's field values and
//     round-trip through this package's own persist path (the cross-language store
//     compatibility check — field names are the shared wire format, #54);
//   - successful refreshes from the fixture's `prior` record → the refresh SUCCEEDS and
//     persists EXACTLY `expected` (access_token, refresh_token, scope, token_type, and
//     expires_at = refresh time + expires_in): an omitted/null/empty/whitespace (incl.
//     U+00A0) or non-string scope keeps the prior grant; an omitted/null/EMPTY
//     refresh_token or token_type keeps the prior value; expires_in at 1 and at the cap
//     succeeds; an unknown field is never validated; the request SENT the prior
//     refresh_token;
//   - the fixture's top-level tables are exactly the six above, so a renamed/added
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
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

// conformanceBody is how a token-endpoint case gives its 2xx body: EXACTLY ONE of
// `body` (JSON, kept raw so the server replays it verbatim — a wrong-typed field (42,
// "soon") or an omitted-vs-null-vs-blank one reaches the companion exactly as authored),
// `raw_body` (sent verbatim), or `raw_body_base64` (decoded bytes JSON can't hold, e.g.
// invalid UTF-8 or a BOM). Shared by all three token-endpoint harnesses.
type conformanceBody struct {
	Body          json.RawMessage `json:"body"`
	RawBody       *string         `json:"raw_body"`
	RawBodyBase64 *string         `json:"raw_body_base64"`
}

// payload resolves the case's body, failing the test (naming the case) unless exactly
// one of body / raw_body / raw_body_base64 is present. verbatim reports a raw_body /
// raw_body_base64 payload (possibly not JSON, so served with no content-type claim).
func (b conformanceBody) payload(t *testing.T, name string) (payload []byte, verbatim bool) {
	t.Helper()
	present := 0
	if len(b.Body) != 0 {
		present++
	}
	if b.RawBody != nil {
		present++
	}
	if b.RawBodyBase64 != nil {
		present++
	}
	if present != 1 {
		t.Fatalf("case %s: must give EXACTLY ONE of body / raw_body / raw_body_base64, found %d", name, present)
	}
	switch {
	case b.RawBodyBase64 != nil:
		decoded, err := base64.StdEncoding.DecodeString(*b.RawBodyBase64)
		if err != nil {
			t.Fatalf("case %s: raw_body_base64 is not valid base64: %v", name, err)
		}
		return decoded, true
	case b.RawBody != nil:
		return []byte(*b.RawBody), true
	default:
		return b.Body, false
	}
}

// serveConformanceBody writes a 2xx carrying the resolved case payload.
func serveConformanceBody(w http.ResponseWriter, payload []byte, verbatim bool) {
	if !verbatim {
		w.Header().Set("Content-Type", "application/json")
	}
	w.WriteHeader(http.StatusOK)
	_, _ = w.Write(payload)
}

// conformanceFixture is the decoded shape of codegen/conformance/auth-cases.json.
type conformanceFixture struct {
	HostileTokenResponses []struct {
		Name string `json:"name"`
		conformanceBody
		MustNotEcho *string `json:"must_not_echo"`
	} `json:"hostile_token_responses"`
	ImplementationDefinedTokenResponses []struct {
		Name string `json:"name"`
		conformanceBody
	} `json:"implementation_defined_token_responses"`
	HostileStoreFiles []struct {
		Name        string  `json:"name"`
		File        string  `json:"file"`
		Content     string  `json:"content"`
		MustNotEcho *string `json:"must_not_echo"`
	} `json:"hostile_store_files"`
	ImplementationDefinedStoreFiles []struct {
		Name     string          `json:"name"`
		File     string          `json:"file"`
		Content  string          `json:"content"`
		Expected json.RawMessage `json:"expected"`
	} `json:"implementation_defined_store_files"`
	RefreshSuccessCases struct {
		Prior struct {
			AccessToken  string `json:"access_token"`
			RefreshToken string `json:"refresh_token"`
			Scope        string `json:"scope"`
			TokenType    string `json:"token_type"`
		} `json:"prior"`
		Cases []struct {
			Name string `json:"name"`
			conformanceBody
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

// The fixture's top-level tables must be EXACTLY the six this leg iterates: a table
// renamed (as refresh_scope_cases → refresh_success_cases was) or added upstream would
// otherwise decode to a zero value / be ignored, silently skipping its cases here.
func TestConformanceFixtureTopLevelTablesAreExactlyTheKnownSix(t *testing.T) {
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
	want := []string{"hostile_store_files", "hostile_token_responses", "implementation_defined_store_files", "implementation_defined_token_responses", "refresh_success_cases", "valid_records"}
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

// assertNoEcho enforces a case's `must_not_echo`: the secret must appear NOWHERE in the
// error's text, nor in the text (or %#v rendering) of any error it chains — walked
// recursively through errors.Unwrap AND joined errors (Unwrap() []error), since a parser
// message quoting the body/file would leak token material through a wrapped cause even
// when the outer message is fixed. A nil mustNotEcho is a no-op; an empty one is a
// fixture bug (it would match everything).
func assertNoEcho(t *testing.T, name string, err error, mustNotEcho *string) {
	t.Helper()
	if mustNotEcho == nil {
		return
	}
	secret := *mustNotEcho
	if secret == "" {
		t.Fatalf("case %s: must_not_echo is empty in the fixture", name)
	}
	var visited int
	var walk func(e error, depth int)
	walk = func(e error, depth int) {
		if e == nil {
			return
		}
		if depth > 64 {
			t.Fatalf("case %s: error chain deeper than 64 — cyclic Unwrap?", name)
		}
		visited++
		for _, rendered := range []string{e.Error(), fmt.Sprintf("%+v", e), fmt.Sprintf("%#v", e)} {
			if strings.Contains(rendered, secret) {
				t.Fatalf("case %s: the error chain echoes the must_not_echo secret %q (at depth %d, %T): %s", name, secret, depth, e, rendered)
			}
		}
		switch u := e.(type) {
		case interface{ Unwrap() []error }:
			for _, inner := range u.Unwrap() {
				walk(inner, depth+1)
			}
		case interface{ Unwrap() error }:
			walk(u.Unwrap(), depth+1)
		}
	}
	walk(err, 0)
	if visited == 0 {
		t.Fatalf("case %s: must_not_echo needs an error to inspect", name)
	}
}

// Every hostile-but-2xx token response must fail the refresh with the typed
// *TokenEndpointError and leave the persisted record byte-identical — the rotated
// refresh token is never burned by a blank/expired Bearer. (A panic escaping to the
// caller fails the t.Run outright, so reaching the assertions proves "never a panic".)
func TestConformanceHostile2xxTokenResponsesFailTypedAndLeaveStoreUntouched(t *testing.T) {
	fixture := loadConformanceFixture(t)
	if n := len(fixture.HostileTokenResponses); n < 27 {
		t.Fatalf("fixture shrank? hostile_token_responses has %d cases, want >= 27", n)
	}

	for _, tc := range fixture.HostileTokenResponses {
		t.Run(tc.Name, func(t *testing.T) {
			payload, verbatim := tc.payload(t, tc.Name)
			var calls atomic.Int32
			srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				calls.Add(1)
				serveConformanceBody(w, payload, verbatim)
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
			assertNoEcho(t, tc.Name, err, tc.MustNotEcho)

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

// Every implementation-defined 2xx token response (a leading UTF-8 BOM, duplicate keys,
// deep nesting inside an unknown field, an integral float expires_in, a key in another
// case) must EITHER succeed and persist access_token = "at-refreshed", a refresh_token
// that is "rt-refreshed" or the prior stored one (never empty), and expires_at = refresh
// time + 3600, OR fail with the typed *TokenEndpointError (2xx status preserved) and leave
// both store files byte-identical. Anything else — a panic, an untyped error, a wrong
// persisted token or expiry, a half-written store — fails, naming the case. The store/manager are seeded
// exactly as in the hostile harness.
func TestConformanceImplementationDefinedTokenResponsesSucceedOrFailTypedUntouched(t *testing.T) {
	fixture := loadConformanceFixture(t)
	if n := len(fixture.ImplementationDefinedTokenResponses); n < 5 {
		t.Fatalf("fixture shrank? implementation_defined_token_responses has %d cases, want >= 5", n)
	}

	for _, tc := range fixture.ImplementationDefinedTokenResponses {
		t.Run(tc.Name, func(t *testing.T) {
			payload, verbatim := tc.payload(t, tc.Name)
			var calls atomic.Int32
			srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				calls.Add(1)
				serveConformanceBody(w, payload, verbatim)
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
			t0 := time.Now().Unix()
			refreshErr := m.ForceRefresh(context.Background())
			t1 := time.Now().Unix()
			if n := calls.Load(); n != 1 {
				t.Fatalf("case %s: want exactly 1 token-endpoint call (a 2xx never takes the reload-retry arm), got %d", tc.Name, n)
			}

			if refreshErr == nil {
				// Accepted: the refreshed record must be persisted with the response's
				// access token — never a stale or mangled one.
				persisted, err := store.LoadTokens()
				if err != nil {
					t.Fatalf("case %s: accepted, but the persisted tokens.json does not load: %v", tc.Name, err)
				}
				if persisted == nil || persisted.AccessToken != "at-refreshed" {
					t.Fatalf("case %s: accepted, so the persisted access_token must be %q, got %+v", tc.Name, "at-refreshed", persisted)
				}
				// The rotation, or (the server's value dropped/blank) the prior stored one —
				// never "" (the next refresh would 400) and never anything else.
				if persisted.RefreshToken != "rt-refreshed" && persisted.RefreshToken != "r1" {
					t.Fatalf("case %s: accepted, so the persisted refresh_token must be %q or the prior %q, got %q", tc.Name, "rt-refreshed", "r1", persisted.RefreshToken)
				}
				if lo, hi := t0+3600, t1+3600; persisted.ExpiresAt < lo || persisted.ExpiresAt > hi {
					t.Fatalf("case %s: accepted, so the persisted expires_at must be within [%d, %d] (refresh time + 3600), got %d", tc.Name, lo, hi, persisted.ExpiresAt)
				}
				t.Logf("case %s: accepted (persisted at-refreshed, refresh_token %q)", tc.Name, persisted.RefreshToken)
				return
			}

			// Rejected: typed, 2xx status preserved, store untouched.
			var te *TokenEndpointError
			if !errors.As(refreshErr, &te) {
				t.Fatalf("case %s: a rejection must be the typed *TokenEndpointError, got %T: %v", tc.Name, refreshErr, refreshErr)
			}
			if te.Status < 200 || te.Status > 299 {
				t.Fatalf("case %s: a rejected 2xx must preserve its 2xx status, got %d", tc.Name, te.Status)
			}
			tokensAfter, err := os.ReadFile(store.TokensPath())
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Equal(tokensBefore, tokensAfter) {
				t.Fatalf("case %s: rejected, so tokens.json must be byte-identical:\nbefore: %s\nafter:  %s", tc.Name, tokensBefore, tokensAfter)
			}
			credsAfter, err := os.ReadFile(store.CredentialsPath())
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Equal(credsBefore, credsAfter) {
				t.Fatalf("case %s: rejected, so credentials.json must be byte-identical", tc.Name)
			}
			t.Logf("case %s: rejected typed (%v)", tc.Name, te)
		})
	}
}

// Every hostile store file must load as the typed *StoreFormatError — never a
// zero-valued record (which would make IsAuthenticated lie), never a panic.
func TestConformanceHostileStoreFilesFailTyped(t *testing.T) {
	fixture := loadConformanceFixture(t)
	if n := len(fixture.HostileStoreFiles); n < 14 {
		t.Fatalf("fixture shrank? hostile_store_files has %d cases, want >= 14", n)
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
			assertNoEcho(t, tc.Name, loadErr, tc.MustNotEcho)
		})
	}
}

// Every implementation-defined store file (content a parser may accept or reject, e.g.
// nesting past its depth limit inside an unknown field) must EITHER load as exactly the
// fixture's `expected` record OR fail with the typed *StoreFormatError (and no record) —
// never an untyped error, a panic, or a record differing from `expected`. "Exactly" is
// checked on the wire shape: the loaded record re-marshalled through this package's own
// JSON tags must equal `expected` field-for-field.
func TestConformanceImplementationDefinedStoreFilesLoadExactlyOrFailTyped(t *testing.T) {
	fixture := loadConformanceFixture(t)
	if n := len(fixture.ImplementationDefinedStoreFiles); n < 1 {
		t.Fatalf("fixture shrank? implementation_defined_store_files has %d cases, want >= 1", n)
	}

	for _, tc := range fixture.ImplementationDefinedStoreFiles {
		t.Run(tc.Name, func(t *testing.T) {
			want := decodeJSONObject(t, tc.Name, "expected", tc.Expected)
			if len(want) == 0 {
				t.Fatalf("case %s: fixture `expected` is missing or empty", tc.Name)
			}
			store := NewStoreAt(t.TempDir())
			if err := os.WriteFile(filepath.Join(store.Dir(), tc.File), []byte(tc.Content), 0o600); err != nil {
				t.Fatal(err)
			}

			var record any
			var loadErr error
			switch tc.File {
			case "tokens.json":
				tokens, err := store.LoadTokens()
				if tokens != nil {
					record = tokens
				}
				loadErr = err
			case "credentials.json":
				creds, err := store.LoadCredentials()
				if creds != nil {
					record = creds
				}
				loadErr = err
			default:
				t.Fatalf("fixture names an unknown store file %q", tc.File)
			}

			if loadErr != nil {
				var sfe *StoreFormatError
				if !errors.As(loadErr, &sfe) {
					t.Fatalf("case %s: a rejection must be the typed *StoreFormatError, got %T: %v", tc.Name, loadErr, loadErr)
				}
				if record != nil {
					t.Fatalf("case %s: a rejection must not also yield a record", tc.Name)
				}
				t.Logf("case %s: rejected typed (%v)", tc.Name, loadErr)
				return
			}
			if record == nil {
				t.Fatalf("case %s: loaded neither a record nor an error (an existing file read as absent)", tc.Name)
			}
			encoded, err := json.Marshal(record)
			if err != nil {
				t.Fatal(err)
			}
			got := decodeJSONObject(t, tc.Name, "loaded record", encoded)
			if !reflect.DeepEqual(got, want) {
				t.Fatalf("case %s: accepted, so the loaded record must be exactly `expected`:\ngot:  %s\nwant: %s", tc.Name, encoded, tc.Expected)
			}
			t.Logf("case %s: accepted (exactly `expected`)", tc.Name)
		})
	}
}

// decodeJSONObject decodes a JSON object with numbers kept as json.Number (so an int64
// expires_at compares exactly, never via float64 rounding).
func decodeJSONObject(t *testing.T, name, what string, data []byte) map[string]any {
	t.Helper()
	dec := json.NewDecoder(bytes.NewReader(data))
	dec.UseNumber()
	var m map[string]any
	if err := dec.Decode(&m); err != nil {
		t.Fatalf("case %s: %s is not a JSON object: %v", name, what, err)
	}
	return m
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
	if n := len(table.Cases); n < 18 {
		t.Fatalf("fixture shrank? refresh_success_cases has %d cases, want >= 18", n)
	}

	for _, tc := range table.Cases {
		t.Run(tc.Name, func(t *testing.T) {
			payload, verbatim := tc.payload(t, tc.Name)
			if tc.Expected.ExpiresIn <= 0 {
				t.Fatalf("case %s: expected.expires_in missing from the fixture", tc.Name)
			}

			var calls atomic.Int32
			var sentMu sync.Mutex
			var sentGrantType, sentRefreshToken string
			srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				calls.Add(1)
				if err := r.ParseForm(); err == nil {
					sentMu.Lock()
					sentGrantType = r.PostForm.Get("grant_type")
					sentRefreshToken = r.PostForm.Get("refresh_token")
					sentMu.Unlock()
				}
				serveConformanceBody(w, payload, verbatim)
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
			// The refresh must present the STORED prior refresh token — a success harness
			// that never checked what was sent would pass a companion refreshing with the
			// wrong (e.g. burned or blank) token.
			sentMu.Lock()
			gotGrant, gotRT := sentGrantType, sentRefreshToken
			sentMu.Unlock()
			if gotGrant != "refresh_token" {
				t.Fatalf("case %s: sent grant_type = %q, want refresh_token", tc.Name, gotGrant)
			}
			if gotRT != p.RefreshToken {
				t.Fatalf("case %s: sent refresh_token = %q, want the stored prior %q", tc.Name, gotRT, p.RefreshToken)
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
