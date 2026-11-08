/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Guild-related Gateway events.
//!
//! Several guild events carry an existing model unchanged, so no dedicated event
//! type exists for them:
//!
//! | Event | Payload |
//! | - | - |
//! | `GUILD_CREATE` | [`GuildCreate`] |
//! | `GUILD_UPDATE` | [`Guild`] |
//! | `GUILD_DELETE` | [`UnavailableGuild`] |
//! | `GUILD_SCHEDULED_EVENT_CREATE` | [`GuildScheduledEvent`] |
//! | `GUILD_SCHEDULED_EVENT_UPDATE` | [`GuildScheduledEvent`] |
//! | `GUILD_SCHEDULED_EVENT_DELETE` | [`GuildScheduledEvent`] |
//! | `GUILD_SOUNDBOARD_SOUND_CREATE` | [`SoundboardSound`] |
//! | `GUILD_SOUNDBOARD_SOUND_UPDATE` | [`SoundboardSound`] |
//! | `INTEGRATION_CREATE` | [`Integration`] |
//! | `INTEGRATION_UPDATE` | [`Integration`] |
//! | `STAGE_INSTANCE_CREATE` | [`StageInstance`] |
//! | `STAGE_INSTANCE_UPDATE` | [`StageInstance`] |
//! | `STAGE_INSTANCE_DELETE` | [`StageInstance`] |
//!
//! Discord documents that `INTEGRATION_CREATE` and `INTEGRATION_UPDATE` omit
//! the integration's `user` key and add `guild_id`; [`Integration`] models
//! `user` as optional, so the payload decodes without a dedicated event type.
//!
//! [`Guild`]: crate::guild::Guild
//! [`GuildCreate`]: crate::event::gateway::GuildCreate
//! [`GuildScheduledEvent`]: crate::guild::GuildScheduledEvent
//! [`Integration`]: crate::guild::Integration
//! [`SoundboardSound`]: crate::guild::SoundboardSound
//! [`StageInstance`]: crate::guild::StageInstance
//! [`UnavailableGuild`]: crate::event::gateway::UnavailableGuild

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::audit_log::AuditLogEntry;
use crate::emoji::Emoji;
use crate::event::presence::Presence;
use crate::guild::{
    GuildId, GuildMember, GuildScheduledEventId, IntegrationId, SoundboardSound, SoundboardSoundId,
};
use crate::role::{Role, RoleId};
use crate::sticker::Sticker;
use crate::user::{AvatarDecorationData, Collectibles, User, UserId};

/// The payload of the `GUILD_AUDIT_LOG_ENTRY_CREATE` event.
///
/// The event flattens an audit log entry and adds the guild's ID. Only sent to
/// applications with the `VIEW_AUDIT_LOG` permission.
///
/// See [Discord's Guild Audit Log Entry Create event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-audit-log-entry-create
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GuildAuditLogEntryCreate {
    /// The audit log entry that was created.
    #[serde(flatten)]
    pub entry: AuditLogEntry,

    /// The guild's ID.
    pub guild_id: GuildId,
}

/// The payload of the `GUILD_MEMBER_ADD` event.
///
/// The event flattens a guild member and adds the guild's ID. Sent when a
/// user joins a guild, and also for users who are already members.
///
/// See [Discord's Guild Member Add event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-member-add
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildMemberAdd {
    /// The member who joined the guild.
    #[serde(flatten)]
    pub member: GuildMember,

    /// The guild's ID.
    pub guild_id: GuildId,
}

/// The payload of the `GUILD_MEMBER_REMOVE` event.
///
/// See [Discord's Guild Member Remove event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-member-remove
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildMemberRemove {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The user who was removed.
    pub user: User,
}

