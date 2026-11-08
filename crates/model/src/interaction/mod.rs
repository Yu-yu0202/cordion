/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL was
 * not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Interaction models.
//!
//! [`Interaction`] models the payload Discord sends for an interaction and
//! [`InteractionResponse`] models the callback an application sends back.
//! Interaction data is selected by the interaction type: application command
//! data, message component data, or modal submit data. Unknown data shapes and
//! metadata for future interaction types are retained as JSON values.
//! [`InteractionCallbackResponse`] models what Discord returns when a callback
//! is acknowledged with `with_response=true`.
//!
//! See [Discord's receiving and responding documentation][reference] for
//! details.
//!
//! [reference]: https://docs.discord.com/developers/interactions/receiving-and-responding

pub mod callback;
mod data;
mod request;
mod response;

pub use callback::{
    InteractionCallbackActivityInstanceResource, InteractionCallbackObject,
    InteractionCallbackResource, InteractionCallbackResponse,
};
use cordion_macro::IdTarget;
pub use data::{
    ApplicationCommandData, ApplicationCommandInteractionDataOption,
    ApplicationCommandInteractionMetadata, AuthorizingIntegrationOwners, CommandOptionValue,
    GuildInstallOwner, InteractionData, InteractionGuild, MessageComponentData,
    MessageComponentInteractionMetadata, MessageInteraction, MessageInteractionMetadata,
    ModalSubmitData, ModalSubmitInteractionMetadata, ResolvedData,
};
pub use request::{CreateFollowupMessageRequest, EditFollowupMessageRequest};
pub use response::{
    InteractionCallbackData, InteractionCallbackMessage, InteractionCallbackType,
    InteractionResponse,
};
use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::channel::{ChannelId, PartialChannel};
use crate::guild::{GuildId, GuildMember};
use crate::id::{Id, Identifiable};
use crate::message::Message;
use crate::monetization::entitlement::Entitlement;
use crate::permission::Permission;
use crate::user::User;

/// The ID of an [`Interaction`].
pub type InteractionId = Id<Interaction>;

/// An interaction sent by Discord.
///
/// Discord's field table marks [`entitlements`](Self::entitlements),
/// [`authorizing_integration_owners`](Self::authorizing_integration_owners),
/// and [`attachment_size_limit`](Self::attachment_size_limit) as required, but
/// documented sample payloads omit them, including for PING interactions.
/// This type therefore treats them as optional or defaulted and does not
/// reject those payloads.
///
/// See [Discord's interaction object][object] for details.
///
/// [object]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object
#[derive(Clone, PartialEq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Interaction {
    /// The interaction's ID.
    pub id: InteractionId,

    /// The ID of the application the interaction is for.
    pub application_id: ApplicationId,

    /// The interaction type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: InteractionType,

    /// The interaction data payload.
    ///
    /// Present for every type except PING; optional to allow future
    /// interaction types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InteractionData>,

    /// The guild the interaction was sent from, when invoked in a guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<InteractionGuild>,

    /// The ID of the guild the interaction was sent from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The channel the interaction was sent from, as a partial channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<PartialChannel>,

    /// The ID of the channel the interaction was sent from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// The guild member data for the invoking user, when invoked in a guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<GuildMember>,

    /// The invoking user, when the interaction was invoked in a DM.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// The continuation token for responding to the interaction.
    pub token: String,

    /// The interaction version. Discord always sends `1`.
    pub version: u16,

    /// The message the component was attached to, for components and modals
    /// triggered by components.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,

    /// The permissions the application has in the source location.
    ///
    /// Absent on PING interactions and omitted by some documented samples.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_permissions: Option<Permission>,

    /// The invoking user's selected language.
    ///
    /// Available on all interaction types except PING.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /// The guild's preferred locale, when invoked in a guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,

    /// Entitlements for the invoking user in monetized applications.
    ///
    /// Absence decodes as an empty list.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entitlements: Vec<Entitlement>,

    /// Installation contexts the interaction was authorized for and their
    /// related user or guild IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorizing_integration_owners: Option<AuthorizingIntegrationOwners>,

    /// The context where the interaction was triggered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<InteractionContextType>,

    /// The attachment size limit in bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_size_limit: Option<u64>,
}

impl Identifiable for Interaction {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The type of an [`Interaction`].
///
/// Unrecognized values are preserved. See [Discord's interaction types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-interaction-type
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InteractionType(u8);

impl InteractionType {
    /// A ping sent to verify an interactions endpoint.
    pub const PING: Self = Self(1);

    /// An application command invocation.
    pub const APPLICATION_COMMAND: Self = Self(2);

    /// A message component interaction.
    pub const MESSAGE_COMPONENT: Self = Self(3);

    /// An application command autocomplete interaction.
    pub const APPLICATION_COMMAND_AUTOCOMPLETE: Self = Self(4);

    /// A modal submission.
    pub const MODAL_SUBMIT: Self = Self(5);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }

    /// Wraps a raw integer value without validating it.
    #[inline]
    #[must_use]
    pub const fn from_raw(value: u8) -> Self {
        Self(value)
    }
}

/// The context in which an interaction can be used or was triggered.
///
/// Unrecognized values are preserved. See [Discord's interaction context
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/interactions/receiving-and-responding#interaction-object-interaction-context-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InteractionContextType(u8);

impl InteractionContextType {
    /// The interaction is used within guilds.
    pub const GUILD: Self = Self(0);

    /// The interaction is used within DMs with the application's bot user.
    pub const BOT_DM: Self = Self(1);

    /// The interaction is used within group DMs and DMs other than with the
    /// application's bot user.
    pub const PRIVATE_CHANNEL: Self = Self(2);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Tag validation for untagged interaction metadata enums.
pub(crate) mod tag {
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer};

    use crate::interaction::InteractionType;

    macro_rules! expect_interaction_type {
        ($name:ident, $value:expr) => {
            pub(super) fn $name<'de, D>(deserializer: D) -> Result<InteractionType, D::Error>
            where
                D: Deserializer<'de>,
            {
                let kind = InteractionType::deserialize(deserializer)?;

                if kind == InteractionType::from_raw($value) {
                    Ok(kind)
                } else {
                    Err(D::Error::custom(format_args!(
                        "expected interaction type {}, found {}",
                        $value,
                        kind.raw()
                    )))
                }
            }
        };
    }

    expect_interaction_type!(application_command, 2);
    expect_interaction_type!(message_component, 3);
    expect_interaction_type!(modal_submit, 5);
}
