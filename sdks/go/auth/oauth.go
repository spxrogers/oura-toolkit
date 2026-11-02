package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"sort"
	"strings"
	"time"
	"unicode/utf8"
)

// maxExpiresIn is the largest expires_in (seconds, ~68 years) a token response may carry:
// 2147483647 (i32::MAX), the shared conformance cap. It keeps `now + expires_in` exact
// (far inside int64, so the ExpiresAt sum below can never overflow or wrap negative) and
// the resulting expires_at readable by EVERY companion's store (incl. those whose numeric
// type is a double or a 32-bit-bounded duration). Anything larger is a hostile/broken
// response, rejected typed (hostile_token_responses/expires_in_above_cap, _i64_max).
const maxExpiresIn = 2147483647

// tokenResponse is the raw token-endpoint response (Oura returns a rotated
// refresh_token on every call).
type tokenResponse struct {
	AccessToken  string `json:"access_token"`
	RefreshToken string `json:"refresh_token"`
	ExpiresIn    int64  `json:"expires_in"`
	TokenType    string `json:"token_type"`
	// Scope is decoded leniently: it is informational only, so a non-string value (a
	// number, object, array…) must NOT fail the refresh — that would burn the rotated
	// refresh token the server just issued. See grantedScope.
	Scope json.RawMessage `json:"scope"`
}

// hasInvalidUnicode reports whether any decoded string field (including a string-typed
// scope) was NOT valid Unicode on the wire. encoding/json does not reject a lone
// surrogate escape such as "\ud800" (or raw invalid UTF-8): it silently substitutes
// U+FFFD (utf8.RuneError). Checking the DECODED value for U+FFFD catches every such
// form with one rule, and costs nothing legitimate: no real token or OAuth scope (RFC
// 6749 §3.3 limits scope-tokens to printable ASCII) contains U+FFFD. A non-string scope
// is not inspected here — it stays lenient ("keep the prior grant", see grantedScope).
func (tr *tokenResponse) hasInvalidUnicode() bool {
	fields := []string{tr.AccessToken, tr.RefreshToken, tr.TokenType}
	var scope string
	if len(tr.Scope) != 0 && json.Unmarshal(tr.Scope, &scope) == nil {
		fields = append(fields, scope)
	}
	for _, f := range fields {
		if strings.ContainsRune(f, utf8.RuneError) {
			return true
		}
	}
	return false
}

// grantedScope returns the scope string the server granted, or "" when the field is
// omitted, null, not a JSON string, or blank after strings.TrimSpace (which also strips
// Unicode spaces such as U+00A0). "" means "unchanged: keep the prior grant".
func (tr *tokenResponse) grantedScope() string {
	var s string
	if len(tr.Scope) == 0 || json.Unmarshal(tr.Scope, &s) != nil {
		return ""
	}
	if strings.TrimSpace(s) == "" {
		return ""
	}
	return s
}

// maxErrorBodyChars caps a non-2xx token-endpoint body carried in a TokenEndpointError,
// in characters (runes): enough to diagnose, never an unbounded blob in logs/UI.
const maxErrorBodyChars = 1024

// redactedMarker replaces every occurrence of a submitted secret in an error body.
const redactedMarker = "[REDACTED]"

// redactSecrets replaces EVERY occurrence of each non-empty secret in body with
// redactedMarker. Longer secrets go first, so a secret that contains another is never
// left half-replaced. An empty secret is skipped (strings.ReplaceAll with an empty old
// string would splice the marker between every rune).
func redactSecrets(body string, secrets ...string) string {
	sorted := make([]string, 0, len(secrets))
	for _, s := range secrets {
		if s != "" {
			sorted = append(sorted, s)
		}
	}
	sort.Slice(sorted, func(i, j int) bool { return len(sorted[i]) > len(sorted[j]) })
	for _, s := range sorted {
		body = strings.ReplaceAll(body, s, redactedMarker)
	}
	return body
}

// capErrorBody bounds body to maxErrorBodyChars characters, cutting on a rune boundary
// and appending "…" when it cut. (A byte that isn't valid UTF-8 counts as one character,
// as `range` decodes it.)
func capErrorBody(body string) string {
	n := 0
	for i := range body {
		if n == maxErrorBodyChars {
			return body[:i] + "…"
		}
		n++
	}
	return body
}

