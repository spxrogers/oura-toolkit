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
import com.ouratoolkit.api.model.PublicRingColor;
import com.ouratoolkit.api.model.PublicRingDesign;
import com.ouratoolkit.api.model.PublicRingHardwareType;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Ring configuration.
 */
@JsonPropertyOrder({
  PublicRingConfiguration.JSON_PROPERTY_ID,
  PublicRingConfiguration.JSON_PROPERTY_COLOR,
  PublicRingConfiguration.JSON_PROPERTY_DESIGN,
  PublicRingConfiguration.JSON_PROPERTY_FIRMWARE_VERSION,
  PublicRingConfiguration.JSON_PROPERTY_HARDWARE_TYPE,
  PublicRingConfiguration.JSON_PROPERTY_SET_UP_AT,
  PublicRingConfiguration.JSON_PROPERTY_SIZE
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicRingConfiguration {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_COLOR = "color";
  private JsonNullable<PublicRingColor> color = JsonNullable.<PublicRingColor>undefined();

  public static final String JSON_PROPERTY_DESIGN = "design";
  private JsonNullable<PublicRingDesign> design = JsonNullable.<PublicRingDesign>undefined();

  public static final String JSON_PROPERTY_FIRMWARE_VERSION = "firmware_version";
  private JsonNullable<String> firmwareVersion = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_HARDWARE_TYPE = "hardware_type";
  private JsonNullable<PublicRingHardwareType> hardwareType = JsonNullable.<PublicRingHardwareType>undefined();

  public static final String JSON_PROPERTY_SET_UP_AT = "set_up_at";
  private JsonNullable<String> setUpAt = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_SIZE = "size";
  private JsonNullable<Integer> size = JsonNullable.<Integer>undefined();

  public PublicRingConfiguration() { 
  }

  public PublicRingConfiguration id(@javax.annotation.Nonnull String id) {
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


  public PublicRingConfiguration color(@javax.annotation.Nullable PublicRingColor color) {
    this.color = JsonNullable.<PublicRingColor>of(color);
    return this;
  }

  /**
   * Get color
   * @return color
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicRingColor getColor() {
        return color.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_COLOR)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicRingColor> getColor_JsonNullable() {
    return color;
  }
  
  @JsonProperty(JSON_PROPERTY_COLOR)
  public void setColor_JsonNullable(JsonNullable<PublicRingColor> color) {
    this.color = color;
  }

  public void setColor(@javax.annotation.Nullable PublicRingColor color) {
    this.color = JsonNullable.<PublicRingColor>of(color);
  }


  public PublicRingConfiguration design(@javax.annotation.Nullable PublicRingDesign design) {
    this.design = JsonNullable.<PublicRingDesign>of(design);
    return this;
  }

  /**
   * Get design
   * @return design
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicRingDesign getDesign() {
        return design.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_DESIGN)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicRingDesign> getDesign_JsonNullable() {
    return design;
  }
  
  @JsonProperty(JSON_PROPERTY_DESIGN)
  public void setDesign_JsonNullable(JsonNullable<PublicRingDesign> design) {
    this.design = design;
  }

  public void setDesign(@javax.annotation.Nullable PublicRingDesign design) {
    this.design = JsonNullable.<PublicRingDesign>of(design);
  }


  public PublicRingConfiguration firmwareVersion(@javax.annotation.Nullable String firmwareVersion) {
    this.firmwareVersion = JsonNullable.<String>of(firmwareVersion);
    return this;
  }

  /**
   * Get firmwareVersion
   * @return firmwareVersion
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getFirmwareVersion() {
        return firmwareVersion.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_FIRMWARE_VERSION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getFirmwareVersion_JsonNullable() {
    return firmwareVersion;
  }
  
  @JsonProperty(JSON_PROPERTY_FIRMWARE_VERSION)
  public void setFirmwareVersion_JsonNullable(JsonNullable<String> firmwareVersion) {
    this.firmwareVersion = firmwareVersion;
  }

  public void setFirmwareVersion(@javax.annotation.Nullable String firmwareVersion) {
    this.firmwareVersion = JsonNullable.<String>of(firmwareVersion);
  }


  public PublicRingConfiguration hardwareType(@javax.annotation.Nullable PublicRingHardwareType hardwareType) {
    this.hardwareType = JsonNullable.<PublicRingHardwareType>of(hardwareType);
    return this;
  }

  /**
   * Get hardwareType
   * @return hardwareType
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicRingHardwareType getHardwareType() {
        return hardwareType.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_HARDWARE_TYPE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicRingHardwareType> getHardwareType_JsonNullable() {
    return hardwareType;
  }
  
  @JsonProperty(JSON_PROPERTY_HARDWARE_TYPE)
  public void setHardwareType_JsonNullable(JsonNullable<PublicRingHardwareType> hardwareType) {
    this.hardwareType = hardwareType;
  }

  public void setHardwareType(@javax.annotation.Nullable PublicRingHardwareType hardwareType) {
    this.hardwareType = JsonNullable.<PublicRingHardwareType>of(hardwareType);
  }


  public PublicRingConfiguration setUpAt(@javax.annotation.Nullable String setUpAt) {
    this.setUpAt = JsonNullable.<String>of(setUpAt);
    return this;
  }

  /**
   * Get setUpAt
   * @return setUpAt
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getSetUpAt() {
        return setUpAt.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SET_UP_AT)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getSetUpAt_JsonNullable() {
    return setUpAt;
  }
  
  @JsonProperty(JSON_PROPERTY_SET_UP_AT)
  public void setSetUpAt_JsonNullable(JsonNullable<String> setUpAt) {
    this.setUpAt = setUpAt;
  }

  public void setSetUpAt(@javax.annotation.Nullable String setUpAt) {
    this.setUpAt = JsonNullable.<String>of(setUpAt);
  }


  public PublicRingConfiguration size(@javax.annotation.Nullable Integer size) {
    this.size = JsonNullable.<Integer>of(size);
    return this;
  }

  /**
   * Get size
   * @return size
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getSize() {
        return size.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SIZE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getSize_JsonNullable() {
    return size;
  }
  
  @JsonProperty(JSON_PROPERTY_SIZE)
  public void setSize_JsonNullable(JsonNullable<Integer> size) {
    this.size = size;
  }

  public void setSize(@javax.annotation.Nullable Integer size) {
    this.size = JsonNullable.<Integer>of(size);
  }


  /**
   * Return true if this PublicRingConfiguration object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicRingConfiguration publicRingConfiguration = (PublicRingConfiguration) o;
    return Objects.equals(this.id, publicRingConfiguration.id) &&
        equalsNullable(this.color, publicRingConfiguration.color) &&
        equalsNullable(this.design, publicRingConfiguration.design) &&
        equalsNullable(this.firmwareVersion, publicRingConfiguration.firmwareVersion) &&
        equalsNullable(this.hardwareType, publicRingConfiguration.hardwareType) &&
        equalsNullable(this.setUpAt, publicRingConfiguration.setUpAt) &&
        equalsNullable(this.size, publicRingConfiguration.size);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, hashCodeNullable(color), hashCodeNullable(design), hashCodeNullable(firmwareVersion), hashCodeNullable(hardwareType), hashCodeNullable(setUpAt), hashCodeNullable(size));
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
    sb.append("class PublicRingConfiguration {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    color: ").append(toIndentedString(color)).append("\n");
    sb.append("    design: ").append(toIndentedString(design)).append("\n");
    sb.append("    firmwareVersion: ").append(toIndentedString(firmwareVersion)).append("\n");
    sb.append("    hardwareType: ").append(toIndentedString(hardwareType)).append("\n");
    sb.append("    setUpAt: ").append(toIndentedString(setUpAt)).append("\n");
    sb.append("    size: ").append(toIndentedString(size)).append("\n");
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

    // add `color` to the URL query string
    if (getColor() != null) {
      joiner.add(String.format("%scolor%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getColor()))));
    }

    // add `design` to the URL query string
    if (getDesign() != null) {
      joiner.add(String.format("%sdesign%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDesign()))));
    }

    // add `firmware_version` to the URL query string
    if (getFirmwareVersion() != null) {
      joiner.add(String.format("%sfirmware_version%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getFirmwareVersion()))));
    }

    // add `hardware_type` to the URL query string
    if (getHardwareType() != null) {
      joiner.add(String.format("%shardware_type%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getHardwareType()))));
    }

    // add `set_up_at` to the URL query string
    if (getSetUpAt() != null) {
      joiner.add(String.format("%sset_up_at%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSetUpAt()))));
    }

    // add `size` to the URL query string
    if (getSize() != null) {
      joiner.add(String.format("%ssize%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSize()))));
    }

    return joiner.toString();
  }
}

