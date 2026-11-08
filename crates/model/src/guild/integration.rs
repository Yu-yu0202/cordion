/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::borrow::Cow;
use std::time::SystemTime;

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::id::{Id, Identifiable};
use crate::role::RoleId;
use crate::user::{PartialUser, User};

/// The ID of an [`Integration`].
pub type IntegrationId = Id<Integration>;

/// A guild integration.
///
/// Discord bot integrations omit the fields marked as absent for them in the
/// documentation, such as [`syncing`](Self::syncing) and
/// [`subscriber_count`](Self::subscriber_count). Optional fields are omitted
/// during serialization when `None`.
///
/// See [Discord's integration object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#integration-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Integration {
    /// The integration's ID.
    pub id: IntegrationId,

    /// The integration's name.
    pub name: String,

    /// The integration type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: IntegrationType,

    /// Whether the integration is enabled.
    pub enabled: bool,

    /// Whether the integration is syncing.
    ///
    /// This is not provided for Discord bot integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syncing: Option<bool>,

    /// The ID of the role this integration uses for subscribers.
    ///
    /// This is not provided for Discord bot integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<RoleId>,

    /// Whether emoticons are synced for this integration.
    ///
    /// This applies to Twitch integrations and is not provided for Discord bot
    /// integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_emoticons: Option<bool>,

    /// The behavior when subscribers expire.
    ///
    /// This is not provided for Discord bot integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_behavior: Option<IntegrationExpireBehavior>,

    /// The grace period in days before subscribers expire.
    ///
    /// This is not provided for Discord bot integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_grace_period: Option<u64>,

    /// The user associated with this integration.
    ///
    /// Some older integrations may not have an attached user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// The integration's account information.
    pub account: IntegrationAccount,

    /// When the integration was last synced.
    ///
    /// This is not provided for Discord bot integrations.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::timestamp::option"
    )]
    pub synced_at: Option<SystemTime>,

    /// How many subscribers the integration has.
    ///
    /// This is not provided for Discord bot integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<u64>,

    /// Whether the integration has been revoked.
    ///
    /// This is not provided for Discord bot integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,

    /// The bot or `OAuth2` application for Discord integrations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<IntegrationApplication>,

    /// The `OAuth2` scopes the application has been authorized for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

impl Identifiable for Integration {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// A partial [`Integration`] embedded in an audit log.
///
/// Audit logs only include the fields needed to identify the integration. See
/// [Discord's partial integration example][example] for details.
///
/// [example]: https://docs.discord.com/developers/resources/audit-log#audit-log-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialIntegration {
    /// The integration's ID.
    pub id: IntegrationId,

    /// The integration's name.
    pub name: String,

    /// The integration type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: IntegrationType,

    /// The integration's account information.
    pub account: IntegrationAccount,

    /// The ID of the application associated with the integration, when
    /// included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
}

/// The service type of an [`Integration`].
///
/// Documented values are exposed as associated constants, and unrecognized
/// values are preserved when decoding. See [Discord's integration structure][object]
/// for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#integration-object
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct IntegrationType(Cow<'static, str>);

impl IntegrationType {
    /// A Twitch integration.
    pub const TWITCH: Self = Self(Cow::Borrowed("twitch"));

    /// A `YouTube` integration.
    pub const YOUTUBE: Self = Self(Cow::Borrowed("youtube"));

    /// A Discord bot integration.
    pub const DISCORD: Self = Self(Cow::Borrowed("discord"));

    /// A guild subscription integration.
    pub const GUILD_SUBSCRIPTION: Self = Self(Cow::Borrowed("guild_subscription"));

    /// Creates an owned integration type from any string-like value.
    ///
    /// Prefer [`IntegrationType::from`] with a `&'static str` to borrow instead
    /// of allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the integration type exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for IntegrationType {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// The behavior when an [`Integration`] subscriber expires.
///
/// Unrecognized values are preserved. See [Discord's expire behaviors][behaviors]
/// for details.
///
/// [behaviors]: https://docs.discord.com/developers/resources/guild#integration-object-integration-expire-behaviors
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct IntegrationExpireBehavior(u8);

impl IntegrationExpireBehavior {
    /// Remove the subscriber's role.
    pub const REMOVE_ROLE: Self = Self(0);

    /// Kick the subscriber from the guild.
    pub const KICK: Self = Self(1);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Account information for a guild integration.
///
/// See [Discord's integration account object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#integration-account-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IntegrationAccount {
    /// The integration account's ID.
    ///
    /// This is an arbitrary string rather than a Discord snowflake; externally
    /// connected integrations may use identifiers such as `"twitch-123"`.
    pub id: String,

    /// The integration account's name.
    pub name: String,
}

/// The bot or `OAuth2` application for a Discord [`Integration`].
///
/// [`bot`](Self::bot) uses [`PartialUser`] rather than [`User`], matching the
/// partial user objects Discord documents for an application's `bot` and
/// `owner` fields: the embedded object only guarantees the fields of a partial
/// user, and decoding it as a full user would reject payloads that omit them.
///
/// See [Discord's integration application object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#integration-application-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IntegrationApplication {
    /// The application's ID.
    pub id: ApplicationId,

    /// The application's name.
    pub name: String,

    /// The application's icon hash, or `None` if it has no icon.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    pub icon: Option<String>,

    /// The application's description.
    pub description: String,

    /// The bot user associated with this application, when included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<PartialUser>,
}
