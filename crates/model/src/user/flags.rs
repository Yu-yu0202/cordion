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
    /// Flags and badges attached to a [`User`](super::User).
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// user flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/user#user-object-user-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct UserFlags: u64 {
        /// No account flag bits are set.
        const NONE = 0;

        /// The account belongs to a Discord employee.
        const STAFF = 1 << 0;

        /// The user owns a partnered server.
        const PARTNER = 1 << 1;

        /// The user is a HypeSquad Events member.
        const HYPESQUAD = 1 << 2;

        /// The user has the Bug Hunter Level 1 badge.
        const BUG_HUNTER_LEVEL_1 = 1 << 3;

        /// The user belongs to HypeSquad House Bravery.
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;

        /// The user belongs to HypeSquad House Brilliance.
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;

        /// The user belongs to HypeSquad House Balance.
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;

        /// The user has the Early Nitro Supporter badge.
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;

        /// The account represents a developer team.
        const TEAM_PSEUDO_USER = 1 << 10;

        /// The user has the Bug Hunter Level 2 badge.
        const BUG_HUNTER_LEVEL_2 = 1 << 14;

        /// The account is a verified bot.
        const VERIFIED_BOT = 1 << 16;

        /// The user has the Early Verified Bot Developer badge.
        const VERIFIED_DEVELOPER = 1 << 17;

        /// The user is a Moderator Programs alumnus.
        const CERTIFIED_MODERATOR = 1 << 18;

        /// The bot uses only HTTP interactions and appears in the online member
        /// list.
        const BOT_HTTP_INTERACTIONS = 1 << 19;
    }
}

impl Serialize for UserFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}
