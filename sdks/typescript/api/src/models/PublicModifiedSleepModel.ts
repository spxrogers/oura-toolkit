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
import type { PublicSleepAnalysisReason } from './PublicSleepAnalysisReason';
import {
    PublicSleepAnalysisReasonFromJSON,
    PublicSleepAnalysisReasonFromJSONTyped,
    PublicSleepAnalysisReasonToJSON,
    PublicSleepAnalysisReasonToJSONTyped,
} from './PublicSleepAnalysisReason';
import type { PublicSleepAlgorithmVersion } from './PublicSleepAlgorithmVersion';
import {
    PublicSleepAlgorithmVersionFromJSON,
    PublicSleepAlgorithmVersionFromJSONTyped,
    PublicSleepAlgorithmVersionToJSON,
    PublicSleepAlgorithmVersionToJSONTyped,
} from './PublicSleepAlgorithmVersion';
import type { PublicSleepType } from './PublicSleepType';
import {
    PublicSleepTypeFromJSON,
    PublicSleepTypeFromJSONTyped,
    PublicSleepTypeToJSON,
    PublicSleepTypeToJSONTyped,
} from './PublicSleepType';
import type { PublicSample } from './PublicSample';
import {
    PublicSampleFromJSON,
    PublicSampleFromJSONTyped,
    PublicSampleToJSON,
    PublicSampleToJSONTyped,
} from './PublicSample';
import type { PublicReadiness } from './PublicReadiness';
import {
    PublicReadinessFromJSON,
    PublicReadinessFromJSONTyped,
    PublicReadinessToJSON,
    PublicReadinessToJSONTyped,
} from './PublicReadiness';

/**
 * 
 * @export
 * @interface PublicModifiedSleepModel
 */
export interface PublicModifiedSleepModel {
    /**
     * Unique identifier of the object.
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    id: string;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    averageBreath?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    averageHeartRate?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    averageHrv?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    awakeTime?: number | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    bedtimeEnd: string | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    bedtimeStart: string | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    day: string | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    deepSleepDuration?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    efficiency?: number | null;
    /**
     * 
     * @type {PublicSample}
     * @memberof PublicModifiedSleepModel
     */
    heartRate?: PublicSample | null;
    /**
     * 
     * @type {PublicSample}
     * @memberof PublicModifiedSleepModel
     */
    hrv?: PublicSample | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    latency?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    lightSleepDuration?: number | null;
    /**
     * Flag indicating if a low battery alert occurred.
     * @type {boolean}
     * @memberof PublicModifiedSleepModel
     */
    lowBatteryAlert: boolean;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    lowestHeartRate?: number | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    movement30Sec?: string | null;
    /**
     * ECore sleep period identifier.
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    period: number;
    /**
     * 
     * @type {PublicReadiness}
     * @memberof PublicModifiedSleepModel
     */
    readiness?: PublicReadiness | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    readinessScoreDelta?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    remSleepDuration?: number | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    restlessPeriods?: number | null;
    /**
     * 
     * @type {PublicSleepAlgorithmVersion}
     * @memberof PublicModifiedSleepModel
     */
    sleepAlgorithmVersion?: PublicSleepAlgorithmVersion | null;
    /**
     * 
     * @type {PublicSleepAnalysisReason}
     * @memberof PublicModifiedSleepModel
     */
    sleepAnalysisReason?: PublicSleepAnalysisReason | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    sleepPhase30Sec?: string | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    sleepPhase5Min?: string | null;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    sleepScoreDelta?: number | null;
    /**
     * Duration spent in bed in seconds.
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    timeInBed: number;
    /**
     * 
     * @type {number}
     * @memberof PublicModifiedSleepModel
     */
    totalSleepDuration?: number | null;
    /**
     * 
     * @type {PublicSleepType}
     * @memberof PublicModifiedSleepModel
     */
    type?: PublicSleepType | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    ringId?: string | null;
    /**
     * 
     * @type {string}
     * @memberof PublicModifiedSleepModel
     */
    appSleepPhase5Min?: string | null;
}



/**
 * Check if a given object implements the PublicModifiedSleepModel interface.
 */
export function instanceOfPublicModifiedSleepModel(value: object): value is PublicModifiedSleepModel {
    if (!('id' in value) || value['id'] === undefined) return false;
    if (!('bedtimeEnd' in value) || value['bedtimeEnd'] === undefined) return false;
    if (!('bedtimeStart' in value) || value['bedtimeStart'] === undefined) return false;
    if (!('day' in value) || value['day'] === undefined) return false;
    if (!('lowBatteryAlert' in value) || value['lowBatteryAlert'] === undefined) return false;
    if (!('period' in value) || value['period'] === undefined) return false;
    if (!('timeInBed' in value) || value['timeInBed'] === undefined) return false;
    return true;
}

export function PublicModifiedSleepModelFromJSON(json: any): PublicModifiedSleepModel {
    return PublicModifiedSleepModelFromJSONTyped(json, false);
}

