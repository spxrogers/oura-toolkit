// Go-client live smoke against Oura's sandbox (network; run via `just test-sandbox-sdks`,
// never CI). Proves the generated client end-to-end: config, bearer injection, request,
// and typed response parsing. The sandbox accepts any bearer value; override with
// OURA_SANDBOX_TOKEN if that ever changes.
package main

import (
	"context"
	"fmt"
	"os"

	api "github.com/spxrogers/oura-toolkit/sdks/go/api"
)

func main() {
	token := os.Getenv("OURA_SANDBOX_TOKEN")
	if token == "" {
		token = "sandbox-smoke"
	}
	ctx := context.WithValue(context.Background(), api.ContextAccessToken, token)
	client := api.NewAPIClient(api.NewConfiguration())
	res, _, err := client.SandboxRoutesAPI.
		SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(ctx).
		StartDate("2026-06-25").EndDate("2026-07-01").Execute()
	if err != nil {
		fmt.Fprintln(os.Stderr, "go smoke FAILED:", err)
		os.Exit(1)
	}
	if res.Data == nil {
		fmt.Fprintln(os.Stderr, "go smoke FAILED: no data array")
		os.Exit(1)
	}
	first := ""
	if len(res.Data) > 0 && res.Data[0].Day.IsSet() {
		first = *res.Data[0].Day.Get()
	}
	fmt.Printf("go smoke OK: %d sandbox daily_sleep docs, first day %s\n", len(res.Data), first)
}
