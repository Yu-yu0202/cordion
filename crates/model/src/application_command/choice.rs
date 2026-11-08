/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Selectable choices for command options.
//!
//! [`CommandOptionChoice`] is the `choices` array of an
//! [`ApplicationCommandOption`](super::ApplicationCommandOption) and the same
//! shape is reused by the autocomplete choices of an interaction response.
//! [`CommandOptionChoiceValue`] is the value Discord stores for the choice.
//!
//! See [Discord's application commands][commands] for details.
//!
//! [commands]: https://docs.discord.com/developers/interactions/application-commands

use serde::{Deserialize, Serialize};

use super::Localizations;

/// One selectable choice for an application command option or an autocomplete
/// response.
///
/// The value is a string, integer, or floating-point number depending on the
/// option type. Discord limits an option to 25 choices.
///
/// See [Discord's option choice object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/application-commands#application-command-object-application-command-option-choice-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct CommandOptionChoice {
    /// The choice's name, between 1 and 100 characters.
    pub name: String,

    /// Localized variants of [`name`](Self::name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Localizations>,

    /// The choice's name localized to the requester's locale.
    ///
    /// Discord also localizes choice names, so localized responses include this
    /// field instead of the full [`name_localizations`](Self::name_localizations)
    /// dictionary. It is omitted during serialization when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<String>,

    /// The value returned when this choice is selected.
    pub value: CommandOptionChoiceValue,
}

/// The value of a [`CommandOptionChoice`].
///
/// Discord sends a string for `STRING` options, an integer for `INTEGER`
/// options, and a floating-point number for `NUMBER` options.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CommandOptionChoiceValue {
    /// A string choice value.
    String(String),

    /// An integer choice value.
    Integer(i64),

    /// A floating-point choice value.
    Number(f64),
}
