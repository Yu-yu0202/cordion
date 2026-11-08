/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Voice state and region models.
//!
//! See [Discord's voice resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/voice

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::guild::{GuildId, GuildMember};
use crate::user::UserId;

/// A user's voice connection status in a guild.
///
/// Voice states included in a [`GUILD_CREATE`](crate::event::gateway::GuildCreate)
/// event lack [`guild_id`](Self::guild_id), because the guild is known from
/// context. [`channel_id`](Self::channel_id) and
/// [`request_to_speak_timestamp`](Self::request_to_speak_timestamp) are always
/// present and serialize as `null` when the user is disconnected or has not
/// requested to speak.
///
/// See [Discord's voice state object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/voice#voice-state-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct VoiceState {
    /// The guild this voice state belongs to.
    ///
    /// Omitted from partial voice states included in a `GUILD_CREATE` event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The voice channel the user is connected to, or `None` when disconnected.
    ///
    /// This field is always present, so serialization emits `null` when it is
    /// `None`.
    pub channel_id: Option<ChannelId>,

    /// The user this voice state is for.
    pub user_id: UserId,

    /// The guild member this voice state is for, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,

    /// The session ID for this voice state.
    pub session_id: String,

    /// Whether this user is deafened by the server.
    pub deaf: bool,

    /// Whether this user is muted by the server.
    pub mute: bool,

    /// Whether this user is locally deafened.
    pub self_deaf: bool,

    /// Whether this user is locally muted.
    pub self_mute: bool,

    /// Whether this user is streaming using "Go Live".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_stream: Option<bool>,

    /// Whether this user's camera is enabled.
    pub self_video: bool,

    /// Whether this user's permission to speak is denied.
    pub suppress: bool,

    /// The time at which the user requested to speak.
    ///
    /// This field is always present, so serialization emits `null` when it is
    /// `None`.
    #[serde(with = "crate::timestamp::option")]
    pub request_to_speak_timestamp: Option<SystemTime>,
}

/// A voice region that can be used for voice and stage channels.
///
/// See [Discord's voice region object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/voice#voice-region-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct VoiceRegion {
    /// The region's unique ID, such as `"us-east"`.
    pub id: String,

    /// The region's display name.
    pub name: String,

    /// Whether this is the single region closest to the current user's client.
    pub optimal: bool,

    /// Whether this is a deprecated region that clients should avoid switching
    /// to.
    pub deprecated: bool,

    /// Whether this is a custom region used for events and similar purposes.
    pub custom: bool,
}

/// The body of `PATCH /guilds/{guild.id}/voice-states/@me`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// The channel must be a stage channel and the target user must already have
/// joined it. See [Discord's modify current user voice state endpoint][endpoint]
/// for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/voice#modify-current-user-voice-state
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyCurrentUserVoiceStateRequest {
    /// The ID of the channel the user is currently in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// Whether to suppress the user from speaking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppress: Option<bool>,

    /// When the user requested to speak.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::timestamp::option"
    )]
    pub request_to_speak_timestamp: Option<SystemTime>,
}

/// The body of `PATCH /guilds/{guild.id}/voice-states/{user.id}`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// Requires the `MUTE_MEMBERS` permission. See [Discord's modify user voice
/// state endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/voice#modify-user-voice-state
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyUserVoiceStateRequest {
    /// The ID of the channel the user is currently in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// Whether to suppress the user from speaking.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppress: Option<bool>,
}
