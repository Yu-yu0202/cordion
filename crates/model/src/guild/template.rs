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

use crate::channel::{ChannelType, OverwriteType};
use crate::guild::{
    DefaultMessageNotificationLevel, ExplicitContentFilterLevel, GuildId, SystemChannelFlags,
    VerificationLevel,
};
use crate::permission::Permission;
use crate::user::{User, UserId};

/// A guild template created from a snapshot of a guild.
///
/// See [Discord's guild template object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild-template#guild-template-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildTemplate {
    /// The template code.
    pub code: String,

    /// The template name.
    pub name: String,

    /// The template description, or `None` if Discord sends `null`.
    pub description: Option<String>,

    /// The number of times this template has been used.
    pub usage_count: u64,

    /// The ID of the user who created the template.
    pub creator_id: UserId,

    /// The user who created the template.
    pub creator: User,

    /// When this template was created.
    #[serde(with = "crate::timestamp")]
    pub created_at: SystemTime,

    /// When this template was last synced to the source guild.
    #[serde(with = "crate::timestamp")]
    pub updated_at: SystemTime,

    /// The ID of the guild this template is based on.
    pub source_guild_id: GuildId,

    /// The guild snapshot this template contains.
    pub serialized_source_guild: TemplateGuild,

    /// Whether the template has unsynced changes.
    ///
    /// Discord sends `null` in its example even though the field table marks it
    /// as nullable but present; this model preserves that shape.
    pub is_dirty: Option<bool>,
}

/// The guild snapshot stored in a [`GuildTemplate`].
///
/// This is a partial guild object whose placeholder IDs are integers rather
/// than snowflakes. It is therefore modeled separately from
/// [`Guild`](super::Guild) and [`PartialGuild`](super::PartialGuild).
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TemplateGuild {
    /// The guild name.
    pub name: String,

    /// The guild description, or `None` if Discord sends `null`.
    pub description: Option<String>,

    /// The guild's voice region.
    ///
    /// Deprecated in the full guild object, but still included in template
    /// snapshots.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// The verification level required to join the guild.
    pub verification_level: VerificationLevel,

    /// The default message notification level.
    pub default_message_notifications: DefaultMessageNotificationLevel,

    /// The explicit content filter level.
    pub explicit_content_filter: ExplicitContentFilterLevel,

    /// The guild's preferred locale.
    pub preferred_locale: String,

    /// The AFK timeout in seconds.
    pub afk_timeout: u64,

    /// The template's roles.
    pub roles: Vec<TemplateRole>,

    /// The template's channels.
    pub channels: Vec<TemplateChannel>,

    /// The placeholder ID of the AFK channel, or `None` if none is set.
    pub afk_channel_id: Option<u64>,

    /// The placeholder ID of the system channel, or `None` if none is set.
    pub system_channel_id: Option<u64>,

    /// Notification flags for the system channel.
    pub system_channel_flags: SystemChannelFlags,

    /// The guild's icon hash, or `None` if it has no icon.
    pub icon_hash: Option<String>,
}

/// A role in a [`TemplateGuild`].
///
/// Placeholder IDs are integers in the template snapshot. The permission set is
/// represented by [`Permission`], which accepts Discord's integer encoding and
/// serializes it as a decimal string under Cordion's wire conventions.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TemplateRole {
    /// The placeholder role ID.
    pub id: u64,

    /// The role's name.
    pub name: String,

    /// The role's permission bit set.
    pub permissions: Permission,

    /// The role's color as a 24-bit RGB value (`0xRRGGBB`).
    pub color: u32,

    /// Whether the role is displayed separately in the member list.
    pub hoist: bool,

    /// Whether the role can be mentioned.
    pub mentionable: bool,
}

/// A channel in a [`TemplateGuild`].
///
/// Placeholder IDs are integers in the template snapshot. Discord's documented
/// example includes voice-channel fields even for category and text channels.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TemplateChannel {
    /// The channel's name.
    pub name: String,

    /// The channel's sorting position.
    pub position: i32,

    /// The channel's topic, or `None` if unset.
    pub topic: Option<String>,

    /// The voice channel's bitrate in bits per second.
    pub bitrate: u64,

    /// The voice channel's user limit; `0` means no limit.
    pub user_limit: u64,

    /// Whether the channel is age-restricted.
    pub nsfw: bool,

    /// Seconds a user must wait before sending another message.
    pub rate_limit_per_user: u64,

    /// The placeholder ID of the parent category, or `None` if none.
    pub parent_id: Option<u64>,

    /// Explicit permission overwrites for roles and members.
    pub permission_overwrites: Vec<TemplateOverwrite>,

    /// The placeholder channel ID.
    pub id: u64,

    /// The channel's type.
    #[serde(rename = "type")]
    pub kind: ChannelType,
}

/// A permission overwrite in a [`TemplateChannel`].
///
/// The overwrite target's placeholder ID is an integer in the template
/// snapshot.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TemplateOverwrite {
    /// The placeholder ID of the role or member whose permissions are
    /// overwritten.
    pub id: u64,

    /// Whether the overwrite applies to a role or a member.
    #[serde(rename = "type")]
    pub kind: OverwriteType,

    /// The permissions granted by this overwrite.
    pub allow: Permission,

    /// The permissions denied by this overwrite.
    pub deny: Permission,
}

/// The body of `POST /guilds/{guild.id}/templates`.
///
/// Discord documents [`name`](Self::name) as a required string and
/// [`description`](Self::description) as an optional nullable string, so an
/// absent description is omitted during serialization.
///
/// See [Discord's create guild template endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild-template#create-guild-template
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGuildTemplateRequest {
    /// The template's name, between 1 and 100 characters.
    pub name: String,

    /// The template's description, between 0 and 120 characters.
    ///
    /// Omitted during serialization when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// The body of `PATCH /guilds/{guild.id}/templates/{template.code}`.
///
/// Discord documents every parameter as optional, so each field is omitted
/// during serialization when `None` and an omitted field leaves the template's
/// metadata unchanged. Because omission means "leave unchanged", the nullable
/// [`description`](Self::description) cannot be cleared by sending `null`
/// through this type.
///
/// See [Discord's modify guild template endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild-template#modify-guild-template
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildTemplateRequest {
    /// The template's name, between 1 and 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The template's description, between 0 and 120 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
