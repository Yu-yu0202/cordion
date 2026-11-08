/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Payloads of the Gateway connection endpoints.
//!
//! Both endpoints return the WSS URL an application connects to, and
//! `GET /gateway/bot` adds sharding and session start limit metadata. The
//! bot-scoped endpoint must not be cached for long, because its values change
//! as the application joins and leaves guilds.
//!
//! See [Discord's Gateway documentation][gateway] for details.
//!
//! [gateway]: https://docs.discord.com/developers/events/gateway

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// The response of `GET /gateway`.
///
/// The endpoint does not require authentication. See [Discord's get gateway
/// endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/events/gateway#get-gateway
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GetGatewayResponse {
    /// The WSS URL to open a Gateway connection on.
    pub url: String,
}

/// The response of `GET /gateway/bot`.
///
/// The endpoint requires a bot token. See [Discord's get gateway bot
/// endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/events/gateway#get-gateway-bot
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GetGatewayBotResponse {
    /// The WSS URL to open a Gateway connection on.
    pub url: String,

    /// The recommended number of shards to connect with.
    pub shards: u64,

    /// The application's session start limit.
    pub session_start_limit: SessionStartLimit,
}

/// The number of sessions an application may start, and how quickly.
///
/// [`reset_after`](Self::reset_after) is milliseconds on the wire and is exposed
/// as a [`Duration`] so that callers do not have to convert it themselves.
///
/// See [Discord's session start limit object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway#session-start-limit-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SessionStartLimit {
    /// The total number of session starts the application is allowed.
    pub total: u64,

    /// The number of session starts the application may still perform.
    pub remaining: u64,

    /// The time after which the limit resets.
    #[serde(with = "milliseconds")]
    pub reset_after: Duration,

    /// The number of identify requests allowed per 5 seconds.
    pub max_concurrency: u64,
}

/// Serde support for integer milliseconds sent as a [`Duration`].
mod milliseconds {
    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes `duration` as its total number of milliseconds.
    pub(super) fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    /// Deserializes a number of milliseconds into a [`Duration`].
    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let milliseconds = u64::deserialize(deserializer)?;

        Ok(Duration::from_millis(milliseconds))
    }
}
