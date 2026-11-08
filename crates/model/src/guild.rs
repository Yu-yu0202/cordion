/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord guild resource.
//!
//! [`Guild`] models the full guild object returned by endpoints such as
//! `GET /guilds/{guild.id}`. Partial payloads use dedicated types:
//! [`PartialGuild`] covers `GET /users/@me/guilds`, [`InviteGuild`] covers the
//! guild embedded in an invite, and
//! [`GuildCreate`](crate::event::gateway::GuildCreate) covers the extra fields sent in
//! the `GUILD_CREATE` event. Guild previews, widgets, onboarding, and templates
//! have their own types in this module, and [`Ban`] models the objects returned
//! by the guild ban endpoints.
//!
//! Request bodies of the guild endpoints are modeled across this module and its
//! submodules. Bodies for a resource that has its own submodule are defined
//! there and re-exported from this module, such as
//! [`CreateGuildRoleRequest`] and [`ModifyGuildMemberRequest`]; the bodies that
//! model the guild module itself, such as [`ModifyGuildRequest`], are defined
//! here.
//!
//! See [Discord's guild resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/guild

mod ban;
mod feature;
mod incidents_data;
mod integration;
mod invite;
mod levels;
mod member;
mod onboarding;
mod partial;
mod preview;
mod prune;
mod role;
mod scheduled_event;
mod soundboard;
mod stage_instance;
mod system_channel;
mod template;
mod welcome_screen;
mod widget;

pub use ban::{Ban, BulkGuildBanRequest, BulkGuildBanResponse, CreateGuildBanRequest};
use cordion_macro::IdTarget;
pub use feature::GuildFeature;
pub use incidents_data::{IncidentsData, ModifyGuildIncidentActionsRequest};
pub use integration::{
    Integration, IntegrationAccount, IntegrationApplication, IntegrationExpireBehavior,
    IntegrationId, IntegrationType, PartialIntegration,
};
pub use invite::InviteGuild;
pub use levels::{
    DefaultMessageNotificationLevel, ExplicitContentFilterLevel, GuildAgeRestrictionLevel,
    MfaLevel, PremiumTier, VerificationLevel,
};
pub use member::{
    AddGuildMemberRequest, GuildMember, GuildMemberFlags, ModifyCurrentMemberRequest,
    ModifyGuildMemberRequest, PartialGuildMember,
};
pub use onboarding::{
    GuildOnboarding, ModifyGuildOnboardingRequest, OnboardingMode, OnboardingPrompt,
    OnboardingPromptOption, OnboardingPromptType,
};
pub use partial::PartialGuild;
pub use preview::GuildPreview;
pub use prune::{BeginGuildPruneRequest, GuildPruneCount};
pub use role::{CreateGuildRoleRequest, ModifyGuildRolePosition, ModifyGuildRoleRequest};
pub use scheduled_event::{
    CreateGuildScheduledEventRequest, GuildScheduledEvent, GuildScheduledEventEntity,
    GuildScheduledEventEntityId, GuildScheduledEventEntityMetadata, GuildScheduledEventEntityType,
    GuildScheduledEventId, GuildScheduledEventPrivacyLevel, GuildScheduledEventRecurrenceFrequency,
    GuildScheduledEventRecurrenceMonth, GuildScheduledEventRecurrenceNWeekday,
    GuildScheduledEventRecurrenceRule, GuildScheduledEventRecurrenceWeekday,
    GuildScheduledEventStatus, GuildScheduledEventUser, ModifyGuildScheduledEventRequest,
};
use serde::{Deserialize, Serialize};
pub use soundboard::{
    CreateGuildSoundboardSoundRequest, ModifyGuildSoundboardSoundRequest,
    SendSoundboardSoundRequest, SoundboardSound, SoundboardSoundId,
};
pub use stage_instance::{
    CreateStageInstanceRequest, ModifyStageInstanceRequest, StageInstance, StageInstanceId,
    StagePrivacyLevel,
};
pub use system_channel::SystemChannelFlags;
pub use template::{
    CreateGuildTemplateRequest, GuildTemplate, ModifyGuildTemplateRequest, TemplateChannel,
    TemplateGuild, TemplateOverwrite, TemplateRole,
};
pub use welcome_screen::{ModifyGuildWelcomeScreenRequest, WelcomeScreen, WelcomeScreenChannel};
pub use widget::{GuildWidget, GuildWidgetMember, GuildWidgetSettings, ModifyGuildWidgetRequest};

