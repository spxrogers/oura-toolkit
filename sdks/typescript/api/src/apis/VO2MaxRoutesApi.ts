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
  HTTPValidationError,
  MultiDocumentResponsePublicVO2Max,
  PublicVO2Max,
} from '../models/index';
import {
    HTTPValidationErrorFromJSON,
    HTTPValidationErrorToJSON,
    MultiDocumentResponsePublicVO2MaxFromJSON,
    MultiDocumentResponsePublicVO2MaxToJSON,
    PublicVO2MaxFromJSON,
    PublicVO2MaxToJSON,
} from '../models/index';

export interface MultipleVO2MaxDocumentsV2UsercollectionVO2MaxGetRequest {
    startDate?: Date;
    endDate?: Date;
    nextToken?: string | null;
    fields?: string | null;
}

export interface SingleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGetRequest {
    documentId: string;
}

/**
 * 
 */
export class VO2MaxRoutesApi extends runtime.BaseAPI {

    /**
     * Multiple Vo2 Max Documents
     */
    async multipleVO2MaxDocumentsV2UsercollectionVO2MaxGetRaw(requestParameters: MultipleVO2MaxDocumentsV2UsercollectionVO2MaxGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<MultiDocumentResponsePublicVO2Max>> {
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

        if (requestParameters['fields'] != null) {
            queryParameters['fields'] = requestParameters['fields'];
        }

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.accessToken) {
            const token = this.configuration.accessToken;
            const tokenString = await token("BearerAuth", []);

            if (tokenString) {
                headerParameters["Authorization"] = `Bearer ${tokenString}`;
            }
        }

        let urlPath = `/v2/usercollection/vO2_max`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => MultiDocumentResponsePublicVO2MaxFromJSON(jsonValue));
    }

    /**
     * Multiple Vo2 Max Documents
     */
    async multipleVO2MaxDocumentsV2UsercollectionVO2MaxGet(requestParameters: MultipleVO2MaxDocumentsV2UsercollectionVO2MaxGetRequest = {}, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<MultiDocumentResponsePublicVO2Max> {
        const response = await this.multipleVO2MaxDocumentsV2UsercollectionVO2MaxGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Single Vo2 Max Document
     */
    async singleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGetRaw(requestParameters: SingleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<PublicVO2Max>> {
        if (requestParameters['documentId'] == null) {
            throw new runtime.RequiredError(
                'documentId',
                'Required parameter "documentId" was null or undefined when calling singleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGet().'
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

        let urlPath = `/v2/usercollection/vO2_max/{document_id}`;
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
     * Single Vo2 Max Document
     */
    async singleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGet(requestParameters: SingleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<PublicVO2Max> {
        const response = await this.singleVO2MaxDocumentV2UsercollectionVO2MaxDocumentIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
