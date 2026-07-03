// TypeScript-client live smoke against Oura's sandbox (network; run via
// `just test-sandbox-sdks`, never CI). Proves the generated client end-to-end: config,
// bearer injection, request, and typed response parsing. The sandbox accepts any bearer
// value; override with OURA_SANDBOX_TOKEN if that ever changes.
const { Configuration, SandboxRoutesApi } = require("../../sdks/typescript/api/dist");

(async () => {
  const api = new SandboxRoutesApi(
    new Configuration({ accessToken: process.env.OURA_SANDBOX_TOKEN || "sandbox-smoke" })
  );
  const res = await api.sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(
    { startDate: new Date("2026-06-25"), endDate: new Date("2026-07-01") }
  );
  if (!Array.isArray(res.data)) throw new Error("sandbox daily_sleep: no data array");
  console.log(`ts smoke OK: ${res.data.length} sandbox daily_sleep docs, first day ${res.data[0]?.day}`);
})().catch((e) => {
  console.error("ts smoke FAILED:", e.message || e);
  process.exit(1);
});
