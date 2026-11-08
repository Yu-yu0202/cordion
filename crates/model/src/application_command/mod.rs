/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Application command models.
//!
//! [`ApplicationCommand`] models a registered command, including its options,
//! choices, and handler. Command permission overwrites are modeled by
//! [`GuildApplicationCommandPermissions`] and [`ApplicationCommandPermissions`].
//!
//! Registration endpoints take a smaller body than the command object they
//! return: `id`, `application_id`, and `version` are absent, and the field set
//! differs between global and guild scope. Those bodies are modeled by
//! [`CreateGlobalApplicationCommandRequest`],
//! [`EditGlobalApplicationCommandRequest`],
//! [`CreateGuildApplicationCommandRequest`],
//! [`EditGuildApplicationCommandRequest`], and
//! [`BulkOverwriteApplicationCommandsRequest`].
//!
//! See [Discord's application commands][commands] for details.
//!
//! [commands]: https://docs.discord.com/developers/interactions/application-commands

mod choice;
mod command;
mod option;
mod permission;
mod request;

pub use choice::{CommandOptionChoice, CommandOptionChoiceValue};
pub use command::{
    ApplicationCommand, ApplicationCommandHandler, ApplicationCommandId, ApplicationCommandType,
    Localizations,
};
pub use option::{
    ApplicationCommandOption, ApplicationCommandOptionType, CommandOptionNumericValue,
    CommandTarget, CommandTargetId,
};
pub use permission::{
    ApplicationCommandPermissionType, ApplicationCommandPermissions,
    GuildApplicationCommandPermissions,
};
pub use request::{
    BulkOverwriteApplicationCommand, BulkOverwriteApplicationCommandsRequest,
    CreateGlobalApplicationCommandRequest, CreateGuildApplicationCommandRequest,
    EditGlobalApplicationCommandRequest, EditGuildApplicationCommandRequest,
};
