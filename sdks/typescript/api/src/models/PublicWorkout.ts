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
import type { PublicWorkoutIntensity } from './PublicWorkoutIntensity';
import {
    PublicWorkoutIntensityFromJSON,
    PublicWorkoutIntensityFromJSONTyped,
    PublicWorkoutIntensityToJSON,
    PublicWorkoutIntensityToJSONTyped,
} from './PublicWorkoutIntensity';
import type { PublicWorkoutSource } from './PublicWorkoutSource';
import {
    PublicWorkoutSourceFromJSON,
    PublicWorkoutSourceFromJSONTyped,
    PublicWorkoutSourceToJSON,
    PublicWorkoutSourceToJSONTyped,
} from './PublicWorkoutSource';

/**
 * Public model for Workout.
 * @export
 * @interface PublicWorkout
 */
export interface PublicWorkout {
    /**
     * Unique identifier of the object.
     * @type {string}
     * @memberof PublicWorkout
     */
    id: string;
    /**
     * Type of the workout activity.
     * @type {string}
     * @memberof PublicWorkout
     */
    activity: string;
    /**
     * 
     * @type {number}
     * @memberof PublicWorkout
     */
    calories?: number | null;
    /**
     * 
     * @type {string}
     * @memberof PublicWorkout
     */
    day: string | null;
    /**
     * 
     * @type {number}
     * @memberof PublicWorkout
     */
    distance?: number | null;
    /**
     * 
     * @type {string}
     * @memberof PublicWorkout
     */
    endDatetime: string | null;
    /**
     * Intensity of the workout.
     * @type {PublicWorkoutIntensity}
     * @memberof PublicWorkout
     */
    intensity: PublicWorkoutIntensity;
    /**
     * 
     * @type {string}
     * @memberof PublicWorkout
     */
    label?: string | null;
    /**
     * Possible workout sources.
     * @type {PublicWorkoutSource}
     * @memberof PublicWorkout
     */
    source: PublicWorkoutSource;
    /**
     * 
     * @type {string}
     * @memberof PublicWorkout
     */
    startDatetime: string | null;
}



/**
 * Check if a given object implements the PublicWorkout interface.
 */
export function instanceOfPublicWorkout(value: object): value is PublicWorkout {
    if (!('id' in value) || value['id'] === undefined) return false;
    if (!('activity' in value) || value['activity'] === undefined) return false;
    if (!('day' in value) || value['day'] === undefined) return false;
    if (!('endDatetime' in value) || value['endDatetime'] === undefined) return false;
    if (!('intensity' in value) || value['intensity'] === undefined) return false;
    if (!('source' in value) || value['source'] === undefined) return false;
    if (!('startDatetime' in value) || value['startDatetime'] === undefined) return false;
    return true;
}

export function PublicWorkoutFromJSON(json: any): PublicWorkout {
    return PublicWorkoutFromJSONTyped(json, false);
}

export function PublicWorkoutFromJSONTyped(json: any, ignoreDiscriminator: boolean): PublicWorkout {
    if (json == null) {
        return json;
    }
    return {
        
        'id': json['id'],
        'activity': json['activity'],
        'calories': json['calories'] == null ? undefined : json['calories'],
        'day': json['day'],
        'distance': json['distance'] == null ? undefined : json['distance'],
        'endDatetime': json['end_datetime'],
        'intensity': PublicWorkoutIntensityFromJSON(json['intensity']),
        'label': json['label'] == null ? undefined : json['label'],
        'source': PublicWorkoutSourceFromJSON(json['source']),
        'startDatetime': json['start_datetime'],
    };
}

export function PublicWorkoutToJSON(json: any): PublicWorkout {
    return PublicWorkoutToJSONTyped(json, false);
}

export function PublicWorkoutToJSONTyped(value?: PublicWorkout | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'id': value['id'],
        'activity': value['activity'],
        'calories': value['calories'],
        'day': value['day'],
        'distance': value['distance'],
        'end_datetime': value['endDatetime'],
        'intensity': PublicWorkoutIntensityToJSON(value['intensity']),
        'label': value['label'],
        'source': PublicWorkoutSourceToJSON(value['source']),
        'start_datetime': value['startDatetime'],
    };
}

