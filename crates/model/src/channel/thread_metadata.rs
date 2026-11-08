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

/// Thread-specific metadata stored in a channel's `thread_metadata` field.
///
/// See [Discord's thread metadata object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#thread-metadata-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ThreadMetadata {
    /// Whether the thread is archived.
    pub archived: bool,

    /// Minutes of inactivity before the thread stops appearing in the channel
    /// list.
    ///
    /// Discord accepts `60`, `1440`, `4320`, and `10080`.
    pub auto_archive_duration: u16,

    /// When the thread's archive status last changed.
    ///
    /// Discord uses this timestamp to calculate recent activity.
    #[serde(with = "crate::timestamp")]
    pub archive_timestamp: SystemTime,

    /// Whether the thread is locked.
    ///
    /// Only users with [`Permission::MANAGE_THREADS`] can unarchive a locked
    /// thread.
    ///
    /// [`Permission::MANAGE_THREADS`]: crate::permission::Permission::MANAGE_THREADS
    pub locked: bool,

    /// Whether non-moderators can add other non-moderators to the thread.
    ///
    /// This field is only returned for private threads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,

    /// When the thread was created.
    ///
    /// Discord only populates this for threads created after 2022-01-09; it is
    /// `None` for older threads.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub create_timestamp: Option<SystemTime>,
}
