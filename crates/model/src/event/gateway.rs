/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Gateway event payloads.
//!
//! See [Discord's Gateway events][events] for details.
//!
//! [events]: https://docs.discord.com/developers/events/gateway-events

mod connection;
mod event_name;
mod events;
mod guild_create;
mod opcode;
mod payload;
mod send;

pub use connection::{GetGatewayBotResponse, GetGatewayResponse, SessionStartLimit};
pub use event_name::GatewayEventName;
pub use events::{
    AutoModerationActionExecution, ChannelInfo, ChannelInfoChannel, ChannelPinsUpdate,
    GuildAuditLogEntryCreate, GuildBanAdd, GuildBanRemove, GuildEmojisUpdate,
    GuildIntegrationsUpdate, GuildMemberAdd, GuildMemberRemove, GuildMemberUpdate,
    GuildMembersChunk, GuildRoleCreate, GuildRoleDelete, GuildRoleUpdate,
    GuildScheduledEventUserAdd, GuildScheduledEventUserRemove, GuildSoundboardSoundDelete,
    GuildSoundboardSoundsUpdate, GuildStickersUpdate, Hello, IntegrationDelete, InvalidSession,
    InviteCreate, InviteDelete, MessageDelete, MessageDeleteBulk, MessagePollVoteAdd,
    MessagePollVoteRemove, MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll,
    MessageReactionRemoveEmoji, RateLimited, ReactionType, Ready, ReadyApplication,
    RequestGuildMemberRateLimitMetadata, SoundboardSounds, ThreadListSync, ThreadMemberUpdate,
    ThreadMembersUpdate, TypingStart, VoiceChannelEffectSend, VoiceChannelStartTimeUpdate,
    VoiceChannelStatusUpdate, VoiceEffectAnimationType, VoiceServerUpdate, WebhooksUpdate,
};
pub use guild_create::{AvailableGuild, GuildCreate, UnavailableGuild};
pub use opcode::GatewayOpcode;
pub use payload::GatewayPayload;
pub use send::{
    GatewayPresenceUpdate, GatewayVoiceStateUpdate, Identify, IdentifyConnectionProperties,
    RequestChannelInfo, RequestGuildMembers, RequestGuildMembersUserIds, RequestSoundboardSounds,
    Resume,
};
