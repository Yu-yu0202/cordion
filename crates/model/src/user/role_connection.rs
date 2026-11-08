/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A user's role connection for an application.
///
/// Applications use this object to attach platform-specific identity and
/// metadata to a user. [`platform_name`](Self::platform_name) and
/// [`platform_username`](Self::platform_username) are nullable and are
/// serialized as `null` when `None`; [`metadata`](Self::metadata) is omitted
/// when `None`.
///
/// See [Discord's application role connection object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/user#application-role-connection-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationRoleConnection {
    /// The vanity name of the platform the application connected, up to 50
    /// characters.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub platform_name: Option<String>,

    /// The username on the connected platform, up to 100 characters.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub platform_username: Option<String>,

    /// Metadata keys mapped to their stringified values for the user.
    ///
    /// Each value is at most 100 characters. The keys correspond to
    /// [`ApplicationRoleConnectionMetadata`](crate::application::ApplicationRoleConnectionMetadata)
    /// records defined by the application.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
}
