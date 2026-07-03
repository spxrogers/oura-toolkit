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
import com.ouratoolkit.api.model.PublicWorkoutIntensity;
import com.ouratoolkit.api.model.PublicWorkoutSource;
import java.math.BigDecimal;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Public model for Workout.
 */
@JsonPropertyOrder({
  PublicWorkout.JSON_PROPERTY_ID,
  PublicWorkout.JSON_PROPERTY_ACTIVITY,
  PublicWorkout.JSON_PROPERTY_CALORIES,
  PublicWorkout.JSON_PROPERTY_DAY,
  PublicWorkout.JSON_PROPERTY_DISTANCE,
  PublicWorkout.JSON_PROPERTY_END_DATETIME,
  PublicWorkout.JSON_PROPERTY_INTENSITY,
  PublicWorkout.JSON_PROPERTY_LABEL,
  PublicWorkout.JSON_PROPERTY_SOURCE,
  PublicWorkout.JSON_PROPERTY_START_DATETIME
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicWorkout {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_ACTIVITY = "activity";
  @javax.annotation.Nonnull
  private String activity;

  public static final String JSON_PROPERTY_CALORIES = "calories";
  private JsonNullable<BigDecimal> calories = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_DAY = "day";
  @javax.annotation.Nullable
  private String day;

  public static final String JSON_PROPERTY_DISTANCE = "distance";
  private JsonNullable<BigDecimal> distance = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_END_DATETIME = "end_datetime";
  @javax.annotation.Nullable
  private String endDatetime;

  public static final String JSON_PROPERTY_INTENSITY = "intensity";
  @javax.annotation.Nonnull
  private PublicWorkoutIntensity intensity;

  public static final String JSON_PROPERTY_LABEL = "label";
  private JsonNullable<String> label = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_SOURCE = "source";
  @javax.annotation.Nonnull
  private PublicWorkoutSource source;

  public static final String JSON_PROPERTY_START_DATETIME = "start_datetime";
  @javax.annotation.Nullable
  private String startDatetime;

  public PublicWorkout() { 
  }

  public PublicWorkout id(@javax.annotation.Nonnull String id) {
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


  public PublicWorkout activity(@javax.annotation.Nonnull String activity) {
    this.activity = activity;
    return this;
  }

  /**
   * Type of the workout activity.
   * @return activity
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_ACTIVITY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getActivity() {
    return activity;
  }


  @JsonProperty(JSON_PROPERTY_ACTIVITY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setActivity(@javax.annotation.Nonnull String activity) {
    this.activity = activity;
  }


  public PublicWorkout calories(@javax.annotation.Nullable BigDecimal calories) {
    this.calories = JsonNullable.<BigDecimal>of(calories);
    return this;
  }

  /**
   * Get calories
   * @return calories
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getCalories() {
        return calories.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_CALORIES)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getCalories_JsonNullable() {
    return calories;
  }
  
  @JsonProperty(JSON_PROPERTY_CALORIES)
  public void setCalories_JsonNullable(JsonNullable<BigDecimal> calories) {
    this.calories = calories;
  }

  public void setCalories(@javax.annotation.Nullable BigDecimal calories) {
    this.calories = JsonNullable.<BigDecimal>of(calories);
  }


  public PublicWorkout day(@javax.annotation.Nullable String day) {
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


  public PublicWorkout distance(@javax.annotation.Nullable BigDecimal distance) {
    this.distance = JsonNullable.<BigDecimal>of(distance);
    return this;
  }

  /**
   * Get distance
   * @return distance
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getDistance() {
        return distance.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_DISTANCE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getDistance_JsonNullable() {
    return distance;
  }
  
  @JsonProperty(JSON_PROPERTY_DISTANCE)
  public void setDistance_JsonNullable(JsonNullable<BigDecimal> distance) {
    this.distance = distance;
  }

  public void setDistance(@javax.annotation.Nullable BigDecimal distance) {
    this.distance = JsonNullable.<BigDecimal>of(distance);
  }


  public PublicWorkout endDatetime(@javax.annotation.Nullable String endDatetime) {
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


  public PublicWorkout intensity(@javax.annotation.Nonnull PublicWorkoutIntensity intensity) {
    this.intensity = intensity;
    return this;
  }

  /**
   * Intensity of the workout.
   * @return intensity
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_INTENSITY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public PublicWorkoutIntensity getIntensity() {
    return intensity;
  }


  @JsonProperty(JSON_PROPERTY_INTENSITY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setIntensity(@javax.annotation.Nonnull PublicWorkoutIntensity intensity) {
    this.intensity = intensity;
  }


  public PublicWorkout label(@javax.annotation.Nullable String label) {
    this.label = JsonNullable.<String>of(label);
    return this;
  }

  /**
   * Get label
   * @return label
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getLabel() {
        return label.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_LABEL)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getLabel_JsonNullable() {
    return label;
  }
  
  @JsonProperty(JSON_PROPERTY_LABEL)
  public void setLabel_JsonNullable(JsonNullable<String> label) {
    this.label = label;
  }

  public void setLabel(@javax.annotation.Nullable String label) {
    this.label = JsonNullable.<String>of(label);
  }


  public PublicWorkout source(@javax.annotation.Nonnull PublicWorkoutSource source) {
    this.source = source;
    return this;
  }

  /**
   * Possible workout sources.
   * @return source
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_SOURCE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public PublicWorkoutSource getSource() {
    return source;
  }


  @JsonProperty(JSON_PROPERTY_SOURCE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setSource(@javax.annotation.Nonnull PublicWorkoutSource source) {
    this.source = source;
  }


  public PublicWorkout startDatetime(@javax.annotation.Nullable String startDatetime) {
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


  /**
   * Return true if this PublicWorkout object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicWorkout publicWorkout = (PublicWorkout) o;
    return Objects.equals(this.id, publicWorkout.id) &&
        Objects.equals(this.activity, publicWorkout.activity) &&
        equalsNullable(this.calories, publicWorkout.calories) &&
        Objects.equals(this.day, publicWorkout.day) &&
        equalsNullable(this.distance, publicWorkout.distance) &&
        Objects.equals(this.endDatetime, publicWorkout.endDatetime) &&
        Objects.equals(this.intensity, publicWorkout.intensity) &&
        equalsNullable(this.label, publicWorkout.label) &&
        Objects.equals(this.source, publicWorkout.source) &&
        Objects.equals(this.startDatetime, publicWorkout.startDatetime);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, activity, hashCodeNullable(calories), day, hashCodeNullable(distance), endDatetime, intensity, hashCodeNullable(label), source, startDatetime);
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
    sb.append("class PublicWorkout {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    activity: ").append(toIndentedString(activity)).append("\n");
    sb.append("    calories: ").append(toIndentedString(calories)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    distance: ").append(toIndentedString(distance)).append("\n");
    sb.append("    endDatetime: ").append(toIndentedString(endDatetime)).append("\n");
    sb.append("    intensity: ").append(toIndentedString(intensity)).append("\n");
    sb.append("    label: ").append(toIndentedString(label)).append("\n");
    sb.append("    source: ").append(toIndentedString(source)).append("\n");
    sb.append("    startDatetime: ").append(toIndentedString(startDatetime)).append("\n");
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

    // add `activity` to the URL query string
    if (getActivity() != null) {
      joiner.add(String.format("%sactivity%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getActivity()))));
    }

    // add `calories` to the URL query string
    if (getCalories() != null) {
      joiner.add(String.format("%scalories%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getCalories()))));
    }

    // add `day` to the URL query string
    if (getDay() != null) {
      joiner.add(String.format("%sday%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDay()))));
    }

    // add `distance` to the URL query string
    if (getDistance() != null) {
      joiner.add(String.format("%sdistance%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDistance()))));
    }

    // add `end_datetime` to the URL query string
    if (getEndDatetime() != null) {
      joiner.add(String.format("%send_datetime%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEndDatetime()))));
    }

    // add `intensity` to the URL query string
    if (getIntensity() != null) {
      joiner.add(String.format("%sintensity%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getIntensity()))));
    }

    // add `label` to the URL query string
    if (getLabel() != null) {
      joiner.add(String.format("%slabel%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLabel()))));
    }

    // add `source` to the URL query string
    if (getSource() != null) {
      joiner.add(String.format("%ssource%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSource()))));
    }

    // add `start_datetime` to the URL query string
    if (getStartDatetime() != null) {
      joiner.add(String.format("%sstart_datetime%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getStartDatetime()))));
    }

    return joiner.toString();
  }
}

