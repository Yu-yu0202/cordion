/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord sticker resource.
//!
//! See [Discord's sticker resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/sticker

mod kind;

use cordion_macro::IdTarget;
pub use kind::{StickerFormatType, StickerType};
use serde::{Deserialize, Serialize};

use crate::guild::Guild;
use crate::id::{Id, Identifiable, snowflake_option};
use crate::monetization::sku::SkuId;
use crate::user::User;

/// The ID of a [`Sticker`].
pub type StickerId = Id<Sticker>;

/// The ID of a [`StickerPack`].
pub type StickerPackId = Id<StickerPack>;

/// A standard sticker pack.
///
/// See [Discord's sticker pack object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/sticker#sticker-pack-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct StickerPack {
    /// The sticker pack's ID.
    pub id: StickerPackId,

    /// The stickers in the pack.
    pub stickers: Vec<Sticker>,

    /// The sticker pack's name.
    pub name: String,

    /// The ID of the pack's SKU.
    pub sku_id: SkuId,

    /// The ID of the sticker shown as the pack's icon.
    ///
    /// This is optional and omitted during serialization when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_sticker_id: Option<StickerId>,

    /// The sticker pack's description.
    pub description: String,

    /// The ID of the sticker pack's banner image.
    ///
    /// This is a snowflake-formatted decimal string but does not refer to a
    /// sticker, so it is exposed as a raw `u64`. The field is optional and
    /// omitted during serialization when `None`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "snowflake_option"
    )]
    pub banner_asset_id: Option<u64>,
}

impl Identifiable for StickerPack {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The smallest amount of data required to render a sticker in a message.
///
/// This is a partial sticker object used by the message's `sticker_items`
/// field. See [Discord's sticker item object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/sticker#sticker-item-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct StickerItem {
    /// The sticker's ID.
    pub id: StickerId,

    /// The sticker's name.
    pub name: String,

    /// The sticker's format.
    pub format_type: StickerFormatType,
}

/// A sticker that can be sent in messages.
///
/// Standard stickers belong to a pack; guild stickers belong to a guild and may
/// include uploader information.
///
/// See [Discord's sticker object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/sticker#sticker-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Sticker {
    /// The sticker's ID.
    pub id: Id<Sticker>,

    /// The ID of the pack this standard sticker belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pack_id: Option<StickerPackId>,

    /// The sticker's name.
    pub name: String,

    /// The sticker's description.
    ///
    /// Discord may send `null` or omit this field; both decode as `None`.
    /// Serialization always emits `null` when this is `None`.
    pub description: Option<String>,

    /// Autocomplete and suggestion tags for the sticker.
    ///
    /// Standard stickers conventionally use a comma-separated list, and Discord
    /// limits this field to 200 characters.
    pub tags: String,

    /// The sticker's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: StickerType,

    /// The sticker's format.
    pub format_type: StickerFormatType,

    /// Whether this guild sticker can be used.
    ///
    /// Discord may set this to `false` after a loss of Server Boosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,

    /// The ID of the guild that owns this sticker.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<Guild>>,

    /// The user that uploaded this guild sticker, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,

    /// The standard sticker's sort order within its pack.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_value: Option<i32>,
}

/// The response of `GET /sticker-packs`.
///
/// See [Discord's list sticker packs endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/sticker#list-sticker-packs
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct StickerPackList {
    /// The available sticker packs.
    pub sticker_packs: Vec<StickerPack>,
}

/// The parameters of `POST /guilds/{guild.id}/stickers`.
///
/// The endpoint sends these fields as `multipart/form-data` together with the
/// sticker file, and returns the created sticker.
///
/// See [Discord's create guild sticker endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/sticker#create-guild-sticker
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGuildStickerRequest {
    /// The sticker's name, between 2 and 30 characters.
    pub name: String,

    /// The sticker's description, empty or between 2 and 100 characters.
    pub description: String,

    /// The sticker's autocomplete and suggestion tags.
    pub tags: String,
}
