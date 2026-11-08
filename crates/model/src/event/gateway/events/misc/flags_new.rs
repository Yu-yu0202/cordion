/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Serde support for the string-serialized `flags_new` application field.

use serde::{Deserialize, Deserializer, Serializer};

use crate::application::ApplicationFlags;

/// Serializes the flags as a decimal string or `null`.
pub(crate) fn serialize<S>(
    flags: &Option<ApplicationFlags>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match flags {
        Some(flags) => serializer.collect_str(&flags.bits()),
        None => serializer.serialize_none(),
    }
}

/// Deserializes the flags from a decimal string, an integer, or `null`.
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<ApplicationFlags>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<ApplicationFlags>::deserialize(deserializer)
}
