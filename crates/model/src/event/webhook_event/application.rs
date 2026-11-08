/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Data payloads of the `APPLICATION_AUTHORIZED` and `APPLICATION_DEAUTHORIZED`
//! webhook events.
//!
//! See [Discord's webhook events documentation][events] for details.
//!
//! [events]: https://docs.discord.com/developers/events/webhook-events

use serde::{Deserialize, Serialize};

use crate::application::ApplicationIntegrationType;
use crate::guild::Guild;
use crate::user::User;

/// The data of the `APPLICATION_AUTHORIZED` webhook event.
///
/// [`guild`](Self::guild) is only present when the application was installed to
/// a guild, and [`integration_type`](Self::integration_type) identifies the
/// installation context. Optional fields are omitted during serialization when
/// `None`.
///
/// See [Discord's application authorized structure][structure] for details.
///
/// [structure]: https://docs.discord.com/developers/events/webhook-events#application-authorized
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationAuthorized {
    /// The installation context the application was authorized for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<ApplicationIntegrationType>,

    /// The user who authorized the application.
    pub user: User,

    /// The `OAuth2` scopes the user authorized.
    pub scopes: Vec<String>,

    /// The guild the application was authorized for.
    ///
    /// Discord documents this field as a guild object and only sends it for
    /// guild installations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<Guild>,
}

/// The data of the `APPLICATION_DEAUTHORIZED` webhook event.
///
/// See [Discord's application deauthorized structure][structure] for details.
///
/// [structure]: https://docs.discord.com/developers/events/webhook-events#application-deauthorized
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationDeauthorized {
    /// The user who deauthorized the application.
    pub user: User,
}
