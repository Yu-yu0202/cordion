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
    /// Notification flags for a guild's system channel.
    ///
    /// Unrecognized bits are retained during deserialization.
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct SystemChannelFlags: u64 {
        /// Suppress member join notifications.
        const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;

        /// Suppress Server Boost notifications.
        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;

        /// Suppress server setup tips.
        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;

        /// Hide member join sticker reply buttons.
        const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;

        /// Suppress role subscription purchase and renewal notifications.
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;

        /// Hide role subscription sticker reply buttons.
        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5;
    }
}

impl Serialize for SystemChannelFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for SystemChannelFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}
