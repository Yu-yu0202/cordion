/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Other Gateway events that do not belong to a single resource group.
//!
//! Every receive event name maps either to a dedicated payload type in this
//! module tree or to an existing model that Discord dispatches unchanged. The
//! complete mapping is:
//!
//! | Event | Payload |
//! | - | - |
//! | `APPLICATION_COMMAND_PERMISSIONS_UPDATE` | [`GuildApplicationCommandPermissions`] |
//! | `AUTO_MODERATION_ACTION_EXECUTION` | [`AutoModerationActionExecution`] |
//! | `AUTO_MODERATION_RULE_CREATE` | [`AutoModerationRule`] |
//! | `AUTO_MODERATION_RULE_UPDATE` | [`AutoModerationRule`] |
//! | `AUTO_MODERATION_RULE_DELETE` | [`AutoModerationRule`] |
//! | `CHANNEL_CREATE` | [`Channel`] |
//! | `CHANNEL_DELETE` | [`Channel`] |
//! | `CHANNEL_INFO` | [`ChannelInfo`] |
//! | `CHANNEL_PINS_UPDATE` | [`ChannelPinsUpdate`] |
//! | `CHANNEL_UPDATE` | [`Channel`] |
//! | `ENTITLEMENT_CREATE` | [`Entitlement`] |
//! | `ENTITLEMENT_DELETE` | [`Entitlement`] |
//! | `ENTITLEMENT_UPDATE` | [`Entitlement`] |
//! | `GUILD_AUDIT_LOG_ENTRY_CREATE` | [`GuildAuditLogEntryCreate`] |
//! | `GUILD_BAN_ADD` | [`GuildBanAdd`] |
//! | `GUILD_BAN_REMOVE` | [`GuildBanRemove`] |
//! | `GUILD_CREATE` | [`GuildCreate`] |
//! | `GUILD_DELETE` | [`UnavailableGuild`] |
//! | `GUILD_EMOJIS_UPDATE` | [`GuildEmojisUpdate`] |
//! | `GUILD_INTEGRATIONS_UPDATE` | [`GuildIntegrationsUpdate`] |
//! | `GUILD_MEMBER_ADD` | [`GuildMemberAdd`] |
//! | `GUILD_MEMBER_REMOVE` | [`GuildMemberRemove`] |
//! | `GUILD_MEMBER_UPDATE` | [`GuildMemberUpdate`] |
//! | `GUILD_MEMBERS_CHUNK` | [`GuildMembersChunk`] |
//! | `GUILD_ROLE_CREATE` | [`GuildRoleCreate`] |
//! | `GUILD_ROLE_DELETE` | [`GuildRoleDelete`] |
//! | `GUILD_ROLE_UPDATE` | [`GuildRoleUpdate`] |
//! | `GUILD_SCHEDULED_EVENT_CREATE` | [`GuildScheduledEvent`] |
//! | `GUILD_SCHEDULED_EVENT_DELETE` | [`GuildScheduledEvent`] |
//! | `GUILD_SCHEDULED_EVENT_UPDATE` | [`GuildScheduledEvent`] |
//! | `GUILD_SCHEDULED_EVENT_USER_ADD` | [`GuildScheduledEventUserAdd`] |
//! | `GUILD_SCHEDULED_EVENT_USER_REMOVE` | [`GuildScheduledEventUserRemove`] |
//! | `GUILD_SOUNDBOARD_SOUND_CREATE` | [`SoundboardSound`] |
//! | `GUILD_SOUNDBOARD_SOUND_DELETE` | [`GuildSoundboardSoundDelete`] |
//! | `GUILD_SOUNDBOARD_SOUND_UPDATE` | [`SoundboardSound`] |
//! | `GUILD_SOUNDBOARD_SOUNDS_UPDATE` | [`GuildSoundboardSoundsUpdate`] |
//! | `GUILD_STICKERS_UPDATE` | [`GuildStickersUpdate`] |
//! | `GUILD_UPDATE` | [`Guild`] |
//! | `HELLO` | [`Hello`] |
//! | `INVALID_SESSION` | [`InvalidSession`] |
//! | `INTEGRATION_CREATE` | [`Integration`] |
//! | `INTEGRATION_DELETE` | [`IntegrationDelete`] |
//! | `INTEGRATION_UPDATE` | [`Integration`] |
//! | `INTERACTION_CREATE` | [`Interaction`] |
//! | `INVITE_CREATE` | [`InviteCreate`] |
//! | `INVITE_DELETE` | [`InviteDelete`] |
//! | `MESSAGE_CREATE` | [`Message`] |
//! | `MESSAGE_DELETE` | [`MessageDelete`] |
//! | `MESSAGE_DELETE_BULK` | [`MessageDeleteBulk`] |
//! | `MESSAGE_POLL_VOTE_ADD` | [`MessagePollVoteAdd`] |
//! | `MESSAGE_POLL_VOTE_REMOVE` | [`MessagePollVoteRemove`] |
//! | `MESSAGE_REACTION_ADD` | [`MessageReactionAdd`] |
//! | `MESSAGE_REACTION_REMOVE` | [`MessageReactionRemove`] |
//! | `MESSAGE_REACTION_REMOVE_ALL` | [`MessageReactionRemoveAll`] |
//! | `MESSAGE_REACTION_REMOVE_EMOJI` | [`MessageReactionRemoveEmoji`] |
//! | `MESSAGE_UPDATE` | [`Message`] |
//! | `PRESENCE_UPDATE` | [`Presence`] |
//! | `RATE_LIMITED` | [`RateLimited`] |
//! | `READY` | [`Ready`] |
//! | `SOUNDBOARD_SOUNDS` | [`SoundboardSounds`] |
//! | `STAGE_INSTANCE_CREATE` | [`StageInstance`] |
//! | `STAGE_INSTANCE_DELETE` | [`StageInstance`] |
//! | `STAGE_INSTANCE_UPDATE` | [`StageInstance`] |
//! | `SUBSCRIPTION_CREATE` | [`Subscription`] |
//! | `SUBSCRIPTION_DELETE` | [`Subscription`] |
//! | `SUBSCRIPTION_UPDATE` | [`Subscription`] |
//! | `THREAD_CREATE` | [`Channel`] |
//! | `THREAD_DELETE` | [`Channel`] |
//! | `THREAD_LIST_SYNC` | [`ThreadListSync`] |
//! | `THREAD_MEMBERS_UPDATE` | [`ThreadMembersUpdate`] |
//! | `THREAD_MEMBER_UPDATE` | [`ThreadMemberUpdate`] |
//! | `THREAD_UPDATE` | [`Channel`] |
//! | `TYPING_START` | [`TypingStart`] |
//! | `USER_UPDATE` | [`User`] |
//! | `VOICE_CHANNEL_EFFECT_SEND` | [`VoiceChannelEffectSend`] |
//! | `VOICE_CHANNEL_START_TIME_UPDATE` | [`VoiceChannelStartTimeUpdate`] |
//! | `VOICE_CHANNEL_STATUS_UPDATE` | [`VoiceChannelStatusUpdate`] |
//! | `VOICE_SERVER_UPDATE` | [`VoiceServerUpdate`] |
//! | `VOICE_STATE_UPDATE` | [`VoiceState`] |
//! | `WEBHOOKS_UPDATE` | [`WebhooksUpdate`] |
//!
//! `MESSAGE_CREATE` and `MESSAGE_UPDATE` carry a [`Message`] with the extra
//! gateway fields Discord documents for those events ([`guild_id`](crate::message::Message::guild_id),
//! [`member`](crate::message::Message::member), and
//! [`channel_type`](crate::message::Message::channel_type)), so [`Message`]
//! models them without a dedicated event type. The `RESUMED` and `RECONNECT`
//! events carry no data and dispatch `d: null`, which
//! [`GatewayPayload`](crate::event::gateway::GatewayPayload) decodes as `None`.
//!
//! [`AutoModerationActionExecution`]: crate::event::gateway::AutoModerationActionExecution
//! [`AutoModerationRule`]: crate::auto_moderation::AutoModerationRule
//! [`Channel`]: crate::channel::Channel
//! [`ChannelInfo`]: crate::event::gateway::ChannelInfo
//! [`ChannelPinsUpdate`]: crate::event::gateway::ChannelPinsUpdate
//! [`Entitlement`]: crate::monetization::entitlement::Entitlement
//! [`Guild`]: crate::guild::Guild
//! [`GuildApplicationCommandPermissions`]: crate::application_command::GuildApplicationCommandPermissions
//! [`GuildAuditLogEntryCreate`]: crate::event::gateway::GuildAuditLogEntryCreate
//! [`GuildBanAdd`]: crate::event::gateway::GuildBanAdd
//! [`GuildBanRemove`]: crate::event::gateway::GuildBanRemove
//! [`GuildCreate`]: crate::event::gateway::GuildCreate
//! [`GuildEmojisUpdate`]: crate::event::gateway::GuildEmojisUpdate
//! [`GuildIntegrationsUpdate`]: crate::event::gateway::GuildIntegrationsUpdate
//! [`GuildMemberAdd`]: crate::event::gateway::GuildMemberAdd
//! [`GuildMemberRemove`]: crate::event::gateway::GuildMemberRemove
//! [`GuildMemberUpdate`]: crate::event::gateway::GuildMemberUpdate
//! [`GuildMembersChunk`]: crate::event::gateway::GuildMembersChunk
//! [`GuildRoleCreate`]: crate::event::gateway::GuildRoleCreate
//! [`GuildRoleDelete`]: crate::event::gateway::GuildRoleDelete
//! [`GuildRoleUpdate`]: crate::event::gateway::GuildRoleUpdate
//! [`GuildScheduledEvent`]: crate::guild::GuildScheduledEvent
//! [`GuildScheduledEventUserAdd`]: crate::event::gateway::GuildScheduledEventUserAdd
//! [`GuildScheduledEventUserRemove`]: crate::event::gateway::GuildScheduledEventUserRemove
//! [`GuildSoundboardSoundDelete`]: crate::event::gateway::GuildSoundboardSoundDelete
//! [`GuildSoundboardSoundsUpdate`]: crate::event::gateway::GuildSoundboardSoundsUpdate
//! [`GuildStickersUpdate`]: crate::event::gateway::GuildStickersUpdate
//! [`Hello`]: crate::event::gateway::Hello
//! [`Integration`]: crate::guild::Integration
//! [`IntegrationDelete`]: crate::event::gateway::IntegrationDelete
//! [`Interaction`]: crate::interaction::Interaction
//! [`InvalidSession`]: crate::event::gateway::InvalidSession
//! [`InviteCreate`]: crate::event::gateway::InviteCreate
//! [`InviteDelete`]: crate::event::gateway::InviteDelete
//! [`Message`]: crate::message::Message
//! [`MessageDelete`]: crate::event::gateway::MessageDelete
//! [`MessageDeleteBulk`]: crate::event::gateway::MessageDeleteBulk
//! [`MessagePollVoteAdd`]: crate::event::gateway::MessagePollVoteAdd
//! [`MessagePollVoteRemove`]: crate::event::gateway::MessagePollVoteRemove
//! [`MessageReactionAdd`]: crate::event::gateway::MessageReactionAdd
//! [`MessageReactionRemove`]: crate::event::gateway::MessageReactionRemove
//! [`MessageReactionRemoveAll`]: crate::event::gateway::MessageReactionRemoveAll
//! [`MessageReactionRemoveEmoji`]: crate::event::gateway::MessageReactionRemoveEmoji
//! [`Presence`]: crate::event::presence::Presence
//! [`RateLimited`]: crate::event::gateway::RateLimited
//! [`Ready`]: crate::event::gateway::Ready
//! [`SoundboardSound`]: crate::guild::SoundboardSound
//! [`SoundboardSounds`]: crate::event::gateway::SoundboardSounds
//! [`StageInstance`]: crate::guild::StageInstance
//! [`Subscription`]: crate::monetization::subscription::Subscription
//! [`ThreadListSync`]: crate::event::gateway::ThreadListSync
//! [`ThreadMembersUpdate`]: crate::event::gateway::ThreadMembersUpdate
//! [`ThreadMemberUpdate`]: crate::event::gateway::ThreadMemberUpdate
//! [`TypingStart`]: crate::event::gateway::TypingStart
//! [`UnavailableGuild`]: crate::event::gateway::UnavailableGuild
//! [`User`]: crate::user::User
//! [`VoiceChannelEffectSend`]: crate::event::gateway::VoiceChannelEffectSend
//! [`VoiceChannelStartTimeUpdate`]: crate::event::gateway::VoiceChannelStartTimeUpdate
//! [`VoiceChannelStatusUpdate`]: crate::event::gateway::VoiceChannelStatusUpdate
//! [`VoiceServerUpdate`]: crate::event::gateway::VoiceServerUpdate
//! [`VoiceState`]: crate::voice::VoiceState
//! [`WebhooksUpdate`]: crate::event::gateway::WebhooksUpdate