export function PublicModifiedSleepModelFromJSONTyped(json: any, ignoreDiscriminator: boolean): PublicModifiedSleepModel {
    if (json == null) {
        return json;
    }
    return {
        
        'id': json['id'],
        'averageBreath': json['average_breath'] == null ? undefined : json['average_breath'],
        'averageHeartRate': json['average_heart_rate'] == null ? undefined : json['average_heart_rate'],
        'averageHrv': json['average_hrv'] == null ? undefined : json['average_hrv'],
        'awakeTime': json['awake_time'] == null ? undefined : json['awake_time'],
        'bedtimeEnd': json['bedtime_end'],
        'bedtimeStart': json['bedtime_start'],
        'day': json['day'],
        'deepSleepDuration': json['deep_sleep_duration'] == null ? undefined : json['deep_sleep_duration'],
        'efficiency': json['efficiency'] == null ? undefined : json['efficiency'],
        'heartRate': json['heart_rate'] == null ? undefined : PublicSampleFromJSON(json['heart_rate']),
        'hrv': json['hrv'] == null ? undefined : PublicSampleFromJSON(json['hrv']),
        'latency': json['latency'] == null ? undefined : json['latency'],
        'lightSleepDuration': json['light_sleep_duration'] == null ? undefined : json['light_sleep_duration'],
        'lowBatteryAlert': json['low_battery_alert'],
        'lowestHeartRate': json['lowest_heart_rate'] == null ? undefined : json['lowest_heart_rate'],
        'movement30Sec': json['movement_30_sec'] == null ? undefined : json['movement_30_sec'],
        'period': json['period'],
        'readiness': json['readiness'] == null ? undefined : PublicReadinessFromJSON(json['readiness']),
        'readinessScoreDelta': json['readiness_score_delta'] == null ? undefined : json['readiness_score_delta'],
        'remSleepDuration': json['rem_sleep_duration'] == null ? undefined : json['rem_sleep_duration'],
        'restlessPeriods': json['restless_periods'] == null ? undefined : json['restless_periods'],
        'sleepAlgorithmVersion': json['sleep_algorithm_version'] == null ? undefined : PublicSleepAlgorithmVersionFromJSON(json['sleep_algorithm_version']),
        'sleepAnalysisReason': json['sleep_analysis_reason'] == null ? undefined : PublicSleepAnalysisReasonFromJSON(json['sleep_analysis_reason']),
        'sleepPhase30Sec': json['sleep_phase_30_sec'] == null ? undefined : json['sleep_phase_30_sec'],
        'sleepPhase5Min': json['sleep_phase_5_min'] == null ? undefined : json['sleep_phase_5_min'],
        'sleepScoreDelta': json['sleep_score_delta'] == null ? undefined : json['sleep_score_delta'],
        'timeInBed': json['time_in_bed'],
        'totalSleepDuration': json['total_sleep_duration'] == null ? undefined : json['total_sleep_duration'],
        'type': json['type'] == null ? undefined : PublicSleepTypeFromJSON(json['type']),
        'ringId': json['ring_id'] == null ? undefined : json['ring_id'],
        'appSleepPhase5Min': json['app_sleep_phase_5_min'] == null ? undefined : json['app_sleep_phase_5_min'],
    };
}

export function PublicModifiedSleepModelToJSON(json: any): PublicModifiedSleepModel {
    return PublicModifiedSleepModelToJSONTyped(json, false);
}

export function PublicModifiedSleepModelToJSONTyped(value?: PublicModifiedSleepModel | null, ignoreDiscriminator: boolean = false): any {
    if (value == null) {
        return value;
    }

    return {
        
        'id': value['id'],
        'average_breath': value['averageBreath'],
        'average_heart_rate': value['averageHeartRate'],
        'average_hrv': value['averageHrv'],
        'awake_time': value['awakeTime'],
        'bedtime_end': value['bedtimeEnd'],
        'bedtime_start': value['bedtimeStart'],
        'day': value['day'],
        'deep_sleep_duration': value['deepSleepDuration'],
        'efficiency': value['efficiency'],
        'heart_rate': PublicSampleToJSON(value['heartRate']),
        'hrv': PublicSampleToJSON(value['hrv']),
        'latency': value['latency'],
        'light_sleep_duration': value['lightSleepDuration'],
        'low_battery_alert': value['lowBatteryAlert'],
        'lowest_heart_rate': value['lowestHeartRate'],
        'movement_30_sec': value['movement30Sec'],
        'period': value['period'],
        'readiness': PublicReadinessToJSON(value['readiness']),
        'readiness_score_delta': value['readinessScoreDelta'],
        'rem_sleep_duration': value['remSleepDuration'],
        'restless_periods': value['restlessPeriods'],
        'sleep_algorithm_version': PublicSleepAlgorithmVersionToJSON(value['sleepAlgorithmVersion']),
        'sleep_analysis_reason': PublicSleepAnalysisReasonToJSON(value['sleepAnalysisReason']),
        'sleep_phase_30_sec': value['sleepPhase30Sec'],
        'sleep_phase_5_min': value['sleepPhase5Min'],
        'sleep_score_delta': value['sleepScoreDelta'],
        'time_in_bed': value['timeInBed'],
        'total_sleep_duration': value['totalSleepDuration'],
        'type': PublicSleepTypeToJSON(value['type']),
        'ring_id': value['ringId'],
        'app_sleep_phase_5_min': value['appSleepPhase5Min'],
    };
}

