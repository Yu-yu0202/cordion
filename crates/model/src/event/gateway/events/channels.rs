/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Channel-related Gateway events.
//!
//! Several channel events carry an existing model unchanged, so no dedicated
//! event type exists for them:
//!
//! | Event | Payload |
//! | - | - |
//! | `CHANNEL_CREATE` | [`Channel`] |
//! | `CHANNEL_UPDATE` | [`Channel`] |
//! | `CHANNEL_DELETE` | [`Channel`] |
//! | `THREAD_CREATE` | [`Channel`] |
//! | `THREAD_UPDATE` | [`Channel`] |
//! | `THREAD_DELETE` | [`Channel`] |
//! | `WEBHOOKS_UPDATE` | [`WebhooksUpdate`] |
//!
//! Discord documents that `THREAD_CREATE` carries an extra `newly_created`
//! boolean when a thread is created and an extra thread member object when the
//! current user is added to an existing private thread. Neither extra is
//! documented as a field of the event, so they are not modeled here; use
//! [`serde_json::Value`] to read them.
//!
//! `THREAD_DELETE` carries only `id`, `guild_id`, `parent_id`, and `type`, all
//! of which [`Channel`] already models. `THREAD_MEMBER_UPDATE` carries a thread
//! member object with an extra `guild_id` field and is modeled by
//! [`ThreadMemberUpdate`].
//!
//! [`Channel`]: crate::channel::Channel

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::application::PartialApplication;
use crate::channel::{Channel, ChannelId, ThreadMember};
use crate::guild::GuildId;
use crate::invite::InviteTargetType;
use crate::role::RoleId;
use crate::user::{User, UserId};

/// The payload of the `CHANNEL_INFO` event.
///
/// See [Discord's Channel Info structure][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#channel-info
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ChannelInfo {
    /// The guild the ephemeral channel data belongs to.
    pub guild_id: GuildId,

    /// The ephemeral data for the guild's channels.
    pub channels: Vec<ChannelInfoChannel>,
}

/// Ephemeral data for one channel in a [`ChannelInfo`] payload.
///
/// Both fields are optional and nullable: Discord sends `null` for the data it
/// does not include and omits the field entirely, so `None` is serialized as
/// `null`.
///
/// See [Discord's channel info channel structure][structure] for details.
///
/// [structure]: https://docs.discord.com/developers/events/gateway-events#channel-info-channel-info-channel-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ChannelInfoChannel {
    /// The channel's ID.
    pub id: ChannelId,

    /// The voice channel's status.
    #[serde(default)]
    pub status: Option<String>,

    /// When the voice session started, as Unix seconds.
    #[serde(default)]
    pub voice_start_time: Option<u64>,
}

/// The payload of the `VOICE_CHANNEL_STATUS_UPDATE` event.
///
/// See [Discord's Voice Channel Status Update][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#voice-channel-status-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct VoiceChannelStatusUpdate {
    /// The channel's ID.
    pub id: ChannelId,

    /// The guild's ID.
    pub guild_id: GuildId,

    /// The new voice channel status.
    ///
    /// Nullable, so `None` is serialized as `null`.
    #[serde(default)]
    pub status: Option<String>,
}

/// The payload of the `VOICE_CHANNEL_START_TIME_UPDATE` event.
///
/// See [Discord's Voice Channel Start Time Update][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#voice-channel-start-time-update
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct VoiceChannelStartTimeUpdate {
    /// The channel's ID.
    pub id: ChannelId,

    /// The guild's ID.
    pub guild_id: GuildId,

    /// When the voice session started, as Unix seconds.
    ///
    /// Nullable, so `None` is serialized as `null`.
    #[serde(default)]
    pub voice_start_time: Option<u64>,
}

/// The payload of the `THREAD_LIST_SYNC` event.
///
/// Sent when the current user gains access to a channel. The event is not tied
/// to a single thread, so it carries the whole thread list of the given parent
/// channels.
///
/// See [Discord's Thread List Sync event fields][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#thread-list-sync
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ThreadListSync {
    /// The guild the threads belong to.
    pub guild_id: GuildId,

    /// The parent channels whose threads are being synced.
    ///
    /// Omitted when the threads were synced for the entire guild. The array may
    /// list channels that have no active threads, so the client can clear their
    /// cached threads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<ChannelId>>,

    /// All active threads the current user can access.
    pub threads: Vec<Channel>,

    /// The thread members of the current user in the synced threads.
    pub members: Vec<ThreadMember>,
}

