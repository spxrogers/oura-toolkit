package auth

import (
	"errors"
	"fmt"
)

// ErrNotAuthenticated means no tokens are stored. The library deliberately embeds no
// remediation hint — callers own the UX (the CLI's is "run `oura auth login`").
var ErrNotAuthenticated = errors.New("not authenticated (no tokens stored)")

// ErrMissingClientCredentials means tokens exist but the client-credentials record is
// missing, so a refresh is impossible (confidential client: the token endpoint requires
// client_id + client_secret). The CLI's remediation is "run `oura auth setup`".
var ErrMissingClientCredentials = errors.New("no client credentials stored")

// ErrNoConfigDir means the platform config directory could not be resolved from the
// environment. configDirFrom wraps it with the platform's variable names.
var ErrNoConfigDir = errors.New("could not determine the config directory")

// TokenEndpointError is a non-2xx response from the token endpoint (e.g. a
// rotated/expired refresh token, which Oura reports as HTTP 400) OR a 2xx response whose
// body is not a usable token set (malformed JSON, no access_token, no/zero expires_in). In
// the hostile-2xx case the Status is the 2xx code and the Body is a FIXED, secret-free
// description — never the raw response, which may carry partial token material.
type TokenEndpointError struct {
	Status int
	Body   string
}

func (e *TokenEndpointError) Error() string {
	return fmt.Sprintf("token endpoint returned HTTP %d: %s", e.Status, e.Body)
}

// StoreFormatError is a store record that exists on disk but is not a valid, complete
// record of its type: malformed JSON, the JSON literal null, a missing required field, or
// a wrong-typed field. It is deliberately TYPED and distinct from an ABSENT record (which
// loads as (nil, nil) — "not logged in / not set up yet") so a partial or corrupt file can
// never be silently loaded as a zero-valued struct, which would let IsAuthenticated report
// non-existent tokens as present. The message names the offending field, never its value —
// a store record holds secrets.
type StoreFormatError struct {
	msg string
}

func (e *StoreFormatError) Error() string { return "token store format error: " + e.msg }
