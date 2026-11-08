/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Request bodies of the application command endpoints.
//!
//! Registration endpoints take a smaller body than the command object they
//! return: `id`, `application_id`, and `version` are absent, and the field set
//! differs between global and guild scope. [`CreateGlobalApplicationCommandRequest`],
//! [`EditGlobalApplicationCommandRequest`], [`CreateGuildApplicationCommandRequest`],
//! and [`EditGuildApplicationCommandRequest`] are those four bodies, while
//! [`BulkOverwriteApplicationCommandsRequest`] wraps the list of
//! [`BulkOverwriteApplicationCommand`] entries the bulk overwrite endpoints
//! take. [`ApplicationCommand`](super::ApplicationCommand) is the object the
//! endpoints return.
//!
//! See [Discord's application commands][commands] for details.
//!
//! [commands]: https://docs.discord.com/developers/interactions/application-commands

use serde::{Deserialize, Serialize};

use super::{
    ApplicationCommandHandler, ApplicationCommandId, ApplicationCommandOption,
    ApplicationCommandType, Localizations,
};
use crate::application::ApplicationIntegrationType;
use crate::interaction::InteractionContextType;
use crate::permission::Permission;

/// The body of `POST /applications/{application.id}/commands`.
///
/// Creates or overwrites a global command. Discord documents this body without
/// `id`, `application_id`, or `version`, which the returned
/// [`ApplicationCommand`](super::ApplicationCommand) carries. `name` is the only
/// required field; every other field is optional and is omitted during
/// serialization when `None`.
///
/// Discord documents [`name_localizations`](Self::name_localizations),
/// [`description_localizations`](Self::description_localizations),
/// [`default_member_permissions`](Self::default_member_permissions),
/// [`dm_permission`](Self::dm_permission),
/// [`default_permission`](Self::default_permission), and
/// [`contexts`](Self::contexts) as nullable. This type cannot emit an explicit
/// `null`, so it cannot clear those values; send the field only when a value is
/// intended.
///
/// See [Discord's create global application command endpoint][endpoint] for
/// details.
///
/// [endpoint]: https://docs.discord.com/developers/interactions/application-commands#create-global-application-command
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct CreateGlobalApplicationCommandRequest {
    /// The command's name, between 1 and 32 characters.
    pub name: String,

    /// Localized variants of [`name`](Self::name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Localizations>,

    /// The command's description, between 1 and 100 characters.
    ///
    /// Discord rejects this field for [`USER`](ApplicationCommandType::USER) and
    /// [`MESSAGE`](ApplicationCommandType::MESSAGE) commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Localized variants of [`description`](Self::description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Localizations>,

    /// The command's parameters, up to 25.
    ///
    /// Only [`CHAT_INPUT`](ApplicationCommandType::CHAT_INPUT) commands can
    /// define options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,

    /// The default permissions a member must have to use the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    /// Discord replaced this field with
    /// [`default_member_permissions`](Self::default_member_permissions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,

    /// The installation contexts where the command is available.
    ///
    /// This only applies to globally-scoped commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,

    /// The interaction contexts where the command can be used.
    ///
    /// This only applies to globally-scoped commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<InteractionContextType>>,

    /// The command's type, serialized as `type`.
    ///
    /// Discord defaults this to [`ApplicationCommandType::CHAT_INPUT`].
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ApplicationCommandType>,

    /// Whether the command is age-restricted.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// How an entry point command is handled.
    ///
    /// This only applies to
    /// [`PRIMARY_ENTRY_POINT`](ApplicationCommandType::PRIMARY_ENTRY_POINT)
    /// commands for applications with the `EMBEDDED` flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<ApplicationCommandHandler>,
}

