/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Request bodies for the Discord thread creation endpoints.
//!
//! [`StartThreadFromMessageRequest`] is the body of `POST
//! /channels/{channel.id}/messages/{message.id}/threads`.
//! [`StartThreadWithoutMessageRequest`] and
//! [`StartThreadInForumOrMediaThreadRequest`] are the two parameter tables of
//! `POST /channels/{channel.id}/threads`, and
//! [`ForumAndMediaThreadMessageParams`] is the nested message payload of the
//! latter.
//!
//! Every optional field is omitted when `None`, except that Discord documents
//! `rate_limit_per_user` as nullable, so `None` is serialized as `null` there.
//!
//! See [Discord's channel resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/channel

use serde::{Deserialize, Serialize};

use crate::channel::{ChannelType, ForumTagId};
use crate::component::Component;
use crate::message::{AllowedMentions, AttachmentRequest, Embed, MessageFlags};
use crate::sticker::StickerId;

/// The body of `POST /channels/{channel.id}/messages/{message.id}/threads`.
///
/// Creates a `PUBLIC_THREAD` in a `GUILD_TEXT` channel and an
/// `ANNOUNCEMENT_THREAD` in a `GUILD_ANNOUNCEMENT` channel. The created thread
/// shares its ID with the source message, so a message can only start one
/// thread.
///
/// See [Discord's start thread from message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#start-thread-from-message
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct StartThreadFromMessageRequest {
    /// The thread's name, between 1 and 100 characters.
    pub name: String,

    /// Minutes of inactivity before the thread stops appearing in the channel
    /// list.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`. Optional; omitted from
    /// the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<u16>,

    /// Seconds a user must wait before sending another message, from 0 through
    /// 21600.
    ///
    /// Bots and users with [`Permission::BYPASS_SLOWMODE`] are unaffected.
    /// Nullable: `None` is serialized as `null`.
    ///
    /// [`Permission::BYPASS_SLOWMODE`]: crate::permission::Permission::BYPASS_SLOWMODE
    #[serde(default)]
    pub rate_limit_per_user: Option<u64>,
}

/// The body of `POST /channels/{channel.id}/threads` in a text, announcement, or
/// forum parent channel.
///
/// [`kind`](Self::kind) currently defaults to `PRIVATE_THREAD` on Discord's side
/// to match the earliest published thread behavior; the field is optional here
/// so an application can state the type explicitly.
///
/// See [Discord's start thread without message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#start-thread-without-message
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct StartThreadWithoutMessageRequest {
    /// The thread's name, between 1 and 100 characters.
    pub name: String,

    /// Minutes of inactivity before the thread stops appearing in the channel
    /// list.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`. Optional; omitted from
    /// the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<u16>,

    /// The type of thread to create, serialized as `type`.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ChannelType>,

    /// Whether non-moderators can add other non-moderators to the thread.
    ///
    /// Only available when creating a private thread. Optional; omitted from the
    /// request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,

    /// Seconds a user must wait before sending another message, from 0 through
    /// 21600.
    ///
    /// Bots and users with [`Permission::BYPASS_SLOWMODE`] are unaffected.
    /// Nullable: `None` is serialized as `null`.
    ///
    /// [`Permission::BYPASS_SLOWMODE`]: crate::permission::Permission::BYPASS_SLOWMODE
    #[serde(default)]
    pub rate_limit_per_user: Option<u64>,
}

/// The body of `POST /channels/{channel.id}/threads` in a `GUILD_FORUM` or
/// `GUILD_MEDIA` channel.
///
/// This endpoint also posts the first message of the thread, so [`message`](Self::message)
/// must satisfy Discord's requirement that at least one of `content`, `embeds`,
/// `sticker_ids`, `components`, or a `files[n]` upload is present.
///
/// See [Discord's start thread in forum or media channel endpoint][endpoint] for
/// details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#start-thread-in-forum-or-media-channel
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct StartThreadInForumOrMediaThreadRequest {
    /// The thread's name, between 1 and 100 characters.
    pub name: String,

    /// Minutes of inactivity before the thread stops appearing in the channel
    /// list.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`. Optional; omitted from
    /// the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<u16>,

    /// Seconds a user must wait before sending another message, from 0 through
    /// 21600.
    ///
    /// Bots and users with [`Permission::BYPASS_SLOWMODE`] are unaffected.
    /// Nullable: `None` is serialized as `null`.
    ///
    /// [`Permission::BYPASS_SLOWMODE`]: crate::permission::Permission::BYPASS_SLOWMODE
    #[serde(default)]
    pub rate_limit_per_user: Option<u64>,

    /// The contents of the first message in the thread.
    ///
    /// Discord requires this field on this endpoint, and requires the nested
    /// params to carry at least one of `content`, `embeds`, `sticker_ids`,
    /// `components`, or a `files[n]` upload.
    pub message: ForumAndMediaThreadMessageParams,

    /// The IDs of the tags applied to the thread, up to five.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<ForumTagId>>,
}

/// The nested `message` payload of a [`StartThreadInForumOrMediaThreadRequest`].
///
/// Discord requires at least one of [`content`](Self::content),
/// [`embeds`](Self::embeds), [`sticker_ids`](Self::sticker_ids),
/// [`components`](Self::components), or a `files[n]` upload; this type models
/// only the JSON body, so a request whose sole payload is a file upload leaves
/// these fields empty. Every field is optional and is omitted when `None`.
///
/// See [Discord's forum and media thread message params object][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#start-thread-in-forum-or-media-channel-forum-and-media-thread-message-params-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ForumAndMediaThreadMessageParams {
    /// The message's contents, up to 2000 characters.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The message's `rich` embeds, up to ten.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,

    /// Mention restrictions for the message.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,

    /// The message's components.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// The IDs of up to three guild stickers to send with the message.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sticker_ids: Option<Vec<StickerId>>,

    /// Metadata for the attachments referenced by the message.
    ///
    /// The files themselves are uploaded as `multipart/form-data` parts, which
    /// this JSON model does not carry. Optional; omitted from the request when
    /// `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,

    /// The message's flags.
    ///
    /// Discord only accepts `SUPPRESS_EMBEDS`, `SUPPRESS_NOTIFICATIONS`, and
    /// `IS_COMPONENTS_V2` here. Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
}
