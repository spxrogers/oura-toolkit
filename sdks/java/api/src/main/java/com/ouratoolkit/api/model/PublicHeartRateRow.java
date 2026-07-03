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
import com.ouratoolkit.api.model.PublicHeartRateSource;
import java.util.Arrays;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Heart rate sample
 */
@JsonPropertyOrder({
  PublicHeartRateRow.JSON_PROPERTY_TIMESTAMP,
  PublicHeartRateRow.JSON_PROPERTY_TIMESTAMP_UNIX,
  PublicHeartRateRow.JSON_PROPERTY_BPM,
  PublicHeartRateRow.JSON_PROPERTY_SOURCE
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicHeartRateRow {
  public static final String JSON_PROPERTY_TIMESTAMP = "timestamp";
  @javax.annotation.Nullable
  private String timestamp;

  public static final String JSON_PROPERTY_TIMESTAMP_UNIX = "timestamp_unix";
  @javax.annotation.Nonnull
  private Integer timestampUnix;

  public static final String JSON_PROPERTY_BPM = "bpm";
  @javax.annotation.Nonnull
  private Integer bpm;

  public static final String JSON_PROPERTY_SOURCE = "source";
  @javax.annotation.Nonnull
  private PublicHeartRateSource source;

  public PublicHeartRateRow() { 
  }

  public PublicHeartRateRow timestamp(@javax.annotation.Nullable String timestamp) {
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


  public PublicHeartRateRow timestampUnix(@javax.annotation.Nonnull Integer timestampUnix) {
    this.timestampUnix = timestampUnix;
    return this;
  }

  /**
   * Timestamp of the discrete sample as unix time in milliseconds.
   * @return timestampUnix
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_TIMESTAMP_UNIX)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getTimestampUnix() {
    return timestampUnix;
  }


  @JsonProperty(JSON_PROPERTY_TIMESTAMP_UNIX)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setTimestampUnix(@javax.annotation.Nonnull Integer timestampUnix) {
    this.timestampUnix = timestampUnix;
  }


  public PublicHeartRateRow bpm(@javax.annotation.Nonnull Integer bpm) {
    this.bpm = bpm;
    return this;
  }

  /**
   * Heart rate as beats per minute.
   * @return bpm
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_BPM)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getBpm() {
    return bpm;
  }


  @JsonProperty(JSON_PROPERTY_BPM)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setBpm(@javax.annotation.Nonnull Integer bpm) {
    this.bpm = bpm;
  }


  public PublicHeartRateRow source(@javax.annotation.Nonnull PublicHeartRateSource source) {
    this.source = source;
    return this;
  }

  /**
   * Source of the sample.
   * @return source
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_SOURCE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public PublicHeartRateSource getSource() {
    return source;
  }


  @JsonProperty(JSON_PROPERTY_SOURCE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setSource(@javax.annotation.Nonnull PublicHeartRateSource source) {
    this.source = source;
  }


  /**
   * Return true if this PublicHeartRateRow object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicHeartRateRow publicHeartRateRow = (PublicHeartRateRow) o;
    return Objects.equals(this.timestamp, publicHeartRateRow.timestamp) &&
        Objects.equals(this.timestampUnix, publicHeartRateRow.timestampUnix) &&
        Objects.equals(this.bpm, publicHeartRateRow.bpm) &&
        Objects.equals(this.source, publicHeartRateRow.source);
  }

  @Override
  public int hashCode() {
    return Objects.hash(timestamp, timestampUnix, bpm, source);
  }

  @Override
  public String toString() {
    StringBuilder sb = new StringBuilder();
    sb.append("class PublicHeartRateRow {\n");
    sb.append("    timestamp: ").append(toIndentedString(timestamp)).append("\n");
    sb.append("    timestampUnix: ").append(toIndentedString(timestampUnix)).append("\n");
    sb.append("    bpm: ").append(toIndentedString(bpm)).append("\n");
    sb.append("    source: ").append(toIndentedString(source)).append("\n");
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

    // add `timestamp` to the URL query string
    if (getTimestamp() != null) {
      joiner.add(String.format("%stimestamp%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTimestamp()))));
    }

    // add `timestamp_unix` to the URL query string
    if (getTimestampUnix() != null) {
      joiner.add(String.format("%stimestamp_unix%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTimestampUnix()))));
    }

    // add `bpm` to the URL query string
    if (getBpm() != null) {
      joiner.add(String.format("%sbpm%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getBpm()))));
    }

    // add `source` to the URL query string
    if (getSource() != null) {
      joiner.add(String.format("%ssource%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSource()))));
    }

    return joiner.toString();
  }
}

