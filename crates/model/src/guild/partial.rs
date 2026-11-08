/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::guild::{GuildFeature, GuildId};
use crate::permission::Permission;

/// A partial guild returned by `GET /users/@me/guilds`.
///
/// The endpoint returns less information than a full [`Guild`](super::Guild):
/// it omits roles, emoji, and channel IDs. `OAuth2` requests require the
/// `guilds` scope, and the counts are only included when `with_counts` is
/// enabled.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialGuild {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's name.
    pub name: String,

    /// The guild's icon hash, or `None` if it has no icon.
    pub icon: Option<String>,

    /// The guild's banner hash, or `None` if it has no banner.
    pub banner: Option<String>,

    /// Whether the requesting user owns the guild.
    pub owner: bool,

    /// The requesting user's total permissions in the guild.
    ///
    /// This excludes channel overwrites and implicit permissions.
    pub permissions: Permission,

    /// Enabled guild features.
    ///
    /// Unknown feature strings are preserved by [`GuildFeature`].
    pub features: Vec<GuildFeature>,

    /// The approximate number of members in the guild.
    ///
    /// Only returned when `with_counts` is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,

    /// The approximate number of non-offline members in the guild.
    ///
    /// Only returned when `with_counts` is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,
}
