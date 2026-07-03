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
import com.ouratoolkit.api.model.PublicMomentMood;
import com.ouratoolkit.api.model.PublicMomentType;
import com.ouratoolkit.api.model.PublicSample;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Public model defining a recorded Session.
 */
@JsonPropertyOrder({
  PublicSession.JSON_PROPERTY_ID,
  PublicSession.JSON_PROPERTY_DAY,
  PublicSession.JSON_PROPERTY_END_DATETIME,
  PublicSession.JSON_PROPERTY_HEART_RATE,
  PublicSession.JSON_PROPERTY_HEART_RATE_VARIABILITY,
  PublicSession.JSON_PROPERTY_MOOD,
  PublicSession.JSON_PROPERTY_MOTION_COUNT,
  PublicSession.JSON_PROPERTY_START_DATETIME,
  PublicSession.JSON_PROPERTY_TYPE
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicSession {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_DAY = "day";
  @javax.annotation.Nullable
  private String day;

  public static final String JSON_PROPERTY_END_DATETIME = "end_datetime";
  @javax.annotation.Nullable
  private String endDatetime;

  public static final String JSON_PROPERTY_HEART_RATE = "heart_rate";
  private JsonNullable<PublicSample> heartRate = JsonNullable.<PublicSample>undefined();

  public static final String JSON_PROPERTY_HEART_RATE_VARIABILITY = "heart_rate_variability";
  private JsonNullable<PublicSample> heartRateVariability = JsonNullable.<PublicSample>undefined();

  public static final String JSON_PROPERTY_MOOD = "mood";
  private JsonNullable<PublicMomentMood> mood = JsonNullable.<PublicMomentMood>undefined();

  public static final String JSON_PROPERTY_MOTION_COUNT = "motion_count";
  private JsonNullable<PublicSample> motionCount = JsonNullable.<PublicSample>undefined();

  public static final String JSON_PROPERTY_START_DATETIME = "start_datetime";
  @javax.annotation.Nullable
  private String startDatetime;

  public static final String JSON_PROPERTY_TYPE = "type";
  @javax.annotation.Nonnull
  private PublicMomentType type;

  public PublicSession() { 
  }

  public PublicSession id(@javax.annotation.Nonnull String id) {
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


  public PublicSession day(@javax.annotation.Nullable String day) {
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


  public PublicSession endDatetime(@javax.annotation.Nullable String endDatetime) {
    this.endDatetime = endDatetime;
    return this;
  }

  /**
   * Get endDatetime
   * @return endDatetime
   */
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_END_DATETIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getEndDatetime() {
    return endDatetime;
  }


  @JsonProperty(JSON_PROPERTY_END_DATETIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setEndDatetime(@javax.annotation.Nullable String endDatetime) {
    this.endDatetime = endDatetime;
  }


  public PublicSession heartRate(@javax.annotation.Nullable PublicSample heartRate) {
    this.heartRate = JsonNullable.<PublicSample>of(heartRate);
    return this;
  }

  /**
   * Get heartRate
   * @return heartRate
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSample getHeartRate() {
        return heartRate.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_HEART_RATE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSample> getHeartRate_JsonNullable() {
    return heartRate;
  }
  
  @JsonProperty(JSON_PROPERTY_HEART_RATE)
  public void setHeartRate_JsonNullable(JsonNullable<PublicSample> heartRate) {
    this.heartRate = heartRate;
  }

  public void setHeartRate(@javax.annotation.Nullable PublicSample heartRate) {
    this.heartRate = JsonNullable.<PublicSample>of(heartRate);
  }


  public PublicSession heartRateVariability(@javax.annotation.Nullable PublicSample heartRateVariability) {
    this.heartRateVariability = JsonNullable.<PublicSample>of(heartRateVariability);
    return this;
  }

  /**
   * Get heartRateVariability
   * @return heartRateVariability
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSample getHeartRateVariability() {
        return heartRateVariability.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_HEART_RATE_VARIABILITY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSample> getHeartRateVariability_JsonNullable() {
    return heartRateVariability;
  }
  
  @JsonProperty(JSON_PROPERTY_HEART_RATE_VARIABILITY)
  public void setHeartRateVariability_JsonNullable(JsonNullable<PublicSample> heartRateVariability) {
    this.heartRateVariability = heartRateVariability;
  }

  public void setHeartRateVariability(@javax.annotation.Nullable PublicSample heartRateVariability) {
    this.heartRateVariability = JsonNullable.<PublicSample>of(heartRateVariability);
  }


  public PublicSession mood(@javax.annotation.Nullable PublicMomentMood mood) {
    this.mood = JsonNullable.<PublicMomentMood>of(mood);
    return this;
  }

  /**
   * Get mood
   * @return mood
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicMomentMood getMood() {
        return mood.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_MOOD)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicMomentMood> getMood_JsonNullable() {
    return mood;
  }
  
  @JsonProperty(JSON_PROPERTY_MOOD)
  public void setMood_JsonNullable(JsonNullable<PublicMomentMood> mood) {
    this.mood = mood;
  }

  public void setMood(@javax.annotation.Nullable PublicMomentMood mood) {
    this.mood = JsonNullable.<PublicMomentMood>of(mood);
  }


  public PublicSession motionCount(@javax.annotation.Nullable PublicSample motionCount) {
    this.motionCount = JsonNullable.<PublicSample>of(motionCount);
    return this;
  }

  /**
   * Get motionCount
   * @return motionCount
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSample getMotionCount() {
        return motionCount.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_MOTION_COUNT)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSample> getMotionCount_JsonNullable() {
    return motionCount;
  }
  
  @JsonProperty(JSON_PROPERTY_MOTION_COUNT)
  public void setMotionCount_JsonNullable(JsonNullable<PublicSample> motionCount) {
    this.motionCount = motionCount;
  }

  public void setMotionCount(@javax.annotation.Nullable PublicSample motionCount) {
    this.motionCount = JsonNullable.<PublicSample>of(motionCount);
  }


  public PublicSession startDatetime(@javax.annotation.Nullable String startDatetime) {
    this.startDatetime = startDatetime;
    return this;
  }

  /**
   * Get startDatetime
   * @return startDatetime
   */
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_START_DATETIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getStartDatetime() {
    return startDatetime;
  }


  @JsonProperty(JSON_PROPERTY_START_DATETIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setStartDatetime(@javax.annotation.Nullable String startDatetime) {
    this.startDatetime = startDatetime;
  }


  public PublicSession type(@javax.annotation.Nonnull PublicMomentType type) {
    this.type = type;
    return this;
  }

  /**
   * Type of the Moment.
   * @return type
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_TYPE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public PublicMomentType getType() {
    return type;
  }


  @JsonProperty(JSON_PROPERTY_TYPE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setType(@javax.annotation.Nonnull PublicMomentType type) {
    this.type = type;
  }


  /**
   * Return true if this PublicSession object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicSession publicSession = (PublicSession) o;
    return Objects.equals(this.id, publicSession.id) &&
        Objects.equals(this.day, publicSession.day) &&
        Objects.equals(this.endDatetime, publicSession.endDatetime) &&
        equalsNullable(this.heartRate, publicSession.heartRate) &&
        equalsNullable(this.heartRateVariability, publicSession.heartRateVariability) &&
        equalsNullable(this.mood, publicSession.mood) &&
        equalsNullable(this.motionCount, publicSession.motionCount) &&
        Objects.equals(this.startDatetime, publicSession.startDatetime) &&
        Objects.equals(this.type, publicSession.type);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, day, endDatetime, hashCodeNullable(heartRate), hashCodeNullable(heartRateVariability), hashCodeNullable(mood), hashCodeNullable(motionCount), startDatetime, type);
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
    sb.append("class PublicSession {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    endDatetime: ").append(toIndentedString(endDatetime)).append("\n");
    sb.append("    heartRate: ").append(toIndentedString(heartRate)).append("\n");
    sb.append("    heartRateVariability: ").append(toIndentedString(heartRateVariability)).append("\n");
    sb.append("    mood: ").append(toIndentedString(mood)).append("\n");
    sb.append("    motionCount: ").append(toIndentedString(motionCount)).append("\n");
    sb.append("    startDatetime: ").append(toIndentedString(startDatetime)).append("\n");
    sb.append("    type: ").append(toIndentedString(type)).append("\n");
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

    // add `day` to the URL query string
    if (getDay() != null) {
      joiner.add(String.format("%sday%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDay()))));
    }

    // add `end_datetime` to the URL query string
    if (getEndDatetime() != null) {
      joiner.add(String.format("%send_datetime%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEndDatetime()))));
    }

    // add `heart_rate` to the URL query string
    if (getHeartRate() != null) {
      joiner.add(getHeartRate().toUrlQueryString(prefix + "heart_rate" + suffix));
    }

    // add `heart_rate_variability` to the URL query string
    if (getHeartRateVariability() != null) {
      joiner.add(getHeartRateVariability().toUrlQueryString(prefix + "heart_rate_variability" + suffix));
    }

    // add `mood` to the URL query string
    if (getMood() != null) {
      joiner.add(String.format("%smood%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getMood()))));
    }

    // add `motion_count` to the URL query string
    if (getMotionCount() != null) {
      joiner.add(getMotionCount().toUrlQueryString(prefix + "motion_count" + suffix));
    }

    // add `start_datetime` to the URL query string
    if (getStartDatetime() != null) {
      joiner.add(String.format("%sstart_datetime%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getStartDatetime()))));
    }

    // add `type` to the URL query string
    if (getType() != null) {
      joiner.add(String.format("%stype%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getType()))));
    }

    return joiner.toString();
  }
}

