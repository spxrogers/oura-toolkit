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
    public interface ISandboxRoutesApiSync : IApiAccessor
    {
        #region Synchronous Operations
        /// <summary>
        /// Sandbox - Multiple Daily Activity Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyActivity</returns>
        MultiDocumentResponsePublicDailyActivity SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Activity Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyActivity</returns>
        ApiResponse<MultiDocumentResponsePublicDailyActivity> SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyCardiovascularAge</returns>
        MultiDocumentResponsePublicDailyCardiovascularAge SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyCardiovascularAge</returns>
        ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge> SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyReadiness</returns>
        MultiDocumentResponsePublicDailyReadiness SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyReadiness</returns>
        ApiResponse<MultiDocumentResponsePublicDailyReadiness> SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponseDailyResilienceModel</returns>
        MultiDocumentResponseDailyResilienceModel SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponseDailyResilienceModel</returns>
        ApiResponse<MultiDocumentResponseDailyResilienceModel> SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailySleep</returns>
        MultiDocumentResponsePublicDailySleep SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailySleep</returns>
        ApiResponse<MultiDocumentResponsePublicDailySleep> SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailySpO2</returns>
        MultiDocumentResponsePublicDailySpO2 SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailySpO2</returns>
        ApiResponse<MultiDocumentResponsePublicDailySpO2> SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyStress</returns>
        MultiDocumentResponsePublicDailyStress SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyStress</returns>
        ApiResponse<MultiDocumentResponsePublicDailyStress> SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponseEnhancedTagModel</returns>
        MultiDocumentResponseEnhancedTagModel SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponseEnhancedTagModel</returns>
        ApiResponse<MultiDocumentResponseEnhancedTagModel> SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Heartrate Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet</returns>
        ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Heartrate Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet</returns>
        ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfo(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicRestModePeriod</returns>
        MultiDocumentResponsePublicRestModePeriod SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicRestModePeriod</returns>
        ApiResponse<MultiDocumentResponsePublicRestModePeriod> SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet</returns>
        ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet</returns>
        ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfo(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicRingConfiguration</returns>
        MultiDocumentResponsePublicRingConfiguration SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet(string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicRingConfiguration</returns>
        ApiResponse<MultiDocumentResponsePublicRingConfiguration> SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfo(string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Session Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicSession</returns>
        MultiDocumentResponsePublicSession SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Session Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicSession</returns>
        ApiResponse<MultiDocumentResponsePublicSession> SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Sleep Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicModifiedSleepModel</returns>
        MultiDocumentResponsePublicModifiedSleepModel SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Sleep Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicModifiedSleepModel</returns>
        ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicSleepTime</returns>
        MultiDocumentResponsePublicSleepTime SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicSleepTime</returns>
        ApiResponse<MultiDocumentResponsePublicSleepTime> SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Tag Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponseTagModel</returns>
        MultiDocumentResponseTagModel SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Tag Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponseTagModel</returns>
        ApiResponse<MultiDocumentResponseTagModel> SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicVO2Max</returns>
        MultiDocumentResponsePublicVO2Max SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicVO2Max</returns>
        ApiResponse<MultiDocumentResponsePublicVO2Max> SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Multiple Workout Documents
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicWorkout</returns>
        MultiDocumentResponsePublicWorkout SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);

        /// <summary>
        /// Sandbox - Multiple Workout Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicWorkout</returns>
        ApiResponse<MultiDocumentResponsePublicWorkout> SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default);
        /// <summary>
        /// Sandbox - Single Daily Activity Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyActivity</returns>
        PublicDailyActivity SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Daily Activity Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyActivity</returns>
        ApiResponse<PublicDailyActivity> SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyCardiovascularAge</returns>
        PublicDailyCardiovascularAge SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyCardiovascularAge</returns>
        ApiResponse<PublicDailyCardiovascularAge> SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Daily Readiness Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyReadiness</returns>
        PublicDailyReadiness SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Daily Readiness Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyReadiness</returns>
        ApiResponse<PublicDailyReadiness> SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Daily Resilience Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>DailyResilienceModel</returns>
        DailyResilienceModel SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Daily Resilience Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of DailyResilienceModel</returns>
        ApiResponse<DailyResilienceModel> SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Daily Sleep Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailySleep</returns>
        PublicDailySleep SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Daily Sleep Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailySleep</returns>
        ApiResponse<PublicDailySleep> SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Daily Spo2 Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailySpO2</returns>
        PublicDailySpO2 SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Daily Spo2 Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailySpO2</returns>
        ApiResponse<PublicDailySpO2> SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Daily Stress Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyStress</returns>
        PublicDailyStress SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Daily Stress Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyStress</returns>
        ApiResponse<PublicDailyStress> SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Enhanced Tag Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>EnhancedTagModel</returns>
        EnhancedTagModel SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Enhanced Tag Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of EnhancedTagModel</returns>
        ApiResponse<EnhancedTagModel> SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Rest Mode Period Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicRestModePeriod</returns>
        PublicRestModePeriod SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Rest Mode Period Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicRestModePeriod</returns>
        ApiResponse<PublicRestModePeriod> SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Ring Configuration Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicRingConfiguration</returns>
        PublicRingConfiguration SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Ring Configuration Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicRingConfiguration</returns>
        ApiResponse<PublicRingConfiguration> SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Session Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicSession</returns>
        PublicSession SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Session Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicSession</returns>
        ApiResponse<PublicSession> SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Sleep Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicModifiedSleepModel</returns>
        PublicModifiedSleepModel SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Sleep Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicModifiedSleepModel</returns>
        ApiResponse<PublicModifiedSleepModel> SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Sleep Time Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicSleepTime</returns>
        PublicSleepTime SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Sleep Time Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicSleepTime</returns>
        ApiResponse<PublicSleepTime> SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Tag Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>TagModel</returns>
        TagModel SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Tag Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of TagModel</returns>
        ApiResponse<TagModel> SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Vo2 Max Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicVO2Max</returns>
        PublicVO2Max SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Vo2 Max Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicVO2Max</returns>
        ApiResponse<PublicVO2Max> SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfo(string documentId);
        /// <summary>
        /// Sandbox - Single Workout Document
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicWorkout</returns>
        PublicWorkout SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet(string documentId);

        /// <summary>
        /// Sandbox - Single Workout Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicWorkout</returns>
        ApiResponse<PublicWorkout> SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfo(string documentId);
        #endregion Synchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface ISandboxRoutesApiAsync : IApiAccessor
    {
        #region Asynchronous Operations
        /// <summary>
        /// Sandbox - Multiple Daily Activity Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyActivity</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyActivity> SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Activity Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyActivity)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicDailyActivity>> SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyCardiovascularAge</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyCardiovascularAge> SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyCardiovascularAge)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge>> SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyReadiness</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyReadiness> SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyReadiness)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicDailyReadiness>> SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponseDailyResilienceModel</returns>
        System.Threading.Tasks.Task<MultiDocumentResponseDailyResilienceModel> SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponseDailyResilienceModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponseDailyResilienceModel>> SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailySleep</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicDailySleep> SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailySleep)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicDailySleep>> SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailySpO2</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicDailySpO2> SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailySpO2)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicDailySpO2>> SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyStress</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyStress> SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyStress)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicDailyStress>> SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponseEnhancedTagModel</returns>
        System.Threading.Tasks.Task<MultiDocumentResponseEnhancedTagModel> SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponseEnhancedTagModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponseEnhancedTagModel>> SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Heartrate Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet</returns>
        System.Threading.Tasks.Task<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Heartrate Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet)</returns>
        System.Threading.Tasks.Task<ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>> SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfoAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicRestModePeriod</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicRestModePeriod> SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicRestModePeriod)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicRestModePeriod>> SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet</returns>
        System.Threading.Tasks.Task<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet)</returns>
        System.Threading.Tasks.Task<ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>> SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfoAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicRingConfiguration</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicRingConfiguration> SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetAsync(string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicRingConfiguration)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicRingConfiguration>> SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfoAsync(string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Session Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicSession</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicSession> SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Session Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicSession)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicSession>> SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Sleep Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicModifiedSleepModel</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicModifiedSleepModel> SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Sleep Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicModifiedSleepModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicModifiedSleepModel>> SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicSleepTime</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicSleepTime> SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicSleepTime)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicSleepTime>> SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Tag Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponseTagModel</returns>
        System.Threading.Tasks.Task<MultiDocumentResponseTagModel> SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Tag Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponseTagModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponseTagModel>> SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicVO2Max</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicVO2Max> SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicVO2Max)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicVO2Max>> SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Multiple Workout Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicWorkout</returns>
        System.Threading.Tasks.Task<MultiDocumentResponsePublicWorkout> SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Multiple Workout Documents
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicWorkout)</returns>
        System.Threading.Tasks.Task<ApiResponse<MultiDocumentResponsePublicWorkout>> SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Daily Activity Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyActivity</returns>
        System.Threading.Tasks.Task<PublicDailyActivity> SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Daily Activity Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyActivity)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicDailyActivity>> SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyCardiovascularAge</returns>
        System.Threading.Tasks.Task<PublicDailyCardiovascularAge> SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyCardiovascularAge)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicDailyCardiovascularAge>> SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Daily Readiness Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyReadiness</returns>
        System.Threading.Tasks.Task<PublicDailyReadiness> SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Daily Readiness Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyReadiness)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicDailyReadiness>> SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Daily Resilience Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of DailyResilienceModel</returns>
        System.Threading.Tasks.Task<DailyResilienceModel> SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Daily Resilience Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (DailyResilienceModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<DailyResilienceModel>> SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Daily Sleep Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailySleep</returns>
        System.Threading.Tasks.Task<PublicDailySleep> SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Daily Sleep Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailySleep)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicDailySleep>> SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Daily Spo2 Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailySpO2</returns>
        System.Threading.Tasks.Task<PublicDailySpO2> SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Daily Spo2 Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailySpO2)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicDailySpO2>> SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Daily Stress Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyStress</returns>
        System.Threading.Tasks.Task<PublicDailyStress> SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Daily Stress Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyStress)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicDailyStress>> SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Enhanced Tag Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of EnhancedTagModel</returns>
        System.Threading.Tasks.Task<EnhancedTagModel> SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Enhanced Tag Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (EnhancedTagModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<EnhancedTagModel>> SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Rest Mode Period Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicRestModePeriod</returns>
        System.Threading.Tasks.Task<PublicRestModePeriod> SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Rest Mode Period Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicRestModePeriod)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicRestModePeriod>> SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Ring Configuration Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicRingConfiguration</returns>
        System.Threading.Tasks.Task<PublicRingConfiguration> SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Ring Configuration Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicRingConfiguration)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicRingConfiguration>> SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Session Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicSession</returns>
        System.Threading.Tasks.Task<PublicSession> SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Session Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicSession)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicSession>> SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Sleep Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicModifiedSleepModel</returns>
        System.Threading.Tasks.Task<PublicModifiedSleepModel> SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Sleep Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicModifiedSleepModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicModifiedSleepModel>> SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Sleep Time Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicSleepTime</returns>
        System.Threading.Tasks.Task<PublicSleepTime> SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Sleep Time Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicSleepTime)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicSleepTime>> SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Tag Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of TagModel</returns>
        System.Threading.Tasks.Task<TagModel> SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Tag Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (TagModel)</returns>
        System.Threading.Tasks.Task<ApiResponse<TagModel>> SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Vo2 Max Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicVO2Max</returns>
        System.Threading.Tasks.Task<PublicVO2Max> SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Vo2 Max Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicVO2Max)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicVO2Max>> SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        /// <summary>
        /// Sandbox - Single Workout Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicWorkout</returns>
        System.Threading.Tasks.Task<PublicWorkout> SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);

        /// <summary>
        /// Sandbox - Single Workout Document
        /// </summary>
        /// <remarks>
        /// 
        /// </remarks>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicWorkout)</returns>
        System.Threading.Tasks.Task<ApiResponse<PublicWorkout>> SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default);
        #endregion Asynchronous Operations
    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public interface ISandboxRoutesApi : ISandboxRoutesApiSync, ISandboxRoutesApiAsync
    {

    }

    /// <summary>
    /// Represents a collection of functions to interact with the API endpoints
    /// </summary>
    public partial class SandboxRoutesApi : IDisposable, ISandboxRoutesApi
    {
        private OuraToolkit.Api.Client.ExceptionFactory _exceptionFactory = (name, response) => null;

        /// <summary>
        /// Initializes a new instance of the <see cref="SandboxRoutesApi"/> class.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <returns></returns>
        public SandboxRoutesApi() : this((string)null)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="SandboxRoutesApi"/> class.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <param name="basePath">The target service's base path in URL format.</param>
        /// <exception cref="ArgumentException"></exception>
        /// <returns></returns>
        public SandboxRoutesApi(string basePath)
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
        /// Initializes a new instance of the <see cref="SandboxRoutesApi"/> class using Configuration object.
        /// **IMPORTANT** This will also create an instance of HttpClient, which is less than ideal.
        /// It's better to reuse the <see href="https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net">HttpClient and HttpClientHandler</see>.
        /// </summary>
        /// <param name="configuration">An instance of Configuration.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <returns></returns>
        public SandboxRoutesApi(OuraToolkit.Api.Client.Configuration configuration)
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
        /// Initializes a new instance of the <see cref="SandboxRoutesApi"/> class.
        /// </summary>
        /// <param name="client">An instance of HttpClient.</param>
        /// <param name="handler">An optional instance of HttpClientHandler that is used by HttpClient.</param>
        /// <exception cref="ArgumentNullException"></exception>
        /// <returns></returns>
        /// <remarks>
        /// Some configuration settings will not be applied without passing an HttpClientHandler.
        /// The features affected are: Setting and Retrieving Cookies, Client Certificates, Proxy settings.
        /// </remarks>
        public SandboxRoutesApi(HttpClient client, HttpClientHandler handler = null) : this(client, (string)null, handler)
        {
        }

        /// <summary>
        /// Initializes a new instance of the <see cref="SandboxRoutesApi"/> class.
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
        public SandboxRoutesApi(HttpClient client, string basePath, HttpClientHandler handler = null)
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
        /// Initializes a new instance of the <see cref="SandboxRoutesApi"/> class using Configuration object.
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
        public SandboxRoutesApi(HttpClient client, OuraToolkit.Api.Client.Configuration configuration, HttpClientHandler handler = null)
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
        /// Initializes a new instance of the <see cref="SandboxRoutesApi"/> class
        /// using a Configuration object and client instance.
        /// </summary>
        /// <param name="client">The client interface for synchronous API access.</param>
        /// <param name="asyncClient">The client interface for asynchronous API access.</param>
        /// <param name="configuration">The configuration object.</param>
        /// <exception cref="ArgumentNullException"></exception>
        public SandboxRoutesApi(OuraToolkit.Api.Client.ISynchronousClient client, OuraToolkit.Api.Client.IAsynchronousClient asyncClient, OuraToolkit.Api.Client.IReadableConfiguration configuration)
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
        /// Sandbox - Multiple Daily Activity Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyActivity</returns>
        public MultiDocumentResponsePublicDailyActivity SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyActivity> localVarResponse = SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Activity Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyActivity</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyActivity> SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicDailyActivity>("/v2/sandbox/usercollection/daily_activity", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Activity Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyActivity</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyActivity> SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyActivity> localVarResponse = await SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Activity Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyActivity)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyActivity>> SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicDailyActivity>("/v2/sandbox/usercollection/daily_activity", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyCardiovascularAge</returns>
        public MultiDocumentResponsePublicDailyCardiovascularAge SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge> localVarResponse = SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyCardiovascularAge</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge> SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicDailyCardiovascularAge>("/v2/sandbox/usercollection/daily_cardiovascular_age", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyCardiovascularAge</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyCardiovascularAge> SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge> localVarResponse = await SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Cardiovascular Age Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyCardiovascularAge)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge>> SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicDailyCardiovascularAge>("/v2/sandbox/usercollection/daily_cardiovascular_age", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyReadiness</returns>
        public MultiDocumentResponsePublicDailyReadiness SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyReadiness> localVarResponse = SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyReadiness</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyReadiness> SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicDailyReadiness>("/v2/sandbox/usercollection/daily_readiness", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyReadiness</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyReadiness> SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyReadiness> localVarResponse = await SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Readiness Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyReadiness)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyReadiness>> SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicDailyReadiness>("/v2/sandbox/usercollection/daily_readiness", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponseDailyResilienceModel</returns>
        public MultiDocumentResponseDailyResilienceModel SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseDailyResilienceModel> localVarResponse = SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponseDailyResilienceModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseDailyResilienceModel> SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponseDailyResilienceModel>("/v2/sandbox/usercollection/daily_resilience", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponseDailyResilienceModel</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponseDailyResilienceModel> SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseDailyResilienceModel> localVarResponse = await SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Resilience Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponseDailyResilienceModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseDailyResilienceModel>> SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponseDailyResilienceModel>("/v2/sandbox/usercollection/daily_resilience", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailySleep</returns>
        public MultiDocumentResponsePublicDailySleep SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySleep> localVarResponse = SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailySleep</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySleep> SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicDailySleep>("/v2/sandbox/usercollection/daily_sleep", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailySleep</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicDailySleep> SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySleep> localVarResponse = await SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailySleep)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySleep>> SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicDailySleep>("/v2/sandbox/usercollection/daily_sleep", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailySpO2</returns>
        public MultiDocumentResponsePublicDailySpO2 SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySpO2> localVarResponse = SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailySpO2</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySpO2> SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicDailySpO2>("/v2/sandbox/usercollection/daily_spo2", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailySpO2</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicDailySpO2> SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySpO2> localVarResponse = await SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Spo2 Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailySpO2)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailySpO2>> SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicDailySpO2>("/v2/sandbox/usercollection/daily_spo2", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicDailyStress</returns>
        public MultiDocumentResponsePublicDailyStress SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyStress> localVarResponse = SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicDailyStress</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyStress> SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicDailyStress>("/v2/sandbox/usercollection/daily_stress", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicDailyStress</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicDailyStress> SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyStress> localVarResponse = await SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Daily Stress Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicDailyStress)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicDailyStress>> SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicDailyStress>("/v2/sandbox/usercollection/daily_stress", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponseEnhancedTagModel</returns>
        public MultiDocumentResponseEnhancedTagModel SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseEnhancedTagModel> localVarResponse = SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponseEnhancedTagModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseEnhancedTagModel> SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponseEnhancedTagModel>("/v2/sandbox/usercollection/enhanced_tag", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponseEnhancedTagModel</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponseEnhancedTagModel> SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseEnhancedTagModel> localVarResponse = await SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Enhanced Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponseEnhancedTagModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseEnhancedTagModel>> SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponseEnhancedTagModel>("/v2/sandbox/usercollection/enhanced_tag", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Heartrate Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet</returns>
        public ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> localVarResponse = SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfo(startDatetime, endDatetime, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Heartrate Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet</returns>
        public OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfo(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default)
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

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>("/v2/sandbox/usercollection/heartrate", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Heartrate Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet</returns>
        public async System.Threading.Tasks.Task<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> localVarResponse = await SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfoAsync(startDatetime, endDatetime, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Heartrate Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>> SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetWithHttpInfoAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>("/v2/sandbox/usercollection/heartrate", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicRestModePeriod</returns>
        public MultiDocumentResponsePublicRestModePeriod SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRestModePeriod> localVarResponse = SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicRestModePeriod</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRestModePeriod> SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicRestModePeriod>("/v2/sandbox/usercollection/rest_mode_period", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicRestModePeriod</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicRestModePeriod> SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRestModePeriod> localVarResponse = await SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Rest Mode Period Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicRestModePeriod)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRestModePeriod>> SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicRestModePeriod>("/v2/sandbox/usercollection/rest_mode_period", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet</returns>
        public ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> localVarResponse = SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfo(startDatetime, endDatetime, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet</returns>
        public OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfo(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default)
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

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>("/v2/sandbox/usercollection/ring_battery_level", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet</returns>
        public async System.Threading.Tasks.Task<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> localVarResponse = await SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfoAsync(startDatetime, endDatetime, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Battery Level Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDatetime"> (optional)</param>
        /// <param name="endDatetime"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>> SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetWithHttpInfoAsync(DateTime? startDatetime = default, DateTime? endDatetime = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>("/v2/sandbox/usercollection/ring_battery_level", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicRingConfiguration</returns>
        public MultiDocumentResponsePublicRingConfiguration SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet(string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRingConfiguration> localVarResponse = SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfo(nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicRingConfiguration</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRingConfiguration> SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfo(string? nextToken = default)
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

            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicRingConfiguration>("/v2/sandbox/usercollection/ring_configuration", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicRingConfiguration</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicRingConfiguration> SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetAsync(string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRingConfiguration> localVarResponse = await SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfoAsync(nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Ring Configuration Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicRingConfiguration)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicRingConfiguration>> SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetWithHttpInfoAsync(string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicRingConfiguration>("/v2/sandbox/usercollection/ring_configuration", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Session Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicSession</returns>
        public MultiDocumentResponsePublicSession SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSession> localVarResponse = SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Session Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicSession</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSession> SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicSession>("/v2/sandbox/usercollection/session", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Session Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicSession</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicSession> SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSession> localVarResponse = await SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Session Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicSession)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSession>> SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicSession>("/v2/sandbox/usercollection/session", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicModifiedSleepModel</returns>
        public MultiDocumentResponsePublicModifiedSleepModel SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> localVarResponse = SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicModifiedSleepModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicModifiedSleepModel>("/v2/sandbox/usercollection/sleep", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicModifiedSleepModel</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicModifiedSleepModel> SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicModifiedSleepModel> localVarResponse = await SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicModifiedSleepModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicModifiedSleepModel>> SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicModifiedSleepModel>("/v2/sandbox/usercollection/sleep", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicSleepTime</returns>
        public MultiDocumentResponsePublicSleepTime SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSleepTime> localVarResponse = SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicSleepTime</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSleepTime> SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicSleepTime>("/v2/sandbox/usercollection/sleep_time", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicSleepTime</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicSleepTime> SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSleepTime> localVarResponse = await SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Sleep Time Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicSleepTime)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicSleepTime>> SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicSleepTime>("/v2/sandbox/usercollection/sleep_time", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponseTagModel</returns>
        public MultiDocumentResponseTagModel SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseTagModel> localVarResponse = SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponseTagModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseTagModel> SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponseTagModel>("/v2/sandbox/usercollection/tag", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponseTagModel</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponseTagModel> SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseTagModel> localVarResponse = await SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Tag Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponseTagModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponseTagModel>> SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponseTagModel>("/v2/sandbox/usercollection/tag", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicVO2Max</returns>
        public MultiDocumentResponsePublicVO2Max SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicVO2Max> localVarResponse = SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicVO2Max</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicVO2Max> SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicVO2Max>("/v2/sandbox/usercollection/vO2_max", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicVO2Max</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicVO2Max> SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicVO2Max> localVarResponse = await SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Vo2 Max Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicVO2Max)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicVO2Max>> SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicVO2Max>("/v2/sandbox/usercollection/vO2_max", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Workout Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>MultiDocumentResponsePublicWorkout</returns>
        public MultiDocumentResponsePublicWorkout SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicWorkout> localVarResponse = SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfo(startDate, endDate, nextToken);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Workout Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <returns>ApiResponse of MultiDocumentResponsePublicWorkout</returns>
        public OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicWorkout> SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfo(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<MultiDocumentResponsePublicWorkout>("/v2/sandbox/usercollection/workout", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Multiple Workout Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of MultiDocumentResponsePublicWorkout</returns>
        public async System.Threading.Tasks.Task<MultiDocumentResponsePublicWorkout> SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicWorkout> localVarResponse = await SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfoAsync(startDate, endDate, nextToken, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Multiple Workout Documents 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="startDate"> (optional)</param>
        /// <param name="endDate"> (optional)</param>
        /// <param name="nextToken"> (optional)</param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (MultiDocumentResponsePublicWorkout)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<MultiDocumentResponsePublicWorkout>> SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetWithHttpInfoAsync(DateOnly? startDate = default, DateOnly? endDate = default, string? nextToken = default, System.Threading.CancellationToken cancellationToken = default)
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

            if (startDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "start_date", startDate));
            }
            if (endDate != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "end_date", endDate));
            }
            if (nextToken != null)
            {
                localVarRequestOptions.QueryParameters.Add(OuraToolkit.Api.Client.ClientUtils.ParameterToMultiMap("", "next_token", nextToken));
            }

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<MultiDocumentResponsePublicWorkout>("/v2/sandbox/usercollection/workout", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Activity Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyActivity</returns>
        public PublicDailyActivity SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyActivity> localVarResponse = SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Activity Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyActivity</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicDailyActivity> SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicDailyActivity>("/v2/sandbox/usercollection/daily_activity/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Activity Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyActivity</returns>
        public async System.Threading.Tasks.Task<PublicDailyActivity> SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyActivity> localVarResponse = await SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Activity Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyActivity)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicDailyActivity>> SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicDailyActivity>("/v2/sandbox/usercollection/daily_activity/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyCardiovascularAge</returns>
        public PublicDailyCardiovascularAge SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyCardiovascularAge> localVarResponse = SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyCardiovascularAge</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicDailyCardiovascularAge> SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicDailyCardiovascularAge>("/v2/sandbox/usercollection/daily_cardiovascular_age/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyCardiovascularAge</returns>
        public async System.Threading.Tasks.Task<PublicDailyCardiovascularAge> SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyCardiovascularAge> localVarResponse = await SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Cardiovascular Age Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyCardiovascularAge)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicDailyCardiovascularAge>> SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicDailyCardiovascularAge>("/v2/sandbox/usercollection/daily_cardiovascular_age/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Readiness Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyReadiness</returns>
        public PublicDailyReadiness SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyReadiness> localVarResponse = SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Readiness Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyReadiness</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicDailyReadiness> SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicDailyReadiness>("/v2/sandbox/usercollection/daily_readiness/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Readiness Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyReadiness</returns>
        public async System.Threading.Tasks.Task<PublicDailyReadiness> SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyReadiness> localVarResponse = await SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Readiness Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyReadiness)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicDailyReadiness>> SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicDailyReadiness>("/v2/sandbox/usercollection/daily_readiness/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Resilience Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>DailyResilienceModel</returns>
        public DailyResilienceModel SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<DailyResilienceModel> localVarResponse = SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Resilience Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of DailyResilienceModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<DailyResilienceModel> SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<DailyResilienceModel>("/v2/sandbox/usercollection/daily_resilience/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Resilience Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of DailyResilienceModel</returns>
        public async System.Threading.Tasks.Task<DailyResilienceModel> SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<DailyResilienceModel> localVarResponse = await SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Resilience Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (DailyResilienceModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<DailyResilienceModel>> SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<DailyResilienceModel>("/v2/sandbox/usercollection/daily_resilience/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailySleep</returns>
        public PublicDailySleep SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailySleep> localVarResponse = SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailySleep</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicDailySleep> SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicDailySleep>("/v2/sandbox/usercollection/daily_sleep/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailySleep</returns>
        public async System.Threading.Tasks.Task<PublicDailySleep> SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailySleep> localVarResponse = await SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailySleep)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicDailySleep>> SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicDailySleep>("/v2/sandbox/usercollection/daily_sleep/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Spo2 Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailySpO2</returns>
        public PublicDailySpO2 SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailySpO2> localVarResponse = SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Spo2 Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailySpO2</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicDailySpO2> SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicDailySpO2>("/v2/sandbox/usercollection/daily_spo2/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Spo2 Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailySpO2</returns>
        public async System.Threading.Tasks.Task<PublicDailySpO2> SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailySpO2> localVarResponse = await SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Spo2 Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailySpO2)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicDailySpO2>> SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicDailySpO2>("/v2/sandbox/usercollection/daily_spo2/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Stress Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicDailyStress</returns>
        public PublicDailyStress SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyStress> localVarResponse = SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Stress Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicDailyStress</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicDailyStress> SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicDailyStress>("/v2/sandbox/usercollection/daily_stress/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Daily Stress Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicDailyStress</returns>
        public async System.Threading.Tasks.Task<PublicDailyStress> SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicDailyStress> localVarResponse = await SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Daily Stress Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicDailyStress)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicDailyStress>> SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicDailyStress>("/v2/sandbox/usercollection/daily_stress/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Enhanced Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>EnhancedTagModel</returns>
        public EnhancedTagModel SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<EnhancedTagModel> localVarResponse = SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Enhanced Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of EnhancedTagModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<EnhancedTagModel> SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<EnhancedTagModel>("/v2/sandbox/usercollection/enhanced_tag/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Enhanced Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of EnhancedTagModel</returns>
        public async System.Threading.Tasks.Task<EnhancedTagModel> SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<EnhancedTagModel> localVarResponse = await SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Enhanced Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (EnhancedTagModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<EnhancedTagModel>> SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<EnhancedTagModel>("/v2/sandbox/usercollection/enhanced_tag/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Rest Mode Period Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicRestModePeriod</returns>
        public PublicRestModePeriod SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicRestModePeriod> localVarResponse = SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Rest Mode Period Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicRestModePeriod</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicRestModePeriod> SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicRestModePeriod>("/v2/sandbox/usercollection/rest_mode_period/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Rest Mode Period Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicRestModePeriod</returns>
        public async System.Threading.Tasks.Task<PublicRestModePeriod> SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicRestModePeriod> localVarResponse = await SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Rest Mode Period Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicRestModePeriod)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicRestModePeriod>> SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicRestModePeriod>("/v2/sandbox/usercollection/rest_mode_period/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Ring Configuration Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicRingConfiguration</returns>
        public PublicRingConfiguration SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicRingConfiguration> localVarResponse = SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Ring Configuration Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicRingConfiguration</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicRingConfiguration> SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicRingConfiguration>("/v2/sandbox/usercollection/ring_configuration/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Ring Configuration Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicRingConfiguration</returns>
        public async System.Threading.Tasks.Task<PublicRingConfiguration> SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicRingConfiguration> localVarResponse = await SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Ring Configuration Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicRingConfiguration)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicRingConfiguration>> SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicRingConfiguration>("/v2/sandbox/usercollection/ring_configuration/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Session Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicSession</returns>
        public PublicSession SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicSession> localVarResponse = SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Session Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicSession</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicSession> SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicSession>("/v2/sandbox/usercollection/session/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Session Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicSession</returns>
        public async System.Threading.Tasks.Task<PublicSession> SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicSession> localVarResponse = await SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Session Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicSession)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicSession>> SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicSession>("/v2/sandbox/usercollection/session/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicModifiedSleepModel</returns>
        public PublicModifiedSleepModel SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicModifiedSleepModel> localVarResponse = SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicModifiedSleepModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicModifiedSleepModel> SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicModifiedSleepModel>("/v2/sandbox/usercollection/sleep/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicModifiedSleepModel</returns>
        public async System.Threading.Tasks.Task<PublicModifiedSleepModel> SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicModifiedSleepModel> localVarResponse = await SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Sleep Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicModifiedSleepModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicModifiedSleepModel>> SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicModifiedSleepModel>("/v2/sandbox/usercollection/sleep/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Sleep Time Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicSleepTime</returns>
        public PublicSleepTime SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicSleepTime> localVarResponse = SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Sleep Time Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicSleepTime</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicSleepTime> SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicSleepTime>("/v2/sandbox/usercollection/sleep_time/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Sleep Time Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicSleepTime</returns>
        public async System.Threading.Tasks.Task<PublicSleepTime> SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicSleepTime> localVarResponse = await SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Sleep Time Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicSleepTime)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicSleepTime>> SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicSleepTime>("/v2/sandbox/usercollection/sleep_time/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>TagModel</returns>
        public TagModel SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<TagModel> localVarResponse = SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of TagModel</returns>
        public OuraToolkit.Api.Client.ApiResponse<TagModel> SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<TagModel>("/v2/sandbox/usercollection/tag/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of TagModel</returns>
        public async System.Threading.Tasks.Task<TagModel> SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<TagModel> localVarResponse = await SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Tag Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (TagModel)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<TagModel>> SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<TagModel>("/v2/sandbox/usercollection/tag/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Vo2 Max Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicVO2Max</returns>
        public PublicVO2Max SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicVO2Max> localVarResponse = SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Vo2 Max Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicVO2Max</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicVO2Max> SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicVO2Max>("/v2/sandbox/usercollection/vO2_max/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Vo2 Max Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicVO2Max</returns>
        public async System.Threading.Tasks.Task<PublicVO2Max> SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicVO2Max> localVarResponse = await SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Vo2 Max Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicVO2Max)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicVO2Max>> SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicVO2Max>("/v2/sandbox/usercollection/vO2_max/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Workout Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>PublicWorkout</returns>
        public PublicWorkout SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet(string documentId)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicWorkout> localVarResponse = SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfo(documentId);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Workout Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <returns>ApiResponse of PublicWorkout</returns>
        public OuraToolkit.Api.Client.ApiResponse<PublicWorkout> SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfo(string documentId)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet");

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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request
            var localVarResponse = this.Client.Get<PublicWorkout>("/v2/sandbox/usercollection/workout/{document_id}", localVarRequestOptions, this.Configuration);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

        /// <summary>
        /// Sandbox - Single Workout Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of PublicWorkout</returns>
        public async System.Threading.Tasks.Task<PublicWorkout> SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            OuraToolkit.Api.Client.ApiResponse<PublicWorkout> localVarResponse = await SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfoAsync(documentId, cancellationToken).ConfigureAwait(false);
            return localVarResponse.Data;
        }

        /// <summary>
        /// Sandbox - Single Workout Document 
        /// </summary>
        /// <exception cref="OuraToolkit.Api.Client.ApiException">Thrown when fails to make API call</exception>
        /// <param name="documentId"></param>
        /// <param name="cancellationToken">Cancellation Token to cancel the request.</param>
        /// <returns>Task of ApiResponse (PublicWorkout)</returns>
        public async System.Threading.Tasks.Task<OuraToolkit.Api.Client.ApiResponse<PublicWorkout>> SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetWithHttpInfoAsync(string documentId, System.Threading.CancellationToken cancellationToken = default)
        {
            // verify the required parameter 'documentId' is set
            if (documentId == null)
                throw new OuraToolkit.Api.Client.ApiException(400, "Missing required parameter 'documentId' when calling SandboxRoutesApi->SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet");


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

            localVarRequestOptions.PathParameters.Add("document_id", OuraToolkit.Api.Client.ClientUtils.ParameterToString(documentId)); // path parameter

            // authentication (BearerAuth) required
            // bearer authentication required
            if (!string.IsNullOrEmpty(this.Configuration.AccessToken) && !localVarRequestOptions.HeaderParameters.ContainsKey("Authorization"))
            {
                localVarRequestOptions.HeaderParameters.Add("Authorization", "Bearer " + this.Configuration.AccessToken);
            }

            // make the HTTP request

            var localVarResponse = await this.AsynchronousClient.GetAsync<PublicWorkout>("/v2/sandbox/usercollection/workout/{document_id}", localVarRequestOptions, this.Configuration, cancellationToken).ConfigureAwait(false);

            if (this.ExceptionFactory != null)
            {
                Exception _exception = this.ExceptionFactory("SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet", localVarResponse);
                if (_exception != null) throw _exception;
            }

            return localVarResponse;
        }

    }
}
