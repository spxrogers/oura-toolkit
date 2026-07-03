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
import type { PublicSample } from './PublicSample';
import {
    PublicSampleFromJSON,
    PublicSampleFromJSONTyped,
    PublicSampleToJSON,
    PublicSampleToJSONTyped,
} from './PublicSample';
import type { PublicActivityContributors } from './PublicActivityContributors';
import {
    PublicActivityContributorsFromJSON,
    PublicActivityContributorsFromJSONTyped,
    PublicActivityContributorsToJSON,
    PublicActivityContributorsToJSONTyped,
} from './PublicActivityContributors';

/**
 * Object defining a daily activity that is a 24-hour period starting at 4 a.m.
 * @export
 * @interface PublicDailyActivity
 */
export interface PublicDailyActivity {
    /**
     * Unique identifier of the object.
     * @type {string}
     * @memberof PublicDailyActivity
     */
    id: string;
    /**
     * Active calories expended in kilocalories.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    activeCalories: number;
    /**
     * Average MET minutes.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    averageMetMinutes: number;
    /**
     * 
     * @type {string}
     * @memberof PublicDailyActivity
     */
    class5Min?: string | null;
    /**
     * Object containing activity score contributors.
     * @type {PublicActivityContributors}
     * @memberof PublicDailyActivity
     */
    contributors: PublicActivityContributors;
    /**
     * 
     * @type {string}
     * @memberof PublicDailyActivity
     */
    day: string | null;
    /**
     * Equivalent walking distance of energe expenditure in meters.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    equivalentWalkingDistance: number;
    /**
     * The total METs of each minute classified as high activity.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    highActivityMetMinutes: number;
    /**
     * The total time in seconds of each minute classified as high activity.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    highActivityTime: number;
    /**
     * Number of inactivity alerts received.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    inactivityAlerts: number;
    /**
     * The total METs of each minute classified as low activity.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    lowActivityMetMinutes: number;
    /**
     * The total time in seconds of each minute classified as low activity.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    lowActivityTime: number;
    /**
     * The total METs of each minute classified as medium activity.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    mediumActivityMetMinutes: number;
    /**
     * The total time in seconds of each minute classified as medium activity.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    mediumActivityTime: number;
    /**
     * Sample containing METs.
     * @type {PublicSample}
     * @memberof PublicDailyActivity
     */
    met: PublicSample;
    /**
     * Meters remaining to target.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    metersToTarget: number;
    /**
     * Ring non-wear time in seconds.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    nonWearTime: number;
    /**
     * Resting time in seconds.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    restingTime: number;
    /**
     * 
     * @type {number}
     * @memberof PublicDailyActivity
     */
    score?: number | null;
    /**
     * Sedentary MET minutes.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    sedentaryMetMinutes: number;
    /**
     * Sedentary time in seconds.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    sedentaryTime: number;
    /**
     * Total number of steps taken.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    steps: number;
    /**
     * Daily activity target in kilocalories.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    targetCalories: number;
    /**
     * Daily activity target in meters.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    targetMeters: number;
    /**
     * 
     * @type {string}
     * @memberof PublicDailyActivity
     */
    timestamp: string | null;
    /**
     * Total calories expended in kilocalories.
     * @type {number}
     * @memberof PublicDailyActivity
     */
    totalCalories: number;
}

/**
 * Check if a given object implements the PublicDailyActivity interface.
 */
