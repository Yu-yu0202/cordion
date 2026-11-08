/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Interaction payloads Discord sends.
//!
//! [`InteractionData`] is the `data` field of an [`Interaction`] and is selected
//! by the interaction type: [`ApplicationCommandData`] for command and
//! autocomplete interactions, [`MessageComponentData`] for component
//! interactions, and [`ModalSubmitData`] for modal submissions.
//! [`ResolvedData`] is the `resolved` table shared by those shapes.
//! [`AuthorizingIntegrationOwners`] and [`InteractionGuild`] are the surrounding
//! context objects, and the [`MessageInteraction`] family is the deprecated and
//! current metadata Discord attaches to messages an interaction produced.
//!
//! See [Discord's receiving and responding documentation][reference] for details.
//!
//! [reference]: https://docs.discord.com/developers/interactions/receiving-and-responding

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::application_command::{
    ApplicationCommandId, ApplicationCommandOptionType, ApplicationCommandType, CommandTargetId,
};
use crate::channel::{ChannelId, PartialChannel};
use crate::component::{ComponentResponse, ComponentType};
use crate::guild::{GuildFeature, GuildId, PartialGuildMember};
use crate::id::snowflake_option;
use crate::interaction::{InteractionId, InteractionType};
use crate::message::{Attachment, AttachmentId, Message, MessageId};
use crate::role::RoleId;
use crate::user::{User, UserId};

/// The partial guild embedded in an [`Interaction`](super::Interaction).
///
/// Discord describes this value as a partial guild object. The documented
/// payload contains the guild ID and may include features and a locale; other
/// guild fields are ignored during deserialization.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InteractionGuild {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's enabled features.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<GuildFeature>>,

    /// The guild's preferred locale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// The installation contexts that authorized an interaction.
///
/// Discord keys this object by application integration type: `"0"` for guild
/// installation and `"1"` for user installation. A key is only present when the
/// application is authorized for that context and the interaction is supported
/// there. Unknown keys are preserved in
/// [`unknown`](Self::unknown).
///
/// See [Discord's authorizing integration owners object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-authorizing-integration-owners-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct AuthorizingIntegrationOwners {
    /// The guild-install owner, keyed by `"0"`.
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub guild_install: Option<GuildInstallOwner>,

    /// The user-install owner, keyed by `"1"`.
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub user_install: Option<UserId>,

    /// Integration-context keys that Cordion does not model.
    #[serde(flatten)]
    pub unknown: BTreeMap<String, serde_json::Value>,
}

/// The owner of a guild installation context.
///
/// Discord sends a guild ID when the interaction came from a server and the
/// string `"0"` when it came from a DM with the application's bot user.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GuildInstallOwner {
    /// The interaction came from this guild.
    Guild(GuildId),

    /// The interaction came from a DM with the application's bot user.
    BotDm,
}

impl Serialize for GuildInstallOwner {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Guild(id) => id.serialize(serializer),
            Self::BotDm => serializer.serialize_str("0"),
        }
    }
}

impl<'de> Deserialize<'de> for GuildInstallOwner {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = GuildId::deserialize(deserializer)?;

        if id.get() == 0 {
            Ok(Self::BotDm)
        } else {
            Ok(Self::Guild(id))
        }
    }
}

/// The data payload of an [`Interaction`](super::Interaction).
///
/// The payload shape is determined by the interaction type and has no tag of
/// its own. Variants are listed from most to least specific, and unrecognized
/// shapes are preserved in [`Unknown`](Self::Unknown).
///
/// See [Discord's interaction data][data] for details.
///
/// [data]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-interaction-data
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum InteractionData {
    /// Data for application command and autocomplete interactions.
    ApplicationCommand(ApplicationCommandData),

    /// Data for message component interactions.
    MessageComponent(MessageComponentData),

    /// Data for modal submit interactions.
    ModalSubmit(ModalSubmitData),

    /// An interaction data shape that Cordion does not model.
    Unknown(serde_json::Value),
}

/// Interaction data for application command and autocomplete interactions.
///
/// Optional fields decode as `None` when absent or `null` and are omitted
/// during serialization.
///
/// See [Discord's application command data structure][data] for details.
///
/// [data]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-application-command-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandData {
    /// The invoked command's ID.
    pub id: ApplicationCommandId,

    /// The invoked command's name.
    pub name: String,

    /// The invoked command's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandType,

    /// Users, members, roles, channels, messages, and attachments resolved from
    /// the interaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedData>,

    /// The parameters and values supplied by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,

    /// The ID of the guild the command is registered to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The user or message targeted by a user or message command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_id: Option<CommandTargetId>,

    /// The command's version.
    ///
    /// Discord includes this field in autocomplete interaction payloads but
    /// does not list it in the application command data field table, so it is
    /// optional. It is serialized as a decimal string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "snowflake_option"
    )]
    pub version: Option<u64>,
}

/// One submitted application command option.
///
/// [`value`](Self::value) and [`options`](Self::options) are mutually exclusive
/// on the wire.
///
/// See [Discord's application command interaction data option][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-application-command-interaction-data-option-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionDataOption {
    /// The option's name.
    pub name: String,

    /// The option's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ApplicationCommandOptionType,

    /// The value supplied by the user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<CommandOptionValue>,

    /// Nested options for a subcommand or subcommand group.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Self>>,

    /// Whether this option is focused for autocomplete.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub focused: Option<bool>,
}

/// A value submitted for an application command option.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CommandOptionValue {
    /// A string value.
    String(String),

    /// An integer value.
    Integer(i64),

    /// A floating-point value.
    Number(f64),

    /// A boolean value.
    Boolean(bool),
}

