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

use crate::application::ApplicationId;
use crate::emoji::EmojiId;

bitflags! {
    /// Flags describing the contents of an [`Activity`].
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// activity flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/events/gateway-events#activity-object-activity-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct ActivityFlags: u64 {
        /// No activity flags are set.
        const NONE = 0;

        /// The activity is an instanced game session.
        const INSTANCE = 1 << 0;

        /// The activity supports joining.
        const JOIN = 1 << 1;

        /// The activity supports spectating.
        const SPECTATE = 1 << 2;

        /// The activity supports join requests.
        const JOIN_REQUEST = 1 << 3;

        /// The activity state is synchronized.
        const SYNC = 1 << 4;

        /// The activity supports playing.
        const PLAY = 1 << 5;

        /// The activity's party is visible to friends only.
        const PARTY_PRIVACY_FRIENDS = 1 << 6;

        /// The activity's party is visible to the voice channel only.
        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;

        /// The activity is an embedded application.
        const EMBEDDED = 1 << 8;
    }
}

impl Serialize for ActivityFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for ActivityFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

/// The type of an [`Activity`].
///
/// Unrecognized values are preserved. See [Discord's activity types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/events/gateway-events#activity-object-activity-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ActivityType(u8);

impl ActivityType {
    /// Playing `{name}`.
    pub const PLAYING: Self = Self(0);

    /// Streaming `{details}`.
    pub const STREAMING: Self = Self(1);

    /// Listening to `{name}`.
    pub const LISTENING: Self = Self(2);

    /// Watching `{name}`.
    pub const WATCHING: Self = Self(3);

    /// A custom status using `{emoji}` and `{state}`.
    pub const CUSTOM: Self = Self(4);

    /// Competing in `{name}`.
    pub const COMPETING: Self = Self(5);
}

/// Which activity field is shown in a user's status text.
///
/// Unrecognized values are preserved. See [Discord's status display
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/events/gateway-events#activity-object-status-display-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct StatusDisplayType(u8);

impl StatusDisplayType {
    /// Show the activity's name.
    pub const NAME: Self = Self(0);

    /// Show the activity's state.
    pub const STATE: Self = Self(1);

    /// Show the activity's details.
    pub const DETAILS: Self = Self(2);
}

/// A user's current activity.
///
/// The `buttons` field is direction-dependent: an array of strings when
/// received over the Gateway, and an array of `{label, url}` objects when
/// sending. See [`Activity::buttons`] for the details.
///
/// Optional fields decode as `None` when absent or `null`, and are omitted
/// during serialization. See [Discord's activity object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway-events#activity-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Activity {
    /// The activity's name.
    pub name: String,

    /// The activity's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ActivityType,

    /// The stream URL, validated by Discord when [`kind`](Self::kind) is
    /// [`ActivityType::STREAMING`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// When the activity was added to the user's session, as Unix milliseconds.
    pub created_at: u64,

    /// Start and end times for the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,

    /// The application ID for the game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    /// Which activity field is shown in the user's status text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_display_type: Option<StatusDisplayType>,

    /// What the player is currently doing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// The URL linked when clicking the details text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_url: Option<String>,

    /// The user's current party status, or the text of a custom status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// The URL linked when clicking the state text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_url: Option<String>,

    /// The emoji used for a custom status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ActivityEmoji>,

    /// Information about the player's current party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,

    /// Images shown for the presence and their hover texts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<ActivityAssets>,

    /// Secrets for joining and spectating the activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<ActivitySecrets>,

    /// Whether the activity is an instanced game session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<bool>,

    /// Flags describing what the activity payload includes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ActivityFlags>,

    /// Custom button labels shown in the Rich Presence, up to two.
    ///
    /// The wire shape depends on the direction: when received over the Gateway
    /// this is an array of the button labels, because bots cannot access a
    /// user's activity button URLs. When sending, Discord requires an array of
    /// objects with a `label` (1-32 characters) and a `url` (1-512 characters).
    /// This type therefore keeps only the received representation, and a request
    /// that sets buttons must build that array itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<String>>,
}

/// Unix timestamps for the start and end of an [`Activity`].
///
/// See [Discord's activity timestamps][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway-events#activity-object-activity-timestamps
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ActivityTimestamps {
    /// When the activity started, as Unix milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,

    /// When the activity ends, as Unix milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
}

/// The emoji used for a custom [`Activity`] status.
///
/// See [Discord's activity emoji object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway-events#activity-object-activity-emoji
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ActivityEmoji {
    /// The emoji's name.
    pub name: String,

    /// The ID of a custom emoji, or `None` for a standard emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<EmojiId>,

    /// Whether the custom emoji is animated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animated: Option<bool>,
}

/// Information about the party of an [`Activity`].
///
/// See [Discord's activity party object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway-events#activity-object-activity-party
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ActivityParty {
    /// The party's ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The party's current and maximum size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<ActivityPartySize>,
}

/// The current and maximum size of an [`ActivityParty`].
///
/// Discord encodes this as a two-element array `[current, max]`. The custom
/// `Serialize` and `Deserialize` implementations preserve that shape.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ActivityPartySize {
    /// The number of users currently in the party.
    pub current: u32,

    /// The maximum number of users allowed in the party.
    pub max: u32,
}

impl Serialize for ActivityPartySize {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        [self.current, self.max].serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ActivityPartySize {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let [current, max] = <[u32; 2]>::deserialize(deserializer)?;

        Ok(Self { current, max })
    }
}

/// Images shown for an [`Activity`] and their hover texts.
///
/// Asset values are arbitrary user-specifiable strings; treat them carefully.
/// See [Discord's activity assets object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway-events#activity-object-activity-assets
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ActivityAssets {
    /// The large image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,

    /// The text shown when hovering over the large image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,

    /// The URL opened when clicking the large image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_url: Option<String>,

    /// The small image asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,

    /// The text shown when hovering over the small image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,

    /// The URL opened when clicking the small image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_url: Option<String>,

    /// The image displayed as a banner on a game invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_cover_image: Option<String>,
}

/// Secrets for joining and spectating an [`Activity`].
///
/// See [Discord's activity secrets object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway-events#activity-object-activity-secrets
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ActivitySecrets {
    /// The secret for joining a party.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join: Option<String>,

    /// The secret for spectating a game.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectate: Option<String>,

    /// The secret for a specific instanced match.
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_secret: Option<String>,
}
