package com.ouratoolkit.smoke;

import java.time.LocalDate;
import java.util.List;

import com.ouratoolkit.api.ApiClient;
import com.ouratoolkit.api.client.SandboxRoutesApi;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicDailySleep;
import com.ouratoolkit.api.model.PublicDailySleep;

/**
 * Java-client live smoke against Oura's sandbox (network; run via
 * {@code just test-sandbox-sdks}, never CI). Proves the generated client end-to-end:
 * config, bearer injection (the same {@code setRequestInterceptor} seam the auth
 * companion's {@code OuraAuth.bearerInterceptor} plugs into), request, and jackson
 * response parsing. The sandbox accepts any bearer value; override with
 * {@code OURA_SANDBOX_TOKEN} if that ever changes.
 */
public final class JavaSmoke {

    private JavaSmoke() {}

    public static void main(String[] args) throws Exception {
        String token = System.getenv().getOrDefault("OURA_SANDBOX_TOKEN", "sandbox-smoke");
        ApiClient client = new ApiClient();
        client.setRequestInterceptor(b -> b.setHeader("Authorization", "Bearer " + token));

        MultiDocumentResponsePublicDailySleep res = new SandboxRoutesApi(client)
                .sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(
                        LocalDate.of(2026, 6, 25), LocalDate.of(2026, 7, 1), null);

        List<PublicDailySleep> data = res.getData();
        if (data == null) {
            System.err.println("java smoke FAILED: no data array");
            System.exit(1);
        }
        String firstDay = data.isEmpty() ? null : data.get(0).getDay();
        System.out.println("java smoke OK: " + data.size()
                + " sandbox daily_sleep docs, first day " + firstDay);
    }
}
