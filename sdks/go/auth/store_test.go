package auth

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"
)

func sampleCredentials() *ClientCredentials {
	return &ClientCredentials{ClientID: "cid", ClientSecret: "SECRET-CS-789"}
}

func sampleTokens() *Tokens {
	return &Tokens{
		AccessToken:  "SECRET-AT-123",
		RefreshToken: "SECRET-RT-456",
		ExpiresAt:    4_102_444_800, // 2100-01-01
		Scope:        "daily personal",
		TokenType:    "Bearer",
	}
}

// The store is SHARED with the Rust CLI (oura auth login writes it, this package reads
// and rewrites it), so the JSON schema is pinned byte-for-byte to the Rust companion's
// serde output (store.rs: field names + serde_json::to_vec_pretty formatting). These
// fixtures are transcriptions of what oura-toolkit-auth persists for store.rs's
// sample_credentials()/sample_tokens().
const (
	rustCredentialsFixture = `{
  "client_id": "cid",
  "client_secret": "SECRET-CS-789"
}`
	rustTokensFixture = `{
  "access_token": "SECRET-AT-123",
  "refresh_token": "SECRET-RT-456",
  "expires_at": 4102444800,
  "scope": "daily personal",
  "token_type": "Bearer"
}`
)

func TestSchemaMatchesRustStoreFixture(t *testing.T) {
	// Read side: a record the Rust CLI wrote parses into the same values.
	var creds ClientCredentials
	if err := json.Unmarshal([]byte(rustCredentialsFixture), &creds); err != nil {
		t.Fatalf("credentials fixture did not parse: %v", err)
	}
	if creds != *sampleCredentials() {
		t.Fatalf("credentials fixture parsed to %#v — field names drifted from store.rs", creds)
	}
	var tokens Tokens
	if err := json.Unmarshal([]byte(rustTokensFixture), &tokens); err != nil {
		t.Fatalf("tokens fixture did not parse: %v", err)
	}
	if tokens != *sampleTokens() {
		t.Fatalf("tokens fixture parsed to %#v — field names drifted from store.rs", tokens)
	}

	// Write side: what this package persists is byte-identical to the Rust output, so a
	// Go-side rewrite (a refresh rotation) hands the CLI exactly the format it wrote.
	gotCreds, err := json.MarshalIndent(sampleCredentials(), "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	if string(gotCreds) != rustCredentialsFixture {
		t.Fatalf("credentials serialization drifted from the Rust store schema:\n%s", gotCreds)
	}
	gotTokens, err := json.MarshalIndent(sampleTokens(), "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	if string(gotTokens) != rustTokensFixture {
		t.Fatalf("tokens serialization drifted from the Rust store schema:\n%s", gotTokens)
	}
}

func TestOptionalTokenFieldsAreOmittedLikeSerde(t *testing.T) {
	// store.rs skips scope/token_type when None (skip_serializing_if); the Go omitempty
	// must match or a Go rewrite would hand the CLI `"scope": ""` where Rust expects
	// Option::None-style absence.
	got, err := json.MarshalIndent(&Tokens{
		AccessToken:  "a",
		RefreshToken: "r",
		ExpiresAt:    1,
	}, "", "  ")
	if err != nil {
		t.Fatal(err)
	}
	want := `{
  "access_token": "a",
  "refresh_token": "r",
  "expires_at": 1
}`
	if string(got) != want {
		t.Fatalf("empty optional fields must be omitted (serde skip_serializing_if parity), got:\n%s", got)
	}
}

func TestBothRecordsRoundtripAndAbsentIsNil(t *testing.T) {
	store := NewStoreAt(t.TempDir())

	if c, err := store.LoadCredentials(); err != nil || c != nil {
		t.Fatalf("absent credentials must be (nil, nil), got (%v, %v)", c, err)
	}
	if tok, err := store.LoadTokens(); err != nil || tok != nil {
		t.Fatalf("absent tokens must be (nil, nil), got (%v, %v)", tok, err)
	}

	if err := store.SaveCredentials(sampleCredentials()); err != nil {
		t.Fatal(err)
	}
	if err := store.SaveTokens(sampleTokens()); err != nil {
		t.Fatal(err)
	}

	creds, err := store.LoadCredentials()
	if err != nil {
		t.Fatal(err)
	}
	if *creds != *sampleCredentials() {
		t.Fatalf("credentials did not roundtrip: %#v", creds)
	}
	tokens, err := store.LoadTokens()
	if err != nil {
		t.Fatal(err)
	}
	if *tokens != *sampleTokens() {
		t.Fatalf("tokens did not roundtrip: %#v", tokens)
	}
}

