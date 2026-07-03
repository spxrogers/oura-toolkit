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
    /// Object defining sleep score contributors.
    /// </summary>
    [DataContract(Name = "PublicSleepContributors")]
    public partial class PublicSleepContributors : IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="PublicSleepContributors" /> class.
        /// </summary>
        /// <param name="deepSleep">deepSleep.</param>
        /// <param name="efficiency">efficiency.</param>
        /// <param name="latency">latency.</param>
        /// <param name="remSleep">remSleep.</param>
        /// <param name="restfulness">restfulness.</param>
        /// <param name="timing">timing.</param>
        /// <param name="totalSleep">totalSleep.</param>
        public PublicSleepContributors(int? deepSleep = default, int? efficiency = default, int? latency = default, int? remSleep = default, int? restfulness = default, int? timing = default, int? totalSleep = default)
        {
            this.DeepSleep = deepSleep;
            this.Efficiency = efficiency;
            this.Latency = latency;
            this.RemSleep = remSleep;
            this.Restfulness = restfulness;
            this.Timing = timing;
            this.TotalSleep = totalSleep;
        }

        /// <summary>
        /// Gets or Sets DeepSleep
        /// </summary>
        [DataMember(Name = "deep_sleep", EmitDefaultValue = true)]
        public int? DeepSleep { get; set; }

        /// <summary>
        /// Gets or Sets Efficiency
        /// </summary>
        [DataMember(Name = "efficiency", EmitDefaultValue = true)]
        public int? Efficiency { get; set; }

        /// <summary>
        /// Gets or Sets Latency
        /// </summary>
        [DataMember(Name = "latency", EmitDefaultValue = true)]
        public int? Latency { get; set; }

        /// <summary>
        /// Gets or Sets RemSleep
        /// </summary>
        [DataMember(Name = "rem_sleep", EmitDefaultValue = true)]
        public int? RemSleep { get; set; }

        /// <summary>
        /// Gets or Sets Restfulness
        /// </summary>
        [DataMember(Name = "restfulness", EmitDefaultValue = true)]
        public int? Restfulness { get; set; }

        /// <summary>
        /// Gets or Sets Timing
        /// </summary>
        [DataMember(Name = "timing", EmitDefaultValue = true)]
        public int? Timing { get; set; }

        /// <summary>
        /// Gets or Sets TotalSleep
        /// </summary>
        [DataMember(Name = "total_sleep", EmitDefaultValue = true)]
        public int? TotalSleep { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class PublicSleepContributors {\n");
            sb.Append("  DeepSleep: ").Append(DeepSleep).Append("\n");
            sb.Append("  Efficiency: ").Append(Efficiency).Append("\n");
            sb.Append("  Latency: ").Append(Latency).Append("\n");
            sb.Append("  RemSleep: ").Append(RemSleep).Append("\n");
            sb.Append("  Restfulness: ").Append(Restfulness).Append("\n");
            sb.Append("  Timing: ").Append(Timing).Append("\n");
            sb.Append("  TotalSleep: ").Append(TotalSleep).Append("\n");
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
            yield break;
        }
    }

}
