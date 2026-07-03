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
 * PersonalInfoResponse
 */
@JsonPropertyOrder({
  PersonalInfoResponse.JSON_PROPERTY_ID,
  PersonalInfoResponse.JSON_PROPERTY_AGE,
  PersonalInfoResponse.JSON_PROPERTY_WEIGHT,
  PersonalInfoResponse.JSON_PROPERTY_HEIGHT,
  PersonalInfoResponse.JSON_PROPERTY_BIOLOGICAL_SEX,
  PersonalInfoResponse.JSON_PROPERTY_EMAIL
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PersonalInfoResponse {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_AGE = "age";
  private JsonNullable<Integer> age = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_WEIGHT = "weight";
  private JsonNullable<BigDecimal> weight = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_HEIGHT = "height";
  private JsonNullable<BigDecimal> height = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_BIOLOGICAL_SEX = "biological_sex";
  private JsonNullable<String> biologicalSex = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_EMAIL = "email";
  private JsonNullable<String> email = JsonNullable.<String>undefined();

  public PersonalInfoResponse() { 
  }

  public PersonalInfoResponse id(@javax.annotation.Nonnull String id) {
    this.id = id;
    return this;
  }

  /**
   * Get id
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


  public PersonalInfoResponse age(@javax.annotation.Nullable Integer age) {
    this.age = JsonNullable.<Integer>of(age);
    return this;
  }

  /**
   * Get age
   * @return age
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getAge() {
        return age.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_AGE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getAge_JsonNullable() {
    return age;
  }
  
  @JsonProperty(JSON_PROPERTY_AGE)
  public void setAge_JsonNullable(JsonNullable<Integer> age) {
    this.age = age;
  }

  public void setAge(@javax.annotation.Nullable Integer age) {
    this.age = JsonNullable.<Integer>of(age);
  }


  public PersonalInfoResponse weight(@javax.annotation.Nullable BigDecimal weight) {
    this.weight = JsonNullable.<BigDecimal>of(weight);
    return this;
  }

  /**
   * Get weight
   * @return weight
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getWeight() {
        return weight.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_WEIGHT)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getWeight_JsonNullable() {
    return weight;
  }
  
  @JsonProperty(JSON_PROPERTY_WEIGHT)
  public void setWeight_JsonNullable(JsonNullable<BigDecimal> weight) {
    this.weight = weight;
  }

  public void setWeight(@javax.annotation.Nullable BigDecimal weight) {
    this.weight = JsonNullable.<BigDecimal>of(weight);
  }


  public PersonalInfoResponse height(@javax.annotation.Nullable BigDecimal height) {
    this.height = JsonNullable.<BigDecimal>of(height);
    return this;
  }

  /**
   * Get height
   * @return height
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getHeight() {
        return height.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_HEIGHT)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getHeight_JsonNullable() {
    return height;
  }
  
  @JsonProperty(JSON_PROPERTY_HEIGHT)
  public void setHeight_JsonNullable(JsonNullable<BigDecimal> height) {
    this.height = height;
  }

  public void setHeight(@javax.annotation.Nullable BigDecimal height) {
    this.height = JsonNullable.<BigDecimal>of(height);
  }


  public PersonalInfoResponse biologicalSex(@javax.annotation.Nullable String biologicalSex) {
    this.biologicalSex = JsonNullable.<String>of(biologicalSex);
    return this;
  }

  /**
   * Get biologicalSex
   * @return biologicalSex
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getBiologicalSex() {
        return biologicalSex.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_BIOLOGICAL_SEX)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getBiologicalSex_JsonNullable() {
    return biologicalSex;
  }
  
  @JsonProperty(JSON_PROPERTY_BIOLOGICAL_SEX)
  public void setBiologicalSex_JsonNullable(JsonNullable<String> biologicalSex) {
    this.biologicalSex = biologicalSex;
  }

  public void setBiologicalSex(@javax.annotation.Nullable String biologicalSex) {
    this.biologicalSex = JsonNullable.<String>of(biologicalSex);
  }


  public PersonalInfoResponse email(@javax.annotation.Nullable String email) {
    this.email = JsonNullable.<String>of(email);
    return this;
  }

  /**
   * Get email
   * @return email
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getEmail() {
        return email.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_EMAIL)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getEmail_JsonNullable() {
    return email;
  }
  
  @JsonProperty(JSON_PROPERTY_EMAIL)
  public void setEmail_JsonNullable(JsonNullable<String> email) {
    this.email = email;
  }

  public void setEmail(@javax.annotation.Nullable String email) {
    this.email = JsonNullable.<String>of(email);
  }


  /**
   * Return true if this PersonalInfoResponse object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PersonalInfoResponse personalInfoResponse = (PersonalInfoResponse) o;
    return Objects.equals(this.id, personalInfoResponse.id) &&
        equalsNullable(this.age, personalInfoResponse.age) &&
        equalsNullable(this.weight, personalInfoResponse.weight) &&
        equalsNullable(this.height, personalInfoResponse.height) &&
        equalsNullable(this.biologicalSex, personalInfoResponse.biologicalSex) &&
        equalsNullable(this.email, personalInfoResponse.email);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, hashCodeNullable(age), hashCodeNullable(weight), hashCodeNullable(height), hashCodeNullable(biologicalSex), hashCodeNullable(email));
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
    sb.append("class PersonalInfoResponse {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    age: ").append(toIndentedString(age)).append("\n");
    sb.append("    weight: ").append(toIndentedString(weight)).append("\n");
    sb.append("    height: ").append(toIndentedString(height)).append("\n");
    sb.append("    biologicalSex: ").append(toIndentedString(biologicalSex)).append("\n");
    sb.append("    email: ").append(toIndentedString(email)).append("\n");
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

    // add `age` to the URL query string
    if (getAge() != null) {
      joiner.add(String.format("%sage%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getAge()))));
    }

    // add `weight` to the URL query string
    if (getWeight() != null) {
      joiner.add(String.format("%sweight%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getWeight()))));
    }

    // add `height` to the URL query string
    if (getHeight() != null) {
      joiner.add(String.format("%sheight%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getHeight()))));
    }

    // add `biological_sex` to the URL query string
    if (getBiologicalSex() != null) {
      joiner.add(String.format("%sbiological_sex%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getBiologicalSex()))));
    }

    // add `email` to the URL query string
    if (getEmail() != null) {
      joiner.add(String.format("%semail%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEmail()))));
    }

    return joiner.toString();
  }
}

