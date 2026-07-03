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

import com.ouratoolkit.api.model.CreateWebhookSubscriptionRequest;
import com.ouratoolkit.api.model.HTTPValidationError;
import com.ouratoolkit.api.model.UpdateWebhookSubscriptionRequest;
import com.ouratoolkit.api.model.WebhookSubscriptionModel;

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
public class WebhookSubscriptionRoutesApi {
  private final HttpClient memberVarHttpClient;
  private final ObjectMapper memberVarObjectMapper;
  private final String memberVarBaseUri;
  private final Consumer<HttpRequest.Builder> memberVarInterceptor;
  private final Duration memberVarReadTimeout;
  private final Consumer<HttpResponse<InputStream>> memberVarResponseInterceptor;
  private final Consumer<HttpResponse<String>> memberVarAsyncResponseInterceptor;

  public WebhookSubscriptionRoutesApi() {
    this(Configuration.getDefaultApiClient());
  }

  public WebhookSubscriptionRoutesApi(ApiClient apiClient) {
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
   * Create Webhook Subscription
   * 
   * @param createWebhookSubscriptionRequest  (required)
   * @return WebhookSubscriptionModel
   * @throws ApiException if fails to make API call
   */
  public WebhookSubscriptionModel createWebhookSubscriptionV2WebhookSubscriptionPost(@javax.annotation.Nonnull CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest) throws ApiException {
    ApiResponse<WebhookSubscriptionModel> localVarResponse = createWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfo(createWebhookSubscriptionRequest);
    return localVarResponse.getData();
  }

  /**
   * Create Webhook Subscription
   * 
   * @param createWebhookSubscriptionRequest  (required)
   * @return ApiResponse&lt;WebhookSubscriptionModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<WebhookSubscriptionModel> createWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfo(@javax.annotation.Nonnull CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = createWebhookSubscriptionV2WebhookSubscriptionPostRequestBuilder(createWebhookSubscriptionRequest);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("createWebhookSubscriptionV2WebhookSubscriptionPost", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<WebhookSubscriptionModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<WebhookSubscriptionModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<WebhookSubscriptionModel>() {})
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

  private HttpRequest.Builder createWebhookSubscriptionV2WebhookSubscriptionPostRequestBuilder(@javax.annotation.Nonnull CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest) throws ApiException {
    // verify the required parameter 'createWebhookSubscriptionRequest' is set
    if (createWebhookSubscriptionRequest == null) {
      throw new ApiException(400, "Missing the required parameter 'createWebhookSubscriptionRequest' when calling createWebhookSubscriptionV2WebhookSubscriptionPost");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/webhook/subscription";

    localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

    localVarRequestBuilder.header("Content-Type", "application/json");
    localVarRequestBuilder.header("Accept", "application/json");

    try {
      byte[] localVarPostBody = memberVarObjectMapper.writeValueAsBytes(createWebhookSubscriptionRequest);
      localVarRequestBuilder.method("POST", HttpRequest.BodyPublishers.ofByteArray(localVarPostBody));
    } catch (IOException e) {
      throw new ApiException(e);
    }
    if (memberVarReadTimeout != null) {
      localVarRequestBuilder.timeout(memberVarReadTimeout);
    }
    if (memberVarInterceptor != null) {
      memberVarInterceptor.accept(localVarRequestBuilder);
    }
    return localVarRequestBuilder;
  }

  /**
   * Delete Webhook Subscription
   * 
   * @param id  (required)
   * @throws ApiException if fails to make API call
   */
  public void deleteWebhookSubscriptionV2WebhookSubscriptionIdDelete(@javax.annotation.Nonnull String id) throws ApiException {
    deleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfo(id);
  }

  /**
   * Delete Webhook Subscription
   * 
   * @param id  (required)
   * @return ApiResponse&lt;Void&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<Void> deleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfo(@javax.annotation.Nonnull String id) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = deleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteRequestBuilder(id);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("deleteWebhookSubscriptionV2WebhookSubscriptionIdDelete", localVarResponse);
        }
        return new ApiResponse<>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            null
        );
      } finally {
        // Drain the InputStream
        while (localVarResponse.body().read() != -1) {
          // Ignore
        }
        localVarResponse.body().close();
      }
    } catch (IOException e) {
      throw new ApiException(e);
    }
    catch (InterruptedException e) {
      Thread.currentThread().interrupt();
      throw new ApiException(e);
    }
  }

  private HttpRequest.Builder deleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteRequestBuilder(@javax.annotation.Nonnull String id) throws ApiException {
    // verify the required parameter 'id' is set
    if (id == null) {
      throw new ApiException(400, "Missing the required parameter 'id' when calling deleteWebhookSubscriptionV2WebhookSubscriptionIdDelete");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/webhook/subscription/{id}"
        .replace("{id}", ApiClient.urlEncode(id.toString()));

    localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

    localVarRequestBuilder.header("Accept", "application/json");

    localVarRequestBuilder.method("DELETE", HttpRequest.BodyPublishers.noBody());
    if (memberVarReadTimeout != null) {
      localVarRequestBuilder.timeout(memberVarReadTimeout);
    }
    if (memberVarInterceptor != null) {
      memberVarInterceptor.accept(localVarRequestBuilder);
    }
    return localVarRequestBuilder;
  }

  /**
   * Get Webhook Subscription
   * 
   * @param id  (required)
   * @return WebhookSubscriptionModel
   * @throws ApiException if fails to make API call
   */
  public WebhookSubscriptionModel getWebhookSubscriptionV2WebhookSubscriptionIdGet(@javax.annotation.Nonnull String id) throws ApiException {
    ApiResponse<WebhookSubscriptionModel> localVarResponse = getWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfo(id);
    return localVarResponse.getData();
  }

  /**
   * Get Webhook Subscription
   * 
   * @param id  (required)
   * @return ApiResponse&lt;WebhookSubscriptionModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<WebhookSubscriptionModel> getWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfo(@javax.annotation.Nonnull String id) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = getWebhookSubscriptionV2WebhookSubscriptionIdGetRequestBuilder(id);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("getWebhookSubscriptionV2WebhookSubscriptionIdGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<WebhookSubscriptionModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<WebhookSubscriptionModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<WebhookSubscriptionModel>() {})
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

  private HttpRequest.Builder getWebhookSubscriptionV2WebhookSubscriptionIdGetRequestBuilder(@javax.annotation.Nonnull String id) throws ApiException {
    // verify the required parameter 'id' is set
    if (id == null) {
      throw new ApiException(400, "Missing the required parameter 'id' when calling getWebhookSubscriptionV2WebhookSubscriptionIdGet");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/webhook/subscription/{id}"
        .replace("{id}", ApiClient.urlEncode(id.toString()));

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
   * List Webhook Subscriptions
   * 
   * @return List&lt;WebhookSubscriptionModel&gt;
   * @throws ApiException if fails to make API call
   */
  public List<WebhookSubscriptionModel> listWebhookSubscriptionsV2WebhookSubscriptionGet() throws ApiException {
    ApiResponse<List<WebhookSubscriptionModel>> localVarResponse = listWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfo();
    return localVarResponse.getData();
  }

  /**
   * List Webhook Subscriptions
   * 
   * @return ApiResponse&lt;List&lt;WebhookSubscriptionModel&gt;&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<List<WebhookSubscriptionModel>> listWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfo() throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = listWebhookSubscriptionsV2WebhookSubscriptionGetRequestBuilder();
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("listWebhookSubscriptionsV2WebhookSubscriptionGet", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<List<WebhookSubscriptionModel>>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<List<WebhookSubscriptionModel>>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<List<WebhookSubscriptionModel>>() {})
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

  private HttpRequest.Builder listWebhookSubscriptionsV2WebhookSubscriptionGetRequestBuilder() throws ApiException {

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/webhook/subscription";

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
   * Renew Webhook Subscription
   * 
   * @param id  (required)
   * @return WebhookSubscriptionModel
   * @throws ApiException if fails to make API call
   */
  public WebhookSubscriptionModel renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut(@javax.annotation.Nonnull String id) throws ApiException {
    ApiResponse<WebhookSubscriptionModel> localVarResponse = renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfo(id);
    return localVarResponse.getData();
  }

  /**
   * Renew Webhook Subscription
   * 
   * @param id  (required)
   * @return ApiResponse&lt;WebhookSubscriptionModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<WebhookSubscriptionModel> renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfo(@javax.annotation.Nonnull String id) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutRequestBuilder(id);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<WebhookSubscriptionModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<WebhookSubscriptionModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<WebhookSubscriptionModel>() {})
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

  private HttpRequest.Builder renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutRequestBuilder(@javax.annotation.Nonnull String id) throws ApiException {
    // verify the required parameter 'id' is set
    if (id == null) {
      throw new ApiException(400, "Missing the required parameter 'id' when calling renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/webhook/subscription/renew/{id}"
        .replace("{id}", ApiClient.urlEncode(id.toString()));

    localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

    localVarRequestBuilder.header("Accept", "application/json");

    localVarRequestBuilder.method("PUT", HttpRequest.BodyPublishers.noBody());
    if (memberVarReadTimeout != null) {
      localVarRequestBuilder.timeout(memberVarReadTimeout);
    }
    if (memberVarInterceptor != null) {
      memberVarInterceptor.accept(localVarRequestBuilder);
    }
    return localVarRequestBuilder;
  }

  /**
   * Update Webhook Subscription
   * 
   * @param id  (required)
   * @param updateWebhookSubscriptionRequest  (required)
   * @return WebhookSubscriptionModel
   * @throws ApiException if fails to make API call
   */
  public WebhookSubscriptionModel updateWebhookSubscriptionV2WebhookSubscriptionIdPut(@javax.annotation.Nonnull String id, @javax.annotation.Nonnull UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest) throws ApiException {
    ApiResponse<WebhookSubscriptionModel> localVarResponse = updateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfo(id, updateWebhookSubscriptionRequest);
    return localVarResponse.getData();
  }

  /**
   * Update Webhook Subscription
   * 
   * @param id  (required)
   * @param updateWebhookSubscriptionRequest  (required)
   * @return ApiResponse&lt;WebhookSubscriptionModel&gt;
   * @throws ApiException if fails to make API call
   */
  public ApiResponse<WebhookSubscriptionModel> updateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfo(@javax.annotation.Nonnull String id, @javax.annotation.Nonnull UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest) throws ApiException {
    HttpRequest.Builder localVarRequestBuilder = updateWebhookSubscriptionV2WebhookSubscriptionIdPutRequestBuilder(id, updateWebhookSubscriptionRequest);
    try {
      HttpResponse<InputStream> localVarResponse = memberVarHttpClient.send(
          localVarRequestBuilder.build(),
          HttpResponse.BodyHandlers.ofInputStream());
      if (memberVarResponseInterceptor != null) {
        memberVarResponseInterceptor.accept(localVarResponse);
      }
      try {
        if (localVarResponse.statusCode()/ 100 != 2) {
          throw getApiException("updateWebhookSubscriptionV2WebhookSubscriptionIdPut", localVarResponse);
        }
        if (localVarResponse.body() == null) {
          return new ApiResponse<WebhookSubscriptionModel>(
              localVarResponse.statusCode(),
              localVarResponse.headers().map(),
              null
          );
        }

        String responseBody = new String(localVarResponse.body().readAllBytes());
        localVarResponse.body().close();

        return new ApiResponse<WebhookSubscriptionModel>(
            localVarResponse.statusCode(),
            localVarResponse.headers().map(),
            responseBody.isBlank()? null: memberVarObjectMapper.readValue(responseBody, new TypeReference<WebhookSubscriptionModel>() {})
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

  private HttpRequest.Builder updateWebhookSubscriptionV2WebhookSubscriptionIdPutRequestBuilder(@javax.annotation.Nonnull String id, @javax.annotation.Nonnull UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest) throws ApiException {
    // verify the required parameter 'id' is set
    if (id == null) {
      throw new ApiException(400, "Missing the required parameter 'id' when calling updateWebhookSubscriptionV2WebhookSubscriptionIdPut");
    }
    // verify the required parameter 'updateWebhookSubscriptionRequest' is set
    if (updateWebhookSubscriptionRequest == null) {
      throw new ApiException(400, "Missing the required parameter 'updateWebhookSubscriptionRequest' when calling updateWebhookSubscriptionV2WebhookSubscriptionIdPut");
    }

    HttpRequest.Builder localVarRequestBuilder = HttpRequest.newBuilder();

    String localVarPath = "/v2/webhook/subscription/{id}"
        .replace("{id}", ApiClient.urlEncode(id.toString()));

    localVarRequestBuilder.uri(URI.create(memberVarBaseUri + localVarPath));

    localVarRequestBuilder.header("Content-Type", "application/json");
    localVarRequestBuilder.header("Accept", "application/json");

    try {
      byte[] localVarPostBody = memberVarObjectMapper.writeValueAsBytes(updateWebhookSubscriptionRequest);
      localVarRequestBuilder.method("PUT", HttpRequest.BodyPublishers.ofByteArray(localVarPostBody));
    } catch (IOException e) {
      throw new ApiException(e);
    }
    if (memberVarReadTimeout != null) {
      localVarRequestBuilder.timeout(memberVarReadTimeout);
    }
    if (memberVarInterceptor != null) {
      memberVarInterceptor.accept(localVarRequestBuilder);
    }
    return localVarRequestBuilder;
  }

}
