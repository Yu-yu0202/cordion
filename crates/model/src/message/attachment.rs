/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Attachment types carried by messages.
//!
//! [`Attachment`] models the attachment object Discord sends with a message.
//! Optional fields decode as `None` when absent or `null` and are omitted
//! during serialization. The `CONTAINS_*` flags may be added after the message
//! is sent when content scanning runs.
//!
//! [`AttachmentRequest`] and [`AttachmentRequestId`] model the attachment
//! request structure used in message create and edit request bodies, which
//! unlike [`Attachment`] omits server-populated URLs and metadata.
//!
//! See [Discord's attachment object][object] and [Discord's attachment request
//! structure][request] for details.
//!
//! [object]: https://docs.discord.com/developers/resources/message#attachment-object
//! [request]: https://docs.discord.com/developers/resources/message#attachment-object-attachment-request-structure

use std::str::FromStr as _;
use std::time::SystemTime;

use bitflags::bitflags;
use cordion_macro::IdTarget;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::application::PartialApplication;
use crate::id::{Id, Identifiable};
use crate::user::User;

/// The ID of an [`Attachment`].
pub type AttachmentId = Id<Attachment>;

/// A file attached to a message.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization. The `CONTAINS_*` flags may be added after the message
/// is sent when content scanning runs.
///
/// See [Discord's attachment object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#attachment-object
#[derive(Clone, PartialEq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Attachment {
    /// The attachment's ID.
    pub id: AttachmentId,

    /// The attached file's name.
    pub filename: String,

    /// The title of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The file's description or alt text, up to 1024 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The attachment's media type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The file size in bytes.
    pub size: u64,

    /// The source URL of the file.
    pub url: String,

    /// A proxied URL of the file.
    ///
    /// Images, videos, and audio files are proxied; other types return an
    /// error from the proxy.
    pub proxy_url: String,

    /// The image or video height.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,

    /// The image or video width.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,

    /// A `ThumbHash` placeholder for the media.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The version of [`placeholder`](Self::placeholder).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u64>,

    /// Whether the attachment is ephemeral.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral: Option<bool>,

    /// The duration of an audio or video file, in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f64>,

    /// A base64-encoded sampled waveform, currently used for voice messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,

    /// Attachment flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<AttachmentFlags>,

    /// Users who participated in the stream, for Clips.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_participants: Option<Vec<User>>,

    /// When the clip was created, for Clips.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub clip_created_at: Option<SystemTime>,

    /// The recognized application in the stream, for Clips.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<PartialApplication>,
}

impl Identifiable for Attachment {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

bitflags! {
    /// Flags attached to an [`Attachment`].
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// attachment flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/message#attachment-object-attachment-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct AttachmentFlags: u64 {
        /// No attachment flags are set.
        const NONE = 0;

        /// The attachment is a Clip from a stream.
        const IS_CLIP = 1 << 0;

        /// The attachment is a media channel's thread thumbnail.
        const IS_THUMBNAIL = 1 << 1;

        /// The attachment was edited with the deprecated remix feature.
        const IS_REMIX = 1 << 2;

        /// The attachment was marked as a spoiler.
        const IS_SPOILER = 1 << 3;

        /// Discord's content scanning flagged the attachment as explicit.
        const CONTAINS_EXPLICIT_CONTENT = 1 << 4;

        /// The attachment is an animated image.
        const IS_ANIMATED = 1 << 5;

        /// Discord's content scanning flagged the attachment as gore.
        const CONTAINS_GORE_CONTENT = 1 << 6;

        /// Discord's content scanning flagged the attachment as self-harm.
        const CONTAINS_SELF_HARM_CONTENT = 1 << 7;
    }
}

impl Serialize for AttachmentFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for AttachmentFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

/// The identifier of an attachment in a message create or edit request.
///
/// Discord accepts either the snowflake of an existing attachment (as a
/// string) or the numeric index of a newly uploaded file matching `files[n]`.
/// The two representations are kept distinct because the numeric form is not a
/// snowflake.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttachmentRequestId {
    /// An existing attachment's snowflake ID.
    Snowflake(AttachmentId),

    /// The index of a newly uploaded file.
    Index(u64),
}

impl Serialize for AttachmentRequestId {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Snowflake(id) => id.serialize(serializer),
            Self::Index(index) => serializer.serialize_u64(*index),
        }
    }
}

impl<'de> Deserialize<'de> for AttachmentRequestId {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            Snowflake(String),
            Index(u64),
        }

        match Repr::deserialize(deserializer)? {
            Repr::Snowflake(value) => {
                AttachmentId::from_str(&value).map(Self::Snowflake).map_err(D::Error::custom)
            },
            Repr::Index(index) => Ok(Self::Index(index)),
        }
    }
}

/// An attachment in a message create or edit request.
///
/// Unlike [`Attachment`], this request shape omits server-populated URLs and
/// metadata and adds [`is_spoiler`](Self::is_spoiler). Optional fields are
/// omitted during serialization when `None`.
///
/// See [Discord's attachment request structure][request] for details.
///
/// [request]: https://docs.discord.com/developers/resources/message#attachment-object-attachment-request-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AttachmentRequest {
    /// The existing attachment's snowflake or a new upload's index.
    pub id: AttachmentRequestId,

    /// The file's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// The file's title.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// The file's description or alt text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The audio or video duration, required for voice messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f64>,

    /// A base64-encoded waveform, required for voice messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waveform: Option<String>,

    /// Whether the attachment should be marked as a spoiler.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_spoiler: Option<bool>,
}
