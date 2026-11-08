/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Purchase availability and subscription access flags for a [`Sku`](super::Sku).
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// SKU flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/sku#sku-object-sku-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct SkuFlags: u16 {
        /// The SKU is available for purchase.
        const AVAILABLE = 1 << 2;

        /// A recurring purchase that grants benefits to every member of one
        /// guild.
        const GUILD_SUBSCRIPTION = 1 << 7;

        /// A recurring purchase that grants the buyer benefits in every guild.
        const USER_SUBSCRIPTION = 1 << 8;
    }
}

impl Serialize for SkuFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for SkuFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u16::deserialize(deserializer)?))
    }
}
