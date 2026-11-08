/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::id::Id;
use crate::permission::Permission;

/// A role or user whose permissions are overwritten in a channel.
///
/// Discord reuses the `id` field for both targets and distinguishes them with
/// [`OverwriteType`]. This marker type keeps overwrite IDs distinct from other
/// snowflakes; use [`Id::cast`] to interpret one as a [`RoleId`] or
/// [`UserId`] after checking [`Overwrite::kind`].
///
/// [`RoleId`]: crate::role::RoleId
/// [`UserId`]: crate::user::UserId
#[derive(Clone, Copy, PartialEq, Eq, IdTarget, Debug)]
pub struct OverwriteTarget;

/// The ID of the role or user targeted by an [`Overwrite`].
pub type OverwriteId = Id<OverwriteTarget>;

/// Whether an [`Overwrite`] applies to a role or a guild member.
///
/// Unrecognized values are preserved. See [Discord's overwrite object][object]
/// for details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#overwrite-object
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct OverwriteType(u8);

impl OverwriteType {
    /// The overwrite applies to a role.
    pub const ROLE: Self = Self(0);

    /// The overwrite applies to a guild member.
    pub const MEMBER: Self = Self(1);
}

/// An explicit channel permission overwrite for a role or member.
///
/// Discord applies `@everyone`, role, and member overwrites in a documented
/// order; this type only represents the stored bits. See [Discord's permission
/// overwrites][overwrites] for the calculation rules.
///
/// [overwrites]: https://docs.discord.com/developers/topics/permissions#permission-overwrites
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Overwrite {
    /// The role or user ID this overwrite applies to.
    pub id: OverwriteId,

    /// Whether [`id`](Self::id) is a role or a member.
    #[serde(rename = "type")]
    pub kind: OverwriteType,

    /// Permission bits explicitly allowed.
    pub allow: Permission,

    /// Permission bits explicitly denied.
    pub deny: Permission,
}
