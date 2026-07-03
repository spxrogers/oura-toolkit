/* tslint:disable */
/* eslint-disable */
/**
 * Oura API Documentation
 * # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | ------------------------------------ | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user\'s Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application\'s end-user tokens so one fan-out app can\'t dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client\'s backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 
 *
 * The version of the OpenAPI document: 2.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */


import * as runtime from '../runtime';
import type {
  DailyResilienceModel,
  EnhancedTagModel,
  HTTPValidationError,
  MultiDocumentResponseDailyResilienceModel,
  MultiDocumentResponseEnhancedTagModel,
  MultiDocumentResponsePublicDailyActivity,
  MultiDocumentResponsePublicDailyCardiovascularAge,
  MultiDocumentResponsePublicDailyReadiness,
  MultiDocumentResponsePublicDailySleep,
  MultiDocumentResponsePublicDailySpO2,
  MultiDocumentResponsePublicDailyStress,
  MultiDocumentResponsePublicModifiedSleepModel,
  MultiDocumentResponsePublicRestModePeriod,
  MultiDocumentResponsePublicRingConfiguration,
  MultiDocumentResponsePublicSession,
  MultiDocumentResponsePublicSleepTime,
  MultiDocumentResponsePublicVO2Max,
  MultiDocumentResponsePublicWorkout,
  MultiDocumentResponseTagModel,
  PublicDailyActivity,
  PublicDailyCardiovascularAge,
  PublicDailyReadiness,
  PublicDailySleep,
  PublicDailySpO2,
  PublicDailyStress,
  PublicModifiedSleepModel,
  PublicRestModePeriod,
  PublicRingConfiguration,
  PublicSession,
  PublicSleepTime,
  PublicVO2Max,
  PublicWorkout,
  ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet,
  ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet,
  TagModel,
} from '../models/index';
import {
    DailyResilienceModelFromJSON,
    DailyResilienceModelToJSON,
    EnhancedTagModelFromJSON,
    EnhancedTagModelToJSON,
    HTTPValidationErrorFromJSON,
    HTTPValidationErrorToJSON,
    MultiDocumentResponseDailyResilienceModelFromJSON,
    MultiDocumentResponseDailyResilienceModelToJSON,
    MultiDocumentResponseEnhancedTagModelFromJSON,
    MultiDocumentResponseEnhancedTagModelToJSON,
    MultiDocumentResponsePublicDailyActivityFromJSON,
    MultiDocumentResponsePublicDailyActivityToJSON,
    MultiDocumentResponsePublicDailyCardiovascularAgeFromJSON,
    MultiDocumentResponsePublicDailyCardiovascularAgeToJSON,
    MultiDocumentResponsePublicDailyReadinessFromJSON,
    MultiDocumentResponsePublicDailyReadinessToJSON,
    MultiDocumentResponsePublicDailySleepFromJSON,
    MultiDocumentResponsePublicDailySleepToJSON,
    MultiDocumentResponsePublicDailySpO2FromJSON,
    MultiDocumentResponsePublicDailySpO2ToJSON,
    MultiDocumentResponsePublicDailyStressFromJSON,
    MultiDocumentResponsePublicDailyStressToJSON,
    MultiDocumentResponsePublicModifiedSleepModelFromJSON,
    MultiDocumentResponsePublicModifiedSleepModelToJSON,
    MultiDocumentResponsePublicRestModePeriodFromJSON,
    MultiDocumentResponsePublicRestModePeriodToJSON,
    MultiDocumentResponsePublicRingConfigurationFromJSON,
    MultiDocumentResponsePublicRingConfigurationToJSON,
    MultiDocumentResponsePublicSessionFromJSON,
    MultiDocumentResponsePublicSessionToJSON,
    MultiDocumentResponsePublicSleepTimeFromJSON,
    MultiDocumentResponsePublicSleepTimeToJSON,
    MultiDocumentResponsePublicVO2MaxFromJSON,
    MultiDocumentResponsePublicVO2MaxToJSON,
    MultiDocumentResponsePublicWorkoutFromJSON,
    MultiDocumentResponsePublicWorkoutToJSON,
    MultiDocumentResponseTagModelFromJSON,
    MultiDocumentResponseTagModelToJSON,
    PublicDailyActivityFromJSON,
    PublicDailyActivityToJSON,
    PublicDailyCardiovascularAgeFromJSON,
    PublicDailyCardiovascularAgeToJSON,
    PublicDailyReadinessFromJSON,
    PublicDailyReadinessToJSON,
    PublicDailySleepFromJSON,
    PublicDailySleepToJSON,
    PublicDailySpO2FromJSON,
    PublicDailySpO2ToJSON,
    PublicDailyStressFromJSON,
    PublicDailyStressToJSON,
    PublicModifiedSleepModelFromJSON,
    PublicModifiedSleepModelToJSON,
    PublicRestModePeriodFromJSON,
    PublicRestModePeriodToJSON,
    PublicRingConfigurationFromJSON,
    PublicRingConfigurationToJSON,
    PublicSessionFromJSON,
    PublicSessionToJSON,
    PublicSleepTimeFromJSON,
    PublicSleepTimeToJSON,
    PublicVO2MaxFromJSON,
    PublicVO2MaxToJSON,
    PublicWorkoutFromJSON,
    PublicWorkoutToJSON,
    ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetFromJSON,
    ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetToJSON,
    ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetFromJSON,
    ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetToJSON,
    TagModelFromJSON,
    TagModelToJSON,
} from '../models/index';

