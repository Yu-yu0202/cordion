/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Auto Moderation models.
//!
//! Auto Moderation rules combine a trigger, which decides whether a rule
//! applies, with a list of actions to execute. Trigger and action metadata are
//! type-dependent: Discord only sends the fields relevant to the corresponding
//! trigger or action type, so every metadata field in this module is optional.
//!
//! All endpoints that access these resources require the `MANAGE_GUILD`
//! permission. The `TIMEOUT` action additionally requires
//! `MODERATE_MEMBERS`.
//!
//! The endpoint bodies in this module are [`CreateAutoModerationRuleRequest`]
//! and [`ModifyAutoModerationRuleRequest`].
//!
//! See [Discord's Auto Moderation resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/auto-moderation

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::guild::GuildId;
use crate::id::{Id, Identifiable};
use crate::role::RoleId;
use crate::user::UserId;

/// The ID of an [`AutoModerationRule`].
pub type AutoModerationRuleId = Id<AutoModerationRule>;

/// The event context in which a rule is checked.
///
/// Unrecognized values are preserved. See [Discord's event types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-rule-object-event-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AutoModerationEventType(u8);

impl AutoModerationEventType {
    /// Check when a member sends or edits a message in the guild.
    pub const MESSAGE_SEND: Self = Self(1);

    /// Check when a member edits their profile.
    pub const MEMBER_UPDATE: Self = Self(2);
}

/// The type of content that can trigger a rule.
///
/// Unrecognized values are preserved. See [Discord's trigger types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-rule-object-trigger-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AutoModerationTriggerType(u8);

impl AutoModerationTriggerType {
    /// Check whether content contains a user-defined keyword.
    pub const KEYWORD: Self = Self(1);

    /// Check whether content represents generic spam.
    pub const SPAM: Self = Self(3);

    /// Check whether content contains a predefined keyword set.
    pub const KEYWORD_PRESET: Self = Self(4);

    /// Check whether content contains more unique mentions than allowed.
    pub const MENTION_SPAM: Self = Self(5);

    /// Check whether a member profile contains a user-defined keyword.
    pub const MEMBER_PROFILE: Self = Self(6);
}

/// The action executed when a rule is triggered.
///
/// Unrecognized values are preserved. See [Discord's action types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-action-object-action-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AutoModerationActionType(u8);

impl AutoModerationActionType {
    /// Block the member's message.
    pub const BLOCK_MESSAGE: Self = Self(1);

    /// Log the user content to a specified channel.
    pub const SEND_ALERT_MESSAGE: Self = Self(2);

    /// Time out the member.
    ///
    /// Only valid for `KEYWORD` and `MENTION_SPAM` rules, and requires the
    /// `MODERATE_MEMBERS` permission.
    pub const TIMEOUT: Self = Self(3);

    /// Prevent the member from interacting with the guild.
    pub const BLOCK_MEMBER_INTERACTION: Self = Self(4);
}

/// A predefined keyword set for a `KEYWORD_PRESET` rule.
///
/// Unrecognized values are preserved. See [Discord's keyword preset types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-rule-object-keyword-preset-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct KeywordPresetType(u8);

impl KeywordPresetType {
    /// Words that may be considered forms of swearing or cursing.
    pub const PROFANITY: Self = Self(1);

    /// Words that refer to sexually explicit behavior or activity.
    pub const SEXUAL_CONTENT: Self = Self(2);

    /// Personal insults or words that may be considered hate speech.
    pub const SLURS: Self = Self(3);
}

/// Additional data used to determine whether a rule triggers.
///
/// Different fields are relevant depending on the rule's
/// [`trigger_type`](AutoModerationRule::trigger_type). Discord only includes
/// the applicable fields, so all fields are optional and omitted during
/// serialization when `None`.
///
/// See [Discord's trigger metadata object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct AutoModerationTriggerMetadata {
    /// Substrings to search for in content.
    ///
    /// Relevant for `KEYWORD` and `MEMBER_PROFILE` rules. A keyword may contain
    /// wildcards and must be 60 characters or less.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword_filter: Option<Vec<String>>,

    /// Rust-flavored regular expressions to match against content.
    ///
    /// Relevant for `KEYWORD` and `MEMBER_PROFILE` rules. Each pattern must be
    /// 260 characters or less.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_patterns: Option<Vec<String>>,

    /// Predefined keyword sets to search for.
    ///
    /// Relevant for `KEYWORD_PRESET` rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<KeywordPresetType>>,

    /// Substrings that should not trigger the rule.
    ///
    /// Relevant for `KEYWORD`, `KEYWORD_PRESET`, and `MEMBER_PROFILE` rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list: Option<Vec<String>>,

    /// The total number of unique role and user mentions allowed per message.
    ///
    /// Relevant for `MENTION_SPAM` rules, with a maximum of 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_total_limit: Option<u64>,

    /// Whether to automatically detect mention raids.
    ///
    /// Relevant for `MENTION_SPAM` rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_raid_protection_enabled: Option<bool>,
}

