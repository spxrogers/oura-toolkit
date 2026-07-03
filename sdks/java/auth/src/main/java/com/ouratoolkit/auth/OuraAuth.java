package com.ouratoolkit.auth;

import java.net.http.HttpRequest;
import java.util.Objects;
import java.util.function.Consumer;

/**
 * The seam between this auth companion and the generated {@code com.ouratoolkit:api}
 * client (which is auth-agnostic by design): a request interceptor that injects a FRESH
 * {@code Authorization: Bearer} header on every call, refreshing + persisting rotations
 * through the shared {@link TokenManager}.
 *
 * <p>The interceptor is a plain JDK {@code Consumer<HttpRequest.Builder>} — exactly what
 * the generated client's {@code ApiClient.setRequestInterceptor} accepts — so this module
 * needs no compile-time dependency on the generated code.
 *
 * <p>Usage with the generated client:
 *
 * <pre>{@code
 * import com.ouratoolkit.api.ApiClient;
 * import com.ouratoolkit.api.client.DailySleepRoutesApi;
 * import com.ouratoolkit.auth.OuraAuth;
 * import com.ouratoolkit.auth.TokenManager;
 *
 * TokenManager manager = TokenManager.load();
 * ApiClient client = new ApiClient();
 * client.setRequestInterceptor(OuraAuth.bearerInterceptor(manager));
 * DailySleepRoutesApi sleep = new DailySleepRoutesApi(client);
 * }</pre>
 *
 * <p>Auth failures inside the interceptor surface as {@link UncheckedAuthException}
 * (Consumers cannot throw checked exceptions); unwrap with {@code getCause()}.
 */
public final class OuraAuth {

    private OuraAuth() {}

    /**
     * A request interceptor that sets {@code Authorization: Bearer <fresh token>} per
     * call. Proactive refresh happens inside {@link TokenManager#getAccessToken()}; for
     * reactive handling of a 401 (a token revoked mid-lifetime), call
     * {@link TokenManager#forceRefresh()} and retry the request once.
     */
    public static Consumer<HttpRequest.Builder> bearerInterceptor(TokenManager manager) {
        Objects.requireNonNull(manager, "manager");
        return builder -> {
            try {
                builder.setHeader("Authorization", "Bearer " + manager.getAccessToken());
            } catch (AuthException e) {
                throw new UncheckedAuthException(e);
            }
        };
    }
}
