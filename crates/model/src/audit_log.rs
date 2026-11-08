/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord audit log resource.
//!
//! Audit log entries describe administrative actions in a guild. Entries are
//! stored for 45 days and require the `VIEW_AUDIT_LOG` permission to fetch.
//!
//! The `changes` array in an entry has no fixed schema: each change corresponds
//! to a field of the changed object, so values are exposed as raw
//! [`serde_json::Value`]s.
//!
//! See [Discord's audit log resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/audit-log

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::application::ApplicationId;
use crate::application_command::ApplicationCommand;
use crate::auto_moderation::AutoModerationRule;
use crate::channel::{Channel, ChannelId};
use crate::guild::{GuildScheduledEvent, PartialIntegration};
use crate::id::{Id, Identifiable, snowflake_option};
use crate::message::MessageId;
use crate::user::{User, UserId};
use crate::webhook::Webhook;

/// The ID of an [`AuditLogEntry`].
pub type AuditLogEntryId = Id<AuditLogEntry>;

/// An audit log for a guild.
///
/// Every collection is documented as always present. See [Discord's audit log
/// object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/audit-log#audit-log-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AuditLog {
    /// The application commands referenced in the audit log.
    pub application_commands: Vec<ApplicationCommand>,

    /// The audit log entries, sorted from most to least recent.
    pub audit_log_entries: Vec<AuditLogEntry>,

    /// The Auto Moderation rules referenced in the audit log.
    pub auto_moderation_rules: Vec<AutoModerationRule>,

    /// The guild scheduled events referenced in the audit log.
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,

    /// The partial integrations referenced in the audit log.
    pub integrations: Vec<PartialIntegration>,

    /// The threads referenced in the audit log.
    ///
    /// Threads are included because archived threads might not be retained by
    /// clients.
    pub threads: Vec<Channel>,

    /// The users referenced in the audit log.
    pub users: Vec<User>,

    /// The webhooks referenced in the audit log.
    pub webhooks: Vec<Webhook>,
}

/// One administrative action recorded in an [`AuditLog`].
///
/// [`target_id`](Self::target_id) is a nullable string because the affected
/// entity varies by event and is not always a snowflake. [`changes`](Self::changes)
/// is optional, and [`options`](Self::options) is only present for certain
/// events.
///
/// See [Discord's audit log entry object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/audit-log#audit-log-entry-object
#[derive(Clone, PartialEq, IdTarget, Serialize, Deserialize, Debug)]
pub struct AuditLogEntry {
    /// The ID of the affected entity, such as a webhook, user, or role.
    ///
    /// This is a string because the entity type depends on the event. It is
    /// nullable and is serialized as `null` when `None`.
    pub target_id: Option<String>,

    /// The changes made to the target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changes: Option<Vec<AuditLogChange>>,

    /// The ID of the user or application that made the changes.
    ///
    /// This is nullable and is serialized as `null` when `None`.
    pub user_id: Option<UserId>,

    /// The audit log entry's ID.
    pub id: AuditLogEntryId,

    /// The type of action that occurred.
    pub action_type: AuditLogEvent,

    /// Additional information for certain event types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<OptionalAuditEntryInfo>,

    /// The reason for the change, between 1 and 512 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl Identifiable for AuditLogEntry {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// One field change within an [`AuditLogEntry`].
///
/// The shape of the values depends on the changed object. An absent `new_value`
/// or `old_value` indicates that the property was reset or was previously
/// `null`, respectively. Explicit `null` values are decoded as `None` and are
/// therefore indistinguishable from absent fields.
///
/// See [Discord's audit log change object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/audit-log#audit-log-change-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AuditLogChange {
    /// The new value of the changed field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<Value>,

    /// The previous value of the changed field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<Value>,

    /// The name of the changed field.
    pub key: String,
}

