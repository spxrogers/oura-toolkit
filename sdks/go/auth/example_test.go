package auth_test

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"net/http/httptest"
	"os"
	"time"

	api "github.com/spxrogers/oura-toolkit/sdks/go/api"
	"github.com/spxrogers/oura-toolkit/sdks/go/auth"
)

// Example wires the auth companion into the generated client. Manager implements
// oauth2.TokenSource, and the generated client accepts one via api.ContextOAuth2 — every
// request then carries a fresh Bearer token, with refresh (and Oura's refresh-token
// rotation) handled and persisted by the companion.
//
// In production you would call auth.LoadManager(): the store is shared with the CLI, so
// running `oura auth setup` and `oura auth login` once lets this program pick up the
// credentials and tokens from the same fixed config directory. This runnable example
// instead constructs the Manager explicitly and points both the token endpoint and the
// data plane at local httptest servers, so it executes hermetically (no network, no real
// credentials) and is checked against the generated client's real API in CI.
func Example() {
	// Stand-in for Oura's data plane: it authorizes on the fresh Bearer the companion
	// injects, then returns a personal-info document.
	data := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("Authorization") != "Bearer access-token-xyz" {
			http.Error(w, "unauthenticated", http.StatusUnauthorized)
			return
		}
		w.Header().Set("Content-Type", "application/json")
		fmt.Fprintln(w, `{"id":"user-42"}`)
	}))
	defer data.Close()

	dir, err := os.MkdirTemp("", "oura-example-store")
	if err != nil {
		log.Fatal(err)
	}
	defer os.RemoveAll(dir)

	// A manager holding a still-valid token (an hour out), so serving a request needs no
	// refresh — the happy path for a freshly logged-in CLI store.
	mgr := auth.NewManager(
		auth.NewStoreAt(dir),
		&auth.ClientCredentials{ClientID: "cid", ClientSecret: "secret"},
		&auth.Tokens{
			AccessToken:  "access-token-xyz",
			RefreshToken: "refresh-token-1",
			ExpiresAt:    time.Now().Add(time.Hour).Unix(),
		},
	)

	cfg := api.NewConfiguration()
	cfg.Servers = api.ServerConfigurations{{URL: data.URL}}
	client := api.NewAPIClient(cfg)

	ctx := context.WithValue(context.Background(), api.ContextOAuth2, mgr)
	info, _, err := client.PersonalInfoRoutesAPI.
		SinglePersonalInfoDocumentV2UsercollectionPersonalInfoGet(ctx).
		Execute()
	if err != nil {
		log.Fatal(err) // auth.ErrNotAuthenticated → run `oura auth login`
	}
	fmt.Println(info.GetId())
	// Output: user-42
}
