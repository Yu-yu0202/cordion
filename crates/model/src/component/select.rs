/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Select menu components.

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::channel::{ChannelId, ChannelType};
use crate::component::ComponentType;
use crate::emoji::PartialEmoji;
use crate::id::Id;
use crate::role::RoleId;
use crate::user::UserId;

/// A marker for a mentionable selected in a mentionable select menu.
///
/// A mentionable value is either a user or a role; the type is determined by
/// the surrounding resolved data. Using a distinct ID target prevents it from
/// being passed to APIs that require a user or role ID without an explicit
/// cast.
#[derive(Clone, Copy, PartialEq, Eq, IdTarget, Debug)]
pub struct Mentionable;

/// The ID of a [`Mentionable`].
pub type MentionableId = Id<Mentionable>;

/// A select menu of developer-defined string options.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization. `min_values` must be omitted or at least `1` when
/// `required` is omitted or `true`. `required` only applies in modals, and
/// `disabled` is rejected in modals.
///
/// See [Discord's string select component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#string-select
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct StringSelect {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::string_select"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// The options shown in the menu, up to 25.
    pub options: Vec<SelectOption>,

    /// Placeholder text shown when nothing is selected, up to 150 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The minimum number of selections, from 0 to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    /// The maximum number of selections, up to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    /// Whether a selection is required in a modal.
    ///
    /// Discord defaults this to `true`; the field is ignored in messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Whether the menu is disabled in a message.
    ///
    /// Discord defaults this to `false`; using it in a modal is an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// One option in a [`StringSelect`].
///
/// See [Discord's select option object][object] for details.
///
/// [object]: https://docs.discord.com/developers/components/reference#string-select-select-option-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct SelectOption {
    /// The option's user-facing name, up to 100 characters.
    pub label: String,

    /// The developer-defined value, up to 100 characters.
    pub value: String,

    /// Additional text describing the option, up to 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The option's emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    /// Whether this option is selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
}

/// A select menu populated with guild users.
///
/// See [Discord's user select component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#user-select
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct UserSelect {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::user_select"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// Placeholder text shown when nothing is selected, up to 150 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The options selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<SelectDefaultValue>>,

    /// The minimum number of selections, from 0 to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    /// The maximum number of selections, up to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    /// Whether a selection is required in a modal.
    ///
    /// Discord defaults this to `true`; the field is ignored in messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Whether the menu is disabled in a message.
    ///
    /// Discord defaults this to `false`; using it in a modal is an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// A select menu populated with guild roles.
///
/// See [Discord's role select component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#role-select
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RoleSelect {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::role_select"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// Placeholder text shown when nothing is selected, up to 150 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The options selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<SelectDefaultValue>>,

    /// The minimum number of selections, from 0 to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    /// The maximum number of selections, up to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    /// Whether a selection is required in a modal.
    ///
    /// Discord defaults this to `true`; the field is ignored in messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Whether the menu is disabled in a message.
    ///
    /// Discord defaults this to `false`; using it in a modal is an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// A select menu populated with guild users and roles.
///
/// See [Discord's mentionable select component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#mentionable-select
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct MentionableSelect {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::mentionable_select"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// Placeholder text shown when nothing is selected, up to 150 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The options selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<SelectDefaultValue>>,

    /// The minimum number of selections, from 0 to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    /// The maximum number of selections, up to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    /// Whether a selection is required in a modal.
    ///
    /// Discord defaults this to `true`; the field is ignored in messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Whether the menu is disabled in a message.
    ///
    /// Discord defaults this to `false`; using it in a modal is an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// A select menu populated with guild channels.
///
/// See [Discord's channel select component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#channel-select
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ChannelSelect {
    /// The component type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::component::tag::channel_select"
    )]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The developer-defined identifier, between 1 and 100 characters.
    pub custom_id: String,

    /// Channel types accepted by the menu.
    ///
    /// `None` accepts all channel types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<ChannelType>>,

    /// Placeholder text shown when nothing is selected, up to 150 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,

    /// The options selected by default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_values: Option<Vec<SelectDefaultValue>>,

    /// The minimum number of selections, from 0 to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    /// The maximum number of selections, up to 25.
    ///
    /// Discord defaults this to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    /// Whether a selection is required in a modal.
    ///
    /// Discord defaults this to `true`; the field is ignored in messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// Whether the menu is disabled in a message.
    ///
    /// Discord defaults this to `false`; using it in a modal is an error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}

/// A default value for an auto-populated select menu.
///
/// The tag is a string, unlike the integer component `type` fields. The ID is
/// typed according to the tag so user, role, and channel IDs cannot be mixed
/// up. See [Discord's select default value object][object] for details.
///
/// [object]: https://docs.discord.com/developers/components/reference#user-select-select-default-value-structure
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SelectDefaultValue {
    /// A user selected by default.
    #[serde(rename = "user")]
    User {
        /// The user's ID.
        id: UserId,
    },

    /// A role selected by default.
    #[serde(rename = "role")]
    Role {
        /// The role's ID.
        id: RoleId,
    },

    /// A channel selected by default.
    #[serde(rename = "channel")]
    Channel {
        /// The channel's ID.
        id: ChannelId,
    },
}
