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
// rotated/expired refresh token, which Oura reports as HTTP 400).
type TokenEndpointError struct {
	Status int
	Body   string
}

func (e *TokenEndpointError) Error() string {
	return fmt.Sprintf("token endpoint returned HTTP %d: %s", e.Status, e.Body)
}
