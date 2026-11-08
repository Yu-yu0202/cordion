/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Application-related models.
//!
//! [`Application`] models Discord's application object. The module also exposes
//! installation configuration, the partial application embedded in invites,
//! role connection metadata, running [`ActivityInstance`] objects, and the
//! [`ApplicationId`] used by other resources.
//!
//! See [Discord's application resource][resource] for the surrounding object.
//!
//! [resource]: https://docs.discord.com/developers/resources/application

mod activity;
mod flags;
mod guild;
mod install_params;
mod integration;
mod partial;
mod role_connection;
pub mod team;

use std::collections::BTreeMap;

pub use activity::{ActivityInstance, ActivityLocation, ActivityLocationKind};
use cordion_macro::IdTarget;
pub use flags::ApplicationFlags;
pub use guild::ApplicationGuild;
pub use install_params::InstallParams;
pub use integration::IntegrationTypeConfig;
pub use partial::PartialApplication;
pub use role_connection::{
    ApplicationRoleConnectionMetadata, ApplicationRoleConnectionMetadataType,
};
use serde::{Deserialize, Serialize};
pub use team::{Team, TeamId, TeamMember, TeamMemberRole, TeamMembershipState};

use crate::guild::GuildId;
use crate::id::{Id, Identifiable};
use crate::monetization::sku::SkuId;
use crate::user::PartialUser;

/// The ID of an [`Application`].
pub type ApplicationId = Id<Application>;

/// An installation context for an application.
///
/// Unrecognized values are preserved. Discord uses this value in command
/// `integration_types` arrays and as the keys of an interaction's
/// `authorizing_integration_owners` object. See [Discord's application
/// integration types][types] for details.
///
/// [types]: https://docs.discord.com/developers/resources/application#application-object-application-integration-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApplicationIntegrationType(u8);

impl ApplicationIntegrationType {
    /// The application can be installed to a guild.
    pub const GUILD_INSTALL: Self = Self(0);

    /// The application can be installed to a user.
    pub const USER_INSTALL: Self = Self(1);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Whether event webhooks are enabled for an [`Application`].
///
/// Unrecognized values are preserved. See [Discord's application event webhook
/// status][status] for details.
///
/// [status]: https://docs.discord.com/developers/resources/application#application-object-application-event-webhook-status
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApplicationEventWebhookStatus(u8);

impl ApplicationEventWebhookStatus {
    /// Webhook events are disabled by the developer.
    pub const DISABLED: Self = Self(1);

    /// Webhook events are enabled by the developer.
    pub const ENABLED: Self = Self(2);

    /// Webhook events are disabled by Discord, usually due to inactivity.
    pub const DISABLED_BY_DISCORD: Self = Self(3);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A Discord application.
///
/// Fields that Discord documents as nullable ([`icon`](Self::icon),
/// [`team`](Self::team), [`interactions_endpoint_url`](Self::interactions_endpoint_url),
/// [`role_connections_verification_url`](Self::role_connections_verification_url),
/// and [`event_webhooks_url`](Self::event_webhooks_url)) are serialized as
/// `null` when `None`. Other optional fields are omitted when `None`.
///
/// See [Discord's application object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application#application-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Application {
    /// The application's ID.
    pub id: ApplicationId,

    /// The application's name.
    pub name: String,

    /// The application's icon hash, or `None` if it has no icon.
    ///
    /// Discord sends this field explicitly as `null` when the application has
    /// no icon, so `None` is serialized as `null`.
    pub icon: Option<String>,

    /// The application's description.
    pub description: String,

    /// The list of RPC origin URLs, if RPC is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rpc_origins: Option<Vec<String>>,

    /// Whether only the application owner can add the application to guilds.
    pub bot_public: bool,

    /// Whether the application's bot joins only after the full `OAuth2` code
    /// grant flow completes.
    pub bot_require_code_grant: bool,

    /// A partial user object for the bot user associated with the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<PartialUser>,

    /// The URL of the application's Terms of Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,

    /// The URL of the application's Privacy Policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,

    /// A partial user object for the application's owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<PartialUser>,

    /// The hex-encoded key used to verify interactions.
    pub verify_key: String,

