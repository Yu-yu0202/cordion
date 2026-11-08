/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Request bodies for creating and modifying channels.
//!
//! [`CreateGuildChannelRequest`] is the body of `POST
//! /guilds/{guild.id}/channels`. [`ModifyGuildChannelRequest`],
//! [`ModifyGroupDmChannelRequest`], and [`ModifyThreadChannelRequest`] are the
//! three parameter tables of `PATCH /channels/{channel.id}`, and
//! [`EditChannelPermissionsRequest`] is the body of `PUT
//! /channels/{channel.id}/permissions/{overwrite.id}`.
//!
//! Two `Option` conventions appear in these types. A field Discord documents as
//! *nullable* serializes as `null` when it is `None`, which Discord reads as
//! "reset this to the default". A field that is only *optional* is omitted when
//! it is `None`, leaving the current value untouched. Each field states which
//! convention applies.
//!
//! See [Discord's channel resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/channel

use serde::{Deserialize, Serialize};

use crate::channel::{
    ChannelFlags, ChannelId, ChannelType, DefaultReaction, ForumLayout, ForumTag, ForumTagId,
    Overwrite, OverwriteType, SortOrderMode, VideoQualityMode,
};
use crate::permission::Permission;

/// The body of `POST /guilds/{guild.id}/channels`.
///
/// Discord documents every parameter as optional and nullable except
/// [`name`](Self::name). Every other field is therefore serialized as `null`
/// when it is `None`, which Discord reads as "use the default"; a request that
/// only names a new channel carries the remaining fields as explicit `null`s.
///
/// See [Discord's create guild channel endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#create-guild-channel
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGuildChannelRequest {
    /// The channel's name, between 1 and 100 characters.
    ///
    /// This is the only field Discord does not document as nullable.
    pub name: String,

    /// The channel's type, serialized as `type`.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(rename = "type", default)]
    pub kind: Option<ChannelType>,

    /// The channel's topic.
    ///
    /// Up to 1024 characters, or up to 4096 for `GUILD_FORUM` and `GUILD_MEDIA`
    /// channels. Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub topic: Option<String>,

    /// The bitrate of a voice or stage channel in bits per second, at least
    /// 8000.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub bitrate: Option<u64>,

    /// The user limit of a voice or stage channel, where `0` means no limit.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub user_limit: Option<u64>,

    /// Seconds a user must wait before sending another message, from 0 through
    /// 21600.
    ///
    /// Bots and users with [`Permission::BYPASS_SLOWMODE`] are unaffected.
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub rate_limit_per_user: Option<u64>,

    /// The channel's sorting position, where channels sharing a position are
    /// sorted by ID.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub position: Option<i32>,

    /// The channel's permission overwrites for roles and members.
    ///
    /// Discord accepts partial overwrite objects whose `allow` and `deny` keys
    /// may be omitted; this type always sends both, and Discord reads an omitted
    /// or `null` value as `"0"`. Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub permission_overwrites: Option<Vec<Overwrite>>,

    /// The ID of the parent category of the channel.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub parent_id: Option<ChannelId>,

    /// Whether the channel is age-restricted.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub nsfw: Option<bool>,

    /// The voice region ID of a voice or stage channel.
    ///
    /// Discord selects the region automatically when this is `null`. Nullable:
    /// `None` is serialized as `null`.
    #[serde(default)]
    pub rtc_region: Option<String>,

    /// The camera video quality mode of a voice or stage channel.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub video_quality_mode: Option<VideoQualityMode>,

    /// The default auto-archive duration, in minutes, that clients use for new
    /// threads in the channel.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`. Nullable: `None` is
    /// serialized as `null`.
    #[serde(default)]
    pub default_auto_archive_duration: Option<u16>,

    /// The emoji shown in the add-reaction button of a new `GUILD_FORUM` or
    /// `GUILD_MEDIA` thread.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub default_reaction_emoji: Option<DefaultReaction>,

    /// The tags available in a `GUILD_FORUM` or `GUILD_MEDIA` channel, up to 20.
    ///
    /// Discord only requires [`ForumTag::name`] for each tag, so the other
    /// fields are sent as the application provides them. Nullable: `None` is
    /// serialized as `null`.
    #[serde(default)]
    pub available_tags: Option<Vec<ForumTag>>,

    /// The default sort order of posts in a `GUILD_FORUM` or `GUILD_MEDIA`
    /// channel.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub default_sort_order: Option<SortOrderMode>,

    /// The default forum layout of a `GUILD_FORUM` channel.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub default_forum_layout: Option<ForumLayout>,

    /// The initial `rate_limit_per_user` copied to new threads in the channel.
    ///
    /// Discord copies this value when a thread is created and does not update it
    /// afterwards. Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub default_thread_rate_limit_per_user: Option<u64>,

    /// The channel's flags.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub flags: Option<ChannelFlags>,
}

