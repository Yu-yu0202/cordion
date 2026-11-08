/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// Incident data for a guild, describing temporary restrictions and detected
/// raids or spam.
///
/// `invites_disabled_until` and `dms_disabled_until` are required fields that may
/// be `null`; the detection timestamps are optional and omitted when unknown.
///
/// See [Discord's incidents data object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#incidents-data-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct IncidentsData {
    /// When invites become enabled again, or `None` if they are not restricted.
    #[serde(with = "crate::timestamp::option")]
    pub invites_disabled_until: Option<SystemTime>,

    /// When direct messages become enabled again, or `None` if they are not
    /// restricted.
    #[serde(with = "crate::timestamp::option")]
    pub dms_disabled_until: Option<SystemTime>,

    /// When Discord detected DM spam, if any.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dm_spam_detected_at: Option<SystemTime>,

    /// When Discord detected a raid, if any.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub raid_detected_at: Option<SystemTime>,
}

/// The body of `PUT /guilds/{guild.id}/incident-actions`.
///
/// Discord documents both fields as optional and nullable, and states that
/// supplying `null` disables the action. Because a `None` field is therefore a
/// meaningful request rather than an omitted one, each field always
/// serializes, emitting `null` when `None`.
///
/// Both actions may be enabled for at most 24 hours into the future.
///
/// See [Discord's modify guild incident actions endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-incident-actions
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildIncidentActionsRequest {
    /// When invites become enabled again.
    ///
    /// `None` disables the invite restriction, and serializes as `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub invites_disabled_until: Option<SystemTime>,

    /// When direct messages become enabled again.
    ///
    /// `None` disables the direct message restriction, and serializes as
    /// `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub dms_disabled_until: Option<SystemTime>,
}
