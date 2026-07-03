package auth

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"strings"
	"time"
)

// tokenResponse is the raw token-endpoint response (Oura returns a rotated
// refresh_token on every call).
type tokenResponse struct {
	AccessToken  string `json:"access_token"`
	RefreshToken string `json:"refresh_token"`
	ExpiresIn    int64  `json:"expires_in"`
	TokenType    string `json:"token_type"`
	Scope        string `json:"scope"`
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
		return nil, &TokenEndpointError{Status: resp.StatusCode, Body: string(body)}
	}

	var tr tokenResponse
	if err := json.NewDecoder(resp.Body).Decode(&tr); err != nil {
		return nil, fmt.Errorf("token endpoint response error: %w", err)
	}

	refreshed := &Tokens{
		AccessToken: tr.AccessToken,
		// Persist the rotated token; fall back to the old one only if the server omits it.
		RefreshToken: tr.RefreshToken,
		ExpiresAt:    time.Now().Unix() + tr.ExpiresIn,
		Scope:        tr.Scope,
		TokenType:    tr.TokenType,
	}
	if refreshed.RefreshToken == "" {
		refreshed.RefreshToken = current.RefreshToken
	}
	if refreshed.Scope == "" {
		refreshed.Scope = current.Scope
	}
	if refreshed.TokenType == "" {
		refreshed.TokenType = current.TokenType
	}
	return refreshed, nil
}
