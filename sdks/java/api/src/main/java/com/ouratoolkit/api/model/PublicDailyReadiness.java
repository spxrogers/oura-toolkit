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
import com.ouratoolkit.api.model.PublicReadinessContributors;
import java.math.BigDecimal;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Public object defining daily readiness.
 */
@JsonPropertyOrder({
  PublicDailyReadiness.JSON_PROPERTY_ID,
  PublicDailyReadiness.JSON_PROPERTY_CONTRIBUTORS,
  PublicDailyReadiness.JSON_PROPERTY_DAY,
  PublicDailyReadiness.JSON_PROPERTY_SCORE,
  PublicDailyReadiness.JSON_PROPERTY_TEMPERATURE_DEVIATION,
  PublicDailyReadiness.JSON_PROPERTY_TEMPERATURE_TREND_DEVIATION,
  PublicDailyReadiness.JSON_PROPERTY_TIMESTAMP
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicDailyReadiness {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_CONTRIBUTORS = "contributors";
  @javax.annotation.Nonnull
  private PublicReadinessContributors contributors;

  public static final String JSON_PROPERTY_DAY = "day";
  @javax.annotation.Nullable
  private String day;

  public static final String JSON_PROPERTY_SCORE = "score";
  private JsonNullable<Integer> score = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_TEMPERATURE_DEVIATION = "temperature_deviation";
  private JsonNullable<BigDecimal> temperatureDeviation = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_TEMPERATURE_TREND_DEVIATION = "temperature_trend_deviation";
  private JsonNullable<BigDecimal> temperatureTrendDeviation = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_TIMESTAMP = "timestamp";
  @javax.annotation.Nullable
  private String timestamp;

  public PublicDailyReadiness() { 
  }

  public PublicDailyReadiness id(@javax.annotation.Nonnull String id) {
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


  public PublicDailyReadiness contributors(@javax.annotation.Nonnull PublicReadinessContributors contributors) {
    this.contributors = contributors;
    return this;
  }

  /**
   * Contributors of the daily readiness score.
   * @return contributors
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_CONTRIBUTORS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public PublicReadinessContributors getContributors() {
    return contributors;
  }


  @JsonProperty(JSON_PROPERTY_CONTRIBUTORS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setContributors(@javax.annotation.Nonnull PublicReadinessContributors contributors) {
    this.contributors = contributors;
  }


  public PublicDailyReadiness day(@javax.annotation.Nullable String day) {
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


  public PublicDailyReadiness score(@javax.annotation.Nullable Integer score) {
    this.score = JsonNullable.<Integer>of(score);
    return this;
  }

  /**
   * Get score
   * @return score
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getScore() {
        return score.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SCORE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getScore_JsonNullable() {
    return score;
  }
  
  @JsonProperty(JSON_PROPERTY_SCORE)
  public void setScore_JsonNullable(JsonNullable<Integer> score) {
    this.score = score;
  }

  public void setScore(@javax.annotation.Nullable Integer score) {
    this.score = JsonNullable.<Integer>of(score);
  }


  public PublicDailyReadiness temperatureDeviation(@javax.annotation.Nullable BigDecimal temperatureDeviation) {
    this.temperatureDeviation = JsonNullable.<BigDecimal>of(temperatureDeviation);
    return this;
  }

  /**
   * Get temperatureDeviation
   * @return temperatureDeviation
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getTemperatureDeviation() {
        return temperatureDeviation.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TEMPERATURE_DEVIATION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getTemperatureDeviation_JsonNullable() {
    return temperatureDeviation;
  }
  
  @JsonProperty(JSON_PROPERTY_TEMPERATURE_DEVIATION)
  public void setTemperatureDeviation_JsonNullable(JsonNullable<BigDecimal> temperatureDeviation) {
    this.temperatureDeviation = temperatureDeviation;
  }

  public void setTemperatureDeviation(@javax.annotation.Nullable BigDecimal temperatureDeviation) {
    this.temperatureDeviation = JsonNullable.<BigDecimal>of(temperatureDeviation);
  }


  public PublicDailyReadiness temperatureTrendDeviation(@javax.annotation.Nullable BigDecimal temperatureTrendDeviation) {
    this.temperatureTrendDeviation = JsonNullable.<BigDecimal>of(temperatureTrendDeviation);
    return this;
  }

  /**
   * Get temperatureTrendDeviation
   * @return temperatureTrendDeviation
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getTemperatureTrendDeviation() {
        return temperatureTrendDeviation.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TEMPERATURE_TREND_DEVIATION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getTemperatureTrendDeviation_JsonNullable() {
    return temperatureTrendDeviation;
  }
  
  @JsonProperty(JSON_PROPERTY_TEMPERATURE_TREND_DEVIATION)
  public void setTemperatureTrendDeviation_JsonNullable(JsonNullable<BigDecimal> temperatureTrendDeviation) {
    this.temperatureTrendDeviation = temperatureTrendDeviation;
  }

  public void setTemperatureTrendDeviation(@javax.annotation.Nullable BigDecimal temperatureTrendDeviation) {
    this.temperatureTrendDeviation = JsonNullable.<BigDecimal>of(temperatureTrendDeviation);
  }


  public PublicDailyReadiness timestamp(@javax.annotation.Nullable String timestamp) {
    this.timestamp = timestamp;
    return this;
  }

  /**
   * Get timestamp
   * @return timestamp
   */
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_TIMESTAMP)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getTimestamp() {
    return timestamp;
  }


  @JsonProperty(JSON_PROPERTY_TIMESTAMP)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setTimestamp(@javax.annotation.Nullable String timestamp) {
    this.timestamp = timestamp;
  }


  /**
   * Return true if this PublicDailyReadiness object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicDailyReadiness publicDailyReadiness = (PublicDailyReadiness) o;
    return Objects.equals(this.id, publicDailyReadiness.id) &&
        Objects.equals(this.contributors, publicDailyReadiness.contributors) &&
        Objects.equals(this.day, publicDailyReadiness.day) &&
        equalsNullable(this.score, publicDailyReadiness.score) &&
        equalsNullable(this.temperatureDeviation, publicDailyReadiness.temperatureDeviation) &&
        equalsNullable(this.temperatureTrendDeviation, publicDailyReadiness.temperatureTrendDeviation) &&
        Objects.equals(this.timestamp, publicDailyReadiness.timestamp);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, contributors, day, hashCodeNullable(score), hashCodeNullable(temperatureDeviation), hashCodeNullable(temperatureTrendDeviation), timestamp);
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
    sb.append("class PublicDailyReadiness {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    contributors: ").append(toIndentedString(contributors)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    score: ").append(toIndentedString(score)).append("\n");
    sb.append("    temperatureDeviation: ").append(toIndentedString(temperatureDeviation)).append("\n");
    sb.append("    temperatureTrendDeviation: ").append(toIndentedString(temperatureTrendDeviation)).append("\n");
    sb.append("    timestamp: ").append(toIndentedString(timestamp)).append("\n");
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

    // add `contributors` to the URL query string
    if (getContributors() != null) {
      joiner.add(getContributors().toUrlQueryString(prefix + "contributors" + suffix));
    }

    // add `day` to the URL query string
    if (getDay() != null) {
      joiner.add(String.format("%sday%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDay()))));
    }

    // add `score` to the URL query string
    if (getScore() != null) {
      joiner.add(String.format("%sscore%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getScore()))));
    }

    // add `temperature_deviation` to the URL query string
    if (getTemperatureDeviation() != null) {
      joiner.add(String.format("%stemperature_deviation%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTemperatureDeviation()))));
    }

    // add `temperature_trend_deviation` to the URL query string
    if (getTemperatureTrendDeviation() != null) {
      joiner.add(String.format("%stemperature_trend_deviation%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTemperatureTrendDeviation()))));
    }

    // add `timestamp` to the URL query string
    if (getTimestamp() != null) {
      joiner.add(String.format("%stimestamp%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTimestamp()))));
    }

    return joiner.toString();
  }
}

