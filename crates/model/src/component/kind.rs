/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Component type identifiers.

use serde::{Deserialize, Serialize};

/// The type of a message or modal component.
///
/// Unrecognized values are preserved. See [Discord's component types][types]
/// for the current list.
///
/// [types]: https://docs.discord.com/developers/components/reference#component-object-component-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ComponentType(u8);

impl ComponentType {
    /// A row of up to five buttons or one select menu.
    pub const ACTION_ROW: Self = Self(1);

    /// A clickable button.
    pub const BUTTON: Self = Self(2);

    /// A select menu of defined string options.
    pub const STRING_SELECT: Self = Self(3);

    /// A free-form text input used in modals.
    pub const TEXT_INPUT: Self = Self(4);

    /// A select menu of users.
    pub const USER_SELECT: Self = Self(5);

    /// A select menu of roles.
    pub const ROLE_SELECT: Self = Self(6);

    /// A select menu of users and roles.
    pub const MENTIONABLE_SELECT: Self = Self(7);

    /// A select menu of channels.
    pub const CHANNEL_SELECT: Self = Self(8);

    /// A layout section with text and an accessory.
    pub const SECTION: Self = Self(9);

    /// Markdown text.
    pub const TEXT_DISPLAY: Self = Self(10);

    /// A small image used as a section accessory.
    pub const THUMBNAIL: Self = Self(11);

    /// A gallery of one to ten media items.
    pub const MEDIA_GALLERY: Self = Self(12);

    /// An attached file.
    pub const FILE: Self = Self(13);

    /// Vertical padding and an optional divider.
    pub const SEPARATOR: Self = Self(14);

    /// A visually grouped collection of components.
    pub const CONTAINER: Self = Self(17);

    /// A modal label wrapping a component.
    pub const LABEL: Self = Self(18);

    /// A file upload used in modals.
    pub const FILE_UPLOAD: Self = Self(19);

    /// A single-choice group used in modals.
    pub const RADIO_GROUP: Self = Self(21);

    /// A multi-choice group used in modals.
    pub const CHECKBOX_GROUP: Self = Self(22);

    /// A single yes-or-no checkbox used in modals.
    pub const CHECKBOX: Self = Self(23);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// Wraps a raw integer value without validating it.
    ///
    /// This is used by the crate's tag validation and is not a wire
    /// conversion; deserialization uses [`ComponentType`]'s `Deserialize`
    /// implementation.
    #[inline]
    #[must_use]
    pub const fn from_raw(value: u8) -> Self {
        Self(value)
    }

    /// Returns whether `value` is one of the component types modeled by
    /// Cordion.
    #[inline]
    #[must_use]
    pub const fn is_known_raw(value: u64) -> bool {
        matches!(
            value,
            1 | 2
                | 3
                | 4
                | 5
                | 6
                | 7
                | 8
                | 9
                | 10
                | 11
                | 12
                | 13
                | 14
                | 17
                | 18
                | 19
                | 21
                | 22
                | 23
        )
    }
}