/// The guild-channel parameter table of `PATCH /channels/{channel.id}`.
///
/// Discord documents every parameter of the endpoint as optional. A field
/// documented as nullable serializes as `null` when `None`, which resets that
/// setting; every other field is omitted when `None`, which leaves the current
/// value in place.
///
/// See [Discord's modify channel endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#modify-channel
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildChannelRequest {
    /// The channel's name, between 1 and 100 characters.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The channel's type, serialized as `type`.
    ///
    /// Discord only supports converting between `GUILD_TEXT` and
    /// `GUILD_ANNOUNCEMENT`, and only in guilds with the `NEWS` feature.
    /// Optional; omitted from the request when `None`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ChannelType>,

    /// The channel's sorting position, where channels sharing a position are
    /// sorted by ID.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub position: Option<i32>,

    /// The channel's topic.
    ///
    /// Up to 1024 characters, or up to 4096 for `GUILD_FORUM` and `GUILD_MEDIA`
    /// channels. Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub topic: Option<String>,

    /// Whether the channel is age-restricted.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub nsfw: Option<bool>,

    /// Seconds a user must wait before sending another message, from 0 through
    /// 21600.
    ///
    /// Bots and users with [`Permission::BYPASS_SLOWMODE`] are unaffected.
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub rate_limit_per_user: Option<u64>,

    /// The bitrate of a voice or stage channel in bits per second, at least
    /// 8000.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub bitrate: Option<u64>,

    /// The user limit of a voice or stage channel, where `0` means no limit.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub user_limit: Option<u64>,

    /// The channel's permission overwrites for roles and members.
    ///
    /// Discord accepts partial overwrite objects whose `allow` and `deny` keys
    /// may be omitted; this type always sends both, and Discord reads an omitted
    /// or `null` value as `"0"`. Modifying overwrites requires the
    /// `MANAGE_ROLES` permission. Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub permission_overwrites: Option<Vec<Overwrite>>,

    /// The ID of the new parent category of the channel.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub parent_id: Option<ChannelId>,

    /// The voice region ID of a voice or stage channel.
    ///
    /// Discord selects the region automatically when this is `null`. Nullable:
    /// `None` is serialized as `null`.
    #[serde(default)]
    pub rtc_region: Option<String>,

    /// The camera video quality mode of a voice or stage channel.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub video_quality_mode: Option<VideoQualityMode>,

    /// The default auto-archive duration, in minutes, that clients use for new
    /// threads in the channel.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`. Nullable: `None` is
    /// serialized as `null`.
    #[serde(default)]
    pub default_auto_archive_duration: Option<u16>,

    /// The channel's flags.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    /// The tags usable in a `GUILD_FORUM` or `GUILD_MEDIA` channel, up to 20.
    ///
    /// Discord only requires [`ForumTag::name`] for each tag, so the other
    /// fields are sent as the application provides them. Optional; omitted from
    /// the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<ForumTag>>,

    /// The emoji shown in the add-reaction button of a new `GUILD_FORUM` or
    /// `GUILD_MEDIA` thread.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub default_reaction_emoji: Option<DefaultReaction>,

    /// The initial `rate_limit_per_user` copied to new threads in the channel.
    ///
    /// Discord copies this value when a thread is created and does not update it
    /// afterwards. Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u64>,

    /// The default sort order of posts in a `GUILD_FORUM` or `GUILD_MEDIA`
    /// channel.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub default_sort_order: Option<SortOrderMode>,

    /// The default forum layout of a `GUILD_FORUM` channel.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayout>,
}

/// The group-DM parameter table of `PATCH /channels/{channel.id}`.
///
/// See [Discord's modify channel endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#modify-channel
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGroupDmChannelRequest {
    /// The channel's name, between 1 and 100 characters.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The base64-encoded group DM icon.
    ///
    /// The data follows Discord's [image data][image] format. Optional; omitted
    /// from the request when `None`.
    ///
    /// [image]: https://docs.discord.com/developers/reference#image-data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

/// The thread parameter table of `PATCH /channels/{channel.id}`.
///
/// Unarchiving a thread requires the `MANAGE_THREADS` permission unless the
/// request also sets [`locked`](Self::locked) to `false`.
///
/// See [Discord's modify channel endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#modify-channel
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyThreadChannelRequest {
    /// The thread's name, between 1 and 100 characters.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether the thread is archived.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    /// Minutes of inactivity before the thread stops appearing in the channel
    /// list.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`. Optional; omitted from
    /// the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<u16>,

    /// Whether the thread is locked.
    ///
    /// Only users with [`Permission::MANAGE_THREADS`] can unarchive a locked
    /// thread. Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    /// Whether non-moderators can add other non-moderators to the thread.
    ///
    /// Only meaningful for private threads. Optional; omitted from the request
    /// when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,

    /// Seconds a user must wait before sending another message, from 0 through
    /// 21600.
    ///
    /// Bots and users with [`Permission::BYPASS_SLOWMODE`] are unaffected.
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub rate_limit_per_user: Option<u64>,

    /// The thread's flags.
    ///
    /// `PINNED` can only be set on threads in forum and media channels.
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    /// The IDs of the tags applied to the thread, up to five.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<ForumTagId>>,
}

/// The body of `PUT /channels/{channel.id}/permissions/{overwrite.id}`.
///
/// Only usable on guild channels, and requires the `MANAGE_ROLES` permission.
/// Discord defaults an omitted or `null` `allow` or `deny` to `"0"`, so both
/// fields are nullable and serialize as `null` when `None`.
///
/// See [Discord's edit channel permissions endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#edit-channel-permissions
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EditChannelPermissionsRequest {
    /// The permission bits explicitly allowed, defaulting to `"0"`.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub allow: Option<Permission>,

    /// The permission bits explicitly denied, defaulting to `"0"`.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub deny: Option<Permission>,

    /// Whether the overwrite applies to a role or a guild member, serialized as
    /// `type`.
    #[serde(rename = "type")]
    pub kind: OverwriteType,
}
