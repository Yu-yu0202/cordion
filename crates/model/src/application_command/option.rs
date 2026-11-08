/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Application command option structures.
//!
//! [`ApplicationCommandOption`] is one parameter of an
//! [`ApplicationCommand`](super::ApplicationCommand): its
//! [`ApplicationCommandOptionType`], its choices
//! ([`CommandOptionChoice`](super::CommandOptionChoice)), its numeric bounds
//! ([`CommandOptionNumericValue`]), and its nested options for subcommands and
//! subcommand groups.
//!
//! [`CommandTarget`] and [`CommandTargetId`] mark the ID of the user or message
//! a user or message command targets.
//!
//! See [Discord's application commands][commands] for details.
//!
//! [commands]: https://docs.discord.com/developers/interactions/application-commands

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use super::{CommandOptionChoice, Localizations};
use crate::channel::ChannelType;
use crate::id::Id;

/// The type of an application command option.
///
/// Unrecognized values are preserved. See [Discord's option types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/interactions/application-commands#application-command-object-application-command-option-type
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApplicationCommandOptionType(u8);

impl ApplicationCommandOptionType {
    /// A subcommand.
    pub const SUB_COMMAND: Self = Self(1);

    /// A group of subcommands.
    pub const SUB_COMMAND_GROUP: Self = Self(2);

    /// A string parameter.
    pub const STRING: Self = Self(3);

    /// An integer parameter.
    pub const INTEGER: Self = Self(4);

    /// A boolean parameter.
    pub const BOOLEAN: Self = Self(5);

    /// A user parameter.
    pub const USER: Self = Self(6);

    /// A channel parameter.
    pub const CHANNEL: Self = Self(7);

    /// A role parameter.
    pub const ROLE: Self = Self(8);

    /// A user or role parameter.
    pub const MENTIONABLE: Self = Self(9);

    /// A floating-point number parameter.
    pub const NUMBER: Self = Self(10);

    /// An attachment parameter.
    pub const ATTACHMENT: Self = Self(11);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// One parameter of an [`ApplicationCommand`](super::ApplicationCommand).
///
/// Required options must be listed before optional options. The nullable
/// localization fields are serialized as `null` when `None`.
///
/// See [Discord's application command option object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/application-commands#application-command-object-application-command-option-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandOption {
    /// The option's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandOptionType,

    /// The option's name, between 1 and 32 characters.
    pub name: String,

    /// Localized variants of [`name`](Self::name).
    #[serde(default)]
    pub name_localizations: Option<Localizations>,

    /// The option's name localized to the requester's locale.
    ///
    /// Like [`ApplicationCommand::name_localized`](super::ApplicationCommand::name_localized),
    /// this only appears in localized responses and is omitted during serialization
    /// when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<String>,

    /// The option's description, between 1 and 100 characters.
    pub description: String,

    /// Localized variants of [`description`](Self::description).
    #[serde(default)]
    pub description_localizations: Option<Localizations>,

    /// The option's description localized to the requester's locale.
    ///
    /// Omitted during serialization when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<String>,

    /// Whether the option is required.
    ///
    /// This does not apply to subcommands or subcommand groups.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,

    /// The choices the user can pick from, up to 25.
    ///
    /// This only applies to [`STRING`](ApplicationCommandOptionType::STRING),
    /// [`INTEGER`](ApplicationCommandOptionType::INTEGER), and
    /// [`NUMBER`](ApplicationCommandOptionType::NUMBER) options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<CommandOptionChoice>>,

    /// Nested options for a subcommand or subcommand group, up to 25.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Self>>,

    /// The channel types shown for a [`CHANNEL`](ApplicationCommandOptionType::CHANNEL)
    /// option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_types: Option<Vec<ChannelType>>,

    /// The minimum permitted value for an integer or number option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<CommandOptionNumericValue>,

    /// The maximum permitted value for an integer or number option.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<CommandOptionNumericValue>,

    /// The minimum allowed length for a string option, between 0 and 6000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u64>,

    /// The maximum allowed length for a string option, between 1 and 6000.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u64>,

    /// Whether autocomplete interactions are enabled for this option.
    ///
    /// This cannot be `true` when [`choices`](Self::choices) are present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autocomplete: Option<bool>,

    /// File types to filter for an
    /// [`ATTACHMENT`](ApplicationCommandOptionType::ATTACHMENT) option.
    ///
    /// Values can be `image`, `video`, `audio`, or a dot-prefixed extension
    /// such as `.pdf`. Discord allows up to 10 entries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_types: Option<Vec<String>>,
}

/// The numeric bound of an [`ApplicationCommandOption`].
///
/// Discord sends an integer for
/// [`INTEGER`](ApplicationCommandOptionType::INTEGER) options and a
/// floating-point number for
/// [`NUMBER`](ApplicationCommandOptionType::NUMBER) options.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CommandOptionNumericValue {
    /// An integer bound.
    Integer(i64),

    /// A floating-point bound.
    Number(f64),
}

/// A marker for the target of a user or message command.
///
/// The target is a user ID for user commands and a message ID for message
/// commands. A distinct ID target keeps it from being passed to APIs that
/// require a specific resource ID without an explicit conversion.
#[derive(Clone, Copy, PartialEq, Eq, IdTarget, Debug)]
pub struct CommandTarget;

/// The ID of a [`CommandTarget`].
pub type CommandTargetId = Id<CommandTarget>;
