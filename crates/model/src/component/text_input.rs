/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Text input components.

use serde::{Deserialize, Serialize};

use crate::component::ComponentType;

/// The style of a [`TextInput`].
///
/// Unrecognized values are preserved. See [Discord's text input styles][styles]
/// for details.
///
/// [styles]: https://docs.discord.com/developers/components/reference#text-input-text-input-styles
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TextInputStyle(u8);

impl TextInputStyle {
    /// A single-line input.
    pub const SHORT: Self = Self(1);

    /// A multi-line input.
    pub const PARAGRAPH: Self = Self(2);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A free-form text input used in modals.
///
/// Text inputs should be placed inside a [`Label`](super::Label) in new
/// modals; placing them in an action row is deprecated by Discord.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's text input component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#text-input
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TextInput {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::text_input"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// The input style.
    pub style: TextInputStyle,

    /// The minimum input length, from 0 to 4000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,

    /// The maximum input length, from 1 to 4000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,

    /// Whether the input is required.
    ///
    /// Discord defaults this to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// A pre-filled value, up to 4000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// Placeholder text shown when the input is empty, up to 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The input's label, up to 45 characters.
    ///
    /// Deprecated in favor of the label and description on a
    /// [`Label`](super::Label) component. Discord still returns this field for
    /// older modals, so it is preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Deprecated by Discord API. Use the surrounding `Label` instead.")]
    pub label: Option<String>,
}
