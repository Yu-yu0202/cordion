/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Prune count response and request payloads for the guild prune endpoints.
//!
//! See [Discord's guild resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/guild

use serde::{Deserialize, Serialize};

use crate::role::RoleId;

/// The response of `GET` and `POST /guilds/{guild.id}/prune`.
///
/// [`pruned`](Self::pruned) is `null` when the count is not computed or a prune
/// did not run, so `None` is serialized as `null`.
///
/// See [Discord's get guild prune count endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#get-guild-prune-count
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildPruneCount {
    /// The number of members that were or would be removed, when computed.
    #[serde(default)]
    pub pruned: Option<u64>,
}

/// The body of `POST /guilds/{guild.id}/prune`.
///
/// See [Discord's begin guild prune endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#begin-guild-prune
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BeginGuildPruneRequest {
    /// The number of days to prune, between 1 and 30.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<u64>,

    /// Whether to compute and return the prune count.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compute_prune_count: Option<bool>,

    /// The roles to include in the prune.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_roles: Option<Vec<RoleId>>,

    /// The reason for the prune.
    ///
    /// Discord deprecated this field in favor of the `X-Audit-Log-Reason`
    /// header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
