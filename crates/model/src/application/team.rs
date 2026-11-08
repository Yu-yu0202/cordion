/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Developer team models.
//!
//! Teams group developers who share access to applications. See [Discord's
//! teams documentation][teams] for details.
//!
//! [teams]: https://docs.discord.com/developers/topics/teams

use std::borrow::Cow;

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::id::{Id, Identifiable};
use crate::user::{User, UserId};

/// The ID of a [`Team`].
pub type TeamId = Id<Team>;

/// A developer team that owns one or more applications.
///
/// See [Discord's team object][object] for details.
///
/// [object]: https://docs.discord.com/developers/topics/teams#data-models-team-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Team {
    /// The hash of the team's icon, or `None` if it has no icon.
    ///
    /// Discord sends this field explicitly as `null` when the team has no icon,
    /// so `None` is serialized as `null`.
    pub icon: Option<String>,

    /// The team's ID.
    pub id: TeamId,

    /// The members of the team.
    pub members: Vec<TeamMember>,

    /// The team's name.
    pub name: String,

    /// The ID of the current team owner.
    pub owner_user_id: UserId,
}

impl Identifiable for Team {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// A member of a developer [`Team`].
///
/// Fields that are not modeled here, such as the legacy `permissions` array
/// present in some application payloads, are ignored during deserialization.
///
/// See [Discord's team member object][object] for details.
///
/// [object]: https://docs.discord.com/developers/topics/teams#data-models-team-member-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TeamMember {
    /// The user's membership state on the team.
    pub membership_state: TeamMembershipState,

    /// The ID of the team the member belongs to.
    pub team_id: TeamId,

    /// The member's user object.
    pub user: User,

    /// The member's role on the team, or `None` if Discord omitted it.
    ///
    /// Discord's team member table lists this field without an optional marker,
    /// but the official application object example omits it, so it is modeled
    /// as optional. The owner role is not represented here; use
    /// [`Team::owner_user_id`] to identify the owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<TeamMemberRole>,
}

/// A user's membership state on a developer [`Team`].
///
/// Unrecognized values are preserved. See [Discord's membership state
/// enum][states] for details.
///
/// [states]: https://docs.discord.com/developers/topics/teams#data-models-membership-state-enum
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TeamMembershipState(u8);

impl TeamMembershipState {
    /// The user has been invited to the team but has not accepted.
    pub const INVITED: Self = Self(1);

    /// The user has accepted the invitation and is a member of the team.
    pub const ACCEPTED: Self = Self(2);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A member's role on a developer [`Team`].
///
/// Documented roles are exposed as associated constants, and unrecognized
/// values are preserved when decoding. The owner role is not represented by
/// this type; use [`Team::owner_user_id`] to identify the owner.
///
/// See [Discord's team member role types][roles] for details.
///
/// [roles]: https://docs.discord.com/developers/topics/teams#team-member-roles-team-member-role-types
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TeamMemberRole(Cow<'static, str>);

impl TeamMemberRole {
    /// A team administrator.
    pub const ADMIN: Self = Self(Cow::Borrowed("admin"));

    /// A developer with access to team-owned applications.
    pub const DEVELOPER: Self = Self(Cow::Borrowed("developer"));

    /// A read-only team member.
    pub const READ_ONLY: Self = Self(Cow::Borrowed("read_only"));

    /// Creates an owned role from any string-like value.
    ///
    /// Prefer [`TeamMemberRole::from`] with a `&'static str` to borrow instead
    /// of allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the role string exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for TeamMemberRole {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
