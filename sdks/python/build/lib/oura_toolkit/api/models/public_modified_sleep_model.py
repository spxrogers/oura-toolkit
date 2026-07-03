# coding: utf-8

"""
    Oura API Documentation

    # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | ------------------------------------ | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 

    The version of the OpenAPI document: 2.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictFloat, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional, Union
from typing_extensions import Annotated
from oura_toolkit.api.models.public_readiness import PublicReadiness
from oura_toolkit.api.models.public_sample import PublicSample
from oura_toolkit.api.models.public_sleep_algorithm_version import PublicSleepAlgorithmVersion
from oura_toolkit.api.models.public_sleep_analysis_reason import PublicSleepAnalysisReason
from oura_toolkit.api.models.public_sleep_type import PublicSleepType
from typing import Optional, Set
from typing_extensions import Self

class PublicModifiedSleepModel(BaseModel):
    """
    PublicModifiedSleepModel
    """ # noqa: E501
    id: Annotated[str, Field(min_length=1, strict=True)] = Field(description="Unique identifier of the object.")
    average_breath: Optional[Union[StrictFloat, StrictInt]] = None
    average_heart_rate: Optional[Union[StrictFloat, StrictInt]] = None
    average_hrv: Optional[StrictInt] = None
    awake_time: Optional[StrictInt] = None
    bedtime_end: Optional[StrictStr]
    bedtime_start: Optional[StrictStr]
    day: Optional[StrictStr]
    deep_sleep_duration: Optional[StrictInt] = None
    efficiency: Optional[StrictInt] = None
    heart_rate: Optional[PublicSample] = None
    hrv: Optional[PublicSample] = None
    latency: Optional[StrictInt] = None
    light_sleep_duration: Optional[StrictInt] = None
    low_battery_alert: StrictBool = Field(description="Flag indicating if a low battery alert occurred.")
    lowest_heart_rate: Optional[StrictInt] = None
    movement_30_sec: Optional[StrictStr] = None
    period: StrictInt = Field(description="ECore sleep period identifier.")
    readiness: Optional[PublicReadiness] = None
    readiness_score_delta: Optional[StrictInt] = None
    rem_sleep_duration: Optional[StrictInt] = None
    restless_periods: Optional[StrictInt] = None
    sleep_algorithm_version: Optional[PublicSleepAlgorithmVersion] = None
    sleep_analysis_reason: Optional[PublicSleepAnalysisReason] = None
    sleep_phase_30_sec: Optional[StrictStr] = None
    sleep_phase_5_min: Optional[StrictStr] = None
    sleep_score_delta: Optional[StrictInt] = None
    time_in_bed: StrictInt = Field(description="Duration spent in bed in seconds.")
    total_sleep_duration: Optional[StrictInt] = None
    type: Optional[PublicSleepType] = None
    ring_id: Optional[StrictStr] = None
    app_sleep_phase_5_min: Optional[StrictStr] = None
    __properties: ClassVar[List[str]] = ["id", "average_breath", "average_heart_rate", "average_hrv", "awake_time", "bedtime_end", "bedtime_start", "day", "deep_sleep_duration", "efficiency", "heart_rate", "hrv", "latency", "light_sleep_duration", "low_battery_alert", "lowest_heart_rate", "movement_30_sec", "period", "readiness", "readiness_score_delta", "rem_sleep_duration", "restless_periods", "sleep_algorithm_version", "sleep_analysis_reason", "sleep_phase_30_sec", "sleep_phase_5_min", "sleep_score_delta", "time_in_bed", "total_sleep_duration", "type", "ring_id", "app_sleep_phase_5_min"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of PublicModifiedSleepModel from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # override the default output from pydantic by calling `to_dict()` of heart_rate
        if self.heart_rate:
            _dict['heart_rate'] = self.heart_rate.to_dict()
        # override the default output from pydantic by calling `to_dict()` of hrv
        if self.hrv:
            _dict['hrv'] = self.hrv.to_dict()
        # override the default output from pydantic by calling `to_dict()` of readiness
        if self.readiness:
            _dict['readiness'] = self.readiness.to_dict()
        # set to None if average_breath (nullable) is None
        # and model_fields_set contains the field
        if self.average_breath is None and "average_breath" in self.model_fields_set:
            _dict['average_breath'] = None

        # set to None if average_heart_rate (nullable) is None
        # and model_fields_set contains the field
        if self.average_heart_rate is None and "average_heart_rate" in self.model_fields_set:
            _dict['average_heart_rate'] = None

        # set to None if average_hrv (nullable) is None
        # and model_fields_set contains the field
        if self.average_hrv is None and "average_hrv" in self.model_fields_set:
            _dict['average_hrv'] = None

        # set to None if awake_time (nullable) is None
        # and model_fields_set contains the field
        if self.awake_time is None and "awake_time" in self.model_fields_set:
            _dict['awake_time'] = None

        # set to None if bedtime_end (nullable) is None
        # and model_fields_set contains the field
        if self.bedtime_end is None and "bedtime_end" in self.model_fields_set:
            _dict['bedtime_end'] = None

        # set to None if bedtime_start (nullable) is None
        # and model_fields_set contains the field
        if self.bedtime_start is None and "bedtime_start" in self.model_fields_set:
            _dict['bedtime_start'] = None

        # set to None if day (nullable) is None
        # and model_fields_set contains the field
        if self.day is None and "day" in self.model_fields_set:
            _dict['day'] = None

        # set to None if deep_sleep_duration (nullable) is None
        # and model_fields_set contains the field
        if self.deep_sleep_duration is None and "deep_sleep_duration" in self.model_fields_set:
            _dict['deep_sleep_duration'] = None

        # set to None if efficiency (nullable) is None
        # and model_fields_set contains the field
        if self.efficiency is None and "efficiency" in self.model_fields_set:
            _dict['efficiency'] = None

        # set to None if heart_rate (nullable) is None
        # and model_fields_set contains the field
        if self.heart_rate is None and "heart_rate" in self.model_fields_set:
            _dict['heart_rate'] = None

        # set to None if hrv (nullable) is None
        # and model_fields_set contains the field
        if self.hrv is None and "hrv" in self.model_fields_set:
            _dict['hrv'] = None

        # set to None if latency (nullable) is None
        # and model_fields_set contains the field
        if self.latency is None and "latency" in self.model_fields_set:
            _dict['latency'] = None

        # set to None if light_sleep_duration (nullable) is None
        # and model_fields_set contains the field
        if self.light_sleep_duration is None and "light_sleep_duration" in self.model_fields_set:
            _dict['light_sleep_duration'] = None

        # set to None if lowest_heart_rate (nullable) is None
        # and model_fields_set contains the field
        if self.lowest_heart_rate is None and "lowest_heart_rate" in self.model_fields_set:
            _dict['lowest_heart_rate'] = None

        # set to None if movement_30_sec (nullable) is None
        # and model_fields_set contains the field
        if self.movement_30_sec is None and "movement_30_sec" in self.model_fields_set:
            _dict['movement_30_sec'] = None

        # set to None if readiness (nullable) is None
        # and model_fields_set contains the field
        if self.readiness is None and "readiness" in self.model_fields_set:
            _dict['readiness'] = None

        # set to None if readiness_score_delta (nullable) is None
        # and model_fields_set contains the field
        if self.readiness_score_delta is None and "readiness_score_delta" in self.model_fields_set:
            _dict['readiness_score_delta'] = None

        # set to None if rem_sleep_duration (nullable) is None
        # and model_fields_set contains the field
        if self.rem_sleep_duration is None and "rem_sleep_duration" in self.model_fields_set:
            _dict['rem_sleep_duration'] = None

        # set to None if restless_periods (nullable) is None
        # and model_fields_set contains the field
        if self.restless_periods is None and "restless_periods" in self.model_fields_set:
            _dict['restless_periods'] = None

        # set to None if sleep_algorithm_version (nullable) is None
        # and model_fields_set contains the field
        if self.sleep_algorithm_version is None and "sleep_algorithm_version" in self.model_fields_set:
            _dict['sleep_algorithm_version'] = None

        # set to None if sleep_analysis_reason (nullable) is None
        # and model_fields_set contains the field
        if self.sleep_analysis_reason is None and "sleep_analysis_reason" in self.model_fields_set:
            _dict['sleep_analysis_reason'] = None

        # set to None if sleep_phase_30_sec (nullable) is None
        # and model_fields_set contains the field
        if self.sleep_phase_30_sec is None and "sleep_phase_30_sec" in self.model_fields_set:
            _dict['sleep_phase_30_sec'] = None

        # set to None if sleep_phase_5_min (nullable) is None
        # and model_fields_set contains the field
        if self.sleep_phase_5_min is None and "sleep_phase_5_min" in self.model_fields_set:
            _dict['sleep_phase_5_min'] = None

        # set to None if sleep_score_delta (nullable) is None
        # and model_fields_set contains the field
        if self.sleep_score_delta is None and "sleep_score_delta" in self.model_fields_set:
            _dict['sleep_score_delta'] = None

        # set to None if total_sleep_duration (nullable) is None
        # and model_fields_set contains the field
        if self.total_sleep_duration is None and "total_sleep_duration" in self.model_fields_set:
            _dict['total_sleep_duration'] = None

        # set to None if type (nullable) is None
        # and model_fields_set contains the field
        if self.type is None and "type" in self.model_fields_set:
            _dict['type'] = None

        # set to None if ring_id (nullable) is None
        # and model_fields_set contains the field
        if self.ring_id is None and "ring_id" in self.model_fields_set:
            _dict['ring_id'] = None

        # set to None if app_sleep_phase_5_min (nullable) is None
        # and model_fields_set contains the field
        if self.app_sleep_phase_5_min is None and "app_sleep_phase_5_min" in self.model_fields_set:
            _dict['app_sleep_phase_5_min'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of PublicModifiedSleepModel from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "average_breath": obj.get("average_breath"),
            "average_heart_rate": obj.get("average_heart_rate"),
            "average_hrv": obj.get("average_hrv"),
            "awake_time": obj.get("awake_time"),
            "bedtime_end": obj.get("bedtime_end"),
            "bedtime_start": obj.get("bedtime_start"),
            "day": obj.get("day"),
            "deep_sleep_duration": obj.get("deep_sleep_duration"),
            "efficiency": obj.get("efficiency"),
            "heart_rate": PublicSample.from_dict(obj["heart_rate"]) if obj.get("heart_rate") is not None else None,
            "hrv": PublicSample.from_dict(obj["hrv"]) if obj.get("hrv") is not None else None,
            "latency": obj.get("latency"),
            "light_sleep_duration": obj.get("light_sleep_duration"),
            "low_battery_alert": obj.get("low_battery_alert"),
            "lowest_heart_rate": obj.get("lowest_heart_rate"),
            "movement_30_sec": obj.get("movement_30_sec"),
            "period": obj.get("period"),
            "readiness": PublicReadiness.from_dict(obj["readiness"]) if obj.get("readiness") is not None else None,
            "readiness_score_delta": obj.get("readiness_score_delta"),
            "rem_sleep_duration": obj.get("rem_sleep_duration"),
            "restless_periods": obj.get("restless_periods"),
            "sleep_algorithm_version": obj.get("sleep_algorithm_version"),
            "sleep_analysis_reason": obj.get("sleep_analysis_reason"),
            "sleep_phase_30_sec": obj.get("sleep_phase_30_sec"),
            "sleep_phase_5_min": obj.get("sleep_phase_5_min"),
            "sleep_score_delta": obj.get("sleep_score_delta"),
            "time_in_bed": obj.get("time_in_bed"),
            "total_sleep_duration": obj.get("total_sleep_duration"),
            "type": obj.get("type"),
            "ring_id": obj.get("ring_id"),
            "app_sleep_phase_5_min": obj.get("app_sleep_phase_5_min")
        })
        return _obj


