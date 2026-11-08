/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Message reference and snapshot types.
//!
//! [`MessageReference`] models the source of a crosspost, channel follow, pin,
//! reply, or forward, and [`MessageReferenceType`] its kind. A forwarded
//! message carries immutable [`MessageSnapshot`]s, whose [`PartialMessage`]
//! subset is the minimal message copy Discord currently captures.
//!
//! See [Discord's message reference structure][reference], [Discord's message
//! reference types][types], and [Discord's message snapshot object][snapshot]
//! for details.
//!
//! [reference]: https://docs.discord.com/developers/resources/message#message-reference-structure
//! [types]: https://docs.discord.com/developers/resources/message#message-reference-types
//! [snapshot]: https://docs.discord.com/developers/resources/message#message-snapshot-object

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::component::Component;
use crate::guild::GuildId;
use crate::message::{Attachment, Embed, MessageFlags, MessageId, MessageType};
use crate::role::RoleId;
use crate::sticker::{Sticker, StickerItem};
use crate::user::User;

/// The source of a message reference.
///
/// Optional fields decode as `None` when absent or omitted. `channel_id` is
/// optional when creating a reply but is always present in received data.
///
/// See [Discord's message reference structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-reference-structure
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageReference {
    /// The reference type, serialized as `type`.
    ///
    /// When absent, Discord treats the reference as
    /// [`DEFAULT`](MessageReferenceType::DEFAULT).
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<MessageReferenceType>,

    /// The ID of the originating message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,

    /// The ID of the originating message's channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// The ID of the originating message's guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// Whether sending should fail if the referenced message does not exist.
    ///
    /// Discord defaults this to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}

/// The type of a [`MessageReference`].
///
/// Unrecognized values are preserved. See [Discord's message reference
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/resources/message#message-reference-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MessageReferenceType(u8);

impl MessageReferenceType {
    /// A standard reply reference.
    pub const DEFAULT: Self = Self(0);

    /// A forward reference pointing to a snapshot.
    pub const FORWARD: Self = Self(1);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A snapshot of a forwarded message.
///
/// See [Discord's message snapshot object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-snapshot-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MessageSnapshot {
    /// The minimal message copy captured when the forward was created.
    pub message: PartialMessage,
}

/// The minimal message subset stored in a [`MessageSnapshot`].
///
/// Discord documents these fields as the current snapshot subset and excludes
/// the author. Collections default to empty when absent so that a narrower
/// snapshot does not fail to decode. This is not a general partial message type
/// for Gateway `MESSAGE_UPDATE` events.
///
/// See [Discord's message snapshot structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-snapshot-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PartialMessage {
    /// The message type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: MessageType,

    /// The message contents.
    pub content: String,

    /// Embedded content.
    #[serde(default)]
    pub embeds: Vec<Embed>,

    /// Attached files.
    #[serde(default)]
    pub attachments: Vec<Attachment>,

    /// When the message was sent.
    #[serde(with = "crate::timestamp")]
    pub timestamp: SystemTime,

    /// When the message was edited, or `None`.
    #[serde(default, with = "crate::timestamp::option")]
    pub edited_timestamp: Option<SystemTime>,

    /// Message flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// Users specifically mentioned in the message.
    #[serde(default)]
    pub mentions: Vec<User>,

    /// IDs of roles specifically mentioned in the message.
    #[serde(default)]
    pub mention_roles: Vec<RoleId>,

    /// Deprecated full sticker objects sent with the message.
    #[serde(default)]
    pub stickers: Vec<Sticker>,

    /// Sticker items sent with the message.
    #[serde(default)]
    pub sticker_items: Vec<StickerItem>,

    /// Components attached to the message.
    #[serde(default)]
    pub components: Vec<Component>,
}
