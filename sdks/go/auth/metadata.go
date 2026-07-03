package auth

import "slices"

// OAuth2 metadata pinned to the vendored spec (spec/openapi.json,
// components.securitySchemes.OAuth2.flows.authorizationCode). Go has no build-script
// step, so the values live here as constants and TestMetadataMatchesSpec is the sync
// tripwire: any spec refresh that moves an endpoint or renames a scope fails CI until
// these constants follow.

// AuthorizeURL is the spec's authorizationUrl (authorization-code consent page).
const AuthorizeURL = "https://cloud.ouraring.com/oauth/authorize"

// TokenURL is the spec's tokenUrl (code exchange + refresh; confidential client).
const TokenURL = "https://api.ouraring.com/oauth/token"

// allScopes is every scope the spec's OAuth2 flow advertises (8).
var allScopes = []string{
	"email",
	"personal",
	"daily",
	"heartrate",
	"workout",
	"tag",
	"session",
	"spo2Daily",
}

// defaultScopeNames is the toolkit's *policy* — everything except "email" — not spec
// metadata; the spec-advertised set is AllScopes.
var defaultScopeNames = []string{
	"personal",
	"daily",
	"heartrate",
	"workout",
	"tag",
	"session",
	"spo2Daily",
}

// AllScopes returns every scope advertised by the vendored spec's OAuth2 flow.
func AllScopes() []string {
	return append([]string(nil), allScopes...)
}

// DefaultScopes returns the scopes the toolkit requests by default: everything except
// "email".
//
// Each entry is validated against AllScopes and FAILS LOUD (panics, naming the missing
// scope) rather than silently narrowing the consent request if a spec refresh ever
// renames one. The unit tests catch this in CI on any spec refresh before it can panic
// at runtime.
func DefaultScopes() []string {
	for _, s := range defaultScopeNames {
		if !slices.Contains(allScopes, s) {
			panic("default scope " + s + " is not advertised by the vendored spec — " +
				"update defaultScopeNames to match the spec's OAuth2 scopes")
		}
	}
	return append([]string(nil), defaultScopeNames...)
}
