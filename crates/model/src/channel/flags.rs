/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Flags attached to a [`Channel`](super::Channel).
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// channel flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/channel#channel-object-channel-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct ChannelFlags: u64 {
        /// No channel flags are set.
        const NONE = 0;

        /// This thread is pinned to the top of its parent forum or media
        /// channel.
        const PINNED = 1 << 1;

        /// A tag is required when creating a thread in a forum or media
        /// channel.
        const REQUIRE_TAG = 1 << 4;

        /// Hide the embedded media download options.
        ///
        /// Only available for media channels.
        const HIDE_MEDIA_DOWNLOAD_OPTIONS = 1 << 15;

        /// The channel's metadata was obfuscated because the current user
        /// cannot view it.
        ///
        /// Only set on channels received over the Gateway; the HTTP API never
        /// sets this flag. When it is set, only `id`, `type`, `position`, and
        /// `parent_id` can be relied on.
        const CHANNEL_OBFUSCATED = 1 << 17;

        /// The channel is a spoiler channel that users must opt in to view.
        ///
        /// Can be set on textual guild channels and voice channels, but not
        /// stage channels, and only when the channel is not age-restricted.
        const IS_SPOILER_CHANNEL = 1 << 21;
    }
}

impl Serialize for ChannelFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for ChannelFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}
