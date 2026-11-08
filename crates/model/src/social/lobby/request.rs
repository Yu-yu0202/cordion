/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Request payloads for the lobby endpoints.
//!
//! Their shapes differ from the returned objects, so they are modeled as
//! separate types from the objects in the sibling `message` module.
//!
//! See [Discord's lobby resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/lobby

use serde::{Deserialize, Serialize};

use super::{LobbyMemberFlags, Metadata};
use crate::channel::ChannelId;
use crate::message::MessageFlags;
use crate::user::UserId;

/// The body of `POST /lobbies`.
///
/// See [Discord's create lobby endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/lobby#create-lobby
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateLobbyRequest {
    /// The lobby metadata to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The members to add to the lobby.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<LobbyMemberRequest>>,

    /// The seconds to wait before shutting down the lobby once it is idle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<u64>,
}

/// The body of `PUT /lobbies`.
///
/// See [Discord's create or join lobby endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/lobby#create-or-join-lobby
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateOrJoinLobbyRequest {
    /// The secret identifying the lobby.
    pub secret: String,

    /// The seconds to wait before shutting down the lobby once it is idle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<u64>,

    /// The lobby metadata to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lobby_metadata: Option<Metadata>,

    /// The calling member's metadata to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_metadata: Option<Metadata>,
}

/// The body of `PATCH /lobbies/{lobby.id}`.
///
/// See [Discord's modify lobby endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/lobby#modify-lobby
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyLobbyRequest {
    /// The lobby metadata to write.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The members to replace the lobby members with.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<LobbyMemberRequest>>,

    /// The seconds to wait before shutting down the lobby once it is idle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<u64>,
}

/// A member entry in a lobby request body.
///
/// Omitting [`additional_name`](Self::additional_name) preserves the member's
/// current value, while an explicit `null` clears it. Because `None` serializes
/// as `null`, use a wrapper that skips the field when the caller wants to
/// preserve the value.
///
/// See [Discord's lobby member JSON params][params] for details.
///
/// [params]: https://docs.discord.com/developers/resources/lobby#lobby-member-json-params
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LobbyMemberRequest {
    /// The user to add to the lobby.
    pub id: UserId,

    /// The member's metadata to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The member's lobby flags to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<LobbyMemberFlags>,

    /// The member's additional display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
}

/// A member entry in the body of `POST /lobbies/{lobby.id}/members/bulk`.
///
/// See [Discord's bulk update lobby members endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/lobby#bulk-update-lobby-members
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BulkLobbyMemberRequest {
    /// The user to add, update, or remove.
    pub id: UserId,

    /// The member's metadata to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The member's lobby flags to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<LobbyMemberFlags>,

    /// The member's additional display name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,

    /// Whether to remove the member instead of upserting them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_member: Option<bool>,
}

/// The body of `PUT /lobbies/{lobby.id}/members/{user.id}`.
///
/// See [Discord's add a member to a lobby endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/lobby#add-a-member-to-a-lobby
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UpsertLobbyMemberRequest {
    /// The member's metadata to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The member's lobby flags to set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<LobbyMemberFlags>,

    /// The member's additional display name.
    ///
    /// Omit the field to preserve the current value or send `null` to clear
    /// it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
}

/// The body of `PATCH /lobbies/{lobby.id}/channel-linking`.
///
/// An empty body unlinks the lobby; sending a request without
/// [`channel_id`](Self::channel_id) achieves the same result.
///
/// See [Discord's link channel to lobby endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/lobby#link-channel-to-lobby
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LinkLobbyChannelRequest {
    /// The channel to link, or `None` to unlink the lobby.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
}

/// The body of `POST /lobbies/{lobby.id}/messages`.
///
/// See [Discord's send lobby message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/lobby#send-lobby-message
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SendLobbyMessageRequest {
    /// The message content, which must be non-empty.
    pub content: String,

    /// Dispatch-only metadata delivered alongside the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The message flags to combine.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
}
