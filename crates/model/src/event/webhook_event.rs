/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Webhook event payloads delivered to an application's endpoint.
//!
//! Webhook events are one-way HTTP events Discord sends to an application's
//! Event Webhooks URL. Unlike Gateway events they are not guaranteed to be
//! realtime or ordered, and the application acknowledges them with a `204`
//! response.
//!
//! [`WebhookEventPayload`] is the outer envelope, [`WebhookEventBody`] the inner
//! event body, and [`WebhookEventBodyData`] the type-specific data. Data shapes
//! that are not modeled are preserved as JSON so that a new event type or an
//! unavailable event does not make a payload undecodable.
//!
//! See [Discord's webhook events documentation][events] for details.
//!
//! [events]: https://docs.discord.com/developers/events/webhook-events

mod application;
mod message;

use std::borrow::Cow;
use std::time::SystemTime;

pub use application::{ApplicationAuthorized, ApplicationDeauthorized};
pub use message::{
    LobbyMessageDelete, SocialSdkDirectMessage, SocialSdkMessage, WebhookEventLobbyMessage,
};
use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::monetization::entitlement::Entitlement;

/// The type of a [`WebhookEventPayload`] envelope.
///
/// Unrecognized values are preserved. See [Discord's webhook types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/events/webhook-events#webhook-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct WebhookEventType(u8);

impl WebhookEventType {
    /// A ping sent to verify the application's endpoint.
    pub const PING: Self = Self(0);

    /// A webhook event carrying an event body.
    pub const EVENT: Self = Self(1);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// Wraps a raw integer value without validating it.
    #[inline]
    #[must_use]
    pub const fn from_raw(value: u8) -> Self {
        Self(value)
    }
}

/// The type of a [`WebhookEventBody`].
///
/// Documented values are exposed as associated constants, and an undocumented
/// value is preserved so that a new event type does not make a payload
/// undecodable. See [Discord's webhook event types][types] for details.
///
/// [types]: https://docs.discord.com/developers/events/webhook-events#event-types
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct WebhookEventBodyType(Cow<'static, str>);

impl WebhookEventBodyType {
    /// `APPLICATION_AUTHORIZED`
    pub const APPLICATION_AUTHORIZED: Self = Self(Cow::Borrowed("APPLICATION_AUTHORIZED"));

    /// `APPLICATION_DEAUTHORIZED`
    pub const APPLICATION_DEAUTHORIZED: Self = Self(Cow::Borrowed("APPLICATION_DEAUTHORIZED"));

    /// `ENTITLEMENT_CREATE`
    pub const ENTITLEMENT_CREATE: Self = Self(Cow::Borrowed("ENTITLEMENT_CREATE"));

    /// `ENTITLEMENT_UPDATE`
    pub const ENTITLEMENT_UPDATE: Self = Self(Cow::Borrowed("ENTITLEMENT_UPDATE"));

    /// `ENTITLEMENT_DELETE`
    pub const ENTITLEMENT_DELETE: Self = Self(Cow::Borrowed("ENTITLEMENT_DELETE"));

    /// `QUEST_USER_ENROLLMENT`
    pub const QUEST_USER_ENROLLMENT: Self = Self(Cow::Borrowed("QUEST_USER_ENROLLMENT"));

    /// `LOBBY_MESSAGE_CREATE`
    pub const LOBBY_MESSAGE_CREATE: Self = Self(Cow::Borrowed("LOBBY_MESSAGE_CREATE"));

    /// `LOBBY_MESSAGE_UPDATE`
    pub const LOBBY_MESSAGE_UPDATE: Self = Self(Cow::Borrowed("LOBBY_MESSAGE_UPDATE"));

    /// `LOBBY_MESSAGE_DELETE`
    pub const LOBBY_MESSAGE_DELETE: Self = Self(Cow::Borrowed("LOBBY_MESSAGE_DELETE"));

    /// `GAME_DIRECT_MESSAGE_CREATE`
    pub const GAME_DIRECT_MESSAGE_CREATE: Self = Self(Cow::Borrowed("GAME_DIRECT_MESSAGE_CREATE"));

    /// `GAME_DIRECT_MESSAGE_UPDATE`
    pub const GAME_DIRECT_MESSAGE_UPDATE: Self = Self(Cow::Borrowed("GAME_DIRECT_MESSAGE_UPDATE"));

    /// `GAME_DIRECT_MESSAGE_DELETE`
    pub const GAME_DIRECT_MESSAGE_DELETE: Self = Self(Cow::Borrowed("GAME_DIRECT_MESSAGE_DELETE"));

    /// Creates an owned event type from any string-like value.
    ///
    /// Prefer [`WebhookEventBodyType::from`] with a `&'static str` to borrow
    /// instead of allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the event type exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for WebhookEventBodyType {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// The outer envelope of a webhook event.
///
/// A payload with [`WebhookEventType::PING`] carries no
/// [`event`](Self::event); Discord sends it to verify the application's
/// endpoint. See [Discord's webhook event payload structure][payload] for
/// details.
///
/// [payload]: https://docs.discord.com/developers/events/webhook-events#payload-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct WebhookEventPayload {
    /// The version of the webhook event scheme.
    ///
    /// Discord currently always sends `1`.
    pub version: u16,

    /// The ID of the application the event is for.
    pub application_id: ApplicationId,

    /// The type of the webhook, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: WebhookEventType,

    /// The event body, absent for a ping.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<WebhookEventBody>,
}

/// The inner body of a webhook event.
///
/// [`data`](Self::data) holds the type-specific payload and is omitted during
/// serialization when `None`. See [Discord's event body object][body] for
/// details.
///
/// [body]: https://docs.discord.com/developers/events/webhook-events#event-body-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct WebhookEventBody {
    /// The event type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: WebhookEventBodyType,

    /// When the event occurred.
    #[serde(with = "crate::timestamp")]
    pub timestamp: SystemTime,

    /// The data payload for this event type.
    ///
    /// The shape depends on [`kind`](Self::kind); unrecognized shapes are
    /// preserved in [`WebhookEventBodyData::Unknown`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<WebhookEventBodyData>,
}