func TestCorruptRecordErrorsInsteadOfNil(t *testing.T) {
	dir := t.TempDir()
	store := NewStoreAt(dir)
	if err := os.WriteFile(store.TokensPath(), []byte("{not json"), 0o600); err != nil {
		t.Fatal(err)
	}
	tok, err := store.LoadTokens()
	if err == nil || tok != nil {
		t.Fatalf("corrupt tokens.json must surface a parse error, got (%v, %v)", tok, err)
	}
	if !strings.Contains(err.Error(), "token store format error") {
		t.Fatalf("corrupt record must be reported as a format error, got: %v", err)
	}
}

// A record that is syntactically valid JSON but not a complete, correctly-typed record of
// its type — a partial object, the literal null, or a wrong-typed field — must load as
// (nil, *StoreFormatError), NEVER as a zero-valued struct with a nil error. If it loaded as
// a zero struct, LoadManager would build a manager whose IsAuthenticated() lies (reporting
// empty tokens as present) or whose refresh sends an empty client_id/secret. This mirrors
// the Rust companion's serde deserialize, which rejects all of these.
func TestLoadRejectsIncompleteOrMistypedRecords(t *testing.T) {
	cases := []struct {
		name    string
		tokens  bool // true → write tokens.json, false → credentials.json
		content string
	}{
		{"tokens: partial (only scope)", true, `{"scope":"daily"}`},
		{"tokens: literal null", true, `null`},
		{"tokens: expires_at as string", true, `{"access_token":"a","refresh_token":"r","expires_at":"soon"}`},
		{"tokens: expires_at as bool", true, `{"access_token":"a","refresh_token":"r","expires_at":true}`},
		{"tokens: missing refresh_token", true, `{"access_token":"a","expires_at":1}`},
		{"tokens: access_token as number", true, `{"access_token":42,"refresh_token":"r","expires_at":1}`},
		{"tokens: null required field", true, `{"access_token":null,"refresh_token":"r","expires_at":1}`},
		{"credentials: partial (only client_id)", false, `{"client_id":"cid"}`},
		{"credentials: literal null", false, `null`},
		{"credentials: client_secret wrong type", false, `{"client_id":"cid","client_secret":123}`},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			store := NewStoreAt(t.TempDir())
			path := store.CredentialsPath()
			if tc.tokens {
				path = store.TokensPath()
			}
			if err := os.WriteFile(path, []byte(tc.content), 0o600); err != nil {
				t.Fatal(err)
			}

			var loadErr error
			if tc.tokens {
				tok, err := store.LoadTokens()
				if tok != nil {
					t.Fatalf("an invalid tokens record must load as (nil, error), got %#v", tok)
				}
				loadErr = err
			} else {
				creds, err := store.LoadCredentials()
				if creds != nil {
					t.Fatalf("an invalid credentials record must load as (nil, error), got %#v", creds)
				}
				loadErr = err
			}
			var sfe *StoreFormatError
			if !errors.As(loadErr, &sfe) {
				t.Fatalf("invalid record must be a typed *StoreFormatError, got %T: %v", loadErr, loadErr)
			}
			if !strings.Contains(loadErr.Error(), "token store format error") {
				t.Fatalf("format error message drifted: %v", loadErr)
			}
		})
	}
}

func envLookup(pairs map[string]string) func(string) (string, bool) {
	return func(key string) (string, bool) {
		v, ok := pairs[key]
		return v, ok
	}
}

func TestConfigDirUnix(t *testing.T) {
	for _, goos := range []string{"linux", "darwin"} {
		dir, err := configDirFrom(goos, envLookup(map[string]string{
			"XDG_CONFIG_HOME": "/xdg", "HOME": "/home/u",
		}))
		if err != nil || dir != filepath.Join("/xdg", "oura-toolkit") {
			t.Fatalf("%s: XDG_CONFIG_HOME must win: got (%q, %v)", goos, dir, err)
		}

		dir, err = configDirFrom(goos, envLookup(map[string]string{"HOME": "/home/u"}))
		if err != nil || dir != filepath.Join("/home/u", ".config", "oura-toolkit") {
			t.Fatalf("%s: HOME fallback must be ~/.config/oura-toolkit: got (%q, %v)", goos, dir, err)
		}
	}
}

