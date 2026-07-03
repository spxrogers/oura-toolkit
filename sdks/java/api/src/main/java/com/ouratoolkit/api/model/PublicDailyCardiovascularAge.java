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
import java.math.BigDecimal;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Daily Cardiovascular Age.
 */
@JsonPropertyOrder({
  PublicDailyCardiovascularAge.JSON_PROPERTY_ID,
  PublicDailyCardiovascularAge.JSON_PROPERTY_DAY,
  PublicDailyCardiovascularAge.JSON_PROPERTY_PULSE_WAVE_VELOCITY,
  PublicDailyCardiovascularAge.JSON_PROPERTY_VASCULAR_AGE
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicDailyCardiovascularAge {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_DAY = "day";
  @javax.annotation.Nullable
  private String day;

  public static final String JSON_PROPERTY_PULSE_WAVE_VELOCITY = "pulse_wave_velocity";
  private JsonNullable<BigDecimal> pulseWaveVelocity = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_VASCULAR_AGE = "vascular_age";
  private JsonNullable<Integer> vascularAge = JsonNullable.<Integer>undefined();

  public PublicDailyCardiovascularAge() { 
  }

  public PublicDailyCardiovascularAge id(@javax.annotation.Nonnull String id) {
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


  public PublicDailyCardiovascularAge day(@javax.annotation.Nullable String day) {
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


  public PublicDailyCardiovascularAge pulseWaveVelocity(@javax.annotation.Nullable BigDecimal pulseWaveVelocity) {
    this.pulseWaveVelocity = JsonNullable.<BigDecimal>of(pulseWaveVelocity);
    return this;
  }

  /**
   * Get pulseWaveVelocity
   * @return pulseWaveVelocity
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getPulseWaveVelocity() {
        return pulseWaveVelocity.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_PULSE_WAVE_VELOCITY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getPulseWaveVelocity_JsonNullable() {
    return pulseWaveVelocity;
  }
  
  @JsonProperty(JSON_PROPERTY_PULSE_WAVE_VELOCITY)
  public void setPulseWaveVelocity_JsonNullable(JsonNullable<BigDecimal> pulseWaveVelocity) {
    this.pulseWaveVelocity = pulseWaveVelocity;
  }

  public void setPulseWaveVelocity(@javax.annotation.Nullable BigDecimal pulseWaveVelocity) {
    this.pulseWaveVelocity = JsonNullable.<BigDecimal>of(pulseWaveVelocity);
  }


  public PublicDailyCardiovascularAge vascularAge(@javax.annotation.Nullable Integer vascularAge) {
    this.vascularAge = JsonNullable.<Integer>of(vascularAge);
    return this;
  }

  /**
   * Get vascularAge
   * @return vascularAge
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getVascularAge() {
        return vascularAge.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_VASCULAR_AGE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getVascularAge_JsonNullable() {
    return vascularAge;
  }
  
  @JsonProperty(JSON_PROPERTY_VASCULAR_AGE)
  public void setVascularAge_JsonNullable(JsonNullable<Integer> vascularAge) {
    this.vascularAge = vascularAge;
  }

  public void setVascularAge(@javax.annotation.Nullable Integer vascularAge) {
    this.vascularAge = JsonNullable.<Integer>of(vascularAge);
  }


  /**
   * Return true if this PublicDailyCardiovascularAge object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicDailyCardiovascularAge publicDailyCardiovascularAge = (PublicDailyCardiovascularAge) o;
    return Objects.equals(this.id, publicDailyCardiovascularAge.id) &&
        Objects.equals(this.day, publicDailyCardiovascularAge.day) &&
        equalsNullable(this.pulseWaveVelocity, publicDailyCardiovascularAge.pulseWaveVelocity) &&
        equalsNullable(this.vascularAge, publicDailyCardiovascularAge.vascularAge);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, day, hashCodeNullable(pulseWaveVelocity), hashCodeNullable(vascularAge));
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
    sb.append("class PublicDailyCardiovascularAge {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    pulseWaveVelocity: ").append(toIndentedString(pulseWaveVelocity)).append("\n");
    sb.append("    vascularAge: ").append(toIndentedString(vascularAge)).append("\n");
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

    // add `pulse_wave_velocity` to the URL query string
    if (getPulseWaveVelocity() != null) {
      joiner.add(String.format("%spulse_wave_velocity%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getPulseWaveVelocity()))));
    }

    // add `vascular_age` to the URL query string
    if (getVascularAge() != null) {
      joiner.add(String.format("%svascular_age%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getVascularAge()))));
    }

    return joiner.toString();
  }
}

