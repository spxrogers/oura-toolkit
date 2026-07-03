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
using System.Reflection;

namespace OuraToolkit.Api.Model
{
    /// <summary>
    /// ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet
    /// </summary>
    [JsonConverter(typeof(ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGetJsonConverter))]
    [DataContract(Name = "Response_Multiple_Heartrate_Documents_V2_Usercollection_Heartrate_Get")]
    public partial class ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet : AbstractOpenAPISchema, IValidatableObject
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet" /> class
        /// with the <see cref="TimeSeriesResponsePublicHeartRateRow" /> class
        /// </summary>
        /// <param name="actualInstance">An instance of TimeSeriesResponsePublicHeartRateRow.</param>
        public ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet(TimeSeriesResponsePublicHeartRateRow actualInstance)
        {
            IsNullable = false;
            SchemaType= "anyOf";
            ActualInstance = actualInstance ?? throw new ArgumentException("Invalid instance found. Must not be null.");
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet" /> class
        /// with the <see cref="TimeSeriesResponseDict" /> class
        /// </summary>
        /// <param name="actualInstance">An instance of TimeSeriesResponseDict.</param>
        public ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet(TimeSeriesResponseDict actualInstance)
        {
            IsNullable = false;
            SchemaType= "anyOf";
            ActualInstance = actualInstance ?? throw new ArgumentException("Invalid instance found. Must not be null.");
        }


        private Object _actualInstance;

        /// <summary>
        /// Gets or Sets ActualInstance
        /// </summary>
        public override Object ActualInstance
        {
            get
            {
                return _actualInstance;
            }
            set
            {
                if (value.GetType() == typeof(TimeSeriesResponseDict))
                {
                    _actualInstance = value;
                }
                else if (value.GetType() == typeof(TimeSeriesResponsePublicHeartRateRow))
                {
                    _actualInstance = value;
                }
                else
                {
                    throw new ArgumentException("Invalid instance found. Must be the following types: TimeSeriesResponseDict, TimeSeriesResponsePublicHeartRateRow");
                }
            }
        }

        /// <summary>
        /// Get the actual instance of `TimeSeriesResponsePublicHeartRateRow`. If the actual instance is not `TimeSeriesResponsePublicHeartRateRow`,
        /// the InvalidClassException will be thrown
        /// </summary>
        /// <returns>An instance of TimeSeriesResponsePublicHeartRateRow</returns>
        public TimeSeriesResponsePublicHeartRateRow GetTimeSeriesResponsePublicHeartRateRow()
        {
            return (TimeSeriesResponsePublicHeartRateRow)ActualInstance;
        }

        /// <summary>
        /// Get the actual instance of `TimeSeriesResponseDict`. If the actual instance is not `TimeSeriesResponseDict`,
        /// the InvalidClassException will be thrown
        /// </summary>
        /// <returns>An instance of TimeSeriesResponseDict</returns>
        public TimeSeriesResponseDict GetTimeSeriesResponseDict()
        {
            return (TimeSeriesResponseDict)ActualInstance;
        }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            var sb = new StringBuilder();
            sb.Append("class ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet {\n");
            sb.Append("  ActualInstance: ").Append(ActualInstance).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public override string ToJson()
        {
            return JsonConvert.SerializeObject(ActualInstance, ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet.SerializerSettings);
        }

        /// <summary>
        /// Converts the JSON string into an instance of ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet
        /// </summary>
        /// <param name="jsonString">JSON string</param>
        /// <returns>An instance of ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet</returns>
        public static ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet FromJson(string jsonString)
        {
            ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet newResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet = null;

            if (string.IsNullOrEmpty(jsonString))
            {
                return newResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet;
            }

            try
            {
                newResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet = new ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet(JsonConvert.DeserializeObject<TimeSeriesResponseDict>(jsonString, ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet.SerializerSettings));
                // deserialization is considered successful at this point if no exception has been thrown.
                return newResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet;
            }
            catch (Exception exception)
            {
                // deserialization failed, try the next one
                System.Diagnostics.Debug.WriteLine(string.Format("Failed to deserialize `{0}` into TimeSeriesResponseDict: {1}", jsonString, exception.ToString()));
            }

            try
            {
                newResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet = new ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet(JsonConvert.DeserializeObject<TimeSeriesResponsePublicHeartRateRow>(jsonString, ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet.SerializerSettings));
                // deserialization is considered successful at this point if no exception has been thrown.
                return newResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet;
            }
            catch (Exception exception)
            {
                // deserialization failed, try the next one
                System.Diagnostics.Debug.WriteLine(string.Format("Failed to deserialize `{0}` into TimeSeriesResponsePublicHeartRateRow: {1}", jsonString, exception.ToString()));
            }

            // no match found, throw an exception
            throw new InvalidDataException("The JSON string `" + jsonString + "` cannot be deserialized into any schema defined.");
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<System.ComponentModel.DataAnnotations.ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            yield break;
        }
    }

    /// <summary>
    /// Custom JSON converter for ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet
    /// </summary>
    public class ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGetJsonConverter : JsonConverter
    {
        /// <summary>
        /// To write the JSON string
        /// </summary>
        /// <param name="writer">JSON writer</param>
        /// <param name="value">Object to be converted into a JSON string</param>
        /// <param name="serializer">JSON Serializer</param>
        public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
        {
            writer.WriteRawValue((string)(typeof(ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet).GetMethod("ToJson").Invoke(value, null)));
        }

        /// <summary>
        /// To convert a JSON string into an object
        /// </summary>
        /// <param name="reader">JSON reader</param>
        /// <param name="objectType">Object type</param>
        /// <param name="existingValue">Existing value</param>
        /// <param name="serializer">JSON Serializer</param>
        /// <returns>The object converted from the JSON string</returns>
        public override object ReadJson(JsonReader reader, Type objectType, object existingValue, JsonSerializer serializer)
        {
            switch(reader.TokenType) 
            {
                case JsonToken.StartObject:
                    return ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet.FromJson(JObject.Load(reader).ToString(Formatting.None));
                case JsonToken.StartArray:
                    return ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet.FromJson(JArray.Load(reader).ToString(Formatting.None));
                default:
                    return null;
            }
        }

        /// <summary>
        /// Check if the object can be converted
        /// </summary>
        /// <param name="objectType">Object type</param>
        /// <returns>True if the object can be converted</returns>
        public override bool CanConvert(Type objectType)
        {
            return false;
        }
    }

}
