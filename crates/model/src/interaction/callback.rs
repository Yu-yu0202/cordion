/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Interaction callback responses.
//!
//! `POST /interactions/{interaction.id}/{interaction.token}/callback` returns
//! `204 No Content` unless the `with_response` query parameter is `true`, in
//! which case it returns an [`InteractionCallbackResponse`]. These types model
//! that response, which is the receive side of the callback: the application
//! sends an [`InteractionResponse`](super::InteractionResponse) and Discord
//! reports what it created.
//!
//! See [Discord's interaction callback response object][response] for details.
//!
//! [response]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-callback-interaction-callback-response-object

use serde::{Deserialize, Serialize};

use crate::interaction::{InteractionCallbackType, InteractionId, InteractionType};
use crate::message::{Message, MessageId};

/// The response Discord returns for an interaction callback requested with
/// `with_response=true`.
///
/// See [Discord's interaction callback response object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-callback-interaction-callback-response-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct InteractionCallbackResponse {
    /// The interaction the response belongs to.
    pub interaction: InteractionCallbackObject,

    /// The resource the interaction response created.
    ///
    /// Present when the callback type creates a resource: a message for
    /// `CHANNEL_MESSAGE_WITH_SOURCE` and `UPDATE_MESSAGE`, and an activity
    /// instance for `LAUNCH_ACTIVITY`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<InteractionCallbackResource>,
}

/// The interaction associated with an [`InteractionCallbackResponse`].
///
/// Discord documents [`kind`](Self::kind) as an interaction type rather than an
/// interaction callback type, so it carries the type of the interaction that
/// was responded to. The optional fields describe the state of the response
/// message and are omitted during serialization when `None`.
///
/// See [Discord's interaction callback object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-callback-interaction-callback-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct InteractionCallbackObject {
    /// The interaction's ID.
    pub id: InteractionId,

    /// The interaction's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: InteractionType,

    /// The instance ID of the Activity that was launched or joined.
    ///
    /// Discord documents this as an opaque string rather than a snowflake; it
    /// matches [`InteractionCallbackActivityInstanceResource::id`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_instance_id: Option<String>,

    /// The ID of the message the interaction response created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_message_id: Option<MessageId>,

    /// Whether the response message is in a loading state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_message_loading: Option<bool>,

    /// Whether the response message is ephemeral.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_message_ephemeral: Option<bool>,
}

/// The resource an [`InteractionCallbackResponse`] reports as created.
///
/// [`activity_instance`](Self::activity_instance) is only present for the
/// `LAUNCH_ACTIVITY` callback type and [`message`](Self::message) only for
/// `CHANNEL_MESSAGE_WITH_SOURCE` and `UPDATE_MESSAGE`; both are omitted during
/// serialization when `None`.
///
/// See [Discord's interaction callback resource object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-callback-interaction-callback-resource-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct InteractionCallbackResource {
    /// The type of the callback that created the resource, serialized as
    /// `type`.
    #[serde(rename = "type")]
    pub kind: InteractionCallbackType,

    /// The Activity launched by this interaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_instance: Option<InteractionCallbackActivityInstanceResource>,

    /// The message created by this interaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
}

/// The Activity instance launched by an interaction.
///
/// See [Discord's interaction callback activity instance resource][resource] for
/// details.
///
/// [resource]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-callback-interaction-callback-activity-instance-resource
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InteractionCallbackActivityInstanceResource {
    /// The instance ID of the Activity that was launched or joined.
    ///
    /// Discord documents this as an opaque string rather than a snowflake.
    pub id: String,
}
