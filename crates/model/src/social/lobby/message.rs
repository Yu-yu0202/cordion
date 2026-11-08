/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Lobby message, lobby member, and lobby invite objects returned by the lobby
//! endpoints.
//!
//! See [Discord's lobby resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/lobby

use serde::{Deserialize, Serialize};

use super::{LobbyId, Metadata};
use crate::application::ApplicationId;
use crate::message::{MessageFlags, MessageId, MessageType};
use crate::user::User;

/// A message sent to a [`Lobby`](super::Lobby).
///
/// [`lobby_member`](Self::lobby_member) carries the author's additional display
/// name captured at send time. `metadata` is dispatch-only and is not
/// persisted on the linked channel message.
///
/// See [Discord's lobby message object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/lobby#lobby-message-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LobbyMessage {
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
    /// Included for compatibility with the messages interface and equal to
    /// [`lobby_id`](Self::lobby_id).
    pub channel_id: LobbyId,

    /// The user that sent the message.
    pub author: User,

    /// The author's lobby member information, when the author had an additional
    /// display name set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lobby_member: Option<LobbyMessageMember>,

    /// Dispatch-only metadata delivered to active clients.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub metadata: Option<Metadata>,

    /// Moderation metadata set for this message.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub moderation_metadata: Option<Metadata>,

    /// The message flags.
    pub flags: MessageFlags,

    /// The ID of the application that sent the message.
    pub application_id: ApplicationId,
}

/// The lobby member information attached to a [`LobbyMessage`].
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LobbyMessageMember {
    /// The author's additional display name when the message was sent.
    pub additional_name: String,
}

/// A single-use invite to a lobby's linked channel.
///
/// See [Discord's lobby invite object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/lobby#lobby-invite-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LobbyInvite {
    /// The invite code for the lobby's linked channel.
    pub code: String,
}
