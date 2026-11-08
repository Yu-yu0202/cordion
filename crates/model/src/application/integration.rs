/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::application::InstallParams;

/// Configuration for one installation context.
///
/// Entries are keyed by integration type (`0` for guild installs and `1` for
/// user installs) in an application's `integration_types_config`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IntegrationTypeConfig {
    /// Install parameters used for this context's default in-app authorization
    /// link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_install_params: Option<InstallParams>,
}
