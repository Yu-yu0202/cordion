/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::convert::Infallible;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::id::Id;
use crate::monetization::sku::Sku;

/// A collectible nameplate equipped by a user.
///
/// See [Discord's nameplate object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/user#nameplate-nameplate-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Nameplate {
    /// The ID of the nameplate SKU.
    pub sku_id: Id<Sku>,

    /// The path to the nameplate asset.
    pub asset: String,

    /// The nameplate's label.
    ///
    /// Discord currently does not set a meaningful value.
    pub label: String,

    /// The nameplate's background color palette.
    pub palette: NameplatePalette,
}

/// A nameplate background color palette.
///
/// Unknown palette identifiers are preserved in [`Unknown`](Self::Unknown).
#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
#[allow(
    missing_docs,
    reason = "Variant names mirror Discord's palette identifiers."
)]
pub enum NameplatePalette {
    Crimson,
    Berry,
    Sky,
    Teal,
    Forest,
    BubbleGum,
    Violet,
    Cobalt,
    Clover,
    Lemon,
    White,

    /// An unrecognized palette identifier returned by Discord.
    Unknown(String),
}

impl NameplatePalette {
    /// Returns the palette identifier used by Discord.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Crimson => "crimson",
            Self::Berry => "berry",
            Self::Sky => "sky",
            Self::Teal => "teal",
            Self::Forest => "forest",
            Self::BubbleGum => "bubble_gum",
            Self::Violet => "violet",
            Self::Cobalt => "cobalt",
            Self::Clover => "clover",
            Self::Lemon => "lemon",
            Self::White => "white",
            Self::Unknown(value) => value.as_str(),
        }
    }

    /// Parses a palette identifier, preserving unrecognized values.
    fn parse(value: &str) -> Self {
        match value {
            "crimson" => Self::Crimson,
            "berry" => Self::Berry,
            "sky" => Self::Sky,
            "teal" => Self::Teal,
            "forest" => Self::Forest,
            "bubble_gum" => Self::BubbleGum,
            "violet" => Self::Violet,
            "cobalt" => Self::Cobalt,
            "clover" => Self::Clover,
            "lemon" => Self::Lemon,
            "white" => Self::White,
            _ => Self::Unknown(value.to_owned()),
        }
    }
}

impl FromStr for NameplatePalette {
    type Err = Infallible;

    /// Parses a palette identifier without failing.
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(value))
    }
}

impl From<String> for NameplatePalette {
    fn from(value: String) -> Self {
        match Self::parse(&value) {
            Self::Unknown(_) => Self::Unknown(value),
            known => known,
        }
    }
}

impl From<&str> for NameplatePalette {
    fn from(value: &str) -> Self {
        Self::parse(value)
    }
}

impl Serialize for NameplatePalette {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for NameplatePalette {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(String::deserialize(deserializer)?))
    }
}
