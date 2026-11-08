/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord role resource.
//!
//! [`Role`] models the full role object. [`PartialRole`] models the reduced role
//! shape assigned by an invite.
//!
//! See [Discord's role object][role] for details.
//!
//! [role]: https://docs.discord.com/developers/topics/permissions#role-object

mod color;
mod flags;
mod partial;
mod tags;

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::id::{Id, Identifiable};
use crate::permission::Permission;
pub use crate::role::color::RoleColors;
pub use crate::role::flags::RoleFlags;
pub use crate::role::partial::PartialRole;
pub use crate::role::tags::RoleTags;

/// The ID of a [`Role`].
pub type RoleId = Id<Role>;

/// A role in a guild.
///
/// The `@everyone` role has the same ID as its guild. See [Discord's role
/// object][object] for details.
///
/// [object]: https://docs.discord.com/developers/topics/permissions#role-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Role {
    /// The role's ID.
    pub id: Id<Role>,

    /// The role's name, up to 100 characters.
    pub name: String,

    /// The role's legacy color value.
    ///
    /// This is deprecated in favor of [`colors`](Self::colors), but Discord still
    /// returns it.
    #[deprecated(note = "Deprecated by Discord API. Use `colors` instead.")]
    pub color: u32,

    /// The role's colors.
    pub colors: RoleColors,

    /// Whether the role is displayed separately in the member list.
    pub hoist: bool,

    /// The role's icon hash.
    ///
    /// Requires the guild's `ROLE_ICONS` feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The role's Unicode emoji.
    ///
    /// Requires the guild's `ROLE_ICONS` feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,

    /// The role's position.
    ///
    /// Roles with the same position are sorted by ID.
    pub position: i32,

    /// The role's permission bit set.
    pub permissions: Permission,

    /// Whether the role is managed by an integration.
    pub managed: bool,

    /// Whether the role can be mentioned.
    pub mentionable: bool,

    /// The role's tags.
    ///
    /// Only present for roles managed by an integration or otherwise special
    /// roles such as the guild's Booster role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTags>,

    /// The role's flags.
    pub flags: RoleFlags,
}

impl Identifiable for Role {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}
