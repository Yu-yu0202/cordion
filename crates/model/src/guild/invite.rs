/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::guild::{GuildAgeRestrictionLevel, GuildFeature, GuildId, VerificationLevel};

/// A partial guild embedded in an invite object.
///
/// This is a smaller shape than [`PartialGuild`](super::PartialGuild), which is
/// returned by `GET /users/@me/guilds`, and than
/// [`WebhookSourceGuild`](crate::webhook::WebhookSourceGuild), which is embedded
/// in webhook `source_guild` objects. The nullable fields below are sent
/// explicitly as `null` by Discord and are therefore serialized as `null` when
/// `None`.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InviteGuild {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's name.
    pub name: String,

    /// The guild's icon hash, or `None` if it has no icon.
    pub icon: Option<String>,

    /// The guild's splash hash, or `None` if it has no splash.
    pub splash: Option<String>,

    /// The guild's banner hash, or `None` if it has no banner.
    pub banner: Option<String>,

    /// The guild's description, or `None` if it has none.
    pub description: Option<String>,

    /// Enabled guild features.
    ///
    /// Unknown feature strings are preserved by [`GuildFeature`].
    pub features: Vec<GuildFeature>,

    /// The verification level required to join the guild.
    pub verification_level: VerificationLevel,

    /// The guild's vanity URL code, or `None` if it has none.
    pub vanity_url_code: Option<String>,

    /// The guild's age-restriction level.
    pub nsfw_level: GuildAgeRestrictionLevel,

    /// The number of boosts the guild currently has.
    ///
    /// Discord may omit this field; an absent value decodes as `None` and is
    /// omitted during serialization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u64>,
}
