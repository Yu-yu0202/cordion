/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Component values submitted in modal interactions.

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::component::{ComponentType, MentionableId};
use crate::interaction::ResolvedData;
use crate::message::AttachmentId;
use crate::role::RoleId;
use crate::user::UserId;

/// A component value submitted by a user in a modal.
///
/// Unlike [`Component`](crate::component::Component), these structures describe
/// the submitted values rather than the component definition. Modal submissions
/// put the submitted component responses inside the `components` array of the
/// interaction data; the enclosing interaction type is
/// [`MODAL_SUBMIT`](crate::interaction::InteractionType::MODAL_SUBMIT).
///
/// Select responses may include [`resolved`](Self) entities. Unknown component
/// response shapes, including component types added by Discord in the future,
/// are preserved in [`Unknown`](Self::Unknown).
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ComponentResponse {
    /// A string select response.
    StringSelect {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::string_select"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The values of the selected options.
        values: Vec<String>,
    },

    /// A text input response.
    TextInput {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::text_input"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The text entered by the user.
        value: String,
    },

    /// A user select response.
    UserSelect {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::user_select"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The IDs of the selected users.
        values: Vec<UserId>,

        /// The resolved entities from the selected users.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        resolved: Option<ResolvedData>,
    },

    /// A role select response.
    RoleSelect {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::role_select"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The IDs of the selected roles.
        values: Vec<RoleId>,

        /// The resolved entities from the selected roles.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        resolved: Option<ResolvedData>,
    },

    /// A mentionable select response.
    MentionableSelect {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::mentionable_select"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The IDs of the selected users and roles.
        values: Vec<MentionableId>,

        /// The resolved entities from the selected users and roles.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        resolved: Option<ResolvedData>,
    },

    /// A channel select response.
    ChannelSelect {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::channel_select"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The IDs of the selected channels.
        values: Vec<ChannelId>,

        /// The resolved entities from the selected channels.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        resolved: Option<ResolvedData>,
    },

    /// A label wrapping another submitted component.
    Label {
        /// The component type, serialized as `type`.
        #[serde(rename = "type", deserialize_with = "crate::component::tag::label")]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The wrapped component response.
        component: Box<ComponentResponse>,
    },

    /// A file upload response.
    FileUpload {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::file_upload"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The IDs of the uploaded attachments, resolved through
        /// [`ResolvedData`](crate::interaction::ResolvedData).
        values: Vec<AttachmentId>,
    },

    /// A radio group response.
    RadioGroup {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::radio_group"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The selected option's value, serialized as `null` when no option
        /// was selected.
        value: Option<String>,
    },

    /// A checkbox group response.
    CheckboxGroup {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::checkbox_group"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// The values of the selected options, or an empty array.
        values: Vec<String>,
    },

    /// A single checkbox response.
    Checkbox {
        /// The component type, serialized as `type`.
        #[serde(rename = "type", deserialize_with = "crate::component::tag::checkbox")]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,

        /// The developer-defined identifier.
        custom_id: String,

        /// Whether the checkbox is selected.
        value: bool,
    },

    /// An action row returned by older modals.
    ///
    /// Discord's current component reference documents labels as the wrapper
    /// for modal components and marks text inputs in action rows as deprecated,
    /// but older modals can still return action rows. The component reference
    /// does not publish a dedicated interaction response structure for action
    /// rows, so the fields mirror the request shape.
    ActionRow {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::action_row"
        )]
        kind: ComponentType,

        /// The component's optional identifier.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<u32>,

        /// The submitted components inside the row.
        components: Vec<ComponentResponse>,
    },

    /// A text display response.
    ///
    /// This variant is intentionally last among the known component types:
    /// text display responses contain only `type` and `id`, so a broader
    /// variant listed earlier would match other component responses first.
    TextDisplay {
        /// The component type, serialized as `type`.
        #[serde(
            rename = "type",
            deserialize_with = "crate::component::tag::text_display"
        )]
        kind: ComponentType,

        /// The component's identifier.
        id: u32,
    },

    /// A component response shape that Cordion does not model.
    Unknown(serde_json::Value),
}