/// One action executed when an [`AutoModerationRule`] triggers.
///
/// See [Discord's action object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-action-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AutoModerationAction {
    /// The action's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: AutoModerationActionType,

    /// Additional metadata required for this action type.
    ///
    /// Can be omitted depending on the action type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AutoModerationActionMetadata>,
}

/// Additional data used when an [`AutoModerationAction`] executes.
///
/// Different fields are relevant depending on the action's
/// [`kind`](AutoModerationAction::kind). Discord only includes the applicable
/// fields, so all fields are optional and omitted during serialization when
/// `None`.
///
/// See [Discord's action metadata object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-action-object-action-metadata
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct AutoModerationActionMetadata {
    /// The channel where user content is logged.
    ///
    /// Relevant for `SEND_ALERT_MESSAGE` actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// The timeout duration in seconds, up to 2,419,200 (four weeks).
    ///
    /// Relevant for `TIMEOUT` actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u64>,

    /// An additional explanation shown when a message is blocked.
    ///
    /// Relevant for `BLOCK_MESSAGE` actions, with a maximum of 150 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

/// A guild's Auto Moderation rule.
///
/// All fields are present in the rule object returned by Discord. The fields
/// inside [`trigger_metadata`](Self::trigger_metadata) and action metadata are
/// type-dependent; see those types for details.
///
/// See [Discord's Auto Moderation rule object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/auto-moderation#auto-moderation-rule-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct AutoModerationRule {
    /// The rule's ID.
    pub id: AutoModerationRuleId,

    /// The guild this rule belongs to.
    pub guild_id: GuildId,

    /// The rule's name.
    pub name: String,

    /// The user who first created this rule.
    pub creator_id: UserId,

    /// The event type that triggers this rule.
    pub event_type: AutoModerationEventType,

    /// The type of content that triggers this rule.
    pub trigger_type: AutoModerationTriggerType,

    /// Additional trigger metadata for this rule.
    pub trigger_metadata: AutoModerationTriggerMetadata,

    /// The actions executed when this rule triggers.
    pub actions: Vec<AutoModerationAction>,

    /// Whether this rule is enabled.
    pub enabled: bool,

    /// Role IDs that should not be affected by this rule, up to 20.
    pub exempt_roles: Vec<RoleId>,

    /// Channel IDs that should not be affected by this rule, up to 50.
    pub exempt_channels: Vec<ChannelId>,
}

impl Identifiable for AutoModerationRule {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The body of `POST /guilds/{guild.id}/auto-moderation/rules`.
///
/// [`name`](Self::name), [`event_type`](Self::event_type),
/// [`trigger_type`](Self::trigger_type), and [`actions`](Self::actions) are
/// required by Discord. The remaining fields are optional and are omitted
/// during serialization when `None`; [`enabled`](Self::enabled) defaults to
/// `false` when omitted.
///
/// See [Discord's create auto moderation rule endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/auto-moderation#create-auto-moderation-rule
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateAutoModerationRuleRequest {
    /// The rule's name.
    pub name: String,

    /// The event type that triggers this rule.
    pub event_type: AutoModerationEventType,

    /// The type of content that triggers this rule.
    pub trigger_type: AutoModerationTriggerType,

    /// Additional trigger metadata for this rule.
    ///
    /// Discord documents this field as optional and omittable based on
    /// [`trigger_type`](Self::trigger_type).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_metadata: Option<AutoModerationTriggerMetadata>,

    /// The actions executed when this rule triggers.
    pub actions: Vec<AutoModerationAction>,

    /// Whether this rule is enabled.
    ///
    /// Discord documents `false` as the default when this field is omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The role IDs that should not be affected by this rule, at most 20.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exempt_roles: Option<Vec<RoleId>>,

    /// The channel IDs that should not be affected by this rule, at most 50.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exempt_channels: Option<Vec<ChannelId>>,
}

/// The body of `PATCH /guilds/{guild.id}/auto-moderation/rules/{rule.id}`.
///
/// Discord documents every parameter of this endpoint as optional, so every
/// field is omitted during serialization when `None` and an omitted field
/// leaves the rule unchanged. Discord does not accept
/// [`trigger_type`](AutoModerationRule::trigger_type) on this endpoint; a rule's
/// trigger type cannot be changed.
///
/// See [Discord's modify auto moderation rule endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/auto-moderation#modify-auto-moderation-rule
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyAutoModerationRuleRequest {
    /// The rule's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The event type that triggers this rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<AutoModerationEventType>,

    /// Additional trigger metadata for this rule.
    ///
    /// Discord documents this field as optional and omittable based on the
    /// rule's existing [`trigger_type`](AutoModerationRule::trigger_type).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_metadata: Option<AutoModerationTriggerMetadata>,

    /// The actions executed when this rule triggers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<AutoModerationAction>>,

    /// Whether this rule is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The role IDs that should not be affected by this rule, at most 20.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exempt_roles: Option<Vec<RoleId>>,

    /// The channel IDs that should not be affected by this rule, at most 50.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exempt_channels: Option<Vec<ChannelId>>,
}
