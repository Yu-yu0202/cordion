/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord message resource.
//!
//! [`Message`] models the message object returned by REST endpoints and Gateway
//! events, including components, embeds, attachments, polls, references, and
//! interaction metadata. The object is frequently delivered with fields reduced
//! or omitted when the application lacks the `MESSAGE_CONTENT` privileged
//! intent; in that case `content`, `embeds`, `attachments`, and `components`
//! are empty and `poll` is absent. The channel pins endpoint response is
//! modeled by [`GetChannelPinsResponse`], and the JSON bodies of the message
//! endpoints by [`CreateMessageRequest`], [`EditMessageRequest`], and
//! [`BulkDeleteMessagesRequest`].
//!
//! See [Discord's message resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/message

mod attachment;
mod embed;
mod misc;
mod poll;
mod reaction;
mod reference;
mod request;

use std::time::SystemTime;

pub use attachment::{
    Attachment, AttachmentFlags, AttachmentId, AttachmentRequest, AttachmentRequestId,
};
use bitflags::bitflags;
use cordion_macro::IdTarget;
pub use embed::{
    Embed, EmbedAuthor, EmbedField, EmbedFlags, EmbedFooter, EmbedImage, EmbedMediaFlags,
    EmbedProvider, EmbedVideo,
};
pub use misc::{
    AllowedMentionType, AllowedMentions, BaseTheme, ChannelMention, MessageActivity,
    MessageActivityType, MessageCall, RoleSubscriptionData, SharedClientTheme,
};
pub use poll::{
    Poll, PollAnswer, PollAnswerCount, PollCreateRequest, PollLayoutType, PollMedia, PollResults,
};
pub use reaction::{Reaction, ReactionCountDetails};
pub use reference::{MessageReference, MessageReferenceType, MessageSnapshot, PartialMessage};
pub use request::{BulkDeleteMessagesRequest, CreateMessageRequest, EditMessageRequest};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::application::{ApplicationId, PartialApplication};
use crate::channel::{Channel, ChannelId, ChannelType};
use crate::component::Component;
use crate::guild::{GuildId, PartialGuildMember};
use crate::id::{Id, Identifiable};
use crate::interaction::{MessageInteraction, MessageInteractionMetadata, ResolvedData};
use crate::role::RoleId;
use crate::sticker::{Sticker, StickerItem};
use crate::user::User;
use crate::webhook::WebhookId;

/// The ID of a [`Message`].
pub type MessageId = Id<Message>;