/// Additional information attached to certain [`AuditLogEntry`] events.
///
/// Every field is optional because Discord only includes the fields relevant to
/// the entry's action type. See [Discord's optional audit entry info][info] for
/// details.
///
/// [info]: https://docs.discord.com/developers/resources/audit-log#audit-log-entry-object-optional-audit-entry-info
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct OptionalAuditEntryInfo {
    /// The ID of the application whose permissions were targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    /// The name of the Auto Moderation rule that was triggered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_name: Option<String>,

    /// The trigger type of the Auto Moderation rule that was triggered.
    ///
    /// Discord sends this value as a string even though the corresponding
    /// trigger type is numeric elsewhere.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_trigger_type: Option<String>,

    /// The channel in which the targeted entities were located.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// The number of targeted entities or position updates.
    ///
    /// Discord sends this value as a string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,

    /// The number of days after which inactive members were kicked.
    ///
    /// Discord sends this value as a string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_member_days: Option<String>,

    /// The ID of the overwritten role or member.
    ///
    /// The value can refer to either a role or a member, so it is exposed as a
    /// raw snowflake `u64`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "snowflake_option"
    )]
    pub id: Option<u64>,

    /// The number of members removed by the prune.
    ///
    /// Discord sends this value as a string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<String>,

    /// The ID of the message that was targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,

    /// The name of the overwritten role, when the overwrite targets a role.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,

    /// The type of overwritten entity, serialized as `type`.
    ///
    /// Discord sends `"0"` for roles and `"1"` for members.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// The type of integration that performed the action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,

    /// The new voice channel status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// The type of action recorded by an [`AuditLogEntry`].
///
/// Unrecognized values are preserved. See [Discord's audit log events][events]
/// for details.
///
/// [events]: https://docs.discord.com/developers/resources/audit-log#audit-log-entry-object-audit-log-events
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct AuditLogEvent(u16);

impl AuditLogEvent {
    /// Server settings were updated.
    pub const GUILD_UPDATE: Self = Self(1);

    /// A channel was created.
    pub const CHANNEL_CREATE: Self = Self(10);

    /// Channel settings were updated.
    pub const CHANNEL_UPDATE: Self = Self(11);

    /// A channel was deleted.
    pub const CHANNEL_DELETE: Self = Self(12);

    /// A permission overwrite was added to a channel.
    pub const CHANNEL_OVERWRITE_CREATE: Self = Self(13);

    /// A permission overwrite was updated for a channel.
    pub const CHANNEL_OVERWRITE_UPDATE: Self = Self(14);

    /// A permission overwrite was deleted from a channel.
    pub const CHANNEL_OVERWRITE_DELETE: Self = Self(15);

    /// Channels were reordered.
    pub const CHANNEL_POSITION_UPDATE: Self = Self(16);

    /// A member was removed from the guild.
    pub const MEMBER_KICK: Self = Self(20);

    /// Members were pruned from the guild.
    pub const MEMBER_PRUNE: Self = Self(21);

    /// A member was banned from the guild.
    pub const MEMBER_BAN_ADD: Self = Self(22);

    /// A ban was lifted for a member.
    pub const MEMBER_BAN_REMOVE: Self = Self(23);

    /// A member was updated in the guild.
    pub const MEMBER_UPDATE: Self = Self(24);

    /// A member was added to or removed from a role.
    pub const MEMBER_ROLE_UPDATE: Self = Self(25);

    /// A member was moved to a different voice channel.
    pub const MEMBER_MOVE: Self = Self(26);

    /// A member was disconnected from a voice channel.
    pub const MEMBER_DISCONNECT: Self = Self(27);

    /// A bot user was added to the guild.
    pub const BOT_ADD: Self = Self(28);

    /// A role was created.
    pub const ROLE_CREATE: Self = Self(30);

    /// A role was edited.
    pub const ROLE_UPDATE: Self = Self(31);

    /// A role was deleted.
    pub const ROLE_DELETE: Self = Self(32);

    /// Roles were reordered.
    pub const ROLE_POSITION_UPDATE: Self = Self(33);

    /// A guild invite was created.
    pub const INVITE_CREATE: Self = Self(40);

    /// A guild invite was updated.
    pub const INVITE_UPDATE: Self = Self(41);

    /// A guild invite was deleted.
    pub const INVITE_DELETE: Self = Self(42);

    /// A webhook was created.
    pub const WEBHOOK_CREATE: Self = Self(50);

    /// Webhook properties or its channel were updated.
    pub const WEBHOOK_UPDATE: Self = Self(51);

    /// A webhook was deleted.
    pub const WEBHOOK_DELETE: Self = Self(52);

    /// An emoji was created.
    pub const EMOJI_CREATE: Self = Self(60);

    /// An emoji's name was updated.
    pub const EMOJI_UPDATE: Self = Self(61);

    /// An emoji was deleted.
    pub const EMOJI_DELETE: Self = Self(62);

    /// A single message was deleted.
    pub const MESSAGE_DELETE: Self = Self(72);

    /// Multiple messages were deleted.
    pub const MESSAGE_BULK_DELETE: Self = Self(73);

    /// A message was pinned to a channel.
    pub const MESSAGE_PIN: Self = Self(74);

