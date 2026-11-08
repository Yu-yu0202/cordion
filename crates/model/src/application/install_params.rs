/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::permission::Permission;

/// Parameters used to build an application's default in-app authorization link.
///
/// See [Discord's install params object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application#install-params-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InstallParams {
    /// `OAuth2` scopes to request when adding the application.
    ///
    /// See [Discord's `OAuth2` scopes][scopes] for the supported values.
    ///
    /// [scopes]: https://docs.discord.com/developers/topics/oauth2#shared-resources-oauth2-scopes
    pub scopes: Vec<String>,

    /// Permissions to request for the bot role.
    pub permissions: Permission,
}