mod flags_new;

use serde::{Deserialize, Serialize};

use crate::application::{ApplicationFlags, ApplicationId};
use crate::auto_moderation::{
    AutoModerationAction, AutoModerationRuleId, AutoModerationTriggerType,
};
use crate::channel::ChannelId;
use crate::event::gateway::{GatewayOpcode, UnavailableGuild};
use crate::guild::GuildId;
use crate::message::MessageId;
use crate::user::{User, UserId};

/// The payload of the `HELLO` event.
///
/// See [Discord's Hello structure][hello] for details.
///
/// [hello]: https://docs.discord.com/developers/events/gateway-events#hello
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Hello {
    /// The interval in milliseconds an application should heartbeat with.
    pub heartbeat_interval: u64,
}

/// The payload of the `READY` event.
///
/// The event contains the initial state of a new session. [`guilds`](Self::guilds)
/// lists unavailable guilds that become available through `GUILD_CREATE` events
/// later. An application receives most of these fields once per session, and
/// the optional fields are omitted during serialization when `None`.
///
/// See [Discord's Ready event][ready] for details.
///
/// [ready]: https://docs.discord.com/developers/events/gateway-events#ready
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Ready {
    /// The API version of the Gateway the session was opened on.
    pub v: u64,

    /// Information about the current user, including email.
    pub user: User,

    /// The guilds the user is in, filtered to the session's shard when sharding.
    ///
    /// The guilds start out unavailable and are filled in by `GUILD_CREATE`
    /// events.
    pub guilds: Vec<UnavailableGuild>,

    /// The session ID, used to resume the connection.
    pub session_id: String,

    /// The Gateway URL to resume the connection on.
    pub resume_gateway_url: String,

    /// The session's shard, as `[shard_id, num_shards]`, when identifying with a
    /// shard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<[u64; 2]>,

    /// The partial application object of the current application.
    pub application: ReadyApplication,
}

