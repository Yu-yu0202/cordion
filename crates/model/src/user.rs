/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord user resource.
//!
//! [`User`] models the user object. Discord embeds it in other resources, which
//! can add context-specific fields: the `mentions` array of Gateway message
//! events carries a partial guild member object in [`User::member`].
//!
//! See [Discord's user resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/user

mod avatar_decoration;
mod collectibles;
mod connection;
mod flags;
mod nameplate;
mod partial;
mod premium_type;
mod primary_guild;
mod role_connection;

use std::collections::BTreeMap;

pub use avatar_decoration::AvatarDecorationData;
pub use collectibles::Collectibles;
pub use connection::{Connection, ConnectionService, ConnectionVisibility};
use cordion_macro::IdTarget;
pub use flags::UserFlags;
pub use nameplate::{Nameplate, NameplatePalette};
pub use partial::PartialUser;
pub use premium_type::PremiumType;
pub use primary_guild::UserPrimaryGuild;
pub use role_connection::ApplicationRoleConnection;
use serde::{Deserialize, Serialize};

use crate::id::{Id, Identifiable};

/// The ID of a [`User`].
pub type UserId = Id<User>;

/// A Discord user.
///
/// Optional fields decode as `None` when absent or `null`; this type does not
/// distinguish the two. Some fields are only returned for the current user or
/// when the relevant `OAuth2` scope is granted.
///
/// See [Discord's user object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/user#user-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct User {
    /// The user's ID.
    pub id: Id<User>,

    /// The user's username, which is not unique across Discord.
    ///
    /// Use [`id`](Self::id) to identify an account reliably.
    pub username: String,

    /// The user's legacy Discord tag, or `"0"` for migrated accounts.
    pub discriminator: String,

    /// The user's display name, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_name: Option<String>,

    /// The user's avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// Whether the user belongs to an `OAuth2` application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,

    /// Whether this is an Official Discord System user used for urgent system
    /// messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,

    /// Whether the user has two-factor authentication enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,

    /// The user's banner hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The user's banner color as a hexadecimal color integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    /// The user's chosen language option, such as `"en-US"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// Whether the email address on this account has been verified.
    ///
    /// Requires the `email` `OAuth2` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,

    /// The user's email address.
    ///
    /// Requires the `email` `OAuth2` scope and may be absent when the account has
    /// no email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// All flags on the user's account.
    ///
    /// Requires the `identify` `OAuth2` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<UserFlags>,

    /// The user's Nitro subscription type.
    ///
    /// Requires the `identify.premium` `OAuth2` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_type: Option<PremiumType>,

    /// The public flags on the user's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,

    /// The user's equipped avatar decoration, when provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<AvatarDecorationData>,

    /// The user's collectibles, excluding avatar decorations and profile
    /// effects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Collectibles>,

    /// The user's primary guild and server tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_guild: Option<UserPrimaryGuild>,

    /// The user's guild member data, when the user object is a mention in a
    /// Gateway message event.
    ///
    /// Discord documents the `mentions` array of `MESSAGE_CREATE` and
    /// `MESSAGE_UPDATE` as user objects optionally carrying an additional
    /// partial guild member field. The field is absent everywhere else, so this
    /// type keeps it separate from the message's own
    /// [`member`](crate::message::Message::member) field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<crate::guild::PartialGuildMember>,
}

impl Identifiable for User {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

impl User {
    /// Returns whether this is a bot account.
    ///
    /// Discord omits the field for normal users, so an absent value is treated
    /// as `false`.
    #[inline]
    #[must_use]
    pub fn is_bot(&self) -> bool {
        self.bot.unwrap_or(false)
    }

    /// Returns whether this is an Official Discord System user.
    ///
    /// Discord omits the field for normal users, so an absent value is treated
    /// as `false`.
    #[inline]
    #[must_use]
    pub fn is_system(&self) -> bool {
        self.system.unwrap_or(false)
    }
}

/// The body of `POST /users/@me/channels` when opening a direct message.
///
/// See [Discord's create DM endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/user#create-dm
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateDmRequest {
    /// The user to open a direct message with.
    pub recipient_id: UserId,
}

/// The body of `POST /users/@me/channels` when opening a group DM.
///
/// See [Discord's create group DM endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/user#create-group-dm
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGroupDmRequest {
    /// The access tokens of users that granted the `gdm.join` scope.
    pub access_tokens: Vec<String>,

    /// The nicknames to assign, keyed by user ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nicks: Option<BTreeMap<UserId, String>>,
}
