/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Additional payload types carried by messages.
//!
//! [`MessageActivity`] carries Rich Presence data for activity-related chat
//! embeds, [`MessageCall`] the call associated with a message,
//! [`RoleSubscriptionData`] purchase data for role subscription messages, and
//! [`SharedClientTheme`] a custom client-side theme, whose base mode is
//! [`BaseTheme`].
//!
//! [`ChannelMention`] models a channel mentioned in a message.
//! [`AllowedMentions`] and [`AllowedMentionType`] model the mention
//! restrictions sent in message create and edit request bodies.
//!
//! See [Discord's message resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/message

use std::borrow::Cow;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::channel::{ChannelId, ChannelType};
use crate::guild::GuildId;
use crate::monetization::sku::SkuId;
use crate::role::RoleId;
use crate::user::UserId;

/// A message's Rich Presence activity.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's message activity object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-object-message-activity-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageActivity {
    /// The activity type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: MessageActivityType,

    /// The party ID from a Rich Presence event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
}

/// A Rich Presence message activity type.
///
/// Unrecognized values are preserved. See [Discord's message activity
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/resources/message#message-object-message-activity-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct MessageActivityType(u8);

impl MessageActivityType {
    /// A join activity.
    pub const JOIN: Self = Self(1);

    /// A spectate activity.
    pub const SPECTATE: Self = Self(2);

    /// A listen activity.
    pub const LISTEN: Self = Self(3);

    /// A join request.
    pub const JOIN_REQUEST: Self = Self(5);

    /// A stream request.
    pub const STREAM_REQUEST: Self = Self(6);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A call associated with a message.
///
/// See [Discord's message call object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-call-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MessageCall {
    /// IDs of the users who participated in the call.
    pub participants: Vec<UserId>,

    /// When the call ended.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ended_timestamp: Option<SystemTime>,
}

/// Role subscription data for a role subscription message.
///
/// See [Discord's role subscription data object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#role-subscription-data-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RoleSubscriptionData {
    /// The ID of the SKU and listing the user is subscribed to.
    pub role_subscription_listing_id: SkuId,

    /// The name of the subscribed tier.
    pub tier_name: String,

    /// The cumulative number of months subscribed.
    pub total_months_subscribed: u32,

    /// Whether this is a renewal rather than a new purchase.
    pub is_renewal: bool,
}

/// A custom client-side theme shared via a message.
///
/// See [Discord's shared client theme object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#shared-client-theme-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SharedClientTheme {
    /// The hexadecimal-encoded theme colors, up to five.
    pub colors: Vec<String>,

    /// The direction of the colors, from 0 to 360.
    pub gradient_angle: u16,

    /// The intensity of the colors, from 0 to 100.
    pub base_mix: u8,

    /// The theme's base mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_theme: Option<BaseTheme>,
}

/// The base mode of a [`SharedClientTheme`].
///
/// Unrecognized values are preserved. See [Discord's base theme types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/resources/message#base-theme-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct BaseTheme(u8);

impl BaseTheme {
    /// The unset base theme, equivalent to [`DARK`](Self::DARK).
    pub const UNSET: Self = Self(0);

    /// The dark base theme.
    pub const DARK: Self = Self(1);

    /// The light base theme.
    pub const LIGHT: Self = Self(2);

    /// The darker base theme.
    pub const DARKER: Self = Self(3);

    /// The midnight base theme.
    pub const MIDNIGHT: Self = Self(4);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A channel mentioned in a message.
///
/// See [Discord's channel mention object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#channel-mention-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ChannelMention {
    /// The channel's ID.
    pub id: ChannelId,

    /// The ID of the guild containing the channel.
    pub guild_id: GuildId,

    /// The channel's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ChannelType,

    /// The channel's name.
    pub name: String,
}

/// Mention restrictions for a message.
///
/// [`parse`](Self::parse) is mutually exclusive with the explicit ID lists on
/// the wire, though this type preserves whatever combination is received.
/// Absent optional fields are omitted during serialization.
///
/// See [Discord's allowed mentions object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#allowed-mentions-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct AllowedMentions {
    /// The mention types to parse from message content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parse: Option<Vec<AllowedMentionType>>,

    /// IDs of roles that may be mentioned, up to 100.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleId>>,

    /// IDs of users that may be mentioned, up to 100.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserId>>,

    /// Whether a reply should mention the author of the replied-to message.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replied_user: Option<bool>,
}

/// A mention type understood by [`AllowedMentions::parse`].
///
/// Discord sends `"roles"`, `"users"`, or `"everyone"`. Unrecognized values
/// are preserved when decoding.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AllowedMentionType(Cow<'static, str>);

impl AllowedMentionType {
    /// Role mentions.
    pub const ROLES: Self = Self(Cow::Borrowed("roles"));

    /// User mentions.
    pub const USERS: Self = Self(Cow::Borrowed("users"));

    /// `@everyone` and `@here` mentions.
    pub const EVERYONE: Self = Self(Cow::Borrowed("everyone"));

    /// Creates an owned mention type from any string-like value.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the mention type exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
