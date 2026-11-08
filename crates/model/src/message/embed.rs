/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Embed types carried by messages.
//!
//! [`Embed`] models the embed object together with its nested
//! [`EmbedAuthor`], [`EmbedFooter`], [`EmbedField`], [`EmbedImage`],
//! [`EmbedVideo`], and [`EmbedProvider`] structures, and the [`EmbedFlags`] and
//! [`EmbedMediaFlags`] flag sets.
//!
//! Content-scanning flags may be added after the message is sent. Unrecognized
//! bits are retained during deserialization, and unrecognized embed types stay
//! usable because the embed type is modeled as a plain string.
//!
//! See [Discord's embed object][object] for details.
//!
//! [object]: https://docs.discord.com/developers/resources/message#embed-object

use std::time::SystemTime;

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::component::Component;

/// An embedded message.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's embed object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#embed-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Embed {
    /// The embed's title, up to 256 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The embed's type, such as `"rich"` or `"poll_result"`, serialized as
    /// `type`.
    ///
    /// Stored as a string so new embed types remain usable.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The embed's description, up to 4096 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The embed's URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The timestamp shown in the embed.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub timestamp: Option<SystemTime>,

    /// The embed's color as an RGB integer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,

    /// The embed footer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<EmbedFooter>,

    /// The embed image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<EmbedImage>,

    /// The embed thumbnail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<EmbedImage>,

    /// The embed video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<EmbedVideo>,

    /// The embed provider.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<EmbedProvider>,

    /// The embed author.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<EmbedAuthor>,

    /// The embed fields, up to 25.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<EmbedField>>,

    /// Components that make up a component embed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// Embed flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<EmbedFlags>,
}

bitflags! {
    /// Flags attached to an [`Embed`].
    ///
    /// Content-scanning flags may be added after the message is sent.
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// embed flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/message#embed-object-embed-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct EmbedFlags: u64 {
        /// No embed flags are set.
        const NONE = 0;

        /// The embed's media was flagged as explicit.
        const CONTAINS_EXPLICIT_CONTENT = 1 << 4;

        /// The embed is a fallback for a reply to an activity card.
        const IS_CONTENT_INVENTORY_ENTRY = 1 << 5;

        /// The embed's media was flagged as gore.
        const CONTAINS_GORE_CONTENT = 1 << 6;

        /// The embed's media was flagged as self-harm.
        const CONTAINS_SELF_HARM_CONTENT = 1 << 7;
    }
}

impl Serialize for EmbedFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for EmbedFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

/// An embed's video.
///
/// See [Discord's embed video structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#embed-object-embed-video-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EmbedVideo {
    /// The source URL of the video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// A proxied URL of the video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,

    /// The video's height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,

    /// The video's width.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,

    /// The video's media type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// A `ThumbHash` placeholder for the video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The version of [`placeholder`](Self::placeholder).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u64>,

    /// The video's description or alt text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Embed media flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<EmbedMediaFlags>,
}

/// An embed's image or thumbnail.
///
/// See [Discord's embed image structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#embed-object-embed-image-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EmbedImage {
    /// The source URL of the image.
    ///
    /// Only `http(s)` URLs and `attachment://` references are supported.
    pub url: String,

    /// A proxied URL of the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,

    /// The image's height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,

    /// The image's width.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,

    /// The image's media type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// A `ThumbHash` placeholder for the image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The version of [`placeholder`](Self::placeholder).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u64>,

    /// The image's description or alt text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Embed media flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<EmbedMediaFlags>,
}

bitflags! {
    /// Flags attached to embed media.
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// embed media flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/message#embed-object-embed-media-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct EmbedMediaFlags: u64 {
        /// No embed media flags are set.
        const NONE = 0;

        /// The image is animated.
        const IS_ANIMATED = 1 << 5;
    }
}

impl Serialize for EmbedMediaFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for EmbedMediaFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

/// An embed's provider.
///
/// See [Discord's embed provider structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#embed-object-embed-provider-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EmbedProvider {
    /// The provider's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The provider's URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// An embed's author.
///
/// See [Discord's embed author structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#embed-object-embed-author-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EmbedAuthor {
    /// The author's name, up to 256 characters.
    pub name: String,

    /// The author's URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The author's icon URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    /// A proxied URL of the author's icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// An embed's footer.
///
/// See [Discord's embed footer structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#embed-object-embed-footer-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EmbedFooter {
    /// The footer text, up to 2048 characters.
    pub text: String,

    /// The footer icon URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,

    /// A proxied URL of the footer icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<String>,
}

/// One field in an [`Embed`].
///
/// See [Discord's embed field structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#embed-object-embed-field-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct EmbedField {
    /// The field's name, up to 256 characters.
    pub name: String,

    /// The field's value, up to 1024 characters.
    pub value: String,

    /// Whether the field should display inline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inline: Option<bool>,
}