export function instanceOfPublicDailyActivity(value: object): value is PublicDailyActivity {
    if (!('id' in value) || value['id'] === undefined) return false;
    if (!('activeCalories' in value) || value['activeCalories'] === undefined) return false;
    if (!('averageMetMinutes' in value) || value['averageMetMinutes'] === undefined) return false;
    if (!('contributors' in value) || value['contributors'] === undefined) return false;
    if (!('day' in value) || value['day'] === undefined) return false;
    if (!('equivalentWalkingDistance' in value) || value['equivalentWalkingDistance'] === undefined) return false;
    if (!('highActivityMetMinutes' in value) || value['highActivityMetMinutes'] === undefined) return false;
    if (!('highActivityTime' in value) || value['highActivityTime'] === undefined) return false;
    if (!('inactivityAlerts' in value) || value['inactivityAlerts'] === undefined) return false;
    if (!('lowActivityMetMinutes' in value) || value['lowActivityMetMinutes'] === undefined) return false;
    if (!('lowActivityTime' in value) || value['lowActivityTime'] === undefined) return false;
    if (!('mediumActivityMetMinutes' in value) || value['mediumActivityMetMinutes'] === undefined) return false;
    if (!('mediumActivityTime' in value) || value['mediumActivityTime'] === undefined) return false;
    if (!('met' in value) || value['met'] === undefined) return false;
    if (!('metersToTarget' in value) || value['metersToTarget'] === undefined) return false;
    if (!('nonWearTime' in value) || value['nonWearTime'] === undefined) return false;
    if (!('restingTime' in value) || value['restingTime'] === undefined) return false;
    if (!('sedentaryMetMinutes' in value) || value['sedentaryMetMinutes'] === undefined) return false;
    if (!('sedentaryTime' in value) || value['sedentaryTime'] === undefined) return false;
    if (!('steps' in value) || value['steps'] === undefined) return false;
    if (!('targetCalories' in value) || value['targetCalories'] === undefined) return false;
    if (!('targetMeters' in value) || value['targetMeters'] === undefined) return false;
    if (!('timestamp' in value) || value['timestamp'] === undefined) return false;
    if (!('totalCalories' in value) || value['totalCalories'] === undefined) return false;
    return true;
}

export function PublicDailyActivityFromJSON(json: any): PublicDailyActivity {
    return PublicDailyActivityFromJSONTyped(json, false);
}

export function PublicDailyActivityFromJSONTyped(json: any, ignoreDiscriminator: boolean): PublicDailyActivity {
    if (json == null) {
        return json;
    }
    return {
        
        'id': json['id'],
        'activeCalories': json['active_calories'],
        'averageMetMinutes': json['average_met_minutes'],
        'class5Min': json['class_5_min'] == null ? undefined : json['class_5_min'],
        'contributors': PublicActivityContributorsFromJSON(json['contributors']),
        'day': json['day'],
        'equivalentWalkingDistance': json['equivalent_walking_distance'],
        'highActivityMetMinutes': json['high_activity_met_minutes'],
        'highActivityTime': json['high_activity_time'],
        'inactivityAlerts': json['inactivity_alerts'],
        'lowActivityMetMinutes': json['low_activity_met_minutes'],
        'lowActivityTime': json['low_activity_time'],
        'mediumActivityMetMinutes': json['medium_activity_met_minutes'],
        'mediumActivityTime': json['medium_activity_time'],
        'met': PublicSampleFromJSON(json['met']),
        'metersToTarget': json['meters_to_target'],
        'nonWearTime': json['non_wear_time'],
        'restingTime': json['resting_time'],
        'score': json['score'] == null ? undefined : json['score'],
        'sedentaryMetMinutes': json['sedentary_met_minutes'],
        'sedentaryTime': json['sedentary_time'],
        'steps': json['steps'],
        'targetCalories': json['target_calories'],
        'targetMeters': json['target_meters'],
        'timestamp': json['timestamp'],
        'totalCalories': json['total_calories'],
    };
}

export function PublicDailyActivityToJSON(json: any): PublicDailyActivity {
    return PublicDailyActivityToJSONTyped(json, false);
}

export function PublicDailyActivityToJSONTyped(value?: PublicDailyActivity | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'id': value['id'],
        'active_calories': value['activeCalories'],
        'average_met_minutes': value['averageMetMinutes'],
        'class_5_min': value['class5Min'],
        'contributors': PublicActivityContributorsToJSON(value['contributors']),
        'day': value['day'],
        'equivalent_walking_distance': value['equivalentWalkingDistance'],
        'high_activity_met_minutes': value['highActivityMetMinutes'],
        'high_activity_time': value['highActivityTime'],
        'inactivity_alerts': value['inactivityAlerts'],
        'low_activity_met_minutes': value['lowActivityMetMinutes'],
        'low_activity_time': value['lowActivityTime'],
        'medium_activity_met_minutes': value['mediumActivityMetMinutes'],
        'medium_activity_time': value['mediumActivityTime'],
        'met': PublicSampleToJSON(value['met']),
        'meters_to_target': value['metersToTarget'],
        'non_wear_time': value['nonWearTime'],
        'resting_time': value['restingTime'],
        'score': value['score'],
        'sedentary_met_minutes': value['sedentaryMetMinutes'],
        'sedentary_time': value['sedentaryTime'],
        'steps': value['steps'],
        'target_calories': value['targetCalories'],
        'target_meters': value['targetMeters'],
        'timestamp': value['timestamp'],
        'total_calories': value['totalCalories'],
    };
}

