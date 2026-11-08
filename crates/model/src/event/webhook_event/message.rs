/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Message-shaped data payloads carried by webhook events: the lobby message
//! objects of the `LOBBY_MESSAGE_*` events and the Social SDK direct message
//! objects of the `GAME_DIRECT_MESSAGE_*` events.
//!
//! See [Discord's webhook events documentation][events] for details.
//!
//! [events]: https://docs.discord.com/developers/events/webhook-events

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::application::{ApplicationId, PartialApplication};
use crate::channel::{Channel, ChannelId};
use crate::component::Component;
use crate::message::{Attachment, MessageActivity, MessageFlags, MessageId, MessageType};
use crate::social::lobby::{LobbyId, LobbyMessageMember, Metadata};
use crate::user::User;

/// The lobby message carried by the `LOBBY_MESSAGE_CREATE` and
/// `LOBBY_MESSAGE_UPDATE` webhook events.
///
/// Discord documents the inner payload as a lobby message object, with
/// `LOBBY_MESSAGE_UPDATE` adding fields for message updates. The webhook events
/// page marks [`application_id`](Self::application_id) as optional, while the
/// lobby resource page marks it required and
/// [`LobbyMessage`](crate::social::lobby::LobbyMessage) models it that way; the
/// documented `LOBBY_MESSAGE_UPDATE` payload omits it, so this type keeps it
/// optional instead of reusing that type.
///
/// [`edited_timestamp`](Self::edited_timestamp) and
/// [`timestamp`](Self::timestamp) are the update-only fields and are `None` for
/// a payload that does not include them.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct WebhookEventLobbyMessage {
    /// The message's ID.
    pub id: MessageId,

    /// The message's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: MessageType,

    /// The message content.
    pub content: String,

    /// The ID of the lobby the message was sent to.
    pub lobby_id: LobbyId,

    /// The ID the messages interface reports for the lobby.
    ///
    /// Equal to [`lobby_id`](Self::lobby_id).
    pub channel_id: LobbyId,

    /// The user that sent the message.
    pub author: User,

    /// The author's lobby member information, when the author had an additional
    /// display name set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lobby_member: Option<LobbyMessageMember>,

    /// Dispatch-only metadata sent with the message.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub metadata: Option<Metadata>,

    /// Moderation metadata set for the message.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub moderation_metadata: Option<Metadata>,

    /// The message flags.
    pub flags: MessageFlags,

    /// The ID of the application that sent the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    /// When the message was last edited.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::timestamp::option"
    )]
    pub edited_timestamp: Option<SystemTime>,

    /// When the message was sent.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::timestamp::option"
    )]
    pub timestamp: Option<SystemTime>,
}

/// The data of the `LOBBY_MESSAGE_DELETE` webhook event.
///
/// See [Discord's lobby message delete structure][structure] for details.
///
/// [structure]: https://docs.discord.com/developers/events/webhook-events#lobby-message-delete
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LobbyMessageDelete {
    /// The ID of the deleted message.
    pub id: MessageId,

    /// The ID of the lobby the message was deleted from.
    pub lobby_id: LobbyId,
}

/// The standard message object carried by the `GAME_DIRECT_MESSAGE_*` webhook
/// events.
///
/// Discord documents these payloads as a message object with the additional
/// [`lobby_id`](Self::lobby_id) and [`channel`](Self::channel) fields. The
/// documented examples do not all carry every field of a full message object,
/// so only the fields the documentation shows are modeled; the remaining
/// message fields are ignored during deserialization and the optional fields
/// are omitted during serialization when `None`.
///
/// See [Discord's message object][message] for the shared message fields.
///
/// [message]: https://docs.discord.com/developers/resources/message#message-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SocialSdkMessage {
    /// The message's ID.
    pub id: MessageId,

    /// The message's type, serialized as `type`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MessageType>,

    /// The message content.
    pub content: String,

    /// The ID of the channel the message was sent in.
    pub channel_id: ChannelId,

    /// The message's author.
    pub author: User,

    /// When the message was sent.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<SystemTime>,

    /// The channel the message was sent in, including its recipients.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<Channel>>,

    /// The ID of the application that sent the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    /// Files attached to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,

    /// Components attached to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// The message flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// The ID of the lobby the message was created in.
    ///
    /// Only present in Linked Channel messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lobby_id: Option<LobbyId>,
}

/// The SDK DM message object carried by the `GAME_DIRECT_MESSAGE_*` webhook
/// events.
///
/// Discord documents this shape for messages between provisional accounts,
/// which exist only in the game session. Unlike [`SocialSdkMessage`] it carries
/// no `channel_id` but always carries [`channel`](Self::channel),
/// [`application_id`](Self::application_id), and [`flags`](Self::flags). The
/// documented examples of the event payloads do not all carry every field, so
/// the message type is optional here; the remaining optional fields are omitted
/// during serialization when `None`.
///
/// See [Discord's SDK DM message object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/webhook-events#sdk-dm-message-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SocialSdkDirectMessage {
    /// The message's ID.
    pub id: MessageId,

    /// The message's type, serialized as `type`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MessageType>,

    /// The message content.
    pub content: String,

    /// The message's author.
    pub author: User,

    /// The message flags.
    pub flags: MessageFlags,

    /// The ID of the application that created the message.
    pub application_id: ApplicationId,

    /// The channel the message was sent in, including its recipients.
    pub channel: Box<Channel>,

    /// Rich Presence activity data sent with the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,

    /// The partial application sent with the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<PartialApplication>,
}
