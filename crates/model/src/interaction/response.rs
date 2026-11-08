/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The callback an application sends back for an interaction.
//!
//! [`InteractionResponse`] is the body of `POST
//! /interactions/{interaction.id}/{interaction.token}/callback`.
//! [`InteractionCallbackType`] selects the response kind, and
//! [`InteractionCallbackData`] carries the payload that kind accepts: the
//! message fields of [`InteractionCallbackMessage`], autocomplete choices, or
//! modal data. What Discord returns for the same call is modeled by
//! [`InteractionCallbackResponse`](super::InteractionCallbackResponse) in
//! [`callback`](super::callback).
//!
//! See [Discord's receiving and responding documentation][reference] for details.
//!
//! [reference]: https://docs.discord.com/developers/interactions/receiving-and-responding

use serde::{Deserialize, Serialize};

use crate::application_command::CommandOptionChoice;
use crate::component::Component;
use crate::message::{AllowedMentions, AttachmentRequest, Embed, MessageFlags, PollCreateRequest};

/// A response to an [`Interaction`](super::Interaction).
///
/// See [Discord's interaction response object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-response-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct InteractionResponse {
    /// The callback type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: InteractionCallbackType,

    /// The callback data, when the callback type carries a payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionCallbackData>,
}

/// The type of an [`InteractionResponse`].
///
/// Unrecognized values are preserved. See [Discord's interaction callback
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-response-object-interaction-callback-type
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InteractionCallbackType(u8);

impl InteractionCallbackType {
    /// Acknowledges a PING.
    pub const PONG: Self = Self(1);

    /// Responds with a message.
    pub const CHANNEL_MESSAGE_WITH_SOURCE: Self = Self(4);

    /// Acknowledges the interaction and shows a loading state until the
    /// response is edited.
    pub const DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE: Self = Self(5);

    /// Acknowledges a component interaction and edits the original message
    /// later without a loading state.
    pub const DEFERRED_UPDATE_MESSAGE: Self = Self(6);

    /// Edits the message the component was attached to.
    pub const UPDATE_MESSAGE: Self = Self(7);

    /// Responds to an autocomplete interaction with choices.
    pub const APPLICATION_COMMAND_AUTOCOMPLETE_RESULT: Self = Self(8);

    /// Responds with a modal.
    pub const MODAL: Self = Self(9);

    /// Responds with an upgrade button.
    ///
    /// Deprecated by Discord.
    pub const PREMIUM_REQUIRED: Self = Self(10);

    /// Launches the Activity associated with the application.
    pub const LAUNCH_ACTIVITY: Self = Self(12);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// The message data of an [`InteractionCallbackData`].
///
/// Every field is optional because the same payload is used for updating an
/// existing message, where only the changed fields are sent. Creating a new
/// message requires at least `content`, `embeds`, `components`, or `poll`.
///
/// Optional fields are omitted during serialization when `None`.
///
/// See [Discord's interaction callback data structure][data] for details.
///
/// [data]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct InteractionCallbackMessage {
    /// Whether the response is text-to-speech.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,

    /// The message content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// The message embeds, up to ten.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,

    /// Mention restrictions for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,

    /// Message flags.
    ///
    /// Only `SUPPRESS_EMBEDS`, `EPHEMERAL`, `IS_COMPONENTS_V2`,
    /// `IS_VOICE_MESSAGE`, and `SUPPRESS_NOTIFICATIONS` can be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// The message components.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// Partial attachments with filenames and descriptions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,

    /// Poll creation data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollCreateRequest>,
}

/// The data payload of an [`InteractionResponse`].
///
/// The correct variant depends on the callback type. Discord documents message
/// data for `CHANNEL_MESSAGE_WITH_SOURCE`, `DEFERRED_CHANNEL_MESSAGE_WITH_SOURCE`,
/// `DEFERRED_UPDATE_MESSAGE`, and `UPDATE_MESSAGE`; autocomplete choices for
/// `APPLICATION_COMMAND_AUTOCOMPLETE_RESULT`; and modal data for `MODAL`.
///
/// The message variant is intentionally last because every message field is
/// optional, making it the broadest shape. Its payload is boxed because it is
/// substantially larger than the other variants.
///
/// See [Discord's interaction callback data structure][data] for details.
///
/// [data]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-response-object-interaction-callback-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InteractionCallbackData {
    /// Autocomplete choices, up to 25.
    Autocomplete {
        /// The suggested choices.
        choices: Vec<CommandOptionChoice>,
    },

    /// Modal data.
    Modal {
        /// The modal's developer-defined identifier, between 1 and 100
        /// characters.
        custom_id: String,

        /// The modal title, up to 45 characters.
        title: String,

        /// The components that make up the modal, from one to five.
        components: Vec<Component>,
    },

    /// Message data.
    Message(Box<InteractionCallbackMessage>),
}
