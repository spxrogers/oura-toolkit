/*
 * Oura API Documentation
 *
 * # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | - -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- - | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 
 *
 * The version of the OpenAPI document: 2.0
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Net.Mime;
using OuraToolkit.Api.Client;
using OuraToolkit.Api.Model;

namespace OuraToolkit.Api.Api
{

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IRingBatteryLevelRoutesApiSync : IApiAccessor
    {
        #region Synchronous Operations
        /// <summary>
        /// Multiple Ring Battery Level Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <returns>ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet</returns>
        ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default);

        /// <summary>
        /// Multiple Ring Battery Level Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <returns>ApiResponse of ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet</returns>
        ApiResponse<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet> MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetWithHttpInfo(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default);
        #endregion Synchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IRingBatteryLevelRoutesApiAsync : IApiAccessor
    {
        #region Asynchronous Operations
        /// <summary>
        /// Multiple Ring Battery Level Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet</returns>
        System.Threading.Tasks.Task<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet> MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Multiple Ring Battery Level Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet)</returns>
        System.Threading.Tasks.Task<ApiResponse<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet>> MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetWithHttpInfoAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default, System.Threading.CancellationToken cancellationToken = default);
        #endregion Asynchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IRingBatteryLevelRoutesApi : IRingBatteryLevelRoutesApiSync, IRingBatteryLevelRoutesApiAsync
    {

    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public partial class RingBatteryLevelRoutesApi : IDisposable, IRingBatteryLevelRoutesApi
    {
        private OuraToolkit.Api.Client.ExceptionFactory _exceptionFactory = (name, response) => null;

        /// <summary>
        /// Initializes a new instance of the <see cref="RingBatteryLevelRoutesApi"/> class.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <returns></returns>
        public RingBatteryLevelRoutesApi() : this((string)null)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="RingBatteryLevelRoutesApi"/> class.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <param name="basePath">The target service's base path in URL format.</param>
        /// <exception cref="ArgumentException"></exception>
        /// <returns></returns>
        public RingBatteryLevelRoutesApi(string basePath)
        {
            this.Configuration = OuraToolkit.Api.Client.Configuration.MergeConfigurations(
                OuraToolkit.Api.Client.GlobalConfiguration.Instance,
                new OuraToolkit.Api.Client.Configuration { BasePath = basePath }
            );
            this.ApiClient = new OuraToolkit.Api.Client.ApiClient(this.Configuration.BasePath);
            this.Client =  this.ApiClient;
            this.AsynchronousClient = this.ApiClient;
            this.ExceptionFactory = OuraToolkit.Api.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="RingBatteryLevelRoutesApi"/> class using Configuration object.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <param name="configuration">An instance of Configuration.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <returns></returns>
        public RingBatteryLevelRoutesApi(OuraToolkit.Api.Client.Configuration configuration)
        {
            if (configuration == null) throw new ArgumentNullException("configuration");

            this.Configuration = OuraToolkit.Api.Client.Configuration.MergeConfigurations(
                OuraToolkit.Api.Client.GlobalConfiguration.Instance,
                configuration
            );
            this.ApiClient = new OuraToolkit.Api.Client.ApiClient(this.Configuration.BasePath);
            this.Client = this.ApiClient;
            this.AsynchronousClient = this.ApiClient;
            ExceptionFactory = OuraToolkit.Api.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="RingBatteryLevelRoutesApi"/> class.
        /// </summary>
        /// <param name="client">An instance of HttpClient.</param>
        /// <param name="handler">An optional instance of HttpClientHandler that is used by HttpClient.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <returns></returns>
        /// <remarks>
        /// Some configuration settings will not be applied without passing an HttpClientHandler.
        /// The features affected are: Setting and Retrieving Cookies, Client Certificates, Proxy settings.
        /// </remarks>
        public RingBatteryLevelRoutesApi(HttpClient client, HttpClientHandler handler = null) : this(client, (string)null, handler)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="RingBatteryLevelRoutesApi"/> class.
        /// </summary>
        /// <param name="client">An instance of HttpClient.</param>
        /// <param name="basePath">The target service's base path in URL format.</param>
        /// <param name="handler">An optional instance of HttpClientHandler that is used by HttpClient.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <exception cref="ArgumentException"></exception>
        /// <returns></returns>
        /// <remarks>
        /// Some configuration settings will not be applied without passing an HttpClientHandler.
        /// The features affected are: Setting and Retrieving Cookies, Client Certificates, Proxy settings.
        /// </remarks>
        public RingBatteryLevelRoutesApi(HttpClient client, string basePath, HttpClientHandler handler = null)
        {
            if (client == null) throw new ArgumentNullException("client");

            this.Configuration = OuraToolkit.Api.Client.Configuration.MergeConfigurations(
                OuraToolkit.Api.Client.GlobalConfiguration.Instance,
                new OuraToolkit.Api.Client.Configuration { BasePath = basePath }
            );
            this.ApiClient = new OuraToolkit.Api.Client.ApiClient(client, this.Configuration.BasePath, handler);
            this.Client =  this.ApiClient;
            this.AsynchronousClient = this.ApiClient;
            this.ExceptionFactory = OuraToolkit.Api.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="RingBatteryLevelRoutesApi"/> class using Configuration object.
        /// </summary>
        /// <param name="client">An instance of HttpClient.</param>
        /// <param name="configuration">An instance of Configuration.</param>
        /// <param name="handler">An optional instance of HttpClientHandler that is used by HttpClient.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <returns></returns>
        /// <remarks>
        /// Some configuration settings will not be applied without passing an HttpClientHandler.
        /// The features affected are: Setting and Retrieving Cookies, Client Certificates, Proxy settings.
        /// </remarks>
        public RingBatteryLevelRoutesApi(HttpClient client, OuraToolkit.Api.Client.Configuration configuration, HttpClientHandler handler = null)
        {
            if (configuration == null) throw new ArgumentNullException("configuration");
            if (client == null) throw new ArgumentNullException("client");

            this.Configuration = OuraToolkit.Api.Client.Configuration.MergeConfigurations(
                OuraToolkit.Api.Client.GlobalConfiguration.Instance,
                configuration
            );
            this.ApiClient = new OuraToolkit.Api.Client.ApiClient(client, this.Configuration.BasePath, handler);
            this.Client = this.ApiClient;
            this.AsynchronousClient = this.ApiClient;
            ExceptionFactory = OuraToolkit.Api.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="RingBatteryLevelRoutesApi"/> class
        /// using a Configuration object and client instance.
        /// </summary>
        /// <param name="client">The client interface for synchronous API access.</param>
        /// <param name="asyncClient">The client interface for asynchronous API access.</param>
        /// <param name="configuration">The configuration object.</param>
        /// <exception cref="ArgumentNullException"></exception>
        public RingBatteryLevelRoutesApi(OuraToolkit.Api.Client.ISynchronousClient client, OuraToolkit.Api.Client.IAsynchronousClient asyncClient, OuraToolkit.Api.Client.IReadableConfiguration configuration)
        {
            if (client == null) throw new ArgumentNullException("client");
            if (asyncClient == null) throw new ArgumentNullException("asyncClient");
            if (configuration == null) throw new ArgumentNullException("configuration");

            this.Client = client;
            this.AsynchronousClient = asyncClient;
            this.Configuration = configuration;
            this.ExceptionFactory = OuraToolkit.Api.Client.Configuration.DefaultExceptionFactory;
        }

        /// <summary>
        /// Disposes resources if they were created by us
        /// </summary>
        public void Dispose()
        {
            this.ApiClient?.Dispose();
        }

        /// <summary>
        /// Holds the ApiClient if created
        /// </summary>
        public OuraToolkit.Api.Client.ApiClient ApiClient { get; set; } = null;

        /// <summary>
        /// The client for accessing this underlying API asynchronously.
        /// </summary>
        public OuraToolkit.Api.Client.IAsynchronousClient AsynchronousClient { get; set; }

        /// <summary>
        /// The client for accessing this underlying API synchronously.
        /// </summary>
        public OuraToolkit.Api.Client.ISynchronousClient Client { get; set; }

        /// <summary>
        /// Gets the base path of the API client.
        /// </summary>
        /// <value>The base path</value>
        public string GetBasePath()
        {
            return this.Configuration.BasePath;
        }

        /// <summary>
        /// Gets or sets the configuration object
        /// </summary>
        /// <value>An instance of the Configuration</value>
        public OuraToolkit.Api.Client.IReadableConfiguration Configuration { get; set; }

        /// <summary>
        /// Provides a factory method hook for the creation of exceptions.
        /// </summary>
        public OuraToolkit.Api.Client.ExceptionFactory ExceptionFactory
        {
            get
            {
                if (_exceptionFactory != null && _exceptionFactory.GetInvocationList().Length > 1)
                {
                    throw new InvalidOperationException("Multicast delegate for ExceptionFactory is unsupported.");
                }
                return _exceptionFactory;
            }
            set { _exceptionFactory = value; }
        }

        /// <summary>
        /// Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <returns>ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet</returns>
        public ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default)
        {
            OuraToolkit.Api.Client.ApiResponse<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet> localVarResponse = MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetWithHttpInfo(startDatetime, endDatetime, nextToken, latest, fields);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <returns>ApiResponse of ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet</returns>
        public OuraToolkit.Api.Client.ApiResponse<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet> MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetWithHttpInfo(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default)
        {
            OuraToolkit.Api.Client.RequestOptions localVarRequestOptions = new OuraToolkit.Api.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = OuraToolkit.Api.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null) localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);

            var localVarAccept = OuraToolkit.Api.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null) localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);

            if (startDatetime != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_datetime", startDatetime));
            }
            if (endDatetime != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_datetime", endDatetime));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }
            if (latest != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "latest", latest));
            }
            if (fields != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "fields", fields));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet>("/v2/usercollection/ring_battery_level", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet</returns>
        public async System.Threading.Tasks.Task<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet> MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet> localVarResponse = await MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetWithHttpInfoAsync(startDatetime, endDatetime, nextToken, latest, fields, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="latest">If True, returns most recent sample. (optional)</param>
        /// <param name="fields">Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet>> MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGetWithHttpInfoAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, bool? latest = default, string? fields = default, System.Threading.CancellationToken cancellationToken = default)
        {

            OuraToolkit.Api.Client.RequestOptions localVarRequestOptions = new OuraToolkit.Api.Client.RequestOptions();

            string[] _contentTypes = new string[] {
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };


            var localVarContentType = OuraToolkit.Api.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null) localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);

            var localVarAccept = OuraToolkit.Api.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null) localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);

            if (startDatetime != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_datetime", startDatetime));
            }
            if (endDatetime != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_datetime", endDatetime));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }
            if (latest != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "latest", latest));
            }
            if (fields != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "fields", fields));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet>("/v2/usercollection/ring_battery_level", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

    }
}