export interface SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetRequest {
    startDatetime?: Date | null;
    endDatetime?: Date | null;
    nextToken?: string | null;
}

export interface SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetRequest {
    startDatetime?: Date | null;
    endDatetime?: Date | null;
    nextToken?: string | null;
}

export interface SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetRequest {
    nextToken?: string | null;
}

export interface SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
}

export interface SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetRequest {
    documentId: string;
}

export interface SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetRequest {
    documentId: string;
}

/**
 * 
 */
export class SandboxRoutesApi extends runtime.BaseAPI {

    /**
     * Sandbox - Multiple Daily Activity Documents
     */
    async sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetRaw(requestParameters: SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicDailyActivity>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_activity`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicDailyActivityFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Daily Activity Documents
     */
    async sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGet(requestParameters: SandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicDailyActivity> {
        const response = await this.sandboxMultipleDailyActivityDocumentsV2SandboxUsercollectionDailyActivityGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Daily Cardiovascular Age Documents
     */
    async sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetRaw(requestParameters: SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicDailyCardiovascularAge>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_cardiovascular_age`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicDailyCardiovascularAgeFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Daily Cardiovascular Age Documents
     */
    async sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGet(requestParameters: SandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicDailyCardiovascularAge> {
        const response = await this.sandboxMultipleDailyCardiovascularAgeDocumentsV2SandboxUsercollectionDailyCardiovascularAgeGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Daily Readiness Documents
     */
    async sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetRaw(requestParameters: SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicDailyReadiness>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_readiness`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicDailyReadinessFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Daily Readiness Documents
     */
    async sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGet(requestParameters: SandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicDailyReadiness> {
        const response = await this.sandboxMultipleDailyReadinessDocumentsV2SandboxUsercollectionDailyReadinessGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Daily Resilience Documents
     */
    async sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetRaw(requestParameters: SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponseDailyResilienceModel>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_resilience`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponseDailyResilienceModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Daily Resilience Documents
     */
    async sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGet(requestParameters: SandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponseDailyResilienceModel> {
        const response = await this.sandboxMultipleDailyResilienceDocumentsV2SandboxUsercollectionDailyResilienceGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Daily Sleep Documents
     */
    async sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetRaw(requestParameters: SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicDailySleep>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_sleep`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicDailySleepFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Daily Sleep Documents
     */
    async sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGet(requestParameters: SandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicDailySleep> {
        const response = await this.sandboxMultipleDailySleepDocumentsV2SandboxUsercollectionDailySleepGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Daily Spo2 Documents
     */
    async sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetRaw(requestParameters: SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicDailySpO2>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_spo2`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicDailySpO2FromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Daily Spo2 Documents
     */
    async sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2Get(requestParameters: SandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicDailySpO2> {
        const response = await this.sandboxMultipleDailySpo2DocumentsV2SandboxUsercollectionDailySpo2GetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Daily Stress Documents
     */
    async sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetRaw(requestParameters: SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicDailyStress>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_stress`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicDailyStressFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Daily Stress Documents
     */
    async sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGet(requestParameters: SandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicDailyStress> {
        const response = await this.sandboxMultipleDailyStressDocumentsV2SandboxUsercollectionDailyStressGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Enhanced Tag Documents
     */
    async sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetRaw(requestParameters: SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponseEnhancedTagModel>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/enhanced_tag`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponseEnhancedTagModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Enhanced Tag Documents
     */
    async sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGet(requestParameters: SandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponseEnhancedTagModel> {
        const response = await this.sandboxMultipleEnhancedTagDocumentsV2SandboxUsercollectionEnhancedTagGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Heartrate Documents
     */
    async sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetRaw(requestParameters: SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet>> {
        const queryParameters: any = {};

        if (requestParameters['startDatetime'] != null) {
            queryParameters['start_datetime'] = (requestParameters['startDatetime'] as any).toISOString();
        }

        if (requestParameters['endDatetime'] != null) {
            queryParameters['end_datetime'] = (requestParameters['endDatetime'] as any).toISOString();
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/heartrate`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Heartrate Documents
     */
    async sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet(requestParameters: SandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet> {
        const response = await this.sandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Rest Mode Period Documents
     */
    async sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetRaw(requestParameters: SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicRestModePeriod>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/rest_mode_period`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicRestModePeriodFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Rest Mode Period Documents
     */
    async sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGet(requestParameters: SandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicRestModePeriod> {
        const response = await this.sandboxMultipleRestModePeriodDocumentsV2SandboxUsercollectionRestModePeriodGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Ring Battery Level Documents
     */
    async sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetRaw(requestParameters: SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet>> {
        const queryParameters: any = {};

        if (requestParameters['startDatetime'] != null) {
            queryParameters['start_datetime'] = (requestParameters['startDatetime'] as any).toISOString();
        }

        if (requestParameters['endDatetime'] != null) {
            queryParameters['end_datetime'] = (requestParameters['endDatetime'] as any).toISOString();
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/ring_battery_level`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Ring Battery Level Documents
     */
    async sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet(requestParameters: SandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet> {
        const response = await this.sandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Ring Configuration Documents
     */
    async sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetRaw(requestParameters: SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicRingConfiguration>> {
        const queryParameters: any = {};

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/ring_configuration`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicRingConfigurationFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Ring Configuration Documents
     */
    async sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGet(requestParameters: SandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicRingConfiguration> {
        const response = await this.sandboxMultipleRingConfigurationDocumentsV2SandboxUsercollectionRingConfigurationGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Session Documents
     */
    async sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetRaw(requestParameters: SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicSession>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/session`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicSessionFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Session Documents
     */
    async sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGet(requestParameters: SandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicSession> {
        const response = await this.sandboxMultipleSessionDocumentsV2SandboxUsercollectionSessionGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Sleep Documents
     */
    async sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetRaw(requestParameters: SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicModifiedSleepModel>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/sleep`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicModifiedSleepModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Sleep Documents
     */
    async sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGet(requestParameters: SandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicModifiedSleepModel> {
        const response = await this.sandboxMultipleSleepDocumentsV2SandboxUsercollectionSleepGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Sleep Time Documents
     */
    async sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetRaw(requestParameters: SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicSleepTime>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/sleep_time`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicSleepTimeFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Sleep Time Documents
     */
    async sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGet(requestParameters: SandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicSleepTime> {
        const response = await this.sandboxMultipleSleepTimeDocumentsV2SandboxUsercollectionSleepTimeGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Tag Documents
     */
    async sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetRaw(requestParameters: SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponseTagModel>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/tag`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponseTagModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Tag Documents
     */
    async sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGet(requestParameters: SandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponseTagModel> {
        const response = await this.sandboxMultipleTagDocumentsV2SandboxUsercollectionTagGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Vo2 Max Documents
     */
    async sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetRaw(requestParameters: SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicVO2Max>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/vO2_max`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicVO2MaxFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Vo2 Max Documents
     */
    async sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGet(requestParameters: SandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicVO2Max> {
        const response = await this.sandboxMultipleVO2MaxDocumentsV2SandboxUsercollectionVO2MaxGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Multiple Workout Documents
     */
    async sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetRaw(requestParameters: SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicWorkout>> {
        const queryParameters: any = {};

        if (requestParameters['startDate'] != null) {
            queryParameters['start_date'] = (requestParameters['startDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['endDate'] != null) {
            queryParameters['end_date'] = (requestParameters['endDate'] as any).toISOString().substring(0,10);
        }

        if (requestParameters['nextToken'] != null) {
            queryParameters['next_token'] = requestParameters['nextToken'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/workout`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicWorkoutFromJSON(jsonValue));
    }

    /**
     * Sandbox - Multiple Workout Documents
     */
    async sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGet(requestParameters: SandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicWorkout> {
        const response = await this.sandboxMultipleWorkoutDocumentsV2SandboxUsercollectionWorkoutGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Daily Activity Document
     */
    async sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetRaw(requestParameters: SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicDailyActivity>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_activity/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicDailyActivityFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Daily Activity Document
     */
    async sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGet(requestParameters: SandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicDailyActivity> {
        const response = await this.sandboxSingleDailyActivityDocumentV2SandboxUsercollectionDailyActivityDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Daily Cardiovascular Age Document
     */
    async sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetRaw(requestParameters: SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicDailyCardiovascularAge>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_cardiovascular_age/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicDailyCardiovascularAgeFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Daily Cardiovascular Age Document
     */
    async sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGet(requestParameters: SandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicDailyCardiovascularAge> {
        const response = await this.sandboxSingleDailyCardiovascularAgeDocumentV2SandboxUsercollectionDailyCardiovascularAgeDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Daily Readiness Document
     */
    async sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetRaw(requestParameters: SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicDailyReadiness>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_readiness/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicDailyReadinessFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Daily Readiness Document
     */
    async sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGet(requestParameters: SandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicDailyReadiness> {
        const response = await this.sandboxSingleDailyReadinessDocumentV2SandboxUsercollectionDailyReadinessDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Daily Resilience Document
     */
    async sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetRaw(requestParameters: SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<DailyResilienceModel>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_resilience/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => DailyResilienceModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Daily Resilience Document
     */
    async sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGet(requestParameters: SandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<DailyResilienceModel> {
        const response = await this.sandboxSingleDailyResilienceDocumentV2SandboxUsercollectionDailyResilienceDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Daily Sleep Document
     */
    async sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetRaw(requestParameters: SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicDailySleep>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_sleep/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicDailySleepFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Daily Sleep Document
     */
    async sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGet(requestParameters: SandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicDailySleep> {
        const response = await this.sandboxSingleDailySleepDocumentV2SandboxUsercollectionDailySleepDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Daily Spo2 Document
     */
    async sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetRaw(requestParameters: SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicDailySpO2>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_spo2/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicDailySpO2FromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Daily Spo2 Document
     */
    async sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGet(requestParameters: SandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicDailySpO2> {
        const response = await this.sandboxSingleDailySpo2DocumentV2SandboxUsercollectionDailySpo2DocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Daily Stress Document
     */
    async sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetRaw(requestParameters: SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicDailyStress>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/daily_stress/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicDailyStressFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Daily Stress Document
     */
    async sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGet(requestParameters: SandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicDailyStress> {
        const response = await this.sandboxSingleDailyStressDocumentV2SandboxUsercollectionDailyStressDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Enhanced Tag Document
     */
    async sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetRaw(requestParameters: SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<EnhancedTagModel>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/enhanced_tag/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => EnhancedTagModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Enhanced Tag Document
     */
    async sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGet(requestParameters: SandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<EnhancedTagModel> {
        const response = await this.sandboxSingleEnhancedTagDocumentV2SandboxUsercollectionEnhancedTagDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Rest Mode Period Document
     */
    async sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetRaw(requestParameters: SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicRestModePeriod>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/rest_mode_period/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicRestModePeriodFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Rest Mode Period Document
     */
    async sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGet(requestParameters: SandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicRestModePeriod> {
        const response = await this.sandboxSingleRestModePeriodDocumentV2SandboxUsercollectionRestModePeriodDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Ring Configuration Document
     */
    async sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetRaw(requestParameters: SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicRingConfiguration>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/ring_configuration/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicRingConfigurationFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Ring Configuration Document
     */
    async sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGet(requestParameters: SandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicRingConfiguration> {
        const response = await this.sandboxSingleRingConfigurationDocumentV2SandboxUsercollectionRingConfigurationDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Session Document
     */
    async sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetRaw(requestParameters: SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicSession>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/session/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicSessionFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Session Document
     */
    async sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGet(requestParameters: SandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicSession> {
        const response = await this.sandboxSingleSessionDocumentV2SandboxUsercollectionSessionDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Sleep Document
     */
    async sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetRaw(requestParameters: SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicModifiedSleepModel>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/sleep/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicModifiedSleepModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Sleep Document
     */
    async sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGet(requestParameters: SandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicModifiedSleepModel> {
        const response = await this.sandboxSingleSleepDocumentV2SandboxUsercollectionSleepDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Sleep Time Document
     */
    async sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetRaw(requestParameters: SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicSleepTime>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/sleep_time/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicSleepTimeFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Sleep Time Document
     */
    async sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGet(requestParameters: SandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicSleepTime> {
        const response = await this.sandboxSingleSleepTimeDocumentV2SandboxUsercollectionSleepTimeDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Tag Document
     */
    async sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetRaw(requestParameters: SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<TagModel>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/tag/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => TagModelFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Tag Document
     */
    async sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGet(requestParameters: SandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<TagModel> {
        const response = await this.sandboxSingleTagDocumentV2SandboxUsercollectionTagDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Vo2 Max Document
     */
    async sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetRaw(requestParameters: SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicVO2Max>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/vO2_max/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicVO2MaxFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Vo2 Max Document
     */
    async sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGet(requestParameters: SandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicVO2Max> {
        const response = await this.sandboxSingleVO2MaxDocumentV2SandboxUsercollectionVO2MaxDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Sandbox - Single Workout Document
     */
    async sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetRaw(requestParameters: SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicWorkout>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/sandbox/usercollection/workout/{document_id}`;
        urlPath = urlPath.replace(`{${"document_id"}}`, encodeURIComponent(String(requestParameters['documentId'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => PublicWorkoutFromJSON(jsonValue));
    }

    /**
     * Sandbox - Single Workout Document
     */
    async sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGet(requestParameters: SandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicWorkout> {
        const response = await this.sandboxSingleWorkoutDocumentV2SandboxUsercollectionWorkoutDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
