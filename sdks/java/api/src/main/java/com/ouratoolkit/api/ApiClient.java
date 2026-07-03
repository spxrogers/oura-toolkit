/*
 * Oura API Documentation
 * # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | ------------------------------------ | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 
 *
 * The version of the OpenAPI document: 2.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

package com.ouratoolkit.api;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;
import org.openapitools.jackson.nullable.JsonNullableModule;

import java.io.InputStream;
import java.net.URI;
import java.net.URLEncoder;
import java.net.http.HttpClient;
import java.net.http.HttpConnectTimeoutException;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;
import java.time.OffsetDateTime;
import java.time.format.DateTimeFormatter;
import java.util.Collection;
import java.util.Collections;
import java.util.List;
import java.util.StringJoiner;
import java.util.function.Consumer;
import java.util.stream.Collectors;

import static java.nio.charset.StandardCharsets.UTF_8;

/**
 * Configuration and utility class for API clients.
 *
 * <p>This class can be constructed and modified, then used to instantiate the
 * various API classes. The API classes use the settings in this class to
 * configure themselves, but otherwise do not store a link to this class.</p>
 *
 * <p>This class is mutable and not synchronized, so it is not thread-safe.
 * The API classes generated from this are immutable and thread-safe.</p>
 *
 * <p>The setter methods of this class return the current object to facilitate
 * a fluent style of configuration.</p>
 */
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class ApiClient {

  protected HttpClient.Builder builder;
  protected ObjectMapper mapper;
  protected String scheme;
  protected String host;
  protected int port;
  protected String basePath;
  protected Consumer<HttpRequest.Builder> interceptor;
  protected Consumer<HttpResponse<InputStream>> responseInterceptor;
  protected Consumer<HttpResponse<String>> asyncResponseInterceptor;
  protected Duration readTimeout;
  protected Duration connectTimeout;

  public static String valueToString(Object value) {
    if (value == null) {
      return "";
    }
    if (value instanceof OffsetDateTime) {
      return ((OffsetDateTime) value).format(DateTimeFormatter.ISO_OFFSET_DATE_TIME);
    }
    return value.toString();
  }

  /**
   * URL encode a string in the UTF-8 encoding.
   *
   * @param s String to encode.
   * @return URL-encoded representation of the input string.
   */
  public static String urlEncode(String s) {
    return URLEncoder.encode(s, UTF_8).replaceAll("\\+", "%20");
  }

  /**
   * Convert a URL query name/value parameter to a list of encoded {@link Pair}
   * objects.
   *
   * <p>The value can be null, in which case an empty list is returned.</p>
   *
   * @param name The query name parameter.
   * @param value The query value, which may not be a collection but may be
   *              null.
   * @return A singleton list of the {@link Pair} objects representing the input
   * parameters, which is encoded for use in a URL. If the value is null, an
   * empty list is returned.
   */
  public static List<Pair> parameterToPairs(String name, Object value) {
    if (name == null || name.isEmpty() || value == null) {
      return Collections.emptyList();
    }
    return Collections.singletonList(new Pair(urlEncode(name), urlEncode(valueToString(value))));
  }

  /**
   * Convert a URL query name/collection parameter to a list of encoded
   * {@link Pair} objects.
   *
   * @param collectionFormat The swagger collectionFormat string (csv, tsv, etc).
   * @param name The query name parameter.
   * @param values A collection of values for the given query name, which may be
   *               null.
   * @return A list of {@link Pair} objects representing the input parameters,
   * which is encoded for use in a URL. If the values collection is null, an
   * empty list is returned.
   */
  public static List<Pair> parameterToPairs(
      String collectionFormat, String name, Collection<?> values) {
    if (name == null || name.isEmpty() || values == null || values.isEmpty()) {
      return Collections.emptyList();
    }

    // get the collection format (default: csv)
    String format = collectionFormat == null || collectionFormat.isEmpty() ? "csv" : collectionFormat;

    // create the params based on the collection format
    if ("multi".equals(format)) {
      return values.stream()
          .map(value -> new Pair(urlEncode(name), urlEncode(valueToString(value))))
          .collect(Collectors.toList());
    }

    String delimiter;
    switch(format) {
      case "csv":
        delimiter = urlEncode(",");
        break;
      case "ssv":
        delimiter = urlEncode(" ");
        break;
      case "tsv":
        delimiter = urlEncode("\t");
        break;
      case "pipes":
        delimiter = urlEncode("|");
        break;
      default:
        throw new IllegalArgumentException("Illegal collection format: " + collectionFormat);
    }

    StringJoiner joiner = new StringJoiner(delimiter);
    for (Object value : values) {
      joiner.add(urlEncode(valueToString(value)));
    }

    return Collections.singletonList(new Pair(urlEncode(name), joiner.toString()));
  }

  /**
   * Create an instance of ApiClient.
   */
  public ApiClient() {
    this.builder = createDefaultHttpClientBuilder();
    this.mapper = createDefaultObjectMapper();
    updateBaseUri(getDefaultBaseUri());
    interceptor = null;
    readTimeout = null;
    connectTimeout = null;
    responseInterceptor = null;
    asyncResponseInterceptor = null;
  }

  /**
   * Create an instance of ApiClient.
   *
   * @param builder Http client builder.
   * @param mapper Object mapper.
   * @param baseUri Base URI
   */
  public ApiClient(HttpClient.Builder builder, ObjectMapper mapper, String baseUri) {
    this.builder = builder;
    this.mapper = mapper;
    updateBaseUri(baseUri != null ? baseUri : getDefaultBaseUri());
    interceptor = null;
    readTimeout = null;
    connectTimeout = null;
    responseInterceptor = null;
    asyncResponseInterceptor = null;
  }

  public static ObjectMapper createDefaultObjectMapper() {
    ObjectMapper mapper = new ObjectMapper();
    mapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);
    mapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
    mapper.configure(DeserializationFeature.FAIL_ON_INVALID_SUBTYPE, false);
    mapper.disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS);
    mapper.enable(SerializationFeature.WRITE_ENUMS_USING_TO_STRING);
    mapper.enable(DeserializationFeature.READ_ENUMS_USING_TO_STRING);
    mapper.disable(DeserializationFeature.ADJUST_DATES_TO_CONTEXT_TIME_ZONE);
    mapper.registerModule(new JavaTimeModule());
    mapper.registerModule(new JsonNullableModule());
    mapper.registerModule(new RFC3339JavaTimeModule());
    return mapper;
  }

  protected String getDefaultBaseUri() {
    return "https://api.ouraring.com";
  }

  public static HttpClient.Builder createDefaultHttpClientBuilder() {
    return HttpClient.newBuilder();
  }

  public final void updateBaseUri(String baseUri) {
    URI uri = URI.create(baseUri);
    scheme = uri.getScheme();
    host = uri.getHost();
    port = uri.getPort();
    basePath = uri.getRawPath();
  }

  /**
   * Set a custom {@link HttpClient.Builder} object to use when creating the
   * {@link HttpClient} that is used by the API client.
   *
   * @param builder Custom client builder.
   * @return This object.
   */
  public ApiClient setHttpClientBuilder(HttpClient.Builder builder) {
    this.builder = builder;
    return this;
  }

  /**
   * Get an {@link HttpClient} based on the current {@link HttpClient.Builder}.
   *
   * <p>The returned object is immutable and thread-safe.</p>
   *
   * @return The HTTP client.
   */
  public HttpClient getHttpClient() {
    return builder.build();
  }

  /**
   * Set a custom {@link ObjectMapper} to serialize and deserialize the request
   * and response bodies.
   *
   * @param mapper Custom object mapper.
   * @return This object.
   */
  public ApiClient setObjectMapper(ObjectMapper mapper) {
    this.mapper = mapper;
    return this;
  }

  /**
   * Get a copy of the current {@link ObjectMapper}.
   *
   * @return A copy of the current object mapper.
   */
  public ObjectMapper getObjectMapper() {
    return mapper.copy();
  }

  /**
   * Set a custom host name for the target service.
   *
   * @param host The host name of the target service.
   * @return This object.
   */
  public ApiClient setHost(String host) {
    this.host = host;
    return this;
  }

  /**
   * Set a custom port number for the target service.
   *
   * @param port The port of the target service. Set this to -1 to reset the
   *             value to the default for the scheme.
   * @return This object.
   */
  public ApiClient setPort(int port) {
    this.port = port;
    return this;
  }

  /**
   * Set a custom base path for the target service, for example '/v2'.
   *
   * @param basePath The base path against which the rest of the path is
   *                 resolved.
   * @return This object.
   */
  public ApiClient setBasePath(String basePath) {
    this.basePath = basePath;
    return this;
  }

  /**
   * Get the base URI to resolve the endpoint paths against.
   *
   * @return The complete base URI that the rest of the API parameters are
   * resolved against.
   */
  public String getBaseUri() {
    return scheme + "://" + host + (port == -1 ? "" : ":" + port) + basePath;
  }

  /**
   * Set a custom scheme for the target service, for example 'https'.
   *
   * @param scheme The scheme of the target service
   * @return This object.
   */
  public ApiClient setScheme(String scheme){
    this.scheme = scheme;
    return this;
  }

  /**
   * Set a custom request interceptor.
   *
   * <p>A request interceptor is a mechanism for altering each request before it
   * is sent. After the request has been fully configured but not yet built, the
   * request builder is passed into this function for further modification,
   * after which it is sent out.</p>
   *
   * <p>This is useful for altering the requests in a custom manner, such as
   * adding headers. It could also be used for logging and monitoring.</p>
   *
   * @param interceptor A function invoked before creating each request. A value
   *                    of null resets the interceptor to a no-op.
   * @return This object.
   */
  public ApiClient setRequestInterceptor(Consumer<HttpRequest.Builder> interceptor) {
    this.interceptor = interceptor;
    return this;
  }

  /**
   * Get the custom interceptor.
   *
   * @return The custom interceptor that was set, or null if there isn't any.
   */
  public Consumer<HttpRequest.Builder> getRequestInterceptor() {
    return interceptor;
  }

  /**
   * Set a custom response interceptor.
   *
   * <p>This is useful for logging, monitoring or extraction of header variables</p>
   *
   * @param interceptor A function invoked before creating each request. A value
   *                    of null resets the interceptor to a no-op.
   * @return This object.
   */
  public ApiClient setResponseInterceptor(Consumer<HttpResponse<InputStream>> interceptor) {
    this.responseInterceptor = interceptor;
    return this;
  }

 /**
   * Get the custom response interceptor.
   *
   * @return The custom interceptor that was set, or null if there isn't any.
   */
  public Consumer<HttpResponse<InputStream>> getResponseInterceptor() {
    return responseInterceptor;
  }

  /**
   * Set a custom async response interceptor. Use this interceptor when asyncNative is set to 'true'.
   *
   * <p>This is useful for logging, monitoring or extraction of header variables</p>
   *
   * @param interceptor A function invoked before creating each request. A value
   *                    of null resets the interceptor to a no-op.
   * @return This object.
   */
  public ApiClient setAsyncResponseInterceptor(Consumer<HttpResponse<String>> interceptor) {
    this.asyncResponseInterceptor = interceptor;
    return this;
  }

 /**
   * Get the custom async response interceptor. Use this interceptor when asyncNative is set to 'true'.
   *
   * @return The custom interceptor that was set, or null if there isn't any.
   */
  public Consumer<HttpResponse<String>> getAsyncResponseInterceptor() {
    return asyncResponseInterceptor;
  }

  /**
   * Set the read timeout for the http client.
   *
   * <p>This is the value used by default for each request, though it can be
   * overridden on a per-request basis with a request interceptor.</p>
   *
   * @param readTimeout The read timeout used by default by the http client.
   *                    Setting this value to null resets the timeout to an
   *                    effectively infinite value.
   * @return This object.
   */
  public ApiClient setReadTimeout(Duration readTimeout) {
    this.readTimeout = readTimeout;
    return this;
  }

  /**
   * Get the read timeout that was set.
   *
   * @return The read timeout, or null if no timeout was set. Null represents
   * an infinite wait time.
   */
  public Duration getReadTimeout() {
    return readTimeout;
  }
  /**
   * Sets the connect timeout (in milliseconds) for the http client.
   *
   * <p> In the case where a new connection needs to be established, if
   * the connection cannot be established within the given {@code
   * duration}, then {@link HttpClient#send(HttpRequest,BodyHandler)
   * HttpClient::send} throws an {@link HttpConnectTimeoutException}, or
   * {@link HttpClient#sendAsync(HttpRequest,BodyHandler)
   * HttpClient::sendAsync} completes exceptionally with an
   * {@code HttpConnectTimeoutException}. If a new connection does not
   * need to be established, for example if a connection can be reused
   * from a previous request, then this timeout duration has no effect.
   *
   * @param connectTimeout connection timeout in milliseconds
   *
   * @return This object.
   */
  public ApiClient setConnectTimeout(Duration connectTimeout) {
    this.connectTimeout = connectTimeout;
    this.builder.connectTimeout(connectTimeout);
    return this;
  }

  /**
   * Get connection timeout (in milliseconds).
   *
   * @return Timeout in milliseconds
   */
  public Duration getConnectTimeout() {
    return connectTimeout;
  }
}
