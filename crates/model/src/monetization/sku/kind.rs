/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

/// The purchase type of a [`Sku`](super::Sku).
///
/// Unrecognized values are preserved. See [Discord's SKU types][types].
///
/// [types]: https://docs.discord.com/developers/resources/sku#sku-object-sku-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SkuKind(u8);

impl SkuKind {
    /// A durable one-time purchase that is not consumed or renewed.
    pub const DURABLE: Self = Self(2);

    /// A consumable one-time purchase, such as a temporary boost.
    pub const CONSUMABLE: Self = Self(3);

    /// A recurring subscription; use this SKU for subscription integrations.
    pub const SUBSCRIPTION: Self = Self(5);

    /// A system-generated group for each [`SUBSCRIPTION`](Self::SUBSCRIPTION)
    /// SKU; not used directly by integrations.
    pub const SUBSCRIPTION_GROUP: Self = Self(6);
}
