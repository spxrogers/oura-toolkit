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
import com.ouratoolkit.api.model.PublicReadiness;
import com.ouratoolkit.api.model.PublicSample;
import com.ouratoolkit.api.model.PublicSleepAlgorithmVersion;
import com.ouratoolkit.api.model.PublicSleepAnalysisReason;
import com.ouratoolkit.api.model.PublicSleepType;
import java.math.BigDecimal;
import java.util.Arrays;
import org.openapitools.jackson.nullable.JsonNullable;
import com.fasterxml.jackson.annotation.JsonIgnore;
import org.openapitools.jackson.nullable.JsonNullable;
import java.util.NoSuchElementException;
import com.fasterxml.jackson.annotation.JsonPropertyOrder;


import com.ouratoolkit.api.ApiClient;
/**
 * PublicModifiedSleepModel
 */
@JsonPropertyOrder({
  PublicModifiedSleepModel.JSON_PROPERTY_ID,
  PublicModifiedSleepModel.JSON_PROPERTY_AVERAGE_BREATH,
  PublicModifiedSleepModel.JSON_PROPERTY_AVERAGE_HEART_RATE,
  PublicModifiedSleepModel.JSON_PROPERTY_AVERAGE_HRV,
  PublicModifiedSleepModel.JSON_PROPERTY_AWAKE_TIME,
  PublicModifiedSleepModel.JSON_PROPERTY_BEDTIME_END,
  PublicModifiedSleepModel.JSON_PROPERTY_BEDTIME_START,
  PublicModifiedSleepModel.JSON_PROPERTY_DAY,
  PublicModifiedSleepModel.JSON_PROPERTY_DEEP_SLEEP_DURATION,
  PublicModifiedSleepModel.JSON_PROPERTY_EFFICIENCY,
  PublicModifiedSleepModel.JSON_PROPERTY_HEART_RATE,
  PublicModifiedSleepModel.JSON_PROPERTY_HRV,
  PublicModifiedSleepModel.JSON_PROPERTY_LATENCY,
  PublicModifiedSleepModel.JSON_PROPERTY_LIGHT_SLEEP_DURATION,
  PublicModifiedSleepModel.JSON_PROPERTY_LOW_BATTERY_ALERT,
  PublicModifiedSleepModel.JSON_PROPERTY_LOWEST_HEART_RATE,
  PublicModifiedSleepModel.JSON_PROPERTY_MOVEMENT30_SEC,
  PublicModifiedSleepModel.JSON_PROPERTY_PERIOD,
  PublicModifiedSleepModel.JSON_PROPERTY_READINESS,
  PublicModifiedSleepModel.JSON_PROPERTY_READINESS_SCORE_DELTA,
  PublicModifiedSleepModel.JSON_PROPERTY_REM_SLEEP_DURATION,
  PublicModifiedSleepModel.JSON_PROPERTY_RESTLESS_PERIODS,
  PublicModifiedSleepModel.JSON_PROPERTY_SLEEP_ALGORITHM_VERSION,
  PublicModifiedSleepModel.JSON_PROPERTY_SLEEP_ANALYSIS_REASON,
  PublicModifiedSleepModel.JSON_PROPERTY_SLEEP_PHASE30_SEC,
  PublicModifiedSleepModel.JSON_PROPERTY_SLEEP_PHASE5_MIN,
  PublicModifiedSleepModel.JSON_PROPERTY_SLEEP_SCORE_DELTA,
  PublicModifiedSleepModel.JSON_PROPERTY_TIME_IN_BED,
  PublicModifiedSleepModel.JSON_PROPERTY_TOTAL_SLEEP_DURATION,
  PublicModifiedSleepModel.JSON_PROPERTY_TYPE,
  PublicModifiedSleepModel.JSON_PROPERTY_RING_ID,
  PublicModifiedSleepModel.JSON_PROPERTY_APP_SLEEP_PHASE5_MIN
})
@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class PublicModifiedSleepModel {
  public static final String JSON_PROPERTY_ID = "id";
  @javax.annotation.Nonnull
  private String id;

  public static final String JSON_PROPERTY_AVERAGE_BREATH = "average_breath";
  private JsonNullable<BigDecimal> averageBreath = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_AVERAGE_HEART_RATE = "average_heart_rate";
  private JsonNullable<BigDecimal> averageHeartRate = JsonNullable.<BigDecimal>undefined();

  public static final String JSON_PROPERTY_AVERAGE_HRV = "average_hrv";
  private JsonNullable<Integer> averageHrv = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_AWAKE_TIME = "awake_time";
  private JsonNullable<Integer> awakeTime = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_BEDTIME_END = "bedtime_end";
  @javax.annotation.Nullable
  private String bedtimeEnd;

  public static final String JSON_PROPERTY_BEDTIME_START = "bedtime_start";
  @javax.annotation.Nullable
  private String bedtimeStart;

  public static final String JSON_PROPERTY_DAY = "day";
  @javax.annotation.Nullable
  private String day;

  public static final String JSON_PROPERTY_DEEP_SLEEP_DURATION = "deep_sleep_duration";
  private JsonNullable<Integer> deepSleepDuration = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_EFFICIENCY = "efficiency";
  private JsonNullable<Integer> efficiency = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_HEART_RATE = "heart_rate";
  private JsonNullable<PublicSample> heartRate = JsonNullable.<PublicSample>undefined();

  public static final String JSON_PROPERTY_HRV = "hrv";
  private JsonNullable<PublicSample> hrv = JsonNullable.<PublicSample>undefined();

  public static final String JSON_PROPERTY_LATENCY = "latency";
  private JsonNullable<Integer> latency = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_LIGHT_SLEEP_DURATION = "light_sleep_duration";
  private JsonNullable<Integer> lightSleepDuration = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_LOW_BATTERY_ALERT = "low_battery_alert";
  @javax.annotation.Nonnull
  private Boolean lowBatteryAlert;

  public static final String JSON_PROPERTY_LOWEST_HEART_RATE = "lowest_heart_rate";
  private JsonNullable<Integer> lowestHeartRate = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_MOVEMENT30_SEC = "movement_30_sec";
  private JsonNullable<String> movement30Sec = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_PERIOD = "period";
  @javax.annotation.Nonnull
  private Integer period;

  public static final String JSON_PROPERTY_READINESS = "readiness";
  private JsonNullable<PublicReadiness> readiness = JsonNullable.<PublicReadiness>undefined();

  public static final String JSON_PROPERTY_READINESS_SCORE_DELTA = "readiness_score_delta";
  private JsonNullable<Integer> readinessScoreDelta = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_REM_SLEEP_DURATION = "rem_sleep_duration";
  private JsonNullable<Integer> remSleepDuration = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_RESTLESS_PERIODS = "restless_periods";
  private JsonNullable<Integer> restlessPeriods = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_SLEEP_ALGORITHM_VERSION = "sleep_algorithm_version";
  private JsonNullable<PublicSleepAlgorithmVersion> sleepAlgorithmVersion = JsonNullable.<PublicSleepAlgorithmVersion>undefined();

  public static final String JSON_PROPERTY_SLEEP_ANALYSIS_REASON = "sleep_analysis_reason";
  private JsonNullable<PublicSleepAnalysisReason> sleepAnalysisReason = JsonNullable.<PublicSleepAnalysisReason>undefined();

  public static final String JSON_PROPERTY_SLEEP_PHASE30_SEC = "sleep_phase_30_sec";
  private JsonNullable<String> sleepPhase30Sec = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_SLEEP_PHASE5_MIN = "sleep_phase_5_min";
  private JsonNullable<String> sleepPhase5Min = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_SLEEP_SCORE_DELTA = "sleep_score_delta";
  private JsonNullable<Integer> sleepScoreDelta = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_TIME_IN_BED = "time_in_bed";
  @javax.annotation.Nonnull
  private Integer timeInBed;

  public static final String JSON_PROPERTY_TOTAL_SLEEP_DURATION = "total_sleep_duration";
  private JsonNullable<Integer> totalSleepDuration = JsonNullable.<Integer>undefined();

  public static final String JSON_PROPERTY_TYPE = "type";
  private JsonNullable<PublicSleepType> type = JsonNullable.<PublicSleepType>undefined();

  public static final String JSON_PROPERTY_RING_ID = "ring_id";
  private JsonNullable<String> ringId = JsonNullable.<String>undefined();

  public static final String JSON_PROPERTY_APP_SLEEP_PHASE5_MIN = "app_sleep_phase_5_min";
  private JsonNullable<String> appSleepPhase5Min = JsonNullable.<String>undefined();

  public PublicModifiedSleepModel() { 
  }

  public PublicModifiedSleepModel id(@javax.annotation.Nonnull String id) {
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


  public PublicModifiedSleepModel averageBreath(@javax.annotation.Nullable BigDecimal averageBreath) {
    this.averageBreath = JsonNullable.<BigDecimal>of(averageBreath);
    return this;
  }

  /**
   * Get averageBreath
   * @return averageBreath
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getAverageBreath() {
        return averageBreath.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_AVERAGE_BREATH)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getAverageBreath_JsonNullable() {
    return averageBreath;
  }
  
  @JsonProperty(JSON_PROPERTY_AVERAGE_BREATH)
  public void setAverageBreath_JsonNullable(JsonNullable<BigDecimal> averageBreath) {
    this.averageBreath = averageBreath;
  }

  public void setAverageBreath(@javax.annotation.Nullable BigDecimal averageBreath) {
    this.averageBreath = JsonNullable.<BigDecimal>of(averageBreath);
  }


  public PublicModifiedSleepModel averageHeartRate(@javax.annotation.Nullable BigDecimal averageHeartRate) {
    this.averageHeartRate = JsonNullable.<BigDecimal>of(averageHeartRate);
    return this;
  }

  /**
   * Get averageHeartRate
   * @return averageHeartRate
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public BigDecimal getAverageHeartRate() {
        return averageHeartRate.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_AVERAGE_HEART_RATE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<BigDecimal> getAverageHeartRate_JsonNullable() {
    return averageHeartRate;
  }
  
  @JsonProperty(JSON_PROPERTY_AVERAGE_HEART_RATE)
  public void setAverageHeartRate_JsonNullable(JsonNullable<BigDecimal> averageHeartRate) {
    this.averageHeartRate = averageHeartRate;
  }

  public void setAverageHeartRate(@javax.annotation.Nullable BigDecimal averageHeartRate) {
    this.averageHeartRate = JsonNullable.<BigDecimal>of(averageHeartRate);
  }


  public PublicModifiedSleepModel averageHrv(@javax.annotation.Nullable Integer averageHrv) {
    this.averageHrv = JsonNullable.<Integer>of(averageHrv);
    return this;
  }

  /**
   * Get averageHrv
   * @return averageHrv
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getAverageHrv() {
        return averageHrv.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_AVERAGE_HRV)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getAverageHrv_JsonNullable() {
    return averageHrv;
  }
  
  @JsonProperty(JSON_PROPERTY_AVERAGE_HRV)
  public void setAverageHrv_JsonNullable(JsonNullable<Integer> averageHrv) {
    this.averageHrv = averageHrv;
  }

  public void setAverageHrv(@javax.annotation.Nullable Integer averageHrv) {
    this.averageHrv = JsonNullable.<Integer>of(averageHrv);
  }


  public PublicModifiedSleepModel awakeTime(@javax.annotation.Nullable Integer awakeTime) {
    this.awakeTime = JsonNullable.<Integer>of(awakeTime);
    return this;
  }

  /**
   * Get awakeTime
   * @return awakeTime
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getAwakeTime() {
        return awakeTime.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_AWAKE_TIME)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getAwakeTime_JsonNullable() {
    return awakeTime;
  }
  
  @JsonProperty(JSON_PROPERTY_AWAKE_TIME)
  public void setAwakeTime_JsonNullable(JsonNullable<Integer> awakeTime) {
    this.awakeTime = awakeTime;
  }

  public void setAwakeTime(@javax.annotation.Nullable Integer awakeTime) {
    this.awakeTime = JsonNullable.<Integer>of(awakeTime);
  }


  public PublicModifiedSleepModel bedtimeEnd(@javax.annotation.Nullable String bedtimeEnd) {
    this.bedtimeEnd = bedtimeEnd;
    return this;
  }

  /**
   * Get bedtimeEnd
   * @return bedtimeEnd
   */
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_BEDTIME_END)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getBedtimeEnd() {
    return bedtimeEnd;
  }


  @JsonProperty(JSON_PROPERTY_BEDTIME_END)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setBedtimeEnd(@javax.annotation.Nullable String bedtimeEnd) {
    this.bedtimeEnd = bedtimeEnd;
  }


  public PublicModifiedSleepModel bedtimeStart(@javax.annotation.Nullable String bedtimeStart) {
    this.bedtimeStart = bedtimeStart;
    return this;
  }

  /**
   * Get bedtimeStart
   * @return bedtimeStart
   */
  @javax.annotation.Nullable
  @JsonProperty(JSON_PROPERTY_BEDTIME_START)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public String getBedtimeStart() {
    return bedtimeStart;
  }


  @JsonProperty(JSON_PROPERTY_BEDTIME_START)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setBedtimeStart(@javax.annotation.Nullable String bedtimeStart) {
    this.bedtimeStart = bedtimeStart;
  }


  public PublicModifiedSleepModel day(@javax.annotation.Nullable String day) {
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


  public PublicModifiedSleepModel deepSleepDuration(@javax.annotation.Nullable Integer deepSleepDuration) {
    this.deepSleepDuration = JsonNullable.<Integer>of(deepSleepDuration);
    return this;
  }

  /**
   * Get deepSleepDuration
   * @return deepSleepDuration
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getDeepSleepDuration() {
        return deepSleepDuration.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_DEEP_SLEEP_DURATION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getDeepSleepDuration_JsonNullable() {
    return deepSleepDuration;
  }
  
  @JsonProperty(JSON_PROPERTY_DEEP_SLEEP_DURATION)
  public void setDeepSleepDuration_JsonNullable(JsonNullable<Integer> deepSleepDuration) {
    this.deepSleepDuration = deepSleepDuration;
  }

  public void setDeepSleepDuration(@javax.annotation.Nullable Integer deepSleepDuration) {
    this.deepSleepDuration = JsonNullable.<Integer>of(deepSleepDuration);
  }


  public PublicModifiedSleepModel efficiency(@javax.annotation.Nullable Integer efficiency) {
    this.efficiency = JsonNullable.<Integer>of(efficiency);
    return this;
  }

  /**
   * Get efficiency
   * @return efficiency
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getEfficiency() {
        return efficiency.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_EFFICIENCY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getEfficiency_JsonNullable() {
    return efficiency;
  }
  
  @JsonProperty(JSON_PROPERTY_EFFICIENCY)
  public void setEfficiency_JsonNullable(JsonNullable<Integer> efficiency) {
    this.efficiency = efficiency;
  }

  public void setEfficiency(@javax.annotation.Nullable Integer efficiency) {
    this.efficiency = JsonNullable.<Integer>of(efficiency);
  }


  public PublicModifiedSleepModel heartRate(@javax.annotation.Nullable PublicSample heartRate) {
    this.heartRate = JsonNullable.<PublicSample>of(heartRate);
    return this;
  }

  /**
   * Get heartRate
   * @return heartRate
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSample getHeartRate() {
        return heartRate.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_HEART_RATE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSample> getHeartRate_JsonNullable() {
    return heartRate;
  }
  
  @JsonProperty(JSON_PROPERTY_HEART_RATE)
  public void setHeartRate_JsonNullable(JsonNullable<PublicSample> heartRate) {
    this.heartRate = heartRate;
  }

  public void setHeartRate(@javax.annotation.Nullable PublicSample heartRate) {
    this.heartRate = JsonNullable.<PublicSample>of(heartRate);
  }


  public PublicModifiedSleepModel hrv(@javax.annotation.Nullable PublicSample hrv) {
    this.hrv = JsonNullable.<PublicSample>of(hrv);
    return this;
  }

  /**
   * Get hrv
   * @return hrv
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSample getHrv() {
        return hrv.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_HRV)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSample> getHrv_JsonNullable() {
    return hrv;
  }
  
  @JsonProperty(JSON_PROPERTY_HRV)
  public void setHrv_JsonNullable(JsonNullable<PublicSample> hrv) {
    this.hrv = hrv;
  }

  public void setHrv(@javax.annotation.Nullable PublicSample hrv) {
    this.hrv = JsonNullable.<PublicSample>of(hrv);
  }


  public PublicModifiedSleepModel latency(@javax.annotation.Nullable Integer latency) {
    this.latency = JsonNullable.<Integer>of(latency);
    return this;
  }

  /**
   * Get latency
   * @return latency
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getLatency() {
        return latency.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_LATENCY)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getLatency_JsonNullable() {
    return latency;
  }
  
  @JsonProperty(JSON_PROPERTY_LATENCY)
  public void setLatency_JsonNullable(JsonNullable<Integer> latency) {
    this.latency = latency;
  }

  public void setLatency(@javax.annotation.Nullable Integer latency) {
    this.latency = JsonNullable.<Integer>of(latency);
  }


  public PublicModifiedSleepModel lightSleepDuration(@javax.annotation.Nullable Integer lightSleepDuration) {
    this.lightSleepDuration = JsonNullable.<Integer>of(lightSleepDuration);
    return this;
  }

  /**
   * Get lightSleepDuration
   * @return lightSleepDuration
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getLightSleepDuration() {
        return lightSleepDuration.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_LIGHT_SLEEP_DURATION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getLightSleepDuration_JsonNullable() {
    return lightSleepDuration;
  }
  
  @JsonProperty(JSON_PROPERTY_LIGHT_SLEEP_DURATION)
  public void setLightSleepDuration_JsonNullable(JsonNullable<Integer> lightSleepDuration) {
    this.lightSleepDuration = lightSleepDuration;
  }

  public void setLightSleepDuration(@javax.annotation.Nullable Integer lightSleepDuration) {
    this.lightSleepDuration = JsonNullable.<Integer>of(lightSleepDuration);
  }


  public PublicModifiedSleepModel lowBatteryAlert(@javax.annotation.Nonnull Boolean lowBatteryAlert) {
    this.lowBatteryAlert = lowBatteryAlert;
    return this;
  }

  /**
   * Flag indicating if a low battery alert occurred.
   * @return lowBatteryAlert
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_LOW_BATTERY_ALERT)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Boolean getLowBatteryAlert() {
    return lowBatteryAlert;
  }


  @JsonProperty(JSON_PROPERTY_LOW_BATTERY_ALERT)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setLowBatteryAlert(@javax.annotation.Nonnull Boolean lowBatteryAlert) {
    this.lowBatteryAlert = lowBatteryAlert;
  }


  public PublicModifiedSleepModel lowestHeartRate(@javax.annotation.Nullable Integer lowestHeartRate) {
    this.lowestHeartRate = JsonNullable.<Integer>of(lowestHeartRate);
    return this;
  }

  /**
   * Get lowestHeartRate
   * @return lowestHeartRate
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getLowestHeartRate() {
        return lowestHeartRate.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_LOWEST_HEART_RATE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getLowestHeartRate_JsonNullable() {
    return lowestHeartRate;
  }
  
  @JsonProperty(JSON_PROPERTY_LOWEST_HEART_RATE)
  public void setLowestHeartRate_JsonNullable(JsonNullable<Integer> lowestHeartRate) {
    this.lowestHeartRate = lowestHeartRate;
  }

  public void setLowestHeartRate(@javax.annotation.Nullable Integer lowestHeartRate) {
    this.lowestHeartRate = JsonNullable.<Integer>of(lowestHeartRate);
  }


  public PublicModifiedSleepModel movement30Sec(@javax.annotation.Nullable String movement30Sec) {
    this.movement30Sec = JsonNullable.<String>of(movement30Sec);
    return this;
  }

  /**
   * Get movement30Sec
   * @return movement30Sec
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getMovement30Sec() {
        return movement30Sec.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_MOVEMENT30_SEC)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getMovement30Sec_JsonNullable() {
    return movement30Sec;
  }
  
  @JsonProperty(JSON_PROPERTY_MOVEMENT30_SEC)
  public void setMovement30Sec_JsonNullable(JsonNullable<String> movement30Sec) {
    this.movement30Sec = movement30Sec;
  }

  public void setMovement30Sec(@javax.annotation.Nullable String movement30Sec) {
    this.movement30Sec = JsonNullable.<String>of(movement30Sec);
  }


  public PublicModifiedSleepModel period(@javax.annotation.Nonnull Integer period) {
    this.period = period;
    return this;
  }

  /**
   * ECore sleep period identifier.
   * @return period
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_PERIOD)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getPeriod() {
    return period;
  }


  @JsonProperty(JSON_PROPERTY_PERIOD)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setPeriod(@javax.annotation.Nonnull Integer period) {
    this.period = period;
  }


  public PublicModifiedSleepModel readiness(@javax.annotation.Nullable PublicReadiness readiness) {
    this.readiness = JsonNullable.<PublicReadiness>of(readiness);
    return this;
  }

  /**
   * Get readiness
   * @return readiness
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicReadiness getReadiness() {
        return readiness.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_READINESS)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicReadiness> getReadiness_JsonNullable() {
    return readiness;
  }
  
  @JsonProperty(JSON_PROPERTY_READINESS)
  public void setReadiness_JsonNullable(JsonNullable<PublicReadiness> readiness) {
    this.readiness = readiness;
  }

  public void setReadiness(@javax.annotation.Nullable PublicReadiness readiness) {
    this.readiness = JsonNullable.<PublicReadiness>of(readiness);
  }


  public PublicModifiedSleepModel readinessScoreDelta(@javax.annotation.Nullable Integer readinessScoreDelta) {
    this.readinessScoreDelta = JsonNullable.<Integer>of(readinessScoreDelta);
    return this;
  }

  /**
   * Get readinessScoreDelta
   * @return readinessScoreDelta
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getReadinessScoreDelta() {
        return readinessScoreDelta.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_READINESS_SCORE_DELTA)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getReadinessScoreDelta_JsonNullable() {
    return readinessScoreDelta;
  }
  
  @JsonProperty(JSON_PROPERTY_READINESS_SCORE_DELTA)
  public void setReadinessScoreDelta_JsonNullable(JsonNullable<Integer> readinessScoreDelta) {
    this.readinessScoreDelta = readinessScoreDelta;
  }

  public void setReadinessScoreDelta(@javax.annotation.Nullable Integer readinessScoreDelta) {
    this.readinessScoreDelta = JsonNullable.<Integer>of(readinessScoreDelta);
  }


  public PublicModifiedSleepModel remSleepDuration(@javax.annotation.Nullable Integer remSleepDuration) {
    this.remSleepDuration = JsonNullable.<Integer>of(remSleepDuration);
    return this;
  }

  /**
   * Get remSleepDuration
   * @return remSleepDuration
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getRemSleepDuration() {
        return remSleepDuration.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_REM_SLEEP_DURATION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getRemSleepDuration_JsonNullable() {
    return remSleepDuration;
  }
  
  @JsonProperty(JSON_PROPERTY_REM_SLEEP_DURATION)
  public void setRemSleepDuration_JsonNullable(JsonNullable<Integer> remSleepDuration) {
    this.remSleepDuration = remSleepDuration;
  }

  public void setRemSleepDuration(@javax.annotation.Nullable Integer remSleepDuration) {
    this.remSleepDuration = JsonNullable.<Integer>of(remSleepDuration);
  }


  public PublicModifiedSleepModel restlessPeriods(@javax.annotation.Nullable Integer restlessPeriods) {
    this.restlessPeriods = JsonNullable.<Integer>of(restlessPeriods);
    return this;
  }

  /**
   * Get restlessPeriods
   * @return restlessPeriods
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getRestlessPeriods() {
        return restlessPeriods.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_RESTLESS_PERIODS)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getRestlessPeriods_JsonNullable() {
    return restlessPeriods;
  }
  
  @JsonProperty(JSON_PROPERTY_RESTLESS_PERIODS)
  public void setRestlessPeriods_JsonNullable(JsonNullable<Integer> restlessPeriods) {
    this.restlessPeriods = restlessPeriods;
  }

  public void setRestlessPeriods(@javax.annotation.Nullable Integer restlessPeriods) {
    this.restlessPeriods = JsonNullable.<Integer>of(restlessPeriods);
  }


  public PublicModifiedSleepModel sleepAlgorithmVersion(@javax.annotation.Nullable PublicSleepAlgorithmVersion sleepAlgorithmVersion) {
    this.sleepAlgorithmVersion = JsonNullable.<PublicSleepAlgorithmVersion>of(sleepAlgorithmVersion);
    return this;
  }

  /**
   * Get sleepAlgorithmVersion
   * @return sleepAlgorithmVersion
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSleepAlgorithmVersion getSleepAlgorithmVersion() {
        return sleepAlgorithmVersion.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SLEEP_ALGORITHM_VERSION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSleepAlgorithmVersion> getSleepAlgorithmVersion_JsonNullable() {
    return sleepAlgorithmVersion;
  }
  
  @JsonProperty(JSON_PROPERTY_SLEEP_ALGORITHM_VERSION)
  public void setSleepAlgorithmVersion_JsonNullable(JsonNullable<PublicSleepAlgorithmVersion> sleepAlgorithmVersion) {
    this.sleepAlgorithmVersion = sleepAlgorithmVersion;
  }

  public void setSleepAlgorithmVersion(@javax.annotation.Nullable PublicSleepAlgorithmVersion sleepAlgorithmVersion) {
    this.sleepAlgorithmVersion = JsonNullable.<PublicSleepAlgorithmVersion>of(sleepAlgorithmVersion);
  }


  public PublicModifiedSleepModel sleepAnalysisReason(@javax.annotation.Nullable PublicSleepAnalysisReason sleepAnalysisReason) {
    this.sleepAnalysisReason = JsonNullable.<PublicSleepAnalysisReason>of(sleepAnalysisReason);
    return this;
  }

  /**
   * Get sleepAnalysisReason
   * @return sleepAnalysisReason
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSleepAnalysisReason getSleepAnalysisReason() {
        return sleepAnalysisReason.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SLEEP_ANALYSIS_REASON)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSleepAnalysisReason> getSleepAnalysisReason_JsonNullable() {
    return sleepAnalysisReason;
  }
  
  @JsonProperty(JSON_PROPERTY_SLEEP_ANALYSIS_REASON)
  public void setSleepAnalysisReason_JsonNullable(JsonNullable<PublicSleepAnalysisReason> sleepAnalysisReason) {
    this.sleepAnalysisReason = sleepAnalysisReason;
  }

  public void setSleepAnalysisReason(@javax.annotation.Nullable PublicSleepAnalysisReason sleepAnalysisReason) {
    this.sleepAnalysisReason = JsonNullable.<PublicSleepAnalysisReason>of(sleepAnalysisReason);
  }


  public PublicModifiedSleepModel sleepPhase30Sec(@javax.annotation.Nullable String sleepPhase30Sec) {
    this.sleepPhase30Sec = JsonNullable.<String>of(sleepPhase30Sec);
    return this;
  }

  /**
   * Get sleepPhase30Sec
   * @return sleepPhase30Sec
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getSleepPhase30Sec() {
        return sleepPhase30Sec.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SLEEP_PHASE30_SEC)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getSleepPhase30Sec_JsonNullable() {
    return sleepPhase30Sec;
  }
  
  @JsonProperty(JSON_PROPERTY_SLEEP_PHASE30_SEC)
  public void setSleepPhase30Sec_JsonNullable(JsonNullable<String> sleepPhase30Sec) {
    this.sleepPhase30Sec = sleepPhase30Sec;
  }

  public void setSleepPhase30Sec(@javax.annotation.Nullable String sleepPhase30Sec) {
    this.sleepPhase30Sec = JsonNullable.<String>of(sleepPhase30Sec);
  }


  public PublicModifiedSleepModel sleepPhase5Min(@javax.annotation.Nullable String sleepPhase5Min) {
    this.sleepPhase5Min = JsonNullable.<String>of(sleepPhase5Min);
    return this;
  }

  /**
   * Get sleepPhase5Min
   * @return sleepPhase5Min
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getSleepPhase5Min() {
        return sleepPhase5Min.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SLEEP_PHASE5_MIN)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getSleepPhase5Min_JsonNullable() {
    return sleepPhase5Min;
  }
  
  @JsonProperty(JSON_PROPERTY_SLEEP_PHASE5_MIN)
  public void setSleepPhase5Min_JsonNullable(JsonNullable<String> sleepPhase5Min) {
    this.sleepPhase5Min = sleepPhase5Min;
  }

  public void setSleepPhase5Min(@javax.annotation.Nullable String sleepPhase5Min) {
    this.sleepPhase5Min = JsonNullable.<String>of(sleepPhase5Min);
  }


  public PublicModifiedSleepModel sleepScoreDelta(@javax.annotation.Nullable Integer sleepScoreDelta) {
    this.sleepScoreDelta = JsonNullable.<Integer>of(sleepScoreDelta);
    return this;
  }

  /**
   * Get sleepScoreDelta
   * @return sleepScoreDelta
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getSleepScoreDelta() {
        return sleepScoreDelta.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_SLEEP_SCORE_DELTA)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getSleepScoreDelta_JsonNullable() {
    return sleepScoreDelta;
  }
  
  @JsonProperty(JSON_PROPERTY_SLEEP_SCORE_DELTA)
  public void setSleepScoreDelta_JsonNullable(JsonNullable<Integer> sleepScoreDelta) {
    this.sleepScoreDelta = sleepScoreDelta;
  }

  public void setSleepScoreDelta(@javax.annotation.Nullable Integer sleepScoreDelta) {
    this.sleepScoreDelta = JsonNullable.<Integer>of(sleepScoreDelta);
  }


  public PublicModifiedSleepModel timeInBed(@javax.annotation.Nonnull Integer timeInBed) {
    this.timeInBed = timeInBed;
    return this;
  }

  /**
   * Duration spent in bed in seconds.
   * @return timeInBed
   */
  @javax.annotation.Nonnull
  @JsonProperty(JSON_PROPERTY_TIME_IN_BED)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public Integer getTimeInBed() {
    return timeInBed;
  }


  @JsonProperty(JSON_PROPERTY_TIME_IN_BED)
  @JsonInclude(value = JsonInclude.Include.ALWAYS)
  public void setTimeInBed(@javax.annotation.Nonnull Integer timeInBed) {
    this.timeInBed = timeInBed;
  }


  public PublicModifiedSleepModel totalSleepDuration(@javax.annotation.Nullable Integer totalSleepDuration) {
    this.totalSleepDuration = JsonNullable.<Integer>of(totalSleepDuration);
    return this;
  }

  /**
   * Get totalSleepDuration
   * @return totalSleepDuration
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public Integer getTotalSleepDuration() {
        return totalSleepDuration.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TOTAL_SLEEP_DURATION)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<Integer> getTotalSleepDuration_JsonNullable() {
    return totalSleepDuration;
  }
  
  @JsonProperty(JSON_PROPERTY_TOTAL_SLEEP_DURATION)
  public void setTotalSleepDuration_JsonNullable(JsonNullable<Integer> totalSleepDuration) {
    this.totalSleepDuration = totalSleepDuration;
  }

  public void setTotalSleepDuration(@javax.annotation.Nullable Integer totalSleepDuration) {
    this.totalSleepDuration = JsonNullable.<Integer>of(totalSleepDuration);
  }


  public PublicModifiedSleepModel type(@javax.annotation.Nullable PublicSleepType type) {
    this.type = JsonNullable.<PublicSleepType>of(type);
    return this;
  }

  /**
   * Get type
   * @return type
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public PublicSleepType getType() {
        return type.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_TYPE)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<PublicSleepType> getType_JsonNullable() {
    return type;
  }
  
  @JsonProperty(JSON_PROPERTY_TYPE)
  public void setType_JsonNullable(JsonNullable<PublicSleepType> type) {
    this.type = type;
  }

  public void setType(@javax.annotation.Nullable PublicSleepType type) {
    this.type = JsonNullable.<PublicSleepType>of(type);
  }


  public PublicModifiedSleepModel ringId(@javax.annotation.Nullable String ringId) {
    this.ringId = JsonNullable.<String>of(ringId);
    return this;
  }

  /**
   * Get ringId
   * @return ringId
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getRingId() {
        return ringId.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_RING_ID)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getRingId_JsonNullable() {
    return ringId;
  }
  
  @JsonProperty(JSON_PROPERTY_RING_ID)
  public void setRingId_JsonNullable(JsonNullable<String> ringId) {
    this.ringId = ringId;
  }

  public void setRingId(@javax.annotation.Nullable String ringId) {
    this.ringId = JsonNullable.<String>of(ringId);
  }


  public PublicModifiedSleepModel appSleepPhase5Min(@javax.annotation.Nullable String appSleepPhase5Min) {
    this.appSleepPhase5Min = JsonNullable.<String>of(appSleepPhase5Min);
    return this;
  }

  /**
   * Get appSleepPhase5Min
   * @return appSleepPhase5Min
   */
  @javax.annotation.Nullable
  @JsonIgnore
  public String getAppSleepPhase5Min() {
        return appSleepPhase5Min.orElse(null);
  }

  @JsonProperty(JSON_PROPERTY_APP_SLEEP_PHASE5_MIN)
  @JsonInclude(value = JsonInclude.Include.USE_DEFAULTS)

  public JsonNullable<String> getAppSleepPhase5Min_JsonNullable() {
    return appSleepPhase5Min;
  }
  
  @JsonProperty(JSON_PROPERTY_APP_SLEEP_PHASE5_MIN)
  public void setAppSleepPhase5Min_JsonNullable(JsonNullable<String> appSleepPhase5Min) {
    this.appSleepPhase5Min = appSleepPhase5Min;
  }

  public void setAppSleepPhase5Min(@javax.annotation.Nullable String appSleepPhase5Min) {
    this.appSleepPhase5Min = JsonNullable.<String>of(appSleepPhase5Min);
  }


  /**
   * Return true if this PublicModifiedSleepModel object is equal to o.
   */
  @Override
  public boolean equals(Object o) {
    if (this == o) {
      return true;
    }
    if (o == null || getClass() != o.getClass()) {
      return false;
    }
    PublicModifiedSleepModel publicModifiedSleepModel = (PublicModifiedSleepModel) o;
    return Objects.equals(this.id, publicModifiedSleepModel.id) &&
        equalsNullable(this.averageBreath, publicModifiedSleepModel.averageBreath) &&
        equalsNullable(this.averageHeartRate, publicModifiedSleepModel.averageHeartRate) &&
        equalsNullable(this.averageHrv, publicModifiedSleepModel.averageHrv) &&
        equalsNullable(this.awakeTime, publicModifiedSleepModel.awakeTime) &&
        Objects.equals(this.bedtimeEnd, publicModifiedSleepModel.bedtimeEnd) &&
        Objects.equals(this.bedtimeStart, publicModifiedSleepModel.bedtimeStart) &&
        Objects.equals(this.day, publicModifiedSleepModel.day) &&
        equalsNullable(this.deepSleepDuration, publicModifiedSleepModel.deepSleepDuration) &&
        equalsNullable(this.efficiency, publicModifiedSleepModel.efficiency) &&
        equalsNullable(this.heartRate, publicModifiedSleepModel.heartRate) &&
        equalsNullable(this.hrv, publicModifiedSleepModel.hrv) &&
        equalsNullable(this.latency, publicModifiedSleepModel.latency) &&
        equalsNullable(this.lightSleepDuration, publicModifiedSleepModel.lightSleepDuration) &&
        Objects.equals(this.lowBatteryAlert, publicModifiedSleepModel.lowBatteryAlert) &&
        equalsNullable(this.lowestHeartRate, publicModifiedSleepModel.lowestHeartRate) &&
        equalsNullable(this.movement30Sec, publicModifiedSleepModel.movement30Sec) &&
        Objects.equals(this.period, publicModifiedSleepModel.period) &&
        equalsNullable(this.readiness, publicModifiedSleepModel.readiness) &&
        equalsNullable(this.readinessScoreDelta, publicModifiedSleepModel.readinessScoreDelta) &&
        equalsNullable(this.remSleepDuration, publicModifiedSleepModel.remSleepDuration) &&
        equalsNullable(this.restlessPeriods, publicModifiedSleepModel.restlessPeriods) &&
        equalsNullable(this.sleepAlgorithmVersion, publicModifiedSleepModel.sleepAlgorithmVersion) &&
        equalsNullable(this.sleepAnalysisReason, publicModifiedSleepModel.sleepAnalysisReason) &&
        equalsNullable(this.sleepPhase30Sec, publicModifiedSleepModel.sleepPhase30Sec) &&
        equalsNullable(this.sleepPhase5Min, publicModifiedSleepModel.sleepPhase5Min) &&
        equalsNullable(this.sleepScoreDelta, publicModifiedSleepModel.sleepScoreDelta) &&
        Objects.equals(this.timeInBed, publicModifiedSleepModel.timeInBed) &&
        equalsNullable(this.totalSleepDuration, publicModifiedSleepModel.totalSleepDuration) &&
        equalsNullable(this.type, publicModifiedSleepModel.type) &&
        equalsNullable(this.ringId, publicModifiedSleepModel.ringId) &&
        equalsNullable(this.appSleepPhase5Min, publicModifiedSleepModel.appSleepPhase5Min);
  }

  private static <T> boolean equalsNullable(JsonNullable<T> a, JsonNullable<T> b) {
    return a == b || (a != null && b != null && a.isPresent() && b.isPresent() && Objects.deepEquals(a.get(), b.get()));
  }

  @Override
  public int hashCode() {
    return Objects.hash(id, hashCodeNullable(averageBreath), hashCodeNullable(averageHeartRate), hashCodeNullable(averageHrv), hashCodeNullable(awakeTime), bedtimeEnd, bedtimeStart, day, hashCodeNullable(deepSleepDuration), hashCodeNullable(efficiency), hashCodeNullable(heartRate), hashCodeNullable(hrv), hashCodeNullable(latency), hashCodeNullable(lightSleepDuration), lowBatteryAlert, hashCodeNullable(lowestHeartRate), hashCodeNullable(movement30Sec), period, hashCodeNullable(readiness), hashCodeNullable(readinessScoreDelta), hashCodeNullable(remSleepDuration), hashCodeNullable(restlessPeriods), hashCodeNullable(sleepAlgorithmVersion), hashCodeNullable(sleepAnalysisReason), hashCodeNullable(sleepPhase30Sec), hashCodeNullable(sleepPhase5Min), hashCodeNullable(sleepScoreDelta), timeInBed, hashCodeNullable(totalSleepDuration), hashCodeNullable(type), hashCodeNullable(ringId), hashCodeNullable(appSleepPhase5Min));
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
    sb.append("class PublicModifiedSleepModel {\n");
    sb.append("    id: ").append(toIndentedString(id)).append("\n");
    sb.append("    averageBreath: ").append(toIndentedString(averageBreath)).append("\n");
    sb.append("    averageHeartRate: ").append(toIndentedString(averageHeartRate)).append("\n");
    sb.append("    averageHrv: ").append(toIndentedString(averageHrv)).append("\n");
    sb.append("    awakeTime: ").append(toIndentedString(awakeTime)).append("\n");
    sb.append("    bedtimeEnd: ").append(toIndentedString(bedtimeEnd)).append("\n");
    sb.append("    bedtimeStart: ").append(toIndentedString(bedtimeStart)).append("\n");
    sb.append("    day: ").append(toIndentedString(day)).append("\n");
    sb.append("    deepSleepDuration: ").append(toIndentedString(deepSleepDuration)).append("\n");
    sb.append("    efficiency: ").append(toIndentedString(efficiency)).append("\n");
    sb.append("    heartRate: ").append(toIndentedString(heartRate)).append("\n");
    sb.append("    hrv: ").append(toIndentedString(hrv)).append("\n");
    sb.append("    latency: ").append(toIndentedString(latency)).append("\n");
    sb.append("    lightSleepDuration: ").append(toIndentedString(lightSleepDuration)).append("\n");
    sb.append("    lowBatteryAlert: ").append(toIndentedString(lowBatteryAlert)).append("\n");
    sb.append("    lowestHeartRate: ").append(toIndentedString(lowestHeartRate)).append("\n");
    sb.append("    movement30Sec: ").append(toIndentedString(movement30Sec)).append("\n");
    sb.append("    period: ").append(toIndentedString(period)).append("\n");
    sb.append("    readiness: ").append(toIndentedString(readiness)).append("\n");
    sb.append("    readinessScoreDelta: ").append(toIndentedString(readinessScoreDelta)).append("\n");
    sb.append("    remSleepDuration: ").append(toIndentedString(remSleepDuration)).append("\n");
    sb.append("    restlessPeriods: ").append(toIndentedString(restlessPeriods)).append("\n");
    sb.append("    sleepAlgorithmVersion: ").append(toIndentedString(sleepAlgorithmVersion)).append("\n");
    sb.append("    sleepAnalysisReason: ").append(toIndentedString(sleepAnalysisReason)).append("\n");
    sb.append("    sleepPhase30Sec: ").append(toIndentedString(sleepPhase30Sec)).append("\n");
    sb.append("    sleepPhase5Min: ").append(toIndentedString(sleepPhase5Min)).append("\n");
    sb.append("    sleepScoreDelta: ").append(toIndentedString(sleepScoreDelta)).append("\n");
    sb.append("    timeInBed: ").append(toIndentedString(timeInBed)).append("\n");
    sb.append("    totalSleepDuration: ").append(toIndentedString(totalSleepDuration)).append("\n");
    sb.append("    type: ").append(toIndentedString(type)).append("\n");
    sb.append("    ringId: ").append(toIndentedString(ringId)).append("\n");
    sb.append("    appSleepPhase5Min: ").append(toIndentedString(appSleepPhase5Min)).append("\n");
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

    // add `average_breath` to the URL query string
    if (getAverageBreath() != null) {
      joiner.add(String.format("%saverage_breath%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getAverageBreath()))));
    }

    // add `average_heart_rate` to the URL query string
    if (getAverageHeartRate() != null) {
      joiner.add(String.format("%saverage_heart_rate%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getAverageHeartRate()))));
    }

    // add `average_hrv` to the URL query string
    if (getAverageHrv() != null) {
      joiner.add(String.format("%saverage_hrv%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getAverageHrv()))));
    }

    // add `awake_time` to the URL query string
    if (getAwakeTime() != null) {
      joiner.add(String.format("%sawake_time%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getAwakeTime()))));
    }

    // add `bedtime_end` to the URL query string
    if (getBedtimeEnd() != null) {
      joiner.add(String.format("%sbedtime_end%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getBedtimeEnd()))));
    }

    // add `bedtime_start` to the URL query string
    if (getBedtimeStart() != null) {
      joiner.add(String.format("%sbedtime_start%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getBedtimeStart()))));
    }

    // add `day` to the URL query string
    if (getDay() != null) {
      joiner.add(String.format("%sday%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDay()))));
    }

    // add `deep_sleep_duration` to the URL query string
    if (getDeepSleepDuration() != null) {
      joiner.add(String.format("%sdeep_sleep_duration%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getDeepSleepDuration()))));
    }

    // add `efficiency` to the URL query string
    if (getEfficiency() != null) {
      joiner.add(String.format("%sefficiency%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getEfficiency()))));
    }

    // add `heart_rate` to the URL query string
    if (getHeartRate() != null) {
      joiner.add(getHeartRate().toUrlQueryString(prefix + "heart_rate" + suffix));
    }

    // add `hrv` to the URL query string
    if (getHrv() != null) {
      joiner.add(getHrv().toUrlQueryString(prefix + "hrv" + suffix));
    }

    // add `latency` to the URL query string
    if (getLatency() != null) {
      joiner.add(String.format("%slatency%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLatency()))));
    }

    // add `light_sleep_duration` to the URL query string
    if (getLightSleepDuration() != null) {
      joiner.add(String.format("%slight_sleep_duration%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLightSleepDuration()))));
    }

    // add `low_battery_alert` to the URL query string
    if (getLowBatteryAlert() != null) {
      joiner.add(String.format("%slow_battery_alert%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLowBatteryAlert()))));
    }

    // add `lowest_heart_rate` to the URL query string
    if (getLowestHeartRate() != null) {
      joiner.add(String.format("%slowest_heart_rate%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getLowestHeartRate()))));
    }

    // add `movement_30_sec` to the URL query string
    if (getMovement30Sec() != null) {
      joiner.add(String.format("%smovement_30_sec%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getMovement30Sec()))));
    }

    // add `period` to the URL query string
    if (getPeriod() != null) {
      joiner.add(String.format("%speriod%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getPeriod()))));
    }

    // add `readiness` to the URL query string
    if (getReadiness() != null) {
      joiner.add(getReadiness().toUrlQueryString(prefix + "readiness" + suffix));
    }

    // add `readiness_score_delta` to the URL query string
    if (getReadinessScoreDelta() != null) {
      joiner.add(String.format("%sreadiness_score_delta%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getReadinessScoreDelta()))));
    }

    // add `rem_sleep_duration` to the URL query string
    if (getRemSleepDuration() != null) {
      joiner.add(String.format("%srem_sleep_duration%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRemSleepDuration()))));
    }

    // add `restless_periods` to the URL query string
    if (getRestlessPeriods() != null) {
      joiner.add(String.format("%srestless_periods%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRestlessPeriods()))));
    }

    // add `sleep_algorithm_version` to the URL query string
    if (getSleepAlgorithmVersion() != null) {
      joiner.add(String.format("%ssleep_algorithm_version%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSleepAlgorithmVersion()))));
    }

    // add `sleep_analysis_reason` to the URL query string
    if (getSleepAnalysisReason() != null) {
      joiner.add(String.format("%ssleep_analysis_reason%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSleepAnalysisReason()))));
    }

    // add `sleep_phase_30_sec` to the URL query string
    if (getSleepPhase30Sec() != null) {
      joiner.add(String.format("%ssleep_phase_30_sec%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSleepPhase30Sec()))));
    }

    // add `sleep_phase_5_min` to the URL query string
    if (getSleepPhase5Min() != null) {
      joiner.add(String.format("%ssleep_phase_5_min%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSleepPhase5Min()))));
    }

    // add `sleep_score_delta` to the URL query string
    if (getSleepScoreDelta() != null) {
      joiner.add(String.format("%ssleep_score_delta%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getSleepScoreDelta()))));
    }

    // add `time_in_bed` to the URL query string
    if (getTimeInBed() != null) {
      joiner.add(String.format("%stime_in_bed%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTimeInBed()))));
    }

    // add `total_sleep_duration` to the URL query string
    if (getTotalSleepDuration() != null) {
      joiner.add(String.format("%stotal_sleep_duration%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getTotalSleepDuration()))));
    }

    // add `type` to the URL query string
    if (getType() != null) {
      joiner.add(String.format("%stype%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getType()))));
    }

    // add `ring_id` to the URL query string
    if (getRingId() != null) {
      joiner.add(String.format("%sring_id%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getRingId()))));
    }

    // add `app_sleep_phase_5_min` to the URL query string
    if (getAppSleepPhase5Min() != null) {
      joiner.add(String.format("%sapp_sleep_phase_5_min%s=%s", prefix, suffix, ApiClient.urlEncode(ApiClient.valueToString(getAppSleepPhase5Min()))));
    }

    return joiner.toString();
  }
}

