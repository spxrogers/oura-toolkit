/*
 * Oura API Documentation
 * # Overview  The Oura API allows Oura users and partner applications to improve their user experience with Oura data. This document describes the Oura API Version 2 (V2), which is the only available integration point for Oura data. The previous V1 API has been sunset. # Getting Started  ## What is an API? An API (Application Programming Interface) allows different software applications to communicate with each other. The Oura API enables you to access your Oura Ring data programmatically. ## Quick Start Guide 1. Register an [API Application](https://cloud.ouraring.com/oauth/applications) and implement OAuth2 2. **Make Your First API Call**:    ```    curl -X GET https://api.ouraring.com/v2/usercollection/personal_info \\    -H \"Authorization: Bearer YOUR_TOKEN_HERE\"    ``` 3. **Explore Data Types**:    - Browse the available endpoints in this documentation to discover what data you can access    - Each endpoint includes example requests and responses 4. **Set Up Webhooks (Strongly Recommended)**:    - Webhooks are the preferred way to consume Oura data    - We have not had customers hit rate limits with webhooks properly implemented    - Make a single request for historical data when a user first connects, then use webhooks for ongoing updates    - Webhook notifications come approximately 30 seconds after data syncs from the mobile app    - [Set up webhooks](#tag/Webhook-Subscription-Routes) to receive notifications when data changes ## Common Questions - **Data Delay**: Different data types sync at different times - sleep data requires users to open the Oura app, while daily activity and stress may sync in the background # Data Access In order to access data, a registered [API Application](https://cloud.ouraring.com/oauth/applications) is required.  API Applications are limited to **10** users before requiring approval from Oura. There is no limit once an application is approved.  Additionally, Oura users **must provide consent** to share each data type an API Application has access to. All data access requests through the Oura API require [Authentication](https://cloud.ouraring.com/docs/authentication). Additionally, we recommend that Oura users keep their mobile app updated to support API access for the latest data types. # Authentication The Oura Cloud API supports authentication through the industry-standard OAuth2 protocol. For more information, see our [Authentication instructions](https://cloud.ouraring.com/docs/authentication). Access tokens must be included in the request header as follows: ```http GET /v2/usercollection/personal_info HTTP/1.1 Host: api.ouraring.com Authorization: Bearer <token> ``` Please note that personal access tokens were deprecated in December 2025 and are no longer available for use. # Oura HTTP Response Codes | Response Code                        | Description | | ------------------------------------ | - | | 200 OK                               | Successful Response         | | 400 Query Parameter Validation Error | The request contains query parameters that are invalid or incorrectly formatted. | | 401 Unauthorized                     | Invalid or expired authentication token. | | 403 Forbidden                        | The requested resource requires additional permissions or the user's Oura subscription has expired. | | 429 Too Many Requests                | Rate limit exceeded. See response headers for retry guidance. |  ## Rate Limits The API enforces rate limits at two layers to ensure fair access across all applications: - a per-access-token limit, which throttles single-token floods, and - a per-application limit, which caps the aggregate traffic across all of an application's end-user tokens so one fan-out app can't dominate shared capacity.  A request that trips either layer receives a `429 Too Many Requests`. The `X-RateLimit-Tier` response header identifies which layer fired.  If your application regularly approaches rate limits, [webhooks](#tag/Webhook-Subscription-Routes) are strongly recommended — most applications that implement webhooks correctly do not encounter rate limit issues.  [Contact us](mailto:api-support@ouraring.com) if you expect your usage to require higher limits.  ## Rate Limit Response Headers When a `429 Too Many Requests` response is returned, five headers are included to guide retries. Prefer these over fixed-interval backoff: - **`Retry-After`** — integer seconds to wait before retrying. RFC 7231-compliant; safe to feed directly into your client's backoff logic. - **`X-RateLimit-Limit`** — the request ceiling for the current window. - **`X-RateLimit-Window`** — the rolling window length in seconds that the ceiling applies to. - **`X-RateLimit-Reset`** — Unix epoch (seconds) at which the window resets and quota is fully restored. - **`X-RateLimit-Tier`** — identifies which limit was exceeded, useful when contacting support. 
 *
 * The version of the OpenAPI document: 2.0
 * 
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

package com.ouratoolkit.api;

import java.io.IOException;
import java.time.Instant;
import java.time.OffsetDateTime;
import java.time.ZoneId;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.time.temporal.Temporal;
import java.time.temporal.TemporalAccessor;
import java.util.function.BiFunction;
import java.util.function.Function;

import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeFeature;
import com.fasterxml.jackson.datatype.jsr310.deser.InstantDeserializer;

@javax.annotation.Generated(value = "org.openapitools.codegen.languages.JavaClientCodegen", comments = "Generator version: 7.14.0")
public class RFC3339InstantDeserializer<T extends Temporal> extends InstantDeserializer<T> {
    private static final long serialVersionUID = 1L;
    private final static boolean DEFAULT_NORMALIZE_ZONE_ID = JavaTimeFeature.NORMALIZE_DESERIALIZED_ZONE_ID.enabledByDefault();
    private final static boolean DEFAULT_ALWAYS_ALLOW_STRINGIFIED_DATE_TIMESTAMPS
    = JavaTimeFeature.ALWAYS_ALLOW_STRINGIFIED_DATE_TIMESTAMPS.enabledByDefault();

    public static final RFC3339InstantDeserializer<Instant> INSTANT = new RFC3339InstantDeserializer<>(
        Instant.class, DateTimeFormatter.ISO_INSTANT,
        Instant::from,
        a -> Instant.ofEpochMilli( a.value ),
        a -> Instant.ofEpochSecond( a.integer, a.fraction ),
        null,
        true, // yes, replace zero offset with Z
        DEFAULT_NORMALIZE_ZONE_ID,
        DEFAULT_ALWAYS_ALLOW_STRINGIFIED_DATE_TIMESTAMPS
    );

    public static final RFC3339InstantDeserializer<OffsetDateTime> OFFSET_DATE_TIME = new RFC3339InstantDeserializer<>(
        OffsetDateTime.class, DateTimeFormatter.ISO_OFFSET_DATE_TIME,
        OffsetDateTime::from,
        a -> OffsetDateTime.ofInstant( Instant.ofEpochMilli( a.value ), a.zoneId ),
        a -> OffsetDateTime.ofInstant( Instant.ofEpochSecond( a.integer, a.fraction ), a.zoneId ),
        (d, z) -> ( d.isEqual( OffsetDateTime.MIN ) || d.isEqual( OffsetDateTime.MAX ) ?
        d :
        d.withOffsetSameInstant( z.getRules().getOffset( d.toLocalDateTime() ) ) ),
        true, // yes, replace zero offset with Z
        DEFAULT_NORMALIZE_ZONE_ID,
        DEFAULT_ALWAYS_ALLOW_STRINGIFIED_DATE_TIMESTAMPS
    );

    public static final RFC3339InstantDeserializer<ZonedDateTime> ZONED_DATE_TIME = new RFC3339InstantDeserializer<>(
        ZonedDateTime.class, DateTimeFormatter.ISO_ZONED_DATE_TIME,
        ZonedDateTime::from,
        a -> ZonedDateTime.ofInstant( Instant.ofEpochMilli( a.value ), a.zoneId ),
        a -> ZonedDateTime.ofInstant( Instant.ofEpochSecond( a.integer, a.fraction ), a.zoneId ),
        ZonedDateTime::withZoneSameInstant,
        false, // keep zero offset and Z separate since zones explicitly supported
        DEFAULT_NORMALIZE_ZONE_ID,
        DEFAULT_ALWAYS_ALLOW_STRINGIFIED_DATE_TIMESTAMPS
    );

    protected RFC3339InstantDeserializer(
            Class<T> supportedType,
            DateTimeFormatter formatter,
            Function<TemporalAccessor, T> parsedToValue,
            Function<FromIntegerArguments, T> fromMilliseconds,
            Function<FromDecimalArguments, T> fromNanoseconds,
            BiFunction<T, ZoneId, T> adjust,
            boolean replaceZeroOffsetAsZ,
            boolean normalizeZoneId,
            boolean readNumericStringsAsTimestamp) {
        super(
                supportedType,
                formatter,
                parsedToValue,
                fromMilliseconds,
                fromNanoseconds,
                adjust,
                replaceZeroOffsetAsZ,
                normalizeZoneId,
                readNumericStringsAsTimestamp
        );
    }

    @Override
    protected T _fromString(JsonParser p, DeserializationContext ctxt, String string0) throws IOException {
        return super._fromString(p, ctxt, string0.replace( ' ', 'T' ));
    }
}