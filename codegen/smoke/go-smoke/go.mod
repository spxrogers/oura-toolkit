// Smoke-only module (never published): imports the Go SDK by its real module path,
// resolved locally via the replace directive.
module github.com/spxrogers/oura-toolkit/codegen/smoke/go-smoke

go 1.25.0

require github.com/spxrogers/oura-toolkit/sdks/go v0.0.0

require golang.org/x/oauth2 v0.36.0 // indirect

replace github.com/spxrogers/oura-toolkit/sdks/go => ../../../sdks/go