func TestConfigDirUnixIgnoresEmptyAndRelativeValues(t *testing.T) {
	for _, bad := range []string{"", "relative/config"} {
		dir, err := configDirFrom("linux", envLookup(map[string]string{
			"XDG_CONFIG_HOME": bad, "HOME": "/home/u",
		}))
		if err != nil || dir != filepath.Join("/home/u", ".config", "oura-toolkit") {
			t.Fatalf("XDG_CONFIG_HOME=%q must be ignored (fall back to HOME): got (%q, %v)", bad, dir, err)
		}
	}
	for _, bad := range []string{"", "relative/home"} {
		_, err := configDirFrom("linux", envLookup(map[string]string{"HOME": bad}))
		if !errors.Is(err, ErrNoConfigDir) {
			t.Fatalf("HOME=%q must not resolve (cwd-dependent secret placement): got %v", bad, err)
		}
	}
	if _, err := configDirFrom("linux", envLookup(nil)); !errors.Is(err, ErrNoConfigDir) {
		t.Fatalf("no env must be ErrNoConfigDir, got %v", err)
	}
}

func TestConfigDirWindowsUsesLocalNotRoaming(t *testing.T) {
	dir, err := configDirFrom("windows", envLookup(map[string]string{
		"LOCALAPPDATA": `C:\Users\u\AppData\Local`,
		"APPDATA":      `C:\Users\u\AppData\Roaming`, // roams to file servers — must never be used
	}))
	if err != nil {
		t.Fatal(err)
	}
	if dir != filepath.Join(`C:\Users\u\AppData\Local`, "oura-toolkit") {
		t.Fatalf("must use machine-local %%LOCALAPPDATA%%, never the roaming profile: got %q", dir)
	}
}

func TestConfigDirWindowsIgnoresEmptyAndRelativeValues(t *testing.T) {
	for _, bad := range []string{"", `relative\path`} {
		_, err := configDirFrom("windows", envLookup(map[string]string{"LOCALAPPDATA": bad}))
		if !errors.Is(err, ErrNoConfigDir) {
			t.Fatalf("LOCALAPPDATA=%q must not resolve: got %v", bad, err)
		}
	}
	// Windows users must not be told about Unix env vars.
	_, err := configDirFrom("windows", envLookup(nil))
	if !errors.Is(err, ErrNoConfigDir) || !strings.Contains(err.Error(), "%LOCALAPPDATA%") {
		t.Fatalf("windows error must name %%LOCALAPPDATA%%: got %v", err)
	}
	if _, err := configDirFrom("linux", envLookup(nil)); err != nil &&
		strings.Contains(err.Error(), "LOCALAPPDATA") {
		t.Fatalf("unix error must not name windows env vars: got %v", err)
	}
}

func TestExpiryUsesSkew(t *testing.T) {
	tok := sampleTokens()
	tok.ExpiresAt = time.Now().Unix() + 30
	if tok.expired(0) {
		t.Fatal("30s out with no skew must NOT be expired")
	}
	if !tok.expired(60 * time.Second) {
		t.Fatal("30s out with 60s skew MUST be treated as expired (proactive refresh window)")
	}
}

func TestStoreLockIsExclusiveAndReleasedOnUnlock(t *testing.T) {
	store := NewStoreAt(t.TempDir())

	held, err := store.LockExclusive()
	if err != nil {
		t.Fatal(err)
	}
	// A second handle (fresh fd on the same .lock file) must not acquire while held.
	contended, err := store.TryLockExclusive()
	if err != nil {
		t.Fatal(err)
	}
	if contended != nil {
		t.Fatal("TryLockExclusive succeeded while the lock was held — the lock is not exclusive")
	}
	if err := held.Unlock(); err != nil {
		t.Fatal(err)
	}
	free, err := store.TryLockExclusive()
	if err != nil {
		t.Fatal(err)
	}
	if free == nil {
		t.Fatal("lock must be free after Unlock")
	}
	free.Unlock()
}
