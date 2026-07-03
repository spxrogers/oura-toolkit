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
import com.ouratoolkit.api.model.PublicRestModeEpisode;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Rest mode episode information.
 */
@JsonPropertyOrder({
  PublicRestModePeriod.JSON_PROPERTY_ID,
  PublicRestModePeriod.JSON_PROPERTY_END_DAY,
  PublicRestModePeriod.JSON_PROPERTY_END_TIME,
  PublicRestModePeriod.JSON_PROPERTY_EPISODES,
  PublicRestModePeriod.JSON_PROPERTY_START_DAY,
  PublicRestModePeriod.JSON_PROPERTY_START_TIME
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicRestModePeriod {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_END_DAY = "end_day";
  private JsonNullable<String> endDay = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_END_TIME = "end_time";
  private JsonNullable<String> endTime = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_EPISODES = "episodes";
  @javax.annotation.Nonnull
  private List<PublicRestModeEpisode> episodes = new ArrayList<>();

  public static final String JSON_PROPERTY_START_DAY = "start_day";
  @javax.annotation.Nullable
  private String startDay;

  public static final String JSON_PROPERTY_START_TIME = "start_time";
  private JsonNullable<String> startTime = JsonNullable.<String>undefined();

  public PublicRestModePeriod() { 
  }

  public PublicRestModePeriod id(@javax.annotation.Nonnull String id) {
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


  public PublicRestModePeriod endDay(@javax.annotation.Nullable String endDay) {
    this.endDay = JsonNullable.<String>of(endDay);
    return this;
  }

  /**
   * Get endDay
   * @return endDay
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getEndDay() {
        return endDay.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_END_DAY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getEndDay_JsonNullable() {
    return endDay;
  }
  
  @JsonProperty(JSON_PROPERTY_END_DAY)
  public void setEndDay_JsonNullable(JsonNullable<String> endDay) {
    this.endDay = endDay;
  }

  public void setEndDay(@javax.annotation.Nullable String endDay) {
    this.endDay = JsonNullable.<String>of(endDay);
  }


  public PublicRestModePeriod endTime(@javax.annotation.Nullable String endTime) {
    this.endTime = JsonNullable.<String>of(endTime);
    return this;
  }

  /**
   * Get endTime
   * @return endTime
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getEndTime() {
        return endTime.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_END_TIME)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getEndTime_JsonNullable() {
    return endTime;
  }
  
  @JsonProperty(JSON_PROPERTY_END_TIME)
  public void setEndTime_JsonNullable(JsonNullable<String> endTime) {
    this.endTime = endTime;
  }

  public void setEndTime(@javax.annotation.Nullable String endTime) {
    this.endTime = JsonNullable.<String>of(endTime);
  }


  public PublicRestModePeriod episodes(@javax.annotation.Nonnull List<PublicRestModeEpisode> episodes) {
    this.episodes = episodes;
    return this;
  }

  public PublicRestModePeriod addEpisodesItem(PublicRestModeEpisode episodesItem) {
    if (this.episodes == null) {
      this.episodes = new ArrayList<>();
    }
    this.episodes.add(episodesItem);
    return this;
  }

  /**
   * Collection of episodes during rest mode, consisting of tags.
   * @return episodes
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_EPISODES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public List<PublicRestModeEpisode> getEpisodes() {
    return episodes;
  }


  @JsonProperty(JSON_PROPERTY_EPISODES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setEpisodes(@javax.annotation.Nonnull List<PublicRestModeEpisode> episodes) {
    this.episodes = episodes;
  }


  public PublicRestModePeriod startDay(@javax.annotation.Nullable String startDay) {
    this.startDay = startDay;
    return this;
  }

  /**
   * Get startDay
   * @return startDay
   */
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_START_DAY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getStartDay() {
    return startDay;
  }


  @JsonProperty(JSON_PROPERTY_START_DAY)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setStartDay(@javax.annotation.Nullable String startDay) {
    this.startDay = startDay;
  }


  public PublicRestModePeriod startTime(@javax.annotation.Nullable String startTime) {
    this.startTime = JsonNullable.<String>of(startTime);
    return this;
  }

  /**
   * Get startTime
   * @return startTime
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getStartTime() {
        return startTime.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_START_TIME)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getStartTime_JsonNullable() {
    return startTime;
  }
  
  @JsonProperty(JSON_PROPERTY_START_TIME)
  public void setStartTime_JsonNullable(JsonNullable<String> startTime) {
    this.startTime = startTime;
  }

  public void setStartTime(@javax.annotation.Nullable String startTime) {
    this.startTime = JsonNullable.<String>of(startTime);
  }


  /**
   * Return true if this PublicRestModePeriod object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicRestModePeriod publicRestModePeriod = (PublicRestModePeriod) o;
    return Objects.equals(this.id, publicRestModePeriod.id) &&
        equalsNullable(this.endDay, publicRestModePeriod.endDay) &&
        equalsNullable(this.endTime, publicRestModePeriod.endTime) &&
        Objects.equals(this.episodes, publicRestModePeriod.episodes) &&
        Objects.equals(this.startDay, publicRestModePeriod.startDay) &&
        equalsNullable(this.startTime, publicRestModePeriod.startTime);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, hashCodeNullable(endDay), hashCodeNullable(endTime), episodes, startDay, hashCodeNullable(startTime));
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
    sb.append("class PublicRestModePeriod {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    endDay: ").append(toIndentedString(endDay)).append("\n");
    sb.append("    endTime: ").append(toIndentedString(endTime)).append("\n");
    sb.append("    episodes: ").append(toIndentedString(episodes)).append("\n");
    sb.append("    startDay: ").append(toIndentedString(startDay)).append("\n");
    sb.append("    startTime: ").append(toIndentedString(startTime)).append("\n");
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

    // add `end_day` to the URL query string
    if (getEndDay() != null) {
      joiner.add(String.format("%send_day%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEndDay()))));
    }

    // add `end_time` to the URL query string
    if (getEndTime() != null) {
      joiner.add(String.format("%send_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEndTime()))));
    }

    // add `episodes` to the URL query string
    if (getEpisodes() != null) {
      for (int i = 0; i < getEpisodes().size(); i++) {
        if (getEpisodes().get(i) != null) {
          joiner.add(getEpisodes().get(i).toUrlQueryString(String.format("%sepisodes%s%s", prefix, suffix,
          "".equals(suffix) ? "" : String.format("%s%d%s", containerPrefix, i, containerSuffix))));
        }
      }
    }

    // add `start_day` to the URL query string
    if (getStartDay() != null) {
      joiner.add(String.format("%sstart_day%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getStartDay()))));
    }

    // add `start_time` to the URL query string
    if (getStartTime() != null) {
      joiner.add(String.format("%sstart_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getStartTime()))));
    }

    return joiner.toString();
  }
}

