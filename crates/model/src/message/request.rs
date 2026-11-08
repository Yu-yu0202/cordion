/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Request bodies for the message endpoints.
//!
//! [`CreateMessageRequest`] is the body of `POST
//! /channels/{channel.id}/messages`, [`EditMessageRequest`] the body of `PATCH
//! /channels/{channel.id}/messages/{message.id}`, and
//! [`BulkDeleteMessagesRequest`] the body of `POST
//! /channels/{channel.id}/messages/bulk-delete`.
//!
//! The `files[n]` contents and the `payload_json` form field belong to a
//! `multipart/form-data` body, which these JSON models do not carry; the
//! attachment metadata carried alongside the files is modeled by
//! [`AttachmentRequest`](crate::message::AttachmentRequest).
//!
//! See [Discord's create message endpoint][create], [Discord's edit message
//! endpoint][edit], and [Discord's bulk delete messages endpoint][delete] for
//! details.
//!
//! [create]: https://docs.discord.com/developers/resources/message#create-message
//! [edit]: https://docs.discord.com/developers/resources/message#edit-message
//! [delete]: https://docs.discord.com/developers/resources/message#bulk-delete-messages

use serde::{Deserialize, Serialize};

use crate::component::Component;
use crate::message::{
    AllowedMentions, AttachmentRequest, Embed, MessageFlags, MessageId, MessageNonce,
    MessageReference, PollCreateRequest, SharedClientTheme,
};
use crate::sticker::StickerId;

/// The body of `POST /channels/{channel.id}/messages`.
///
/// Discord requires at least one of [`content`](Self::content),
/// [`embeds`](Self::embeds), [`sticker_ids`](Self::sticker_ids),
/// [`components`](Self::components), [`poll`](Self::poll),
/// [`shared_client_theme`](Self::shared_client_theme), or a `files[n]` upload,
/// except when [`message_reference`](Self::message_reference) makes the message
/// a reply or a forward. The `files[n]` contents and the `payload_json` form
/// field belong to a `multipart/form-data` body, which this JSON model does not
/// carry; [`attachments`](Self::attachments) carries their metadata.
///
/// Every field is optional and is omitted when `None`.
///
/// See [Discord's create message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/message#create-message
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct CreateMessageRequest {
    /// The message's contents, up to 2000 characters.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// A value used to verify the message was sent, up to 25 characters.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<MessageNonce>,

    /// Whether the message is a text-to-speech message.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,

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

    /// The reference that makes this message a reply or a forward.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,

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

    /// Metadata for the attachments sent with the message.
    ///
    /// The files themselves are uploaded as `multipart/form-data` parts, which
    /// this JSON model does not carry. Optional; omitted from the request when
    /// `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,

    /// The message's flags.
    ///
    /// Discord only accepts `SUPPRESS_EMBEDS`, `SUPPRESS_NOTIFICATIONS`,
    /// `IS_VOICE_MESSAGE`, and `IS_COMPONENTS_V2`; once `IS_COMPONENTS_V2` is
    /// set it cannot be removed, and the message may then only carry
    /// [`components`](Self::components). Optional; omitted from the request when
    /// `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// Whether an existing message with the same author and
    /// [`nonce`](Self::nonce) should be returned instead of creating a new
    /// message.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_nonce: Option<bool>,

    /// The poll to attach to the message.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollCreateRequest>,

    /// The custom client-side theme to share with the message.
    ///
    /// Optional; omitted from the request when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_client_theme: Option<SharedClientTheme>,
}

/// The body of `PATCH /channels/{channel.id}/messages/{message.id}`.
///
/// Discord documents every parameter of this endpoint as optional and nullable,
/// and only the original author may edit `content`, `embeds`, `flags`, and
/// `components`. A nullable field serializes as `null` when `None`: that resets
/// the value, which is how the `IS_COMPONENTS_V2` layout requires
/// [`content`](Self::content) to be cleared and [`embeds`](Self::embeds) to be
/// emptied. The `files[n]` contents and the `payload_json` form field belong to
/// a `multipart/form-data` body, which this JSON model does not carry.
///
/// See [Discord's edit message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/message#edit-message
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct EditMessageRequest {
    /// The message's contents, up to 2000 characters.
    ///
    /// Nullable: `None` is serialized as `null`, which clears the content.
    #[serde(default)]
    pub content: Option<String>,

    /// The message's `rich` embeds, up to ten.
    ///
    /// Nullable: `None` is serialized as `null`; send an empty vector to clear
    /// the embeds of a message that uses the `IS_COMPONENTS_V2` layout.
    #[serde(default)]
    pub embeds: Option<Vec<Embed>>,

    /// The message's flags.
    ///
    /// Discord only accepts `SUPPRESS_EMBEDS`, which can be both set and unset,
    /// and `IS_COMPONENTS_V2`, which can only be set. Send every bit the message
    /// should keep. Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub flags: Option<MessageFlags>,

    /// Mention restrictions for the message.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub allowed_mentions: Option<AllowedMentions>,

    /// The message's components.
    ///
    /// Nullable: `None` is serialized as `null`.
    #[serde(default)]
    pub components: Option<Vec<Component>>,

    /// The attachments the message should keep after the edit, with their
    /// metadata.
    ///
    /// Discord requires the array to list both retained and newly uploaded
    /// attachments. The files themselves are uploaded as `multipart/form-data`
    /// parts, which this JSON model does not carry. Nullable: `None` is
    /// serialized as `null`.
    #[serde(default)]
    pub attachments: Option<Vec<AttachmentRequest>>,
}

/// The body of `POST /channels/{channel.id}/messages/bulk-delete`.
///
/// Discord deletes 2 to 100 messages that are younger than two weeks per
/// request; duplicate IDs are rejected. Only usable on guild channels, and
/// requires the `MANAGE_MESSAGES` permission.
///
/// See [Discord's bulk delete messages endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/message#bulk-delete-messages
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BulkDeleteMessagesRequest {
    /// The IDs of the messages to delete, at least 2 and at most 100.
    pub messages: Vec<MessageId>,
}
