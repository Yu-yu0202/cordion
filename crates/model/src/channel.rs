/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord channel resource.
//!
//! [`Channel`] models the full channel object returned by REST endpoints and
//! Gateway events. [`PartialChannel`] models the smaller channel shapes embedded
//! in invites, webhook source data, and guild widgets. Over the Gateway, guild
//! channels the current user cannot view may be sent with obfuscated metadata;
//! only `id`, `type`, `position`, and `parent_id` can be relied on in that case.
//! Check [`ChannelFlags::CHANNEL_OBFUSCATED`] rather than inspecting other
//! fields. The JSON bodies of the channel endpoints are modeled by
//! [`CreateGuildChannelRequest`], the [`ModifyGuildChannelRequest`] family,
//! [`EditChannelPermissionsRequest`], and the thread creation requests such as
//! [`StartThreadFromMessageRequest`].
//!
//! See [Discord's channel resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/channel

mod default_reaction;
mod flags;
mod followed_channel;
mod forum_tag;
mod kind;
mod modify;
mod overwrite;
mod partial;
mod thread;
mod thread_member;
mod thread_metadata;

use std::time::SystemTime;

use cordion_macro::IdTarget;
pub use default_reaction::DefaultReaction;
pub use flags::ChannelFlags;
pub use followed_channel::FollowedChannel;
pub use forum_tag::{ForumTag, ForumTagId};
pub use kind::{ChannelType, ForumLayout, SortOrderMode, VideoQualityMode};
pub use modify::{
    CreateGuildChannelRequest, EditChannelPermissionsRequest, ModifyGroupDmChannelRequest,
    ModifyGuildChannelRequest, ModifyThreadChannelRequest,
};
pub use overwrite::{Overwrite, OverwriteId, OverwriteTarget, OverwriteType};
pub use partial::PartialChannel;
use serde::{Deserialize, Serialize};
pub use thread::{
    ForumAndMediaThreadMessageParams, StartThreadFromMessageRequest,
    StartThreadInForumOrMediaThreadRequest, StartThreadWithoutMessageRequest,
};
pub use thread_member::ThreadMember;
pub use thread_metadata::ThreadMetadata;

use crate::application::ApplicationId;
use crate::guild::GuildId;
use crate::id::{Id, Identifiable};
use crate::message::MessageId;
use crate::permission::Permission;
use crate::user::{User, UserId};

/// The ID of a [`Channel`].
pub type ChannelId = Id<Channel>;

/// A guild channel, direct message, group direct message, or thread.
///
/// Only `id` and `type` are guaranteed to be present; Discord marks the other
/// fields as optional, and obfuscated Gateway channels may populate them with
/// values that must not be trusted. `None` means the field was absent or
/// `null`, and optional fields are omitted during serialization.
///
/// See [Discord's channel object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#channel-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Channel {
    /// The channel's ID.
    pub id: ChannelId,

    /// The channel's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ChannelType,

    /// The guild this channel belongs to.
    ///
    /// Discord may omit this in channel objects received through Gateway guild
    /// dispatches even when the channel is a guild channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The channel's sorting position.
    ///
    /// Channels with the same position are sorted by ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    /// Explicit permission overwrites for roles and members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_overwrites: Option<Vec<Overwrite>>,

    /// The channel's name, between 1 and 100 characters.
    ///
    /// Discord may send `null` for channels without a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The channel's topic.
    ///
    /// Up to 4096 characters for `GUILD_FORUM` and `GUILD_MEDIA` channels, and
    /// up to 1024 characters otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    /// Whether the channel is age-restricted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// The ID of the last message sent in this channel or thread.
    ///
    /// Discord warns that this may not point to an existing or valid message or
    /// thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,

    /// The voice channel's bitrate in bits per second.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u64>,

    /// The voice channel's user limit.
    ///
    /// `0` means no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u64>,

    /// Seconds a user must wait before sending another message.
    ///
    /// Valid values are 0 through 21600. Bots and users with
    /// [`Permission::BYPASS_SLOWMODE`] are unaffected. This also limits how
    /// often a user can create a thread in the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u64>,

    /// The recipients of a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<User>>,

    /// The icon hash of a group direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The ID of the creator of a group direct message or thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<UserId>,

    /// The application associated with the channel.
    ///
    /// For group direct messages, this is the application that created the
    /// group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    /// Whether a group direct message is managed by an application through the
    /// `gdm.join` `OAuth2` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,

    /// The parent category of a guild channel, or the parent text channel of a
    /// thread.
    ///
    /// A category can contain up to 50 child channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,

    /// When the most recent pinned message was pinned.
    ///
    /// Discord may send `null` in events such as [`GUILD_CREATE`] when no
    /// message is pinned.
    ///
    /// [`GUILD_CREATE`]: crate::event::gateway::GuildCreate
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_pin_timestamp: Option<SystemTime>,

    /// The voice region ID for a voice or stage channel.
    ///
    /// `None` means automatic region selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<String>,

    /// The camera video quality mode of a voice or stage channel.
    ///
    /// When absent, Discord treats the mode as [`VideoQualityMode::AUTO`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,

    /// The number of messages in a thread, excluding the initial message and
    /// deleted messages.
    ///
    /// Discord notes that this is inaccurate above 50 for threads created
    /// before July 1, 2022.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<u64>,

    /// An approximate number of users in a thread, capped at 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,

    /// Thread-specific metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,

    /// The thread member object for the current user, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ThreadMember>,

    /// The default auto-archive duration, in minutes, for newly created threads.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<u16>,

    /// Permissions for the invoking user in this channel, including overwrites.
    ///
    /// Only included in the `resolved` data of an interaction. It does not
    /// include implicit permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,

    /// Permissions for the bot user in this channel, including overwrites.
    ///
    /// Only included in the `resolved` data of an interaction. It does not
    /// include implicit permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_permissions: Option<Permission>,

    /// Channel flags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    /// The number of messages ever sent in a thread.
    ///
    /// Unlike [`message_count`](Self::message_count), this does not decrease
    /// when messages are deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_message_sent: Option<u64>,

    /// The tags available in a forum or media channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<ForumTag>>,

    /// The IDs of tags applied to this forum or media thread.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<ForumTagId>>,

    /// The default reaction emoji for new forum or media threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<DefaultReaction>,

    /// The initial `rate_limit_per_user` copied to newly created threads.
    ///
    /// This is copied at thread creation and does not update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u64>,

    /// The default sort order for posts in a forum or media channel.
    ///
    /// `None` means no sort order has been set by a channel administrator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<SortOrderMode>,

    /// The default forum layout for a forum channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayout>,
}

impl Identifiable for Channel {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}
