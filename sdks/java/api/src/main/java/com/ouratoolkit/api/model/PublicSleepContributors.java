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


package com.ouratoolkit.api.model;

import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;
import java.util.StringJoiner;
import java.util.Objects;
import java.util.Map;
import java.util.HashMap;
import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonCreator;
import com.fasterxml.jackson.annotation.JsonTypeName;
import com.fasterxml.jackson.annotation.JsonValue;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Object defining sleep score contributors.
 */
@JsonPropertyOrder({
  PublicSleepContributors.JSON_PROPERTY_DEEP_SLEEP,
  PublicSleepContributors.JSON_PROPERTY_EFFICIENCY,
  PublicSleepContributors.JSON_PROPERTY_LATENCY,
  PublicSleepContributors.JSON_PROPERTY_REM_SLEEP,
  PublicSleepContributors.JSON_PROPERTY_RESTFULNESS,
  PublicSleepContributors.JSON_PROPERTY_TIMING,
  PublicSleepContributors.JSON_PROPERTY_TOTAL_SLEEP
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicSleepContributors {
  public static final String JSON_PROPERTY_DEEP_SLEEP = "deep_sleep";
  private JsonNullable<Integer> deepSleep = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_EFFICIENCY = "efficiency";
  private JsonNullable<Integer> efficiency = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_LATENCY = "latency";
  private JsonNullable<Integer> latency = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_REM_SLEEP = "rem_sleep";
  private JsonNullable<Integer> remSleep = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_RESTFULNESS = "restfulness";
  private JsonNullable<Integer> restfulness = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_TIMING = "timing";
  private JsonNullable<Integer> timing = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_TOTAL_SLEEP = "total_sleep";
  private JsonNullable<Integer> totalSleep = JsonNullable.<Integer>undefined();

  public PublicSleepContributors() { 
  }

  public PublicSleepContributors deepSleep(@javax.annotation.Nullable Integer deepSleep) {
    this.deepSleep = JsonNullable.<Integer>of(deepSleep);
    return this;
  }

  /**
   * Get deepSleep
   * @return deepSleep
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getDeepSleep() {
        return deepSleep.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_DEEP_SLEEP)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getDeepSleep_JsonNullable() {
    return deepSleep;
  }
  
  @JsonProperty(JSON_PROPERTY_DEEP_SLEEP)
  public void setDeepSleep_JsonNullable(JsonNullable<Integer> deepSleep) {
    this.deepSleep = deepSleep;
  }

  public void setDeepSleep(@javax.annotation.Nullable Integer deepSleep) {
    this.deepSleep = JsonNullable.<Integer>of(deepSleep);
  }


  public PublicSleepContributors efficiency(@javax.annotation.Nullable Integer efficiency) {
    this.efficiency = JsonNullable.<Integer>of(efficiency);
    return this;
  }

  /**
   * Get efficiency
   * @return efficiency
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getEfficiency() {
        return efficiency.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_EFFICIENCY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getEfficiency_JsonNullable() {
    return efficiency;
  }
  
  @JsonProperty(JSON_PROPERTY_EFFICIENCY)
  public void setEfficiency_JsonNullable(JsonNullable<Integer> efficiency) {
    this.efficiency = efficiency;
  }

  public void setEfficiency(@javax.annotation.Nullable Integer efficiency) {
    this.efficiency = JsonNullable.<Integer>of(efficiency);
  }


  public PublicSleepContributors latency(@javax.annotation.Nullable Integer latency) {
    this.latency = JsonNullable.<Integer>of(latency);
    return this;
  }

  /**
   * Get latency
   * @return latency
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getLatency() {
        return latency.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_LATENCY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getLatency_JsonNullable() {
    return latency;
  }
  
  @JsonProperty(JSON_PROPERTY_LATENCY)
  public void setLatency_JsonNullable(JsonNullable<Integer> latency) {
    this.latency = latency;
  }

  public void setLatency(@javax.annotation.Nullable Integer latency) {
    this.latency = JsonNullable.<Integer>of(latency);
  }


  public PublicSleepContributors remSleep(@javax.annotation.Nullable Integer remSleep) {
    this.remSleep = JsonNullable.<Integer>of(remSleep);
    return this;
  }

  /**
   * Get remSleep
   * @return remSleep
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getRemSleep() {
        return remSleep.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_REM_SLEEP)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getRemSleep_JsonNullable() {
    return remSleep;
  }
  
  @JsonProperty(JSON_PROPERTY_REM_SLEEP)
  public void setRemSleep_JsonNullable(JsonNullable<Integer> remSleep) {
    this.remSleep = remSleep;
  }

  public void setRemSleep(@javax.annotation.Nullable Integer remSleep) {
    this.remSleep = JsonNullable.<Integer>of(remSleep);
  }


  public PublicSleepContributors restfulness(@javax.annotation.Nullable Integer restfulness) {
    this.restfulness = JsonNullable.<Integer>of(restfulness);
    return this;
  }

  /**
   * Get restfulness
   * @return restfulness
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getRestfulness() {
        return restfulness.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_RESTFULNESS)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getRestfulness_JsonNullable() {
    return restfulness;
  }
  
  @JsonProperty(JSON_PROPERTY_RESTFULNESS)
  public void setRestfulness_JsonNullable(JsonNullable<Integer> restfulness) {
    this.restfulness = restfulness;
  }

  public void setRestfulness(@javax.annotation.Nullable Integer restfulness) {
    this.restfulness = JsonNullable.<Integer>of(restfulness);
  }


  public PublicSleepContributors timing(@javax.annotation.Nullable Integer timing) {
    this.timing = JsonNullable.<Integer>of(timing);
    return this;
  }

  /**
   * Get timing
   * @return timing
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getTiming() {
        return timing.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TIMING)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getTiming_JsonNullable() {
    return timing;
  }
  
  @JsonProperty(JSON_PROPERTY_TIMING)
  public void setTiming_JsonNullable(JsonNullable<Integer> timing) {
    this.timing = timing;
  }

  public void setTiming(@javax.annotation.Nullable Integer timing) {
    this.timing = JsonNullable.<Integer>of(timing);
  }


  public PublicSleepContributors totalSleep(@javax.annotation.Nullable Integer totalSleep) {
    this.totalSleep = JsonNullable.<Integer>of(totalSleep);
    return this;
  }

  /**
   * Get totalSleep
   * @return totalSleep
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getTotalSleep() {
        return totalSleep.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TOTAL_SLEEP)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getTotalSleep_JsonNullable() {
    return totalSleep;
  }
  
  @JsonProperty(JSON_PROPERTY_TOTAL_SLEEP)
  public void setTotalSleep_JsonNullable(JsonNullable<Integer> totalSleep) {
    this.totalSleep = totalSleep;
  }

  public void setTotalSleep(@javax.annotation.Nullable Integer totalSleep) {
    this.totalSleep = JsonNullable.<Integer>of(totalSleep);
  }


  /**
   * Return true if this PublicSleepContributors object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicSleepContributors publicSleepContributors = (PublicSleepContributors) o;
    return equalsNullable(this.deepSleep, publicSleepContributors.deepSleep) &&
        equalsNullable(this.efficiency, publicSleepContributors.efficiency) &&
        equalsNullable(this.latency, publicSleepContributors.latency) &&
        equalsNullable(this.remSleep, publicSleepContributors.remSleep) &&
        equalsNullable(this.restfulness, publicSleepContributors.restfulness) &&
        equalsNullable(this.timing, publicSleepContributors.timing) &&
        equalsNullable(this.totalSleep, publicSleepContributors.totalSleep);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(hashCodeNullable(deepSleep), hashCodeNullable(efficiency), hashCodeNullable(latency), hashCodeNullable(remSleep), hashCodeNullable(restfulness), hashCodeNullable(timing), hashCodeNullable(totalSleep));
  }

  private static <T> int hashCodeNullable(JsonNullable<T> a) {
    if (a == null) {
      return 1;
    }
    return a.isPresent() ? Arrays.deepHashCode(new Object[]{a.get()}) : 31;
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class PublicSleepContributors {\n");
    sb.append("    deepSleep: ").append(toIndentedString(deepSleep)).append("\n");
    sb.append("    efficiency: ").append(toIndentedString(efficiency)).append("\n");
    sb.append("    latency: ").append(toIndentedString(latency)).append("\n");
    sb.append("    remSleep: ").append(toIndentedString(remSleep)).append("\n");
    sb.append("    restfulness: ").append(toIndentedString(restfulness)).append("\n");
    sb.append("    timing: ").append(toIndentedString(timing)).append("\n");
    sb.append("    totalSleep: ").append(toIndentedString(totalSleep)).append("\n");
    sb.append("}");
    return sb.toString();
  }

  /**
   * Convert the given object to string with each line indented by 4 spaces
   * (except the first line).
   */
  private String toIndentedString(Object o) {
    if (o == null) {
      return "null";
    }
    return o.toString().replace("\n", "\n    ");
  }

  /**
   * Convert the instance into URL query string.
   *
   * @return URL query string
   */
  public String toUrlQueryString() {
    return toUrlQueryString(null);
  }

  /**
   * Convert the instance into URL query string.
   *
   * @param prefix prefix of the query string
   * @return URL query string
   */
  public String toUrlQueryString(String prefix) {
    String suffix = "";
    String containerSuffix = "";
    String containerPrefix = "";
    if (prefix == null) {
      // style=form, explode=true, e.g. /pet?name=cat&type=manx
      prefix = "";
    } else {
      // deepObject style e.g. /pet?id[name]=cat&id[type]=manx
      prefix = prefix + "[";
      suffix = "]";
      containerSuffix = "]";
      containerPrefix = "[";
    }

    StringJoiner joiner = new StringJoiner("&");

    // add `deep_sleep` to the URL query string
    if (getDeepSleep() != null) {
      joiner.add(String.format("%sdeep_sleep%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDeepSleep()))));
    }

    // add `efficiency` to the URL query string
    if (getEfficiency() != null) {
      joiner.add(String.format("%sefficiency%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEfficiency()))));
    }

    // add `latency` to the URL query string
    if (getLatency() != null) {
      joiner.add(String.format("%slatency%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLatency()))));
    }

    // add `rem_sleep` to the URL query string
    if (getRemSleep() != null) {
      joiner.add(String.format("%srem_sleep%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRemSleep()))));
    }

    // add `restfulness` to the URL query string
    if (getRestfulness() != null) {
      joiner.add(String.format("%srestfulness%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRestfulness()))));
    }

    // add `timing` to the URL query string
    if (getTiming() != null) {
      joiner.add(String.format("%stiming%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTiming()))));
    }

    // add `total_sleep` to the URL query string
    if (getTotalSleep() != null) {
      joiner.add(String.format("%stotal_sleep%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTotalSleep()))));
    }

    return joiner.toString();
  }
}

