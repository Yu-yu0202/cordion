/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Numeric guild configuration levels.
//!
//! Each type wraps Discord's integer value and preserves unrecognized values.

use serde::{Deserialize, Serialize};

/// The default message notification level.
///
/// Unrecognized values are preserved. See [Discord's documentation][levels].
///
/// [levels]: https://docs.discord.com/developers/resources/guild#guild-object-default-message-notification-level
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DefaultMessageNotificationLevel(u8);

impl DefaultMessageNotificationLevel {
    /// Members receive notifications for all messages by default.
    pub const ALL_MESSAGES: Self = Self(0);

    /// Members only receive notifications for messages that mention them.
    pub const ONLY_MENTIONS: Self = Self(1);
}

/// The explicit content filter level.
///
/// Unrecognized values are preserved. See [Discord's documentation][levels].
///
/// [levels]: https://docs.discord.com/developers/resources/guild#guild-object-explicit-content-filter-level
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ExplicitContentFilterLevel(u8);

impl ExplicitContentFilterLevel {
    /// Media content is not scanned.
    pub const NONE: Self = Self(0);

    /// Media content sent by members without roles is scanned.
    pub const MEMBERS_WITHOUT_ROLES: Self = Self(1);

    /// Media content sent by all members is scanned.
    pub const ALL_MEMBERS: Self = Self(2);
}

/// The MFA (Multi-Factor Authentication) level required for moderation actions.
///
/// Unrecognized values are preserved. See [Discord's documentation][levels].
///
/// [levels]: https://docs.discord.com/developers/resources/guild#guild-object-mfa-level
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MfaLevel(u8);

impl MfaLevel {
    /// The guild has no MFA requirement.
    pub const NONE: Self = Self(0);

    /// The guild requires MFA for moderation actions.
    pub const ELEVATED: Self = Self(1);
}

/// The verification level required to join the guild.
///
/// Unrecognized values are preserved. See [Discord's documentation][levels].
///
/// [levels]: https://docs.discord.com/developers/resources/guild#guild-object-verification-level
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct VerificationLevel(u8);

impl VerificationLevel {
    /// Joining is unrestricted.
    pub const NONE: Self = Self(0);

    /// The user must have a verified email address.
    pub const LOW: Self = Self(1);

    /// The user must have been registered on Discord for more than five
    /// minutes.
    pub const MEDIUM: Self = Self(2);

    /// The user must have been a member of the guild for more than ten minutes.
    pub const HIGH: Self = Self(3);

    /// The user must have a verified phone number.
    pub const VERY_HIGH: Self = Self(4);
}

/// The guild's age-restriction level.
///
/// Unrecognized values are preserved. See [Discord's documentation][levels].
///
/// [levels]: https://docs.discord.com/developers/resources/guild#guild-object-guild-nsfw-level
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GuildAgeRestrictionLevel(u8);

impl GuildAgeRestrictionLevel {
    /// The guild has no age restriction (Discord's `DEFAULT`).
    pub const NONE: Self = Self(0);

    /// The guild is explicitly marked as age-restricted.
    pub const EXPLICIT: Self = Self(1);

    /// The guild is marked as safe for all ages.
    pub const SAFE: Self = Self(2);

    /// The guild is age-restricted for users under 18.
    pub const AGE_RESTRICTED: Self = Self(3);
}

/// The guild's Server Boost tier.
///
/// Unrecognized values are preserved. See [Discord's documentation][levels].
///
/// [levels]: https://docs.discord.com/developers/resources/guild#guild-object-premium-tier
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PremiumTier(u8);

impl PremiumTier {
    /// The guild has not unlocked any Server Boost perks.
    pub const NONE: Self = Self(0);

    /// The guild has unlocked Server Boost level 1 perks.
    pub const TIER_1: Self = Self(1);

    /// The guild has unlocked Server Boost level 2 perks.
    pub const TIER_2: Self = Self(2);

    /// The guild has unlocked Server Boost level 3 perks.
    pub const TIER_3: Self = Self(3);
}
