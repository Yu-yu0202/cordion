/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::guild::PartialIntegration;

/// A third-party connection attached to a user's Discord account.
///
/// Connections are returned by `GET /users/@me/connections`, which requires the
/// `connections` `OAuth2` scope. See [Discord's connection object][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/resources/user#connection-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Connection {
    /// The connection account's ID.
    ///
    /// This is an arbitrary string rather than a Discord snowflake.
    pub id: String,

    /// The username of the connection account.
    pub name: String,

    /// The service of this connection, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ConnectionService,

    /// Whether the connection is revoked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,

    /// The partial guild integrations associated with this connection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integrations: Option<Vec<PartialIntegration>>,

    /// Whether the connection is verified.
    pub verified: bool,

    /// Whether friend sync is enabled for this connection.
    pub friend_sync: bool,

    /// Whether activities related to this connection are shown in presence
    /// updates.
    pub show_activity: bool,

    /// Whether this connection has a corresponding third-party `OAuth2` token.
    pub two_way_link: bool,

    /// The connection's visibility.
    pub visibility: ConnectionVisibility,
}

/// The visibility of a [`Connection`].
///
/// Unrecognized values are preserved. See [Discord's visibility types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/resources/user#connection-object-visibility-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ConnectionVisibility(u8);

impl ConnectionVisibility {
    /// The connection is invisible to everyone except the user themselves.
    pub const NONE: Self = Self(0);

    /// The connection is visible to everyone.
    pub const EVERYONE: Self = Self(1);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// The service of a [`Connection`].
///
/// Documented services are exposed as associated constants, and unrecognized
/// values are preserved when decoding. See [Discord's connection services][services]
/// for details.
///
/// [services]: https://docs.discord.com/developers/resources/user#connection-object-services
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ConnectionService(Cow<'static, str>);

impl ConnectionService {
    /// Amazon Music.
    pub const AMAZON_MUSIC: Self = Self(Cow::Borrowed("amazon-music"));

    /// Bungie.net.
    pub const BUNGIE: Self = Self(Cow::Borrowed("bungie"));

    /// Bluesky.
    pub const BLUESKY: Self = Self(Cow::Borrowed("bluesky"));

    /// Crunchyroll.
    pub const CRUNCHYROLL: Self = Self(Cow::Borrowed("crunchyroll"));

    /// Domain.
    pub const DOMAIN: Self = Self(Cow::Borrowed("domain"));

    /// eBay.
    pub const EBAY: Self = Self(Cow::Borrowed("ebay"));

    /// Epic Games.
    pub const EPICGAMES: Self = Self(Cow::Borrowed("epicgames"));

    /// Facebook.
    pub const FACEBOOK: Self = Self(Cow::Borrowed("facebook"));

    /// GitHub.
    pub const GITHUB: Self = Self(Cow::Borrowed("github"));

    /// Instagram.
    ///
    /// This service can no longer be added by users.
    pub const INSTAGRAM: Self = Self(Cow::Borrowed("instagram"));

    /// Mastodon.
    pub const MASTODON: Self = Self(Cow::Borrowed("mastodon"));

    /// `PayPal`.
    pub const PAYPAL: Self = Self(Cow::Borrowed("paypal"));

    /// `PlayStation` Network.
    pub const PLAYSTATION: Self = Self(Cow::Borrowed("playstation"));

    /// Reddit.
    pub const REDDIT: Self = Self(Cow::Borrowed("reddit"));

    /// Roblox.
    pub const ROBLOX: Self = Self(Cow::Borrowed("roblox"));

    /// Spotify.
    pub const SPOTIFY: Self = Self(Cow::Borrowed("spotify"));

    /// Skype.
    ///
    /// This service can no longer be added by users.
    pub const SKYPE: Self = Self(Cow::Borrowed("skype"));

    /// Steam.
    pub const STEAM: Self = Self(Cow::Borrowed("steam"));

    /// `TikTok`.
    pub const TIKTOK: Self = Self(Cow::Borrowed("tiktok"));

    /// Twitch.
    pub const TWITCH: Self = Self(Cow::Borrowed("twitch"));

    /// X (formerly Twitter).
    pub const TWITTER: Self = Self(Cow::Borrowed("twitter"));

    /// Xbox.
    pub const XBOX: Self = Self(Cow::Borrowed("xbox"));

    /// `YouTube`.
    pub const YOUTUBE: Self = Self(Cow::Borrowed("youtube"));

    /// Creates an owned service from any string-like value.
    ///
    /// Prefer [`ConnectionService::from`] with a `&'static str` to borrow
    /// instead of allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the service exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for ConnectionService {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
