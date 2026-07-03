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
    /// Public model for Workout.
    /// </summary>
    [DataContract(Name = "PublicWorkout")]
    public partial class PublicWorkout : IValidatableObject
    {

        /// <summary>
        /// Intensity of the workout.
        /// </summary>
        /// <value>Intensity of the workout.</value>
        [DataMember(Name = "intensity", IsRequired = true, EmitDefaultValue = true)]
        public PublicWorkoutIntensity Intensity { get; set; }

        /// <summary>
        /// Possible workout sources.
        /// </summary>
        /// <value>Possible workout sources.</value>
        [DataMember(Name = "source", IsRequired = true, EmitDefaultValue = true)]
        public PublicWorkoutSource Source { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="PublicWorkout" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected PublicWorkout() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="PublicWorkout" /> class.
        /// </summary>
        /// <param name="id">Unique identifier of the object. (required).</param>
        /// <param name="activity">Type of the workout activity. (required).</param>
        /// <param name="calories">calories.</param>
        /// <param name="day">day (required).</param>
        /// <param name="distance">distance.</param>
        /// <param name="endDatetime">endDatetime (required).</param>
        /// <param name="intensity">Intensity of the workout. (required).</param>
        /// <param name="label">label.</param>
        /// <param name="source">Possible workout sources. (required).</param>
        /// <param name="startDatetime">startDatetime (required).</param>
        public PublicWorkout(string id = default, string activity = default, decimal? calories = default, string day = default, decimal? distance = default, string endDatetime = default, PublicWorkoutIntensity intensity = default, string label = default, PublicWorkoutSource source = default, string startDatetime = default)
        {
            // to ensure "id" is required (not null)
            if (id == null)
            {
                throw new ArgumentNullException("id is a required property for PublicWorkout and cannot be null");
            }
            this.Id = id;
            // to ensure "activity" is required (not null)
            if (activity == null)
            {
                throw new ArgumentNullException("activity is a required property for PublicWorkout and cannot be null");
            }
            this.Activity = activity;
            // to ensure "day" is required (not null)
            if (day == null)
            {
                throw new ArgumentNullException("day is a required property for PublicWorkout and cannot be null");
            }
            this.Day = day;
            // to ensure "endDatetime" is required (not null)
            if (endDatetime == null)
            {
                throw new ArgumentNullException("endDatetime is a required property for PublicWorkout and cannot be null");
            }
            this.EndDatetime = endDatetime;
            this.Intensity = intensity;
            this.Source = source;
            // to ensure "startDatetime" is required (not null)
            if (startDatetime == null)
            {
                throw new ArgumentNullException("startDatetime is a required property for PublicWorkout and cannot be null");
            }
            this.StartDatetime = startDatetime;
            this.Calories = calories;
            this.Distance = distance;
            this.Label = label;
        }

        /// <summary>
        /// Unique identifier of the object.
        /// </summary>
        /// <value>Unique identifier of the object.</value>
        [DataMember(Name = "id", IsRequired = true, EmitDefaultValue = true)]
        public string Id { get; set; }

        /// <summary>
        /// Type of the workout activity.
        /// </summary>
        /// <value>Type of the workout activity.</value>
        [DataMember(Name = "activity", IsRequired = true, EmitDefaultValue = true)]
        public string Activity { get; set; }

        /// <summary>
        /// Gets or Sets Calories
        /// </summary>
        [DataMember(Name = "calories", EmitDefaultValue = true)]
        public decimal? Calories { get; set; }

        /// <summary>
        /// Gets or Sets Day
        /// </summary>
        [DataMember(Name = "day", IsRequired = true, EmitDefaultValue = true)]
        public string Day { get; set; }

        /// <summary>
        /// Gets or Sets Distance
        /// </summary>
        [DataMember(Name = "distance", EmitDefaultValue = true)]
        public decimal? Distance { get; set; }

        /// <summary>
        /// Gets or Sets EndDatetime
        /// </summary>
        [DataMember(Name = "end_datetime", IsRequired = true, EmitDefaultValue = true)]
        public string EndDatetime { get; set; }

        /// <summary>
        /// Gets or Sets Label
        /// </summary>
        [DataMember(Name = "label", EmitDefaultValue = true)]
        public string Label { get; set; }

        /// <summary>
        /// Gets or Sets StartDatetime
        /// </summary>
        [DataMember(Name = "start_datetime", IsRequired = true, EmitDefaultValue = true)]
        public string StartDatetime { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class PublicWorkout {\n");
            sb.Append("  Id: ").Append(Id).Append("\n");
            sb.Append("  Activity: ").Append(Activity).Append("\n");
            sb.Append("  Calories: ").Append(Calories).Append("\n");
            sb.Append("  Day: ").Append(Day).Append("\n");
            sb.Append("  Distance: ").Append(Distance).Append("\n");
            sb.Append("  EndDatetime: ").Append(EndDatetime).Append("\n");
            sb.Append("  Intensity: ").Append(Intensity).Append("\n");
            sb.Append("  Label: ").Append(Label).Append("\n");
            sb.Append("  Source: ").Append(Source).Append("\n");
            sb.Append("  StartDatetime: ").Append(StartDatetime).Append("\n");
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
