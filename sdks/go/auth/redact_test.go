package auth

import (
	"fmt"
	"strings"
	"testing"
)

// The "no secrets in logs" attack test: every fmt path a stray debug line could take —
// %v, %+v, %#v, %s, fmt.Sprint, value and pointer, and the records nested inside another
// struct — must never emit the client secret, access token, or refresh token.
func TestFormattingNeverLeaksSecrets(t *testing.T) {
	creds := sampleCredentials() // ClientSecret "SECRET-CS-789"
	tokens := sampleTokens()     // AccessToken "SECRET-AT-123", RefreshToken "SECRET-RT-456"
	type carrier struct {        // e.g. a caller logging its own config struct
		C ClientCredentials
		T Tokens
	}
	nested := carrier{C: *creds, T: *tokens}

	rendered := []string{
		fmt.Sprintf("%v", *creds), fmt.Sprintf("%v", creds),
		fmt.Sprintf("%+v", *creds), fmt.Sprintf("%+v", creds),
		fmt.Sprintf("%#v", *creds), fmt.Sprintf("%#v", creds),
		fmt.Sprintf("%s", *creds), fmt.Sprint(creds),
		creds.String(),
		fmt.Sprintf("%v", *tokens), fmt.Sprintf("%v", tokens),
		fmt.Sprintf("%+v", *tokens), fmt.Sprintf("%+v", tokens),
		fmt.Sprintf("%#v", *tokens), fmt.Sprintf("%#v", tokens),
		fmt.Sprintf("%s", *tokens), fmt.Sprint(tokens),
		tokens.String(),
		fmt.Sprintf("%v", nested), fmt.Sprintf("%+v", nested),
	}
	for _, out := range rendered {
		for _, secret := range []string{"SECRET-CS-789", "SECRET-AT-123", "SECRET-RT-456"} {
			if strings.Contains(out, secret) {
				t.Fatalf("secret %s leaked through formatting: %s", secret, out)
			}
		}
	}

	// The redaction must not blank the record entirely: non-secret fields stay debuggable.
	if !strings.Contains(creds.String(), "cid") {
		t.Fatalf("client_id should remain visible: %s", creds.String())
	}
	if !strings.Contains(tokens.String(), "4102444800") {
		t.Fatalf("expires_at should remain visible: %s", tokens.String())
	}
	if !strings.Contains(tokens.String(), "[REDACTED]") {
		t.Fatalf("redaction marker missing: %s", tokens.String())
	}
}
