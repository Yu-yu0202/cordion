/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord permission flags.
//!
//! See [Discord's permissions documentation](https://docs.discord.com/developers/topics/permissions).

use bitflags::bitflags;
use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Guild and channel permission flags.
    ///
    /// Discord encodes permissions as a decimal string bitfield. This type
    /// serializes as a string, accepts either a string or an integer when
    /// deserializing, and preserves bits that Cordion does not define.
    ///
    /// See [Discord's permissions documentation](https://docs.discord.com/developers/topics/permissions)
    /// for permission hierarchy, channel overwrites, and implicit restrictions.
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Permission: u64 {
        /// No permission bits are set.
        const NONE = 0;

        /// Create instant invites.
        const CREATE_INSTANT_INVITE = 1 << 0;

        /// Kick guild members, subject to role hierarchy restrictions.
        const KICK_MEMBERS = 1 << 1;

        /// Ban guild members, subject to role hierarchy restrictions.
        const BAN_MEMBERS = 1 << 2;

        /// Grant all permissions and bypass channel permission overwrites.
        const ADMINISTRATOR = 1 << 3;

        /// Manage and edit channels.
        const MANAGE_CHANNELS = 1 << 4;

        /// Manage and edit the guild.
        const MANAGE_GUILD = 1 << 5;

        /// Add new reactions to messages.
        ///
        /// Reacting with an emoji already present on a message does not require this permission.
        const ADD_REACTIONS = 1 << 6;

        /// View the guild's audit log.
        const VIEW_AUDIT_LOG = 1 << 7;

        /// Use priority speaker in voice channels.
        const PRIORITY_SPEAKER = 1 << 8;

        /// Go live in voice and stage channels.
        const STREAM = 1 << 9;

        /// View guild channels, read messages in text channels, and join voice
        /// channels.
        ///
        /// Viewing threads also requires this permission in their parent channel.
        const VIEW_CHANNEL = 1 << 10;

        /// Send channel messages and create threads in forum channels.
        ///
        /// Sending messages in threads requires [`Self::SEND_MESSAGES_IN_THREADS`] instead.
        const SEND_MESSAGES = 1 << 11;

        /// Send text-to-speech messages using `/tts`.
        const SEND_TTS_MESSAGES = 1 << 12;

        /// Delete messages sent by other users.
        const MANAGE_MESSAGES = 1 << 13;

        /// Automatically embed links in sent messages.
        const EMBED_LINKS = 1 << 14;

        /// Upload images and files.
        const ATTACH_FILES = 1 << 15;

        /// Read past messages in a channel.
        const READ_MESSAGE_HISTORY = 1 << 16;

        /// Notify everyone in a channel with `@everyone`, or online members with `@here`.
        const MENTION_EVERYONE = 1 << 17;

        /// Use custom emojis from other guilds.
        const USE_EXTERNAL_EMOJIS = 1 << 18;

        /// View guild insights.
        const VIEW_GUILD_INSIGHTS = 1 << 19;

        /// Join voice and stage channels.
        const CONNECT = 1 << 20;

        /// Speak in voice channels.
        const SPEAK = 1 << 21;

        /// Mute members in voice and stage channels.
        const MUTE_MEMBERS = 1 << 22;

        /// Deafen members in voice channels.
        const DEAFEN_MEMBERS = 1 << 23;

        /// Move members between voice channels, including stage channels.
        const MOVE_MEMBERS = 1 << 24;

        /// Use voice activity detection in voice channels.
        const USE_VAD = 1 << 25;

        /// Change the current user's guild nickname.
        const CHANGE_NICKNAME = 1 << 26;

        /// Change other members' guild nicknames, subject to role hierarchy restrictions.
        const MANAGE_NICKNAMES = 1 << 27;

        /// Manage and edit roles, subject to role hierarchy restrictions.
        const MANAGE_ROLES = 1 << 28;

        /// Manage and edit webhooks.
        const MANAGE_WEBHOOKS = 1 << 29;

        /// Edit and delete any user's emojis, stickers, and soundboard sounds.
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;

        /// Use application commands, including slash commands and context menu commands.
        const USE_APPLICATION_COMMANDS = 1 << 31;

        /// Request to speak in stage channels.
        const REQUEST_TO_SPEAK = 1 << 32;

        /// Edit and delete scheduled events created by any user.
        const MANAGE_EVENTS = 1 << 33;

        /// Delete and archive threads, and view all private threads.
        const MANAGE_THREADS = 1 << 34;

        /// Create public and announcement threads.
        const CREATE_PUBLIC_THREADS = 1 << 35;

        /// Create private threads.
        const CREATE_PRIVATE_THREADS = 1 << 36;

        /// Use custom stickers from other guilds.
        const USE_EXTERNAL_STICKERS = 1 << 37;

        /// Send messages in threads.
        const SEND_MESSAGES_IN_THREADS = 1 << 38;

        /// Use Activities (applications with the `EMBEDDED` flag) in text and voice channels.
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;

        /// Time out members.
        ///
        /// Timed out members retain only [`Self::VIEW_CHANNEL`] and [`Self::READ_MESSAGE_HISTORY`].
        /// Guild owners and members with [`Self::ADMINISTRATOR`] are exempt.
        const MODERATE_MEMBERS = 1 << 40;

        /// View role subscription insights.
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;

        /// Play soundboard sounds in voice channels.
        const USE_SOUNDBOARD = 1 << 42;

        /// Create emojis, stickers, and soundboard sounds, and edit or delete the current user's own.
        const CREATE_GUILD_EXPRESSIONS = 1 << 43;

        /// Create scheduled events, and edit or delete those created by the current user.
        const CREATE_EVENTS = 1 << 44;

        /// Use custom soundboard sounds from other guilds in voice channels.
        const USE_EXTERNAL_SOUNDS = 1 << 45;

        /// Send voice messages.
        const SEND_VOICE_MESSAGES = 1 << 46;

        /// Set the status of a voice channel.
        const SET_VOICE_CHANNEL_STATUS = 1 << 48;

        /// Send polls.
        const SEND_POLLS = 1 << 49;

        /// Allow user-installed apps to respond publicly when they are not also guild-installed.
        ///
        /// Without this permission, these apps can still be used, but their responses are ephemeral.
        const USE_EXTERNAL_APPS = 1 << 50;

        /// Pin and unpin messages.
        const PIN_MESSAGES = 1 << 51;

        /// Bypass channel slowmode restrictions.
        ///
        /// Bots are not affected by slowmode restrictions.
        const BYPASS_SLOWMODE = 1 << 52;
    }
}

