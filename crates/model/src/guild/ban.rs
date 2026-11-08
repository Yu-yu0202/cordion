/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::user::{User, UserId};

/// A ban of a guild member.
///
/// Returned by `GET /guilds/{guild.id}/bans` as an array and by
/// `GET /guilds/{guild.id}/bans/{user.id}` as a single object.
///
/// See [Discord's ban object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#ban-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Ban {
    /// The reason recorded for the ban.
    ///
    /// Discord documents this field as nullable and sends `null` when no
    /// reason was recorded, so `None` is serialized as `null`.
    #[serde(default)]
    pub reason: Option<String>,

    /// The banned user.
    pub user: User,
}

/// The body of `POST /guilds/{guild.id}/bans` and
/// `PUT /guilds/{guild.id}/bans/{user.id}`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// See [Discord's create guild ban endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#create-guild-ban
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateGuildBanRequest {
    /// The number of seconds of the user's messages to delete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<u64>,
}

/// The body of `POST /guilds/{guild.id}/bulk-ban`.
///
/// See [Discord's bulk guild ban endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#bulk-guild-ban
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BulkGuildBanRequest {
    /// The users to ban, at most 200.
    pub user_ids: Vec<UserId>,

    /// The number of seconds of each user's messages to delete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<u64>,
}

/// The response of `POST /guilds/{guild.id}/bulk-ban`.
///
/// See [Discord's bulk ban response][response] for details.
///
/// [response]: https://docs.discord.com/developers/resources/guild#bulk-ban-response
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BulkGuildBanResponse {
    /// The users that were successfully banned.
    pub banned_users: Vec<UserId>,

    /// The users that could not be banned or were already banned.
    pub failed_users: Vec<UserId>,
}
