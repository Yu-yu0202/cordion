/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

/// A user's Nitro subscription type.
///
/// Unrecognized values are preserved. See [Discord's premium types][types].
///
/// [types]: https://docs.discord.com/developers/resources/user#user-object-premium-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PremiumType(u8);

impl PremiumType {
    /// The user has no Nitro subscription.
    pub const NONE: Self = Self(0);

    /// The user has Discord Nitro Classic.
    pub const NITRO_CLASSIC: Self = Self(1);

    /// The user has Discord Nitro.
    pub const NITRO: Self = Self(2);

    /// The user has Discord Nitro Basic.
    pub const NITRO_BASIC: Self = Self(3);
}
