/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Request payloads for the guild role endpoints.
//!
//! See [Discord's guild resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/guild

use serde::{Deserialize, Serialize};

use crate::permission::Permission;
use crate::role::{RoleColors, RoleId};

/// The body of `POST /guilds/{guild.id}/roles`.
///
/// Discord documents every parameter as optional, so every field is omitted
/// during serialization when `None`. An omitted field takes Discord's
/// documented default: `"new role"` for [`name`](Self::name), the guild's
/// `@everyone` permissions for [`permissions`](Self::permissions), and `false`
/// for [`hoist`](Self::hoist) and [`mentionable`](Self::mentionable).
///
/// [`icon`](Self::icon) carries image data as a [Data URI scheme] string, for
/// example `data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA`.
///
/// See [Discord's create guild role endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#create-guild-role
/// [Data URI scheme]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGuildRoleRequest {
    /// The role's name, at most 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The role's permission bit set.
    ///
    /// Serialized as a decimal string; unknown bits are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,

    /// The role's legacy color value.
    ///
    /// Discord deprecated this field in favor of [`colors`](Self::colors) and
    /// still returns it, but requests should use `colors`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,

    /// The role's colors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colors: Option<RoleColors>,

    /// Whether the role is displayed separately in the member list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,

    /// The role's icon as image data, when the guild has the `ROLE_ICONS`
    /// feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The role's Unicode emoji as a standard emoji, when the guild has the
    /// `ROLE_ICONS` feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,

    /// Whether the role can be mentioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}

/// One entry of the `PATCH /guilds/{guild.id}/roles` request body.
///
/// Only the roles whose position changes need to be sent. See [Discord's modify
/// guild role positions endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-role-positions
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildRolePosition {
    /// The role to move.
    pub id: RoleId,

    /// The role's sorting position.
    ///
    /// Roles with the same position are sorted by ID. Discord documents this
    /// field as optional, so an omitted value keeps the current position.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

/// The body of `PATCH /guilds/{guild.id}/roles/{role.id}`.
///
/// Discord documents every parameter as optional and nullable, so every field
/// is omitted during serialization when `None` and an omitted field leaves the
/// role unchanged. Because omission means "leave unchanged", the fields Discord
/// documents as nullable cannot be cleared by sending `null` through this type.
///
/// [`icon`](Self::icon) carries image data as a [Data URI scheme] string, for
/// example `data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA`.
///
/// See [Discord's modify guild role endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-role
/// [Data URI scheme]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildRoleRequest {
    /// The role's name, at most 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The role's permission bit set.
    ///
    /// Serialized as a decimal string; unknown bits are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,

    /// The role's legacy color value.
    ///
    /// Discord deprecated this field in favor of [`colors`](Self::colors) and
    /// still returns it, but requests should use `colors`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,

    /// The role's colors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colors: Option<RoleColors>,

    /// Whether the role is displayed separately in the member list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,

    /// The role's icon as image data, when the guild has the `ROLE_ICONS`
    /// feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The role's Unicode emoji as a standard emoji, when the guild has the
    /// `ROLE_ICONS` feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,

    /// Whether the role can be mentioned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}
