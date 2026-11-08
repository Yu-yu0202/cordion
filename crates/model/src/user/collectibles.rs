/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::user::nameplate::Nameplate;

/// Collectibles owned by a user.
///
/// This excludes avatar decorations and profile effects. See [Discord's
/// collectibles object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/user#collectibles
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Collectibles {
    /// The nameplate the user has equipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameplate: Option<Nameplate>,
}
