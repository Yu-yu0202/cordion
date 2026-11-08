/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Message-related Gateway events.
//!
//! `MESSAGE_CREATE` and `MESSAGE_UPDATE` carry a [`Message`], which models the
//! extra fields those events add (`guild_id`, `member`, `mentions`, and
//! `channel_type`), so no dedicated event type exists for them. Ephemeral
//! messages omit `guild_id` and `member`, because they are not sent through the
//! guild channel.
//!
//! [`Message`]: crate::message::Message

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::emoji::PartialEmoji;
use crate::guild::{GuildId, GuildMember};
use crate::message::MessageId;
use crate::user::UserId;

/// The payload of the `MESSAGE_DELETE` event.
///
/// See [Discord's Message Delete event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-delete
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageDelete {
    /// The ID of the deleted message.
    pub id: MessageId,

    /// The ID of the channel the message was in.
    pub channel_id: ChannelId,

    /// The ID of the guild, when the message was sent in one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

/// The payload of the `MESSAGE_DELETE_BULK` event.
///
/// See [Discord's Message Delete Bulk event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-delete-bulk
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageDeleteBulk {
    /// The IDs of the deleted messages.
    pub ids: Vec<MessageId>,

    /// The ID of the channel the messages were in.
    pub channel_id: ChannelId,

    /// The ID of the guild, when the messages were sent in one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

/// The payload of the `MESSAGE_REACTION_ADD` event.
///
/// [`message_author_id`](Self::message_author_id) and
/// [`burst_colors`](Self::burst_colors) are optional and are omitted during
/// serialization when `None`; [`guild_id`](Self::guild_id) and
/// [`member`](Self::member) are absent outside guilds.
///
/// See [Discord's Message Reaction Add event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-reaction-add
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageReactionAdd {
    /// The ID of the user who reacted.
    pub user_id: UserId,

    /// The ID of the channel the message is in.
    pub channel_id: ChannelId,

    /// The ID of the message that was reacted to.
    pub message_id: MessageId,

    /// The ID of the guild, when the reaction happened in one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The member who reacted, when the reaction happened in a guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,

    /// The emoji that was used.
    pub emoji: PartialEmoji,

    /// The ID of the user who authored the reacted message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_author_id: Option<UserId>,

    /// Whether this is a super-reaction.
    pub burst: bool,

    /// The colors used for the super-reaction animation, in `#rrggbb` format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub burst_colors: Option<Vec<String>>,

    /// The type of reaction, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ReactionType,
}

/// The payload of the `MESSAGE_REACTION_REMOVE` event.
///
/// See [Discord's Message Reaction Remove event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-reaction-remove
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageReactionRemove {
    /// The ID of the user who removed the reaction.
    pub user_id: UserId,

    /// The ID of the channel the message is in.
    pub channel_id: ChannelId,

    /// The ID of the message.
    pub message_id: MessageId,

    /// The ID of the guild, when the reaction happened in one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The emoji that was removed.
    pub emoji: PartialEmoji,

    /// Whether the removed reaction was a super-reaction.
    pub burst: bool,

    /// The type of reaction, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ReactionType,
}

/// The payload of the `MESSAGE_REACTION_REMOVE_ALL` event.
///
/// See [Discord's Message Reaction Remove All event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-reaction-remove-all
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageReactionRemoveAll {
    /// The ID of the channel the message is in.
    pub channel_id: ChannelId,

    /// The ID of the message.
    pub message_id: MessageId,

    /// The ID of the guild, when the message is in a guild channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

/// The payload of the `MESSAGE_REACTION_REMOVE_EMOJI` event.
///
/// See [Discord's Message Reaction Remove Emoji event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-reaction-remove-emoji
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageReactionRemoveEmoji {
    /// The ID of the channel the message is in.
    pub channel_id: ChannelId,

    /// The ID of the guild, when the message is in a guild channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The ID of the message.
    pub message_id: MessageId,

    /// The emoji whose reactions were removed.
    pub emoji: PartialEmoji,
}

/// The payload of the `MESSAGE_POLL_VOTE_ADD` event.
///
/// Discord sends one event per answer when a poll allows multiple selections.
///
/// See [Discord's Message Poll Vote Add event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-poll-vote-add
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessagePollVoteAdd {
    /// The ID of the user who voted.
    pub user_id: UserId,

    /// The ID of the channel the poll is in.
    pub channel_id: ChannelId,

    /// The ID of the message the poll is attached to.
    pub message_id: MessageId,

    /// The ID of the guild, when the poll is in a guild channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The ID of the answer that was voted for.
    pub answer_id: u64,
}

/// The payload of the `MESSAGE_POLL_VOTE_REMOVE` event.
///
/// Discord sends one event per answer when a poll allows multiple selections.
///
/// See [Discord's Message Poll Vote Remove event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#message-poll-vote-remove
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessagePollVoteRemove {
    /// The ID of the user who removed their vote.
    pub user_id: UserId,

    /// The ID of the channel the poll is in.
    pub channel_id: ChannelId,

    /// The ID of the message the poll is attached to.
    pub message_id: MessageId,

    /// The ID of the guild, when the poll is in a guild channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The ID of the answer whose vote was removed.
    pub answer_id: u64,
}

/// The payload of the `TYPING_START` event.
///
/// [`timestamp`](Self::timestamp) is in Unix seconds rather than the
/// milliseconds used by other Gateway timestamps.
///
/// See [Discord's Typing Start event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#typing-start
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TypingStart {
    /// The ID of the channel the user is typing in.
    pub channel_id: ChannelId,

    /// The ID of the guild, when the user is typing in a guild channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The ID of the user who started typing.
    pub user_id: UserId,

    /// When the user started typing, as Unix seconds.
    pub timestamp: u64,

    /// The member who started typing, when this happened in a guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,
}

/// The type of a message reaction.
///
/// Unrecognized values are preserved. See [Discord's reaction types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/resources/message#get-reactions-reaction-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ReactionType(u8);

impl ReactionType {
    /// A normal reaction.
    pub const NORMAL: Self = Self(0);

    /// A super-reaction, sent by a Nitro subscriber.
    pub const BURST: Self = Self(1);

    /// Returns the reaction type exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}
