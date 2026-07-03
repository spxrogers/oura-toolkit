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

from pydantic import BaseModel, ConfigDict, Field, StrictFloat, StrictInt, StrictStr
from typing import Any, ClassVar, Dict, List, Optional, Union
from typing_extensions import Annotated
from oura_toolkit.api.models.public_activity_contributors import PublicActivityContributors
from oura_toolkit.api.models.public_sample import PublicSample
from typing import Optional, Set
from typing_extensions import Self

class PublicDailyActivity(BaseModel):
    """
    Object defining a daily activity that is a 24-hour period starting at 4 a.m.
    """ # noqa: E501
    id: Annotated[str, Field(min_length=1, strict=True)] = Field(description="Unique identifier of the object.")
    active_calories: StrictInt = Field(description="Active calories expended in kilocalories.")
    average_met_minutes: Union[StrictFloat, StrictInt] = Field(description="Average MET minutes.")
    class_5_min: Optional[StrictStr] = None
    contributors: PublicActivityContributors = Field(description="Object containing activity score contributors.")
    day: Optional[StrictStr]
    equivalent_walking_distance: StrictInt = Field(description="Equivalent walking distance of energe expenditure in meters.")
    high_activity_met_minutes: StrictInt = Field(description="The total METs of each minute classified as high activity.")
    high_activity_time: StrictInt = Field(description="The total time in seconds of each minute classified as high activity.")
    inactivity_alerts: StrictInt = Field(description="Number of inactivity alerts received.")
    low_activity_met_minutes: StrictInt = Field(description="The total METs of each minute classified as low activity.")
    low_activity_time: StrictInt = Field(description="The total time in seconds of each minute classified as low activity.")
    medium_activity_met_minutes: StrictInt = Field(description="The total METs of each minute classified as medium activity.")
    medium_activity_time: StrictInt = Field(description="The total time in seconds of each minute classified as medium activity.")
    met: PublicSample = Field(description="Sample containing METs.")
    meters_to_target: StrictInt = Field(description="Meters remaining to target.")
    non_wear_time: StrictInt = Field(description="Ring non-wear time in seconds.")
    resting_time: StrictInt = Field(description="Resting time in seconds.")
    score: Optional[StrictInt] = None
    sedentary_met_minutes: StrictInt = Field(description="Sedentary MET minutes.")
    sedentary_time: StrictInt = Field(description="Sedentary time in seconds.")
    steps: StrictInt = Field(description="Total number of steps taken.")
    target_calories: StrictInt = Field(description="Daily activity target in kilocalories.")
    target_meters: StrictInt = Field(description="Daily activity target in meters.")
    timestamp: Optional[StrictStr]
    total_calories: StrictInt = Field(description="Total calories expended in kilocalories.")
    __properties: ClassVar[List[str]] = ["id", "active_calories", "average_met_minutes", "class_5_min", "contributors", "day", "equivalent_walking_distance", "high_activity_met_minutes", "high_activity_time", "inactivity_alerts", "low_activity_met_minutes", "low_activity_time", "medium_activity_met_minutes", "medium_activity_time", "met", "meters_to_target", "non_wear_time", "resting_time", "score", "sedentary_met_minutes", "sedentary_time", "steps", "target_calories", "target_meters", "timestamp", "total_calories"]

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
        """Create an instance of PublicDailyActivity from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of contributors
        if self.contributors:
            _dict['contributors'] = self.contributors.to_dict()
        # override the default output from pydantic by calling `to_dict()` of met
        if self.met:
            _dict['met'] = self.met.to_dict()
        # set to None if class_5_min (nullable) is None
        # and model_fields_set contains the field
        if self.class_5_min is None and "class_5_min" in self.model_fields_set:
            _dict['class_5_min'] = None

        # set to None if day (nullable) is None
        # and model_fields_set contains the field
        if self.day is None and "day" in self.model_fields_set:
            _dict['day'] = None

        # set to None if score (nullable) is None
        # and model_fields_set contains the field
        if self.score is None and "score" in self.model_fields_set:
            _dict['score'] = None

        # set to None if timestamp (nullable) is None
        # and model_fields_set contains the field
        if self.timestamp is None and "timestamp" in self.model_fields_set:
            _dict['timestamp'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of PublicDailyActivity from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "id": obj.get("id"),
            "active_calories": obj.get("active_calories"),
            "average_met_minutes": obj.get("average_met_minutes"),
            "class_5_min": obj.get("class_5_min"),
            "contributors": PublicActivityContributors.from_dict(obj["contributors"]) if obj.get("contributors") is not None else None,
            "day": obj.get("day"),
            "equivalent_walking_distance": obj.get("equivalent_walking_distance"),
            "high_activity_met_minutes": obj.get("high_activity_met_minutes"),
            "high_activity_time": obj.get("high_activity_time"),
            "inactivity_alerts": obj.get("inactivity_alerts"),
            "low_activity_met_minutes": obj.get("low_activity_met_minutes"),
            "low_activity_time": obj.get("low_activity_time"),
            "medium_activity_met_minutes": obj.get("medium_activity_met_minutes"),
            "medium_activity_time": obj.get("medium_activity_time"),
            "met": PublicSample.from_dict(obj["met"]) if obj.get("met") is not None else None,
            "meters_to_target": obj.get("meters_to_target"),
            "non_wear_time": obj.get("non_wear_time"),
            "resting_time": obj.get("resting_time"),
            "score": obj.get("score"),
            "sedentary_met_minutes": obj.get("sedentary_met_minutes"),
            "sedentary_time": obj.get("sedentary_time"),
            "steps": obj.get("steps"),
            "target_calories": obj.get("target_calories"),
            "target_meters": obj.get("target_meters"),
            "timestamp": obj.get("timestamp"),
            "total_calories": obj.get("total_calories")
        })
        return _obj