    /// A message was unpinned from a channel.
    pub const MESSAGE_UNPIN: Self = Self(75);

    /// An application was added to the guild.
    pub const INTEGRATION_CREATE: Self = Self(80);

    /// An application integration was updated.
    pub const INTEGRATION_UPDATE: Self = Self(81);

    /// An application was removed from the guild.
    pub const INTEGRATION_DELETE: Self = Self(82);

    /// A stage instance was created.
    pub const STAGE_INSTANCE_CREATE: Self = Self(83);

    /// A stage instance was updated.
    pub const STAGE_INSTANCE_UPDATE: Self = Self(84);

    /// A stage instance was deleted.
    pub const STAGE_INSTANCE_DELETE: Self = Self(85);

    /// A sticker was created.
    pub const STICKER_CREATE: Self = Self(90);

    /// Sticker details were updated.
    pub const STICKER_UPDATE: Self = Self(91);

    /// A sticker was deleted.
    pub const STICKER_DELETE: Self = Self(92);

    /// A guild scheduled event was created.
    pub const GUILD_SCHEDULED_EVENT_CREATE: Self = Self(100);

    /// A guild scheduled event was updated.
    pub const GUILD_SCHEDULED_EVENT_UPDATE: Self = Self(101);

    /// A guild scheduled event was cancelled.
    pub const GUILD_SCHEDULED_EVENT_DELETE: Self = Self(102);

    /// A thread was created in a channel.
    pub const THREAD_CREATE: Self = Self(110);

    /// A thread was updated.
    pub const THREAD_UPDATE: Self = Self(111);

    /// A thread was deleted.
    pub const THREAD_DELETE: Self = Self(112);

    /// Command permissions were updated.
    pub const APPLICATION_COMMAND_PERMISSION_UPDATE: Self = Self(121);

    /// A soundboard sound was created.
    pub const SOUNDBOARD_SOUND_CREATE: Self = Self(130);

    /// A soundboard sound was updated.
    pub const SOUNDBOARD_SOUND_UPDATE: Self = Self(131);

    /// A soundboard sound was deleted.
    pub const SOUNDBOARD_SOUND_DELETE: Self = Self(132);

    /// An Auto Moderation rule was created.
    pub const AUTO_MODERATION_RULE_CREATE: Self = Self(140);

    /// An Auto Moderation rule was updated.
    pub const AUTO_MODERATION_RULE_UPDATE: Self = Self(141);

    /// An Auto Moderation rule was deleted.
    pub const AUTO_MODERATION_RULE_DELETE: Self = Self(142);

    /// A message was blocked by Auto Moderation.
    pub const AUTO_MODERATION_BLOCK_MESSAGE: Self = Self(143);

    /// A message was flagged by Auto Moderation.
    pub const AUTO_MODERATION_FLAG_TO_CHANNEL: Self = Self(144);

    /// A member was timed out by Auto Moderation.
    pub const AUTO_MODERATION_USER_COMMUNICATION_DISABLED: Self = Self(145);

    /// A member was quarantined by Auto Moderation.
    pub const AUTO_MODERATION_QUARANTINE_USER: Self = Self(146);

    /// A creator monetization request was created.
    pub const CREATOR_MONETIZATION_REQUEST_CREATED: Self = Self(150);

    /// Creator monetization terms were accepted.
    pub const CREATOR_MONETIZATION_TERMS_ACCEPTED: Self = Self(151);

    /// A guild onboarding prompt was created.
    pub const ONBOARDING_PROMPT_CREATE: Self = Self(163);

    /// A guild onboarding prompt was updated.
    pub const ONBOARDING_PROMPT_UPDATE: Self = Self(164);

    /// A guild onboarding prompt was deleted.
    pub const ONBOARDING_PROMPT_DELETE: Self = Self(165);

    /// Guild onboarding was created.
    pub const ONBOARDING_CREATE: Self = Self(166);

    /// Guild onboarding was updated.
    pub const ONBOARDING_UPDATE: Self = Self(167);

    /// The guild's Server Guide was created.
    pub const HOME_SETTINGS_CREATE: Self = Self(190);

    /// The guild's Server Guide was updated.
    pub const HOME_SETTINGS_UPDATE: Self = Self(191);

    /// A voice channel status was set.
    pub const VOICE_CHANNEL_STATUS_CREATE: Self = Self(192);

    /// A voice channel status was deleted.
    pub const VOICE_CHANNEL_STATUS_DELETE: Self = Self(193);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u16 {
        self.0
    }
}
