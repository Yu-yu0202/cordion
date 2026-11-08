/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Payloads of the events Discord dispatches over the Gateway.
//!
//! Each type in this module is the `d` field of a [`GatewayPayload`](super::GatewayPayload)
//! whose `op` is [`GatewayOpcode::DISPATCH`](super::GatewayOpcode::DISPATCH) and
//! whose `t` is the name of the corresponding event, as listed by
//! [`GatewayEventName`](super::GatewayEventName).
//!
//! Many events dispatch an existing resource model unchanged; those events are
//! listed in the documentation of the module they belong to, and their payload
//! is decoded into that model directly.
//!
//! See [Discord's Gateway receive events][receive] for the full list.
//!
//! [receive]: https://docs.discord.com/developers/events/gateway-events#receive-events

mod channels;
mod guilds;
mod messages;
mod misc;
mod voice;

pub use channels::{
    ChannelInfo, ChannelInfoChannel, ChannelPinsUpdate, InviteCreate, InviteDelete, ThreadListSync,
    ThreadMemberUpdate, ThreadMembersUpdate, VoiceChannelStartTimeUpdate, VoiceChannelStatusUpdate,
    WebhooksUpdate,
};
pub use guilds::{
    GuildAuditLogEntryCreate, GuildBanAdd, GuildBanRemove, GuildEmojisUpdate,
    GuildIntegrationsUpdate, GuildMemberAdd, GuildMemberRemove, GuildMemberUpdate,
    GuildMembersChunk, GuildRoleCreate, GuildRoleDelete, GuildRoleUpdate,
    GuildScheduledEventUserAdd, GuildScheduledEventUserRemove, GuildSoundboardSoundDelete,
    GuildSoundboardSoundsUpdate, GuildStickersUpdate, IntegrationDelete, SoundboardSounds,
};
pub use messages::{
    MessageDelete, MessageDeleteBulk, MessagePollVoteAdd, MessagePollVoteRemove,
    MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll,
    MessageReactionRemoveEmoji, ReactionType, TypingStart,
};
pub use misc::{
    AutoModerationActionExecution, Hello, InvalidSession, RateLimited, Ready, ReadyApplication,
    RequestGuildMemberRateLimitMetadata,
};
pub use voice::{VoiceChannelEffectSend, VoiceEffectAnimationType, VoiceServerUpdate};
