/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Tag validation for untagged component enums.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer};

use crate::component::ComponentType;

macro_rules! expect_component_type {
    ($name:ident, $value:expr) => {
        pub(super) fn $name<'de, D>(deserializer: D) -> Result<ComponentType, D::Error>
        where
            D: Deserializer<'de>,
        {
            let kind = ComponentType::deserialize(deserializer)?;

            if kind == ComponentType::from_raw($value) {
                Ok(kind)
            } else {
                Err(D::Error::custom(format_args!(
                    "expected component type {}, found {}",
                    $value,
                    kind.raw()
                )))
            }
        }
    };
}

expect_component_type!(action_row, 1);
expect_component_type!(button, 2);
expect_component_type!(string_select, 3);
expect_component_type!(text_input, 4);
expect_component_type!(user_select, 5);
expect_component_type!(role_select, 6);
expect_component_type!(mentionable_select, 7);
expect_component_type!(channel_select, 8);
expect_component_type!(section, 9);
expect_component_type!(text_display, 10);
expect_component_type!(thumbnail, 11);
expect_component_type!(media_gallery, 12);
expect_component_type!(file, 13);
expect_component_type!(separator, 14);
expect_component_type!(container, 17);
expect_component_type!(label, 18);
expect_component_type!(file_upload, 19);
expect_component_type!(radio_group, 21);
expect_component_type!(checkbox_group, 22);
expect_component_type!(checkbox, 23);