use crate::application::ApplicationId;
use crate::channel::{Channel, ChannelId, ThreadMember};
use crate::emoji::Emoji;
use crate::id::{Id, Identifiable};
use crate::permission::Permission;
use crate::role::Role;
use crate::sticker::Sticker;
use crate::user::User;

/// The ID of a [`Guild`].
pub type GuildId = Id<Guild>;

/// A guild (server) as returned by the Discord API.
///
/// Fields that Discord may send as `null` are represented as `Option`; required
/// nullable fields serialize as `null` when `None`, while optional fields are
/// omitted. Each field documents its availability.
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Guild {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's name, between 2 and 100 characters.
    pub name: String,

    /// The guild's icon hash, or `None` if it has no icon.
    pub icon: Option<String>,

    /// The guild's icon hash, returned when the guild is a template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<String>,

    /// The guild's splash hash, or `None` if it has no splash.
    pub splash: Option<String>,

    /// The guild's discovery splash hash.
    ///
    /// Only present for guilds with the `DISCOVERABLE` feature.
    pub discovery_splash: Option<String>,

    /// Whether the requesting user owns the guild.
    ///
    /// Only present in partial guild payloads; see [`PartialGuild`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,

    /// The ID of the guild owner.
    pub owner_id: Id<User>,

    /// The requesting user's total permissions in the guild.
    ///
    /// This excludes channel overwrites and implicit permissions. Only present
    /// in partial guild payloads; see [`PartialGuild`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,

    /// The guild's voice region ID.
    ///
    /// Deprecated by Discord; use `channel.rtc_region` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Deprecated by Discord API. Use `channel.rtc_region` instead.")]
    pub region: Option<String>,

    /// The ID of the guild's AFK voice channel, or `None` if none is set.
    pub afk_channel_id: Option<ChannelId>,

    /// The AFK timeout in seconds.
    ///
    /// Discord accepts `60`, `300`, `900`, `1800`, and `3600`.
    pub afk_timeout: u64,

    /// Whether the guild's widget is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>,

    /// The channel whose invite the widget uses, or `None` for no invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<ChannelId>,

    /// The verification level required to join the guild.
    pub verification_level: VerificationLevel,

    /// The default message notification level.
    pub default_message_notifications: DefaultMessageNotificationLevel,

    /// The explicit content filter level.
    pub explicit_content_filter: ExplicitContentFilterLevel,

    /// The guild's roles.
    pub roles: Vec<Role>,

    /// The guild's custom emoji.
    pub emojis: Vec<Emoji>,

    /// Enabled guild features.
    ///
    /// Unknown feature strings are preserved by [`GuildFeature`].
    pub features: Vec<GuildFeature>,

    /// The MFA (Multi-Factor Authentication) level required for moderation
    /// actions.
    pub mfa_level: MfaLevel,

    /// The application ID of the guild creator, if the guild was bot-created.
    pub application_id: Option<ApplicationId>,

    /// The channel where guild notices such as welcome messages and boost
    /// events are posted.
    pub system_channel_id: Option<ChannelId>,

    /// Notification flags for the system channel.
    pub system_channel_flags: SystemChannelFlags,

    /// The channel where Community guilds display rules and guidelines.
    pub rules_channel_id: Option<ChannelId>,

    /// The maximum number of presences for the guild.
    ///
    /// Discord returns `null` for all but the largest guilds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u64>,

    /// The maximum number of members for the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u64>,

    /// The guild's vanity URL code, or `None` if it has none.
    pub vanity_url_code: Option<String>,

    /// The guild's description, or `None` if it has none.
    pub description: Option<String>,

    /// The guild's banner hash, or `None` if it has no banner.
    pub banner: Option<String>,

    /// The guild's Server Boost tier.
    pub premium_tier: PremiumTier,

    /// The number of boosts the guild currently has.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u64>,

    /// The preferred locale of a Community guild.
    ///
    /// Defaults to `"en-US"` for guilds that are not Community guilds.
    pub preferred_locale: String,

    /// The channel where Community guild administrators and moderators receive
    /// notices from Discord.
    pub public_updates_channel_id: Option<ChannelId>,

    /// The maximum number of users in a video channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u64>,

    /// The maximum number of users in a stage video channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stage_video_channel_users: Option<u64>,

    /// The approximate number of members in the guild.
    ///
    /// Returned by `GET /guilds/{guild.id}` and `GET /users/@me/guilds` when
    /// `with_counts` is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,

    /// The approximate number of non-offline members in the guild.
    ///
    /// Returned by `GET /guilds/{guild.id}` and `GET /users/@me/guilds` when
    /// `with_counts` is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,

    /// The welcome screen of a Community guild.
    ///
    /// This may also appear in the guild object embedded in an invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_screen: Option<WelcomeScreen>,

    /// The guild's age-restriction level.
    pub nsfw_level: GuildAgeRestrictionLevel,

    /// The guild's custom stickers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stickers: Option<Vec<Sticker>>,

    /// Whether the guild has the boost progress bar enabled.
    pub premium_progress_bar_enabled: bool,

    /// The channel where Community guild administrators and moderators receive
    /// safety alerts from Discord.
    pub safety_alerts_channel_id: Option<ChannelId>,

    /// Incident data for the guild.
    ///
    /// Discord may send `null` or omit this field; both decode as `None`.
    /// Serialization emits `null` when this is `None`.
    pub incidents_data: Option<IncidentsData>,
}