/// The body of `PATCH /applications/{application.id}/commands/{command.id}`.
///
/// Edits a global command. Discord documents every parameter as optional, and
/// each provided field replaces the command's stored value entirely. The
/// nullable fields that [`CreateGlobalApplicationCommandRequest`] documents
/// cannot be cleared by sending an explicit `null` here either, because `None`
/// is omitted instead.
///
/// See [Discord's edit global application command endpoint][endpoint] for
/// details.
///
/// [endpoint]: https://docs.discord.com/developers/interactions/application-commands#edit-global-application-command
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct EditGlobalApplicationCommandRequest {
    /// The command's name, between 1 and 32 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Localized variants of [`name`](Self::name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Localizations>,

    /// The command's description, between 1 and 100 characters.
    ///
    /// Discord rejects this field for [`USER`](ApplicationCommandType::USER) and
    /// [`MESSAGE`](ApplicationCommandType::MESSAGE) commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Localized variants of [`description`](Self::description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Localizations>,

    /// The command's parameters, up to 25.
    ///
    /// Only [`CHAT_INPUT`](ApplicationCommandType::CHAT_INPUT) commands can
    /// define options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,

    /// The default permissions a member must have to use the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    /// Discord replaced this field with
    /// [`default_member_permissions`](Self::default_member_permissions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,

    /// The installation contexts where the command is available.
    ///
    /// This only applies to globally-scoped commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,

    /// The interaction contexts where the command can be used.
    ///
    /// This only applies to globally-scoped commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<InteractionContextType>>,

    /// The command's type, serialized as `type`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ApplicationCommandType>,

    /// Whether the command is age-restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// How an entry point command is handled.
    ///
    /// This only applies to
    /// [`PRIMARY_ENTRY_POINT`](ApplicationCommandType::PRIMARY_ENTRY_POINT)
    /// commands for applications with the `EMBEDDED` flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<ApplicationCommandHandler>,
}