/// A message sent in a channel.
///
/// The required fields follow Discord's message object. Fields that Discord
/// documents as nullable are represented as `Option` and serialize as `null`
/// where the protocol keeps the field present; optional fields are omitted when
/// `None`. Discord's own examples omit some fields that its field table marks
/// required, so descriptive metadata is modeled with `Option` where the wire
/// can legitimately be narrower.
///
/// The `MESSAGE_CREATE` and `MESSAGE_UPDATE` gateway events add
/// [`guild_id`](Self::guild_id), [`member`](Self::member), and
/// [`channel_type`](Self::channel_type) to the message object; they are modeled
/// here and are absent from REST payloads.
///
/// See [Discord's message object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-object
#[derive(Clone, PartialEq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Message {
    /// The message's ID.
    pub id: MessageId,

    /// The ID of the channel the message was sent in.
    pub channel_id: ChannelId,

    /// The message's author.
    ///
    /// For webhook messages this object contains the webhook's ID, username,
    /// and avatar rather than a valid user account; check
    /// [`webhook_id`](Self::webhook_id).
    pub author: User,

    /// The message contents.
    ///
    /// Empty when the application lacks the `MESSAGE_CONTENT` intent.
    pub content: String,

    /// When the message was sent.
    #[serde(with = "crate::timestamp")]
    pub timestamp: SystemTime,

    /// When the message was last edited, or `None` if it was never edited.
    ///
    /// Discord sends `null`; serialization preserves that.
    #[serde(with = "crate::timestamp::option")]
    pub edited_timestamp: Option<SystemTime>,

    /// Whether this is a text-to-speech message.
    pub tts: bool,

    /// Whether this message mentions everyone.
    pub mention_everyone: bool,

    /// Users specifically mentioned in the message.
    ///
    /// In gateway message events, each user object may carry an additional
    /// partial guild member object; see [`User::member`].
    pub mentions: Vec<User>,

    /// IDs of roles specifically mentioned in the message.
    pub mention_roles: Vec<RoleId>,

    /// Channels specifically mentioned in the message.
    ///
    /// Only visible textual channels in public guilds and crossposted messages
    /// currently populate this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_channels: Option<Vec<ChannelMention>>,

    /// Files attached to the message that are not referenced in embeds or
    /// components.
    pub attachments: Vec<Attachment>,

    /// Embedded content.
    pub embeds: Vec<Embed>,

    /// Reactions to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<Reaction>>,

    /// The nonce used to validate that the message was sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<MessageNonce>,

    /// Whether the message is pinned.
    pub pinned: bool,

    /// The ID of the webhook that generated the message, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<WebhookId>,

    /// The message type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: MessageType,

    /// Rich Presence activity data for activity-related chat embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,

    /// The partial application for Rich Presence-related chat embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<PartialApplication>,

    /// The ID of the application that sent the message, when it is an
    /// interaction response or application-owned webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    /// Message flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// The source of a crosspost, channel follow, pin, reply, or forward.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,

    /// Immutable snapshots associated with a forwarded message.
    ///
    /// Discord currently sends at most one snapshot and limits nesting depth to
    /// one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_snapshots: Option<Vec<MessageSnapshot>>,

    /// The message associated with [`message_reference`](Self::message_reference).
    ///
    /// Discord omits the field when it did not attempt to fetch the referenced
    /// message and sends `null` when the referenced message was deleted. Both
    /// cases decode as `None`, so this representation does not preserve the
    /// distinction; compare the surrounding message type and reference when it
    /// matters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub referenced_message: Option<Box<Message>>,

    /// Metadata about the interaction that produced the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_metadata: Option<MessageInteractionMetadata>,

    /// Deprecated metadata about the interaction that produced the message.
    ///
    /// Discord recommends [`interaction_metadata`](Self::interaction_metadata);
    /// this field is preserved because older payloads still contain it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction: Option<MessageInteraction>,

    /// The thread started from this message, including its thread member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread: Option<Channel>,

    /// Components attached to the message.
    ///
    /// Empty when the application lacks the `MESSAGE_CONTENT` intent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// Stickers sent with the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_items: Option<Vec<StickerItem>>,

    /// Deprecated full sticker objects sent with the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Sticker>>,

    /// The approximate position of the message in a thread.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u64>,

    /// Role subscription purchase or renewal data for
    /// [`ROLE_SUBSCRIPTION_PURCHASE`](MessageType::ROLE_SUBSCRIPTION_PURCHASE)
    /// messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_subscription_data: Option<RoleSubscriptionData>,

    /// Users, members, channels, roles, and messages referenced in the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedData>,

    /// A poll attached to the message.
    ///
    /// Discord omits this field when the application lacks the
    /// `MESSAGE_CONTENT` intent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    /// The call associated with the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call: Option<MessageCall>,

    /// A custom client-side theme shared via the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_client_theme: Option<SharedClientTheme>,

    /// The ID of the guild the message was sent in.
    ///
    /// Only present in gateway message events (`MESSAGE_CREATE` and
    /// `MESSAGE_UPDATE`) and omitted for ephemeral messages, which are not sent
    /// through the guild channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The message author's guild member data.
    ///
    /// Only present in gateway message events, and missing there for ephemeral
    /// messages and messages sent by webhooks. This is a partial guild member
    /// object, so fields that Discord omits in this context decode as `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialGuildMember>,

    /// The type of the channel the message was sent in.
    ///
    /// Only present in gateway message events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<ChannelType>,
}

impl Identifiable for Message {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// A message type.
///
/// Unrecognized values are preserved. See [Discord's message types][types] for
/// the current list.
///
/// [types]: https://docs.discord.com/developers/resources/message#message-object-message-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MessageType(u8);

impl MessageType {
    /// A regular user or bot message.
    pub const DEFAULT: Self = Self(0);

    /// A recipient was added to a group DM or thread.
    pub const RECIPIENT_ADD: Self = Self(1);

    /// A recipient was removed from a group DM.
    pub const RECIPIENT_REMOVE: Self = Self(2);

    /// A call was started.
    pub const CALL: Self = Self(3);

    /// A channel was renamed.
    pub const CHANNEL_NAME_CHANGE: Self = Self(4);

    /// A channel icon was changed.
    pub const CHANNEL_ICON_CHANGE: Self = Self(5);

    /// A message was pinned.
    pub const CHANNEL_PINNED_MESSAGE: Self = Self(6);

    /// A user joined the guild.
    pub const USER_JOIN: Self = Self(7);

    /// A guild was boosted.
    pub const GUILD_BOOST: Self = Self(8);

    /// A guild reached boost tier 1.
    pub const GUILD_BOOST_TIER_1: Self = Self(9);

    /// A guild reached boost tier 2.
    pub const GUILD_BOOST_TIER_2: Self = Self(10);

    /// A guild reached boost tier 3.
    pub const GUILD_BOOST_TIER_3: Self = Self(11);

    /// A channel was followed into another channel.
    pub const CHANNEL_FOLLOW_ADD: Self = Self(12);

    /// A guild was disqualified from discovery.
    pub const GUILD_DISCOVERY_DISQUALIFIED: Self = Self(14);

    /// A guild was requalified for discovery.
    pub const GUILD_DISCOVERY_REQUALIFIED: Self = Self(15);

    /// A guild received the initial discovery grace-period warning.
    pub const GUILD_DISCOVERY_GRACE_PERIOD_INITIAL_WARNING: Self = Self(16);

