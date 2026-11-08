/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::emoji::Emoji;
use crate::guild::GuildId;
use crate::id::Id;
use crate::role::RoleId;

/// A guild's onboarding flow.
///
/// See [Discord's guild onboarding object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#guild-onboarding-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildOnboarding {
    /// The ID of the guild this onboarding belongs to.
    pub guild_id: GuildId,

    /// Prompts shown during onboarding and in the customize community view.
    pub prompts: Vec<OnboardingPrompt>,

    /// Channel IDs that members are opted into automatically.
    pub default_channel_ids: Vec<ChannelId>,

    /// Whether onboarding is enabled in the guild.
    pub enabled: bool,

    /// The current onboarding mode.
    ///
    /// Discord's field table marks this as required, but its own example omits
    /// the field. This model accepts either shape and omits it during
    /// serialization when absent. Unknown values are preserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<OnboardingMode>,
}

/// The criteria used to satisfy onboarding constraints.
///
/// Unrecognized values are preserved. See [Discord's onboarding mode][mode] for
/// details.
///
/// [mode]: https://docs.discord.com/developers/resources/guild#guild-onboarding-object-onboarding-mode
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct OnboardingMode(u8);

impl OnboardingMode {
    /// Counts only default channels towards constraints (Discord's
    /// `ONBOARDING_DEFAULT`).
    pub const ONBOARDING_DEFAULT: Self = Self(0);

    /// Counts default channels and questions towards constraints (Discord's
    /// `ONBOARDING_ADVANCED`).
    pub const ONBOARDING_ADVANCED: Self = Self(1);
}

/// The interaction style of an [`OnboardingPrompt`].
///
/// Unrecognized values are preserved. See [Discord's prompt types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/resources/guild#guild-onboarding-object-prompt-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct OnboardingPromptType(u8);

impl OnboardingPromptType {
    /// Members may select multiple options.
    pub const MULTIPLE_CHOICE: Self = Self(0);

    /// Members select one option from a dropdown.
    pub const DROPDOWN: Self = Self(1);
}

/// One prompt in a [`GuildOnboarding`].
///
/// See [Discord's onboarding prompt object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#guild-onboarding-object-onboarding-prompt-structure
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct OnboardingPrompt {
    /// The prompt's ID.
    pub id: Id<Self>,

    /// The prompt's interaction style.
    #[serde(rename = "type")]
    pub kind: OnboardingPromptType,

    /// Options available within the prompt.
    pub options: Vec<OnboardingPromptOption>,

    /// The prompt's title.
    pub title: String,

    /// Whether members are limited to selecting one option.
    pub single_select: bool,

    /// Whether the prompt is required before onboarding completes.
    pub required: bool,

    /// Whether the prompt appears in the onboarding flow.
    ///
    /// When `false`, the prompt only appears in the Channels & Roles tab.
    pub in_onboarding: bool,
}

/// One option within an [`OnboardingPrompt`].
///
/// Discord's field table marks [`emoji`](Self::emoji), `emoji_id`,
/// `emoji_name`, and `emoji_animated` as optional. Responses may include an
/// emoji object or the individual emoji fields; create and update requests are
/// documented to require the individual fields instead of the object.
///
/// See [Discord's prompt option object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#guild-onboarding-object-prompt-option-structure
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct OnboardingPromptOption {
    /// The option's ID.
    pub id: Id<Self>,

    /// Channel IDs that a member is added to when this option is selected.
    pub channel_ids: Vec<ChannelId>,

    /// Role IDs assigned to a member when this option is selected.
    pub role_ids: Vec<RoleId>,

    /// The option's emoji object, when included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Emoji>,

    /// The ID of the option's emoji, when included separately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<Id<Emoji>>,

    /// The name of the option's emoji, when included separately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,

    /// Whether the option's emoji is animated, when included separately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_animated: Option<bool>,

    /// The option's title.
    pub title: String,

    /// The option's description, or `None` if no description is set.
    ///
    /// Discord sends `null` when the option has no description; this field is
    /// serialized as `null` when `None`.
    pub description: Option<String>,
}

/// The body of `PUT /guilds/{guild.id}/onboarding`.
///
/// Discord documents every parameter of this endpoint as optional, so every
/// field is omitted during serialization when `None` and an omitted field
/// leaves the onboarding configuration unchanged.
///
/// See [Discord's modify guild onboarding endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-onboarding
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildOnboardingRequest {
    /// The prompts shown during onboarding and in the customize community view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<OnboardingPrompt>>,

    /// The channel IDs that members are opted into automatically.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_channel_ids: Option<Vec<ChannelId>>,

    /// Whether onboarding is enabled in the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The current onboarding mode.
    ///
    /// Discord's field table marks this as required, but its own example omits
    /// the field; unknown values are preserved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<OnboardingMode>,
}
