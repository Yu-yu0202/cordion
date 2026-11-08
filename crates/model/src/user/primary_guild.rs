/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::guild::Guild;
use crate::id::Id;

/// The user's primary guild and server tag settings.
///
/// See [Discord's user primary guild object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/user#user-object-user-primary-guild
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UserPrimaryGuild {
    /// The ID of the user's primary guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_guild_id: Option<Id<Guild>>,

    /// Whether the user is displaying the primary guild's server tag.
    ///
    /// `None` means Discord cleared the identity, for example because the guild
    /// no longer supports tags. `Some(false)` means the user removed the tag
    /// manually.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_enabled: Option<bool>,

    /// The primary guild's server tag text, limited to four characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,

    /// The primary guild's server tag badge hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<String>,
}
