/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::channel::{ChannelId, PartialChannel};
use crate::event::presence::UserStatus;
use crate::guild::GuildId;

/// The widget settings of a guild.
///
/// See [Discord's guild widget settings object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#guild-widget-settings-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildWidgetSettings {
    /// Whether the widget is enabled.
    pub enabled: bool,

    /// The widget channel's ID, or `None` if no channel is selected.
    ///
    /// Discord sends this field as `null` when unset, so it is serialized as
    /// `null` when `None`.
    pub channel_id: Option<ChannelId>,
}

/// The public widget of a guild.
///
/// The member objects are widget-specific: Discord anonymizes their `id`,
/// `discriminator`, and `avatar` fields. Use [`GuildWidgetMember`] rather than
/// [`User`](crate::user::User) for these entries.
///
/// See [Discord's guild widget object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#guild-widget-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildWidget {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's name.
    pub name: String,

    /// An instant invite for the widget's invite channel, or `None` if no
    /// invite is available.
    pub instant_invite: Option<String>,

    /// Voice and stage channels accessible by `@everyone`.
    pub channels: Vec<PartialChannel>,

    /// Users currently online, up to 100.
    pub members: Vec<GuildWidgetMember>,

    /// The number of online members in the guild.
    pub presence_count: u64,
}

/// One anonymized member entry in a [`GuildWidget`].
///
/// Discord replaces `id`, `discriminator`, and `avatar` with anonymized values
/// to prevent abuse, so [`id`](Self::id) is a string rather than a snowflake.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildWidgetMember {
    /// The anonymized user ID.
    pub id: String,

    /// The anonymized username.
    pub username: String,

    /// The anonymized discriminator.
    pub discriminator: String,

    /// The anonymized avatar hash, or `None` if the user has no avatar.
    pub avatar: Option<String>,

    /// The user's presence status.
    pub status: UserStatus,

    /// The URL of the user's widget avatar.
    pub avatar_url: String,
}

/// The body of `PATCH /guilds/{guild.id}/widget`.
///
/// Discord documents that all attributes of the [guild widget settings
/// object][object] may be passed in with JSON, so both fields of that object
/// are optional and omitted during serialization when `None`.
///
/// See [Discord's modify guild widget endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-widget
/// [object]: https://docs.discord.com/developers/resources/guild#guild-widget-settings-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildWidgetRequest {
    /// Whether the widget is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The ID of the channel the widget generates an invite to, or `None` to
    /// leave it unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
}
