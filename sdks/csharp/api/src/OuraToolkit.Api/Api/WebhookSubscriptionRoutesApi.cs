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
    public interface IWebhookSubscriptionRoutesApiSync : IApiAccessor
    {
        #region Synchronous Operations
        /// <summary>
        /// Create Webhook Subscription
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        WebhookSubscriptionModel CreateWebhookSubscriptionV2WebhookSubscriptionPost(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest);

        /// <summary>
        /// Create Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        ApiResponse<WebhookSubscriptionModel> CreateWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfo(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest);
        /// <summary>
        /// Delete Webhook Subscription
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns></returns>
        void DeleteWebhookSubscriptionV2WebhookSubscriptionIdDelete(string id);

        /// <summary>
        /// Delete Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>ApiResponse of Object(void)</returns>
        ApiResponse<Object> DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfo(string id);
        /// <summary>
        /// Get Webhook Subscription
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        WebhookSubscriptionModel GetWebhookSubscriptionV2WebhookSubscriptionIdGet(string id);

        /// <summary>
        /// Get Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        ApiResponse<WebhookSubscriptionModel> GetWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfo(string id);
        /// <summary>
        /// List Webhook Subscriptions
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <returns>List&lt;WebhookSubscriptionModel&gt;</returns>
        List<WebhookSubscriptionModel> ListWebhookSubscriptionsV2WebhookSubscriptionGet();

        /// <summary>
        /// List Webhook Subscriptions
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <returns>ApiResponse of List&lt;WebhookSubscriptionModel&gt;</returns>
        ApiResponse<List<WebhookSubscriptionModel>> ListWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfo();
        /// <summary>
        /// Renew Webhook Subscription
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        WebhookSubscriptionModel RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut(string id);

        /// <summary>
        /// Renew Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        ApiResponse<WebhookSubscriptionModel> RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfo(string id);
        /// <summary>
        /// Update Webhook Subscription
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        WebhookSubscriptionModel UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest);

        /// <summary>
        /// Update Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        ApiResponse<WebhookSubscriptionModel> UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfo(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest);
        #endregion Synchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IWebhookSubscriptionRoutesApiAsync : IApiAccessor
    {
        #region Asynchronous Operations
        /// <summary>
        /// Create Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        System.Threading.Tasks.Task<WebhookSubscriptionModel> CreateWebhookSubscriptionV2WebhookSubscriptionPostAsync(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Create Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<WebhookSubscriptionModel>> CreateWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfoAsync(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Delete Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of void</returns>
        System.Threading.Tasks.Task DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteAsync(string id, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Delete Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse</returns>
        System.Threading.Tasks.Task<ApiResponse<Object>> DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfoAsync(string id, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Get Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        System.Threading.Tasks.Task<WebhookSubscriptionModel> GetWebhookSubscriptionV2WebhookSubscriptionIdGetAsync(string id, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Get Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<WebhookSubscriptionModel>> GetWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfoAsync(string id, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// List Webhook Subscriptions
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of List&lt;WebhookSubscriptionModel&gt;</returns>
        System.Threading.Tasks.Task<List<WebhookSubscriptionModel>> ListWebhookSubscriptionsV2WebhookSubscriptionGetAsync(System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// List Webhook Subscriptions
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (List&lt;WebhookSubscriptionModel&gt;)</returns>
        System.Threading.Tasks.Task<ApiResponse<List<WebhookSubscriptionModel>>> ListWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfoAsync(System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Renew Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        System.Threading.Tasks.Task<WebhookSubscriptionModel> RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutAsync(string id, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Renew Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<WebhookSubscriptionModel>> RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfoAsync(string id, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Update Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        System.Threading.Tasks.Task<WebhookSubscriptionModel> UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutAsync(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Update Webhook Subscription
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<WebhookSubscriptionModel>> UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfoAsync(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default);
        #endregion Asynchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface IWebhookSubscriptionRoutesApi : IWebhookSubscriptionRoutesApiSync, IWebhookSubscriptionRoutesApiAsync
    {

    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public partial class WebhookSubscriptionRoutesApi : IDisposable, IWebhookSubscriptionRoutesApi
    {
        private OuraToolkit.Api.Client.ExceptionFactory _exceptionFactory = (name, response) => null;

        /// <summary>
        /// Initializes a new instance of the <see cref="WebhookSubscriptionRoutesApi"/> class.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <returns></returns>
        public WebhookSubscriptionRoutesApi() : this((string)null)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="WebhookSubscriptionRoutesApi"/> class.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <param name="basePath">The target service's base path in URL format.</param>
        /// <exception cref="ArgumentException"></exception>
        /// <returns></returns>
        public WebhookSubscriptionRoutesApi(string basePath)
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
        /// Initializes a new instance of the <see cref="WebhookSubscriptionRoutesApi"/> class using Configuration object.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <param name="configuration">An instance of Configuration.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <returns></returns>
        public WebhookSubscriptionRoutesApi(OuraToolkit.Api.Client.Configuration configuration)
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
        /// Initializes a new instance of the <see cref="WebhookSubscriptionRoutesApi"/> class.
        /// </summary>
        /// <param name="client">An instance of HttpClient.</param>
        /// <param name="handler">An optional instance of HttpClientHandler that is used by HttpClient.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <returns></returns>
        /// <remarks>
        /// Some configuration settings will not be applied without passing an HttpClientHandler.
        /// The features affected are: Setting and Retrieving Cookies, Client Certificates, Proxy settings.
        /// </remarks>
        public WebhookSubscriptionRoutesApi(HttpClient client, HttpClientHandler handler = null) : this(client, (string)null, handler)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="WebhookSubscriptionRoutesApi"/> class.
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
        public WebhookSubscriptionRoutesApi(HttpClient client, string basePath, HttpClientHandler handler = null)
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
        /// Initializes a new instance of the <see cref="WebhookSubscriptionRoutesApi"/> class using Configuration object.
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
        public WebhookSubscriptionRoutesApi(HttpClient client, OuraToolkit.Api.Client.Configuration configuration, HttpClientHandler handler = null)
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
        /// Initializes a new instance of the <see cref="WebhookSubscriptionRoutesApi"/> class
        /// using a Configuration object and client instance.
        /// </summary>
        /// <param name="client">The client interface for synchronous API access.</param>
        /// <param name="asyncClient">The client interface for asynchronous API access.</param>
        /// <param name="configuration">The configuration object.</param>
        /// <exception cref="ArgumentNullException"></exception>
        public WebhookSubscriptionRoutesApi(OuraToolkit.Api.Client.ISynchronousClient client, OuraToolkit.Api.Client.IAsynchronousClient asyncClient, OuraToolkit.Api.Client.IReadableConfiguration configuration)
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
        /// Create Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        public WebhookSubscriptionModel CreateWebhookSubscriptionV2WebhookSubscriptionPost(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = CreateWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfo(createWebhookSubscriptionRequest);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Create Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> CreateWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfo(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest)
        {
            // verify the required parameter 'createWebhookSubscriptionRequest' is set
            if (createWebhookSubscriptionRequest == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'createWebhookSubscriptionRequest' when calling WebhookSubscriptionRoutesApi->CreateWebhookSubscriptionV2WebhookSubscriptionPost");

            OuraToolkit.Api.Client.RequestOptions localVarRequestOptions = new OuraToolkit.Api.Client.RequestOptions();

            string[] _contentTypes = new string[] {
                "application/json"
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = OuraToolkit.Api.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null) localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);

            var localVarAccept = OuraToolkit.Api.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null) localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);

            localVarRequestOptions.Data = createWebhookSubscriptionRequest;

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request
            var localVarResponse = this.Client.Post<WebhookSubscriptionModel>("/v2/webhook/subscription", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("CreateWebhookSubscriptionV2WebhookSubscriptionPost", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Create Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        public async System.Threading.Tasks.Task<WebhookSubscriptionModel> CreateWebhookSubscriptionV2WebhookSubscriptionPostAsync(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = await CreateWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfoAsync(createWebhookSubscriptionRequest, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Create Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="createWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel>> CreateWebhookSubscriptionV2WebhookSubscriptionPostWithHttpInfoAsync(CreateWebhookSubscriptionRequest createWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'createWebhookSubscriptionRequest' is set
            if (createWebhookSubscriptionRequest == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'createWebhookSubscriptionRequest' when calling WebhookSubscriptionRoutesApi->CreateWebhookSubscriptionV2WebhookSubscriptionPost");


            OuraToolkit.Api.Client.RequestOptions localVarRequestOptions = new OuraToolkit.Api.Client.RequestOptions();

            string[] _contentTypes = new string[] {
                "application/json"
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };


            var localVarContentType = OuraToolkit.Api.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null) localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);

            var localVarAccept = OuraToolkit.Api.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null) localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);

            localVarRequestOptions.Data = createWebhookSubscriptionRequest;

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.PostAsync<WebhookSubscriptionModel>("/v2/webhook/subscription", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("CreateWebhookSubscriptionV2WebhookSubscriptionPost", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Delete Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns></returns>
        public void DeleteWebhookSubscriptionV2WebhookSubscriptionIdDelete(string id)
        {
            DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfo(id);
        }

        /// <summary>
        /// Delete Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>ApiResponse of Object(void)</returns>
        public OuraToolkit.Api.Client.ApiResponse<Object> DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfo(string id)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->DeleteWebhookSubscriptionV2WebhookSubscriptionIdDelete");

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

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request
            var localVarResponse = this.Client.Delete<Object>("/v2/webhook/subscription/{id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("DeleteWebhookSubscriptionV2WebhookSubscriptionIdDelete", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Delete Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of void</returns>
        public async System.Threading.Tasks.Task DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteAsync(string id, System.Threading.CancellationToken cancellationToken = default)
        {
            await DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfoAsync(id, cancellationToken).ConfigureAwait(false);
        }

        /// <summary>
        /// Delete Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<Object>> DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteWithHttpInfoAsync(string id, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->DeleteWebhookSubscriptionV2WebhookSubscriptionIdDelete");


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

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.DeleteAsync<Object>("/v2/webhook/subscription/{id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("DeleteWebhookSubscriptionV2WebhookSubscriptionIdDelete", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Get Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        public WebhookSubscriptionModel GetWebhookSubscriptionV2WebhookSubscriptionIdGet(string id)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = GetWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfo(id);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Get Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> GetWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfo(string id)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->GetWebhookSubscriptionV2WebhookSubscriptionIdGet");

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

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<WebhookSubscriptionModel>("/v2/webhook/subscription/{id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("GetWebhookSubscriptionV2WebhookSubscriptionIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Get Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        public async System.Threading.Tasks.Task<WebhookSubscriptionModel> GetWebhookSubscriptionV2WebhookSubscriptionIdGetAsync(string id, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = await GetWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfoAsync(id, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Get Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel>> GetWebhookSubscriptionV2WebhookSubscriptionIdGetWithHttpInfoAsync(string id, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->GetWebhookSubscriptionV2WebhookSubscriptionIdGet");


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

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<WebhookSubscriptionModel>("/v2/webhook/subscription/{id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("GetWebhookSubscriptionV2WebhookSubscriptionIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// List Webhook Subscriptions 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <returns>List&lt;WebhookSubscriptionModel&gt;</returns>
        public List<WebhookSubscriptionModel> ListWebhookSubscriptionsV2WebhookSubscriptionGet()
        {
            OuraToolkit.Api.Client.ApiResponse<List<WebhookSubscriptionModel>> localVarResponse = ListWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfo();
            return localVarResponse.Data;
        }

        /// <summary>
        /// List Webhook Subscriptions 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <returns>ApiResponse of List&lt;WebhookSubscriptionModel&gt;</returns>
        public OuraToolkit.Api.Client.ApiResponse<List<WebhookSubscriptionModel>> ListWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfo()
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


            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<List<WebhookSubscriptionModel>>("/v2/webhook/subscription", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("ListWebhookSubscriptionsV2WebhookSubscriptionGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// List Webhook Subscriptions 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of List&lt;WebhookSubscriptionModel&gt;</returns>
        public async System.Threading.Tasks.Task<List<WebhookSubscriptionModel>> ListWebhookSubscriptionsV2WebhookSubscriptionGetAsync(System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<List<WebhookSubscriptionModel>> localVarResponse = await ListWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfoAsync(cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// List Webhook Subscriptions 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (List&lt;WebhookSubscriptionModel&gt;)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<List<WebhookSubscriptionModel>>> ListWebhookSubscriptionsV2WebhookSubscriptionGetWithHttpInfoAsync(System.Threading.CancellationToken cancellationToken = default)
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


            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<List<WebhookSubscriptionModel>>("/v2/webhook/subscription", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("ListWebhookSubscriptionsV2WebhookSubscriptionGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Renew Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        public WebhookSubscriptionModel RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut(string id)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfo(id);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Renew Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfo(string id)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut");

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

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request
            var localVarResponse = this.Client.Put<WebhookSubscriptionModel>("/v2/webhook/subscription/renew/{id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Renew Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        public async System.Threading.Tasks.Task<WebhookSubscriptionModel> RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutAsync(string id, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = await RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfoAsync(id, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Renew Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel>> RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutWithHttpInfoAsync(string id, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut");


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

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.PutAsync<WebhookSubscriptionModel>("/v2/webhook/subscription/renew/{id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Update Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <returns>WebhookSubscriptionModel</returns>
        public WebhookSubscriptionModel UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfo(id, updateWebhookSubscriptionRequest);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Update Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <returns>ApiResponse of WebhookSubscriptionModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfo(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut");

            // verify the required parameter 'updateWebhookSubscriptionRequest' is set
            if (updateWebhookSubscriptionRequest == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'updateWebhookSubscriptionRequest' when calling WebhookSubscriptionRoutesApi->UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut");

            OuraToolkit.Api.Client.RequestOptions localVarRequestOptions = new OuraToolkit.Api.Client.RequestOptions();

            string[] _contentTypes = new string[] {
                "application/json"
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };

            var localVarContentType = OuraToolkit.Api.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null) localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);

            var localVarAccept = OuraToolkit.Api.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null) localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter
            localVarRequestOptions.Data = updateWebhookSubscriptionRequest;

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request
            var localVarResponse = this.Client.Put<WebhookSubscriptionModel>("/v2/webhook/subscription/{id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Update Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of WebhookSubscriptionModel</returns>
        public async System.Threading.Tasks.Task<WebhookSubscriptionModel> UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutAsync(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel> localVarResponse = await UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfoAsync(id, updateWebhookSubscriptionRequest, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Update Webhook Subscription 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="id"></param>
        /// <param name="updateWebhookSubscriptionRequest"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (WebhookSubscriptionModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<WebhookSubscriptionModel>> UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutWithHttpInfoAsync(string id, UpdateWebhookSubscriptionRequest updateWebhookSubscriptionRequest, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'id' is set
            if (id == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'id' when calling WebhookSubscriptionRoutesApi->UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut");

            // verify the required parameter 'updateWebhookSubscriptionRequest' is set
            if (updateWebhookSubscriptionRequest == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'updateWebhookSubscriptionRequest' when calling WebhookSubscriptionRoutesApi->UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut");


            OuraToolkit.Api.Client.RequestOptions localVarRequestOptions = new OuraToolkit.Api.Client.RequestOptions();

            string[] _contentTypes = new string[] {
                "application/json"
            };

            // to determine the Accept header
            string[] _accepts = new string[] {
                "application/json"
            };


            var localVarContentType = OuraToolkit.Api.Client.ClientUtils.SelectHeaderContentType(_contentTypes);
            if (localVarContentType != null) localVarRequestOptions.HeaderParameters.Add("Content-Type", localVarContentType);

            var localVarAccept = OuraToolkit.Api.Client.ClientUtils.SelectHeaderAccept(_accepts);
            if (localVarAccept != null) localVarRequestOptions.HeaderParameters.Add("Accept", localVarAccept);

            localVarRequestOptions.PathParameters.Add("id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(id)); // path parameter
            localVarRequestOptions.Data = updateWebhookSubscriptionRequest;

            // authentication (ClientIdAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-id")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-id", this.Configuration.GetApiKeyWithPrefix("x-client-id"));
            }
            // authentication (ClientSecretAuth) required
            if (!string.IsNullOrEmpty(this.Configuration.GetApiKeyWithPrefix("x-client-secret")))
            {
                localVarRequestOptions.HeaderParameters.Add("x-client-secret", this.Configuration.GetApiKeyWithPrefix("x-client-secret"));
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.PutAsync<WebhookSubscriptionModel>("/v2/webhook/subscription/{id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

    }
}
