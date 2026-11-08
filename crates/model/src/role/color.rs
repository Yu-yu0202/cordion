/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

/// A role's color configuration.
///
/// `primary_color` is always populated. Secondary and tertiary colors are only
/// set when the guild has the `ENHANCED_ROLE_COLORS` feature: a secondary color
/// creates a gradient, and a tertiary color creates a holographic style.
///
/// See [Discord's role colors object][object] for details.
///
/// [object]: https://docs.discord.com/developers/topics/permissions#role-object-role-colors-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RoleColors {
    /// The primary color as a 24-bit RGB value (`0xRRGGBB`).
    pub primary_color: u32,

    /// The secondary color as a 24-bit RGB value (`0xRRGGBB`).
    ///
    /// Discord sends `null` when unset. When set, the role color is a gradient
    /// from the primary color.
    pub secondary_color: Option<u32>,

    /// The tertiary color as a 24-bit RGB value (`0xRRGGBB`).
    ///
    /// Discord sends `null` when unset. When set, the role color becomes
    /// holographic.
    pub tertiary_color: Option<u32>,
}
