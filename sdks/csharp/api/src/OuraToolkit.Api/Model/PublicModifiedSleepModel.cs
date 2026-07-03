/*
 * Oura API Documentation
 *
 * # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | - -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- - | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 
 *
 * The version of the OpenAPI document: 2.0
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using System.Text.RegularExpressions;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System.ComponentModel.DataAnnotations;
using FileParameter = OuraToolkit.Api.Client.FileParameter;
using OpenAPIDateConverter = OuraToolkit.Api.Client.OpenAPIDateConverter;

namespace OuraToolkit.Api.Model
{
    /// <summary>
    /// PublicModifiedSleepModel
    /// </summary>
    [DataContract(Name = "PublicModifiedSleepModel")]
    public partial class PublicModifiedSleepModel : IValidatableObject
    {

        /// <summary>
        /// Gets or Sets SleepAlgorithmVersion
        /// </summary>
        [DataMember(Name = "sleep_algorithm_version", EmitDefaultValue = true)]
        public PublicSleepAlgorithmVersion? SleepAlgorithmVersion { get; set; }

        /// <summary>
        /// Gets or Sets SleepAnalysisReason
        /// </summary>
        [DataMember(Name = "sleep_analysis_reason", EmitDefaultValue = true)]
        public PublicSleepAnalysisReason? SleepAnalysisReason { get; set; }

        /// <summary>
        /// Gets or Sets Type
        /// </summary>
        [DataMember(Name = "type", EmitDefaultValue = true)]
        public PublicSleepType? Type { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="PublicModifiedSleepModel" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected PublicModifiedSleepModel() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="PublicModifiedSleepModel" /> class.
        /// </summary>
        /// <param name="id">Unique identifier of the object. (required).</param>
        /// <param name="averageBreath">averageBreath.</param>
        /// <param name="averageHeartRate">averageHeartRate.</param>
        /// <param name="averageHrv">averageHrv.</param>
        /// <param name="awakeTime">awakeTime.</param>
        /// <param name="bedtimeEnd">bedtimeEnd (required).</param>
        /// <param name="bedtimeStart">bedtimeStart (required).</param>
        /// <param name="day">day (required).</param>
        /// <param name="deepSleepDuration">deepSleepDuration.</param>
        /// <param name="efficiency">efficiency.</param>
        /// <param name="heartRate">heartRate.</param>
        /// <param name="hrv">hrv.</param>
        /// <param name="latency">latency.</param>
        /// <param name="lightSleepDuration">lightSleepDuration.</param>
        /// <param name="lowBatteryAlert">Flag indicating if a low battery alert occurred. (required).</param>
        /// <param name="lowestHeartRate">lowestHeartRate.</param>
        /// <param name="movement30Sec">movement30Sec.</param>
        /// <param name="period">ECore sleep period identifier. (required).</param>
        /// <param name="readiness">readiness.</param>
        /// <param name="readinessScoreDelta">readinessScoreDelta.</param>
        /// <param name="remSleepDuration">remSleepDuration.</param>
        /// <param name="restlessPeriods">restlessPeriods.</param>
        /// <param name="sleepAlgorithmVersion">sleepAlgorithmVersion.</param>
        /// <param name="sleepAnalysisReason">sleepAnalysisReason.</param>
        /// <param name="sleepPhase30Sec">sleepPhase30Sec.</param>
        /// <param name="sleepPhase5Min">sleepPhase5Min.</param>
        /// <param name="sleepScoreDelta">sleepScoreDelta.</param>
        /// <param name="timeInBed">Duration spent in bed in seconds. (required).</param>
        /// <param name="totalSleepDuration">totalSleepDuration.</param>
        /// <param name="type">type.</param>
        /// <param name="ringId">ringId.</param>
        /// <param name="appSleepPhase5Min">appSleepPhase5Min.</param>
        public PublicModifiedSleepModel(string id = default, decimal? averageBreath = default, decimal? averageHeartRate = default, int? averageHrv = default, int? awakeTime = default, string bedtimeEnd = default, string bedtimeStart = default, string day = default, int? deepSleepDuration = default, int? efficiency = default, PublicSample heartRate = default, PublicSample hrv = default, int? latency = default, int? lightSleepDuration = default, bool lowBatteryAlert = default, int? lowestHeartRate = default, string movement30Sec = default, int period = default, PublicReadiness readiness = default, int? readinessScoreDelta = default, int? remSleepDuration = default, int? restlessPeriods = default, PublicSleepAlgorithmVersion? sleepAlgorithmVersion = default, PublicSleepAnalysisReason? sleepAnalysisReason = default, string sleepPhase30Sec = default, string sleepPhase5Min = default, int? sleepScoreDelta = default, int timeInBed = default, int? totalSleepDuration = default, PublicSleepType? type = default, string ringId = default, string appSleepPhase5Min = default)
        {
            // to ensure "id" is required (not null)
            if (id == null)
            {
                throw new ArgumentNullException("id is a required property for PublicModifiedSleepModel and cannot be null");
            }
            this.Id = id;
            // to ensure "bedtimeEnd" is required (not null)
            if (bedtimeEnd == null)
            {
                throw new ArgumentNullException("bedtimeEnd is a required property for PublicModifiedSleepModel and cannot be null");
            }
            this.BedtimeEnd = bedtimeEnd;
            // to ensure "bedtimeStart" is required (not null)
            if (bedtimeStart == null)
            {
                throw new ArgumentNullException("bedtimeStart is a required property for PublicModifiedSleepModel and cannot be null");
            }
            this.BedtimeStart = bedtimeStart;
            // to ensure "day" is required (not null)
            if (day == null)
            {
                throw new ArgumentNullException("day is a required property for PublicModifiedSleepModel and cannot be null");
            }
            this.Day = day;
            this.LowBatteryAlert = lowBatteryAlert;
            this.Period = period;
            this.TimeInBed = timeInBed;
            this.AverageBreath = averageBreath;
            this.AverageHeartRate = averageHeartRate;
            this.AverageHrv = averageHrv;
            this.AwakeTime = awakeTime;
            this.DeepSleepDuration = deepSleepDuration;
            this.Efficiency = efficiency;
            this.HeartRate = heartRate;
            this.Hrv = hrv;
            this.Latency = latency;
            this.LightSleepDuration = lightSleepDuration;
            this.LowestHeartRate = lowestHeartRate;
            this.Movement30Sec = movement30Sec;
            this.Readiness = readiness;
            this.ReadinessScoreDelta = readinessScoreDelta;
            this.RemSleepDuration = remSleepDuration;
            this.RestlessPeriods = restlessPeriods;
            this.SleepAlgorithmVersion = sleepAlgorithmVersion;
            this.SleepAnalysisReason = sleepAnalysisReason;
            this.SleepPhase30Sec = sleepPhase30Sec;
            this.SleepPhase5Min = sleepPhase5Min;
            this.SleepScoreDelta = sleepScoreDelta;
            this.TotalSleepDuration = totalSleepDuration;
            this.Type = type;
            this.RingId = ringId;
            this.AppSleepPhase5Min = appSleepPhase5Min;
        }

        /// <summary>
        /// Unique identifier of the object.
        /// </summary>
        /// <value>Unique identifier of the object.</value>
        [DataMember(Name = "id", IsRequired = true, EmitDefaultValue = true)]
        public string Id { get; set; }

        /// <summary>
        /// Gets or Sets AverageBreath
        /// </summary>
        [DataMember(Name = "average_breath", EmitDefaultValue = true)]
        public decimal? AverageBreath { get; set; }

        /// <summary>
        /// Gets or Sets AverageHeartRate
        /// </summary>
        [DataMember(Name = "average_heart_rate", EmitDefaultValue = true)]
        public decimal? AverageHeartRate { get; set; }

        /// <summary>
        /// Gets or Sets AverageHrv
        /// </summary>
        [DataMember(Name = "average_hrv", EmitDefaultValue = true)]
        public int? AverageHrv { get; set; }

        /// <summary>
        /// Gets or Sets AwakeTime
        /// </summary>
        [DataMember(Name = "awake_time", EmitDefaultValue = true)]
        public int? AwakeTime { get; set; }

        /// <summary>
        /// Gets or Sets BedtimeEnd
        /// </summary>
        [DataMember(Name = "bedtime_end", IsRequired = true, EmitDefaultValue = true)]
        public string BedtimeEnd { get; set; }

        /// <summary>
        /// Gets or Sets BedtimeStart
        /// </summary>
        [DataMember(Name = "bedtime_start", IsRequired = true, EmitDefaultValue = true)]
        public string BedtimeStart { get; set; }

        /// <summary>
        /// Gets or Sets Day
        /// </summary>
        [DataMember(Name = "day", IsRequired = true, EmitDefaultValue = true)]
        public string Day { get; set; }

        /// <summary>
        /// Gets or Sets DeepSleepDuration
        /// </summary>
        [DataMember(Name = "deep_sleep_duration", EmitDefaultValue = true)]
        public int? DeepSleepDuration { get; set; }

        /// <summary>
        /// Gets or Sets Efficiency
        /// </summary>
        [DataMember(Name = "efficiency", EmitDefaultValue = true)]
        public int? Efficiency { get; set; }

        /// <summary>
        /// Gets or Sets HeartRate
        /// </summary>
        [DataMember(Name = "heart_rate", EmitDefaultValue = true)]
        public PublicSample HeartRate { get; set; }

        /// <summary>
        /// Gets or Sets Hrv
        /// </summary>
        [DataMember(Name = "hrv", EmitDefaultValue = true)]
        public PublicSample Hrv { get; set; }

        /// <summary>
        /// Gets or Sets Latency
        /// </summary>
        [DataMember(Name = "latency", EmitDefaultValue = true)]
        public int? Latency { get; set; }

        /// <summary>
        /// Gets or Sets LightSleepDuration
        /// </summary>
        [DataMember(Name = "light_sleep_duration", EmitDefaultValue = true)]
        public int? LightSleepDuration { get; set; }

        /// <summary>
        /// Flag indicating if a low battery alert occurred.
        /// </summary>
        /// <value>Flag indicating if a low battery alert occurred.</value>
        [DataMember(Name = "low_battery_alert", IsRequired = true, EmitDefaultValue = true)]
        public bool LowBatteryAlert { get; set; }

        /// <summary>
        /// Gets or Sets LowestHeartRate
        /// </summary>
        [DataMember(Name = "lowest_heart_rate", EmitDefaultValue = true)]
        public int? LowestHeartRate { get; set; }

        /// <summary>
        /// Gets or Sets Movement30Sec
        /// </summary>
        [DataMember(Name = "movement_30_sec", EmitDefaultValue = true)]
        public string Movement30Sec { get; set; }

        /// <summary>
        /// ECore sleep period identifier.
        /// </summary>
        /// <value>ECore sleep period identifier.</value>
        [DataMember(Name = "period", IsRequired = true, EmitDefaultValue = true)]
        public int Period { get; set; }

        /// <summary>
        /// Gets or Sets Readiness
        /// </summary>
        [DataMember(Name = "readiness", EmitDefaultValue = true)]
        public PublicReadiness Readiness { get; set; }

        /// <summary>
        /// Gets or Sets ReadinessScoreDelta
        /// </summary>
        [DataMember(Name = "readiness_score_delta", EmitDefaultValue = true)]
        public int? ReadinessScoreDelta { get; set; }

        /// <summary>
        /// Gets or Sets RemSleepDuration
        /// </summary>
        [DataMember(Name = "rem_sleep_duration", EmitDefaultValue = true)]
        public int? RemSleepDuration { get; set; }

        /// <summary>
        /// Gets or Sets RestlessPeriods
        /// </summary>
        [DataMember(Name = "restless_periods", EmitDefaultValue = true)]
        public int? RestlessPeriods { get; set; }

        /// <summary>
        /// Gets or Sets SleepPhase30Sec
        /// </summary>
        [DataMember(Name = "sleep_phase_30_sec", EmitDefaultValue = true)]
        public string SleepPhase30Sec { get; set; }

        /// <summary>
        /// Gets or Sets SleepPhase5Min
        /// </summary>
        [DataMember(Name = "sleep_phase_5_min", EmitDefaultValue = true)]
        public string SleepPhase5Min { get; set; }

        /// <summary>
        /// Gets or Sets SleepScoreDelta
        /// </summary>
        [DataMember(Name = "sleep_score_delta", EmitDefaultValue = true)]
        public int? SleepScoreDelta { get; set; }

        /// <summary>
        /// Duration spent in bed in seconds.
        /// </summary>
        /// <value>Duration spent in bed in seconds.</value>
        [DataMember(Name = "time_in_bed", IsRequired = true, EmitDefaultValue = true)]
        public int TimeInBed { get; set; }

        /// <summary>
        /// Gets or Sets TotalSleepDuration
        /// </summary>
        [DataMember(Name = "total_sleep_duration", EmitDefaultValue = true)]
        public int? TotalSleepDuration { get; set; }

        /// <summary>
        /// Gets or Sets RingId
        /// </summary>
        [DataMember(Name = "ring_id", EmitDefaultValue = true)]
        public string RingId { get; set; }

        /// <summary>
        /// Gets or Sets AppSleepPhase5Min
        /// </summary>
        [DataMember(Name = "app_sleep_phase_5_min", EmitDefaultValue = true)]
        public string AppSleepPhase5Min { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class PublicModifiedSleepModel {\n");
            sb.Append("  Id: ").Append(Id).Append("\n");
            sb.Append("  AverageBreath: ").Append(AverageBreath).Append("\n");
            sb.Append("  AverageHeartRate: ").Append(AverageHeartRate).Append("\n");
            sb.Append("  AverageHrv: ").Append(AverageHrv).Append("\n");
            sb.Append("  AwakeTime: ").Append(AwakeTime).Append("\n");
            sb.Append("  BedtimeEnd: ").Append(BedtimeEnd).Append("\n");
            sb.Append("  BedtimeStart: ").Append(BedtimeStart).Append("\n");
            sb.Append("  Day: ").Append(Day).Append("\n");
            sb.Append("  DeepSleepDuration: ").Append(DeepSleepDuration).Append("\n");
            sb.Append("  Efficiency: ").Append(Efficiency).Append("\n");
            sb.Append("  HeartRate: ").Append(HeartRate).Append("\n");
            sb.Append("  Hrv: ").Append(Hrv).Append("\n");
            sb.Append("  Latency: ").Append(Latency).Append("\n");
            sb.Append("  LightSleepDuration: ").Append(LightSleepDuration).Append("\n");
            sb.Append("  LowBatteryAlert: ").Append(LowBatteryAlert).Append("\n");
            sb.Append("  LowestHeartRate: ").Append(LowestHeartRate).Append("\n");
            sb.Append("  Movement30Sec: ").Append(Movement30Sec).Append("\n");
            sb.Append("  Period: ").Append(Period).Append("\n");
            sb.Append("  Readiness: ").Append(Readiness).Append("\n");
            sb.Append("  ReadinessScoreDelta: ").Append(ReadinessScoreDelta).Append("\n");
            sb.Append("  RemSleepDuration: ").Append(RemSleepDuration).Append("\n");
            sb.Append("  RestlessPeriods: ").Append(RestlessPeriods).Append("\n");
            sb.Append("  SleepAlgorithmVersion: ").Append(SleepAlgorithmVersion).Append("\n");
            sb.Append("  SleepAnalysisReason: ").Append(SleepAnalysisReason).Append("\n");
            sb.Append("  SleepPhase30Sec: ").Append(SleepPhase30Sec).Append("\n");
            sb.Append("  SleepPhase5Min: ").Append(SleepPhase5Min).Append("\n");
            sb.Append("  SleepScoreDelta: ").Append(SleepScoreDelta).Append("\n");
            sb.Append("  TimeInBed: ").Append(TimeInBed).Append("\n");
            sb.Append("  TotalSleepDuration: ").Append(TotalSleepDuration).Append("\n");
            sb.Append("  Type: ").Append(Type).Append("\n");
            sb.Append("  RingId: ").Append(RingId).Append("\n");
            sb.Append("  AppSleepPhase5Min: ").Append(AppSleepPhase5Min).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            // Id (string) minLength
            if (this.Id != null && this.Id.Length < 1)
            {
                yield return new ValidationResult("Invalid value for Id, length must be greater than 1.", new [] { "Id" });
            }

            yield break;
        }
    }

}
