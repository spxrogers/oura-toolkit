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
 * Object defining readiness score contributors.
 */
@JsonPropertyOrder({
  PublicReadinessContributors.JSON_PROPERTY_ACTIVITY_BALANCE,
  PublicReadinessContributors.JSON_PROPERTY_BODY_TEMPERATURE,
  PublicReadinessContributors.JSON_PROPERTY_HRV_BALANCE,
  PublicReadinessContributors.JSON_PROPERTY_PREVIOUS_DAY_ACTIVITY,
  PublicReadinessContributors.JSON_PROPERTY_PREVIOUS_NIGHT,
  PublicReadinessContributors.JSON_PROPERTY_RECOVERY_INDEX,
  PublicReadinessContributors.JSON_PROPERTY_RESTING_HEART_RATE,
  PublicReadinessContributors.JSON_PROPERTY_SLEEP_BALANCE,
  PublicReadinessContributors.JSON_PROPERTY_SLEEP_REGULARITY
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicReadinessContributors {
  public static final String JSON_PROPERTY_ACTIVITY_BALANCE = "activity_balance";
  private JsonNullable<Integer> activityBalance = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_BODY_TEMPERATURE = "body_temperature";
  private JsonNullable<Integer> bodyTemperature = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_HRV_BALANCE = "hrv_balance";
  private JsonNullable<Integer> hrvBalance = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_PREVIOUS_DAY_ACTIVITY = "previous_day_activity";
  private JsonNullable<Integer> previousDayActivity = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_PREVIOUS_NIGHT = "previous_night";
  private JsonNullable<Integer> previousNight = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_RECOVERY_INDEX = "recovery_index";
  private JsonNullable<Integer> recoveryIndex = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_RESTING_HEART_RATE = "resting_heart_rate";
  private JsonNullable<Integer> restingHeartRate = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_SLEEP_BALANCE = "sleep_balance";
  private JsonNullable<Integer> sleepBalance = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_SLEEP_REGULARITY = "sleep_regularity";
  private JsonNullable<Integer> sleepRegularity = JsonNullable.<Integer>undefined();

  public PublicReadinessContributors() { 
  }

  public PublicReadinessContributors activityBalance(@javax.annotation.Nullable Integer activityBalance) {
    this.activityBalance = JsonNullable.<Integer>of(activityBalance);
    return this;
  }

  /**
   * Get activityBalance
   * @return activityBalance
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getActivityBalance() {
        return activityBalance.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_ACTIVITY_BALANCE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getActivityBalance_JsonNullable() {
    return activityBalance;
  }
  
  @JsonProperty(JSON_PROPERTY_ACTIVITY_BALANCE)
  public void setActivityBalance_JsonNullable(JsonNullable<Integer> activityBalance) {
    this.activityBalance = activityBalance;
  }

  public void setActivityBalance(@javax.annotation.Nullable Integer activityBalance) {
    this.activityBalance = JsonNullable.<Integer>of(activityBalance);
  }


  public PublicReadinessContributors bodyTemperature(@javax.annotation.Nullable Integer bodyTemperature) {
    this.bodyTemperature = JsonNullable.<Integer>of(bodyTemperature);
    return this;
  }

  /**
   * Get bodyTemperature
   * @return bodyTemperature
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getBodyTemperature() {
        return bodyTemperature.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_BODY_TEMPERATURE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getBodyTemperature_JsonNullable() {
    return bodyTemperature;
  }
  
  @JsonProperty(JSON_PROPERTY_BODY_TEMPERATURE)
  public void setBodyTemperature_JsonNullable(JsonNullable<Integer> bodyTemperature) {
    this.bodyTemperature = bodyTemperature;
  }

  public void setBodyTemperature(@javax.annotation.Nullable Integer bodyTemperature) {
    this.bodyTemperature = JsonNullable.<Integer>of(bodyTemperature);
  }


  public PublicReadinessContributors hrvBalance(@javax.annotation.Nullable Integer hrvBalance) {
    this.hrvBalance = JsonNullable.<Integer>of(hrvBalance);
    return this;
  }

  /**
   * Get hrvBalance
   * @return hrvBalance
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getHrvBalance() {
        return hrvBalance.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_HRV_BALANCE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getHrvBalance_JsonNullable() {
    return hrvBalance;
  }
  
  @JsonProperty(JSON_PROPERTY_HRV_BALANCE)
  public void setHrvBalance_JsonNullable(JsonNullable<Integer> hrvBalance) {
    this.hrvBalance = hrvBalance;
  }

  public void setHrvBalance(@javax.annotation.Nullable Integer hrvBalance) {
    this.hrvBalance = JsonNullable.<Integer>of(hrvBalance);
  }


  public PublicReadinessContributors previousDayActivity(@javax.annotation.Nullable Integer previousDayActivity) {
    this.previousDayActivity = JsonNullable.<Integer>of(previousDayActivity);
    return this;
  }

  /**
   * Get previousDayActivity
   * @return previousDayActivity
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getPreviousDayActivity() {
        return previousDayActivity.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_PREVIOUS_DAY_ACTIVITY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getPreviousDayActivity_JsonNullable() {
    return previousDayActivity;
  }
  
  @JsonProperty(JSON_PROPERTY_PREVIOUS_DAY_ACTIVITY)
  public void setPreviousDayActivity_JsonNullable(JsonNullable<Integer> previousDayActivity) {
    this.previousDayActivity = previousDayActivity;
  }

  public void setPreviousDayActivity(@javax.annotation.Nullable Integer previousDayActivity) {
    this.previousDayActivity = JsonNullable.<Integer>of(previousDayActivity);
  }


  public PublicReadinessContributors previousNight(@javax.annotation.Nullable Integer previousNight) {
    this.previousNight = JsonNullable.<Integer>of(previousNight);
    return this;
  }

  /**
   * Get previousNight
   * @return previousNight
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getPreviousNight() {
        return previousNight.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_PREVIOUS_NIGHT)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getPreviousNight_JsonNullable() {
    return previousNight;
  }
  
  @JsonProperty(JSON_PROPERTY_PREVIOUS_NIGHT)
  public void setPreviousNight_JsonNullable(JsonNullable<Integer> previousNight) {
    this.previousNight = previousNight;
  }

  public void setPreviousNight(@javax.annotation.Nullable Integer previousNight) {
    this.previousNight = JsonNullable.<Integer>of(previousNight);
  }


  public PublicReadinessContributors recoveryIndex(@javax.annotation.Nullable Integer recoveryIndex) {
    this.recoveryIndex = JsonNullable.<Integer>of(recoveryIndex);
    return this;
  }

  /**
   * Get recoveryIndex
   * @return recoveryIndex
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getRecoveryIndex() {
        return recoveryIndex.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_RECOVERY_INDEX)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getRecoveryIndex_JsonNullable() {
    return recoveryIndex;
  }
  
  @JsonProperty(JSON_PROPERTY_RECOVERY_INDEX)
  public void setRecoveryIndex_JsonNullable(JsonNullable<Integer> recoveryIndex) {
    this.recoveryIndex = recoveryIndex;
  }

  public void setRecoveryIndex(@javax.annotation.Nullable Integer recoveryIndex) {
    this.recoveryIndex = JsonNullable.<Integer>of(recoveryIndex);
  }


  public PublicReadinessContributors restingHeartRate(@javax.annotation.Nullable Integer restingHeartRate) {
    this.restingHeartRate = JsonNullable.<Integer>of(restingHeartRate);
    return this;
  }

  /**
   * Get restingHeartRate
   * @return restingHeartRate
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getRestingHeartRate() {
        return restingHeartRate.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_RESTING_HEART_RATE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getRestingHeartRate_JsonNullable() {
    return restingHeartRate;
  }
  
  @JsonProperty(JSON_PROPERTY_RESTING_HEART_RATE)
  public void setRestingHeartRate_JsonNullable(JsonNullable<Integer> restingHeartRate) {
    this.restingHeartRate = restingHeartRate;
  }

  public void setRestingHeartRate(@javax.annotation.Nullable Integer restingHeartRate) {
    this.restingHeartRate = JsonNullable.<Integer>of(restingHeartRate);
  }


  public PublicReadinessContributors sleepBalance(@javax.annotation.Nullable Integer sleepBalance) {
    this.sleepBalance = JsonNullable.<Integer>of(sleepBalance);
    return this;
  }

  /**
   * Get sleepBalance
   * @return sleepBalance
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getSleepBalance() {
        return sleepBalance.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SLEEP_BALANCE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getSleepBalance_JsonNullable() {
    return sleepBalance;
  }
  
  @JsonProperty(JSON_PROPERTY_SLEEP_BALANCE)
  public void setSleepBalance_JsonNullable(JsonNullable<Integer> sleepBalance) {
    this.sleepBalance = sleepBalance;
  }

  public void setSleepBalance(@javax.annotation.Nullable Integer sleepBalance) {
    this.sleepBalance = JsonNullable.<Integer>of(sleepBalance);
  }


  public PublicReadinessContributors sleepRegularity(@javax.annotation.Nullable Integer sleepRegularity) {
    this.sleepRegularity = JsonNullable.<Integer>of(sleepRegularity);
    return this;
  }

  /**
   * Get sleepRegularity
   * @return sleepRegularity
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getSleepRegularity() {
        return sleepRegularity.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SLEEP_REGULARITY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getSleepRegularity_JsonNullable() {
    return sleepRegularity;
  }
  
  @JsonProperty(JSON_PROPERTY_SLEEP_REGULARITY)
  public void setSleepRegularity_JsonNullable(JsonNullable<Integer> sleepRegularity) {
    this.sleepRegularity = sleepRegularity;
  }

  public void setSleepRegularity(@javax.annotation.Nullable Integer sleepRegularity) {
    this.sleepRegularity = JsonNullable.<Integer>of(sleepRegularity);
  }


  /**
   * Return true if this PublicReadinessContributors object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicReadinessContributors publicReadinessContributors = (PublicReadinessContributors) o;
    return equalsNullable(this.activityBalance, publicReadinessContributors.activityBalance) &&
        equalsNullable(this.bodyTemperature, publicReadinessContributors.bodyTemperature) &&
        equalsNullable(this.hrvBalance, publicReadinessContributors.hrvBalance) &&
        equalsNullable(this.previousDayActivity, publicReadinessContributors.previousDayActivity) &&
        equalsNullable(this.previousNight, publicReadinessContributors.previousNight) &&
        equalsNullable(this.recoveryIndex, publicReadinessContributors.recoveryIndex) &&
        equalsNullable(this.restingHeartRate, publicReadinessContributors.restingHeartRate) &&
        equalsNullable(this.sleepBalance, publicReadinessContributors.sleepBalance) &&
        equalsNullable(this.sleepRegularity, publicReadinessContributors.sleepRegularity);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(hashCodeNullable(activityBalance), hashCodeNullable(bodyTemperature), hashCodeNullable(hrvBalance), hashCodeNullable(previousDayActivity), hashCodeNullable(previousNight), hashCodeNullable(recoveryIndex), hashCodeNullable(restingHeartRate), hashCodeNullable(sleepBalance), hashCodeNullable(sleepRegularity));
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
    sb.append("class PublicReadinessContributors {\n");
    sb.append("    activityBalance: ").append(toIndentedString(activityBalance)).append("\n");
    sb.append("    bodyTemperature: ").append(toIndentedString(bodyTemperature)).append("\n");
    sb.append("    hrvBalance: ").append(toIndentedString(hrvBalance)).append("\n");
    sb.append("    previousDayActivity: ").append(toIndentedString(previousDayActivity)).append("\n");
    sb.append("    previousNight: ").append(toIndentedString(previousNight)).append("\n");
    sb.append("    recoveryIndex: ").append(toIndentedString(recoveryIndex)).append("\n");
    sb.append("    restingHeartRate: ").append(toIndentedString(restingHeartRate)).append("\n");
    sb.append("    sleepBalance: ").append(toIndentedString(sleepBalance)).append("\n");
    sb.append("    sleepRegularity: ").append(toIndentedString(sleepRegularity)).append("\n");
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

    // add `activity_balance` to the URL query string
    if (getActivityBalance() != null) {
      joiner.add(String.format("%sactivity_balance%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getActivityBalance()))));
    }

    // add `body_temperature` to the URL query string
    if (getBodyTemperature() != null) {
      joiner.add(String.format("%sbody_temperature%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getBodyTemperature()))));
    }

    // add `hrv_balance` to the URL query string
    if (getHrvBalance() != null) {
      joiner.add(String.format("%shrv_balance%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getHrvBalance()))));
    }

    // add `previous_day_activity` to the URL query string
    if (getPreviousDayActivity() != null) {
      joiner.add(String.format("%sprevious_day_activity%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getPreviousDayActivity()))));
    }

    // add `previous_night` to the URL query string
    if (getPreviousNight() != null) {
      joiner.add(String.format("%sprevious_night%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getPreviousNight()))));
    }

    // add `recovery_index` to the URL query string
    if (getRecoveryIndex() != null) {
      joiner.add(String.format("%srecovery_index%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRecoveryIndex()))));
    }

    // add `resting_heart_rate` to the URL query string
    if (getRestingHeartRate() != null) {
      joiner.add(String.format("%sresting_heart_rate%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRestingHeartRate()))));
    }

    // add `sleep_balance` to the URL query string
    if (getSleepBalance() != null) {
      joiner.add(String.format("%ssleep_balance%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSleepBalance()))));
    }

    // add `sleep_regularity` to the URL query string
    if (getSleepRegularity() != null) {
      joiner.add(String.format("%ssleep_regularity%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSleepRegularity()))));
    }

    return joiner.toString();
  }
}