/// The data payload of a [`WebhookEventBody`].
///
/// Discord documents a distinct shape per event type, and several event types
/// share a shape: the entitlement events carry an [`Entitlement`], the lobby
/// message events carry a lobby message object, and the game direct message
/// events carry a message object of the Discord Social SDK. Variants are listed
/// from most to least specific, and a shape Cordion does not model is preserved
/// in [`Unknown`](Self::Unknown) so that an unavailable event such as
/// `QUEST_USER_ENROLLMENT` still decodes.
///
/// The application authorization variants are boxed because they embed a whole
/// user object, and the authorized variant also embeds a whole guild object,
/// which are both much larger than the other variants.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WebhookEventBodyData {
    /// The data of `APPLICATION_AUTHORIZED`.
    ApplicationAuthorized(Box<ApplicationAuthorized>),

    /// The data of `APPLICATION_DEAUTHORIZED`.
    ApplicationDeauthorized(Box<ApplicationDeauthorized>),

    /// The data of `ENTITLEMENT_CREATE`, `ENTITLEMENT_UPDATE`, and
    /// `ENTITLEMENT_DELETE`.
    Entitlement(Entitlement),

    /// The data of `LOBBY_MESSAGE_CREATE` and `LOBBY_MESSAGE_UPDATE`.
    LobbyMessage(Box<WebhookEventLobbyMessage>),

    /// The data of `LOBBY_MESSAGE_DELETE`.
    LobbyMessageDelete(LobbyMessageDelete),

    /// The data of `GAME_DIRECT_MESSAGE_CREATE`, `GAME_DIRECT_MESSAGE_UPDATE`,
    /// and `GAME_DIRECT_MESSAGE_DELETE` for a standard message.
    GameDirectMessage(Box<SocialSdkMessage>),

    /// The data of `GAME_DIRECT_MESSAGE_CREATE`, `GAME_DIRECT_MESSAGE_UPDATE`,
    /// and `GAME_DIRECT_MESSAGE_DELETE` for a message between provisional
    /// accounts.
    GameDirectMessageSdk(Box<SocialSdkDirectMessage>),

    /// A data shape that Cordion does not model.
    Unknown(serde_json::Value),
}