/// The body of `POST /applications/{application.id}/guilds/{guild.id}/commands`.
///
/// Creates or overwrites a guild command. Discord documents a smaller body for
/// guild commands than for global commands: `dm_permission`, `integration_types`,
/// `contexts`, and `handler` are absent, because they only apply to
/// globally-scoped commands. `name` is the only required field and every other
/// field is optional and omitted during serialization when `None`.
///
/// See [Discord's create guild application command endpoint][endpoint] for
/// details.
///
/// [endpoint]: https://docs.discord.com/developers/interactions/application-commands#create-guild-application-command
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct CreateGuildApplicationCommandRequest {
    /// The command's name, between 1 and 32 characters.
    pub name: String,

    /// Localized variants of [`name`](Self::name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Localizations>,

    /// The command's description, between 1 and 100 characters.
    ///
    /// Discord rejects this field for [`USER`](ApplicationCommandType::USER) and
    /// [`MESSAGE`](ApplicationCommandType::MESSAGE) commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Localized variants of [`description`](Self::description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Localizations>,

    /// The command's parameters, up to 25.
    ///
    /// Only [`CHAT_INPUT`](ApplicationCommandType::CHAT_INPUT) commands can
    /// define options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,

    /// The default permissions a member must have to use the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permission>,

    /// Whether the command is enabled by default when the application is added
    /// to a guild.
    ///
    /// Discord replaced this field with
    /// [`default_member_permissions`](Self::default_member_permissions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,

    /// The command's type, serialized as `type`.
    ///
    /// Discord defaults this to [`ApplicationCommandType::CHAT_INPUT`].
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ApplicationCommandType>,

    /// Whether the command is age-restricted.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

/// The body of `PATCH /applications/{application.id}/guilds/{guild.id}/commands/{command.id}`.
///
/// Edits a guild command. Discord documents every parameter as optional, and
/// each provided field replaces the command's stored value entirely. As with
/// [`CreateGuildApplicationCommandRequest`], the globally-scoped command fields
/// are not documented for this endpoint and are therefore not modeled.
///
/// See [Discord's edit guild application command endpoint][endpoint] for
/// details.
///
/// [endpoint]: https://docs.discord.com/developers/interactions/application-commands#edit-guild-application-command
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct EditGuildApplicationCommandRequest {
    /// The command's name, between 1 and 32 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Localized variants of [`name`](Self::name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Localizations>,

    /// The command's description, between 1 and 100 characters.
    ///
    /// Discord rejects this field for [`USER`](ApplicationCommandType::USER) and
    /// [`MESSAGE`](ApplicationCommandType::MESSAGE) commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Localized variants of [`description`](Self::description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Localizations>,

    /// The command's parameters, up to 25.
    ///
    /// Only [`CHAT_INPUT`](ApplicationCommandType::CHAT_INPUT) commands can
    /// define options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,

    /// The default permissions a member must have to use the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permission>,

    /// Whether the command is enabled by default when the application is added
    /// to a guild.
    ///
    /// Discord replaced this field with
    /// [`default_member_permissions`](Self::default_member_permissions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,

    /// The command's type, serialized as `type`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ApplicationCommandType>,

    /// Whether the command is age-restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

/// One entry of a bulk application command overwrite body.
///
/// Discord documents this shape under
/// [`Bulk Overwrite Guild Application Commands`], and the global bulk overwrite
/// endpoint takes the same list without documenting its own field table. Unlike
/// [`CreateGuildApplicationCommandRequest`], the list may carry the command's
/// [`id`](Self::id) when it is already known, and the guild overwrite table
/// documents `dm_permission`, `integration_types`, and `contexts`.
///
/// [`description`](Self::description) is documented without a `?` in Discord's
/// table, but Discord also documents that `USER` and `MESSAGE` commands must not
/// send it, so it is optional and omitted when `None` here.
///
/// [`Bulk Overwrite Guild Application Commands`]: https://docs.discord.com/developers/interactions/application-commands#bulk-overwrite-guild-application-commands
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct BulkOverwriteApplicationCommand {
    /// The ID of the command, when it is already registered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ApplicationCommandId>,

    /// The command's name, between 1 and 32 characters.
    pub name: String,

    /// Localized variants of [`name`](Self::name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Localizations>,

    /// The command's description, between 1 and 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Localized variants of [`description`](Self::description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Localizations>,

    /// The command's parameters.
    ///
    /// Only [`CHAT_INPUT`](ApplicationCommandType::CHAT_INPUT) commands can
    /// define options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandOption>>,

    /// The default permissions a member must have to use the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permission>,

    /// Whether the command is available in DMs with the application.
    ///
    /// Deprecated by Discord in favor of [`contexts`](Self::contexts).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,

    /// Whether the command is enabled by default when the application is added
    /// to a guild.
    ///
    /// Discord replaced this field with
    /// [`default_member_permissions`](Self::default_member_permissions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,

    /// The installation contexts where the command is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,

    /// The interaction contexts where the command can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<InteractionContextType>>,

    /// The command's type, serialized as `type`.
    ///
    /// Discord defaults this to [`ApplicationCommandType::CHAT_INPUT`].
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<ApplicationCommandType>,

    /// Whether the command is age-restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    /// How an entry point command is handled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<ApplicationCommandHandler>,
}

/// The body of `PUT /applications/{application.id}/commands` and
/// `PUT /applications/{application.id}/guilds/{guild.id}/commands`.
///
/// A bulk overwrite replaces every command of every type in the target scope.
/// The body is the bare JSON array of
/// [`BulkOverwriteApplicationCommand`] entries, so this newtype serializes and
/// deserializes as that array.
///
/// See [Discord's bulk overwrite global application commands endpoint][global]
/// and [Discord's bulk overwrite guild application commands endpoint][guild]
/// for details.
///
/// [global]: https://docs.discord.com/developers/interactions/application-commands#bulk-overwrite-global-application-commands
/// [guild]: https://docs.discord.com/developers/interactions/application-commands#bulk-overwrite-guild-application-commands
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct BulkOverwriteApplicationCommandsRequest {
    /// The commands that make up the scope's complete command list.
    pub commands: Vec<BulkOverwriteApplicationCommand>,
}
