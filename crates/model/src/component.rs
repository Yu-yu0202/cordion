/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Message and modal components.
//!
//! The types in this module mirror the JSON component objects Discord sends
//! and accepts. Component objects are untagged on the wire: the integer `type`
//! field selects the shape. [`ComponentType`] preserves unrecognized values,
//! and [`Component`] retains component shapes Cordion does not model in
//! [`Component::Unknown`].
//!
//! See [Discord's component reference][reference] for details.
//!
//! [reference]: https://docs.discord.com/developers/components/reference

mod button;
mod content;
mod kind;
mod layout;
mod modal;
mod response;
mod select;
pub(crate) mod tag;
mod text_input;

pub use button::{Button, ButtonStyle};
pub use content::{
    ContentScanMetadata, ContentScanMetadataFlags, FileComponent, MediaGallery, MediaGalleryItem,
    TextDisplay, Thumbnail, UnfurledMediaItem, UnfurledMediaItemFlags,
};
pub use kind::ComponentType;
pub use layout::{ActionRow, Container, Label, Section, Separator, SeparatorSpacing};
pub use modal::{
    Checkbox, CheckboxGroup, CheckboxGroupOption, FileUpload, RadioGroup, RadioGroupOption,
};
pub use response::ComponentResponse;
pub use select::{
    ChannelSelect, Mentionable, MentionableId, MentionableSelect, RoleSelect, SelectDefaultValue,
    SelectOption, StringSelect, UserSelect,
};
use serde::{Deserialize, Serialize};
pub use text_input::{TextInput, TextInputStyle};

/// A message or modal component.
///
/// Discord selects the concrete shape with the integer `type` field. Each known
/// variant validates that tag during deserialization, and
/// [`Unknown`](Self::Unknown) preserves component objects whose type Cordion
/// does not model. Unknown fields inside known components are ignored as
/// Discord adds them.
///
/// See [Discord's component object][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#component-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Component {
    /// An action row.
    ActionRow(ActionRow),

    /// A button.
    Button(Button),

    /// A string select menu.
    StringSelect(StringSelect),

    /// A text input used in modals.
    TextInput(TextInput),

    /// A user select menu.
    UserSelect(UserSelect),

    /// A role select menu.
    RoleSelect(RoleSelect),

    /// A mentionable select menu.
    MentionableSelect(MentionableSelect),

    /// A channel select menu.
    ChannelSelect(ChannelSelect),

    /// A section.
    Section(Section),

    /// Markdown text.
    TextDisplay(TextDisplay),

    /// A thumbnail.
    Thumbnail(Thumbnail),

    /// A media gallery.
    MediaGallery(MediaGallery),

    /// An attached file.
    File(FileComponent),

    /// A separator.
    Separator(Separator),

    /// A container.
    Container(Container),

    /// A label wrapping a modal component.
    Label(Label),

    /// A file upload.
    FileUpload(FileUpload),

    /// A radio group.
    RadioGroup(RadioGroup),

    /// A checkbox group.
    CheckboxGroup(CheckboxGroup),

    /// A checkbox.
    Checkbox(Checkbox),

    /// A component shape that Cordion does not model.
    Unknown(serde_json::Value),
}
