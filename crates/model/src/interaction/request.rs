/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Bodies of the interaction followup message endpoints.
//!
//! Discord documents these endpoints as behaving like the equivalent webhook
//! endpoints, so they accept the same message fields as the message resource.
//! [`CreateFollowupMessageRequest`] is the body of `POST
//! /webhooks/{application.id}/{interaction.token}` and
//! [`EditFollowupMessageRequest`] is the body of `PATCH
//! /webhooks/{application.id}/{interaction.token}/messages/{message.id}`.
//!
//! See [Discord's receiving and responding documentation][reference] for details.
//!
//! [reference]: https://docs.discord.com/developers/interactions/receiving-and-responding

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::component::Component;
use crate::message::{AllowedMentions, AttachmentRequest, Embed, MessageFlags, PollCreateRequest};

/// The body of `POST /webhooks/{application.id}/{interaction.token}`.
///
/// Creates a followup message for an interaction. Discord documents this
/// endpoint as behaving like [Execute Webhook](crate::webhook), so the same
/// message fields are accepted; the `username`, `avatar_url`, and `thread_id`
/// parameters are the documented exceptions and are therefore not modeled.
/// All fields are optional and are omitted during serialization when `None`,
/// and at least one of `content`, `embeds`, `components`, `attachments`, or
/// `poll` must be present on the wire.
///
/// Interaction followup messages are additionally limited to five per
/// interaction when the interaction was authorized only for a user-installed
/// application.
///
/// See [Discord's create followup message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/interactions/receiving-and-responding#create-followup-message
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct CreateFollowupMessageRequest {
    /// The message contents, up to 2000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Whether the message is a text-to-speech message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,

    /// Embedded rich content, up to ten embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,

    /// Mention restrictions for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,

    /// The components to include with the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// Metadata for retained and newly uploaded attachments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,

    /// Message flags combined as a bitfield.
    ///
    /// Followup messages can use `EPHEMERAL` to stay visible to the invoking
    /// user only and `IS_COMPONENTS_V2` for a component-driven message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// The name of the thread to create.
    ///
    /// Requires the interaction's channel to be a forum or media channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<String>,

    /// The IDs of the tags to apply to the created thread.
    ///
    /// Requires the interaction's channel to be a forum or media channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<ChannelId>>,

    /// The poll to attach to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollCreateRequest>,
}

/// The body of `PATCH /webhooks/{application.id}/{interaction.token}/messages/{message.id}`.
///
/// Edits a followup message. Discord documents this endpoint as behaving like
/// [Edit Webhook Message](crate::webhook), so the same message fields are
/// accepted. All fields are optional and are omitted during serialization when
/// `None`.
///
/// See [Discord's edit followup message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/interactions/receiving-and-responding#edit-followup-message
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct EditFollowupMessageRequest {
    /// The new message contents, up to 2000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Embedded rich content, up to ten embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,

    /// Message flags combined as a bitfield.
    ///
    /// Discord only accepts [`MessageFlags::SUPPRESS_EMBEDS`] and
    /// [`MessageFlags::IS_COMPONENTS_V2`] here, and
    /// [`MessageFlags::IS_COMPONENTS_V2`] can only be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// Mention restrictions for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,

    /// The components to include with the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// The attachments to keep on the message after the edit, with their
    /// metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,

    /// The poll to attach to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollCreateRequest>,
}
