/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::monetization::sku::Sku;

/// An avatar decoration equipped by a user.
///
/// See [Discord's avatar decoration data object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/user#avatar-decoration-data-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AvatarDecorationData {
    /// The avatar decoration's asset hash.
    pub asset: String,

    /// The ID of the avatar decoration SKU.
    pub sku_id: Id<Sku>,
}
