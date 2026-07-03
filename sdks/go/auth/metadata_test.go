package auth

import (
	"encoding/json"
	"os"
	"path/filepath"
	"slices"
	"testing"
)

// repoSpecPath walks up from the package dir to the repo root's spec/openapi.json (the
// vendored source of truth).
func repoSpecPath(t *testing.T) string {
	t.Helper()
	dir, err := os.Getwd()
	if err != nil {
		t.Fatal(err)
	}
	for {
		candidate := filepath.Join(dir, "spec", "openapi.json")
		if _, err := os.Stat(candidate); err == nil {
			return candidate
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			t.Fatal("spec/openapi.json not found above the auth package — vendored spec missing?")
		}
		dir = parent
	}
}

// The sync tripwire behind metadata.go's constants: Go has no build-script spec read, so
// any spec refresh that moves an OAuth endpoint or changes the scope set must fail here
// until the constants follow (CLAUDE.md: never hardcode without the spec pinning it).
func TestMetadataMatchesSpec(t *testing.T) {
	raw, err := os.ReadFile(repoSpecPath(t))
	if err != nil {
		t.Fatal(err)
	}
	var spec struct {
		Components struct {
			SecuritySchemes struct {
				OAuth2 struct {
					Flows struct {
						AuthorizationCode struct {
							AuthorizationURL string            `json:"authorizationUrl"`
							TokenURL         string            `json:"tokenUrl"`
							Scopes           map[string]string `json:"scopes"`
						} `json:"authorizationCode"`
					} `json:"flows"`
				} `json:"OAuth2"`
			} `json:"securitySchemes"`
		} `json:"components"`
	}
	if err := json.Unmarshal(raw, &spec); err != nil {
		t.Fatal(err)
	}
	flow := spec.Components.SecuritySchemes.OAuth2.Flows.AuthorizationCode

	if flow.AuthorizationURL == "" || flow.TokenURL == "" || len(flow.Scopes) == 0 {
		t.Fatal("spec's OAuth2 authorizationCode flow not found where expected — sync test is blind")
	}
	if AuthorizeURL != flow.AuthorizationURL {
		t.Fatalf("AuthorizeURL %q drifted from the spec's authorizationUrl %q", AuthorizeURL, flow.AuthorizationURL)
	}
	if TokenURL != flow.TokenURL {
		t.Fatalf("TokenURL %q drifted from the spec's tokenUrl %q", TokenURL, flow.TokenURL)
	}

	specScopes := make([]string, 0, len(flow.Scopes))
	for s := range flow.Scopes {
		specScopes = append(specScopes, s)
	}
	slices.Sort(specScopes)
	ours := AllScopes()
	slices.Sort(ours)
	if !slices.Equal(ours, specScopes) {
		t.Fatalf("AllScopes() %v drifted from the spec's OAuth2 scopes %v", ours, specScopes)
	}
}

func TestDefaultScopesAreAllValidAndExcludeEmail(t *testing.T) {
	scopes := DefaultScopes()
	if len(scopes) != 7 {
		t.Fatalf("default scopes must be the 7 non-email scopes, got %d: %v", len(scopes), scopes)
	}
	if slices.Contains(scopes, "email") {
		t.Fatalf("default scopes must omit email (CLAUDE.md default-consent policy): %v", scopes)
	}
	all := AllScopes()
	for _, s := range scopes {
		if !slices.Contains(all, s) {
			t.Fatalf("default scope %q is not spec-advertised", s)
		}
	}
}

func TestScopeAccessorsReturnCopies(t *testing.T) {
	AllScopes()[0] = "mutated"
	DefaultScopes()[0] = "mutated"
	if AllScopes()[0] == "mutated" || DefaultScopes()[0] == "mutated" {
		t.Fatal("scope accessors must return defensive copies — a caller mutated the package state")
	}
}