/// The payload of the `GUILD_MEMBER_UPDATE` event.
///
/// Sent when a guild member is updated, including when the member's user object
/// changes. The member fields other than [`roles`](Self::roles) and
/// [`user`](Self::user) are nullable: Discord sends `null` for a value that was
/// removed and omits the field when it did not change, so both decode as `None`
/// and serialize as `null`.
///
/// See [Discord's Guild Member Update event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-member-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildMemberUpdate {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The member's role IDs.
    ///
    /// The `@everyone` role is implicit and is not listed.
    pub roles: Vec<RoleId>,

    /// The member's user.
    pub user: User,

    /// The member's guild nickname, or `None` if it was removed.
    #[serde(default)]
    pub nick: Option<String>,

    /// The member's guild avatar hash.
    #[serde(default)]
    pub avatar: Option<String>,

    /// The member's guild banner hash.
    #[serde(default)]
    pub banner: Option<String>,

    /// When the member joined the guild.
    #[serde(default, with = "crate::timestamp::option")]
    pub joined_at: Option<SystemTime>,

    /// When the member started boosting the guild.
    #[serde(default, with = "crate::timestamp::option")]
    pub premium_since: Option<SystemTime>,

    /// Whether the member is deafened in voice channels.
    #[serde(default)]
    pub deaf: Option<bool>,

    /// Whether the member is muted in voice channels.
    #[serde(default)]
    pub mute: Option<bool>,

    /// Whether the member has not yet passed membership screening.
    #[serde(default)]
    pub pending: Option<bool>,

    /// When the member's timeout expires.
    ///
    /// Discord sends `null` or a time in the past when the member is not timed
    /// out.
    #[serde(default, with = "crate::timestamp::option")]
    pub communication_disabled_until: Option<SystemTime>,

    /// The member's guild avatar decoration.
    #[serde(default)]
    pub avatar_decoration_data: Option<AvatarDecorationData>,

    /// The member's collectibles.
    #[serde(default)]
    pub collectibles: Option<Collectibles>,
}

/// The payload of the `GUILD_MEMBERS_CHUNK` event.
///
/// Sent in response to a Request Guild Members send event. [`chunk_index`](Self::chunk_index)
/// and [`chunk_count`](Self::chunk_count) describe how many chunks remain.
///
/// See [Discord's Guild Members Chunk event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-members-chunk
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildMembersChunk {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The members matching the request.
    pub members: Vec<GuildMember>,

    /// The index of this chunk, between `0` and [`chunk_count`](Self::chunk_count).
    pub chunk_index: u64,

    /// The total number of expected chunks.
    pub chunk_count: u64,

    /// The requested user IDs that Discord could not resolve.
    ///
    /// Discord documents this only as an array; the entries correspond to the
    /// `user_ids` of the request, so they are modeled as user IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_found: Option<Vec<UserId>>,

    /// The presences of the returned members, when requested.
    ///
    /// Requires the `GUILD_PRESENCES` intent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presences: Option<Vec<Presence>>,

    /// The nonce of the request this chunk answers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// The payload of the `GUILD_BAN_ADD` event.
///
/// Only sent to applications with the `BAN_MEMBERS` or `VIEW_AUDIT_LOG`
/// permission.
///
/// See [Discord's Guild Ban Add event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-ban-add
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildBanAdd {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The user who was banned.
    pub user: User,
}

/// The payload of the `GUILD_BAN_REMOVE` event.
///
/// Only sent to applications with the `BAN_MEMBERS` or `VIEW_AUDIT_LOG`
/// permission.
///
/// See [Discord's Guild Ban Remove event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-ban-remove
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildBanRemove {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The user who was unbanned.
    pub user: User,
}

/// The payload of the `GUILD_EMOJIS_UPDATE` event.
///
/// See [Discord's Guild Emojis Update event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-emojis-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildEmojisUpdate {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The guild's emojis after the update.
    pub emojis: Vec<Emoji>,
}

/// The payload of the `GUILD_STICKERS_UPDATE` event.
///
/// See [Discord's Guild Stickers Update event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-stickers-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildStickersUpdate {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The guild's stickers after the update.
    pub stickers: Vec<Sticker>,
}

