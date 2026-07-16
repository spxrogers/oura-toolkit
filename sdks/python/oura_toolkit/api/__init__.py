# coding: utf-8

# flake8: noqa

"""
    Oura API Documentation

    # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | ------------------------------------ | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 

    The version of the OpenAPI document: 2.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


__version__ = "0.2.2"

# Define package exports
__all__ = [
    "DailyActivityRoutesApi",
    "DailyCardiovascularAgeRoutesApi",
    "DailyReadinessRoutesApi",
    "DailyResilienceRoutesApi",
    "DailySleepRoutesApi",
    "DailySpo2RoutesApi",
    "DailyStressRoutesApi",
    "EnhancedTagRoutesApi",
    "HeartRateRoutesApi",
    "PersonalInfoRoutesApi",
    "RestModePeriodRoutesApi",
    "RingBatteryLevelRoutesApi",
    "RingConfigurationRoutesApi",
    "SandboxRoutesApi",
    "SessionRoutesApi",
    "SleepRoutesApi",
    "SleepTimeRoutesApi",
    "TagRoutesApi",
    "VO2MaxRoutesApi",
    "WebhookSubscriptionRoutesApi",
    "WorkoutRoutesApi",
    "ApiResponse",
    "ApiClient",
    "Configuration",
    "OpenApiException",
    "ApiTypeError",
    "ApiValueError",
    "ApiKeyError",
    "ApiAttributeError",
    "ApiException",
    "CreateWebhookSubscriptionRequest",
    "DailyResilienceModel",
    "EnhancedTagModel",
    "ExtApiV2DataType",
    "HTTPValidationError",
    "LongTermResilienceLevel",
    "MultiDocumentResponseDailyResilienceModel",
    "MultiDocumentResponseEnhancedTagModel",
    "MultiDocumentResponsePublicDailyActivity",
    "MultiDocumentResponsePublicDailyCardiovascularAge",
    "MultiDocumentResponsePublicDailyReadiness",
    "MultiDocumentResponsePublicDailySleep",
    "MultiDocumentResponsePublicDailySpO2",
    "MultiDocumentResponsePublicDailyStress",
    "MultiDocumentResponsePublicModifiedSleepModel",
    "MultiDocumentResponsePublicRestModePeriod",
    "MultiDocumentResponsePublicRingConfiguration",
    "MultiDocumentResponsePublicSession",
    "MultiDocumentResponsePublicSleepTime",
    "MultiDocumentResponsePublicVO2Max",
    "MultiDocumentResponsePublicWorkout",
    "MultiDocumentResponseTagModel",
    "PersonalInfoResponse",
    "PublicActivityContributors",
    "PublicDailyActivity",
    "PublicDailyCardiovascularAge",
    "PublicDailyReadiness",
    "PublicDailySleep",
    "PublicDailySpO2",
    "PublicDailyStress",
    "PublicDailyStressSummary",
    "PublicHeartRateRow",
    "PublicHeartRateSource",
    "PublicModifiedSleepModel",
    "PublicMomentMood",
    "PublicMomentType",
    "PublicReadiness",
    "PublicReadinessContributors",
    "PublicRestModeEpisode",
    "PublicRestModePeriod",
    "PublicRingBatteryLevelRow",
    "PublicRingColor",
    "PublicRingConfiguration",
    "PublicRingDesign",
    "PublicRingHardwareType",
    "PublicSample",
    "PublicSession",
    "PublicSleepAlgorithmVersion",
    "PublicSleepAnalysisReason",
    "PublicSleepContributors",
    "PublicSleepTime",
    "PublicSleepTimeRecommendation",
    "PublicSleepTimeStatus",
    "PublicSleepTimeWindow",
    "PublicSleepType",
    "PublicSpo2AggregatedValues",
    "PublicVO2Max",
    "PublicWorkout",
    "PublicWorkoutIntensity",
    "PublicWorkoutSource",
    "ResilienceContributors",
    "ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet",
    "ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet",
    "ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet",
    "ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet",
    "TagModel",
    "TimeSeriesResponseDict",
    "TimeSeriesResponsePublicHeartRateRow",
    "TimeSeriesResponsePublicRingBatteryLevelRow",
    "UpdateWebhookSubscriptionRequest",
    "ValidationError",
    "ValidationErrorLocInner",
    "WebhookOperation",
    "WebhookSubscriptionModel",
]

# import apis into sdk package
from oura_toolkit.api.api.daily_activity_routes_api import DailyActivityRoutesApi as DailyActivityRoutesApi
from oura_toolkit.api.api.daily_cardiovascular_age_routes_api import DailyCardiovascularAgeRoutesApi as DailyCardiovascularAgeRoutesApi
from oura_toolkit.api.api.daily_readiness_routes_api import DailyReadinessRoutesApi as DailyReadinessRoutesApi
from oura_toolkit.api.api.daily_resilience_routes_api import DailyResilienceRoutesApi as DailyResilienceRoutesApi
from oura_toolkit.api.api.daily_sleep_routes_api import DailySleepRoutesApi as DailySleepRoutesApi
from oura_toolkit.api.api.daily_spo2_routes_api import DailySpo2RoutesApi as DailySpo2RoutesApi
from oura_toolkit.api.api.daily_stress_routes_api import DailyStressRoutesApi as DailyStressRoutesApi
from oura_toolkit.api.api.enhanced_tag_routes_api import EnhancedTagRoutesApi as EnhancedTagRoutesApi
from oura_toolkit.api.api.heart_rate_routes_api import HeartRateRoutesApi as HeartRateRoutesApi
from oura_toolkit.api.api.personal_info_routes_api import PersonalInfoRoutesApi as PersonalInfoRoutesApi
from oura_toolkit.api.api.rest_mode_period_routes_api import RestModePeriodRoutesApi as RestModePeriodRoutesApi
from oura_toolkit.api.api.ring_battery_level_routes_api import RingBatteryLevelRoutesApi as RingBatteryLevelRoutesApi
from oura_toolkit.api.api.ring_configuration_routes_api import RingConfigurationRoutesApi as RingConfigurationRoutesApi
from oura_toolkit.api.api.sandbox_routes_api import SandboxRoutesApi as SandboxRoutesApi
from oura_toolkit.api.api.session_routes_api import SessionRoutesApi as SessionRoutesApi
from oura_toolkit.api.api.sleep_routes_api import SleepRoutesApi as SleepRoutesApi
from oura_toolkit.api.api.sleep_time_routes_api import SleepTimeRoutesApi as SleepTimeRoutesApi
from oura_toolkit.api.api.tag_routes_api import TagRoutesApi as TagRoutesApi
from oura_toolkit.api.api.vo2_max_routes_api import VO2MaxRoutesApi as VO2MaxRoutesApi
from oura_toolkit.api.api.webhook_subscription_routes_api import WebhookSubscriptionRoutesApi as WebhookSubscriptionRoutesApi
from oura_toolkit.api.api.workout_routes_api import WorkoutRoutesApi as WorkoutRoutesApi

# import ApiClient
from oura_toolkit.api.api_response import ApiResponse as ApiResponse
from oura_toolkit.api.api_client import ApiClient as ApiClient
from oura_toolkit.api.configuration import Configuration as Configuration
from oura_toolkit.api.exceptions import OpenApiException as OpenApiException
from oura_toolkit.api.exceptions import ApiTypeError as ApiTypeError
from oura_toolkit.api.exceptions import ApiValueError as ApiValueError
from oura_toolkit.api.exceptions import ApiKeyError as ApiKeyError
from oura_toolkit.api.exceptions import ApiAttributeError as ApiAttributeError
from oura_toolkit.api.exceptions import ApiException as ApiException

# import models into sdk package
from oura_toolkit.api.models.create_webhook_subscription_request import CreateWebhookSubscriptionRequest as CreateWebhookSubscriptionRequest
from oura_toolkit.api.models.daily_resilience_model import DailyResilienceModel as DailyResilienceModel
from oura_toolkit.api.models.enhanced_tag_model import EnhancedTagModel as EnhancedTagModel
from oura_toolkit.api.models.ext_api_v2_data_type import ExtApiV2DataType as ExtApiV2DataType
from oura_toolkit.api.models.http_validation_error import HTTPValidationError as HTTPValidationError
from oura_toolkit.api.models.long_term_resilience_level import LongTermResilienceLevel as LongTermResilienceLevel
from oura_toolkit.api.models.multi_document_response_daily_resilience_model import MultiDocumentResponseDailyResilienceModel as MultiDocumentResponseDailyResilienceModel
from oura_toolkit.api.models.multi_document_response_enhanced_tag_model import MultiDocumentResponseEnhancedTagModel as MultiDocumentResponseEnhancedTagModel
from oura_toolkit.api.models.multi_document_response_public_daily_activity import MultiDocumentResponsePublicDailyActivity as MultiDocumentResponsePublicDailyActivity
from oura_toolkit.api.models.multi_document_response_public_daily_cardiovascular_age import MultiDocumentResponsePublicDailyCardiovascularAge as MultiDocumentResponsePublicDailyCardiovascularAge
from oura_toolkit.api.models.multi_document_response_public_daily_readiness import MultiDocumentResponsePublicDailyReadiness as MultiDocumentResponsePublicDailyReadiness
from oura_toolkit.api.models.multi_document_response_public_daily_sleep import MultiDocumentResponsePublicDailySleep as MultiDocumentResponsePublicDailySleep
from oura_toolkit.api.models.multi_document_response_public_daily_sp_o2 import MultiDocumentResponsePublicDailySpO2 as MultiDocumentResponsePublicDailySpO2
from oura_toolkit.api.models.multi_document_response_public_daily_stress import MultiDocumentResponsePublicDailyStress as MultiDocumentResponsePublicDailyStress
from oura_toolkit.api.models.multi_document_response_public_modified_sleep_model import MultiDocumentResponsePublicModifiedSleepModel as MultiDocumentResponsePublicModifiedSleepModel
from oura_toolkit.api.models.multi_document_response_public_rest_mode_period import MultiDocumentResponsePublicRestModePeriod as MultiDocumentResponsePublicRestModePeriod
from oura_toolkit.api.models.multi_document_response_public_ring_configuration import MultiDocumentResponsePublicRingConfiguration as MultiDocumentResponsePublicRingConfiguration
from oura_toolkit.api.models.multi_document_response_public_session import MultiDocumentResponsePublicSession as MultiDocumentResponsePublicSession
from oura_toolkit.api.models.multi_document_response_public_sleep_time import MultiDocumentResponsePublicSleepTime as MultiDocumentResponsePublicSleepTime
from oura_toolkit.api.models.multi_document_response_public_vo2_max import MultiDocumentResponsePublicVO2Max as MultiDocumentResponsePublicVO2Max
from oura_toolkit.api.models.multi_document_response_public_workout import MultiDocumentResponsePublicWorkout as MultiDocumentResponsePublicWorkout
from oura_toolkit.api.models.multi_document_response_tag_model import MultiDocumentResponseTagModel as MultiDocumentResponseTagModel
from oura_toolkit.api.models.personal_info_response import PersonalInfoResponse as PersonalInfoResponse
from oura_toolkit.api.models.public_activity_contributors import PublicActivityContributors as PublicActivityContributors
from oura_toolkit.api.models.public_daily_activity import PublicDailyActivity as PublicDailyActivity
from oura_toolkit.api.models.public_daily_cardiovascular_age import PublicDailyCardiovascularAge as PublicDailyCardiovascularAge
from oura_toolkit.api.models.public_daily_readiness import PublicDailyReadiness as PublicDailyReadiness
from oura_toolkit.api.models.public_daily_sleep import PublicDailySleep as PublicDailySleep
from oura_toolkit.api.models.public_daily_sp_o2 import PublicDailySpO2 as PublicDailySpO2
from oura_toolkit.api.models.public_daily_stress import PublicDailyStress as PublicDailyStress
from oura_toolkit.api.models.public_daily_stress_summary import PublicDailyStressSummary as PublicDailyStressSummary
from oura_toolkit.api.models.public_heart_rate_row import PublicHeartRateRow as PublicHeartRateRow
from oura_toolkit.api.models.public_heart_rate_source import PublicHeartRateSource as PublicHeartRateSource
from oura_toolkit.api.models.public_modified_sleep_model import PublicModifiedSleepModel as PublicModifiedSleepModel
from oura_toolkit.api.models.public_moment_mood import PublicMomentMood as PublicMomentMood
from oura_toolkit.api.models.public_moment_type import PublicMomentType as PublicMomentType
from oura_toolkit.api.models.public_readiness import PublicReadiness as PublicReadiness
from oura_toolkit.api.models.public_readiness_contributors import PublicReadinessContributors as PublicReadinessContributors
from oura_toolkit.api.models.public_rest_mode_episode import PublicRestModeEpisode as PublicRestModeEpisode
from oura_toolkit.api.models.public_rest_mode_period import PublicRestModePeriod as PublicRestModePeriod
from oura_toolkit.api.models.public_ring_battery_level_row import PublicRingBatteryLevelRow as PublicRingBatteryLevelRow
from oura_toolkit.api.models.public_ring_color import PublicRingColor as PublicRingColor
from oura_toolkit.api.models.public_ring_configuration import PublicRingConfiguration as PublicRingConfiguration
from oura_toolkit.api.models.public_ring_design import PublicRingDesign as PublicRingDesign
from oura_toolkit.api.models.public_ring_hardware_type import PublicRingHardwareType as PublicRingHardwareType
from oura_toolkit.api.models.public_sample import PublicSample as PublicSample
from oura_toolkit.api.models.public_session import PublicSession as PublicSession
from oura_toolkit.api.models.public_sleep_algorithm_version import PublicSleepAlgorithmVersion as PublicSleepAlgorithmVersion
from oura_toolkit.api.models.public_sleep_analysis_reason import PublicSleepAnalysisReason as PublicSleepAnalysisReason
from oura_toolkit.api.models.public_sleep_contributors import PublicSleepContributors as PublicSleepContributors
from oura_toolkit.api.models.public_sleep_time import PublicSleepTime as PublicSleepTime
from oura_toolkit.api.models.public_sleep_time_recommendation import PublicSleepTimeRecommendation as PublicSleepTimeRecommendation
from oura_toolkit.api.models.public_sleep_time_status import PublicSleepTimeStatus as PublicSleepTimeStatus
from oura_toolkit.api.models.public_sleep_time_window import PublicSleepTimeWindow as PublicSleepTimeWindow
from oura_toolkit.api.models.public_sleep_type import PublicSleepType as PublicSleepType
from oura_toolkit.api.models.public_spo2_aggregated_values import PublicSpo2AggregatedValues as PublicSpo2AggregatedValues
from oura_toolkit.api.models.public_vo2_max import PublicVO2Max as PublicVO2Max
from oura_toolkit.api.models.public_workout import PublicWorkout as PublicWorkout
from oura_toolkit.api.models.public_workout_intensity import PublicWorkoutIntensity as PublicWorkoutIntensity
from oura_toolkit.api.models.public_workout_source import PublicWorkoutSource as PublicWorkoutSource
from oura_toolkit.api.models.resilience_contributors import ResilienceContributors as ResilienceContributors
from oura_toolkit.api.models.response_multiple_heartrate_documents_v2_usercollection_heartrate_get import ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet as ResponseMultipleHeartrateDocumentsV2UsercollectionHeartrateGet
from oura_toolkit.api.models.response_multiple_ring_battery_level_documents_v2_usercollection_ring_battery_level_get import ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet as ResponseMultipleRingBatteryLevelDocumentsV2UsercollectionRingBatteryLevelGet
from oura_toolkit.api.models.response_sandbox_multiple_heartrate_documents_v2_sandbox_usercollection_heartrate_get import ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet as ResponseSandboxMultipleHeartrateDocumentsV2SandboxUsercollectionHeartrateGet
from oura_toolkit.api.models.response_sandbox_multiple_ring_battery_level_documents_v2_sandbox_usercollection_ring_battery_level_get import ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet as ResponseSandboxMultipleRingBatteryLevelDocumentsV2SandboxUsercollectionRingBatteryLevelGet
from oura_toolkit.api.models.tag_model import TagModel as TagModel
from oura_toolkit.api.models.time_series_response_dict import TimeSeriesResponseDict as TimeSeriesResponseDict
from oura_toolkit.api.models.time_series_response_public_heart_rate_row import TimeSeriesResponsePublicHeartRateRow as TimeSeriesResponsePublicHeartRateRow
from oura_toolkit.api.models.time_series_response_public_ring_battery_level_row import TimeSeriesResponsePublicRingBatteryLevelRow as TimeSeriesResponsePublicRingBatteryLevelRow
from oura_toolkit.api.models.update_webhook_subscription_request import UpdateWebhookSubscriptionRequest as UpdateWebhookSubscriptionRequest
from oura_toolkit.api.models.validation_error import ValidationError as ValidationError
from oura_toolkit.api.models.validation_error_loc_inner import ValidationErrorLocInner as ValidationErrorLocInner
from oura_toolkit.api.models.webhook_operation import WebhookOperation as WebhookOperation
from oura_toolkit.api.models.webhook_subscription_model import WebhookSubscriptionModel as WebhookSubscriptionModel