/// The partial application object of a [`Ready`] event.
///
/// Discord documents this object as containing `id`, `flags`, and `flags_new`,
/// and the real payload can carry more fields, which are ignored during
/// deserialization. Because Discord does not publish a schema for the object,
/// only the documented subset is modeled: [`id`](Self::id) is required, while
/// the flags are optional so that a payload without them still decodes.
///
/// [`flags`](Self::flags) is the legacy integer bitfield and
/// [`flags_new`](Self::flags_new) is the same bitfield as a decimal string.
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ReadyApplication {
    /// The application's ID.
    pub id: ApplicationId,

    /// The application's legacy public flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ApplicationFlags>,

    /// The application's public flags as a decimal string.
    ///
    /// Serialized as a decimal string, matching Discord's application object.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flags_new")]
    pub flags_new: Option<ApplicationFlags>,
}

/// The payload of the `AUTO_MODERATION_ACTION_EXECUTION` event.
///
/// Sent when an Auto Moderation rule is triggered and an action is executed.
/// The optional fields are omitted during serialization when `None`, while
/// [`matched_keyword`](Self::matched_keyword) is nullable and serializes as
/// `null`.
///
/// [`content`](Self::content) and [`matched_content`](Self::matched_content)
/// require the `MESSAGE_CONTENT` intent and are otherwise absent, and
/// [`alert_system_message_id`](Self::alert_system_message_id) is only present
/// for the `SEND_ALERT_MESSAGE` action.
///
/// See [Discord's Auto Moderation Action Execution event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#auto-moderation-action-execution
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AutoModerationActionExecution {
    /// The ID of the guild the action was executed in.
    pub guild_id: GuildId,

    /// The action that was executed.
    pub action: AutoModerationAction,

    /// The ID of the rule the action belongs to.
    pub rule_id: AutoModerationRuleId,

    /// The trigger type of the rule that was triggered.
    pub rule_trigger_type: AutoModerationTriggerType,

    /// The ID of the user who generated the content that triggered the rule.
    pub user_id: UserId,

    /// The ID of the channel the user content was posted in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// The ID of the user message the content belongs to.
    ///
    /// Absent when Auto Moderation blocked the message or the content was not
    /// part of a message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,

    /// The ID of the system message Auto Moderation posted for this action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_system_message_id: Option<MessageId>,

    /// The user-generated text content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The keyword or phrase configured in the rule that triggered it.
    ///
    /// Nullable, so `None` is serialized as `null`.
    #[serde(default)]
    pub matched_keyword: Option<String>,

    /// The substring of the content that triggered the rule.
    ///
    /// Nullable, so `None` is serialized as `null`.
    #[serde(default)]
    pub matched_content: Option<String>,
}

