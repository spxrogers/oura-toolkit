package auth_test

import (
	"context"
	"fmt"
	"log"

	api "github.com/spxrogers/oura-toolkit/sdks/go/api"
	"github.com/spxrogers/oura-toolkit/sdks/go/auth"
)

// Example wires the auth companion into the generated client. Manager implements
// oauth2.TokenSource, and the generated client accepts one via api.ContextOAuth2 — every
// request then carries a fresh Bearer token, with refresh (and Oura's refresh-token
// rotation) handled and persisted by the companion.
//
// The store is shared with the CLI: run `oura auth setup` and `oura auth login` once, and
// this program picks the credentials and tokens up from the same fixed config directory.
//
// (No Output comment: this example is compile-checked, not executed — it needs real
// credentials.)
func Example() {
	mgr, err := auth.LoadManager()
	if err != nil {
		log.Fatal(err)
	}

	ctx := context.WithValue(context.Background(), api.ContextOAuth2, mgr)
	client := api.NewAPIClient(api.NewConfiguration())

	info, _, err := client.PersonalInfoRoutesAPI.
		SinglePersonalInfoDocumentV2UsercollectionPersonalInfoGet(ctx).
		Execute()
	if err != nil {
		log.Fatal(err) // auth.ErrNotAuthenticated → run `oura auth login`
	}
	fmt.Println(info.GetId())
}