/// The payload of the `THREAD_MEMBERS_UPDATE` event.
///
/// Sent when users are added to or removed from a thread. Discord also includes
/// a guild member and a nullable presence object for each added thread member;
/// those are not part of [`ThreadMember`] and are not retained here.
///
/// See [Discord's Thread Members Update event fields][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#thread-members-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ThreadMembersUpdate {
    /// The thread's ID.
    pub id: ChannelId,

    /// The guild the thread belongs to.
    pub guild_id: GuildId,

    /// The approximate number of members in the thread, capped at 50.
    pub member_count: u64,

    /// The users who were added to the thread.
    ///
    /// Omitted when no user was added.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_members: Option<Vec<ThreadMember>>,

    /// The IDs of the users who were removed from the thread.
    ///
    /// Omitted when no user was removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed_member_ids: Option<Vec<UserId>>,
}

/// The payload of the `THREAD_MEMBER_UPDATE` event.
///
/// Sent when the current user's thread member object is updated. The payload is
/// a [`ThreadMember`] with the extra `guild_id` field documented for the event,
/// so the thread member fields are flattened into this type.
///
/// See [Discord's Thread Member Update event fields][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#thread-member-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ThreadMemberUpdate {
    /// The thread member object of the current user.
    #[serde(flatten)]
    pub member: ThreadMember,

    /// The ID of the guild the thread belongs to.
    pub guild_id: GuildId,
}

/// The payload of the `CHANNEL_PINS_UPDATE` event.
///
/// Sent when a message is pinned or unpinned, but not when a pinned message is
/// deleted.
///
/// See [Discord's Channel Pins Update event fields][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#channel-pins-update
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ChannelPinsUpdate {
    /// The guild's ID, when the channel belongs to a guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The channel's ID.
    pub channel_id: ChannelId,

    /// When the most recent pinned message was pinned.
    ///
    /// Discord sends `null` when no message is pinned; nullable, so `None` is
    /// serialized as `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub last_pin_timestamp: Option<SystemTime>,
}

/// The payload of the `INVITE_CREATE` event.
///
/// Unlike the invite resource, this event always carries the invite's usage and
/// lifetime fields. [`expires_at`](Self::expires_at) is nullable, so `None` is
/// serialized as `null`; the remaining optional fields are omitted when `None`.
///
/// See [Discord's Invite Create event fields][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#invite-create
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InviteCreate {
    /// The channel the invite is for.
    pub channel_id: ChannelId,

    /// The unique invite code.
    pub code: String,

    /// When the invite was created.
    #[serde(with = "crate::timestamp")]
    pub created_at: SystemTime,

    /// The guild of the invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The user who created the invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,

    /// How long the invite is valid for, in seconds.
    pub max_age: u64,

    /// The maximum number of times the invite can be used.
    pub max_uses: u64,

    /// The type of target for a voice channel invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,

    /// The user whose stream is displayed for a stream invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,

    /// The embedded application opened by an embedded application invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_application: Option<PartialApplication>,

    /// Whether the invite grants temporary membership.
    pub temporary: bool,

    /// How many times the invite has been used.
    ///
    /// Discord always reports `0` for this event.
    pub uses: u64,

    /// When the invite expires.
    ///
    /// Nullable, so `None` is serialized as `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub expires_at: Option<SystemTime>,

    /// The roles granted to users who accept the invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<RoleId>>,
}

/// The payload of the `INVITE_DELETE` event.
///
/// See [Discord's Invite Delete event fields][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#invite-delete
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InviteDelete {
    /// The channel of the invite.
    pub channel_id: ChannelId,

    /// The guild of the invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The unique invite code.
    pub code: String,
}

/// The payload of the `WEBHOOKS_UPDATE` event.
///
/// Sent when a guild channel's webhook is created, updated, or deleted. The
/// event does not identify the webhook, so a client has to refetch the
/// channel's webhooks to find out what changed.
///
/// See [Discord's Webhooks Update event fields][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#webhooks-update
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct WebhooksUpdate {
    /// The ID of the guild the webhook belongs to.
    pub guild_id: GuildId,

    /// The ID of the channel the webhook belongs to.
    pub channel_id: ChannelId,
}