/// Interaction data for message component interactions.
///
/// [`values`](Self::values) is present for select menu components.
/// The component's `id` is documented in payload examples but not in Discord's
/// field table, so it is optional.
///
/// See [Discord's message component data structure][data] for details.
///
/// [data]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-message-component-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MessageComponentData {
    /// The component's developer-defined identifier.
    pub custom_id: String,

    /// The component's type, serialized as `component_type`.
    #[serde(rename = "component_type")]
    pub component_type: ComponentType,

    /// The values selected in a select menu.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,

    /// The entities resolved from the selected values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedData>,

    /// The component's identifier, when included.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

/// Interaction data for modal submissions.
///
/// See [Discord's modal submit data structure][data] for details.
///
/// [data]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-modal-submit-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ModalSubmitData {
    /// The custom ID provided for the modal.
    pub custom_id: String,

    /// The component values submitted by the user.
    pub components: Vec<ComponentResponse>,

    /// The entities resolved from the submitted values.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<ResolvedData>,
}

/// Entities resolved from an interaction's submitted values.
///
/// Map keys are snowflakes serialized as strings. Members are missing `user`,
/// `deaf`, and `mute`; the corresponding users are provided in
/// [`users`](Self::users). Channels are partial channel objects. Discord labels
/// [`messages`](Self::messages) as partial message objects; the documented
/// payloads carry the standard message shape, so this field uses [`Message`].
///
/// See [Discord's resolved data structure][data] for details.
///
/// [data]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-resolved-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct ResolvedData {
    /// Users keyed by user ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<BTreeMap<UserId, User>>,

    /// Partial members keyed by user ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<BTreeMap<UserId, PartialGuildMember>>,

    /// Roles keyed by role ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<BTreeMap<RoleId, crate::role::Role>>,

    /// Partial channels keyed by channel ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<BTreeMap<ChannelId, PartialChannel>>,

    /// Messages keyed by message ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<BTreeMap<MessageId, Message>>,

    /// Attachments keyed by attachment ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<BTreeMap<AttachmentId, Attachment>>,
}

/// The deprecated message interaction object on a [`Message`].
///
/// Discord recommends [`MessageInteractionMetadata`] through the message's
/// `interaction_metadata` field.
///
/// See [Discord's message interaction structure][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#message-interaction-object-message-interaction-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MessageInteraction {
    /// The interaction's ID.
    pub id: InteractionId,

    /// The interaction type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: InteractionType,

    /// The command name, including subcommands and subcommand groups.
    pub name: String,

    /// The user who invoked the interaction.
    pub user: User,

    /// The member who invoked the interaction in the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialGuildMember>,
}

/// Metadata about the interaction that produced a [`Message`].
///
/// The concrete shape depends on the interaction type. Unrecognized shapes are
/// preserved in [`Unknown`](Self::Unknown) so that future interaction types do
/// not fail decoding. Variants are boxed because the metadata structures are
/// much larger than the surrounding enum.
///
/// See [Discord's message interaction metadata object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-interaction-metadata-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MessageInteractionMetadata {
    /// Metadata for a modal submit interaction.
    ModalSubmit(Box<ModalSubmitInteractionMetadata>),

    /// Metadata for a message component interaction.
    MessageComponent(Box<MessageComponentInteractionMetadata>),

    /// Metadata for an application command interaction.
    ApplicationCommand(Box<ApplicationCommandInteractionMetadata>),

    /// A metadata shape that Cordion does not model.
    Unknown(serde_json::Value),
}

/// Metadata for an application command interaction.
///
/// See [Discord's application command interaction metadata structure][object]
/// for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-interaction-metadata-object-application-command-interaction-metadata-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionMetadata {
    /// The interaction's ID.
    pub id: InteractionId,

    /// The interaction type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::interaction::tag::application_command"
    )]
    pub kind: InteractionType,

    /// The user who triggered the interaction.
    pub user: User,

    /// Installation contexts the interaction was authorized for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizing_integration_owners: Option<AuthorizingIntegrationOwners>,

    /// The original response message, present only on follow-up messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<MessageId>,

    /// The user the command was run on, for user commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,

    /// The message the command was run on, for message commands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_message_id: Option<MessageId>,
}

/// Metadata for a message component interaction.
///
/// See [Discord's message component interaction metadata structure][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-interaction-metadata-object-message-component-interaction-metadata-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct MessageComponentInteractionMetadata {
    /// The interaction's ID.
    pub id: InteractionId,

    /// The interaction type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::interaction::tag::message_component"
    )]
    pub kind: InteractionType,

    /// The user who triggered the interaction.
    pub user: User,

    /// Installation contexts the interaction was authorized for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizing_integration_owners: Option<AuthorizingIntegrationOwners>,

    /// The original response message, present only on follow-up messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<MessageId>,

    /// The message that contained the interactive component.
    pub interacted_message_id: MessageId,
}

/// Metadata for a modal submit interaction.
///
/// See [Discord's modal submit interaction metadata structure][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/resources/message#message-interaction-metadata-object-modal-submit-interaction-metadata-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ModalSubmitInteractionMetadata {
    /// The interaction's ID.
    pub id: InteractionId,

    /// The interaction type, serialized as `type`.
    #[serde(
        rename = "type",
        deserialize_with = "crate::interaction::tag::modal_submit"
    )]
    pub kind: InteractionType,

    /// The user who triggered the interaction.
    pub user: User,

    /// Installation contexts the interaction was authorized for.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizing_integration_owners: Option<AuthorizingIntegrationOwners>,

    /// The original response message, present only on follow-up messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<MessageId>,

    /// Metadata for the interaction that opened the modal.
    pub triggering_interaction_metadata: Box<MessageInteractionMetadata>,
}