/// The payload of the `GUILD_INTEGRATIONS_UPDATE` event.
///
/// See [Discord's Guild Integrations Update event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-integrations-update
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildIntegrationsUpdate {
    /// The guild's ID.
    pub guild_id: GuildId,
}

/// The payload of the `GUILD_ROLE_CREATE` event.
///
/// See [Discord's Guild Role Create event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-role-create
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildRoleCreate {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The role that was created.
    pub role: Role,
}

/// The payload of the `GUILD_ROLE_UPDATE` event.
///
/// See [Discord's Guild Role Update event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-role-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildRoleUpdate {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The role that was updated.
    pub role: Role,
}

/// The payload of the `GUILD_ROLE_DELETE` event.
///
/// See [Discord's Guild Role Delete event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-role-delete
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildRoleDelete {
    /// The guild's ID.
    pub guild_id: GuildId,

    /// The ID of the role that was deleted.
    pub role_id: RoleId,
}

/// The payload of the `GUILD_SCHEDULED_EVENT_USER_ADD` event.
///
/// Sent when a user subscribes to a guild scheduled event.
///
/// See [Discord's Guild Scheduled Event User Add event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-scheduled-event-user-add
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventUserAdd {
    /// The ID of the guild scheduled event.
    pub guild_scheduled_event_id: GuildScheduledEventId,

    /// The ID of the user who subscribed.
    pub user_id: UserId,

    /// The guild's ID.
    pub guild_id: GuildId,
}

/// The payload of the `GUILD_SCHEDULED_EVENT_USER_REMOVE` event.
///
/// Sent when a user unsubscribes from a guild scheduled event.
///
/// See [Discord's Guild Scheduled Event User Remove event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-scheduled-event-user-remove
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildScheduledEventUserRemove {
    /// The ID of the guild scheduled event.
    pub guild_scheduled_event_id: GuildScheduledEventId,

    /// The ID of the user who unsubscribed.
    pub user_id: UserId,

    /// The guild's ID.
    pub guild_id: GuildId,
}

/// The payload of the `GUILD_SOUNDBOARD_SOUND_DELETE` event.
///
/// See [Discord's Guild Soundboard Sound Delete event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-soundboard-sound-delete
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildSoundboardSoundDelete {
    /// The ID of the sound that was deleted.
    pub sound_id: SoundboardSoundId,

    /// The ID of the guild the sound was in.
    pub guild_id: GuildId,
}

/// The payload of the `GUILD_SOUNDBOARD_SOUNDS_UPDATE` event.
///
/// Sent when multiple soundboard sounds of a guild are updated.
///
/// See [Discord's Guild Soundboard Sounds Update event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-soundboard-sounds-update
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GuildSoundboardSoundsUpdate {
    /// The guild's soundboard sounds after the update.
    pub soundboard_sounds: Vec<SoundboardSound>,

    /// The guild's ID.
    pub guild_id: GuildId,
}

/// The payload of the `SOUNDBOARD_SOUNDS` event.
///
/// Sent in response to a Request Soundboard Sounds send event. The payload is
/// identical to [`GuildSoundboardSoundsUpdate`], but the event answers a request
/// instead of reporting an update.
///
/// See [Discord's Soundboard Sounds event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#soundboard-sounds
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SoundboardSounds {
    /// The guild's soundboard sounds.
    pub soundboard_sounds: Vec<SoundboardSound>,

    /// The guild's ID.
    pub guild_id: GuildId,
}

/// The payload of the `INTEGRATION_DELETE` event.
///
/// See [Discord's Integration Delete event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#integration-delete
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IntegrationDelete {
    /// The ID of the integration that was deleted.
    pub id: IntegrationId,

    /// The guild's ID.
    pub guild_id: GuildId,

    /// The ID of the bot or `OAuth2` application of the integration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
}
