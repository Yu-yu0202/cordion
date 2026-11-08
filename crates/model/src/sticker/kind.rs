/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

/// The type of a [`Sticker`](super::Sticker).
///
/// Unrecognized values are preserved. See [Discord's sticker types][types].
///
/// [types]: https://docs.discord.com/developers/resources/sticker#sticker-object-sticker-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct StickerType(u8);

impl StickerType {
    /// An official sticker in a pack.
    pub const STANDARD: Self = Self(1);

    /// A sticker uploaded to a guild for its members.
    pub const GUILD: Self = Self(2);
}

/// The format of a [`Sticker`](super::Sticker).
///
/// Unrecognized values are preserved. See [Discord's sticker formats][formats].
///
/// [formats]: https://docs.discord.com/developers/resources/sticker#sticker-object-sticker-format-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct StickerFormatType(u8);

impl StickerFormatType {
    /// A PNG image.
    pub const PNG: Self = Self(1);

    /// An APNG image.
    pub const APNG: Self = Self(2);

    /// A Lottie JSON animation.
    pub const LOTTIE: Self = Self(3);

    /// A GIF image.
    pub const GIF: Self = Self(4);
}
