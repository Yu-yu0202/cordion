/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Content components and unfurled media.

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::component::ComponentType;
use crate::message::AttachmentId;

bitflags! {
    /// Flags attached to an [`UnfurledMediaItem`].
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// unfurled media item flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/components/reference#unfurled-media-item-unfurled-media-item-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct UnfurledMediaItemFlags: u64 {
        /// No media flags are set.
        const NONE = 0;

        /// The image is animated.
        const IS_ANIMATED = 1 << 0;
    }
}

impl Serialize for UnfurledMediaItemFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for UnfurledMediaItemFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

bitflags! {
    /// Content-scanning flags for an [`UnfurledMediaItem`].
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// content scan metadata flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/components/reference#unfurled-media-item-content-scan-metadata-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct ContentScanMetadataFlags: u64 {
        /// No content scan flags are set.
        const NONE = 0;

        /// The media was flagged as explicit by Discord's content scanning.
        const IS_EXPLICIT_CONTENT = 1 << 0;

        /// The media was flagged as gore by Discord's content scanning.
        const IS_GORE_CONTENT = 1 << 1;

        /// The media was flagged as self-harm by Discord's content scanning.
        const IS_SELF_HARM_CONTENT = 1 << 2;
    }
}

impl Serialize for ContentScanMetadataFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for ContentScanMetadataFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

/// Results of Discord's content scanning for an [`UnfurledMediaItem`].
///
/// See [Discord's content scan metadata object][object] for details.
///
/// [object]: https://docs.discord.com/developers/components/reference#unfurled-media-item-content-scan-metadata-structure
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ContentScanMetadata {
    /// Flags describing the scan result.
    pub flags: ContentScanMetadataFlags,
}

/// A URL-backed piece of media used in a component.
///
/// Only [`url`](Self::url) is settable by developers; Discord populates the
/// remaining fields in responses. The URL may be an arbitrary direct link or
/// an `attachment://<filename>` reference.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's unfurled media item object][object] for details.
///
/// [object]: https://docs.discord.com/developers/components/reference#unfurled-media-item
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UnfurledMediaItem {
    /// The media URL or an `attachment://` reference.
    pub url: String,

    /// The proxied URL of the media.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<String>,

    /// The media's height, when it is an image or video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,

    /// The media's width, when it is an image or video.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,

    /// A `ThumbHash` placeholder for the media.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The version of [`placeholder`](Self::placeholder).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u64>,

    /// The media's content type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Flags attached to the media.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<UnfurledMediaItemFlags>,

    /// Results of Discord's content scanning, when the media was scanned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_scan_metadata: Option<ContentScanMetadata>,

    /// The ID of the uploaded attachment backing this media item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<AttachmentId>,
}

/// Markdown text displayed in a message or modal.
///
/// See [Discord's text display component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#text-display
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TextDisplay {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::text_display"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// Markdown text displayed to the user.
    pub content: String,
}

/// A small image used as a section accessory.
///
/// See [Discord's thumbnail component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#thumbnail
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::thumbnail")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The thumbnail's media.
    pub media: UnfurledMediaItem,

    /// Alternative text for the media, up to 1024 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the thumbnail is hidden as a spoiler.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,
}

/// A gallery of media items.
///
/// See [Discord's media gallery component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#media-gallery
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MediaGallery {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::media_gallery"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The gallery's items, from 1 to 10.
    pub items: Vec<MediaGalleryItem>,
}

/// One item in a [`MediaGallery`].
///
/// See [Discord's media gallery item object][object] for details.
///
/// [object]: https://docs.discord.com/developers/components/reference#media-gallery-media-gallery-item-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MediaGalleryItem {
    /// The item's media.
    pub media: UnfurledMediaItem,

    /// Alternative text for the media, up to 1024 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the media is hidden as a spoiler.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,
}

/// An attached file displayed as a component.
///
/// Unlike other media components, the [`file`](Self::file) URL only supports
/// `attachment://<filename>` references.
///
/// See [Discord's file component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#file
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct FileComponent {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::file")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The file's media item.
    pub file: UnfurledMediaItem,

    /// Whether the file is hidden as a spoiler.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,

    /// The file's name.
    ///
    /// Discord ignores this value in requests and populates it in responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The file's size in bytes.
    ///
    /// Discord ignores this value in requests and populates it in responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}