    /// A guild received the final discovery grace-period warning.
    pub const GUILD_DISCOVERY_GRACE_PERIOD_FINAL_WARNING: Self = Self(17);

    /// A thread was created.
    pub const THREAD_CREATED: Self = Self(18);

    /// A reply to another message.
    pub const REPLY: Self = Self(19);

    /// A chat input command response.
    pub const CHAT_INPUT_COMMAND: Self = Self(20);

    /// The first message in a thread created from a message.
    pub const THREAD_STARTER_MESSAGE: Self = Self(21);

    /// A guild invite reminder.
    pub const GUILD_INVITE_REMINDER: Self = Self(22);

    /// A context menu command response.
    pub const CONTEXT_MENU_COMMAND: Self = Self(23);

    /// An auto moderation action was taken.
    pub const AUTO_MODERATION_ACTION: Self = Self(24);

    /// A role subscription was purchased or renewed.
    pub const ROLE_SUBSCRIPTION_PURCHASE: Self = Self(25);

    /// An interaction premium upsell.
    pub const INTERACTION_PREMIUM_UPSELL: Self = Self(26);

    /// A stage instance started.
    pub const STAGE_START: Self = Self(27);

    /// A stage instance ended.
    pub const STAGE_END: Self = Self(28);

    /// A user became a stage speaker.
    pub const STAGE_SPEAKER: Self = Self(29);

    /// A stage topic changed.
    pub const STAGE_TOPIC: Self = Self(31);

    /// A guild application premium subscription.
    pub const GUILD_APPLICATION_PREMIUM_SUBSCRIPTION: Self = Self(32);

    /// A guild enabled incident alert mode.
    pub const GUILD_INCIDENT_ALERT_MODE_ENABLED: Self = Self(36);

    /// A guild disabled incident alert mode.
    pub const GUILD_INCIDENT_ALERT_MODE_DISABLED: Self = Self(37);

    /// A guild reported a raid.
    pub const GUILD_INCIDENT_REPORT_RAID: Self = Self(38);

    /// A guild reported a false alarm.
    pub const GUILD_INCIDENT_REPORT_FALSE_ALARM: Self = Self(39);

    /// A purchase notification.
    pub const PURCHASE_NOTIFICATION: Self = Self(44);

    /// A poll result message.
    pub const POLL_RESULT: Self = Self(46);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

bitflags! {
    /// Flags attached to a [`Message`].
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// message flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/message#message-object-message-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct MessageFlags: u64 {
        /// No message flags are set.
        const NONE = 0;

        /// The message was published to subscribed channels.
        const CROSSPOSTED = 1 << 0;

        /// The message originated from another channel.
        const IS_CROSSPOST = 1 << 1;

        /// Embeds should not be included when serializing the message.
        const SUPPRESS_EMBEDS = 1 << 2;

        /// The source message for this crosspost was deleted.
        const SOURCE_MESSAGE_DELETED = 1 << 3;

        /// The message came from the urgent message system.
        const URGENT = 1 << 4;

        /// The message has a thread with the same ID.
        const HAS_THREAD = 1 << 5;

        /// The message is only visible to the user who invoked the interaction.
        const EPHEMERAL = 1 << 6;

        /// The interaction response is still loading.
        const LOADING = 1 << 7;

        /// The message failed to mention some roles and add their members to
        /// the thread.
        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;

        /// The message does not trigger push or desktop notifications.
        const SUPPRESS_NOTIFICATIONS = 1 << 12;

        /// The message is a voice message.
        const IS_VOICE_MESSAGE = 1 << 13;

        /// The message has a forward snapshot.
        const HAS_SNAPSHOT = 1 << 14;

        /// The message uses a fully component-driven layout.
        ///
        /// Once set, Discord does not allow this flag to be removed.
        const IS_COMPONENTS_V2 = 1 << 15;
    }
}

impl Serialize for MessageFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for MessageFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

/// A user or string nonce attached to a [`Message`].
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageNonce {
    /// A string nonce.
    String(String),

    /// An integer nonce.
    Integer(i64),
}

/// A pinned message and the time it was pinned.
///
/// See [Discord's message pin object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-pin-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MessagePin {
    /// When the message was pinned.
    #[serde(with = "crate::timestamp")]
    pub pinned_at: SystemTime,

    /// The pinned message.
    pub message: Box<Message>,
}

/// The response of `GET /channels/{channel.id}/messages/pins`.
///
/// [`has_more`](Self::has_more) states whether more pins exist before the
/// [`pinned_at`](MessagePin::pinned_at) of the last returned item, which is the
/// value to pass as the `before` parameter of the next request.
///
/// See [Discord's Get Channel Pins response structure][response] for details.
///
/// [response]: https://docs.discord.com/developers/resources/message#get-channel-pins
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct GetChannelPinsResponse {
    /// The pins of the channel, at most 50 per page.
    pub items: Vec<MessagePin>,

    /// Whether the channel has more pins before the returned page.
    pub has_more: bool,
}
