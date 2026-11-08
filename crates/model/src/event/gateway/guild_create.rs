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

use crate::channel::Channel;
use crate::event::presence::Presence;
use crate::guild::{
    Guild, GuildId, GuildMember, GuildScheduledEvent, SoundboardSound, StageInstance,
};
use crate::voice::VoiceState;

/// The payload of a `GUILD_CREATE` Gateway event.
///
/// Discord sends either an available guild with extra fields or a minimal
/// unavailable guild, so the two shapes are distinguished by trying
/// [`AvailableGuild`] first and falling back to [`UnavailableGuild`].
///
/// See [Discord's Guild Create event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#guild-create
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
// Both variants are owned wire payloads; boxing the larger one would make the
// public API require dereferencing for no runtime benefit.
#[allow(clippy::large_enum_variant)]
pub enum GuildCreate {
    /// A guild whose full information is included.
    Available(AvailableGuild),

    /// A guild that is currently unavailable.
    Unavailable(UnavailableGuild),
}

/// An available guild in a `GUILD_CREATE` event.
///
/// This is a full [`Guild`] plus fields that only appear in this event.
/// Discord may omit most members and presences when the bot lacks the
/// `GUILD_PRESENCES` intent or the guild has more than 75,000 members; the
/// arrays themselves are still present.
///
/// See [Discord's Guild Create extra fields][fields] for details.
///
/// [fields]: https://docs.discord.com/developers/events/gateway-events#guild-create-guild-create-extra-fields
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AvailableGuild {
    /// The full guild object.
    #[serde(flatten)]
    pub guild: Guild,

    /// When the current user joined the guild.
    #[serde(with = "crate::timestamp")]
    pub joined_at: SystemTime,

    /// Whether Discord considers this a large guild.
    pub large: bool,

    /// Whether the guild is unavailable due to an outage.
    ///
    /// This is normally absent or `false` for an available guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable: Option<bool>,

    /// The total number of members in the guild.
    pub member_count: u64,

    /// States of members currently in voice channels.
    ///
    /// These partial voice states omit [`guild_id`](VoiceState::guild_id).
    pub voice_states: Vec<VoiceState>,

    /// Users in the guild.
    pub members: Vec<GuildMember>,

    /// Channels in the guild.
    pub channels: Vec<Channel>,

    /// Active threads the current user can view.
    pub threads: Vec<Channel>,

    /// Presences of members in the guild.
    ///
    /// These partial presences omit [`guild_id`](Presence::guild_id).
    pub presences: Vec<Presence>,

    /// Stage instances in the guild.
    pub stage_instances: Vec<StageInstance>,

    /// Scheduled events in the guild.
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,

    /// Soundboard sounds in the guild.
    pub soundboard_sounds: Vec<SoundboardSound>,
}

/// A minimal guild object for a guild whose information is unavailable.
///
/// The same shape is used by the `GUILD_DELETE` event and by the guilds listed
/// in the `READY` event. See [Discord's unavailable guild object][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#unavailable-guild-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UnavailableGuild {
    /// The guild's ID.
    pub id: GuildId,

    /// Whether the guild is unavailable.
    ///
    /// Discord sends `true` in practice, but this is modeled as a plain boolean
    /// because the field is not explicitly documented as always `true`.
    pub unavailable: bool,
}
