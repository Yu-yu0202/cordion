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
    /// Flags attached to a [`Role`](super::Role).
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// role flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/topics/permissions#role-object-role-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct RoleFlags: u16 {
        /// No role flags are set.
        const NONE = 0;

        /// The role can be selected in an onboarding prompt.
        const IN_PROMPT = 1 << 0;
    }
}

impl Serialize for RoleFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for RoleFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u16::deserialize(deserializer)?))
    }
}
