/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Helpers shared by the wire-format test files.

#![allow(clippy::unwrap_used)]
#![allow(dead_code)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use cordion_model::channel::Channel;
use cordion_model::id::Id;
use cordion_model::role::Role;

pub(crate) fn timestamp(seconds: u64, nanos: u32) -> SystemTime {
    UNIX_EPOCH + Duration::new(seconds, nanos)
}

/// The `@everyone` role ID shared with the guild ID in these fixtures.
pub(crate) const EVERYONE_ROLE: Id<Role> = Id::new(41_771_983_429_993_937);

/// Returns the Unix timestamp identified by `seconds` and `nanos`.
pub(crate) fn request_timestamp(seconds: u64, nanos: u32) -> SystemTime {
    UNIX_EPOCH + Duration::new(seconds, nanos)
}

/// Returns a channel ID for use in request fixtures.
pub(crate) const fn request_channel_id(value: u64) -> Id<Channel> {
    Id::new(value)
}
