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

import com.ouratoolkit.api.model.DailyResilienceModel;
import com.ouratoolkit.api.model.EnhancedTagModel;
import com.ouratoolkit.api.model.HTTPValidationError;
import java.time.LocalDate;
import com.ouratoolkit.api.model.MultiDocumentResponseDailyResilienceModel;
import com.ouratoolkit.api.model.MultiDocumentResponseEnhancedTagModel;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicDailyActivity;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicDailyCardiovascularAge;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicDailyReadiness;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicDailySleep;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicDailySpO2;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicDailyStress;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicModifiedSleepModel;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicRestModePeriod;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicRingConfiguration;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicSession;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicSleepTime;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicVO2Max;
import com.ouratoolkit.api.model.MultiDocumentResponsePublicWorkout;
import com.ouratoolkit.api.model.MultiDocumentResponseTagModel;
import java.time.OffsetDateTime;
import com.ouratoolkit.api.model.PublicDailyActivity;
import com.ouratoolkit.api.model.PublicDailyCardiovascularAge;
import com.ouratoolkit.api.model.PublicDailyReadiness;
import com.ouratoolkit.api.model.PublicDailySleep;
import com.ouratoolkit.api.model.PublicDailySpO2;
import com.ouratoolkit.api.model.PublicDailyStress;
import com.ouratoolkit.api.model.PublicModifiedSleepModel;
import com.ouratoolkit.api.model.PublicRestModePeriod;
import com.ouratoolkit.api.model.PublicRingConfiguration;
import com.ouratoolkit.api.model.PublicSession;
import com.ouratoolkit.api.model.PublicSleepTime;
import com.ouratoolkit.api.model.PublicVO2Max;
import com.ouratoolkit.api.model.PublicWorkout;
import com.ouratoolkit.api.model.ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet;
import com.ouratoolkit.api.model.ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet;
import com.ouratoolkit.api.model.TagModel;

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
public class SandboxRoutesApi {
  private final HttpClient memberVarHttpClient;
  private final ObjectMapper memberVarObjectMapper;
  private final String memberVarBaseUri;
  private final Consumer<HttpRequest.Builder> memberVarInterceptor;
  private final Duration memberVarReadTimeout;
  private final Consumer<HttpResponse<InputStream>> memberVarResponseInterceptor;
  private final Consumer<HttpResponse<String>> memberVarAsyncResponseInterceptor;

  public SandboxRoutesApi() {
    this(Configuration.getDefaultApiClient());
  }

  public SandboxRoutesApi(ApiClient apiClient) {
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
   * Sandbox - Multiple Daily Activity Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicDailyActivity
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicDailyActivity sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicDailyActivity> localVarResponse = sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Daily Activity Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicDailyActivity&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicDailyActivity> sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicDailyActivity>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicDailyActivity>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicDailyActivity>() {})
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

