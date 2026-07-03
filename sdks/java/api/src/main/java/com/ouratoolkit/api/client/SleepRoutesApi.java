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

package com.ouratoolkit.api.client;

import com.ouratoolkit.api.ApiClient;
import com.ouratoolkit.api.ApiException;
import com.ouratoolkit.api.ApiResponse;
import com.ouratoolkit.api.Configuration;
import com.ouratoolkit.api.Pair;

import com.ouratoolkit.api.model.HTTPValidationError;
import java.time.LocalDate;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicModifiedSleepModel;
import com.ouratoolkit.api.model.PublicModifiedSleepModel;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;

import java.io.InputStream;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.io.OutputStream;
import java.net.http.HttpRequest;
import java.nio.channels.Channels;
import java.nio.channels.Pipe;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;

import java.util.ArrayList;
import java.util.StringJoiner;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.function.Consumer;

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class SleepRoutesApi {
  private final HttpClient memberVarHttpClient;
  private final ObjectMapper memberVarObjectMapper;
  private final String memberVarBaseUri;
  private final Consumer<HttpRequest.Builder> memberVarInterceptor;
  private final Duration memberVarReadTimeout;
  private final Consumer<HttpResponse<InputStream>> memberVarResponseInterceptor;
  private final Consumer<HttpResponse<String>> memberVarAsyncResponseInterceptor;

  public SleepRoutesApi() {
    this(Configuration.getDefaultApiClient());
  }

  public SleepRoutesApi(ApiClient apiClient) {
    memberVarHttpClient = apiClient.getHttpClient();
    memberVarObjectMapper = apiClient.getObjectMapper();
    memberVarBaseUri = apiClient.getBaseUri();
    memberVarInterceptor = apiClient.getRequestInterceptor();
    memberVarReadTimeout = apiClient.getReadTimeout();
    memberVarResponseInterceptor = apiClient.getResponseInterceptor();
    memberVarAsyncResponseInterceptor = apiClient.getAsyncResponseInterceptor();
  }

  protected ApiException getApiException(String operationId, HttpResponse<InputStream> response) throws IOException {
    String body = response.body() == null ? null : new String(response.body().readAllBytes());
    String message = formatExceptionMessage(operationId, response.statusCode(), body);
    return new ApiException(response.statusCode(), message, response.headers(), body);
  }

  private String formatExceptionMessage(String operationId, int statusCode, String body) {
    if (body == null || body.isEmpty()) {
      body = "[no body]";
    }
    return operationId + " call failed with: " + statusCode + " - " + body;
  }

  /**
   * Multiple Sleep Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @param fields Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)
   * @return MultiDocumentResponsePublicModifiedSleepModel
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicModifiedSleepModel multipleSleepDocumentsV2UsercollectionSleepGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken, @javax.annotation.Nullable String fields) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> localVarResponse = multipleSleepDocumentsV2UsercollectionSleepGetWithHttpInfo(startDate, endDate, nextToken, fields);
    return localVarResponse.getData();
  }

  /**
   * Multiple Sleep Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @param fields Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicModifiedSleepModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> multipleSleepDocumentsV2UsercollectionSleepGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken, @javax.annotation.Nullable String fields) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = multipleSleepDocumentsV2UsercollectionSleepGetRequestBuilder(startDate, endDate, nextToken, fields);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("multipleSleepDocumentsV2UsercollectionSleepGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicModifiedSleepModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicModifiedSleepModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicModifiedSleepModel>() {})
        );
      } finally {
      }
    } catch (IOException e) {
      throw new ApiException(e);
    }
    catch (InterruptedException e) {
      Thread.currentThread().interrupt();
      throw new ApiException(e);
    }
  }

  private HttpRequest.Builder multipleSleepDocumentsV2UsercollectionSleepGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken, @javax.annotation.Nullable String fields) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/usercollection/sleep";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));
    localVarQueryParameterBaseName = "fields";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("fields", fields));

    if (!localVarQueryParams.isEmpty() || localVarQueryStringJoiner.length() != 0) {
      StringJoiner queryJoiner = new StringJoiner("&");
      localVarQueryParams.forEach(p -> queryJoiner.add(p.getName() + '=' + p.getValue()));
      if (localVarQueryStringJoiner.length() != 0) {
        queryJoiner.add(localVarQueryStringJoiner.toString());
      }
      localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath + '?' + queryJoiner.toString()));
    } else {
      localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));
    }

    localVarRequestBuilder.header("Accept", "application/json");

    localVarRequestBuilder.method("GET", HttpRequest.BodyPublishers.noBody());
    if (memberVarReadTimeout != null) {
      localVarRequestBuilder.timeout(memberVarReadTimeout);
    }
    if (memberVarInterceptor != null) {
      memberVarInterceptor.accept(localVarRequestBuilder);
    }
    return localVarRequestBuilder;
  }

  /**
   * Single Sleep Document
   * 
   * @param documentId  (required)
   * @return PublicModifiedSleepModel
   * @throws ApiException if fails to make API call
   */
  public PublicModifiedSleepModel singleSleepDocumentV2UsercollectionSleepDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicModifiedSleepModel> localVarResponse = singleSleepDocumentV2UsercollectionSleepDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Single Sleep Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicModifiedSleepModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicModifiedSleepModel> singleSleepDocumentV2UsercollectionSleepDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = singleSleepDocumentV2UsercollectionSleepDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("singleSleepDocumentV2UsercollectionSleepDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicModifiedSleepModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicModifiedSleepModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicModifiedSleepModel>() {})
        );
      } finally {
      }
    } catch (IOException e) {
      throw new ApiException(e);
    }
    catch (InterruptedException e) {
      Thread.currentThread().interrupt();
      throw new ApiException(e);
    }
  }

  private HttpRequest.Builder singleSleepDocumentV2UsercollectionSleepDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling singleSleepDocumentV2UsercollectionSleepDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/usercollection/sleep/{document_id}"
        .replace("{document_id}", ApiClient.urlEncode(documentId.toString()));

    localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

    localVarRequestBuilder.header("Accept", "application/json");

    localVarRequestBuilder.method("GET", HttpRequest.BodyPublishers.noBody());
    if (memberVarReadTimeout != null) {
      localVarRequestBuilder.timeout(memberVarReadTimeout);
    }
    if (memberVarInterceptor != null) {
      memberVarInterceptor.accept(localVarRequestBuilder);
    }
    return localVarRequestBuilder;
  }

}
