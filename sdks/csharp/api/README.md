# OuraToolkit.Api - the C# library for the Oura API Documentation

# Overview 
The Oura API allows Oura users and partner applications to improve their user experience with Oura data.
This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset.
# Getting Started 
## What is an API?
An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically.
## Quick Start Guide
1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2
2. **Make Your First API Call**:
   ```
   curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\
   -H \"Authorization: Bearer YOUR_TOKEN_HERE\"
   ```
3. **Explore Data Types**:
   - Browse the available endpoints in this documentation to discover what data you can access
   - Each endpoint includes example requests and responses
4. **Set Up Webhooks (Strongly Recommended)**:
   - Webhooks are the preferred way to consume Oura data
   - We have not had customers hit rate limits with webhooks properly implemented
   - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates
   - Webhook notifications come approximately 30 seconds after data syncs from the mobile app
   - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes
## Common Questions
- **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background
# Data Access
In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.
 API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.
 Additionally, Oura users **must provide consent** to share each data type an API Application has access to.
All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication).
Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types.
# Authentication
The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication).
Access tokens must be included in the request header as follows:
```http
GET /v2/usercollection/personal_info HTTP/1.1
Host: api.ouraring.com
Authorization: Bearer <token>
```
Please note that personal access tokens were deprecated in December 2025 and are no longer available for use.
# Oura HTTP Response Codes
| Response Code                        | Description |
| - -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- - | - |
| 200 OK                               | Successful Response         |
| 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. |
| 401 Unauthorized                     | Invalid or expired authentication token. |
| 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. |
| 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |

## Rate Limits
The API enforces rate limits at two layers to ensure fair access across all applications:
- a per-access-token limit, which throttles single-token floods, and
- a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.

A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.

If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.

[Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.

## Rate Limit Response Headers
When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff:
- **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic.
- **`X-RateLimit-Limit`** — the request ceiling for the current window.
- **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to.
- **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored.
- **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support.


This C# SDK is automatically generated by the [OpenAPI Generator](https://openapi-generator.tech) project:

- API version: 2.0
- SDK version: 0.3.0
- Generator version: 7.14.0
- Build package: org.openapitools.codegen.languages.CSharpClientCodegen

<a id="frameworks-supported"></a>
## Frameworks supported
- .NET Core >=1.0
- .NET Framework >=4.6
- Mono/Xamarin >=vNext

<a id="dependencies"></a>
## Dependencies

- [Json.NET](https://www.nuget.org/packages/Newtonsoft.Json/) - 13.0.2 or later
- [JsonSubTypes](https://www.nuget.org/packages/JsonSubTypes/) - 1.8.0 or later
- [System.ComponentModel.Annotations](https://www.nuget.org/packages/System.ComponentModel.Annotations) - 5.0.0 or later

The DLLs included in the package may not be the latest version. We recommend using [NuGet](https://docs.nuget.org/consume/installing-nuget) to obtain the latest version of the packages:
```
Install-Package Newtonsoft.Json
Install-Package JsonSubTypes
Install-Package System.ComponentModel.Annotations
```
<a id="installation"></a>
## Installation
Generate the DLL using your preferred tool (e.g. `dotnet build`)

Then include the DLL (under the `bin` folder) in the C# project, and use the namespaces:
```csharp
using OuraToolkit.Api.Api;
using OuraToolkit.Api.Client;
using OuraToolkit.Api.Model;
```
<a id="usage"></a>
## Usage

To use the API client with a HTTP proxy, setup a `System.Net.WebProxy`
```csharp
Configuration c = new Configuration();
System.Net.WebProxy webProxy = new System.Net.WebProxy("http://myProxyUrl:80/");
webProxy.Credentials = System.Net.CredentialCache.DefaultCredentials;
c.Proxy = webProxy;
```

### Connections
Each ApiClass (properly the ApiClient inside it) will create an instance of HttpClient. It will use that for the entire lifecycle and dispose it when called the Dispose method.

To better manager the connections it's a common practice to reuse the HttpClient and HttpClientHandler (see [here](https://docs.microsoft.com/en-us/dotnet/architecture/microservices/implement-resilient-applications/use-httpclientfactory-to-implement-resilient-http-requests#issues-with-the-original-httpclient-class-available-in-net) for details). To use your own HttpClient instance just pass it to the ApiClass constructor.

```csharp
HttpClientHandler yourHandler = new HttpClientHandler();
HttpClient yourHttpClient = new HttpClient(yourHandler);
var api = new YourApiClass(yourHttpClient, yourHandler);
```

If you want to use an HttpClient and don't have access to the handler, for example in a DI context in Asp.net Core when using IHttpClientFactory.

```csharp
HttpClient yourHttpClient = new HttpClient();
var api = new YourApiClass(yourHttpClient);
```
You'll loose some configuration settings, the features affected are: Setting and Retrieving Cookies, Client Certificates, Proxy settings. You need to either manually handle those in your setup of the HttpClient or they won't be available.

Here an example of DI setup in a sample web project:

```csharp
services.AddHttpClient<YourApiClass>(httpClient =>
   new PetApi(httpClient));
```


<a id="getting-started"></a>
## Getting Started

```csharp
using System.Collections.Generic;
using System.Diagnostics;
using System.Net.Http;
using OuraToolkit.Api.Api;
using OuraToolkit.Api.Client;
using OuraToolkit.Api.Model;

namespace Example
{
    public class Example
    {
        public static void Main()
        {

            Configuration config = new Configuration();
            config.BasePath = "https://api.ouraring.com";
            // Configure Bearer token for authorization: BearerAuth
            config.AccessToken = "YOUR_BEARER_TOKEN";

            // create instances of HttpClient, HttpClientHandler to be reused later with different Api classes
            HttpClient httpClient = new HttpClient();
            HttpClientHandler httpClientHandler = new HttpClientHandler();
            var apiInstance = new DailyActivityRoutesApi(httpClient, config, httpClientHandler);
            var startDate = DateTime.Parse("2013-10-20");  // DateTime? |  (optional) 
            var endDate = DateTime.Parse("2013-10-20");  // DateTime? |  (optional) 
            var nextToken = "nextToken_example";  // string? |  (optional) 
            var fields = "fields_example";  // string? | Comma-separated list of fields to include in the response, in addition to the always returned fields. Defaults to all fields if not provided. (optional) 

            try
            {
                // Multiple Daily Activity Documents
                MultiDocumentResponsePublicDailyActivity result = apiInstance.MultipleDailyActivityDocumentsV2UsercollectionDailyActivityGet(startDate, endDate, nextToken, fields);
                Debug.WriteLine(result);
            }
            catch (ApiException e)
            {
                Debug.Print("Exception when calling DailyActivityRoutesApi.MultipleDailyActivityDocumentsV2UsercollectionDailyActivityGet: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }

        }
    }
}
```

<a id="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *https://api.ouraring.com*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DailyActivityRoutesApi* | [**MultipleDailyActivityDocumentsV2UsercollectionDailyActivityGet**](docs/DailyActivityRoutesApi.md#multipledailyactivitydocumentsv2usercollectiondailyactivityget) | **GET** /v2/usercollection/daily_activity | Multiple Daily Activity Documents
*DailyActivityRoutesApi* | [**SingleDailyActivityDocumentV2UsercollectionDailyActivityDocumentIdGet**](docs/DailyActivityRoutesApi.md#singledailyactivitydocumentv2usercollectiondailyactivitydocumentidget) | **GET** /v2/usercollection/daily_activity/{document_id} | Single Daily Activity Document
*DailyCardiovascularAgeRoutesApi* | [**MultipleDailyCardiovascularAgeDocumentsV2UsercollectionDailyCardiovascularAgeGet**](docs/DailyCardiovascularAgeRoutesApi.md#multipledailycardiovascularagedocumentsv2usercollectiondailycardiovascularageget) | **GET** /v2/usercollection/daily_cardiovascular_age | Multiple Daily Cardiovascular Age Documents
*DailyCardiovascularAgeRoutesApi* | [**SingleDailyCardiovascularAgeDocumentV2UsercollectionDailyCardiovascularAgeDocumentIdGet**](docs/DailyCardiovascularAgeRoutesApi.md#singledailycardiovascularagedocumentv2usercollectiondailycardiovascularagedocumentidget) | **GET** /v2/usercollection/daily_cardiovascular_age/{document_id} | Single Daily Cardiovascular Age Document
*DailyReadinessRoutesApi* | [**MultipleDailyReadinessDocumentsV2UsercollectionDailyReadinessGet**](docs/DailyReadinessRoutesApi.md#multipledailyreadinessdocumentsv2usercollectiondailyreadinessget) | **GET** /v2/usercollection/daily_readiness | Multiple Daily Readiness Documents
*DailyReadinessRoutesApi* | [**SingleDailyReadinessDocumentV2UsercollectionDailyReadinessDocumentIdGet**](docs/DailyReadinessRoutesApi.md#singledailyreadinessdocumentv2usercollectiondailyreadinessdocumentidget) | **GET** /v2/usercollection/daily_readiness/{document_id} | Single Daily Readiness Document
*DailyResilienceRoutesApi* | [**MultipleDailyResilienceDocumentsV2UsercollectionDailyResilienceGet**](docs/DailyResilienceRoutesApi.md#multipledailyresiliencedocumentsv2usercollectiondailyresilienceget) | **GET** /v2/usercollection/daily_resilience | Multiple Daily Resilience Documents
*DailyResilienceRoutesApi* | [**SingleDailyResilienceDocumentV2UsercollectionDailyResilienceDocumentIdGet**](docs/DailyResilienceRoutesApi.md#singledailyresiliencedocumentv2usercollectiondailyresiliencedocumentidget) | **GET** /v2/usercollection/daily_resilience/{document_id} | Single Daily Resilience Document
*DailySleepRoutesApi* | [**MultipleDailySleepDocumentsV2UsercollectionDailySleepGet**](docs/DailySleepRoutesApi.md#multipledailysleepdocumentsv2usercollectiondailysleepget) | **GET** /v2/usercollection/daily_sleep | Multiple Daily Sleep Documents
*DailySleepRoutesApi* | [**SingleDailySleepDocumentV2UsercollectionDailySleepDocumentIdGet**](docs/DailySleepRoutesApi.md#singledailysleepdocumentv2usercollectiondailysleepdocumentidget) | **GET** /v2/usercollection/daily_sleep/{document_id} | Single Daily Sleep Document
*DailySpo2RoutesApi* | [**MultipleDailySpo2DocumentsV2UsercollectionDailySpo2Get**](docs/DailySpo2RoutesApi.md#multipledailyspo2documentsv2usercollectiondailyspo2get) | **GET** /v2/usercollection/daily_spo2 | Multiple Daily Spo2 Documents
*DailySpo2RoutesApi* | [**SingleDailySpo2DocumentV2UsercollectionDailySpo2DocumentIdGet**](docs/DailySpo2RoutesApi.md#singledailyspo2documentv2usercollectiondailyspo2documentidget) | **GET** /v2/usercollection/daily_spo2/{document_id} | Single Daily Spo2 Document
*DailyStressRoutesApi* | [**MultipleDailyStressDocumentsV2UsercollectionDailyStressGet**](docs/DailyStressRoutesApi.md#multipledailystressdocumentsv2usercollectiondailystressget) | **GET** /v2/usercollection/daily_stress | Multiple Daily Stress Documents
*DailyStressRoutesApi* | [**SingleDailyStressDocumentV2UsercollectionDailyStressDocumentIdGet**](docs/DailyStressRoutesApi.md#singledailystressdocumentv2usercollectiondailystressdocumentidget) | **GET** /v2/usercollection/daily_stress/{document_id} | Single Daily Stress Document
*EnhancedTagRoutesApi* | [**MultipleEnhancedTagDocumentsV2UsercollectionEnhancedTagGet**](docs/EnhancedTagRoutesApi.md#multipleenhancedtagdocumentsv2usercollectionenhancedtagget) | **GET** /v2/usercollection/enhanced_tag | Multiple Enhanced Tag Documents
*EnhancedTagRoutesApi* | [**SingleEnhancedTagDocumentV2UsercollectionEnhancedTagDocumentIdGet**](docs/EnhancedTagRoutesApi.md#singleenhancedtagdocumentv2usercollectionenhancedtagdocumentidget) | **GET** /v2/usercollection/enhanced_tag/{document_id} | Single Enhanced Tag Document
*HeartRateRoutesApi* | [**MultipleHeartrateDocumentsV2UsercollectionHeartrateGet**](docs/HeartRateRoutesApi.md#multipleheartratedocumentsv2usercollectionheartrateget) | **GET** /v2/usercollection/heartrate | Multiple Heartrate Documents
*PersonalInfoRoutesApi* | [**SinglePersonalInfoDocumentV2UsercollectionPersonalInfoGet**](docs/PersonalInfoRoutesApi.md#singlepersonalinfodocumentv2usercollectionpersonalinfoget) | **GET** /v2/usercollection/personal_info | Single Personal Info Document
*RestModePeriodRoutesApi* | [**MultipleRestModePeriodDocumentsV2UsercollectionRestModePeriodGet**](docs/RestModePeriodRoutesApi.md#multiplerestmodeperioddocumentsv2usercollectionrestmodeperiodget) | **GET** /v2/usercollection/rest_mode_period | Multiple Rest Mode Period Documents
*RestModePeriodRoutesApi* | [**SingleRestModePeriodDocumentV2UsercollectionRestModePeriodDocumentIdGet**](docs/RestModePeriodRoutesApi.md#singlerestmodeperioddocumentv2usercollectionrestmodeperioddocumentidget) | **GET** /v2/usercollection/rest_mode_period/{document_id} | Single Rest Mode Period Document
*RingBatteryLevelRoutesApi* | [**MultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet**](docs/RingBatteryLevelRoutesApi.md#multipleringbatteryleveldocumentsv2usercollectionringbatterylevelget) | **GET** /v2/usercollection/ring_battery_level | Multiple Ring Battery Level Documents
*RingConfigurationRoutesApi* | [**MultipleRingConfigurationDocumentsV2UsercollectionRingConfigurationGet**](docs/RingConfigurationRoutesApi.md#multipleringconfigurationdocumentsv2usercollectionringconfigurationget) | **GET** /v2/usercollection/ring_configuration | Multiple Ring Configuration Documents
*RingConfigurationRoutesApi* | [**SingleRingConfigurationDocumentV2UsercollectionRingConfigurationDocumentIdGet**](docs/RingConfigurationRoutesApi.md#singleringconfigurationdocumentv2usercollectionringconfigurationdocumentidget) | **GET** /v2/usercollection/ring_configuration/{document_id} | Single Ring Configuration Document
*SandboxRoutesApi* | [**SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet**](docs/SandboxRoutesApi.md#sandboxmultipledailyactivitydocumentsv2sandboxusercollectiondailyactivityget) | **GET** /v2/sandbox/usercollection/daily_activity | Sandbox - Multiple Daily Activity Documents
*SandboxRoutesApi* | [**SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet**](docs/SandboxRoutesApi.md#sandboxmultipledailycardiovascularagedocumentsv2sandboxusercollectiondailycardiovascularageget) | **GET** /v2/sandbox/usercollection/daily_cardiovascular_age | Sandbox - Multiple Daily Cardiovascular Age Documents
*SandboxRoutesApi* | [**SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet**](docs/SandboxRoutesApi.md#sandboxmultipledailyreadinessdocumentsv2sandboxusercollectiondailyreadinessget) | **GET** /v2/sandbox/usercollection/daily_readiness | Sandbox - Multiple Daily Readiness Documents
*SandboxRoutesApi* | [**SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet**](docs/SandboxRoutesApi.md#sandboxmultipledailyresiliencedocumentsv2sandboxusercollectiondailyresilienceget) | **GET** /v2/sandbox/usercollection/daily_resilience | Sandbox - Multiple Daily Resilience Documents
*SandboxRoutesApi* | [**SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet**](docs/SandboxRoutesApi.md#sandboxmultipledailysleepdocumentsv2sandboxusercollectiondailysleepget) | **GET** /v2/sandbox/usercollection/daily_sleep | Sandbox - Multiple Daily Sleep Documents
*SandboxRoutesApi* | [**SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get**](docs/SandboxRoutesApi.md#sandboxmultipledailyspo2documentsv2sandboxusercollectiondailyspo2get) | **GET** /v2/sandbox/usercollection/daily_spo2 | Sandbox - Multiple Daily Spo2 Documents
*SandboxRoutesApi* | [**SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet**](docs/SandboxRoutesApi.md#sandboxmultipledailystressdocumentsv2sandboxusercollectiondailystressget) | **GET** /v2/sandbox/usercollection/daily_stress | Sandbox - Multiple Daily Stress Documents
*SandboxRoutesApi* | [**SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet**](docs/SandboxRoutesApi.md#sandboxmultipleenhancedtagdocumentsv2sandboxusercollectionenhancedtagget) | **GET** /v2/sandbox/usercollection/enhanced_tag | Sandbox - Multiple Enhanced Tag Documents
*SandboxRoutesApi* | [**SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet**](docs/SandboxRoutesApi.md#sandboxmultipleheartratedocumentsv2sandboxusercollectionheartrateget) | **GET** /v2/sandbox/usercollection/heartrate | Sandbox - Multiple Heartrate Documents
*SandboxRoutesApi* | [**SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet**](docs/SandboxRoutesApi.md#sandboxmultiplerestmodeperioddocumentsv2sandboxusercollectionrestmodeperiodget) | **GET** /v2/sandbox/usercollection/rest_mode_period | Sandbox - Multiple Rest Mode Period Documents
*SandboxRoutesApi* | [**SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet**](docs/SandboxRoutesApi.md#sandboxmultipleringbatteryleveldocumentsv2sandboxusercollectionringbatterylevelget) | **GET** /v2/sandbox/usercollection/ring_battery_level | Sandbox - Multiple Ring Battery Level Documents
*SandboxRoutesApi* | [**SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet**](docs/SandboxRoutesApi.md#sandboxmultipleringconfigurationdocumentsv2sandboxusercollectionringconfigurationget) | **GET** /v2/sandbox/usercollection/ring_configuration | Sandbox - Multiple Ring Configuration Documents
*SandboxRoutesApi* | [**SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet**](docs/SandboxRoutesApi.md#sandboxmultiplesessiondocumentsv2sandboxusercollectionsessionget) | **GET** /v2/sandbox/usercollection/session | Sandbox - Multiple Session Documents
*SandboxRoutesApi* | [**SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet**](docs/SandboxRoutesApi.md#sandboxmultiplesleepdocumentsv2sandboxusercollectionsleepget) | **GET** /v2/sandbox/usercollection/sleep | Sandbox - Multiple Sleep Documents
*SandboxRoutesApi* | [**SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet**](docs/SandboxRoutesApi.md#sandboxmultiplesleeptimedocumentsv2sandboxusercollectionsleeptimeget) | **GET** /v2/sandbox/usercollection/sleep_time | Sandbox - Multiple Sleep Time Documents
*SandboxRoutesApi* | [**SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet**](docs/SandboxRoutesApi.md#sandboxmultipletagdocumentsv2sandboxusercollectiontagget) | **GET** /v2/sandbox/usercollection/tag | Sandbox - Multiple Tag Documents
*SandboxRoutesApi* | [**SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet**](docs/SandboxRoutesApi.md#sandboxmultiplevo2maxdocumentsv2sandboxusercollectionvo2maxget) | **GET** /v2/sandbox/usercollection/vO2_max | Sandbox - Multiple Vo2 Max Documents
*SandboxRoutesApi* | [**SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet**](docs/SandboxRoutesApi.md#sandboxmultipleworkoutdocumentsv2sandboxusercollectionworkoutget) | **GET** /v2/sandbox/usercollection/workout | Sandbox - Multiple Workout Documents
*SandboxRoutesApi* | [**SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingledailyactivitydocumentv2sandboxusercollectiondailyactivitydocumentidget) | **GET** /v2/sandbox/usercollection/daily_activity/{document_id} | Sandbox - Single Daily Activity Document
*SandboxRoutesApi* | [**SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingledailycardiovascularagedocumentv2sandboxusercollectiondailycardiovascularagedocumentidget) | **GET** /v2/sandbox/usercollection/daily_cardiovascular_age/{document_id} | Sandbox - Single Daily Cardiovascular Age Document
*SandboxRoutesApi* | [**SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingledailyreadinessdocumentv2sandboxusercollectiondailyreadinessdocumentidget) | **GET** /v2/sandbox/usercollection/daily_readiness/{document_id} | Sandbox - Single Daily Readiness Document
*SandboxRoutesApi* | [**SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingledailyresiliencedocumentv2sandboxusercollectiondailyresiliencedocumentidget) | **GET** /v2/sandbox/usercollection/daily_resilience/{document_id} | Sandbox - Single Daily Resilience Document
*SandboxRoutesApi* | [**SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingledailysleepdocumentv2sandboxusercollectiondailysleepdocumentidget) | **GET** /v2/sandbox/usercollection/daily_sleep/{document_id} | Sandbox - Single Daily Sleep Document
*SandboxRoutesApi* | [**SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingledailyspo2documentv2sandboxusercollectiondailyspo2documentidget) | **GET** /v2/sandbox/usercollection/daily_spo2/{document_id} | Sandbox - Single Daily Spo2 Document
*SandboxRoutesApi* | [**SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingledailystressdocumentv2sandboxusercollectiondailystressdocumentidget) | **GET** /v2/sandbox/usercollection/daily_stress/{document_id} | Sandbox - Single Daily Stress Document
*SandboxRoutesApi* | [**SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingleenhancedtagdocumentv2sandboxusercollectionenhancedtagdocumentidget) | **GET** /v2/sandbox/usercollection/enhanced_tag/{document_id} | Sandbox - Single Enhanced Tag Document
*SandboxRoutesApi* | [**SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsinglerestmodeperioddocumentv2sandboxusercollectionrestmodeperioddocumentidget) | **GET** /v2/sandbox/usercollection/rest_mode_period/{document_id} | Sandbox - Single Rest Mode Period Document
*SandboxRoutesApi* | [**SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingleringconfigurationdocumentv2sandboxusercollectionringconfigurationdocumentidget) | **GET** /v2/sandbox/usercollection/ring_configuration/{document_id} | Sandbox - Single Ring Configuration Document
*SandboxRoutesApi* | [**SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsinglesessiondocumentv2sandboxusercollectionsessiondocumentidget) | **GET** /v2/sandbox/usercollection/session/{document_id} | Sandbox - Single Session Document
*SandboxRoutesApi* | [**SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsinglesleepdocumentv2sandboxusercollectionsleepdocumentidget) | **GET** /v2/sandbox/usercollection/sleep/{document_id} | Sandbox - Single Sleep Document
*SandboxRoutesApi* | [**SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsinglesleeptimedocumentv2sandboxusercollectionsleeptimedocumentidget) | **GET** /v2/sandbox/usercollection/sleep_time/{document_id} | Sandbox - Single Sleep Time Document
*SandboxRoutesApi* | [**SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingletagdocumentv2sandboxusercollectiontagdocumentidget) | **GET** /v2/sandbox/usercollection/tag/{document_id} | Sandbox - Single Tag Document
*SandboxRoutesApi* | [**SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsinglevo2maxdocumentv2sandboxusercollectionvo2maxdocumentidget) | **GET** /v2/sandbox/usercollection/vO2_max/{document_id} | Sandbox - Single Vo2 Max Document
*SandboxRoutesApi* | [**SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet**](docs/SandboxRoutesApi.md#sandboxsingleworkoutdocumentv2sandboxusercollectionworkoutdocumentidget) | **GET** /v2/sandbox/usercollection/workout/{document_id} | Sandbox - Single Workout Document
*SessionRoutesApi* | [**MultipleSessionDocumentsV2UsercollectionSessionGet**](docs/SessionRoutesApi.md#multiplesessiondocumentsv2usercollectionsessionget) | **GET** /v2/usercollection/session | Multiple Session Documents
*SessionRoutesApi* | [**SingleSessionDocumentV2UsercollectionSessionDocumentIdGet**](docs/SessionRoutesApi.md#singlesessiondocumentv2usercollectionsessiondocumentidget) | **GET** /v2/usercollection/session/{document_id} | Single Session Document
*SleepRoutesApi* | [**MultipleSleepDocumentsV2UsercollectionSleepGet**](docs/SleepRoutesApi.md#multiplesleepdocumentsv2usercollectionsleepget) | **GET** /v2/usercollection/sleep | Multiple Sleep Documents
*SleepRoutesApi* | [**SingleSleepDocumentV2UsercollectionSleepDocumentIdGet**](docs/SleepRoutesApi.md#singlesleepdocumentv2usercollectionsleepdocumentidget) | **GET** /v2/usercollection/sleep/{document_id} | Single Sleep Document
*SleepTimeRoutesApi* | [**MultipleSleepTimeDocumentsV2UsercollectionSleepTimeGet**](docs/SleepTimeRoutesApi.md#multiplesleeptimedocumentsv2usercollectionsleeptimeget) | **GET** /v2/usercollection/sleep_time | Multiple Sleep Time Documents
*SleepTimeRoutesApi* | [**SingleSleepTimeDocumentV2UsercollectionSleepTimeDocumentIdGet**](docs/SleepTimeRoutesApi.md#singlesleeptimedocumentv2usercollectionsleeptimedocumentidget) | **GET** /v2/usercollection/sleep_time/{document_id} | Single Sleep Time Document
*TagRoutesApi* | [**MultipleTagDocumentsV2UsercollectionTagGet**](docs/TagRoutesApi.md#multipletagdocumentsv2usercollectiontagget) | **GET** /v2/usercollection/tag | Multiple Tag Documents
*TagRoutesApi* | [**SingleTagDocumentV2UsercollectionTagDocumentIdGet**](docs/TagRoutesApi.md#singletagdocumentv2usercollectiontagdocumentidget) | **GET** /v2/usercollection/tag/{document_id} | Single Tag Document
*VO2MaxRoutesApi* | [**MultipleVO2MaxDocumentsV2UsercollectionVO2MaxGet**](docs/VO2MaxRoutesApi.md#multiplevo2maxdocumentsv2usercollectionvo2maxget) | **GET** /v2/usercollection/vO2_max | Multiple Vo2 Max Documents
*VO2MaxRoutesApi* | [**SingleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGet**](docs/VO2MaxRoutesApi.md#singlevo2maxdocumentv2usercollectionvo2maxdocumentidget) | **GET** /v2/usercollection/vO2_max/{document_id} | Single Vo2 Max Document
*WebhookSubscriptionRoutesApi* | [**CreateWebhookSubscriptionV2WebhookSubscriptionPost**](docs/WebhookSubscriptionRoutesApi.md#createwebhooksubscriptionv2webhooksubscriptionpost) | **POST** /v2/webhook/subscription | Create Webhook Subscription
*WebhookSubscriptionRoutesApi* | [**DeleteWebhookSubscriptionV2WebhookSubscriptionIdDelete**](docs/WebhookSubscriptionRoutesApi.md#deletewebhooksubscriptionv2webhooksubscriptioniddelete) | **DELETE** /v2/webhook/subscription/{id} | Delete Webhook Subscription
*WebhookSubscriptionRoutesApi* | [**GetWebhookSubscriptionV2WebhookSubscriptionIdGet**](docs/WebhookSubscriptionRoutesApi.md#getwebhooksubscriptionv2webhooksubscriptionidget) | **GET** /v2/webhook/subscription/{id} | Get Webhook Subscription
*WebhookSubscriptionRoutesApi* | [**ListWebhookSubscriptionsV2WebhookSubscriptionGet**](docs/WebhookSubscriptionRoutesApi.md#listwebhooksubscriptionsv2webhooksubscriptionget) | **GET** /v2/webhook/subscription | List Webhook Subscriptions
*WebhookSubscriptionRoutesApi* | [**RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut**](docs/WebhookSubscriptionRoutesApi.md#renewwebhooksubscriptionv2webhooksubscriptionrenewidput) | **PUT** /v2/webhook/subscription/renew/{id} | Renew Webhook Subscription
*WebhookSubscriptionRoutesApi* | [**UpdateWebhookSubscriptionV2WebhookSubscriptionIdPut**](docs/WebhookSubscriptionRoutesApi.md#updatewebhooksubscriptionv2webhooksubscriptionidput) | **PUT** /v2/webhook/subscription/{id} | Update Webhook Subscription
*WorkoutRoutesApi* | [**MultipleWorkoutDocumentsV2UsercollectionWorkoutGet**](docs/WorkoutRoutesApi.md#multipleworkoutdocumentsv2usercollectionworkoutget) | **GET** /v2/usercollection/workout | Multiple Workout Documents
*WorkoutRoutesApi* | [**SingleWorkoutDocumentV2UsercollectionWorkoutDocumentIdGet**](docs/WorkoutRoutesApi.md#singleworkoutdocumentv2usercollectionworkoutdocumentidget) | **GET** /v2/usercollection/workout/{document_id} | Single Workout Document


<a id="documentation-for-models"></a>
## Documentation for Models

 - [Model.CreateWebhookSubscriptionRequest](docs/CreateWebhookSubscriptionRequest.md)
 - [Model.DailyResilienceModel](docs/DailyResilienceModel.md)
 - [Model.EnhancedTagModel](docs/EnhancedTagModel.md)
 - [Model.ExtApiV2DataType](docs/ExtApiV2DataType.md)
 - [Model.HTTPValidationError](docs/HTTPValidationError.md)
 - [Model.LongTermResilienceLevel](docs/LongTermResilienceLevel.md)
 - [Model.MultiDocumentResponseDailyResilienceModel](docs/MultiDocumentResponseDailyResilienceModel.md)
 - [Model.MultiDocumentResponseEnhancedTagModel](docs/MultiDocumentResponseEnhancedTagModel.md)
 - [Model.MultiDocumentResponsePublicDailyActivity](docs/MultiDocumentResponsePublicDailyActivity.md)
 - [Model.MultiDocumentResponsePublicDailyCardiovascularAge](docs/MultiDocumentResponsePublicDailyCardiovascularAge.md)
 - [Model.MultiDocumentResponsePublicDailyReadiness](docs/MultiDocumentResponsePublicDailyReadiness.md)
 - [Model.MultiDocumentResponsePublicDailySleep](docs/MultiDocumentResponsePublicDailySleep.md)
 - [Model.MultiDocumentResponsePublicDailySpO2](docs/MultiDocumentResponsePublicDailySpO2.md)
 - [Model.MultiDocumentResponsePublicDailyStress](docs/MultiDocumentResponsePublicDailyStress.md)
 - [Model.MultiDocumentResponsePublicModifiedSleepModel](docs/MultiDocumentResponsePublicModifiedSleepModel.md)
 - [Model.MultiDocumentResponsePublicRestModePeriod](docs/MultiDocumentResponsePublicRestModePeriod.md)
 - [Model.MultiDocumentResponsePublicRingConfiguration](docs/MultiDocumentResponsePublicRingConfiguration.md)
 - [Model.MultiDocumentResponsePublicSession](docs/MultiDocumentResponsePublicSession.md)
 - [Model.MultiDocumentResponsePublicSleepTime](docs/MultiDocumentResponsePublicSleepTime.md)
 - [Model.MultiDocumentResponsePublicVO2Max](docs/MultiDocumentResponsePublicVO2Max.md)
 - [Model.MultiDocumentResponsePublicWorkout](docs/MultiDocumentResponsePublicWorkout.md)
 - [Model.MultiDocumentResponseTagModel](docs/MultiDocumentResponseTagModel.md)
 - [Model.PersonalInfoResponse](docs/PersonalInfoResponse.md)
 - [Model.PublicActivityContributors](docs/PublicActivityContributors.md)
 - [Model.PublicDailyActivity](docs/PublicDailyActivity.md)
 - [Model.PublicDailyCardiovascularAge](docs/PublicDailyCardiovascularAge.md)
 - [Model.PublicDailyReadiness](docs/PublicDailyReadiness.md)
 - [Model.PublicDailySleep](docs/PublicDailySleep.md)
 - [Model.PublicDailySpO2](docs/PublicDailySpO2.md)
 - [Model.PublicDailyStress](docs/PublicDailyStress.md)
 - [Model.PublicDailyStressSummary](docs/PublicDailyStressSummary.md)
 - [Model.PublicHeartRateRow](docs/PublicHeartRateRow.md)
 - [Model.PublicHeartRateSource](docs/PublicHeartRateSource.md)
 - [Model.PublicModifiedSleepModel](docs/PublicModifiedSleepModel.md)
 - [Model.PublicMomentMood](docs/PublicMomentMood.md)
 - [Model.PublicMomentType](docs/PublicMomentType.md)
 - [Model.PublicReadiness](docs/PublicReadiness.md)
 - [Model.PublicReadinessContributors](docs/PublicReadinessContributors.md)
 - [Model.PublicRestModeEpisode](docs/PublicRestModeEpisode.md)
 - [Model.PublicRestModePeriod](docs/PublicRestModePeriod.md)
 - [Model.PublicRingBatteryLevelRow](docs/PublicRingBatteryLevelRow.md)
 - [Model.PublicRingColor](docs/PublicRingColor.md)
 - [Model.PublicRingConfiguration](docs/PublicRingConfiguration.md)
 - [Model.PublicRingDesign](docs/PublicRingDesign.md)
 - [Model.PublicRingHardwareType](docs/PublicRingHardwareType.md)
 - [Model.PublicSample](docs/PublicSample.md)
 - [Model.PublicSession](docs/PublicSession.md)
 - [Model.PublicSleepAlgorithmVersion](docs/PublicSleepAlgorithmVersion.md)
 - [Model.PublicSleepAnalysisReason](docs/PublicSleepAnalysisReason.md)
 - [Model.PublicSleepContributors](docs/PublicSleepContributors.md)
 - [Model.PublicSleepTime](docs/PublicSleepTime.md)
 - [Model.PublicSleepTimeRecommendation](docs/PublicSleepTimeRecommendation.md)
 - [Model.PublicSleepTimeStatus](docs/PublicSleepTimeStatus.md)
 - [Model.PublicSleepTimeWindow](docs/PublicSleepTimeWindow.md)
 - [Model.PublicSleepType](docs/PublicSleepType.md)
 - [Model.PublicSpo2AggregatedValues](docs/PublicSpo2AggregatedValues.md)
 - [Model.PublicVO2Max](docs/PublicVO2Max.md)
 - [Model.PublicWorkout](docs/PublicWorkout.md)
 - [Model.PublicWorkoutIntensity](docs/PublicWorkoutIntensity.md)
 - [Model.PublicWorkoutSource](docs/PublicWorkoutSource.md)
 - [Model.ResilienceContributors](docs/ResilienceContributors.md)
 - [Model.ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet](docs/ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet.md)
 - [Model.ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet](docs/ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet.md)
 - [Model.ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet](docs/ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet.md)
 - [Model.ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet](docs/ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet.md)
 - [Model.TagModel](docs/TagModel.md)
 - [Model.TimeSeriesResponseDict](docs/TimeSeriesResponseDict.md)
 - [Model.TimeSeriesResponsePublicHeartRateRow](docs/TimeSeriesResponsePublicHeartRateRow.md)
 - [Model.TimeSeriesResponsePublicRingBatteryLevelRow](docs/TimeSeriesResponsePublicRingBatteryLevelRow.md)
 - [Model.UpdateWebhookSubscriptionRequest](docs/UpdateWebhookSubscriptionRequest.md)
 - [Model.ValidationError](docs/ValidationError.md)
 - [Model.ValidationErrorLocInner](docs/ValidationErrorLocInner.md)
 - [Model.WebhookOperation](docs/WebhookOperation.md)
 - [Model.WebhookSubscriptionModel](docs/WebhookSubscriptionModel.md)


<a id="documentation-for-authorization"></a>
## Documentation for Authorization


Authentication schemes defined for the API:
<a id="BearerAuth"></a>
### BearerAuth

- **Type**: Bearer Authentication

<a id="OAuth2"></a>
### OAuth2

- **Type**: OAuth
- **Flow**: accessCode
- **Authorization URL**: https://cloud.ouraring.com/oauth/authorize
- **Scopes**: 
  - email: Email address of the user
  - personal: Personal information (gender, age, height, weight)
  - daily: Daily summaries of sleep, activity and readiness
  - heartrate: Time series heart rate for Gen 3 users
  - workout: Summaries for auto-detected and user entered workouts
  - tag: User entered tags
  - session: Guided and unguided sessions in the Oura app
  - spo2Daily: SpO2 Average recorded during sleep

<a id="ClientIdAuth"></a>
### ClientIdAuth

- **Type**: API key
- **API key parameter name**: x-client-id
- **Location**: HTTP header

<a id="ClientSecretAuth"></a>
### ClientSecretAuth

- **Type**: API key
- **API key parameter name**: x-client-secret
- **Location**: HTTP header

