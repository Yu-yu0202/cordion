/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The common structure shared by every Gateway payload.

use serde::{Deserialize, Serialize};

use super::{GatewayEventName, GatewayOpcode};

/// A Gateway payload.
///
/// `op` is always present; the other fields follow the opcode. `s` and `t` are
/// `null` when `op` is not [`GatewayOpcode::DISPATCH`], so they decode as
/// `None` there. `d` holds the event data, whose shape depends on the opcode and
/// the event name.
///
/// For the Heartbeat send payload, `d` is the last sequence number received, so
/// use `GatewayPayload<u64>` and read [`d`](Self::d) as the sequence number, or
/// `None` when the client has not received one yet. Events whose payload is
/// empty, such as Heartbeat Ack and Reconnect, send `d: null`.
///
/// # Examples
///
/// ```
/// use cordion_model::event::gateway::{GatewayOpcode, GatewayPayload};
///
/// let hello: GatewayPayload<serde_json::Value> =
///     serde_json::from_str(r#"{"op": 10, "d": {"heartbeat_interval": 45000}}"#)
///         .unwrap();
///
/// assert_eq!(hello.op, GatewayOpcode::HELLO);
/// assert_eq!(hello.s, None);
/// assert_eq!(hello.t, None);
/// ```
///
/// See [Discord's Gateway payload structure][payload] for details.
///
/// [payload]: https://docs.discord.com/developers/events/gateway-events#payload-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GatewayPayload<E> {
    /// The opcode that determines how the rest of the payload is interpreted.
    pub op: GatewayOpcode,

    /// The event data, or `None` when the event carries no data.
    ///
    /// The type is chosen by the caller: the event struct for dispatch payloads,
    /// `u64` for the Heartbeat send payload, or [`serde_json::Value`] to decode
    /// an event whose payload is not modeled.
    #[serde(default = "none")]
    pub d: Option<E>,

    /// The sequence number of a dispatched event, used for resuming and
    /// heartbeating.
    ///
    /// `None` when the payload is not a dispatch payload.
    #[serde(default)]
    pub s: Option<u64>,

    /// The name of the dispatched event.
    ///
    /// `None` when the payload is not a dispatch payload.
    #[serde(default)]
    pub t: Option<GatewayEventName>,
}

/// Returns the default value of a [`GatewayPayload`]'s event data.
///
/// A dedicated function keeps the deserialization of `GatewayPayload<E>` from
/// requiring `E: Default`.
const fn none<E>() -> Option<E> {
    None
}