/// The payload of the `RATE_LIMITED` event.
///
/// Sent when the application hits a Gateway rate limit. Discord currently
/// documents [`meta`](Self::meta) for opcode 8 only, so the metadata is modeled
/// as [`RequestGuildMemberRateLimitMetadata`].
///
/// See [Discord's Rate Limited event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#rate-limited
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct RateLimited {
    /// The opcode of the event that was rate limited.
    pub opcode: GatewayOpcode,

    /// The number of seconds to wait before submitting another request.
    pub retry_after: f64,

    /// Metadata for the event that was rate limited.
    pub meta: RequestGuildMemberRateLimitMetadata,
}

/// The rate limit metadata Discord sends for opcode 8.
///
/// See [Discord's Request Guild Member rate limit metadata][metadata] for
/// details.
///
/// [metadata]: https://docs.discord.com/developers/events/gateway-events#rate-limited-request-guild-member-rate-limit-metadata-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RequestGuildMemberRateLimitMetadata {
    /// The ID of the guild the request was for.
    pub guild_id: GuildId,

    /// The nonce of the rate limited request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
}

/// The payload of the `INVALID_SESSION` event.
///
/// The event's data is a single boolean stating whether the session may be
/// resumed. Discord sends `false` when it cannot resume, so an absent value is
/// modeled as `None` rather than as a resumable session.
///
/// See [Discord's Invalid Session event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#invalid-session
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InvalidSession {
    /// Whether the session may be resumed.
    #[serde(default)]
    pub resumable: Option<bool>,
}
