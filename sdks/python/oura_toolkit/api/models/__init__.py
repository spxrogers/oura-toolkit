# coding: utf-8

# flake8: noqa
"""
    Oura API Documentation

    # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | ------------------------------------ | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 

    The version of the OpenAPI document: 2.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


# import models into model package
from oura_toolkit.api.models.create_webhook_subscription_request import CreateWebhookSubscriptionRequest
from oura_toolkit.api.models.daily_resilience_model import DailyResilienceModel
from oura_toolkit.api.models.enhanced_tag_model import EnhancedTagModel
from oura_toolkit.api.models.ext_api_v2_data_type import ExtApiV2DataType
from oura_toolkit.api.models.http_validation_error import HTTPValidationError
from oura_toolkit.api.models.long_term_resilience_level import LongTermResilienceLevel
from oura_toolkit.api.models.multi_document_response_daily_resilience_model import MultiDocumentResponseDailyResilienceModel
from oura_toolkit.api.models.multi_document_response_enhanced_tag_model import MultiDocumentResponseEnhancedTagModel
from oura_toolkit.api.models.multi_document_response_public_daily_activity import MultiDocumentResponsePublicDailyActivity
from oura_toolkit.api.models.multi_document_response_public_daily_cardiovascular_age import MultiDocumentResponsePublicDailyCardiovascularAge
from oura_toolkit.api.models.multi_document_response_public_daily_readiness import MultiDocumentResponsePublicDailyReadiness
from oura_toolkit.api.models.multi_document_response_public_daily_sleep import MultiDocumentResponsePublicDailySleep
from oura_toolkit.api.models.multi_document_response_public_daily_sp_o2 import MultiDocumentResponsePublicDailySpO2
from oura_toolkit.api.models.multi_document_response_public_daily_stress import MultiDocumentResponsePublicDailyStress
from oura_toolkit.api.models.multi_document_response_public_modified_sleep_model import MultiDocumentResponsePublicModifiedSleepModel
from oura_toolkit.api.models.multi_document_response_public_rest_mode_period import MultiDocumentResponsePublicRestModePeriod
from oura_toolkit.api.models.multi_document_response_public_ring_configuration import MultiDocumentResponsePublicRingConfiguration
from oura_toolkit.api.models.multi_document_response_public_session import MultiDocumentResponsePublicSession
from oura_toolkit.api.models.multi_document_response_public_sleep_time import MultiDocumentResponsePublicSleepTime
from oura_toolkit.api.models.multi_document_response_public_vo2_max import MultiDocumentResponsePublicVO2Max
from oura_toolkit.api.models.multi_document_response_public_workout import MultiDocumentResponsePublicWorkout
from oura_toolkit.api.models.multi_document_response_tag_model import MultiDocumentResponseTagModel
from oura_toolkit.api.models.personal_info_response import PersonalInfoResponse
from oura_toolkit.api.models.public_activity_contributors import PublicActivityContributors
from oura_toolkit.api.models.public_daily_activity import PublicDailyActivity
from oura_toolkit.api.models.public_daily_cardiovascular_age import PublicDailyCardiovascularAge
from oura_toolkit.api.models.public_daily_readiness import PublicDailyReadiness
from oura_toolkit.api.models.public_daily_sleep import PublicDailySleep
from oura_toolkit.api.models.public_daily_sp_o2 import PublicDailySpO2
from oura_toolkit.api.models.public_daily_stress import PublicDailyStress
from oura_toolkit.api.models.public_daily_stress_summary import PublicDailyStressSummary
from oura_toolkit.api.models.public_heart_rate_row import PublicHeartRateRow
from oura_toolkit.api.models.public_heart_rate_source import PublicHeartRateSource
from oura_toolkit.api.models.public_modified_sleep_model import PublicModifiedSleepModel
from oura_toolkit.api.models.public_moment_mood import PublicMomentMood
from oura_toolkit.api.models.public_moment_type import PublicMomentType
from oura_toolkit.api.models.public_readiness import PublicReadiness
from oura_toolkit.api.models.public_readiness_contributors import PublicReadinessContributors
from oura_toolkit.api.models.public_rest_mode_episode import PublicRestModeEpisode
from oura_toolkit.api.models.public_rest_mode_period import PublicRestModePeriod
from oura_toolkit.api.models.public_ring_battery_level_row import PublicRingBatteryLevelRow
from oura_toolkit.api.models.public_ring_color import PublicRingColor
from oura_toolkit.api.models.public_ring_configuration import PublicRingConfiguration
from oura_toolkit.api.models.public_ring_design import PublicRingDesign
from oura_toolkit.api.models.public_ring_hardware_type import PublicRingHardwareType
from oura_toolkit.api.models.public_sample import PublicSample
from oura_toolkit.api.models.public_session import PublicSession
from oura_toolkit.api.models.public_sleep_algorithm_version import PublicSleepAlgorithmVersion
from oura_toolkit.api.models.public_sleep_analysis_reason import PublicSleepAnalysisReason
from oura_toolkit.api.models.public_sleep_contributors import PublicSleepContributors
from oura_toolkit.api.models.public_sleep_time import PublicSleepTime
from oura_toolkit.api.models.public_sleep_time_recommendation import PublicSleepTimeRecommendation
from oura_toolkit.api.models.public_sleep_time_status import PublicSleepTimeStatus
from oura_toolkit.api.models.public_sleep_time_window import PublicSleepTimeWindow
from oura_toolkit.api.models.public_sleep_type import PublicSleepType
from oura_toolkit.api.models.public_spo2_aggregated_values import PublicSpo2AggregatedValues
from oura_toolkit.api.models.public_vo2_max import PublicVO2Max
from oura_toolkit.api.models.public_workout import PublicWorkout
from oura_toolkit.api.models.public_workout_intensity import PublicWorkoutIntensity
from oura_toolkit.api.models.public_workout_source import PublicWorkoutSource
from oura_toolkit.api.models.resilience_contributors import ResilienceContributors
from oura_toolkit.api.models.response_multiple_heartrate_documents_v2_usercollection_heartrate_get import ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet
from oura_toolkit.api.models.response_multiple_ring_battery_level_documents_v2_usercollection_ring_battery_level_get import ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet
from oura_toolkit.api.models.response_sandbox_multiple_heartrate_documents_v2_sandbox_usercollection_heartrate_get import ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet
from oura_toolkit.api.models.response_sandbox_multiple_ring_battery_level_documents_v2_sandbox_usercollection_ring_battery_level_get import ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet
from oura_toolkit.api.models.tag_model import TagModel
from oura_toolkit.api.models.time_series_response_dict import TimeSeriesResponseDict
from oura_toolkit.api.models.time_series_response_public_heart_rate_row import TimeSeriesResponsePublicHeartRateRow
from oura_toolkit.api.models.time_series_response_public_ring_battery_level_row import TimeSeriesResponsePublicRingBatteryLevelRow
from oura_toolkit.api.models.update_webhook_subscription_request import UpdateWebhookSubscriptionRequest
from oura_toolkit.api.models.validation_error import ValidationError
from oura_toolkit.api.models.validation_error_loc_inner import ValidationErrorLocInner
from oura_toolkit.api.models.webhook_operation import WebhookOperation
from oura_toolkit.api.models.webhook_subscription_model import WebhookSubscriptionModel
