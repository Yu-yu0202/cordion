/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Gateway payloads an application sends to Discord.
//!
//! Each type here is the `d` field of a [`GatewayPayload`](super::GatewayPayload)
//! whose `op` is the corresponding [`GatewayOpcode`](super::GatewayOpcode). The
//! payloads are modeled for sending: optional fields are omitted during
//! serialization when `None`, and fields Discord documents as nullable are
//! serialized as `null`.
//!
//! See [Discord's Gateway send events][send] for details.
//!
//! [send]: https://docs.discord.com/developers/events/gateway-events#send-events

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::event::presence::{Activity, UserStatus};
use crate::guild::GuildId;
use crate::user::UserId;

/// The payload of the Identify send event.
///
/// [`compress`](Self::compress), [`large_threshold`](Self::large_threshold),
/// [`shard`](Self::shard), [`presence`](Self::presence), and
/// [`capabilities`](Self::capabilities) are optional and are omitted during
/// serialization when `None`. [`intents`](Self::intents) is always sent.
///
/// See [Discord's Identify structure][identify] for details.
///
/// [identify]: https://docs.discord.com/developers/events/gateway-events#identify
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Identify {
    /// The application's authentication token.
    pub token: String,

    /// The connection properties that describe the client.
    pub properties: IdentifyConnectionProperties,

    /// Whether this connection supports compression of packets.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,

    /// The number of members at which Discord stops sending offline members.
    ///
    /// Discord requires a value between 50 and 250 and defaults to `50`. The
    /// value is not validated by this type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<u64>,

    /// The shard this connection serves, as `[shard_id, num_shards]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u64; 2]>,

    /// The initial presence of the connection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence: Option<GatewayPresenceUpdate>,

    /// The Gateway intents the application wishes to receive.
    ///
    /// The events an application receives are limited to the intents it
    /// requests.
    pub intents: u64,

    /// A bitfield of Gateway client capabilities.
    ///
    /// Discord documents `CHANNEL_OBFUSCATION` (`1 << 15`) and defaults this to
    /// `0`. The bitfield is not decoded by this type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<u64>,
}

/// The connection properties of an [`Identify`] payload.
///
/// The fields were originally sent with a `$` prefix, which Discord has
/// deprecated in favor of these names.
///
/// See [Discord's identify connection properties][properties] for details.
///
/// [properties]: https://docs.discord.com/developers/events/gateway-events#identify-identify-connection-properties
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IdentifyConnectionProperties {
    /// The client's operating system.
    pub os: String,

    /// The client library's name.
    pub browser: String,

    /// The client library's name.
    pub device: String,
}

/// The payload of the Resume send event.
///
/// See [Discord's Resume structure][resume] for details.
///
/// [resume]: https://docs.discord.com/developers/events/gateway-events#resume
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Resume {
    /// The session token to resume.
    pub token: String,

    /// The ID of the session to resume.
    pub session_id: String,

    /// The last sequence number received on the dropped connection.
    pub seq: u64,
}

/// The payload of the Request Guild Members send event.
///
/// Discord requires either [`query`](Self::query) or
/// [`user_ids`](Self::user_ids), and [`limit`](Self::limit) is required when a
/// query is given. All of these are optional in this model because the request
/// is valid with either alternative. Requesting more than one guild per payload
/// is rejected by Discord.
///
/// See [Discord's Request Guild Members structure][request] for details.
///
/// [request]: https://docs.discord.com/developers/events/gateway-events#request-guild-members
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RequestGuildMembers {
    /// The guild to request members for.
    pub guild_id: GuildId,

    /// A prefix that a member's username must start with.
    ///
    /// Use an empty string to request every member. Discord returns at most 100
    /// members when this is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// The maximum number of members to return for [`query`](Self::query).
    ///
    /// `0` together with an empty [`query`](Self::query) requests every member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Whether to include the presences of the matched members.
    ///
    /// Requires the `GUILD_PRESENCES` intent; Discord otherwise treats this as
    /// `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presences: Option<bool>,

    /// The users to request members for.
    ///
    /// Discord returns at most 100 members for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<RequestGuildMembersUserIds>,

    /// A nonce echoed back on the resulting Guild Members Chunk events.
    ///
    /// Discord limits a nonce to 32 bytes and ignores longer values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// The users requested by a [`RequestGuildMembers`] payload.
///
/// Discord accepts a single snowflake or an array of snowflakes, so both shapes
/// are preserved instead of forcing one representation.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestGuildMembersUserIds {
    /// A single user ID.
    One(UserId),

    /// Two or more user IDs.
    Many(Vec<UserId>),
}

/// The payload of the Request Soundboard Sounds send event.
///
/// See [Discord's Request Soundboard Sounds structure][request] for details.
///
/// [request]: https://docs.discord.com/developers/events/gateway-events#request-soundboard-sounds
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RequestSoundboardSounds {
    /// The guilds to request soundboard sounds for.
    pub guild_ids: Vec<GuildId>,
}

/// The payload of the Request Channel Info send event.
///
/// [`fields`](Self::fields) names the ephemeral channel data to request. Discord
/// currently documents `status` and `voice_start_time`, and the names are
/// preserved verbatim so that a field Discord adds later still round-trips.
///
/// See [Discord's Request Channel Info structure][request] for details.
///
/// [request]: https://docs.discord.com/developers/events/gateway-events#request-channel-info
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RequestChannelInfo {
    /// The guild to request channel data for.
    pub guild_id: GuildId,

    /// The names of the fields to request.
    pub fields: Vec<String>,
}

/// The payload of the Update Voice State send event.
///
/// [`channel_id`](Self::channel_id) is `null` when the client disconnects, so
/// `None` is serialized as `null`.
///
/// See [Discord's Gateway Voice State Update structure][update] for details.
///
/// [update]: https://docs.discord.com/developers/events/gateway-events#update-voice-state
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GatewayVoiceStateUpdate {
    /// The guild whose voice state is updated.
    pub guild_id: GuildId,

    /// The voice channel to join, or `None` to disconnect.
    #[serde(default)]
    pub channel_id: Option<ChannelId>,

    /// Whether the client is muted.
    pub self_mute: bool,

    /// Whether the client is deafened.
    pub self_deaf: bool,
}

/// The payload of the Update Presence send event.
///
/// [`since`](Self::since) is `null` when the client is not idle, so `None` is
/// serialized as `null`. Discord documents five statuses, of which
/// [`UserStatus`] models `online`, `idle`, `dnd`, and `offline`; an
/// undocumented status such as `invisible` is preserved.
///
/// See [Discord's Gateway Presence Update structure][update] for details.
///
/// [update]: https://docs.discord.com/developers/events/gateway-events#update-presence
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GatewayPresenceUpdate {
    /// When the client went idle, as Unix milliseconds.
    ///
    /// `null` while the client is not idle.
    #[serde(default)]
    pub since: Option<u64>,

    /// The client's activities.
    pub activities: Vec<Activity>,

    /// The client's new status.
    pub status: UserStatus,

    /// Whether the client is AFK.
    pub afk: bool,
}
