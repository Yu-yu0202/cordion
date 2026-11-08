/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord emoji resource.
//!
//! [`Emoji`] models the emoji object of the emoji endpoints and of the resources
//! that embed guild or application emoji. [`PartialEmoji`] models the reduced
//! shape used in message reactions and other embedded contexts, where every
//! field is optional and the name may be `null`.
//!
//! The endpoint bodies in this module are [`CreateGuildEmojiRequest`],
//! [`ModifyGuildEmojiRequest`], [`CreateApplicationEmojiRequest`], and
//! [`ModifyApplicationEmojiRequest`].
//!
//! See [Discord's emoji resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/emoji

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::role::{Role, RoleId};
use crate::user::User;

/// The ID of an [`Emoji`].
pub type EmojiId = Id<Emoji>;

/// A partial emoji embedded in another resource.
///
/// Discord uses this reduced shape in message reactions, buttons, select
/// options, polls, and other objects. Only the identity of the emoji is
/// present: [`id`](Self::id), [`name`](Self::name), and
/// [`animated`](Self::animated). Standard Unicode emoji normally have no ID,
/// while custom emoji always have one.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialEmoji {
    /// The emoji's ID, or `None` for a standard Unicode emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<EmojiId>,

    /// The emoji's name.
    ///
    /// For standard emoji this is the Unicode character(s); for custom emoji it
    /// is the uploaded name. Discord documents this field as nullable in
    /// reaction emoji objects, where the custom emoji data may no longer be
    /// available, so `None` is a valid reaction payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether this emoji is animated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

/// A custom or standard emoji returned by Discord.
///
/// Standard emoji have no [`id`](Self::id); custom emoji have an ID and may
/// include role restrictions and creation metadata.
///
/// [`name`](Self::name) is the only field Discord documents as nullable in this
/// object, and only for reaction emoji objects. Reaction emoji objects are
/// modeled by [`PartialEmoji`], which keeps every field optional; a guild or
/// application emoji always carries a name, so `None` here means the payload
/// came from a context this type does not describe.
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Emoji {
    /// The emoji's ID, or `None` for a standard Unicode emoji.
    ///
    /// Discord sends `null` for standard emoji; serialization preserves that.
    pub id: Option<Id<Emoji>>,

    /// The emoji name.
    ///
    /// For standard emoji this is the Unicode character(s); for custom emoji it
    /// is the uploaded name. Discord may send `null` for reaction emoji objects,
    /// which [`PartialEmoji`] models; `None` is serialized as `null`.
    pub name: Option<String>,

    /// Role IDs allowed to use this emoji.
    ///
    /// `None` means the emoji is available to everyone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<Id<Role>>>,

    /// The user that created this emoji, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,

    /// Whether this emoji must be wrapped in colons to be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_colons: Option<bool>,

    /// Whether this emoji is managed by an integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,

    /// Whether this emoji is animated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,

    /// Whether this emoji can be used.
    ///
    /// Discord may set this to `false` after a loss of Server Boosts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
}

/// The body of `POST /guilds/{guild.id}/emojis`.
///
/// [`name`](Self::name) and [`image`](Self::image) are required by Discord.
/// [`roles`](Self::roles) is optional and omitted during serialization when
/// `None`.
///
/// [`image`](Self::image) carries image data as a [Data URI scheme] string, for
/// example `data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA`. Discord
/// accepts JPEG, PNG, GIF, WebP, and AVIF uploads of at most 256 KiB and
/// requires a 128x128 image.
///
/// See [Discord's create guild emoji endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/emoji#create-guild-emoji
/// [Data URI scheme]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGuildEmojiRequest {
    /// The emoji's name.
    pub name: String,

    /// The 128x128 emoji image as image data.
    pub image: String,

    /// The role IDs allowed to use this emoji.
    ///
    /// Discord's parameter table lists this as a plain array of snowflakes
    /// rather than a nullable one, while the emoji object marks it optional. It
    /// is modeled as optional because an omitted array and an empty array both
    /// make the emoji available to everyone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleId>>,
}

/// The body of `PATCH /guilds/{guild.id}/emojis/{emoji.id}`.
///
/// Discord documents every parameter of this endpoint as optional, so every
/// field is omitted during serialization when `None` and an omitted field
/// leaves the emoji unchanged. Because omission means "leave unchanged", the
/// nullable [`roles`](Self::roles) cannot be cleared by sending `null` through
/// this type.
///
/// See [Discord's modify guild emoji endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/emoji#modify-guild-emoji
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildEmojiRequest {
    /// The emoji's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The role IDs allowed to use this emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleId>>,
}

/// The body of `POST /applications/{application.id}/emojis`.
///
/// [`name`](Self::name) and [`image`](Self::image) are required by Discord.
///
/// [`image`](Self::image) carries image data as a [Data URI scheme] string, for
/// example `data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA`. Discord
/// accepts JPEG, PNG, GIF, WebP, and AVIF uploads of at most 256 KiB and
/// requires a 128x128 image.
///
/// See [Discord's create application emoji endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/emoji#create-application-emoji
/// [Data URI scheme]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateApplicationEmojiRequest {
    /// The emoji's name.
    pub name: String,

    /// The 128x128 emoji image as image data.
    pub image: String,
}

/// The body of `PATCH /applications/{application.id}/emojis/{emoji.id}`.
///
/// Discord documents [`name`](Self::name) as the only parameter of this
/// endpoint, so it is omitted during serialization when `None`.
///
/// See [Discord's modify application emoji endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/emoji#modify-application-emoji
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyApplicationEmojiRequest {
    /// The emoji's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
