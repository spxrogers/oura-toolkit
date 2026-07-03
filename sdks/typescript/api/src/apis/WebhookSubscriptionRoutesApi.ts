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
  CreateWebhookSubscriptionRequest,
  HTTPValidationError,
  UpdateWebhookSubscriptionRequest,
  WebhookSubscriptionModel,
} from '../models/index';
import {
    CreateWebhookSubscriptionRequestFromJSON,
    CreateWebhookSubscriptionRequestToJSON,
    HTTPValidationErrorFromJSON,
    HTTPValidationErrorToJSON,
    UpdateWebhookSubscriptionRequestFromJSON,
    UpdateWebhookSubscriptionRequestToJSON,
    WebhookSubscriptionModelFromJSON,
    WebhookSubscriptionModelToJSON,
} from '../models/index';

export interface CreateWebhookSubscriptionV2WebhookSubscriptionPostRequest {
    createWebhookSubscriptionRequest: CreateWebhookSubscriptionRequest;
}

export interface DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteRequest {
    id: string;
}

export interface GetWebhookSubscriptionV2WebhookSubscriptionIdGetRequest {
    id: string;
}

export interface RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutRequest {
    id: string;
}

export interface UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutRequest {
    id: string;
    updateWebhookSubscriptionRequest: UpdateWebhookSubscriptionRequest;
}

/**
 * 
 */
export class WebhookSubscriptionRoutesApi extends runtime.BaseAPI {

