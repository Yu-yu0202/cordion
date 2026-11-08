/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::role::{RoleColors, RoleId};

/// A partial role assigned when an invite is accepted.
///
/// Discord's invite object includes only `id`, `name`, `position`, `color`,
/// `colors`, `icon`, and `unicode_emoji` for these roles; the fields present
/// elsewhere in the full role object are not sent. Unknown fields from future
/// partial role shapes are ignored.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialRole {
    /// The role's ID.
    pub id: RoleId,

    /// The role's name, up to 100 characters.
    pub name: String,

    /// The role's position.
    ///
    /// Roles with the same position are sorted by ID.
    pub position: i32,

    /// The role's legacy color value.
    ///
    /// This is deprecated in favor of [`colors`](Self::colors), but Discord
    /// still includes it in partial roles.
    #[deprecated(note = "Deprecated by Discord API. Use `colors` instead.")]
    pub color: u32,

    /// The role's colors.
    pub colors: RoleColors,

    /// The role's icon hash, when set.
    ///
    /// Requires the guild's `ROLE_ICONS` feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The role's Unicode emoji, when set.
    ///
    /// Requires the guild's `ROLE_ICONS` feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
}
