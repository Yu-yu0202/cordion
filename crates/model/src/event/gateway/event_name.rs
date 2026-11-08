/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Gateway event names.
//!
//! Dispatch payloads are labeled with the name of the event they carry. The
//! names Discord documents are exposed as associated constants, and an
//! undocumented name is preserved so that a new event does not make a payload
//! undecodable.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// The name of a Gateway receive event.
///
/// `t` is only present on dispatch payloads; see [`GatewayPayload`](super::GatewayPayload).
/// Discord sends event names in upper case with underscores between words. Use
/// [`as_str`](Self::as_str) to compare a name against the constant of the event
/// it represents, or [`new`](Self::new) to build one from a string.
///
/// See [Discord's Gateway events][events] for the list of event names.
///
/// [events]: https://docs.discord.com/developers/events/gateway-events
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GatewayEventName(Cow<'static, str>);

impl GatewayEventName {
    /// Creates an owned event name from any string-like value.
    ///
    /// Prefer [`GatewayEventName::from`] with a `&'static str` to borrow instead
    /// of allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the event name exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl GatewayEventName {
    /// `APPLICATION_COMMAND_PERMISSIONS_UPDATE`
    pub const APPLICATION_COMMAND_PERMISSIONS_UPDATE: Self =
        Self(Cow::Borrowed("APPLICATION_COMMAND_PERMISSIONS_UPDATE"));

    /// `AUTO_MODERATION_ACTION_EXECUTION`
    pub const AUTO_MODERATION_ACTION_EXECUTION: Self =
        Self(Cow::Borrowed("AUTO_MODERATION_ACTION_EXECUTION"));

    /// `AUTO_MODERATION_RULE_CREATE`
    pub const AUTO_MODERATION_RULE_CREATE: Self =
        Self(Cow::Borrowed("AUTO_MODERATION_RULE_CREATE"));

    /// `AUTO_MODERATION_RULE_DELETE`
    pub const AUTO_MODERATION_RULE_DELETE: Self =
        Self(Cow::Borrowed("AUTO_MODERATION_RULE_DELETE"));

    /// `AUTO_MODERATION_RULE_UPDATE`
    pub const AUTO_MODERATION_RULE_UPDATE: Self =
        Self(Cow::Borrowed("AUTO_MODERATION_RULE_UPDATE"));

    /// `CHANNEL_CREATE`
    pub const CHANNEL_CREATE: Self = Self(Cow::Borrowed("CHANNEL_CREATE"));

    /// `CHANNEL_DELETE`
    pub const CHANNEL_DELETE: Self = Self(Cow::Borrowed("CHANNEL_DELETE"));

    /// `CHANNEL_INFO`
    pub const CHANNEL_INFO: Self = Self(Cow::Borrowed("CHANNEL_INFO"));

    /// `CHANNEL_PINS_UPDATE`
    pub const CHANNEL_PINS_UPDATE: Self = Self(Cow::Borrowed("CHANNEL_PINS_UPDATE"));

    /// `CHANNEL_UPDATE`
    pub const CHANNEL_UPDATE: Self = Self(Cow::Borrowed("CHANNEL_UPDATE"));

    /// `ENTITLEMENT_CREATE`
    pub const ENTITLEMENT_CREATE: Self = Self(Cow::Borrowed("ENTITLEMENT_CREATE"));

    /// `ENTITLEMENT_DELETE`
    pub const ENTITLEMENT_DELETE: Self = Self(Cow::Borrowed("ENTITLEMENT_DELETE"));

    /// `ENTITLEMENT_UPDATE`
    pub const ENTITLEMENT_UPDATE: Self = Self(Cow::Borrowed("ENTITLEMENT_UPDATE"));

    /// `GUILD_AUDIT_LOG_ENTRY_CREATE`
    pub const GUILD_AUDIT_LOG_ENTRY_CREATE: Self =
        Self(Cow::Borrowed("GUILD_AUDIT_LOG_ENTRY_CREATE"));

    /// `GUILD_BAN_ADD`
    pub const GUILD_BAN_ADD: Self = Self(Cow::Borrowed("GUILD_BAN_ADD"));

    /// `GUILD_BAN_REMOVE`
    pub const GUILD_BAN_REMOVE: Self = Self(Cow::Borrowed("GUILD_BAN_REMOVE"));

    /// `GUILD_CREATE`
    pub const GUILD_CREATE: Self = Self(Cow::Borrowed("GUILD_CREATE"));

    /// `GUILD_DELETE`
    pub const GUILD_DELETE: Self = Self(Cow::Borrowed("GUILD_DELETE"));

    /// `GUILD_EMOJIS_UPDATE`
    pub const GUILD_EMOJIS_UPDATE: Self = Self(Cow::Borrowed("GUILD_EMOJIS_UPDATE"));

    /// `GUILD_INTEGRATIONS_UPDATE`
    pub const GUILD_INTEGRATIONS_UPDATE: Self = Self(Cow::Borrowed("GUILD_INTEGRATIONS_UPDATE"));

    /// `GUILD_MEMBER_ADD`
    pub const GUILD_MEMBER_ADD: Self = Self(Cow::Borrowed("GUILD_MEMBER_ADD"));

    /// `GUILD_MEMBER_REMOVE`
    pub const GUILD_MEMBER_REMOVE: Self = Self(Cow::Borrowed("GUILD_MEMBER_REMOVE"));

    /// `GUILD_MEMBER_UPDATE`
    pub const GUILD_MEMBER_UPDATE: Self = Self(Cow::Borrowed("GUILD_MEMBER_UPDATE"));

    /// `GUILD_MEMBERS_CHUNK`
    pub const GUILD_MEMBERS_CHUNK: Self = Self(Cow::Borrowed("GUILD_MEMBERS_CHUNK"));

    /// `GUILD_ROLE_CREATE`
    pub const GUILD_ROLE_CREATE: Self = Self(Cow::Borrowed("GUILD_ROLE_CREATE"));

    /// `GUILD_ROLE_DELETE`
    pub const GUILD_ROLE_DELETE: Self = Self(Cow::Borrowed("GUILD_ROLE_DELETE"));

    /// `GUILD_ROLE_UPDATE`
    pub const GUILD_ROLE_UPDATE: Self = Self(Cow::Borrowed("GUILD_ROLE_UPDATE"));

    /// `GUILD_SCHEDULED_EVENT_CREATE`
    pub const GUILD_SCHEDULED_EVENT_CREATE: Self =
        Self(Cow::Borrowed("GUILD_SCHEDULED_EVENT_CREATE"));

    /// `GUILD_SCHEDULED_EVENT_DELETE`
    pub const GUILD_SCHEDULED_EVENT_DELETE: Self =
        Self(Cow::Borrowed("GUILD_SCHEDULED_EVENT_DELETE"));

    /// `GUILD_SCHEDULED_EVENT_UPDATE`
    pub const GUILD_SCHEDULED_EVENT_UPDATE: Self =
        Self(Cow::Borrowed("GUILD_SCHEDULED_EVENT_UPDATE"));

    /// `GUILD_SCHEDULED_EVENT_USER_ADD`
    pub const GUILD_SCHEDULED_EVENT_USER_ADD: Self =
        Self(Cow::Borrowed("GUILD_SCHEDULED_EVENT_USER_ADD"));

    /// `GUILD_SCHEDULED_EVENT_USER_REMOVE`
    pub const GUILD_SCHEDULED_EVENT_USER_REMOVE: Self =
        Self(Cow::Borrowed("GUILD_SCHEDULED_EVENT_USER_REMOVE"));

    /// `GUILD_SOUNDBOARD_SOUND_CREATE`
    pub const GUILD_SOUNDBOARD_SOUND_CREATE: Self =
        Self(Cow::Borrowed("GUILD_SOUNDBOARD_SOUND_CREATE"));

    /// `GUILD_SOUNDBOARD_SOUND_DELETE`
    pub const GUILD_SOUNDBOARD_SOUND_DELETE: Self =
        Self(Cow::Borrowed("GUILD_SOUNDBOARD_SOUND_DELETE"));

    /// `GUILD_SOUNDBOARD_SOUND_UPDATE`
    pub const GUILD_SOUNDBOARD_SOUND_UPDATE: Self =
        Self(Cow::Borrowed("GUILD_SOUNDBOARD_SOUND_UPDATE"));

    /// `GUILD_SOUNDBOARD_SOUNDS_UPDATE`
    pub const GUILD_SOUNDBOARD_SOUNDS_UPDATE: Self =
        Self(Cow::Borrowed("GUILD_SOUNDBOARD_SOUNDS_UPDATE"));

    /// `GUILD_STICKERS_UPDATE`
    pub const GUILD_STICKERS_UPDATE: Self = Self(Cow::Borrowed("GUILD_STICKERS_UPDATE"));

    /// `GUILD_UPDATE`
    pub const GUILD_UPDATE: Self = Self(Cow::Borrowed("GUILD_UPDATE"));

    /// `INTEGRATION_CREATE`
    pub const INTEGRATION_CREATE: Self = Self(Cow::Borrowed("INTEGRATION_CREATE"));

    /// `INTEGRATION_DELETE`
    pub const INTEGRATION_DELETE: Self = Self(Cow::Borrowed("INTEGRATION_DELETE"));

    /// `INTEGRATION_UPDATE`
    pub const INTEGRATION_UPDATE: Self = Self(Cow::Borrowed("INTEGRATION_UPDATE"));

    /// `INTERACTION_CREATE`
    pub const INTERACTION_CREATE: Self = Self(Cow::Borrowed("INTERACTION_CREATE"));

    /// `INVITE_CREATE`
    pub const INVITE_CREATE: Self = Self(Cow::Borrowed("INVITE_CREATE"));

    /// `INVITE_DELETE`
    pub const INVITE_DELETE: Self = Self(Cow::Borrowed("INVITE_DELETE"));

    /// `INVALID_SESSION`
    pub const INVALID_SESSION: Self = Self(Cow::Borrowed("INVALID_SESSION"));

    /// `MESSAGE_CREATE`
    pub const MESSAGE_CREATE: Self = Self(Cow::Borrowed("MESSAGE_CREATE"));

    /// `MESSAGE_DELETE`
    pub const MESSAGE_DELETE: Self = Self(Cow::Borrowed("MESSAGE_DELETE"));

    /// `MESSAGE_DELETE_BULK`
    pub const MESSAGE_DELETE_BULK: Self = Self(Cow::Borrowed("MESSAGE_DELETE_BULK"));

    /// `MESSAGE_POLL_VOTE_ADD`
    pub const MESSAGE_POLL_VOTE_ADD: Self = Self(Cow::Borrowed("MESSAGE_POLL_VOTE_ADD"));

    /// `MESSAGE_POLL_VOTE_REMOVE`
    pub const MESSAGE_POLL_VOTE_REMOVE: Self = Self(Cow::Borrowed("MESSAGE_POLL_VOTE_REMOVE"));

    /// `MESSAGE_REACTION_ADD`
    pub const MESSAGE_REACTION_ADD: Self = Self(Cow::Borrowed("MESSAGE_REACTION_ADD"));

    /// `MESSAGE_REACTION_REMOVE`
    pub const MESSAGE_REACTION_REMOVE: Self = Self(Cow::Borrowed("MESSAGE_REACTION_REMOVE"));

    /// `MESSAGE_REACTION_REMOVE_ALL`
    pub const MESSAGE_REACTION_REMOVE_ALL: Self =
        Self(Cow::Borrowed("MESSAGE_REACTION_REMOVE_ALL"));

    /// `MESSAGE_REACTION_REMOVE_EMOJI`
    pub const MESSAGE_REACTION_REMOVE_EMOJI: Self =
        Self(Cow::Borrowed("MESSAGE_REACTION_REMOVE_EMOJI"));

    /// `MESSAGE_UPDATE`
    pub const MESSAGE_UPDATE: Self = Self(Cow::Borrowed("MESSAGE_UPDATE"));

    /// `PRESENCE_UPDATE`
    pub const PRESENCE_UPDATE: Self = Self(Cow::Borrowed("PRESENCE_UPDATE"));

    /// `RATE_LIMITED`
    pub const RATE_LIMITED: Self = Self(Cow::Borrowed("RATE_LIMITED"));

    /// `READY`
    pub const READY: Self = Self(Cow::Borrowed("READY"));

    /// `RECONNECT`
    pub const RECONNECT: Self = Self(Cow::Borrowed("RECONNECT"));

    /// `RESUMED`
    pub const RESUMED: Self = Self(Cow::Borrowed("RESUMED"));

    /// `SOUNDBOARD_SOUNDS`
    pub const SOUNDBOARD_SOUNDS: Self = Self(Cow::Borrowed("SOUNDBOARD_SOUNDS"));

    /// `STAGE_INSTANCE_CREATE`
    pub const STAGE_INSTANCE_CREATE: Self = Self(Cow::Borrowed("STAGE_INSTANCE_CREATE"));

    /// `STAGE_INSTANCE_DELETE`
    pub const STAGE_INSTANCE_DELETE: Self = Self(Cow::Borrowed("STAGE_INSTANCE_DELETE"));

    /// `STAGE_INSTANCE_UPDATE`
    pub const STAGE_INSTANCE_UPDATE: Self = Self(Cow::Borrowed("STAGE_INSTANCE_UPDATE"));

    /// `SUBSCRIPTION_CREATE`
    pub const SUBSCRIPTION_CREATE: Self = Self(Cow::Borrowed("SUBSCRIPTION_CREATE"));

    /// `SUBSCRIPTION_DELETE`
    pub const SUBSCRIPTION_DELETE: Self = Self(Cow::Borrowed("SUBSCRIPTION_DELETE"));

    /// `SUBSCRIPTION_UPDATE`
    pub const SUBSCRIPTION_UPDATE: Self = Self(Cow::Borrowed("SUBSCRIPTION_UPDATE"));

    /// `THREAD_CREATE`
    pub const THREAD_CREATE: Self = Self(Cow::Borrowed("THREAD_CREATE"));

    /// `THREAD_DELETE`
    pub const THREAD_DELETE: Self = Self(Cow::Borrowed("THREAD_DELETE"));

    /// `THREAD_LIST_SYNC`
    pub const THREAD_LIST_SYNC: Self = Self(Cow::Borrowed("THREAD_LIST_SYNC"));

    /// `THREAD_MEMBERS_UPDATE`
    pub const THREAD_MEMBERS_UPDATE: Self = Self(Cow::Borrowed("THREAD_MEMBERS_UPDATE"));

    /// `THREAD_MEMBER_UPDATE`
    pub const THREAD_MEMBER_UPDATE: Self = Self(Cow::Borrowed("THREAD_MEMBER_UPDATE"));

    /// `THREAD_UPDATE`
    pub const THREAD_UPDATE: Self = Self(Cow::Borrowed("THREAD_UPDATE"));

    /// `TYPING_START`
    pub const TYPING_START: Self = Self(Cow::Borrowed("TYPING_START"));

    /// `USER_UPDATE`
    pub const USER_UPDATE: Self = Self(Cow::Borrowed("USER_UPDATE"));

    /// `VOICE_CHANNEL_EFFECT_SEND`
    pub const VOICE_CHANNEL_EFFECT_SEND: Self = Self(Cow::Borrowed("VOICE_CHANNEL_EFFECT_SEND"));

    /// `VOICE_CHANNEL_START_TIME_UPDATE`
    pub const VOICE_CHANNEL_START_TIME_UPDATE: Self =
        Self(Cow::Borrowed("VOICE_CHANNEL_START_TIME_UPDATE"));

    /// `VOICE_CHANNEL_STATUS_UPDATE`
    pub const VOICE_CHANNEL_STATUS_UPDATE: Self =
        Self(Cow::Borrowed("VOICE_CHANNEL_STATUS_UPDATE"));

    /// `VOICE_SERVER_UPDATE`
    pub const VOICE_SERVER_UPDATE: Self = Self(Cow::Borrowed("VOICE_SERVER_UPDATE"));

    /// `VOICE_STATE_UPDATE`
    pub const VOICE_STATE_UPDATE: Self = Self(Cow::Borrowed("VOICE_STATE_UPDATE"));

    /// `WEBHOOKS_UPDATE`
    pub const WEBHOOKS_UPDATE: Self = Self(Cow::Borrowed("WEBHOOKS_UPDATE"));
}

impl<T: Into<Cow<'static, str>>> From<T> for GatewayEventName {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
