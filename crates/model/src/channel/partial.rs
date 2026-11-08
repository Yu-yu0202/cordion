/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::channel::{ChannelFlags, ChannelId, ChannelType, ThreadMetadata};
use crate::guild::GuildId;
use crate::message::MessageId;
use crate::permission::Permission;

/// A partial channel embedded in another resource.
///
/// Discord reuses this shape in invite objects, webhook `source_channel`
/// objects, guild widgets, interaction payloads, and resolved interaction data.
/// Only [`id`](Self::id) is guaranteed to be present; the remaining fields and
/// their availability depend on the enclosing resource. Absent optional fields
/// are omitted during serialization.
///
/// Resolved interaction channels document a subset that includes
/// [`id`](Self::id), [`name`](Self::name), [`kind`](Self::kind),
/// [`permissions`](Self::permissions), [`app_permissions`](Self::app_permissions),
/// [`last_message_id`](Self::last_message_id),
/// [`last_pin_timestamp`](Self::last_pin_timestamp), [`nsfw`](Self::nsfw),
/// [`parent_id`](Self::parent_id), [`guild_id`](Self::guild_id),
/// [`flags`](Self::flags), [`rate_limit_per_user`](Self::rate_limit_per_user),
/// [`topic`](Self::topic), and [`position`](Self::position); threads also carry
/// [`thread_metadata`](Self::thread_metadata). Payloads may still contain more
/// fields, which are ignored.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialChannel {
    /// The channel's ID.
    pub id: ChannelId,

    /// The channel's name, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The channel's type, serialized as `type`, when included.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<ChannelType>,

    /// The channel's sorting position, when included.
    ///
    /// Channels with the same position are sorted by ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    /// The ID of the guild containing the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The permissions of the current user in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,

    /// The permissions of the current application in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_permissions: Option<Permission>,

    /// The ID of the last message sent in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,

    /// When the last message was pinned.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_pin_timestamp: Option<SystemTime>,

    /// Whether the channel is age-restricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// The ID of the channel's parent category or thread parent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,

    /// The channel's flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    /// The slowmode delay in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,

    /// The channel's topic, when it has one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    /// Thread-specific metadata for thread channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
}
