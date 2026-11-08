/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Interactive components used in modals.

use serde::{Deserialize, Serialize};

use crate::component::ComponentType;

/// A file upload control used in modals.
///
/// File uploads must be placed inside a [`Label`](super::Label). The maximum
/// upload size is determined by the user's upload limit in the channel.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization. `min_values` must be omitted or at least `1` when
/// `required` is omitted or `true`.
///
/// See [Discord's file upload component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#file-upload
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct FileUpload {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::file_upload"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// The minimum number of files, from 0 to 10.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    /// The maximum number of files, up to 10.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    /// Whether at least one file must be uploaded before submitting the modal.
    ///
    /// Discord defaults this to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// File type filters, up to 10 entries.
    ///
    /// Each entry is `image`, `video`, `audio`, or a dot-prefixed extension
    /// such as `.pdf`. Discord matches only the file extension.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_types: Option<Vec<String>>,
}

/// One option in a [`RadioGroup`].
///
/// See [Discord's radio group option object][object] for details.
///
/// [object]: https://docs.discord.com/developers/components/reference#radio-group-radio-group-option-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RadioGroupOption {
    /// The developer-defined value, up to 100 characters.
    pub value: String,

    /// The user-facing label, up to 100 characters.
    pub label: String,

    /// Additional text describing the option, up to 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the option is selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// A control for selecting exactly one option from a list.
///
/// Radio groups are only available in modals and must be placed inside a
/// [`Label`](super::Label). Discord requires between two and ten options.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's radio group component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#radio-group
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RadioGroup {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::radio_group"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// The options shown in the group, from two to ten.
    pub options: Vec<RadioGroupOption>,

    /// Whether a selection is required before submitting the modal.
    ///
    /// Discord defaults this to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// One option in a [`CheckboxGroup`].
///
/// See [Discord's checkbox group option object][object] for details.
///
/// [object]: https://docs.discord.com/developers/components/reference#checkbox-group-checkbox-group-option-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CheckboxGroupOption {
    /// The developer-defined value, up to 100 characters.
    pub value: String,

    /// The user-facing label, up to 100 characters.
    pub label: String,

    /// Additional text describing the option, up to 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the option is selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// A control for selecting one or more options with checkboxes.
///
/// Checkbox groups are only available in modals and must be placed inside a
/// [`Label`](super::Label). Discord requires between one and ten options.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization. `min_values` must be omitted or at least `1` when
/// `required` is omitted or `true`.
///
/// See [Discord's checkbox group component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#checkbox-group
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CheckboxGroup {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::checkbox_group"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// The options shown in the group, from one to ten.
    pub options: Vec<CheckboxGroupOption>,

    /// The minimum number of selections, from 0 to 10.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    /// The maximum number of selections, from 1 to 10.
    ///
    /// Discord defaults this to the number of options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    /// Whether a selection is required before submitting the modal.
    ///
    /// Discord defaults this to `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// A single yes-or-no checkbox.
///
/// Checkboxes are only available in modals and must be placed inside a
/// [`Label`](super::Label). Discord cannot mark a standalone checkbox as
/// required; use a checkbox group with one option for that behavior.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's checkbox component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#checkbox
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Checkbox {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::checkbox")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// Whether the checkbox is selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}