// refreshTokens refreshes at the token endpoint using the stored refresh token.
// Oura is a CONFIDENTIAL client: the call carries client_id AND client_secret (no PKCE,
// no public-client path). The response carries a ROTATED refresh token which the caller
// MUST persist (Oura invalidates the previous one).
func refreshTokens(
	ctx context.Context,
	hc *http.Client,
	tokenURL string,
	creds *ClientCredentials,
	current *Tokens,
) (*Tokens, error) {
	form := url.Values{
		"grant_type":    {"refresh_token"},
		"refresh_token": {current.RefreshToken},
		"client_id":     {creds.ClientID},
		"client_secret": {creds.ClientSecret},
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, tokenURL, strings.NewReader(form.Encode()))
	if err != nil {
		return nil, fmt.Errorf("token endpoint request error: %w", err)
	}
	req.Header.Set("Content-Type", "application/x-www-form-urlencoded")

	resp, err := hc.Do(req)
	if err != nil {
		return nil, fmt.Errorf("token endpoint http error: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode > 299 {
		body, _ := io.ReadAll(io.LimitReader(resp.Body, 64<<10))
		// The body is kept for diagnosis, but a server may echo what we sent: redact every
		// secret this request submitted BEFORE capping (so a cut can never leave a partial
		// secret behind), then bound it (shared conformance rejected_token_responses).
		diag := redactSecrets(string(body), current.RefreshToken, creds.ClientSecret)
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: capErrorBody(diag)}
	}

	// Bound the success-path read too (a hostile 2xx could stream unboundedly).
	body, err := io.ReadAll(io.LimitReader(resp.Body, 64<<10))
	if err != nil {
		return nil, fmt.Errorf("token endpoint response error: %w", err)
	}

	// A hostile or broken 2xx body must fail as the typed *TokenEndpointError, never a raw
	// decode error detonating downstream and never a half-populated token persisted to the
	// store (an empty access_token or a zero expiry would only resurface as a baffling 400
	// on the NEXT refresh, long after the cause — fail loud here, leaving the store
	// untouched). The Body is a FIXED, secret-free description: the raw response is NOT
	// echoed, since a partial 2xx payload may carry token material.
	//
	// The body must be valid UTF-8 ANYWHERE, unknown fields included (RFC 8259 §8.1: it
	// isn't JSON text otherwise). encoding/json would silently substitute U+FFFD for the
	// bad bytes, and in an unknown field nothing downstream would ever notice
	// (hostile_token_responses/body_invalid_utf8_*).
	if !utf8.Valid(body) {
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: "token-endpoint 2xx response was not valid UTF-8"}
	}
	// expires_in decodes into an int64, so a fractional (3600.5), numeric-string ("3600")
	// or beyond-any-machine-integer (1e400) value already fails this Unmarshal.
	var tr tokenResponse
	if err := json.Unmarshal(body, &tr); err != nil {
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: "token-endpoint 2xx response was not valid JSON"}
	}
	// A malformed string (lone surrogate / invalid UTF-8) in any of the four fields read
	// makes the whole response malformed: persisting a U+FFFD-mangled value would be a
	// silent lie (shared conformance cases hostile_token_responses/*_lone_surrogate).
	// Unknown fields are not validated. A non-string refresh_token/token_type already
	// fails the json.Unmarshal above (string-typed fields).
	if tr.hasInvalidUnicode() {
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: "token-endpoint 2xx response contained a string that was not valid Unicode"}
	}
	if tr.AccessToken == "" {
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: "token-endpoint 2xx response missing access_token"}
	}
	if tr.ExpiresIn <= 0 {
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: "token-endpoint 2xx response missing or invalid expires_in"}
	}
	if tr.ExpiresIn > maxExpiresIn {
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: "token-endpoint 2xx response expires_in exceeds the supported maximum"}
	}

	refreshed := &Tokens{
		AccessToken: tr.AccessToken,
		// Persist the rotated token; fall back to the old one only if the server omits it
		// (omitted, null and "" all decode to "" — see below).
		RefreshToken: tr.RefreshToken,
		// Exact: 1 <= ExpiresIn <= maxExpiresIn, so the sum cannot overflow int64.
		ExpiresAt: time.Now().Unix() + tr.ExpiresIn,
		Scope:     tr.grantedScope(),
		TokenType: tr.TokenType,
	}
	// An omitted, null or EMPTY refresh_token/token_type all decode to "" and mean "the
	// server didn't rotate it": keep the current value (persisting "" would make the next
	// refresh 400). Pinned by the shared refresh_success_cases table
	// (refresh_token_{omitted,null,empty}, token_type_{null,empty}).
	if refreshed.RefreshToken == "" {
		refreshed.RefreshToken = current.RefreshToken
	}
	// An omitted, null, empty, whitespace-only (incl. U+00A0), or non-string scope means
	// "unchanged" (RFC 6749 §5.1 lets the server omit it): keep the prior grant rather than
	// persisting a blank that would erase it (#116's re-consent check reads it). Pinned by
	// the shared refresh_success_cases conformance table.
	if refreshed.Scope == "" {
		refreshed.Scope = current.Scope
	}
	if refreshed.TokenType == "" {
		refreshed.TokenType = current.TokenType
	}
	return refreshed, nil
}
