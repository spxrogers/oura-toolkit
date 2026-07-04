// C#-client live smoke against Oura's sandbox (network; run via `just test-sandbox-sdks`,
// never CI). Proves the generated client end-to-end: config, bearer injection, request,
// and typed response parsing. The sandbox accepts any bearer value; override with
// OURA_SANDBOX_TOKEN if that ever changes.
using OuraToolkit.Api.Api;
using OuraToolkit.Api.Client;

var token = Environment.GetEnvironmentVariable("OURA_SANDBOX_TOKEN") ?? "sandbox-smoke";
var config = new Configuration { AccessToken = token };
var api = new SandboxRoutesApi(config);

// The netstandard2.0 client is Newtonsoft-based and types the collapsed `date` params as
// DateTime? (not DateOnly — that BCL type does not exist on netstandard2.0), so the smoke
// passes DateTime. The wire format is Configuration.DateTimeFormat ("o", full ISO-8601),
// which Oura's sandbox accepts (verified live: 6 daily_sleep docs returned).
var res = api.SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(
    new DateTime(2026, 6, 25), new DateTime(2026, 7, 1));
if (res?.Data is not { } data)
{
    Console.Error.WriteLine("csharp smoke FAILED: no data array");
    return 1;
}
var first = data.Count > 0 ? data[0].Day : "";
Console.WriteLine($"csharp smoke OK: {data.Count} sandbox daily_sleep docs, first day {first}");
return 0;
