/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Button components.

use serde::{Deserialize, Serialize};

use crate::component::ComponentType;
use crate::emoji::PartialEmoji;
use crate::monetization::sku::SkuId;

/// The visual style and behavior of a [`Button`].
///
/// Unrecognized values are preserved. See [Discord's button styles][styles] for
/// details.
///
/// [styles]: https://docs.discord.com/developers/components/reference#button-button-styles
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ButtonStyle(u8);

impl ButtonStyle {
    /// The most important or recommended action.
    pub const PRIMARY: Self = Self(1);

    /// An alternative or supporting action.
    pub const SECONDARY: Self = Self(2);

    /// A positive confirmation or completion action.
    pub const SUCCESS: Self = Self(3);

    /// An action with irreversible consequences.
    pub const DANGER: Self = Self(4);

    /// A button that navigates to a URL.
    pub const LINK: Self = Self(5);

    /// A button that starts a SKU purchase.
    pub const PREMIUM: Self = Self(6);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A clickable button.
///
/// Which fields are valid depends on [`style`](Self::style):
///
/// - `PRIMARY`, `SECONDARY`, `SUCCESS`, and `DANGER` require
///   [`custom_id`](Self::custom_id) and cannot include [`url`](Self::url) or
///   [`sku_id`](Self::sku_id).
/// - `LINK` requires [`url`](Self::url) and cannot include
///   [`custom_id`](Self::custom_id).
/// - `PREMIUM` requires [`sku_id`](Self::sku_id) and cannot include
///   [`custom_id`](Self::custom_id), [`label`](Self::label),
///   [`url`](Self::url), or [`emoji`](Self::emoji).
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's button component][component] for details.
///
/// [component]: https://docs.discord.com/developers/components/reference#button
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Button {
    /// The component type, serialized as `type`.
    #[serde(rename = "type", deserialize_with = "crate::component::tag::button")]
    pub kind: ComponentType,

    /// The component's optional 32-bit identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    /// The button's style.
    pub style: ButtonStyle,

    /// The button's label, up to 80 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,

    /// The button's emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    /// The developer-defined identifier, between 1 and 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,

    /// The SKU purchased by a premium button.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<SkuId>,

    /// The URL opened by a link button, up to 512 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Whether the button is disabled.
    ///
    /// Discord defaults this to `false` when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
}
