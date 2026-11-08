/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The registered application command object.
//!
//! [`ApplicationCommand`] is what Discord's command endpoints return: the
//! command's parameters ([`ApplicationCommandOption`](super::ApplicationCommandOption)),
//! its choices ([`CommandOptionChoice`](super::CommandOptionChoice)), its
//! localized strings ([`Localizations`]), and its
//! [`ApplicationCommandType`] and [`ApplicationCommandHandler`]. Registration
//! endpoints take smaller bodies, which are modeled in
//! [`request`](super::request), and the per-guild overwrites are modeled in
//! [`permission`](super::permission).
//!
//! See [Discord's application commands][commands] for details.
//!
//! [commands]: https://docs.discord.com/developers/interactions/application-commands

use std::collections::BTreeMap;

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use super::ApplicationCommandOption;
use crate::application::{ApplicationId, ApplicationIntegrationType};
use crate::guild::GuildId;
use crate::id::{Id, Identifiable, snowflake};
use crate::interaction::InteractionContextType;
use crate::permission::Permission;

/// The ID of an [`ApplicationCommand`].
pub type ApplicationCommandId = Id<ApplicationCommand>;

/// A registered application command.
///
/// [`kind`](Self::kind) defaults to [`ApplicationCommandType::CHAT_INPUT`] when
/// the field is absent. [`default_member_permissions`](Self::default_member_permissions)
/// is required but nullable; a missing field is rejected during
/// deserialization. The nullable fields documented by Discord
/// ([`name_localizations`](Self::name_localizations),
/// [`description_localizations`](Self::description_localizations),
/// [`default_member_permissions`](Self::default_member_permissions),
/// [`default_permission`](Self::default_permission), and
/// [`contexts`](Self::contexts)) are serialized as `null` when `None`.
///
/// See [Discord's application command object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/application-commands#application-command-object-application-command-structure
#[derive(Clone, PartialEq, IdTarget, Serialize, Deserialize, Debug)]
pub struct ApplicationCommand {
    /// The command's ID.
    pub id: ApplicationCommandId,

    /// The command's type, serialized as `type`.
    #[serde(rename = "type", default)]
    pub kind: ApplicationCommandType,

    /// The ID of the parent application.
    pub application_id: ApplicationId,

    /// The ID of the guild the command is registered to, if it is not global.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The command's name, between 1 and 32 characters.
    pub name: String,

    /// Localized variants of [`name`](Self::name).
    #[serde(default)]
    pub name_localizations: Option<Localizations>,

    /// The command's name localized to the requester's locale.
    ///
    /// Discord adds this field to commands returned by endpoints that list
    /// every command in a scope while localization is active, instead of the
    /// full [`name_localizations`](Self::name_localizations) dictionary. It is
    /// absent on responses that model the plain command structure and is
    /// omitted during serialization when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<String>,

    /// The command's description, between 1 and 100 characters.
    ///
    /// This is an empty string for [`USER`](ApplicationCommandType::USER) and
    /// [`MESSAGE`](ApplicationCommandType::MESSAGE) commands.
    pub description: String,

    /// Localized variants of [`description`](Self::description).
    #[serde(default)]
    pub description_localizations: Option<Localizations>,

    /// The command's description localized to the requester's locale.
    ///
    /// Like [`name_localized`](Self::name_localized), this only appears in
    /// localized responses and is omitted during serialization when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<String>,

    /// The command's parameters, up to 25.
    ///
    /// Only [`CHAT_INPUT`](ApplicationCommandType::CHAT_INPUT) commands can
    /// define options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,

    /// The default permissions a member must have to use the command.
    ///
    /// Setting this to the empty permission set (`"0"`) restricts the command
    /// to administrators unless a permission overwrite applies.
    pub default_member_permissions: Option<Permission>,

    /// Whether the command is available in DMs with the application.
    ///
    /// Deprecated by Discord in favor of [`contexts`](Self::contexts). This
    /// only applies to globally-scoped commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,

    /// Whether the command is enabled by default when the application is added
    /// to a guild.
    ///
    /// Discord recommends [`default_member_permissions`](Self::default_member_permissions)
    /// and [`contexts`](Self::contexts) instead.
    #[serde(default)]
    pub default_permission: Option<bool>,

    /// Whether the command is age-restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// The installation contexts where the command is available.
    ///
    /// This only applies to globally-scoped commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,

    /// The interaction contexts where the command can be used.
    ///
    /// This only applies to globally-scoped commands.
    #[serde(default)]
    pub contexts: Option<Vec<InteractionContextType>>,

    /// An autoincrementing version identifier updated during substantial record
    /// changes.
    ///
    /// Discord sends this as a snowflake-formatted decimal string. It is not a
    /// resource ID, so it is exposed as a raw `u64`.
    #[serde(with = "snowflake")]
    pub version: u64,

    /// How an entry point command is handled.
    ///
    /// This only applies to
    /// [`PRIMARY_ENTRY_POINT`](ApplicationCommandType::PRIMARY_ENTRY_POINT)
    /// commands for applications with the `EMBEDDED` flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<ApplicationCommandHandler>,
}

impl Identifiable for ApplicationCommand {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The type of an application command.
///
/// Unrecognized values are preserved. See [Discord's command types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/interactions/application-commands#application-command-object-application-command-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApplicationCommandType(u8);

impl ApplicationCommandType {
    /// A text-based slash command shown when a user types `/`.
    pub const CHAT_INPUT: Self = Self(1);

    /// A UI-based command shown when a user right-clicks or taps a user.
    pub const USER: Self = Self(2);

    /// A UI-based command shown when a user right-clicks or taps a message.
    pub const MESSAGE: Self = Self(3);

    /// A UI-based command that represents the primary way to launch an
    /// Activity.
    pub const PRIMARY_ENTRY_POINT: Self = Self(4);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

impl Default for ApplicationCommandType {
    #[inline]
    fn default() -> Self {
        Self::CHAT_INPUT
    }
}

/// How an entry point application command is handled.
///
/// Unrecognized values are preserved. See [Discord's entry point handler
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/interactions/application-commands#application-command-object-entry-point-command-handler-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApplicationCommandHandler(u8);

impl ApplicationCommandHandler {
    /// The application handles the interaction using an interaction token.
    pub const APP_HANDLER: Self = Self(1);

    /// Discord handles the interaction by launching an Activity.
    pub const DISCORD_LAUNCH_ACTIVITY: Self = Self(2);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A localized string map.
///
/// Discord keys localization dictionaries by locale, such as `"en-US"`, and
/// values are the localized strings. Unknown locale keys are preserved.
pub type Localizations = BTreeMap<String, String>;
