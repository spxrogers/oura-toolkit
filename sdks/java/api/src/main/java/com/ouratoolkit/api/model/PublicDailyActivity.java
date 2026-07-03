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
import com.ouratoolkit.api.model.PublicActivityContributors;
import com.ouratoolkit.api.model.PublicSample;
import java.math.BigDecimal;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * Object defining a daily activity that is a 24-hour period starting at 4 a.m.
 */
@JsonPropertyOrder({
  PublicDailyActivity.JSON_PROPERTY_ID,
  PublicDailyActivity.JSON_PROPERTY_ACTIVE_CALORIES,
  PublicDailyActivity.JSON_PROPERTY_AVERAGE_MET_MINUTES,
  PublicDailyActivity.JSON_PROPERTY_CLASS5_MIN,
  PublicDailyActivity.JSON_PROPERTY_CONTRIBUTORS,
  PublicDailyActivity.JSON_PROPERTY_DAY,
  PublicDailyActivity.JSON_PROPERTY_EQUIVALENT_WALKING_DISTANCE,
  PublicDailyActivity.JSON_PROPERTY_HIGH_ACTIVITY_MET_MINUTES,
  PublicDailyActivity.JSON_PROPERTY_HIGH_ACTIVITY_TIME,
  PublicDailyActivity.JSON_PROPERTY_INACTIVITY_ALERTS,
  PublicDailyActivity.JSON_PROPERTY_LOW_ACTIVITY_MET_MINUTES,
  PublicDailyActivity.JSON_PROPERTY_LOW_ACTIVITY_TIME,
  PublicDailyActivity.JSON_PROPERTY_MEDIUM_ACTIVITY_MET_MINUTES,
  PublicDailyActivity.JSON_PROPERTY_MEDIUM_ACTIVITY_TIME,
  PublicDailyActivity.JSON_PROPERTY_MET,
  PublicDailyActivity.JSON_PROPERTY_METERS_TO_TARGET,
  PublicDailyActivity.JSON_PROPERTY_NON_WEAR_TIME,
  PublicDailyActivity.JSON_PROPERTY_RESTING_TIME,
  PublicDailyActivity.JSON_PROPERTY_SCORE,
  PublicDailyActivity.JSON_PROPERTY_SEDENTARY_MET_MINUTES,
  PublicDailyActivity.JSON_PROPERTY_SEDENTARY_TIME,
  PublicDailyActivity.JSON_PROPERTY_STEPS,
  PublicDailyActivity.JSON_PROPERTY_TARGET_CALORIES,
  PublicDailyActivity.JSON_PROPERTY_TARGET_METERS,
  PublicDailyActivity.JSON_PROPERTY_TIMESTAMP,
  PublicDailyActivity.JSON_PROPERTY_TOTAL_CALORIES
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicDailyActivity {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_ACTIVE_CALORIES = "active_calories";
  @javax.annotation.Nonnull
  private Integer activeCalories;

  public static final String JSON_PROPERTY_AVERAGE_MET_MINUTES = "average_met_minutes";
  @javax.annotation.Nonnull
  private BigDecimal averageMetMinutes;

  public static final String JSON_PROPERTY_CLASS5_MIN = "class_5_min";
  private JsonNullable<String> class5Min = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_CONTRIBUTORS = "contributors";
  @javax.annotation.Nonnull
  private PublicActivityContributors contributors;

  public static final String JSON_PROPERTY_DAY = "day";
  @javax.annotation.Nullable
  private String day;

  public static final String JSON_PROPERTY_EQUIVALENT_WALKING_DISTANCE = "equivalent_walking_distance";
  @javax.annotation.Nonnull
  private Integer equivalentWalkingDistance;

  public static final String JSON_PROPERTY_HIGH_ACTIVITY_MET_MINUTES = "high_activity_met_minutes";
  @javax.annotation.Nonnull
  private Integer highActivityMetMinutes;

  public static final String JSON_PROPERTY_HIGH_ACTIVITY_TIME = "high_activity_time";
  @javax.annotation.Nonnull
  private Integer highActivityTime;

  public static final String JSON_PROPERTY_INACTIVITY_ALERTS = "inactivity_alerts";
  @javax.annotation.Nonnull
  private Integer inactivityAlerts;

  public static final String JSON_PROPERTY_LOW_ACTIVITY_MET_MINUTES = "low_activity_met_minutes";
  @javax.annotation.Nonnull
  private Integer lowActivityMetMinutes;

  public static final String JSON_PROPERTY_LOW_ACTIVITY_TIME = "low_activity_time";
  @javax.annotation.Nonnull
  private Integer lowActivityTime;

  public static final String JSON_PROPERTY_MEDIUM_ACTIVITY_MET_MINUTES = "medium_activity_met_minutes";
  @javax.annotation.Nonnull
  private Integer mediumActivityMetMinutes;

  public static final String JSON_PROPERTY_MEDIUM_ACTIVITY_TIME = "medium_activity_time";
  @javax.annotation.Nonnull
  private Integer mediumActivityTime;

  public static final String JSON_PROPERTY_MET = "met";
  @javax.annotation.Nonnull
  private PublicSample met;

  public static final String JSON_PROPERTY_METERS_TO_TARGET = "meters_to_target";
  @javax.annotation.Nonnull
  private Integer metersToTarget;

  public static final String JSON_PROPERTY_NON_WEAR_TIME = "non_wear_time";
  @javax.annotation.Nonnull
  private Integer nonWearTime;

  public static final String JSON_PROPERTY_RESTING_TIME = "resting_time";
  @javax.annotation.Nonnull
  private Integer restingTime;

  public static final String JSON_PROPERTY_SCORE = "score";
  private JsonNullable<Integer> score = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_SEDENTARY_MET_MINUTES = "sedentary_met_minutes";
  @javax.annotation.Nonnull
  private Integer sedentaryMetMinutes;

  public static final String JSON_PROPERTY_SEDENTARY_TIME = "sedentary_time";
  @javax.annotation.Nonnull
  private Integer sedentaryTime;

  public static final String JSON_PROPERTY_STEPS = "steps";
  @javax.annotation.Nonnull
  private Integer steps;

  public static final String JSON_PROPERTY_TARGET_CALORIES = "target_calories";
  @javax.annotation.Nonnull
  private Integer targetCalories;

  public static final String JSON_PROPERTY_TARGET_METERS = "target_meters";
  @javax.annotation.Nonnull
  private Integer targetMeters;

  public static final String JSON_PROPERTY_TIMESTAMP = "timestamp";
  @javax.annotation.Nullable
  private String timestamp;

  public static final String JSON_PROPERTY_TOTAL_CALORIES = "total_calories";
  @javax.annotation.Nonnull
  private Integer totalCalories;

  public PublicDailyActivity() { 
  }

  public PublicDailyActivity id(@javax.annotation.Nonnull String id) {
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


  public PublicDailyActivity activeCalories(@javax.annotation.Nonnull Integer activeCalories) {
    this.activeCalories = activeCalories;
    return this;
  }

  /**
   * Active calories expended in kilocalories.
   * @return activeCalories
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_ACTIVE_CALORIES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getActiveCalories() {
    return activeCalories;
  }


  @JsonProperty(JSON_PROPERTY_ACTIVE_CALORIES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setActiveCalories(@javax.annotation.Nonnull Integer activeCalories) {
    this.activeCalories = activeCalories;
  }


  public PublicDailyActivity averageMetMinutes(@javax.annotation.Nonnull BigDecimal averageMetMinutes) {
    this.averageMetMinutes = averageMetMinutes;
    return this;
  }

  /**
   * Average MET minutes.
   * @return averageMetMinutes
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_AVERAGE_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public BigDecimal getAverageMetMinutes() {
    return averageMetMinutes;
  }


  @JsonProperty(JSON_PROPERTY_AVERAGE_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setAverageMetMinutes(@javax.annotation.Nonnull BigDecimal averageMetMinutes) {
    this.averageMetMinutes = averageMetMinutes;
  }


  public PublicDailyActivity class5Min(@javax.annotation.Nullable String class5Min) {
    this.class5Min = JsonNullable.<String>of(class5Min);
    return this;
  }

  /**
   * Get class5Min
   * @return class5Min
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getClass5Min() {
        return class5Min.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_CLASS5_MIN)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getClass5Min_JsonNullable() {
    return class5Min;
  }
  
  @JsonProperty(JSON_PROPERTY_CLASS5_MIN)
  public void setClass5Min_JsonNullable(JsonNullable<String> class5Min) {
    this.class5Min = class5Min;
  }

  public void setClass5Min(@javax.annotation.Nullable String class5Min) {
    this.class5Min = JsonNullable.<String>of(class5Min);
  }


  public PublicDailyActivity contributors(@javax.annotation.Nonnull PublicActivityContributors contributors) {
    this.contributors = contributors;
    return this;
  }

  /**
   * Object containing activity score contributors.
   * @return contributors
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_CONTRIBUTORS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public PublicActivityContributors getContributors() {
    return contributors;
  }


  @JsonProperty(JSON_PROPERTY_CONTRIBUTORS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setContributors(@javax.annotation.Nonnull PublicActivityContributors contributors) {
    this.contributors = contributors;
  }


  public PublicDailyActivity day(@javax.annotation.Nullable String day) {
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


  public PublicDailyActivity equivalentWalkingDistance(@javax.annotation.Nonnull Integer equivalentWalkingDistance) {
    this.equivalentWalkingDistance = equivalentWalkingDistance;
    return this;
  }

  /**
   * Equivalent walking distance of energe expenditure in meters.
   * @return equivalentWalkingDistance
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_EQUIVALENT_WALKING_DISTANCE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getEquivalentWalkingDistance() {
    return equivalentWalkingDistance;
  }


  @JsonProperty(JSON_PROPERTY_EQUIVALENT_WALKING_DISTANCE)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setEquivalentWalkingDistance(@javax.annotation.Nonnull Integer equivalentWalkingDistance) {
    this.equivalentWalkingDistance = equivalentWalkingDistance;
  }


  public PublicDailyActivity highActivityMetMinutes(@javax.annotation.Nonnull Integer highActivityMetMinutes) {
    this.highActivityMetMinutes = highActivityMetMinutes;
    return this;
  }

  /**
   * The total METs of each minute classified as high activity.
   * @return highActivityMetMinutes
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_HIGH_ACTIVITY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getHighActivityMetMinutes() {
    return highActivityMetMinutes;
  }


  @JsonProperty(JSON_PROPERTY_HIGH_ACTIVITY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setHighActivityMetMinutes(@javax.annotation.Nonnull Integer highActivityMetMinutes) {
    this.highActivityMetMinutes = highActivityMetMinutes;
  }


  public PublicDailyActivity highActivityTime(@javax.annotation.Nonnull Integer highActivityTime) {
    this.highActivityTime = highActivityTime;
    return this;
  }

  /**
   * The total time in seconds of each minute classified as high activity.
   * @return highActivityTime
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_HIGH_ACTIVITY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getHighActivityTime() {
    return highActivityTime;
  }


  @JsonProperty(JSON_PROPERTY_HIGH_ACTIVITY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setHighActivityTime(@javax.annotation.Nonnull Integer highActivityTime) {
    this.highActivityTime = highActivityTime;
  }


  public PublicDailyActivity inactivityAlerts(@javax.annotation.Nonnull Integer inactivityAlerts) {
    this.inactivityAlerts = inactivityAlerts;
    return this;
  }

  /**
   * Number of inactivity alerts received.
   * @return inactivityAlerts
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_INACTIVITY_ALERTS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getInactivityAlerts() {
    return inactivityAlerts;
  }


  @JsonProperty(JSON_PROPERTY_INACTIVITY_ALERTS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setInactivityAlerts(@javax.annotation.Nonnull Integer inactivityAlerts) {
    this.inactivityAlerts = inactivityAlerts;
  }


  public PublicDailyActivity lowActivityMetMinutes(@javax.annotation.Nonnull Integer lowActivityMetMinutes) {
    this.lowActivityMetMinutes = lowActivityMetMinutes;
    return this;
  }

  /**
   * The total METs of each minute classified as low activity.
   * @return lowActivityMetMinutes
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_LOW_ACTIVITY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getLowActivityMetMinutes() {
    return lowActivityMetMinutes;
  }


  @JsonProperty(JSON_PROPERTY_LOW_ACTIVITY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setLowActivityMetMinutes(@javax.annotation.Nonnull Integer lowActivityMetMinutes) {
    this.lowActivityMetMinutes = lowActivityMetMinutes;
  }


  public PublicDailyActivity lowActivityTime(@javax.annotation.Nonnull Integer lowActivityTime) {
    this.lowActivityTime = lowActivityTime;
    return this;
  }

  /**
   * The total time in seconds of each minute classified as low activity.
   * @return lowActivityTime
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_LOW_ACTIVITY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getLowActivityTime() {
    return lowActivityTime;
  }


  @JsonProperty(JSON_PROPERTY_LOW_ACTIVITY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setLowActivityTime(@javax.annotation.Nonnull Integer lowActivityTime) {
    this.lowActivityTime = lowActivityTime;
  }


  public PublicDailyActivity mediumActivityMetMinutes(@javax.annotation.Nonnull Integer mediumActivityMetMinutes) {
    this.mediumActivityMetMinutes = mediumActivityMetMinutes;
    return this;
  }

  /**
   * The total METs of each minute classified as medium activity.
   * @return mediumActivityMetMinutes
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_MEDIUM_ACTIVITY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getMediumActivityMetMinutes() {
    return mediumActivityMetMinutes;
  }


  @JsonProperty(JSON_PROPERTY_MEDIUM_ACTIVITY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setMediumActivityMetMinutes(@javax.annotation.Nonnull Integer mediumActivityMetMinutes) {
    this.mediumActivityMetMinutes = mediumActivityMetMinutes;
  }


  public PublicDailyActivity mediumActivityTime(@javax.annotation.Nonnull Integer mediumActivityTime) {
    this.mediumActivityTime = mediumActivityTime;
    return this;
  }

  /**
   * The total time in seconds of each minute classified as medium activity.
   * @return mediumActivityTime
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_MEDIUM_ACTIVITY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getMediumActivityTime() {
    return mediumActivityTime;
  }


  @JsonProperty(JSON_PROPERTY_MEDIUM_ACTIVITY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setMediumActivityTime(@javax.annotation.Nonnull Integer mediumActivityTime) {
    this.mediumActivityTime = mediumActivityTime;
  }


  public PublicDailyActivity met(@javax.annotation.Nonnull PublicSample met) {
    this.met = met;
    return this;
  }

  /**
   * Sample containing METs.
   * @return met
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_MET)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public PublicSample getMet() {
    return met;
  }


  @JsonProperty(JSON_PROPERTY_MET)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setMet(@javax.annotation.Nonnull PublicSample met) {
    this.met = met;
  }


  public PublicDailyActivity metersToTarget(@javax.annotation.Nonnull Integer metersToTarget) {
    this.metersToTarget = metersToTarget;
    return this;
  }

  /**
   * Meters remaining to target.
   * @return metersToTarget
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_METERS_TO_TARGET)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getMetersToTarget() {
    return metersToTarget;
  }


  @JsonProperty(JSON_PROPERTY_METERS_TO_TARGET)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setMetersToTarget(@javax.annotation.Nonnull Integer metersToTarget) {
    this.metersToTarget = metersToTarget;
  }


  public PublicDailyActivity nonWearTime(@javax.annotation.Nonnull Integer nonWearTime) {
    this.nonWearTime = nonWearTime;
    return this;
  }

  /**
   * Ring non-wear time in seconds.
   * @return nonWearTime
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_NON_WEAR_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getNonWearTime() {
    return nonWearTime;
  }


  @JsonProperty(JSON_PROPERTY_NON_WEAR_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setNonWearTime(@javax.annotation.Nonnull Integer nonWearTime) {
    this.nonWearTime = nonWearTime;
  }


  public PublicDailyActivity restingTime(@javax.annotation.Nonnull Integer restingTime) {
    this.restingTime = restingTime;
    return this;
  }

  /**
   * Resting time in seconds.
   * @return restingTime
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_RESTING_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getRestingTime() {
    return restingTime;
  }


  @JsonProperty(JSON_PROPERTY_RESTING_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setRestingTime(@javax.annotation.Nonnull Integer restingTime) {
    this.restingTime = restingTime;
  }


  public PublicDailyActivity score(@javax.annotation.Nullable Integer score) {
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


  public PublicDailyActivity sedentaryMetMinutes(@javax.annotation.Nonnull Integer sedentaryMetMinutes) {
    this.sedentaryMetMinutes = sedentaryMetMinutes;
    return this;
  }

  /**
   * Sedentary MET minutes.
   * @return sedentaryMetMinutes
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_SEDENTARY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getSedentaryMetMinutes() {
    return sedentaryMetMinutes;
  }


  @JsonProperty(JSON_PROPERTY_SEDENTARY_MET_MINUTES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setSedentaryMetMinutes(@javax.annotation.Nonnull Integer sedentaryMetMinutes) {
    this.sedentaryMetMinutes = sedentaryMetMinutes;
  }


  public PublicDailyActivity sedentaryTime(@javax.annotation.Nonnull Integer sedentaryTime) {
    this.sedentaryTime = sedentaryTime;
    return this;
  }

  /**
   * Sedentary time in seconds.
   * @return sedentaryTime
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_SEDENTARY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getSedentaryTime() {
    return sedentaryTime;
  }


  @JsonProperty(JSON_PROPERTY_SEDENTARY_TIME)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setSedentaryTime(@javax.annotation.Nonnull Integer sedentaryTime) {
    this.sedentaryTime = sedentaryTime;
  }


  public PublicDailyActivity steps(@javax.annotation.Nonnull Integer steps) {
    this.steps = steps;
    return this;
  }

  /**
   * Total number of steps taken.
   * @return steps
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_STEPS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getSteps() {
    return steps;
  }


  @JsonProperty(JSON_PROPERTY_STEPS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setSteps(@javax.annotation.Nonnull Integer steps) {
    this.steps = steps;
  }


  public PublicDailyActivity targetCalories(@javax.annotation.Nonnull Integer targetCalories) {
    this.targetCalories = targetCalories;
    return this;
  }

  /**
   * Daily activity target in kilocalories.
   * @return targetCalories
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_TARGET_CALORIES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getTargetCalories() {
    return targetCalories;
  }


  @JsonProperty(JSON_PROPERTY_TARGET_CALORIES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setTargetCalories(@javax.annotation.Nonnull Integer targetCalories) {
    this.targetCalories = targetCalories;
  }


  public PublicDailyActivity targetMeters(@javax.annotation.Nonnull Integer targetMeters) {
    this.targetMeters = targetMeters;
    return this;
  }

  /**
   * Daily activity target in meters.
   * @return targetMeters
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_TARGET_METERS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getTargetMeters() {
    return targetMeters;
  }


  @JsonProperty(JSON_PROPERTY_TARGET_METERS)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setTargetMeters(@javax.annotation.Nonnull Integer targetMeters) {
    this.targetMeters = targetMeters;
  }


  public PublicDailyActivity timestamp(@javax.annotation.Nullable String timestamp) {
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


  public PublicDailyActivity totalCalories(@javax.annotation.Nonnull Integer totalCalories) {
    this.totalCalories = totalCalories;
    return this;
  }

  /**
   * Total calories expended in kilocalories.
   * @return totalCalories
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_TOTAL_CALORIES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getTotalCalories() {
    return totalCalories;
  }


  @JsonProperty(JSON_PROPERTY_TOTAL_CALORIES)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setTotalCalories(@javax.annotation.Nonnull Integer totalCalories) {
    this.totalCalories = totalCalories;
  }


  /**
   * Return true if this PublicDailyActivity object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicDailyActivity publicDailyActivity = (PublicDailyActivity) o;
    return Objects.equals(this.id, publicDailyActivity.id) &&
        Objects.equals(this.activeCalories, publicDailyActivity.activeCalories) &&
        Objects.equals(this.averageMetMinutes, publicDailyActivity.averageMetMinutes) &&
        equalsNullable(this.class5Min, publicDailyActivity.class5Min) &&
        Objects.equals(this.contributors, publicDailyActivity.contributors) &&
        Objects.equals(this.day, publicDailyActivity.day) &&
        Objects.equals(this.equivalentWalkingDistance, publicDailyActivity.equivalentWalkingDistance) &&
        Objects.equals(this.highActivityMetMinutes, publicDailyActivity.highActivityMetMinutes) &&
        Objects.equals(this.highActivityTime, publicDailyActivity.highActivityTime) &&
        Objects.equals(this.inactivityAlerts, publicDailyActivity.inactivityAlerts) &&
        Objects.equals(this.lowActivityMetMinutes, publicDailyActivity.lowActivityMetMinutes) &&
        Objects.equals(this.lowActivityTime, publicDailyActivity.lowActivityTime) &&
        Objects.equals(this.mediumActivityMetMinutes, publicDailyActivity.mediumActivityMetMinutes) &&
        Objects.equals(this.mediumActivityTime, publicDailyActivity.mediumActivityTime) &&
        Objects.equals(this.met, publicDailyActivity.met) &&
        Objects.equals(this.metersToTarget, publicDailyActivity.metersToTarget) &&
        Objects.equals(this.nonWearTime, publicDailyActivity.nonWearTime) &&
        Objects.equals(this.restingTime, publicDailyActivity.restingTime) &&
        equalsNullable(this.score, publicDailyActivity.score) &&
        Objects.equals(this.sedentaryMetMinutes, publicDailyActivity.sedentaryMetMinutes) &&
        Objects.equals(this.sedentaryTime, publicDailyActivity.sedentaryTime) &&
        Objects.equals(this.steps, publicDailyActivity.steps) &&
        Objects.equals(this.targetCalories, publicDailyActivity.targetCalories) &&
        Objects.equals(this.targetMeters, publicDailyActivity.targetMeters) &&
        Objects.equals(this.timestamp, publicDailyActivity.timestamp) &&
        Objects.equals(this.totalCalories, publicDailyActivity.totalCalories);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, activeCalories, averageMetMinutes, hashCodeNullable(class5Min), contributors, day, equivalentWalkingDistance, highActivityMetMinutes, highActivityTime, inactivityAlerts, lowActivityMetMinutes, lowActivityTime, mediumActivityMetMinutes, mediumActivityTime, met, metersToTarget, nonWearTime, restingTime, hashCodeNullable(score), sedentaryMetMinutes, sedentaryTime, steps, targetCalories, targetMeters, timestamp, totalCalories);
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
    sb.append("class PublicDailyActivity {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    activeCalories: ").append(toIndentedString(activeCalories)).append("\n");
    sb.append("    averageMetMinutes: ").append(toIndentedString(averageMetMinutes)).append("\n");
    sb.append("    class5Min: ").append(toIndentedString(class5Min)).append("\n");
    sb.append("    contributors: ").append(toIndentedString(contributors)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    equivalentWalkingDistance: ").append(toIndentedString(equivalentWalkingDistance)).append("\n");
    sb.append("    highActivityMetMinutes: ").append(toIndentedString(highActivityMetMinutes)).append("\n");
    sb.append("    highActivityTime: ").append(toIndentedString(highActivityTime)).append("\n");
    sb.append("    inactivityAlerts: ").append(toIndentedString(inactivityAlerts)).append("\n");
    sb.append("    lowActivityMetMinutes: ").append(toIndentedString(lowActivityMetMinutes)).append("\n");
    sb.append("    lowActivityTime: ").append(toIndentedString(lowActivityTime)).append("\n");
    sb.append("    mediumActivityMetMinutes: ").append(toIndentedString(mediumActivityMetMinutes)).append("\n");
    sb.append("    mediumActivityTime: ").append(toIndentedString(mediumActivityTime)).append("\n");
    sb.append("    met: ").append(toIndentedString(met)).append("\n");
    sb.append("    metersToTarget: ").append(toIndentedString(metersToTarget)).append("\n");
    sb.append("    nonWearTime: ").append(toIndentedString(nonWearTime)).append("\n");
    sb.append("    restingTime: ").append(toIndentedString(restingTime)).append("\n");
    sb.append("    score: ").append(toIndentedString(score)).append("\n");
    sb.append("    sedentaryMetMinutes: ").append(toIndentedString(sedentaryMetMinutes)).append("\n");
    sb.append("    sedentaryTime: ").append(toIndentedString(sedentaryTime)).append("\n");
    sb.append("    steps: ").append(toIndentedString(steps)).append("\n");
    sb.append("    targetCalories: ").append(toIndentedString(targetCalories)).append("\n");
    sb.append("    targetMeters: ").append(toIndentedString(targetMeters)).append("\n");
    sb.append("    timestamp: ").append(toIndentedString(timestamp)).append("\n");
    sb.append("    totalCalories: ").append(toIndentedString(totalCalories)).append("\n");
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

    // add `active_calories` to the URL query string
    if (getActiveCalories() != null) {
      joiner.add(String.format("%sactive_calories%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getActiveCalories()))));
    }

    // add `average_met_minutes` to the URL query string
    if (getAverageMetMinutes() != null) {
      joiner.add(String.format("%saverage_met_minutes%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getAverageMetMinutes()))));
    }

    // add `class_5_min` to the URL query string
    if (getClass5Min() != null) {
      joiner.add(String.format("%sclass_5_min%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getClass5Min()))));
    }

    // add `contributors` to the URL query string
    if (getContributors() != null) {
      joiner.add(getContributors().toUrlQueryString(prefix + "contributors" + suffix));
    }

    // add `day` to the URL query string
    if (getDay() != null) {
      joiner.add(String.format("%sday%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDay()))));
    }

    // add `equivalent_walking_distance` to the URL query string
    if (getEquivalentWalkingDistance() != null) {
      joiner.add(String.format("%sequivalent_walking_distance%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEquivalentWalkingDistance()))));
    }

    // add `high_activity_met_minutes` to the URL query string
    if (getHighActivityMetMinutes() != null) {
      joiner.add(String.format("%shigh_activity_met_minutes%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getHighActivityMetMinutes()))));
    }

    // add `high_activity_time` to the URL query string
    if (getHighActivityTime() != null) {
      joiner.add(String.format("%shigh_activity_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getHighActivityTime()))));
    }

    // add `inactivity_alerts` to the URL query string
    if (getInactivityAlerts() != null) {
      joiner.add(String.format("%sinactivity_alerts%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getInactivityAlerts()))));
    }

    // add `low_activity_met_minutes` to the URL query string
    if (getLowActivityMetMinutes() != null) {
      joiner.add(String.format("%slow_activity_met_minutes%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLowActivityMetMinutes()))));
    }

    // add `low_activity_time` to the URL query string
    if (getLowActivityTime() != null) {
      joiner.add(String.format("%slow_activity_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLowActivityTime()))));
    }

    // add `medium_activity_met_minutes` to the URL query string
    if (getMediumActivityMetMinutes() != null) {
      joiner.add(String.format("%smedium_activity_met_minutes%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getMediumActivityMetMinutes()))));
    }

    // add `medium_activity_time` to the URL query string
    if (getMediumActivityTime() != null) {
      joiner.add(String.format("%smedium_activity_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getMediumActivityTime()))));
    }

    // add `met` to the URL query string
    if (getMet() != null) {
      joiner.add(getMet().toUrlQueryString(prefix + "met" + suffix));
    }

    // add `meters_to_target` to the URL query string
    if (getMetersToTarget() != null) {
      joiner.add(String.format("%smeters_to_target%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getMetersToTarget()))));
    }

    // add `non_wear_time` to the URL query string
    if (getNonWearTime() != null) {
      joiner.add(String.format("%snon_wear_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getNonWearTime()))));
    }

    // add `resting_time` to the URL query string
    if (getRestingTime() != null) {
      joiner.add(String.format("%sresting_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRestingTime()))));
    }

    // add `score` to the URL query string
    if (getScore() != null) {
      joiner.add(String.format("%sscore%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getScore()))));
    }

    // add `sedentary_met_minutes` to the URL query string
    if (getSedentaryMetMinutes() != null) {
      joiner.add(String.format("%ssedentary_met_minutes%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSedentaryMetMinutes()))));
    }

    // add `sedentary_time` to the URL query string
    if (getSedentaryTime() != null) {
      joiner.add(String.format("%ssedentary_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSedentaryTime()))));
    }

    // add `steps` to the URL query string
    if (getSteps() != null) {
      joiner.add(String.format("%ssteps%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSteps()))));
    }

    // add `target_calories` to the URL query string
    if (getTargetCalories() != null) {
      joiner.add(String.format("%starget_calories%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTargetCalories()))));
    }

    // add `target_meters` to the URL query string
    if (getTargetMeters() != null) {
      joiner.add(String.format("%starget_meters%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTargetMeters()))));
    }

    // add `timestamp` to the URL query string
    if (getTimestamp() != null) {
      joiner.add(String.format("%stimestamp%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTimestamp()))));
    }

    // add `total_calories` to the URL query string
    if (getTotalCalories() != null) {
      joiner.add(String.format("%stotal_calories%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTotalCalories()))));
    }

    return joiner.toString();
  }
}

