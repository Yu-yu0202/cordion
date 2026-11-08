/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Per-guild command permission overwrites.
//!
//! [`GuildApplicationCommandPermissions`] is the `permissions` object Discord
//! returns for a guild's commands; it carries the command ID or, for the
//! default permissions, the application ID. Each
//! [`ApplicationCommandPermissions`] entry is one role, user, or channel
//! overwrite, whose target is identified by
//! [`ApplicationCommandPermissionType`].
//!
//! See [Discord's application command permissions object][object] for details.
//!
//! [object]: https://docs.discord.com/developers/interactions/application-commands#application-command-permissions-object

use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::guild::GuildId;
use crate::id::snowflake;

/// The permissions for an application's command in a guild.
///
/// When [`id`](Self::id) is the application ID rather than a command ID, the
/// permissions apply to all commands that do not have explicit overwrites.
///
/// See [Discord's guild application command permissions object][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/interactions/application-commands#application-command-permissions-object-guild-application-command-permissions-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildApplicationCommandPermissions {
    /// The ID of the command, or the application ID for default permissions.
    ///
    /// Discord sends this as a snowflake-formatted decimal string, but the
    /// value can refer to either resource type, so it is exposed as a raw
    /// `u64`.
    #[serde(with = "snowflake")]
    pub id: u64,

    /// The ID of the application the command belongs to.
    pub application_id: ApplicationId,

    /// The ID of the guild.
    pub guild_id: GuildId,

    /// The permission overwrites for the command, up to 100.
    pub permissions: Vec<ApplicationCommandPermissions>,
}

/// One permission overwrite for an application command.
///
/// See [Discord's application command permissions object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/application-commands#application-command-permissions-object-application-command-permissions-structure
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandPermissions {
    /// The ID of the role, user, or channel, or a permission constant.
    ///
    /// Permission constants are the guild ID (`@everyone`) and the guild ID
    /// minus one (all channels), so this value is not tied to a single resource
    /// type. Discord sends it as a snowflake-formatted decimal string.
    #[serde(with = "snowflake")]
    pub id: u64,

    /// The permission target type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandPermissionType,

    /// Whether the permission is allowed (`true`) or denied (`false`).
    pub permission: bool,
}

/// The target type of an [`ApplicationCommandPermissions`] overwrite.
///
/// Unrecognized values are preserved. See [Discord's permission type table][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/interactions/application-commands#application-command-permissions-object-application-command-permission-type
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApplicationCommandPermissionType(u8);

impl ApplicationCommandPermissionType {
    /// The overwrite targets a role.
    pub const ROLE: Self = Self(1);

    /// The overwrite targets a user.
    pub const USER: Self = Self(2);

    /// The overwrite targets a channel.
    pub const CHANNEL: Self = Self(3);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}
