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
 * Object defining activity score contributors.
 */
@JsonPropertyOrder({
  PublicActivityContributors.JSON_PROPERTY_MEET_DAILY_TARGETS,
  PublicActivityContributors.JSON_PROPERTY_MOVE_EVERY_HOUR,
  PublicActivityContributors.JSON_PROPERTY_RECOVERY_TIME,
  PublicActivityContributors.JSON_PROPERTY_STAY_ACTIVE,
  PublicActivityContributors.JSON_PROPERTY_TRAINING_FREQUENCY,
  PublicActivityContributors.JSON_PROPERTY_TRAINING_VOLUME
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicActivityContributors {
  public static final String JSON_PROPERTY_MEET_DAILY_TARGETS = "meet_daily_targets";
  private JsonNullable<Integer> meetDailyTargets = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_MOVE_EVERY_HOUR = "move_every_hour";
  private JsonNullable<Integer> moveEveryHour = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_RECOVERY_TIME = "recovery_time";
  private JsonNullable<Integer> recoveryTime = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_STAY_ACTIVE = "stay_active";
  private JsonNullable<Integer> stayActive = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_TRAINING_FREQUENCY = "training_frequency";
  private JsonNullable<Integer> trainingFrequency = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_TRAINING_VOLUME = "training_volume";
  private JsonNullable<Integer> trainingVolume = JsonNullable.<Integer>undefined();

  public PublicActivityContributors() { 
  }

  public PublicActivityContributors meetDailyTargets(@javax.annotation.Nullable Integer meetDailyTargets) {
    this.meetDailyTargets = JsonNullable.<Integer>of(meetDailyTargets);
    return this;
  }

  /**
   * Get meetDailyTargets
   * @return meetDailyTargets
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getMeetDailyTargets() {
        return meetDailyTargets.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_MEET_DAILY_TARGETS)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getMeetDailyTargets_JsonNullable() {
    return meetDailyTargets;
  }
  
  @JsonProperty(JSON_PROPERTY_MEET_DAILY_TARGETS)
  public void setMeetDailyTargets_JsonNullable(JsonNullable<Integer> meetDailyTargets) {
    this.meetDailyTargets = meetDailyTargets;
  }

  public void setMeetDailyTargets(@javax.annotation.Nullable Integer meetDailyTargets) {
    this.meetDailyTargets = JsonNullable.<Integer>of(meetDailyTargets);
  }


  public PublicActivityContributors moveEveryHour(@javax.annotation.Nullable Integer moveEveryHour) {
    this.moveEveryHour = JsonNullable.<Integer>of(moveEveryHour);
    return this;
  }

  /**
   * Get moveEveryHour
   * @return moveEveryHour
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getMoveEveryHour() {
        return moveEveryHour.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_MOVE_EVERY_HOUR)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getMoveEveryHour_JsonNullable() {
    return moveEveryHour;
  }
  
  @JsonProperty(JSON_PROPERTY_MOVE_EVERY_HOUR)
  public void setMoveEveryHour_JsonNullable(JsonNullable<Integer> moveEveryHour) {
    this.moveEveryHour = moveEveryHour;
  }

  public void setMoveEveryHour(@javax.annotation.Nullable Integer moveEveryHour) {
    this.moveEveryHour = JsonNullable.<Integer>of(moveEveryHour);
  }


  public PublicActivityContributors recoveryTime(@javax.annotation.Nullable Integer recoveryTime) {
    this.recoveryTime = JsonNullable.<Integer>of(recoveryTime);
    return this;
  }

  /**
   * Get recoveryTime
   * @return recoveryTime
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getRecoveryTime() {
        return recoveryTime.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_RECOVERY_TIME)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getRecoveryTime_JsonNullable() {
    return recoveryTime;
  }
  
  @JsonProperty(JSON_PROPERTY_RECOVERY_TIME)
  public void setRecoveryTime_JsonNullable(JsonNullable<Integer> recoveryTime) {
    this.recoveryTime = recoveryTime;
  }

  public void setRecoveryTime(@javax.annotation.Nullable Integer recoveryTime) {
    this.recoveryTime = JsonNullable.<Integer>of(recoveryTime);
  }


  public PublicActivityContributors stayActive(@javax.annotation.Nullable Integer stayActive) {
    this.stayActive = JsonNullable.<Integer>of(stayActive);
    return this;
  }

  /**
   * Get stayActive
   * @return stayActive
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getStayActive() {
        return stayActive.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_STAY_ACTIVE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getStayActive_JsonNullable() {
    return stayActive;
  }
  
  @JsonProperty(JSON_PROPERTY_STAY_ACTIVE)
  public void setStayActive_JsonNullable(JsonNullable<Integer> stayActive) {
    this.stayActive = stayActive;
  }

  public void setStayActive(@javax.annotation.Nullable Integer stayActive) {
    this.stayActive = JsonNullable.<Integer>of(stayActive);
  }


  public PublicActivityContributors trainingFrequency(@javax.annotation.Nullable Integer trainingFrequency) {
    this.trainingFrequency = JsonNullable.<Integer>of(trainingFrequency);
    return this;
  }

  /**
   * Get trainingFrequency
   * @return trainingFrequency
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getTrainingFrequency() {
        return trainingFrequency.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TRAINING_FREQUENCY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getTrainingFrequency_JsonNullable() {
    return trainingFrequency;
  }
  
  @JsonProperty(JSON_PROPERTY_TRAINING_FREQUENCY)
  public void setTrainingFrequency_JsonNullable(JsonNullable<Integer> trainingFrequency) {
    this.trainingFrequency = trainingFrequency;
  }

  public void setTrainingFrequency(@javax.annotation.Nullable Integer trainingFrequency) {
    this.trainingFrequency = JsonNullable.<Integer>of(trainingFrequency);
  }


  public PublicActivityContributors trainingVolume(@javax.annotation.Nullable Integer trainingVolume) {
    this.trainingVolume = JsonNullable.<Integer>of(trainingVolume);
    return this;
  }

  /**
   * Get trainingVolume
   * @return trainingVolume
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getTrainingVolume() {
        return trainingVolume.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TRAINING_VOLUME)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getTrainingVolume_JsonNullable() {
    return trainingVolume;
  }
  
  @JsonProperty(JSON_PROPERTY_TRAINING_VOLUME)
  public void setTrainingVolume_JsonNullable(JsonNullable<Integer> trainingVolume) {
    this.trainingVolume = trainingVolume;
  }

  public void setTrainingVolume(@javax.annotation.Nullable Integer trainingVolume) {
    this.trainingVolume = JsonNullable.<Integer>of(trainingVolume);
  }


  /**
   * Return true if this PublicActivityContributors object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicActivityContributors publicActivityContributors = (PublicActivityContributors) o;
    return equalsNullable(this.meetDailyTargets, publicActivityContributors.meetDailyTargets) &&
        equalsNullable(this.moveEveryHour, publicActivityContributors.moveEveryHour) &&
        equalsNullable(this.recoveryTime, publicActivityContributors.recoveryTime) &&
        equalsNullable(this.stayActive, publicActivityContributors.stayActive) &&
        equalsNullable(this.trainingFrequency, publicActivityContributors.trainingFrequency) &&
        equalsNullable(this.trainingVolume, publicActivityContributors.trainingVolume);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(hashCodeNullable(meetDailyTargets), hashCodeNullable(moveEveryHour), hashCodeNullable(recoveryTime), hashCodeNullable(stayActive), hashCodeNullable(trainingFrequency), hashCodeNullable(trainingVolume));
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
    sb.append("class PublicActivityContributors {\n");
    sb.append("    meetDailyTargets: ").append(toIndentedString(meetDailyTargets)).append("\n");
    sb.append("    moveEveryHour: ").append(toIndentedString(moveEveryHour)).append("\n");
    sb.append("    recoveryTime: ").append(toIndentedString(recoveryTime)).append("\n");
    sb.append("    stayActive: ").append(toIndentedString(stayActive)).append("\n");
    sb.append("    trainingFrequency: ").append(toIndentedString(trainingFrequency)).append("\n");
    sb.append("    trainingVolume: ").append(toIndentedString(trainingVolume)).append("\n");
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

    // add `meet_daily_targets` to the URL query string
    if (getMeetDailyTargets() != null) {
      joiner.add(String.format("%smeet_daily_targets%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getMeetDailyTargets()))));
    }

    // add `move_every_hour` to the URL query string
    if (getMoveEveryHour() != null) {
      joiner.add(String.format("%smove_every_hour%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getMoveEveryHour()))));
    }

    // add `recovery_time` to the URL query string
    if (getRecoveryTime() != null) {
      joiner.add(String.format("%srecovery_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRecoveryTime()))));
    }

    // add `stay_active` to the URL query string
    if (getStayActive() != null) {
      joiner.add(String.format("%sstay_active%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getStayActive()))));
    }

    // add `training_frequency` to the URL query string
    if (getTrainingFrequency() != null) {
      joiner.add(String.format("%straining_frequency%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTrainingFrequency()))));
    }

    // add `training_volume` to the URL query string
    if (getTrainingVolume() != null) {
      joiner.add(String.format("%straining_volume%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTrainingVolume()))));
    }

    return joiner.toString();
  }
}

