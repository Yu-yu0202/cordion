/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Numeric channel enumerations.
//!
//! Each type wraps the integer Discord sends and preserves unrecognized values
//! so that new channel variants do not make decoding fail.

use serde::{Deserialize, Serialize};

/// The type of a [`Channel`](super::Channel).
///
/// Unrecognized values are preserved. See [Discord's channel types][types] for
/// the current list.
///
/// [types]: https://docs.discord.com/developers/resources/channel#channel-object-channel-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ChannelType(u8);

impl ChannelType {
    /// A text channel within a guild.
    pub const GUILD_TEXT: Self = Self(0);

    /// A direct message between two users.
    pub const DM: Self = Self(1);

    /// A voice channel within a guild.
    pub const GUILD_VOICE: Self = Self(2);

    /// A direct message between multiple users.
    pub const GROUP_DM: Self = Self(3);

    /// An organizational category that contains up to 50 channels.
    pub const GUILD_CATEGORY: Self = Self(4);

    /// A channel whose posts can be followed and crossposted into other guilds.
    pub const GUILD_ANNOUNCEMENT: Self = Self(5);

    /// A temporary thread within a `GUILD_ANNOUNCEMENT` channel.
    pub const ANNOUNCEMENT_THREAD: Self = Self(10);

    /// A temporary thread within a `GUILD_TEXT` or `GUILD_FORUM` channel.
    pub const PUBLIC_THREAD: Self = Self(11);

    /// A temporary thread within a `GUILD_TEXT` channel that only invited
    /// members and members with `MANAGE_THREADS` can view.
    pub const PRIVATE_THREAD: Self = Self(12);

    /// A voice channel for hosting events with an audience.
    pub const GUILD_STAGE_VOICE: Self = Self(13);

    /// The channel in a student hub that lists its servers.
    pub const GUILD_DIRECTORY: Self = Self(14);

    /// A channel that can only contain threads.
    pub const GUILD_FORUM: Self = Self(15);

    /// A channel that can only contain threads, similar to
    /// [`GUILD_FORUM`](Self::GUILD_FORUM).
    ///
    /// Discord still describes this type as under active development.
    pub const GUILD_MEDIA: Self = Self(16);
}

/// The camera video quality mode of a voice or stage channel.
///
/// Unrecognized values are preserved. When Discord omits the field, the
/// effective mode is [`AUTO`](Self::AUTO). See [Discord's video quality
/// modes][modes] for details.
///
/// [modes]: https://docs.discord.com/developers/resources/channel#channel-object-video-quality-modes
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct VideoQualityMode(u8);

impl VideoQualityMode {
    /// Discord chooses the quality for optimal performance.
    pub const AUTO: Self = Self(1);

    /// 720p.
    pub const FULL: Self = Self(2);
}

/// The default sort order for posts in a forum or media channel.
///
/// Unrecognized values are preserved. Discord may send `null` when no sort
/// order has been set by a channel administrator. See [Discord's sort order
/// types][orders] for details.
///
/// [orders]: https://docs.discord.com/developers/resources/channel#channel-object-sort-order-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SortOrderMode(u8);

impl SortOrderMode {
    /// Sort posts by activity.
    pub const LATEST_ACTIVITY: Self = Self(0);

    /// Sort posts by creation time, most recent first.
    pub const CREATION_DATE: Self = Self(1);
}

/// The default layout used to display posts in a forum channel.
///
/// Unrecognized values are preserved. See [Discord's forum layout types][layouts]
/// for details.
///
/// [layouts]: https://docs.discord.com/developers/resources/channel#channel-object-forum-layout-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ForumLayout(u8);

impl ForumLayout {
    /// No default has been set for the forum channel.
    pub const NOT_SET: Self = Self(0);

    /// Display posts as a list.
    pub const LIST_VIEW: Self = Self(1);

    /// Display posts as a collection of tiles.
    pub const GALLERY_VIEW: Self = Self(2);
}
