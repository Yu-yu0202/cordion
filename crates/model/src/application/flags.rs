/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::fmt;
use std::str::FromStr;

use bitflags::bitflags;
use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Flags and badges attached to an [`Application`](super::Application).
    ///
    /// Discord serializes the legacy `flags` field as an integer and the newer
    /// `flags_new` field as a decimal string. Unrecognized bits are retained
    /// during deserialization. See [Discord's application flags][flags] for
    /// details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/application#application-object-application-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct ApplicationFlags: u64 {
        /// No application flag bits are set.
        const NONE = 0;

        /// The application uses the Auto Moderation API.
        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;

        /// The application has the `GATEWAY_PRESENCE` intent, which is
        /// required for bots in 100 or more servers to receive presence
        /// updates.
        const GATEWAY_PRESENCE = 1 << 12;

        /// The application has the limited `GATEWAY_PRESENCE` intent, which is
        /// required for bots in fewer than 100 servers to receive presence
        /// updates.
        const GATEWAY_PRESENCE_LIMITED = 1 << 13;

        /// The application has the `GUILD_MEMBERS` intent, which is required
        /// for bots in 100 or more servers to receive member-related events.
        const GATEWAY_GUILD_MEMBERS = 1 << 14;

        /// The application has the limited `GUILD_MEMBERS` intent, which is
        /// required for bots in fewer than 100 servers to receive
        /// member-related events.
        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;

        /// Unusual growth of the application prevents verification.
        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;

        /// The application is embedded within the Discord client.
        const EMBEDDED = 1 << 17;

        /// The application has the `MESSAGE_CONTENT` intent, which is required
        /// for bots in 100 or more servers to receive message content.
        const GATEWAY_MESSAGE_CONTENT = 1 << 18;

        /// The application has the limited `MESSAGE_CONTENT` intent, which is
        /// required for bots in fewer than 100 servers to receive message
        /// content.
        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;

        /// The application has registered global application commands.
        const APPLICATION_COMMAND_BADGE = 1 << 23;
    }
}

impl Serialize for ApplicationFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for ApplicationFlags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FlagsVisitor;

        impl<'de> Visitor<'de> for FlagsVisitor {
            type Value = ApplicationFlags;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an application flags bitfield as an integer or decimal string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let unsigned = u64::try_from(value).map_err(|_| {
                    E::invalid_value(Unexpected::Signed(value), &"a u64 application flags bitfield")
                })?;

                self.visit_u64(unsigned)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(ApplicationFlags::from_bits_retain(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let value = u64::from_str(value).map_err(|_| {
                    E::invalid_value(Unexpected::Str(value), &"a u64 application flags bitfield")
                })?;

                self.visit_u64(value)
            }
        }

        deserializer.deserialize_any(FlagsVisitor)
    }
}

/// Serde helpers for the string-serialized `flags_new` application field.
pub(crate) mod flags_new {
    use serde::{Deserialize, Deserializer, Serializer};

    use super::ApplicationFlags;

    /// Serializes optional flags as a decimal string or `null`.
    pub(crate) fn serialize<S>(
        flags: &Option<ApplicationFlags>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match flags {
            Some(flags) => serializer.collect_str(&flags.bits()),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes optional flags from a decimal string, integer, or `null`.
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<ApplicationFlags>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<ApplicationFlags>::deserialize(deserializer)
    }
}
