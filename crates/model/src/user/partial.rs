/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::user::{AvatarDecorationData, UserFlags, UserId};

/// A partial user object.
///
/// Discord uses partial user objects in several places, including presence
/// updates and an application's `bot` and `owner` fields. Those contexts do not
/// guarantee a fixed set of fields, so this type requires only the ID and models
/// the commonly included fields as optional. Decoding still fails if a field
/// that is present has an unexpected JSON type; use a raw JSON representation
/// if values must be preserved regardless of their shape.
///
/// Optional fields decode as `None` when absent or `null`, and are omitted
/// during serialization. A partial user object never carries a guild member
/// object; see [`User::member`] for the mentions case, which this type does not
/// cover.
///
/// See [Discord's presence update][presence] for details.
///
/// [presence]: https://docs.discord.com/developers/events/gateway-events#presence-update
/// [`User::member`]: crate::user::User::member
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialUser {
    /// The user's ID.
    pub id: UserId,

    /// The user's username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// The user's legacy Discord tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminator: Option<String>,

    /// The user's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,

    /// The user's avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// Whether the user belongs to an `OAuth2` application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,

    /// Whether this is an Official Discord System user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,

    /// All flags on the user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,

    /// The public flags on the user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,

    /// The user's equipped avatar decoration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

impl PartialUser {
    /// Returns whether this is a bot account.
    ///
    /// An absent `bot` field is treated as `false`, matching [`User::is_bot`].
    ///
    /// [`User::is_bot`]: crate::user::User::is_bot
    #[inline]
    #[must_use]
    pub fn is_bot(&self) -> bool {
        self.bot.unwrap_or(false)
    }

    /// Returns whether this is an Official Discord System user.
    ///
    /// An absent `system` field is treated as `false`, matching
    /// [`User::is_system`].
    ///
    /// [`User::is_system`]: crate::user::User::is_system
    #[inline]
    #[must_use]
    pub fn is_system(&self) -> bool {
        self.system.unwrap_or(false)
    }
}