impl Permission {
    /// Applies channel overwrite bits to these permissions.
    ///
    /// Allowed bits are added while denied bits are excluded, and the existing
    /// bits in `self` are retained. This is a single-step bitwise operation, not
    /// Discord's full overwrite algorithm, which applies `@everyone`, role, and
    /// member overwrites in order.
    ///
    /// # Examples
    ///
    /// ```
    /// use cordion_model::permission::Permission;
    ///
    /// let base = Permission::VIEW_CHANNEL;
    /// let allow = Permission::SEND_MESSAGES;
    /// let deny = Permission::MANAGE_MESSAGES;
    ///
    /// let result = base.apply_overwrites(&allow, &deny);
    /// assert!(result.contains(Permission::VIEW_CHANNEL));
    /// assert!(result.contains(Permission::SEND_MESSAGES));
    /// assert!(!result.contains(Permission::MANAGE_MESSAGES));
    /// ```
    #[inline]
    #[must_use]
    pub const fn apply_overwrites(&self, allow: &Self, deny: &Self) -> Self {
        Self::from_bits_retain(self.bits() | allow.bits() & !deny.bits())
    }
}

impl Serialize for Permission {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.bits())
    }
}

impl<'de> Deserialize<'de> for Permission {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermissionVisitor;

        impl<'de> Visitor<'de> for PermissionVisitor {
            type Value = Permission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a permission bitfield as a decimal string or integer")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Permission::from_bits_retain(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let bits = u64::try_from(value).map_err(|_| {
                    E::invalid_value(
                        Unexpected::Signed(value),
                        &"a non-negative permission bitfield",
                    )
                })?;

                Ok(Permission::from_bits_retain(bits))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let bits = value.parse::<u64>().map_err(|_| {
                    E::invalid_value(Unexpected::Str(value), &"a decimal permission bitfield")
                })?;

                Ok(Permission::from_bits_retain(bits))
            }
        }

        deserializer.deserialize_any(PermissionVisitor)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Tests that permissions can be deserialized from strings and integers.
    #[test]
    fn deserializes_string_and_integer() {
        let from_string: Permission = serde_json::from_str("\"2112\"").unwrap();
        let from_integer: Permission = serde_json::from_str("2112").unwrap();

        assert_eq!(from_string, from_integer);
        assert_eq!(from_string.bits(), 2112);
    }

    /// Tests that permission bits unknown to Cordion are preserved.
    #[test]
    fn preserves_unknown_bits() {
        let permission: Permission = serde_json::from_str("\"9007199254740992\"").unwrap();

        assert_eq!(permission.bits(), 1 << 53);
    }

    /// Tests that permissions serialize as a decimal string.
    #[test]
    fn serializes_as_string() {
        let permission = Permission::CREATE_INSTANT_INVITE | Permission::VIEW_CHANNEL;

        assert_eq!(serde_json::to_value(permission).unwrap(), serde_json::json!("1025"));
    }

    /// Tests that permissions use the full 64-bit range.
    #[test]
    fn preserves_full_u64_range() {
        let permission: Permission = serde_json::from_str("\"18446744073709551615\"").unwrap();

        assert_eq!(permission.bits(), u64::MAX);
        assert_eq!(
            serde_json::to_value(permission).unwrap(),
            serde_json::json!("18446744073709551615")
        );
    }

    /// Tests that invalid permission values are rejected.
    #[test]
    fn rejects_invalid_values() {
        assert!(serde_json::from_str::<Permission>("\"-1\"").is_err());
        assert!(serde_json::from_str::<Permission>("\"not a number\"").is_err());
        assert!(serde_json::from_str::<Permission>("-1").is_err());
        assert!(serde_json::from_str::<Permission>("\"18446744073709551616\"").is_err());
    }
}
