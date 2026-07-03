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
    /// Object defining a daily activity that is a 24-hour period starting at 4 a.m.
    /// </summary>
    [DataContract(Name = "PublicDailyActivity")]
    public partial class PublicDailyActivity : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="PublicDailyActivity" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected PublicDailyActivity() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="PublicDailyActivity" /> class.
        /// </summary>
        /// <param name="id">Unique identifier of the object. (required).</param>
        /// <param name="activeCalories">Active calories expended in kilocalories. (required).</param>
        /// <param name="averageMetMinutes">Average MET minutes. (required).</param>
        /// <param name="class5Min">class5Min.</param>
        /// <param name="contributors">Object containing activity score contributors. (required).</param>
        /// <param name="day">day (required).</param>
        /// <param name="equivalentWalkingDistance">Equivalent walking distance of energe expenditure in meters. (required).</param>
        /// <param name="highActivityMetMinutes">The total METs of each minute classified as high activity. (required).</param>
        /// <param name="highActivityTime">The total time in seconds of each minute classified as high activity. (required).</param>
        /// <param name="inactivityAlerts">Number of inactivity alerts received. (required).</param>
        /// <param name="lowActivityMetMinutes">The total METs of each minute classified as low activity. (required).</param>
        /// <param name="lowActivityTime">The total time in seconds of each minute classified as low activity. (required).</param>
        /// <param name="mediumActivityMetMinutes">The total METs of each minute classified as medium activity. (required).</param>
        /// <param name="mediumActivityTime">The total time in seconds of each minute classified as medium activity. (required).</param>
        /// <param name="met">Sample containing METs. (required).</param>
        /// <param name="metersToTarget">Meters remaining to target. (required).</param>
        /// <param name="nonWearTime">Ring non-wear time in seconds. (required).</param>
        /// <param name="restingTime">Resting time in seconds. (required).</param>
        /// <param name="score">score.</param>
        /// <param name="sedentaryMetMinutes">Sedentary MET minutes. (required).</param>
        /// <param name="sedentaryTime">Sedentary time in seconds. (required).</param>
        /// <param name="steps">Total number of steps taken. (required).</param>
        /// <param name="targetCalories">Daily activity target in kilocalories. (required).</param>
        /// <param name="targetMeters">Daily activity target in meters. (required).</param>
        /// <param name="timestamp">timestamp (required).</param>
        /// <param name="totalCalories">Total calories expended in kilocalories. (required).</param>
        public PublicDailyActivity(string id = default, int activeCalories = default, decimal averageMetMinutes = default, string class5Min = default, PublicActivityContributors contributors = default, string day = default, int equivalentWalkingDistance = default, int highActivityMetMinutes = default, int highActivityTime = default, int inactivityAlerts = default, int lowActivityMetMinutes = default, int lowActivityTime = default, int mediumActivityMetMinutes = default, int mediumActivityTime = default, PublicSample met = default, int metersToTarget = default, int nonWearTime = default, int restingTime = default, int? score = default, int sedentaryMetMinutes = default, int sedentaryTime = default, int steps = default, int targetCalories = default, int targetMeters = default, string timestamp = default, int totalCalories = default)
        {
            // to ensure "id" is required (not null)
            if (id == null)
            {
                throw new ArgumentNullException("id is a required property for PublicDailyActivity and cannot be null");
            }
            this.Id = id;
            this.ActiveCalories = activeCalories;
            this.AverageMetMinutes = averageMetMinutes;
            // to ensure "contributors" is required (not null)
            if (contributors == null)
            {
                throw new ArgumentNullException("contributors is a required property for PublicDailyActivity and cannot be null");
            }
            this.Contributors = contributors;
            // to ensure "day" is required (not null)
            if (day == null)
            {
                throw new ArgumentNullException("day is a required property for PublicDailyActivity and cannot be null");
            }
            this.Day = day;
            this.EquivalentWalkingDistance = equivalentWalkingDistance;
            this.HighActivityMetMinutes = highActivityMetMinutes;
            this.HighActivityTime = highActivityTime;
            this.InactivityAlerts = inactivityAlerts;
            this.LowActivityMetMinutes = lowActivityMetMinutes;
            this.LowActivityTime = lowActivityTime;
            this.MediumActivityMetMinutes = mediumActivityMetMinutes;
            this.MediumActivityTime = mediumActivityTime;
            // to ensure "met" is required (not null)
            if (met == null)
            {
                throw new ArgumentNullException("met is a required property for PublicDailyActivity and cannot be null");
            }
            this.Met = met;
            this.MetersToTarget = metersToTarget;
            this.NonWearTime = nonWearTime;
            this.RestingTime = restingTime;
            this.SedentaryMetMinutes = sedentaryMetMinutes;
            this.SedentaryTime = sedentaryTime;
            this.Steps = steps;
            this.TargetCalories = targetCalories;
            this.TargetMeters = targetMeters;
            // to ensure "timestamp" is required (not null)
            if (timestamp == null)
            {
                throw new ArgumentNullException("timestamp is a required property for PublicDailyActivity and cannot be null");
            }
            this.Timestamp = timestamp;
            this.TotalCalories = totalCalories;
            this.Class5Min = class5Min;
            this.Score = score;
        }

        /// <summary>
        /// Unique identifier of the object.
        /// </summary>
        /// <value>Unique identifier of the object.</value>
        [DataMember(Name = "id", IsRequired = true, EmitDefaultValue = true)]
        public string Id { get; set; }

        /// <summary>
        /// Active calories expended in kilocalories.
        /// </summary>
        /// <value>Active calories expended in kilocalories.</value>
        [DataMember(Name = "active_calories", IsRequired = true, EmitDefaultValue = true)]
        public int ActiveCalories { get; set; }

        /// <summary>
        /// Average MET minutes.
        /// </summary>
        /// <value>Average MET minutes.</value>
        [DataMember(Name = "average_met_minutes", IsRequired = true, EmitDefaultValue = true)]
        public decimal AverageMetMinutes { get; set; }

        /// <summary>
        /// Gets or Sets Class5Min
        /// </summary>
        [DataMember(Name = "class_5_min", EmitDefaultValue = true)]
        public string Class5Min { get; set; }

        /// <summary>
        /// Object containing activity score contributors.
        /// </summary>
        /// <value>Object containing activity score contributors.</value>
        [DataMember(Name = "contributors", IsRequired = true, EmitDefaultValue = true)]
        public PublicActivityContributors Contributors { get; set; }

        /// <summary>
        /// Gets or Sets Day
        /// </summary>
        [DataMember(Name = "day", IsRequired = true, EmitDefaultValue = true)]
        public string Day { get; set; }

        /// <summary>
        /// Equivalent walking distance of energe expenditure in meters.
        /// </summary>
        /// <value>Equivalent walking distance of energe expenditure in meters.</value>
        [DataMember(Name = "equivalent_walking_distance", IsRequired = true, EmitDefaultValue = true)]
        public int EquivalentWalkingDistance { get; set; }

        /// <summary>
        /// The total METs of each minute classified as high activity.
        /// </summary>
        /// <value>The total METs of each minute classified as high activity.</value>
        [DataMember(Name = "high_activity_met_minutes", IsRequired = true, EmitDefaultValue = true)]
        public int HighActivityMetMinutes { get; set; }

        /// <summary>
        /// The total time in seconds of each minute classified as high activity.
        /// </summary>
        /// <value>The total time in seconds of each minute classified as high activity.</value>
        [DataMember(Name = "high_activity_time", IsRequired = true, EmitDefaultValue = true)]
        public int HighActivityTime { get; set; }

        /// <summary>
        /// Number of inactivity alerts received.
        /// </summary>
        /// <value>Number of inactivity alerts received.</value>
        [DataMember(Name = "inactivity_alerts", IsRequired = true, EmitDefaultValue = true)]
        public int InactivityAlerts { get; set; }

        /// <summary>
        /// The total METs of each minute classified as low activity.
        /// </summary>
        /// <value>The total METs of each minute classified as low activity.</value>
        [DataMember(Name = "low_activity_met_minutes", IsRequired = true, EmitDefaultValue = true)]
        public int LowActivityMetMinutes { get; set; }

        /// <summary>
        /// The total time in seconds of each minute classified as low activity.
        /// </summary>
        /// <value>The total time in seconds of each minute classified as low activity.</value>
        [DataMember(Name = "low_activity_time", IsRequired = true, EmitDefaultValue = true)]
        public int LowActivityTime { get; set; }

        /// <summary>
        /// The total METs of each minute classified as medium activity.
        /// </summary>
        /// <value>The total METs of each minute classified as medium activity.</value>
        [DataMember(Name = "medium_activity_met_minutes", IsRequired = true, EmitDefaultValue = true)]
        public int MediumActivityMetMinutes { get; set; }

        /// <summary>
        /// The total time in seconds of each minute classified as medium activity.
        /// </summary>
        /// <value>The total time in seconds of each minute classified as medium activity.</value>
        [DataMember(Name = "medium_activity_time", IsRequired = true, EmitDefaultValue = true)]
        public int MediumActivityTime { get; set; }

        /// <summary>
        /// Sample containing METs.
        /// </summary>
        /// <value>Sample containing METs.</value>
        [DataMember(Name = "met", IsRequired = true, EmitDefaultValue = true)]
        public PublicSample Met { get; set; }

        /// <summary>
        /// Meters remaining to target.
        /// </summary>
        /// <value>Meters remaining to target.</value>
        [DataMember(Name = "meters_to_target", IsRequired = true, EmitDefaultValue = true)]
        public int MetersToTarget { get; set; }

        /// <summary>
        /// Ring non-wear time in seconds.
        /// </summary>
        /// <value>Ring non-wear time in seconds.</value>
        [DataMember(Name = "non_wear_time", IsRequired = true, EmitDefaultValue = true)]
        public int NonWearTime { get; set; }

        /// <summary>
        /// Resting time in seconds.
        /// </summary>
        /// <value>Resting time in seconds.</value>
        [DataMember(Name = "resting_time", IsRequired = true, EmitDefaultValue = true)]
        public int RestingTime { get; set; }

        /// <summary>
        /// Gets or Sets Score
        /// </summary>
        [DataMember(Name = "score", EmitDefaultValue = true)]
        public int? Score { get; set; }

        /// <summary>
        /// Sedentary MET minutes.
        /// </summary>
        /// <value>Sedentary MET minutes.</value>
        [DataMember(Name = "sedentary_met_minutes", IsRequired = true, EmitDefaultValue = true)]
        public int SedentaryMetMinutes { get; set; }

        /// <summary>
        /// Sedentary time in seconds.
        /// </summary>
        /// <value>Sedentary time in seconds.</value>
        [DataMember(Name = "sedentary_time", IsRequired = true, EmitDefaultValue = true)]
        public int SedentaryTime { get; set; }

        /// <summary>
        /// Total number of steps taken.
        /// </summary>
        /// <value>Total number of steps taken.</value>
        [DataMember(Name = "steps", IsRequired = true, EmitDefaultValue = true)]
        public int Steps { get; set; }

        /// <summary>
        /// Daily activity target in kilocalories.
        /// </summary>
        /// <value>Daily activity target in kilocalories.</value>
        [DataMember(Name = "target_calories", IsRequired = true, EmitDefaultValue = true)]
        public int TargetCalories { get; set; }

        /// <summary>
        /// Daily activity target in meters.
        /// </summary>
        /// <value>Daily activity target in meters.</value>
        [DataMember(Name = "target_meters", IsRequired = true, EmitDefaultValue = true)]
        public int TargetMeters { get; set; }

        /// <summary>
        /// Gets or Sets Timestamp
        /// </summary>
        [DataMember(Name = "timestamp", IsRequired = true, EmitDefaultValue = true)]
        public string Timestamp { get; set; }

        /// <summary>
        /// Total calories expended in kilocalories.
        /// </summary>
        /// <value>Total calories expended in kilocalories.</value>
        [DataMember(Name = "total_calories", IsRequired = true, EmitDefaultValue = true)]
        public int TotalCalories { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class PublicDailyActivity {\n");
            sb.Append("  Id: ").Append(Id).Append("\n");
            sb.Append("  ActiveCalories: ").Append(ActiveCalories).Append("\n");
            sb.Append("  AverageMetMinutes: ").Append(AverageMetMinutes).Append("\n");
            sb.Append("  Class5Min: ").Append(Class5Min).Append("\n");
            sb.Append("  Contributors: ").Append(Contributors).Append("\n");
            sb.Append("  Day: ").Append(Day).Append("\n");
            sb.Append("  EquivalentWalkingDistance: ").Append(EquivalentWalkingDistance).Append("\n");
            sb.Append("  HighActivityMetMinutes: ").Append(HighActivityMetMinutes).Append("\n");
            sb.Append("  HighActivityTime: ").Append(HighActivityTime).Append("\n");
            sb.Append("  InactivityAlerts: ").Append(InactivityAlerts).Append("\n");
            sb.Append("  LowActivityMetMinutes: ").Append(LowActivityMetMinutes).Append("\n");
            sb.Append("  LowActivityTime: ").Append(LowActivityTime).Append("\n");
            sb.Append("  MediumActivityMetMinutes: ").Append(MediumActivityMetMinutes).Append("\n");
            sb.Append("  MediumActivityTime: ").Append(MediumActivityTime).Append("\n");
            sb.Append("  Met: ").Append(Met).Append("\n");
            sb.Append("  MetersToTarget: ").Append(MetersToTarget).Append("\n");
            sb.Append("  NonWearTime: ").Append(NonWearTime).Append("\n");
            sb.Append("  RestingTime: ").Append(RestingTime).Append("\n");
            sb.Append("  Score: ").Append(Score).Append("\n");
            sb.Append("  SedentaryMetMinutes: ").Append(SedentaryMetMinutes).Append("\n");
            sb.Append("  SedentaryTime: ").Append(SedentaryTime).Append("\n");
            sb.Append("  Steps: ").Append(Steps).Append("\n");
            sb.Append("  TargetCalories: ").Append(TargetCalories).Append("\n");
            sb.Append("  TargetMeters: ").Append(TargetMeters).Append("\n");
            sb.Append("  Timestamp: ").Append(Timestamp).Append("\n");
            sb.Append("  TotalCalories: ").Append(TotalCalories).Append("\n");
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
