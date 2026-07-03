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

import { mapValues } from '../runtime';
/**
 * Object defining readiness score contributors.
 * @export
 * @interface PublicReadinessContributors
 */
export interface PublicReadinessContributors {
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    activityBalance?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    bodyTemperature?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    hrvBalance?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    previousDayActivity?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    previousNight?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    recoveryIndex?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    restingHeartRate?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    sleepBalance?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicReadinessContributors
     */
    sleepRegularity?: number | null;
}

/**
 * Check if a given object implements the PublicReadinessContributors interface.
 */
export function instanceOfPublicReadinessContributors(value: object): value is PublicReadinessContributors {
    return true;
}

export function PublicReadinessContributorsFromJSON(json: any): PublicReadinessContributors {
    return PublicReadinessContributorsFromJSONTyped(json, false);
}

export function PublicReadinessContributorsFromJSONTyped(json: any, ignoreDiscriminator: boolean): PublicReadinessContributors {
    if (json == null) {
        return json;
    }
    return {
        
        'activityBalance': json['activity_balance'] == null ? undefined : json['activity_balance'],
        'bodyTemperature': json['body_temperature'] == null ? undefined : json['body_temperature'],
        'hrvBalance': json['hrv_balance'] == null ? undefined : json['hrv_balance'],
        'previousDayActivity': json['previous_day_activity'] == null ? undefined : json['previous_day_activity'],
        'previousNight': json['previous_night'] == null ? undefined : json['previous_night'],
        'recoveryIndex': json['recovery_index'] == null ? undefined : json['recovery_index'],
        'restingHeartRate': json['resting_heart_rate'] == null ? undefined : json['resting_heart_rate'],
        'sleepBalance': json['sleep_balance'] == null ? undefined : json['sleep_balance'],
        'sleepRegularity': json['sleep_regularity'] == null ? undefined : json['sleep_regularity'],
    };
}

export function PublicReadinessContributorsToJSON(json: any): PublicReadinessContributors {
    return PublicReadinessContributorsToJSONTyped(json, false);
}

export function PublicReadinessContributorsToJSONTyped(value?: PublicReadinessContributors | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'activity_balance': value['activityBalance'],
        'body_temperature': value['bodyTemperature'],
        'hrv_balance': value['hrvBalance'],
        'previous_day_activity': value['previousDayActivity'],
        'previous_night': value['previousNight'],
        'recovery_index': value['recoveryIndex'],
        'resting_heart_rate': value['restingHeartRate'],
        'sleep_balance': value['sleepBalance'],
        'sleep_regularity': value['sleepRegularity'],
    };
}