  private HttpRequest.Builder sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_activity";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Daily Cardiovascular Age Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicDailyCardiovascularAge
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicDailyCardiovascularAge sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge> localVarResponse = sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Daily Cardiovascular Age Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicDailyCardiovascularAge&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge> sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicDailyCardiovascularAge>() {})
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

  private HttpRequest.Builder sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_cardiovascular_age";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Daily Readiness Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicDailyReadiness
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicDailyReadiness sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicDailyReadiness> localVarResponse = sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Daily Readiness Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicDailyReadiness&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicDailyReadiness> sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicDailyReadiness>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicDailyReadiness>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicDailyReadiness>() {})
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

  private HttpRequest.Builder sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_readiness";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Daily Resilience Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponseDailyResilienceModel
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponseDailyResilienceModel sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponseDailyResilienceModel> localVarResponse = sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Daily Resilience Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponseDailyResilienceModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponseDailyResilienceModel> sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponseDailyResilienceModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponseDailyResilienceModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponseDailyResilienceModel>() {})
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

  private HttpRequest.Builder sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_resilience";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Daily Sleep Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicDailySleep
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicDailySleep sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicDailySleep> localVarResponse = sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Daily Sleep Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicDailySleep&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicDailySleep> sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicDailySleep>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicDailySleep>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicDailySleep>() {})
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

  private HttpRequest.Builder sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_sleep";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Daily Spo2 Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicDailySpO2
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicDailySpO2 sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicDailySpO2> localVarResponse = sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Daily Spo2 Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicDailySpO2&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicDailySpO2> sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicDailySpO2>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicDailySpO2>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicDailySpO2>() {})
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

  private HttpRequest.Builder sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_spo2";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Daily Stress Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicDailyStress
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicDailyStress sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicDailyStress> localVarResponse = sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Daily Stress Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicDailyStress&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicDailyStress> sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicDailyStress>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicDailyStress>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicDailyStress>() {})
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

  private HttpRequest.Builder sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_stress";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Enhanced Tag Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponseEnhancedTagModel
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponseEnhancedTagModel sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponseEnhancedTagModel> localVarResponse = sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Enhanced Tag Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponseEnhancedTagModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponseEnhancedTagModel> sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponseEnhancedTagModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponseEnhancedTagModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponseEnhancedTagModel>() {})
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

  private HttpRequest.Builder sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/enhanced_tag";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Heartrate Documents
   * 
   * @param startDatetime  (optional)
   * @param endDatetime  (optional)
   * @param nextToken  (optional)
   * @return ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet
   * @throws ApiException if fails to make API call
   */
  public ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet(@javax.annotation.Nullable OffsetDateTime startDatetime, @javax.annotation.Nullable OffsetDateTime endDatetime, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> localVarResponse = sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfo(startDatetime, endDatetime, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Heartrate Documents
   * 
   * @param startDatetime  (optional)
   * @param endDatetime  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfo(@javax.annotation.Nullable OffsetDateTime startDatetime, @javax.annotation.Nullable OffsetDateTime endDatetime, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetRequestBuilder(startDatetime, endDatetime, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>() {})
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

  private HttpRequest.Builder sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetRequestBuilder(@javax.annotation.Nullable OffsetDateTime startDatetime, @javax.annotation.Nullable OffsetDateTime endDatetime, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/heartrate";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_datetime";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_datetime", startDatetime));
    localVarQueryParameterBaseName = "end_datetime";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_datetime", endDatetime));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Rest Mode Period Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicRestModePeriod
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicRestModePeriod sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicRestModePeriod> localVarResponse = sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Rest Mode Period Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicRestModePeriod&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicRestModePeriod> sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicRestModePeriod>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicRestModePeriod>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicRestModePeriod>() {})
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

  private HttpRequest.Builder sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/rest_mode_period";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Ring Battery Level Documents
   * 
   * @param startDatetime  (optional)
   * @param endDatetime  (optional)
   * @param nextToken  (optional)
   * @return ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet
   * @throws ApiException if fails to make API call
   */
  public ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet(@javax.annotation.Nullable OffsetDateTime startDatetime, @javax.annotation.Nullable OffsetDateTime endDatetime, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> localVarResponse = sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfo(startDatetime, endDatetime, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Ring Battery Level Documents
   * 
   * @param startDatetime  (optional)
   * @param endDatetime  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfo(@javax.annotation.Nullable OffsetDateTime startDatetime, @javax.annotation.Nullable OffsetDateTime endDatetime, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetRequestBuilder(startDatetime, endDatetime, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>() {})
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

  private HttpRequest.Builder sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetRequestBuilder(@javax.annotation.Nullable OffsetDateTime startDatetime, @javax.annotation.Nullable OffsetDateTime endDatetime, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/ring_battery_level";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_datetime";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_datetime", startDatetime));
    localVarQueryParameterBaseName = "end_datetime";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_datetime", endDatetime));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Ring Configuration Documents
   * 
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicRingConfiguration
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicRingConfiguration sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet(@javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicRingConfiguration> localVarResponse = sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfo(nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Ring Configuration Documents
   * 
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicRingConfiguration&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicRingConfiguration> sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfo(@javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetRequestBuilder(nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicRingConfiguration>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicRingConfiguration>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicRingConfiguration>() {})
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

  private HttpRequest.Builder sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetRequestBuilder(@javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/ring_configuration";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Session Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicSession
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicSession sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicSession> localVarResponse = sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Session Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicSession&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicSession> sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicSession>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicSession>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicSession>() {})
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

  private HttpRequest.Builder sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/session";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Sleep Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicModifiedSleepModel
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicModifiedSleepModel sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> localVarResponse = sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Sleep Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicModifiedSleepModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet", localVarResponse);
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

  private HttpRequest.Builder sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/sleep";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Sleep Time Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicSleepTime
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicSleepTime sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicSleepTime> localVarResponse = sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Sleep Time Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicSleepTime&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicSleepTime> sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicSleepTime>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicSleepTime>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicSleepTime>() {})
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

  private HttpRequest.Builder sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/sleep_time";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Tag Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponseTagModel
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponseTagModel sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponseTagModel> localVarResponse = sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Tag Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponseTagModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponseTagModel> sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponseTagModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponseTagModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponseTagModel>() {})
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

  private HttpRequest.Builder sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/tag";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Vo2 Max Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicVO2Max
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicVO2Max sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicVO2Max> localVarResponse = sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Vo2 Max Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicVO2Max&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicVO2Max> sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicVO2Max>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicVO2Max>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicVO2Max>() {})
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

  private HttpRequest.Builder sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/vO2_max";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Multiple Workout Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return MultiDocumentResponsePublicWorkout
   * @throws ApiException if fails to make API call
   */
  public MultiDocumentResponsePublicWorkout sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    ApiResponse<MultiDocumentResponsePublicWorkout> localVarResponse = sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfo(startDate, endDate, nextToken);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Multiple Workout Documents
   * 
   * @param startDate  (optional)
   * @param endDate  (optional)
   * @param nextToken  (optional)
   * @return ApiResponse&lt;MultiDocumentResponsePublicWorkout&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<MultiDocumentResponsePublicWorkout> sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfo(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetRequestBuilder(startDate, endDate, nextToken);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<MultiDocumentResponsePublicWorkout>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<MultiDocumentResponsePublicWorkout>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<MultiDocumentResponsePublicWorkout>() {})
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

  private HttpRequest.Builder sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetRequestBuilder(@javax.annotation.Nullable LocalDate startDate, @javax.annotation.Nullable LocalDate endDate, @javax.annotation.Nullable String nextToken) throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/workout";

    List<Pair> localVarQueryParams = new ArrayList<>();
    StringJoiner localVarQueryStringJoiner = new StringJoiner("&");
    String localVarQueryParameterBaseName;
    localVarQueryParameterBaseName = "start_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("start_date", startDate));
    localVarQueryParameterBaseName = "end_date";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("end_date", endDate));
    localVarQueryParameterBaseName = "next_token";
    localVarQueryParams.addAll(ApiClient.parameterToPairs("next_token", nextToken));

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
   * Sandbox - Single Daily Activity Document
   * 
   * @param documentId  (required)
   * @return PublicDailyActivity
   * @throws ApiException if fails to make API call
   */
  public PublicDailyActivity sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicDailyActivity> localVarResponse = sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Daily Activity Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicDailyActivity&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicDailyActivity> sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicDailyActivity>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicDailyActivity>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicDailyActivity>() {})
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

  private HttpRequest.Builder sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_activity/{document_id}"
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

  /**
   * Sandbox - Single Daily Cardiovascular Age Document
   * 
   * @param documentId  (required)
   * @return PublicDailyCardiovascularAge
   * @throws ApiException if fails to make API call
   */
  public PublicDailyCardiovascularAge sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicDailyCardiovascularAge> localVarResponse = sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Daily Cardiovascular Age Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicDailyCardiovascularAge&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicDailyCardiovascularAge> sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicDailyCardiovascularAge>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicDailyCardiovascularAge>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicDailyCardiovascularAge>() {})
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

  private HttpRequest.Builder sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_cardiovascular_age/{document_id}"
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

  /**
   * Sandbox - Single Daily Readiness Document
   * 
   * @param documentId  (required)
   * @return PublicDailyReadiness
   * @throws ApiException if fails to make API call
   */
  public PublicDailyReadiness sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicDailyReadiness> localVarResponse = sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Daily Readiness Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicDailyReadiness&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicDailyReadiness> sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicDailyReadiness>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicDailyReadiness>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicDailyReadiness>() {})
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

  private HttpRequest.Builder sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_readiness/{document_id}"
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

  /**
   * Sandbox - Single Daily Resilience Document
   * 
   * @param documentId  (required)
   * @return DailyResilienceModel
   * @throws ApiException if fails to make API call
   */
  public DailyResilienceModel sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<DailyResilienceModel> localVarResponse = sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Daily Resilience Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;DailyResilienceModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<DailyResilienceModel> sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<DailyResilienceModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<DailyResilienceModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<DailyResilienceModel>() {})
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

  private HttpRequest.Builder sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_resilience/{document_id}"
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

  /**
   * Sandbox - Single Daily Sleep Document
   * 
   * @param documentId  (required)
   * @return PublicDailySleep
   * @throws ApiException if fails to make API call
   */
  public PublicDailySleep sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicDailySleep> localVarResponse = sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Daily Sleep Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicDailySleep&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicDailySleep> sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicDailySleep>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicDailySleep>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicDailySleep>() {})
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

  private HttpRequest.Builder sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_sleep/{document_id}"
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

  /**
   * Sandbox - Single Daily Spo2 Document
   * 
   * @param documentId  (required)
   * @return PublicDailySpO2
   * @throws ApiException if fails to make API call
   */
  public PublicDailySpO2 sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicDailySpO2> localVarResponse = sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Daily Spo2 Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicDailySpO2&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicDailySpO2> sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicDailySpO2>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicDailySpO2>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicDailySpO2>() {})
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

  private HttpRequest.Builder sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_spo2/{document_id}"
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

  /**
   * Sandbox - Single Daily Stress Document
   * 
   * @param documentId  (required)
   * @return PublicDailyStress
   * @throws ApiException if fails to make API call
   */
  public PublicDailyStress sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicDailyStress> localVarResponse = sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Daily Stress Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicDailyStress&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicDailyStress> sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicDailyStress>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicDailyStress>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicDailyStress>() {})
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

  private HttpRequest.Builder sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/daily_stress/{document_id}"
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

  /**
   * Sandbox - Single Enhanced Tag Document
   * 
   * @param documentId  (required)
   * @return EnhancedTagModel
   * @throws ApiException if fails to make API call
   */
  public EnhancedTagModel sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<EnhancedTagModel> localVarResponse = sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Enhanced Tag Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;EnhancedTagModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<EnhancedTagModel> sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<EnhancedTagModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<EnhancedTagModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<EnhancedTagModel>() {})
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

  private HttpRequest.Builder sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/enhanced_tag/{document_id}"
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

  /**
   * Sandbox - Single Rest Mode Period Document
   * 
   * @param documentId  (required)
   * @return PublicRestModePeriod
   * @throws ApiException if fails to make API call
   */
  public PublicRestModePeriod sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicRestModePeriod> localVarResponse = sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Rest Mode Period Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicRestModePeriod&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicRestModePeriod> sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicRestModePeriod>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicRestModePeriod>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicRestModePeriod>() {})
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

  private HttpRequest.Builder sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/rest_mode_period/{document_id}"
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

  /**
   * Sandbox - Single Ring Configuration Document
   * 
   * @param documentId  (required)
   * @return PublicRingConfiguration
   * @throws ApiException if fails to make API call
   */
  public PublicRingConfiguration sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicRingConfiguration> localVarResponse = sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Ring Configuration Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicRingConfiguration&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicRingConfiguration> sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicRingConfiguration>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicRingConfiguration>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicRingConfiguration>() {})
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

  private HttpRequest.Builder sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/ring_configuration/{document_id}"
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

  /**
   * Sandbox - Single Session Document
   * 
   * @param documentId  (required)
   * @return PublicSession
   * @throws ApiException if fails to make API call
   */
  public PublicSession sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicSession> localVarResponse = sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Session Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicSession&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicSession> sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicSession>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicSession>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicSession>() {})
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

  private HttpRequest.Builder sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/session/{document_id}"
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

  /**
   * Sandbox - Single Sleep Document
   * 
   * @param documentId  (required)
   * @return PublicModifiedSleepModel
   * @throws ApiException if fails to make API call
   */
  public PublicModifiedSleepModel sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicModifiedSleepModel> localVarResponse = sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Sleep Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicModifiedSleepModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicModifiedSleepModel> sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet", localVarResponse);
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

  private HttpRequest.Builder sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/sleep/{document_id}"
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

  /**
   * Sandbox - Single Sleep Time Document
   * 
   * @param documentId  (required)
   * @return PublicSleepTime
   * @throws ApiException if fails to make API call
   */
  public PublicSleepTime sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicSleepTime> localVarResponse = sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Sleep Time Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicSleepTime&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicSleepTime> sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicSleepTime>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicSleepTime>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicSleepTime>() {})
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

  private HttpRequest.Builder sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/sleep_time/{document_id}"
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

  /**
   * Sandbox - Single Tag Document
   * 
   * @param documentId  (required)
   * @return TagModel
   * @throws ApiException if fails to make API call
   */
  public TagModel sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<TagModel> localVarResponse = sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Tag Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;TagModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<TagModel> sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<TagModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<TagModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<TagModel>() {})
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

  private HttpRequest.Builder sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/tag/{document_id}"
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

  /**
   * Sandbox - Single Vo2 Max Document
   * 
   * @param documentId  (required)
   * @return PublicVO2Max
   * @throws ApiException if fails to make API call
   */
  public PublicVO2Max sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicVO2Max> localVarResponse = sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Vo2 Max Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicVO2Max&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicVO2Max> sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicVO2Max>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicVO2Max>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicVO2Max>() {})
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

  private HttpRequest.Builder sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/vO2_max/{document_id}"
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

  /**
   * Sandbox - Single Workout Document
   * 
   * @param documentId  (required)
   * @return PublicWorkout
   * @throws ApiException if fails to make API call
   */
  public PublicWorkout sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet(@javax.annotation.Nonnull String documentId) throws ApiException {
    ApiResponse<PublicWorkout> localVarResponse = sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfo(documentId);
    return localVarResponse.getData();
  }

  /**
   * Sandbox - Single Workout Document
   * 
   * @param documentId  (required)
   * @return ApiResponse&lt;PublicWorkout&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<PublicWorkout> sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfo(@javax.annotation.Nonnull String documentId) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetRequestBuilder(documentId);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<PublicWorkout>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<PublicWorkout>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<PublicWorkout>() {})
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

  private HttpRequest.Builder sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetRequestBuilder(@javax.annotation.Nonnull String documentId) throws ApiException {
    // verify the required parameter 'documentId' is set
    if (documentId == null) {
      throw new ApiException(400, "Missing the required parameter 'documentId' when calling sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/sandbox/usercollection/workout/{document_id}"
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
