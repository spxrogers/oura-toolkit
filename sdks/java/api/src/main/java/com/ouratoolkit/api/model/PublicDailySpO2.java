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
import com.ouratoolkit.api.model.PublicSpo2AggregatedValues;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Daily SpO2 (Oxygen saturation).
 */
@JsonPropertyOrder({
  PublicDailySpO2.JSON_PROPERTY_ID,
  PublicDailySpO2.JSON_PROPERTY_BREATHING_DISTURBANCE_INDEX,
  PublicDailySpO2.JSON_PROPERTY_DAY,
  PublicDailySpO2.JSON_PROPERTY_SPO2_PERCENTAGE
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicDailySpO2 {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_BREATHING_DISTURBANCE_INDEX = "breathing_disturbance_index";
  private JsonNullable<Integer> breathingDisturbanceIndex = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_DAY = "day";
  @javax.annotation.Nullable
  private String day;

  public static final String JSON_PROPERTY_SPO2_PERCENTAGE = "spo2_percentage";
  private JsonNullable<PublicSpo2AggregatedValues> spo2Percentage = JsonNullable.<PublicSpo2AggregatedValues>undefined();

  public PublicDailySpO2() { 
  }

  public PublicDailySpO2 id(@javax.annotation.Nonnull String id) {
    this.id = id;
    return this;
  }

  /**
   * Unique identifier of the object.
   * @return id
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_ID)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getId() {
    return id;
  }


  @JsonProperty(JSON_PROPERTY_ID)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setId(@javax.annotation.Nonnull String id) {
    this.id = id;
  }


  public PublicDailySpO2 breathingDisturbanceIndex(@javax.annotation.Nullable Integer breathingDisturbanceIndex) {
    this.breathingDisturbanceIndex = JsonNullable.<Integer>of(breathingDisturbanceIndex);
    return this;
  }

  /**
   * Get breathingDisturbanceIndex
   * @return breathingDisturbanceIndex
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getBreathingDisturbanceIndex() {
        return breathingDisturbanceIndex.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_BREATHING_DISTURBANCE_INDEX)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getBreathingDisturbanceIndex_JsonNullable() {
    return breathingDisturbanceIndex;
  }
  
  @JsonProperty(JSON_PROPERTY_BREATHING_DISTURBANCE_INDEX)
  public void setBreathingDisturbanceIndex_JsonNullable(JsonNullable<Integer> breathingDisturbanceIndex) {
    this.breathingDisturbanceIndex = breathingDisturbanceIndex;
  }

  public void setBreathingDisturbanceIndex(@javax.annotation.Nullable Integer breathingDisturbanceIndex) {
    this.breathingDisturbanceIndex = JsonNullable.<Integer>of(breathingDisturbanceIndex);
  }


  public PublicDailySpO2 day(@javax.annotation.Nullable String day) {
    this.day = day;
    return this;
  }

  /**
   * Get day
   * @return day
   */
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_DAY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getDay() {
    return day;
  }


  @JsonProperty(JSON_PROPERTY_DAY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setDay(@javax.annotation.Nullable String day) {
    this.day = day;
  }


  public PublicDailySpO2 spo2Percentage(@javax.annotation.Nullable PublicSpo2AggregatedValues spo2Percentage) {
    this.spo2Percentage = JsonNullable.<PublicSpo2AggregatedValues>of(spo2Percentage);
    return this;
  }

  /**
   * Get spo2Percentage
   * @return spo2Percentage
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSpo2AggregatedValues getSpo2Percentage() {
        return spo2Percentage.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SPO2_PERCENTAGE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSpo2AggregatedValues> getSpo2Percentage_JsonNullable() {
    return spo2Percentage;
  }
  
  @JsonProperty(JSON_PROPERTY_SPO2_PERCENTAGE)
  public void setSpo2Percentage_JsonNullable(JsonNullable<PublicSpo2AggregatedValues> spo2Percentage) {
    this.spo2Percentage = spo2Percentage;
  }

  public void setSpo2Percentage(@javax.annotation.Nullable PublicSpo2AggregatedValues spo2Percentage) {
    this.spo2Percentage = JsonNullable.<PublicSpo2AggregatedValues>of(spo2Percentage);
  }


  /**
   * Return true if this PublicDailySpO2 object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicDailySpO2 publicDailySpO2 = (PublicDailySpO2) o;
    return Objects.equals(this.id, publicDailySpO2.id) &&
        equalsNullable(this.breathingDisturbanceIndex, publicDailySpO2.breathingDisturbanceIndex) &&
        Objects.equals(this.day, publicDailySpO2.day) &&
        equalsNullable(this.spo2Percentage, publicDailySpO2.spo2Percentage);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, hashCodeNullable(breathingDisturbanceIndex), day, hashCodeNullable(spo2Percentage));
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
    sb.append("class PublicDailySpO2 {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    breathingDisturbanceIndex: ").append(toIndentedString(breathingDisturbanceIndex)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    spo2Percentage: ").append(toIndentedString(spo2Percentage)).append("\n");
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

    // add `id` to the URL query string
    if (getId() != null) {
      joiner.add(String.format("%sid%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getId()))));
    }

    // add `breathing_disturbance_index` to the URL query string
    if (getBreathingDisturbanceIndex() != null) {
      joiner.add(String.format("%sbreathing_disturbance_index%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getBreathingDisturbanceIndex()))));
    }

    // add `day` to the URL query string
    if (getDay() != null) {
      joiner.add(String.format("%sday%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDay()))));
    }

    // add `spo2_percentage` to the URL query string
    if (getSpo2Percentage() != null) {
      joiner.add(getSpo2Percentage().toUrlQueryString(prefix + "spo2_percentage" + suffix));
    }

    return joiner.toString();
  }
}

