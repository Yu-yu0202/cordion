/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::emoji::Emoji;
use crate::guild::{GuildFeature, GuildId};
use crate::sticker::Sticker;

/// A preview of a discoverable guild.
///
/// The preview omits the roles, channels, and member data of a full
/// [`Guild`](super::Guild). The nullable fields below are sent explicitly as
/// `null` and are serialized as `null` when `None`.
///
/// See [Discord's guild preview object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#guild-preview-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildPreview {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's name.
    pub name: String,

    /// The guild's icon hash, or `None` if it has no icon.
    pub icon: Option<String>,

    /// The guild's splash hash, or `None` if it has no splash.
    pub splash: Option<String>,

    /// The guild's discovery splash hash, or `None` if it has none.
    pub discovery_splash: Option<String>,

    /// The guild's custom emoji.
    pub emojis: Vec<Emoji>,

    /// Enabled guild features.
    ///
    /// Unknown feature strings are preserved by [`GuildFeature`].
    pub features: Vec<GuildFeature>,

    /// The approximate number of members in the guild.
    pub approximate_member_count: u64,

    /// The approximate number of online members in the guild.
    pub approximate_presence_count: u64,

    /// The guild's description, or `None` if it has none.
    pub description: Option<String>,

    /// The guild's custom stickers.
    pub stickers: Vec<Sticker>,
}
