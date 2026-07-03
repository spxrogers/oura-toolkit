package com.ouratoolkit.auth;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.net.URLDecoder;
import java.nio.charset.StandardCharsets;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.function.Function;

import com.sun.net.httpserver.HttpServer;

/**
 * Hermetic loopback token endpoint (jdk.httpserver — zero extra deps, per the repo's
 * "hermetic just test" law: no network, no real credentials). Each test supplies a
 * handler from parsed form params to a canned response; {@link #requests} counts calls so
 * tests can pin "exactly one refresh" claims the way the Rust suite's wiremock
 * {@code expect(1)} does.
 */
final class TokenEndpointStub implements AutoCloseable {

    /** A canned HTTP response. */
    static final class Response {
        final int status;
        final String body;

        Response(int status, String body) {
            this.status = status;
            this.body = body;
        }
    }

    final AtomicInteger requests = new AtomicInteger();
    private final HttpServer server;

    TokenEndpointStub(Function<Map<String, String>, Response> handler) throws IOException {
        server = HttpServer.create(new InetSocketAddress("127.0.0.1", 0), 0);
        server.createContext("/token", exchange -> {
            requests.incrementAndGet();
            Map<String, String> form = parseForm(readAll(exchange.getRequestBody()));
            Response response = handler.apply(form);
            byte[] bytes = response.body.getBytes(StandardCharsets.UTF_8);
            exchange.getResponseHeaders().set("Content-Type", "application/json");
            exchange.sendResponseHeaders(response.status, bytes.length);
            try (OutputStream out = exchange.getResponseBody()) {
                out.write(bytes);
            }
        });
        server.start();
    }

    String url() {
        return "http://127.0.0.1:" + server.getAddress().getPort() + "/token";
    }

    /** A 200 token response; pass a null refresh to omit rotation. */
    static Response ok(String accessToken, String refreshToken, long expiresIn) {
        String rotated = refreshToken == null
                ? ""
                : "\"refresh_token\":\"" + refreshToken + "\",";
        return new Response(200,
                "{\"access_token\":\"" + accessToken + "\"," + rotated
                        + "\"expires_in\":" + expiresIn + ",\"token_type\":\"Bearer\"}");
    }

    static Response invalidGrant() {
        return new Response(400, "{\"error\":\"invalid_grant\"}");
    }

    private static String readAll(InputStream in) throws IOException {
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        byte[] buf = new byte[4096];
        int n;
        while ((n = in.read(buf)) != -1) {
            out.write(buf, 0, n);
        }
        return new String(out.toByteArray(), StandardCharsets.UTF_8);
    }

    private static Map<String, String> parseForm(String body) {
        Map<String, String> form = new LinkedHashMap<>();
        for (String pair : body.split("&")) {
            int eq = pair.indexOf('=');
            if (eq > 0) {
                form.put(
                        URLDecoder.decode(pair.substring(0, eq), StandardCharsets.UTF_8),
                        URLDecoder.decode(pair.substring(eq + 1), StandardCharsets.UTF_8));
            }
        }
        return form;
    }

    @Override
    public void close() {
        server.stop(0);
    }
}