    /**
     * Create Webhook Subscription
     */
    async createWebhookSubscriptionV2WebhookSubscriptionPostRaw(requestParameters: CreateWebhookSubscriptionV2WebhookSubscriptionPostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<WebhookSubscriptionModel>> {
        if (requestParameters['createWebhookSubscriptionRequest'] == null) {
            throw new runtime.RequiredError(
                'createWebhookSubscriptionRequest',
                'Required parameter "createWebhookSubscriptionRequest" was null or undefined when calling createWebhookSubscriptionV2WebhookSubscriptionPost().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-id"] = await this.configuration.apiKey("x-client-id"); // ClientIdAuth authentication
        }

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-secret"] = await this.configuration.apiKey("x-client-secret"); // ClientSecretAuth authentication
        }


        let urlPath = `/v2/webhook/subscription`;

        const response = await this.request({
            path: urlPath,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateWebhookSubscriptionRequestToJSON(requestParameters['createWebhookSubscriptionRequest']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => WebhookSubscriptionModelFromJSON(jsonValue));
    }

    /**
     * Create Webhook Subscription
     */
    async createWebhookSubscriptionV2WebhookSubscriptionPost(requestParameters: CreateWebhookSubscriptionV2WebhookSubscriptionPostRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<WebhookSubscriptionModel> {
        const response = await this.createWebhookSubscriptionV2WebhookSubscriptionPostRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Delete Webhook Subscription
     */
    async deleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteRaw(requestParameters: DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<void>> {
        if (requestParameters['id'] == null) {
            throw new runtime.RequiredError(
                'id',
                'Required parameter "id" was null or undefined when calling deleteWebhookSubscriptionV2WebhookSubscriptionIdDelete().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-id"] = await this.configuration.apiKey("x-client-id"); // ClientIdAuth authentication
        }

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-secret"] = await this.configuration.apiKey("x-client-secret"); // ClientSecretAuth authentication
        }


        let urlPath = `/v2/webhook/subscription/{id}`;
        urlPath = urlPath.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters['id'])));

        const response = await this.request({
            path: urlPath,
            method: 'DELETE',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.VoidApiResponse(response);
    }

    /**
     * Delete Webhook Subscription
     */
    async deleteWebhookSubscriptionV2WebhookSubscriptionIdDelete(requestParameters: DeleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<void> {
        await this.deleteWebhookSubscriptionV2WebhookSubscriptionIdDeleteRaw(requestParameters, initOverrides);
    }

    /**
     * Get Webhook Subscription
     */
    async getWebhookSubscriptionV2WebhookSubscriptionIdGetRaw(requestParameters: GetWebhookSubscriptionV2WebhookSubscriptionIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<WebhookSubscriptionModel>> {
        if (requestParameters['id'] == null) {
            throw new runtime.RequiredError(
                'id',
                'Required parameter "id" was null or undefined when calling getWebhookSubscriptionV2WebhookSubscriptionIdGet().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-id"] = await this.configuration.apiKey("x-client-id"); // ClientIdAuth authentication
        }

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-secret"] = await this.configuration.apiKey("x-client-secret"); // ClientSecretAuth authentication
        }


        let urlPath = `/v2/webhook/subscription/{id}`;
        urlPath = urlPath.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters['id'])));

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => WebhookSubscriptionModelFromJSON(jsonValue));
    }

    /**
     * Get Webhook Subscription
     */
    async getWebhookSubscriptionV2WebhookSubscriptionIdGet(requestParameters: GetWebhookSubscriptionV2WebhookSubscriptionIdGetRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<WebhookSubscriptionModel> {
        const response = await this.getWebhookSubscriptionV2WebhookSubscriptionIdGetRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * List Webhook Subscriptions
     */
    async listWebhookSubscriptionsV2WebhookSubscriptionGetRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<Array<WebhookSubscriptionModel>>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-id"] = await this.configuration.apiKey("x-client-id"); // ClientIdAuth authentication
        }

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-secret"] = await this.configuration.apiKey("x-client-secret"); // ClientSecretAuth authentication
        }


        let urlPath = `/v2/webhook/subscription`;

        const response = await this.request({
            path: urlPath,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => jsonValue.map(WebhookSubscriptionModelFromJSON));
    }

    /**
     * List Webhook Subscriptions
     */
    async listWebhookSubscriptionsV2WebhookSubscriptionGet(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<Array<WebhookSubscriptionModel>> {
        const response = await this.listWebhookSubscriptionsV2WebhookSubscriptionGetRaw(initOverrides);
        return await response.value();
    }

    /**
     * Renew Webhook Subscription
     */
    async renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutRaw(requestParameters: RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<WebhookSubscriptionModel>> {
        if (requestParameters['id'] == null) {
            throw new runtime.RequiredError(
                'id',
                'Required parameter "id" was null or undefined when calling renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-id"] = await this.configuration.apiKey("x-client-id"); // ClientIdAuth authentication
        }

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-secret"] = await this.configuration.apiKey("x-client-secret"); // ClientSecretAuth authentication
        }


        let urlPath = `/v2/webhook/subscription/renew/{id}`;
        urlPath = urlPath.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters['id'])));

        const response = await this.request({
            path: urlPath,
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => WebhookSubscriptionModelFromJSON(jsonValue));
    }

    /**
     * Renew Webhook Subscription
     */
    async renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPut(requestParameters: RenewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<WebhookSubscriptionModel> {
        const response = await this.renewWebhookSubscriptionV2WebhookSubscriptionRenewIdPutRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Update Webhook Subscription
     */
    async updateWebhookSubscriptionV2WebhookSubscriptionIdPutRaw(requestParameters: UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<WebhookSubscriptionModel>> {
        if (requestParameters['id'] == null) {
            throw new runtime.RequiredError(
                'id',
                'Required parameter "id" was null or undefined when calling updateWebhookSubscriptionV2WebhookSubscriptionIdPut().'
            );
        }

        if (requestParameters['updateWebhookSubscriptionRequest'] == null) {
            throw new runtime.RequiredError(
                'updateWebhookSubscriptionRequest',
                'Required parameter "updateWebhookSubscriptionRequest" was null or undefined when calling updateWebhookSubscriptionV2WebhookSubscriptionIdPut().'
            );
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-id"] = await this.configuration.apiKey("x-client-id"); // ClientIdAuth authentication
        }

        if (this.configuration && this.configuration.apiKey) {
            headerParameters["x-client-secret"] = await this.configuration.apiKey("x-client-secret"); // ClientSecretAuth authentication
        }


        let urlPath = `/v2/webhook/subscription/{id}`;
        urlPath = urlPath.replace(`{${"id"}}`, encodeURIComponent(String(requestParameters['id'])));

        const response = await this.request({
            path: urlPath,
            method: 'PUT',
            headers: headerParameters,
            query: queryParameters,
            body: UpdateWebhookSubscriptionRequestToJSON(requestParameters['updateWebhookSubscriptionRequest']),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => WebhookSubscriptionModelFromJSON(jsonValue));
    }

    /**
     * Update Webhook Subscription
     */
    async updateWebhookSubscriptionV2WebhookSubscriptionIdPut(requestParameters: UpdateWebhookSubscriptionV2WebhookSubscriptionIdPutRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<WebhookSubscriptionModel> {
        const response = await this.updateWebhookSubscriptionV2WebhookSubscriptionIdPutRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
