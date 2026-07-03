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

from pydantic import BaseModel, ConfigDict, StrictInt
from typing import Any, ClassVar, Dict, List, Optional
from typing import Optional, Set
from typing_extensions import Self

class PublicReadinessContributors(BaseModel):
    """
    Object defining readiness score contributors.
    """ # noqa: E501
    activity_balance: Optional[StrictInt] = None
    body_temperature: Optional[StrictInt] = None
    hrv_balance: Optional[StrictInt] = None
    previous_day_activity: Optional[StrictInt] = None
    previous_night: Optional[StrictInt] = None
    recovery_index: Optional[StrictInt] = None
    resting_heart_rate: Optional[StrictInt] = None
    sleep_balance: Optional[StrictInt] = None
    sleep_regularity: Optional[StrictInt] = None
    __properties: ClassVar[List[str]] = ["activity_balance", "body_temperature", "hrv_balance", "previous_day_activity", "previous_night", "recovery_index", "resting_heart_rate", "sleep_balance", "sleep_regularity"]

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
        """Create an instance of PublicReadinessContributors from a JSON string"""
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
        # set to None if activity_balance (nullable) is None
        # and model_fields_set contains the field
        if self.activity_balance is None and "activity_balance" in self.model_fields_set:
            _dict['activity_balance'] = None

        # set to None if body_temperature (nullable) is None
        # and model_fields_set contains the field
        if self.body_temperature is None and "body_temperature" in self.model_fields_set:
            _dict['body_temperature'] = None

        # set to None if hrv_balance (nullable) is None
        # and model_fields_set contains the field
        if self.hrv_balance is None and "hrv_balance" in self.model_fields_set:
            _dict['hrv_balance'] = None

        # set to None if previous_day_activity (nullable) is None
        # and model_fields_set contains the field
        if self.previous_day_activity is None and "previous_day_activity" in self.model_fields_set:
            _dict['previous_day_activity'] = None

        # set to None if previous_night (nullable) is None
        # and model_fields_set contains the field
        if self.previous_night is None and "previous_night" in self.model_fields_set:
            _dict['previous_night'] = None

        # set to None if recovery_index (nullable) is None
        # and model_fields_set contains the field
        if self.recovery_index is None and "recovery_index" in self.model_fields_set:
            _dict['recovery_index'] = None

        # set to None if resting_heart_rate (nullable) is None
        # and model_fields_set contains the field
        if self.resting_heart_rate is None and "resting_heart_rate" in self.model_fields_set:
            _dict['resting_heart_rate'] = None

        # set to None if sleep_balance (nullable) is None
        # and model_fields_set contains the field
        if self.sleep_balance is None and "sleep_balance" in self.model_fields_set:
            _dict['sleep_balance'] = None

        # set to None if sleep_regularity (nullable) is None
        # and model_fields_set contains the field
        if self.sleep_regularity is None and "sleep_regularity" in self.model_fields_set:
            _dict['sleep_regularity'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of PublicReadinessContributors from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "activity_balance": obj.get("activity_balance"),
            "body_temperature": obj.get("body_temperature"),
            "hrv_balance": obj.get("hrv_balance"),
            "previous_day_activity": obj.get("previous_day_activity"),
            "previous_night": obj.get("previous_night"),
            "recovery_index": obj.get("recovery_index"),
            "resting_heart_rate": obj.get("resting_heart_rate"),
            "sleep_balance": obj.get("sleep_balance"),
            "sleep_regularity": obj.get("sleep_regularity")
        })
        return _obj