    /// The developer team that owns the application, or `None` if the
    /// application is owned by an individual.
    ///
    /// Discord sends this field explicitly as `null` when the application does
    /// not belong to a team, so `None` is serialized as `null`.
    pub team: Option<Team>,

    /// The ID of the guild associated with the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// A partial guild object for the guild associated with the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<ApplicationGuild>,

    /// The ID of the game SKU, if the application is a game sold on Discord.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_sku_id: Option<SkuId>,

    /// The URL slug for the store page, if the application is a game sold on
    /// Discord.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    /// The application's default rich presence invite cover image hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,

    /// The application's legacy public flags.
    ///
    /// Discord serializes this field as an integer and does not grow it beyond
    /// 31 bits; use [`flags_new`](Self::flags_new) for the full set of flag
    /// bits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ApplicationFlags>,

    /// The application's public flags as a decimal string.
    ///
    /// This response-only field contains the full set of flag bits. It is
    /// omitted during serialization when `None`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "flags::flags_new"
    )]
    pub flags_new: Option<ApplicationFlags>,

    /// The approximate number of guilds the application has been added to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_guild_count: Option<u64>,

    /// The approximate number of users that have installed the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_user_install_count: Option<u64>,

    /// The approximate number of users that have `OAuth2` authorizations for
    /// the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_user_authorization_count: Option<u64>,

    /// The application's redirect URIs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,

    /// The interactions endpoint URL, or `None` if none is configured.
    ///
    /// Discord may send this field as `null`; `None` is serialized as `null`.
    #[serde(default)]
    pub interactions_endpoint_url: Option<String>,

    /// The role connection verification URL, or `None` if none is configured.
    ///
    /// Discord may send this field as `null`; `None` is serialized as `null`.
    #[serde(default)]
    pub role_connections_verification_url: Option<String>,

    /// The event webhooks URL, or `None` if none is configured.
    ///
    /// Discord may send this field as `null`; `None` is serialized as `null`.
    #[serde(default)]
    pub event_webhooks_url: Option<String>,

    /// Whether event webhooks are enabled for the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_webhooks_status: Option<ApplicationEventWebhookStatus>,

    /// The webhook event types the application subscribes to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_webhooks_types: Option<Vec<String>>,

    /// Tags describing the application's content and functionality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// Settings for the application's default in-app authorization link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_params: Option<InstallParams>,

    /// Default scopes and permissions for each supported installation context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_types_config:
        Option<BTreeMap<ApplicationIntegrationType, IntegrationTypeConfig>>,

    /// The default custom authorization URL, if enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_install_url: Option<String>,
}

impl Identifiable for Application {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The body of `PATCH /applications/@me`.
///
/// Every field is optional and is omitted during serialization when `None`, so
/// only the properties that should change have to be sent. Discord documents
/// [`flags`](Self::flags) as an integer bitfield that only accepts the limited
/// intent flags, and [`icon`](Self::icon) and [`cover_image`](Self::cover_image)
/// as base64-encoded image data.
///
/// See [Discord's edit current application endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/application#edit-current-application
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EditCurrentApplicationRequest {
    /// The app's default custom authorization URL, if enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_install_url: Option<String>,

    /// The app's description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The app's role connection verification URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_connections_verification_url: Option<String>,

    /// Settings for the app's default in-app authorization link.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_params: Option<InstallParams>,

    /// Default scopes and permissions for each supported installation context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_types_config:
        Option<BTreeMap<ApplicationIntegrationType, IntegrationTypeConfig>>,

    /// The app's public flags.
    ///
    /// Discord only accepts the limited intent flags through the API.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ApplicationFlags>,

    /// The app's icon as base64-encoded image data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The app's default rich presence invite cover image as base64-encoded
    /// image data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,

    /// The app's interactions endpoint URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interactions_endpoint_url: Option<String>,

    /// The app's tags, up to five tags of at most 20 characters each.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    /// The URL the app receives webhook events on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_webhooks_url: Option<String>,

    /// Whether webhook events are enabled for the app.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_webhooks_status: Option<ApplicationEventWebhookStatus>,

    /// The webhook event types the app subscribes to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_webhooks_types: Option<Vec<String>>,
}
