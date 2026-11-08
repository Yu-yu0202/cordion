/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::guild::{
    GuildAgeRestrictionLevel, GuildFeature, GuildId, PremiumTier, VerificationLevel,
};
use crate::user::UserId;

/// A partial guild object embedded in an [`Application`](super::Application).
///
/// Discord describes the application's `guild` field only as a partial guild
/// object and does not publish a dedicated schema for it. Only the guild ID is
/// required here; the remaining commonly included fields are optional. Fields
/// that are not modeled are ignored during deserialization, and optional fields
/// are omitted during serialization when absent.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationGuild {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The guild's icon hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The guild's invite splash hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splash: Option<String>,

    /// The guild's banner hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The guild's description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Enabled guild features.
    ///
    /// Unknown feature strings are preserved by [`GuildFeature`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<GuildFeature>>,

    /// The verification level required to join the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,

    /// The guild's vanity URL code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<String>,

    /// Whether the guild is age-restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// The guild's age-restriction level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw_level: Option<GuildAgeRestrictionLevel>,

    /// The ID of the guild's owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<UserId>,

    /// The guild's preferred locale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,

    /// The guild's premium tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_tier: Option<PremiumTier>,
}
