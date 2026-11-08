/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Layout components that arrange other components.

use serde::{Deserialize, Serialize};

use crate::component::{Component, ComponentType};

/// A row of up to five buttons or one select menu.
///
/// Action rows are used in messages. Discord now recommends
/// [`Label`] over an action row for modal components; text inputs inside action
/// rows are deprecated.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's action row component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#action-row
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ActionRow {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::action_row"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The buttons or select menu contained in the row.
    pub components: Vec<Component>,
}

/// A section that associates text content with an accessory.
///
/// Sections are only available in messages that set the
/// `IS_COMPONENTS_V2` flag. Discord documents text display children and button
/// or thumbnail accessories, but may add more component kinds, so the child and
/// accessory fields use the general [`Component`] type.
///
/// See [Discord's section component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#section
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Section {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::section")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The section's content components, currently one to three text displays.
    pub components: Vec<Component>,

    /// A component contextually associated with the content, such as a
    /// thumbnail or button.
    pub accessory: Box<Component>,
}

/// A visually grouped collection of components.
///
/// Containers are only available in messages that set the `IS_COMPONENTS_V2`
/// flag. Discord documents action row, text display, section, media gallery,
/// separator, and file children, but may add more component kinds, so this
/// field uses the general [`Component`] type.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's container component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#container
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Container {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::container")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The components encapsulated by the container.
    pub components: Vec<Component>,

    /// The container's accent color as an RGB integer from `0x000000` to
    /// `0xFFFFFF`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    /// Whether the container is hidden as a spoiler.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spoiler: Option<bool>,
}

/// A label that wraps a modal component with text and an optional description.
///
/// The child component is stored as a [`Component`] rather than as a narrower
/// enum so that new label-compatible components do not break decoding.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's label component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#label
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Label {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::label")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The label text, up to 45 characters.
    pub label: String,

    /// An optional description, up to 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The component wrapped by the label.
    pub component: Box<Component>,
}

/// The vertical padding of a [`Separator`].
///
/// Unrecognized values are preserved. See [Discord's separator
/// component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#separator
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SeparatorSpacing(u8);

impl SeparatorSpacing {
    /// Small padding.
    pub const SMALL: Self = Self(1);

    /// Large padding.
    pub const LARGE: Self = Self(2);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Vertical padding and an optional visual divider.
///
/// Separators are only available in messages that set the `IS_COMPONENTS_V2`
/// flag. Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's separator component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#separator
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Separator {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::separator")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// Whether to draw a visual divider.
    ///
    /// Discord defaults this to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divider: Option<bool>,

    /// The separator's padding size.
    ///
    /// Discord defaults this to [`SeparatorSpacing::SMALL`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacing: Option<SeparatorSpacing>,
}