impl Identifiable for Guild {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The body of `PATCH /guilds/{guild.id}`.
///
/// Discord documents every parameter of this endpoint as optional, so every
/// field is omitted during serialization when `None` and an omitted field
/// leaves the guild unchanged. Because omission means "leave unchanged", the
/// fields Discord documents as nullable cannot be cleared by sending `null`
/// through this type.
///
/// [`icon`](Self::icon), [`splash`](Self::splash),
/// [`discovery_splash`](Self::discovery_splash), and [`banner`](Self::banner)
/// carry image data as a [Data URI scheme] string, for example
/// `data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA`. Discord's reference
/// names `image/jpeg`, `image/png`, and `image/gif` as the supported content
/// types.
///
/// See [Discord's modify guild endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild
/// [Data URI scheme]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildRequest {
    /// The guild's name, between 2 and 100 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The guild's voice region ID.
    ///
    /// Discord deprecated this field in favor of `channel.rtc_region`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// The verification level required to join the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,

    /// The default message notification level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<DefaultMessageNotificationLevel>,

    /// The explicit content filter level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,

    /// The ID of the AFK channel, or `None` to leave it unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<ChannelId>,

    /// The AFK timeout in seconds.
    ///
    /// Discord accepts `60`, `300`, `900`, `1800`, and `3600`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u64>,

    /// The guild's icon as image data.
    ///
    /// Base64 1024x1024 PNG, JPEG, or GIF image; a GIF may be animated when the
    /// guild has the `ANIMATED_ICON` feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    /// The guild's splash as image data.
    ///
    /// Base64 16:9 PNG or JPEG image, used when the guild has the
    /// `INVITE_SPLASH` feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splash: Option<String>,

    /// The guild's discovery splash as image data.
    ///
    /// Base64 16:9 PNG or JPEG image, used when the guild has the
    /// `DISCOVERABLE` feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<String>,

    /// The guild's banner as image data.
    ///
    /// Base64 16:9 PNG or JPEG image, used when the guild has the `BANNER`
    /// feature; a GIF may be animated when the guild has the `ANIMATED_BANNER`
    /// feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The ID of the channel where guild notices are posted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<ChannelId>,

    /// Notification flags for the system channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,

    /// The ID of the channel where Community guilds display rules and
    /// guidelines.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<ChannelId>,

    /// The ID of the channel where Community guild administrators and moderators
    /// receive notices from Discord.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<ChannelId>,

    /// The preferred locale of a Community guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<String>,

    /// The guild's enabled features.
    ///
    /// Adding or removing `COMMUNITY` requires the `ADMINISTRATOR` permission.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<GuildFeature>>,

    /// The guild's description.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the guild's boost progress bar is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<bool>,

    /// The ID of the channel where Community guild administrators and moderators
    /// receive safety alerts from Discord.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<ChannelId>,
}

/// The response of `GET /guilds/{guild.id}/threads/active`.
///
/// See [Discord's list active guild threads endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#list-active-guild-threads
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ListGuildThreadsResponse {
    /// The active threads in the guild.
    pub threads: Vec<Channel>,

    /// A thread member for every returned thread the current user has joined.
    pub members: Vec<ThreadMember>,
}

/// One entry of the `PATCH /guilds/{guild.id}/channels` request body.
///
/// Only the channels to modify need to be sent, and at most one entry may
/// change `parent_id`. See [Discord's modify guild channel positions
/// endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-channel-positions
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildChannelPosition {
    /// The channel to move.
    pub id: ChannelId,

    /// The channel's sorting position.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    /// Whether to sync the channel's permission overwrites with its new parent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_permissions: Option<bool>,

    /// The new parent category of the channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
}
