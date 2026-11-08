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

use crate::channel::ChannelId;
use crate::guild::GuildMember;
use crate::user::UserId;

/// A user that has joined a thread.
///
/// `id` and `user_id` are omitted in the thread members included in
/// [`GUILD_CREATE`](crate::event::gateway::GuildCreate), because the thread and user
/// are already known from context. `member` is only included by endpoints that
/// request member details.
///
/// See [Discord's thread member object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#thread-member-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ThreadMember {
    /// The thread's ID, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ChannelId>,

    /// The user's ID, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    /// When the user most recently joined the thread.
    #[serde(with = "crate::timestamp")]
    pub join_timestamp: SystemTime,

    /// User-specific thread settings.
    ///
    /// Discord does not document individual bits; unrecognized bits are
    /// preserved because this field is stored as a raw integer.
    pub flags: u64,

    /// Additional guild member information, when requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
}
