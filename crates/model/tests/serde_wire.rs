/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests using payloads based on Discord's documentation.

#![allow(clippy::unwrap_used)]

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use cordion_model::application::team::{Team, TeamMember, TeamMemberRole, TeamMembershipState};
use cordion_model::application::{
    ActivityInstance, ActivityLocationKind, Application, ApplicationEventWebhookStatus,
    ApplicationFlags, ApplicationGuild, ApplicationIntegrationType,
    ApplicationRoleConnectionMetadata, ApplicationRoleConnectionMetadataType,
    EditCurrentApplicationRequest, IntegrationTypeConfig,
};
use cordion_model::application_command::{
    ApplicationCommand, ApplicationCommandHandler, ApplicationCommandOptionType,
    ApplicationCommandPermissionType, ApplicationCommandType, CommandOptionChoiceValue,
    CommandOptionNumericValue, GuildApplicationCommandPermissions,
};
use cordion_model::audit_log::{AuditLog, AuditLogEvent};
use cordion_model::auto_moderation::{
    AutoModerationActionType, AutoModerationEventType, AutoModerationRule,
    AutoModerationTriggerType, KeywordPresetType,
};
use cordion_model::channel::{Channel, ChannelFlags, ChannelType, ForumLayout, ThreadMetadata};
use cordion_model::component::{
    ButtonStyle, Component, ComponentResponse, ComponentType, SelectDefaultValue,
};
use cordion_model::emoji::Emoji;
use cordion_model::event::gateway::{
    AutoModerationActionExecution, ChannelInfo, ChannelInfoChannel, ChannelPinsUpdate,
    GatewayEventName, GatewayOpcode, GatewayPayload, GatewayPresenceUpdate,
    GatewayVoiceStateUpdate, GetGatewayBotResponse, GetGatewayResponse, GuildAuditLogEntryCreate,
    GuildBanAdd, GuildBanRemove, GuildCreate, GuildEmojisUpdate, GuildIntegrationsUpdate,
    GuildMemberAdd, GuildMemberRemove, GuildMemberUpdate, GuildMembersChunk, GuildRoleCreate,
    GuildRoleDelete, GuildRoleUpdate, GuildScheduledEventUserAdd, GuildScheduledEventUserRemove,
    GuildSoundboardSoundDelete, GuildSoundboardSoundsUpdate, GuildStickersUpdate, Hello, Identify,
    IdentifyConnectionProperties, IntegrationDelete, InvalidSession, InviteCreate, InviteDelete,
    MessageDelete, MessageDeleteBulk, MessagePollVoteAdd, MessagePollVoteRemove,
    MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll,
    MessageReactionRemoveEmoji, RateLimited, ReactionType, Ready, ReadyApplication,
    RequestChannelInfo, RequestGuildMemberRateLimitMetadata, RequestGuildMembers,
    RequestGuildMembersUserIds, RequestSoundboardSounds, Resume, SessionStartLimit,
    SoundboardSounds, ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate, TypingStart,
    VoiceChannelEffectSend, VoiceChannelStartTimeUpdate, VoiceChannelStatusUpdate,
    VoiceEffectAnimationType, VoiceServerUpdate, WebhooksUpdate,
};
use cordion_model::event::presence::{ActivityFlags, ActivityType, Presence, UserStatus};
use cordion_model::event::webhook_event::{
    WebhookEventBodyData, WebhookEventBodyType, WebhookEventPayload, WebhookEventType,
};
use cordion_model::guild::{
    AddGuildMemberRequest, Ban, BeginGuildPruneRequest, BulkGuildBanRequest, BulkGuildBanResponse,
    CreateGuildBanRequest, DefaultMessageNotificationLevel, ExplicitContentFilterLevel, Guild,
    GuildAgeRestrictionLevel, GuildFeature, GuildMember, GuildMemberFlags, GuildOnboarding,
    GuildPreview, GuildPruneCount, GuildScheduledEvent, GuildScheduledEventEntityType,
    GuildScheduledEventPrivacyLevel, GuildScheduledEventRecurrenceFrequency,
    GuildScheduledEventRecurrenceWeekday, GuildScheduledEventStatus, GuildScheduledEventUser,
    GuildTemplate, GuildWidget, GuildWidgetSettings, IncidentsData, Integration,
    IntegrationAccount, IntegrationExpireBehavior, IntegrationType, ListGuildThreadsResponse,
    MfaLevel, ModifyCurrentMemberRequest, ModifyGuildChannelPosition, ModifyGuildMemberRequest,
    OnboardingMode, OnboardingPromptOption, OnboardingPromptType, PartialGuild, PartialIntegration,
    PremiumTier, SoundboardSound, StageInstance, StagePrivacyLevel, SystemChannelFlags,
    VerificationLevel,
};
use cordion_model::id::Id;
use cordion_model::interaction::{
    AuthorizingIntegrationOwners, CommandOptionValue, GuildInstallOwner, Interaction,
    InteractionCallbackData, InteractionCallbackResponse, InteractionCallbackType,
    InteractionContextType, InteractionData, InteractionResponse, InteractionType,
    MessageInteraction, MessageInteractionMetadata,
};
use cordion_model::invite::{
    GuildInviteFlags, Invite, InviteTargetType, InviteType, InviteWithMetadata, PartialInvite,
};
use cordion_model::message::{
    GetChannelPinsResponse, Message, MessageFlags, MessagePin, MessageReferenceType, MessageType,
    PollLayoutType,
};
use cordion_model::monetization::entitlement::{Entitlement, EntitlementType};
use cordion_model::monetization::sku::{Sku, SkuFlags, SkuKind};
use cordion_model::monetization::subscription::{Subscription, SubscriptionStatus};
use cordion_model::permission::Permission;
use cordion_model::role::{Role, RoleColors, RoleFlags, RoleTags};
use cordion_model::social::identity_profile::{
    ApplicationIdentityList, ApplicationIdentityProfile, DynamicField, DynamicFieldType,
    DynamicFieldValue, Media, ProfileData, ProviderType, UpdateApplicationIdentityProfileRequest,
};
use cordion_model::social::lobby::{
    BulkLobbyMemberRequest, CreateLobbyRequest, CreateOrJoinLobbyRequest, LinkLobbyChannelRequest,
    Lobby, LobbyInvite, LobbyMemberFlags, LobbyMessage, ModifyLobbyRequest,
    SendLobbyMessageRequest, UpsertLobbyMemberRequest,
};
use cordion_model::sticker::{
    CreateGuildStickerRequest, Sticker, StickerFormatType, StickerPack, StickerPackList,
    StickerType,
};
use cordion_model::user::{
    ApplicationRoleConnection, Connection, ConnectionService, ConnectionVisibility,
    CreateDmRequest, CreateGroupDmRequest, NameplatePalette, PremiumType, User, UserFlags,
};
use cordion_model::voice::{
    ModifyCurrentUserVoiceStateRequest, ModifyUserVoiceStateRequest, VoiceRegion, VoiceState,
};
use cordion_model::webhook::{Webhook, WebhookType};
use serde_json::Value;

/// Returns the Unix timestamp identified by `seconds` and `nanos`.
fn timestamp(seconds: u64, nanos: u32) -> SystemTime {
    UNIX_EPOCH + Duration::new(seconds, nanos)
}

/// Tests that permissions accept decimal strings and serialize as strings.
#[test]
fn permission_is_stringified() {
    let from_string: Permission = serde_json::from_str("\"2112\"").unwrap();
    let from_integer: Permission = serde_json::from_str("2112").unwrap();

    assert_eq!(from_string.bits(), 2112);
    assert_eq!(from_string, from_integer);
    assert_eq!(serde_json::to_value(from_string).unwrap(), serde_json::json!("2112"));
}

/// Tests that unknown permission bits survive deserialization.
#[test]
fn permission_preserves_unknown_bits() {
    let permission: Permission = serde_json::from_str("\"9007199254740992\"").unwrap();

    assert_eq!(permission.bits(), 1 << 53);
}

/// Tests the example thread channel from Discord's channel documentation.
#[test]
fn thread_metadata_matches_discord_example() {
    let metadata: ThreadMetadata = serde_json::from_str(
        r#"{
            "archived": false,
            "auto_archive_duration": 1440,
            "archive_timestamp": "2021-04-12T23:40:39.855793+00:00",
            "locked": false
        }"#,
    )
    .unwrap();

    assert!(!metadata.archived);
    assert_eq!(metadata.auto_archive_duration, 1440);
    assert_eq!(metadata.archive_timestamp, timestamp(1_618_270_839, 855_793_000));
    assert!(!metadata.locked);
    assert_eq!(metadata.invitable, None);
    assert_eq!(metadata.create_timestamp, None);

    let value = serde_json::to_value(metadata).unwrap();
    assert_eq!(
        value["archive_timestamp"],
        serde_json::json!("2021-04-12T23:40:39.855793+00:00")
    );
    assert!(value.get("invitable").is_none());
    assert!(value.get("create_timestamp").is_none());
}

/// Tests the example incidents data from Discord's guild documentation.
#[test]
fn incidents_data_matches_discord_example() {
    let incidents: IncidentsData = serde_json::from_str(
        r#"{
            "invites_disabled_until": "2023-09-01T14:48:02.222000+00:00",
            "dms_disabled_until": null
        }"#,
    )
    .unwrap();

    assert_eq!(incidents.invites_disabled_until, Some(timestamp(1_693_579_682, 222_000_000)));
    assert_eq!(incidents.dms_disabled_until, None);
    assert_eq!(incidents.dm_spam_detected_at, None);
    assert_eq!(incidents.raid_detected_at, None);

    let value = serde_json::to_value(incidents).unwrap();
    assert_eq!(
        value["invites_disabled_until"],
        serde_json::json!("2023-09-01T14:48:02.222+00:00")
    );
    assert_eq!(value["dms_disabled_until"], serde_json::Value::Null);
    assert!(value.get("dm_spam_detected_at").is_none());
    assert!(value.get("raid_detected_at").is_none());
}

/// Tests that required nullable incidents fields cannot be omitted.
#[test]
fn incidents_data_requires_present_timestamps() {
    assert!(serde_json::from_str::<IncidentsData>(r#"{ "dms_disabled_until": null }"#).is_err());
    assert!(
        serde_json::from_str::<IncidentsData>(r#"{ "invites_disabled_until": null }"#).is_err()
    );
}

/// Tests the example sticker from Discord's sticker documentation.
#[test]
fn sticker_matches_discord_example() {
    let sticker: Sticker = serde_json::from_str(
        r#"{
            "id": "749054660769218631",
            "name": "Wave",
            "tags": "wumpus, hello",
            "type": 1,
            "format_type": 3,
            "description": "Wumpus waves hello",
            "pack_id": "847199849233514549",
            "sort_value": 12
        }"#,
    )
    .unwrap();

    assert_eq!(sticker.kind, StickerType::STANDARD);
    assert_eq!(sticker.format_type, StickerFormatType::LOTTIE);
    assert_eq!(sticker.description.as_deref(), Some("Wumpus waves hello"));
    assert_eq!(sticker.tags, "wumpus, hello");
    assert_eq!(sticker.pack_id.map(|id| id.get()), Some(847_199_849_233_514_549));
    assert_eq!(sticker.available, None);
    assert_eq!(sticker.guild_id, None);
    assert_eq!(sticker.user, None);
    assert_eq!(sticker.sort_value, Some(12));

    let value = serde_json::to_value(sticker).unwrap();
    assert_eq!(value["type"], serde_json::json!(1));
    assert!(value.get("kind").is_none());
    assert_eq!(value["pack_id"], serde_json::json!("847199849233514549"));
    assert_eq!(value["tags"], serde_json::json!("wumpus, hello"));
}

/// Tests that a required nullable sticker description serializes as `null`.
#[test]
fn sticker_description_serializes_as_null() {
    let sticker: Sticker = serde_json::from_str(
        r#"{
            "id": "749054660769218631",
            "name": "Wave",
            "tags": "wumpus",
            "type": 1,
            "format_type": 3,
            "description": null
        }"#,
    )
    .unwrap();

    assert_eq!(sticker.description, None);
    let value = serde_json::to_value(sticker).unwrap();
    assert_eq!(value["description"], Value::Null);
    assert!(value.get("pack_id").is_none());
}

/// Tests that the required sticker tags field cannot be omitted.
#[test]
fn sticker_requires_tags() {
    assert!(
        serde_json::from_str::<Sticker>(
            r#"{
                "id": "749054660769218631",
                "name": "Wave",
                "type": 1,
                "format_type": 3,
                "description": null
            }"#
        )
        .is_err()
    );
}

/// Tests the example partial guild from Discord's user documentation.
#[test]
fn partial_guild_matches_discord_example() {
    let guild: PartialGuild = serde_json::from_str(
        r#"{
            "id": "80351110224678912",
            "name": "1337 Krew",
            "icon": "8342729096ea3675442027381ff50dfe",
            "banner": "bb42bdc37653b7cf58c4c8cc622e76cb",
            "owner": true,
            "permissions": "36953089",
            "features": ["COMMUNITY", "NEWS", "ANIMATED_ICON", "INVITE_SPLASH", "BANNER", "ROLE_ICONS"],
            "approximate_member_count": 3268,
            "approximate_presence_count": 784
        }"#,
    )
    .unwrap();

    assert_eq!(guild.id.get(), 80_351_110_224_678_912);
    assert_eq!(guild.name, "1337 Krew");
    assert_eq!(guild.icon.as_deref(), Some("8342729096ea3675442027381ff50dfe"));
    assert_eq!(guild.banner.as_deref(), Some("bb42bdc37653b7cf58c4c8cc622e76cb"));
    assert!(guild.owner);
    assert_eq!(guild.permissions.bits(), 36_953_089);
    assert_eq!(guild.features.len(), 6);
    assert_eq!(guild.features[0].as_str(), "COMMUNITY");
    assert_eq!(guild.features[5].as_str(), "ROLE_ICONS");
    assert_eq!(guild.approximate_member_count, Some(3268));
    assert_eq!(guild.approximate_presence_count, Some(784));
}

/// Tests nullable partial guild fields and omitted optional counts.
#[test]
fn partial_guild_nullable_fields() {
    let guild: PartialGuild = serde_json::from_str(
        r#"{
            "id": "80351110224678912",
            "name": "1337 Krew",
            "icon": null,
            "banner": null,
            "owner": false,
            "permissions": "0",
            "features": []
        }"#,
    )
    .unwrap();

    assert_eq!(guild.icon, None);
    assert_eq!(guild.banner, None);
    assert_eq!(guild.approximate_member_count, None);
    assert_eq!(guild.approximate_presence_count, None);

    let value = serde_json::to_value(guild).unwrap();
    assert_eq!(value["icon"], Value::Null);
    assert_eq!(value["banner"], Value::Null);
    assert!(value.get("approximate_member_count").is_none());
    assert!(value.get("approximate_presence_count").is_none());
}

/// Tests that the integration account ID accepts arbitrary strings.
#[test]
fn integration_account_accepts_external_ids() {
    let account: IntegrationAccount = serde_json::from_str(
        r#"{
            "id": "twitch-123",
            "name": "Twitch"
        }"#,
    )
    .unwrap();

    assert_eq!(account.id, "twitch-123");
    assert_eq!(serde_json::to_value(account).unwrap()["id"], "twitch-123");
}

/// Tests the example SKU from Discord's SKU documentation.
#[test]
fn sku_matches_discord_example() {
    let sku: Sku = serde_json::from_str(
        r#"{
            "id": "1088510058284990888",
            "type": 5,
            "dependent_sku_id": null,
            "application_id": "788708323867885999",
            "manifest_labels": null,
            "access_type": 1,
            "name": "Test Premium",
            "features": [],
            "release_date": null,
            "premium": false,
            "slug": "test-premium",
            "flags": 128,
            "show_age_gate": false
        }"#,
    )
    .unwrap();

    assert_eq!(sku.id.get(), 1_088_510_058_284_990_888);
    assert_eq!(sku.kind, SkuKind::SUBSCRIPTION);
    assert_eq!(sku.application_id.get(), 788_708_323_867_885_999);
    assert_eq!(sku.flags, SkuFlags::GUILD_SUBSCRIPTION);
}

/// Tests a full guild payload with roles, emoji, and stickers as objects.
#[test]
fn guild_decodes_nested_resources() {
    let guild: Guild = serde_json::from_str(
        r#"{
            "id": "197038439483310086",
            "name": "Discord Testers",
            "icon": null,
            "splash": null,
            "discovery_splash": null,
            "owner_id": "73193882359173120",
            "afk_channel_id": null,
            "afk_timeout": 300,
            "verification_level": 3,
            "default_message_notifications": 1,
            "explicit_content_filter": 2,
            "roles": [
                {
                    "id": "197038439483310086",
                    "name": "@everyone",
                    "color": 0,
                    "colors": {
                        "primary_color": 0,
                        "secondary_color": null,
                        "tertiary_color": null
                    },
                    "hoist": false,
                    "position": 0,
                    "permissions": "104324673",
                    "managed": false,
                    "mentionable": false,
                    "tags": {
                        "premium_subscriber": null
                    },
                    "flags": 0
                }
            ],
            "emojis": [
                {
                    "id": "41771983423143937",
                    "name": "test",
                    "roles": [],
                    "require_colons": true,
                    "managed": false,
                    "animated": false,
                    "available": true
                }
            ],
            "features": ["COMMUNITY"],
            "mfa_level": 1,
            "application_id": null,
            "system_channel_id": null,
            "system_channel_flags": 0,
            "rules_channel_id": "441688182833020939",
            "max_presences": null,
            "max_members": 250000,
            "vanity_url_code": "discord-testers",
            "description": "The official place to report Discord Bugs!",
            "banner": "9b6439a7de04f1d26af92f84ac9e1e4a",
            "premium_tier": 3,
            "premium_subscription_count": 33,
            "preferred_locale": "en-US",
            "public_updates_channel_id": "281283303326089216",
            "max_video_channel_users": 25,
            "max_stage_video_channel_users": 50,
            "welcome_screen": {
                "description": null,
                "welcome_channels": [
                    {
                        "channel_id": "697138785317814292",
                        "description": "Follow for official Discord API updates",
                        "emoji_id": null,
                        "emoji_name": "📡"
                    }
                ]
            },
            "nsfw_level": 0,
            "stickers": [
                {
                    "id": "749054660769218631",
                    "name": "Wave",
                    "description": null,
                    "tags": "wumpus",
                    "type": 2,
                    "format_type": 1,
                    "available": true,
                    "guild_id": "197038439483310086"
                }
            ],
            "premium_progress_bar_enabled": false,
            "safety_alerts_channel_id": "281283303326089216",
            "incidents_data": null
        }"#,
    )
    .unwrap();

    assert_eq!(guild.id.get(), 197_038_439_483_310_086);
    assert_eq!(guild.owner_id.get(), 73_193_882_359_173_120);
    assert_eq!(guild.roles.len(), 1);
    assert_eq!(guild.roles[0].id.get(), guild.id.get());
    assert_eq!(guild.roles[0].name, "@everyone");
    assert_eq!(guild.roles[0].permissions.bits(), 104_324_673);
    let tags = guild.roles[0].tags.as_ref().unwrap();
    assert!(tags.premium_subscriber);
    assert!(!tags.available_for_purchase);
    assert_eq!(guild.emojis.len(), 1);
    assert_eq!(guild.emojis[0].id.map(|id| id.get()), Some(41_771_983_423_143_937));
    assert_eq!(guild.emojis[0].name.as_deref(), Some("test"));

    let stickers = guild.stickers.as_ref().unwrap();
    assert_eq!(stickers.len(), 1);
    assert_eq!(stickers[0].kind, StickerType::GUILD);
    assert_eq!(stickers[0].guild_id, Some(guild.id));

    assert_eq!(guild.rules_channel_id.unwrap().get(), 441_688_182_833_020_939);
    assert_eq!(guild.public_updates_channel_id.unwrap().get(), 281_283_303_326_089_216);
    assert_eq!(guild.safety_alerts_channel_id.unwrap().get(), 281_283_303_326_089_216);
    assert_eq!(guild.application_id, None);
    assert_eq!(guild.incidents_data, None);
    assert_eq!(guild.owner, None);
    assert!(!guild.premium_progress_bar_enabled);

    let welcome = guild.welcome_screen.as_ref().unwrap();
    assert_eq!(welcome.description, None);
    assert_eq!(welcome.welcome_channels[0].channel_id.get(), 697_138_785_317_814_292);

    let value = serde_json::to_value(guild).unwrap();
    assert!(value["roles"].is_array());
    assert_eq!(value["roles"][0]["name"], "@everyone");
    assert!(value["emojis"].is_array());
    assert_eq!(value["emojis"][0]["name"], "test");
    assert!(value["stickers"][0].get("pack_id").is_none());
    assert_eq!(value["application_id"], Value::Null);
    assert_eq!(value["icon"], Value::Null);
    assert_eq!(value["incidents_data"], Value::Null);
    assert!(value.get("owner").is_none());
    assert_eq!(value["welcome_screen"]["description"], Value::Null);
    assert_eq!(value["welcome_screen"]["welcome_channels"][0]["emoji_id"], Value::Null);
}

/// Tests the snowflake string wire format shared by ID fields.
#[test]
fn id_serializes_as_string_and_accepts_integer() {
    let from_string: Id<Guild> = serde_json::from_str("\"1556204224512000000\"").unwrap();
    let from_integer: Id<Guild> = serde_json::from_str("1556204224512000000").unwrap();

    assert_eq!(from_string, from_integer);
    assert_eq!(
        serde_json::to_value(from_string).unwrap(),
        serde_json::json!("1556204224512000000")
    );
    assert!(serde_json::from_str::<Id<Guild>>("\"not-a-snowflake\"").is_err());
    assert!(serde_json::from_str::<Id<Guild>>("-1").is_err());
}

/// Tests the example custom emoji from Discord's emoji documentation.
#[test]
fn emoji_matches_discord_example() {
    let emoji: Emoji = serde_json::from_str(
        r#"{
            "id": "41771983429993937",
            "name": "LUL",
            "roles": ["41771983429993000", "41771983429993111"],
            "user": {
                "username": "Luigi",
                "discriminator": "0002",
                "id": "96008815106887111",
                "avatar": "5500909a3274e1812beb4e8de6631111",
                "public_flags": 131328
            },
            "require_colons": true,
            "managed": false,
            "animated": false
        }"#,
    )
    .unwrap();

    assert_eq!(emoji.id.map(|id| id.get()), Some(41_771_983_429_993_937));
    assert_eq!(emoji.name.as_deref(), Some("LUL"));
    let roles = emoji.roles.as_ref().unwrap();
    assert_eq!(roles.len(), 2);
    assert_eq!(roles[0].get(), 41_771_983_429_993_000);
    assert_eq!(roles[1].get(), 41_771_983_429_993_111);
    let user = emoji.user.as_ref().unwrap();
    assert_eq!(user.id.get(), 96_008_815_106_887_111);
    assert_eq!(user.username, "Luigi");
    assert_eq!(user.public_flags.map(|flags| flags.bits()), Some(131_328));
    assert_eq!(emoji.require_colons, Some(true));
    assert_eq!(emoji.managed, Some(false));
    assert_eq!(emoji.animated, Some(false));
    assert_eq!(emoji.available, None);

    let value = serde_json::to_value(emoji).unwrap();
    assert_eq!(value["id"], "41771983429993937");
    assert_eq!(value["roles"][0], "41771983429993000");
    assert_eq!(value["user"]["id"], "96008815106887111");
    assert!(value.get("available").is_none());
}

/// Tests the standard emoji example, whose ID is explicitly `null`.
#[test]
fn standard_emoji_serializes_null_id() {
    let emoji: Emoji = serde_json::from_str(r#"{"id": null, "name": "🔥"}"#).unwrap();

    assert_eq!(emoji.id, None);
    assert_eq!(emoji.name.as_deref(), Some("🔥"));
    assert_eq!(emoji.roles, None);

    let value = serde_json::to_value(emoji).unwrap();
    assert_eq!(value["id"], Value::Null);
    assert!(value.get("roles").is_none());
}

/// Tests the example role from Discord's permissions documentation.
#[test]
#[allow(deprecated)]
fn role_matches_discord_example() {
    let role: Role = serde_json::from_str(
        r#"{
            "id": "41771983423143936",
            "name": "WE DEM BOYZZ!!!!!!",
            "color": 3447003,
            "colors": {
                "primary_color": 3447003,
                "secondary_color": null,
                "tertiary_color": null
            },
            "hoist": true,
            "icon": "cf3ced8600b777c9486c6d8d84fb4327",
            "unicode_emoji": null,
            "position": 1,
            "permissions": "66321471",
            "managed": false,
            "mentionable": false,
            "flags": 0
        }"#,
    )
    .unwrap();

    assert_eq!(role.id.get(), 41_771_983_423_143_936);
    assert_eq!(role.name, "WE DEM BOYZZ!!!!!!");
    assert_eq!(role.color, 3_447_003);
    assert_eq!(
        role.colors,
        RoleColors {
            primary_color: 3_447_003,
            secondary_color: None,
            tertiary_color: None,
        }
    );
    assert!(role.hoist);
    assert_eq!(role.icon.as_deref(), Some("cf3ced8600b777c9486c6d8d84fb4327"));
    assert_eq!(role.unicode_emoji, None);
    assert_eq!(role.position, 1);
    assert_eq!(role.permissions.bits(), 66_321_471);
    assert!(!role.managed);
    assert!(!role.mentionable);
    assert_eq!(role.flags, RoleFlags::NONE);

    let value = serde_json::to_value(role).unwrap();
    assert_eq!(value["id"], "41771983423143936");
    assert_eq!(value["permissions"], "66321471");
    assert_eq!(value["colors"]["primary_color"], serde_json::json!(3_447_003));
    assert_eq!(value["colors"]["secondary_color"], Value::Null);
    assert_eq!(value["colors"]["tertiary_color"], Value::Null);
    assert_eq!(value["flags"], serde_json::json!(0));
    assert!(value.get("unicode_emoji").is_none());
}

/// Tests enhanced role colors with gradient and holographic values.
#[test]
fn role_colors_serialize_enhanced_values() {
    let colors: RoleColors = serde_json::from_str(
        r#"{
            "primary_color": 1193046,
            "secondary_color": 11259375,
            "tertiary_color": 16777215
        }"#,
    )
    .unwrap();

    assert_eq!(colors.primary_color, 1_193_046);
    assert_eq!(colors.secondary_color, Some(11_259_375));
    assert_eq!(colors.tertiary_color, Some(16_777_215));

    let value = serde_json::to_value(colors).unwrap();
    assert_eq!(value["secondary_color"], serde_json::json!(11_259_375));
    assert_eq!(value["tertiary_color"], serde_json::json!(16_777_215));
}

/// Tests that unknown role flag bits are preserved.
#[test]
fn role_flags_preserve_unknown_bits() {
    let bits = RoleFlags::IN_PROMPT.bits() | (1 << 9);
    let flags: RoleFlags = serde_json::from_str(&bits.to_string()).unwrap();

    assert!(flags.contains(RoleFlags::IN_PROMPT));
    assert_eq!(flags.bits(), bits);
    assert_eq!(serde_json::to_value(flags).unwrap(), serde_json::json!(bits));
}

/// Tests that unknown guild feature strings are preserved.
#[test]
fn guild_features_preserve_unknown_values() {
    let features: Vec<GuildFeature> =
        serde_json::from_str(r#"["COMMUNITY", "FUTURE_FEATURE"]"#).unwrap();

    assert_eq!(features[0], GuildFeature::COMMUNITY);
    assert_eq!(features[1].as_str(), "FUTURE_FEATURE");
    assert_eq!(
        serde_json::to_value(&features).unwrap(),
        serde_json::json!(["COMMUNITY", "FUTURE_FEATURE"])
    );
}

/// Tests that unknown system channel flag bits are preserved.
#[test]
fn system_channel_flags_preserve_unknown_bits() {
    let bits = SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS.bits() | (1 << 40);
    let flags: SystemChannelFlags = serde_json::from_str(&bits.to_string()).unwrap();

    assert!(flags.contains(SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS));
    assert_eq!(flags.bits(), bits);
    assert_eq!(serde_json::to_value(flags).unwrap(), serde_json::json!(bits));
}

/// Tests known constants and unknown numeric guild levels.
#[test]
fn guild_levels_preserve_unknown_values() {
    assert_eq!(serde_json::from_str::<VerificationLevel>("3").unwrap(), VerificationLevel::HIGH);
    assert_eq!(
        serde_json::from_str::<DefaultMessageNotificationLevel>("1").unwrap(),
        DefaultMessageNotificationLevel::ONLY_MENTIONS
    );
    assert_eq!(
        serde_json::from_str::<ExplicitContentFilterLevel>("2").unwrap(),
        ExplicitContentFilterLevel::ALL_MEMBERS
    );
    assert_eq!(serde_json::from_str::<MfaLevel>("1").unwrap(), MfaLevel::ELEVATED);
    assert_eq!(
        serde_json::from_str::<GuildAgeRestrictionLevel>("3").unwrap(),
        GuildAgeRestrictionLevel::AGE_RESTRICTED
    );
    assert_eq!(serde_json::from_str::<PremiumTier>("3").unwrap(), PremiumTier::TIER_3);

    for raw in ["9", "255"] {
        let value: VerificationLevel = serde_json::from_str(raw).unwrap();
        assert_eq!(
            serde_json::to_value(value).unwrap(),
            serde_json::json!(raw.parse::<u8>().unwrap())
        );
    }
}

/// Tests that unknown premium types are preserved.
#[test]
fn premium_type_preserves_unknown_value() {
    assert_eq!(serde_json::from_str::<PremiumType>("2").unwrap(), PremiumType::NITRO);

    let unknown: PremiumType = serde_json::from_str("9").unwrap();
    assert_eq!(serde_json::to_value(unknown).unwrap(), serde_json::json!(9));
}

/// Tests that unknown SKU kinds and flag bits are preserved.
#[test]
fn sku_kind_and_flags_preserve_unknown_values() {
    let kind: SkuKind = serde_json::from_str("42").unwrap();
    assert_eq!(serde_json::to_value(kind).unwrap(), serde_json::json!(42));

    let bits = SkuFlags::AVAILABLE.bits() | (1 << 9);
    let flags: SkuFlags = serde_json::from_str(&bits.to_string()).unwrap();
    assert!(flags.contains(SkuFlags::AVAILABLE));
    assert_eq!(flags.bits(), bits);
    assert_eq!(serde_json::to_value(flags).unwrap(), serde_json::json!(bits));
}

/// Tests the example user from Discord's user documentation.
#[test]
fn user_matches_discord_example() {
    let user: User = serde_json::from_str(
        r#"{
            "id": "80351110224678912",
            "username": "Nelly",
            "global_name": null,
            "discriminator": "1337",
            "avatar": "8342729096ea3675442027381ff50dfe",
            "verified": true,
            "email": "nelly@discord.com",
            "flags": 64,
            "banner": "06c16474723fe537c283b8efa61a30c8",
            "accent_color": 16711680,
            "premium_type": 0,
            "public_flags": 64,
            "avatar_decoration_data": {
                "sku_id": "1144058844004233369",
                "asset": "a_fed43ab12698df65902ba06727e20c0e"
            },
            "collectibles": {
                "nameplate": {
                    "sku_id": "2247558840304243311",
                    "asset": "nameplates/nameplates/twilight/",
                    "label": "",
                    "palette": "cobalt"
                }
            },
            "primary_guild": {
                "identity_guild_id": "1234647491267808778",
                "identity_enabled": true,
                "tag": "DISC",
                "badge": "7d1734ae5a615e82bc7a4033b98fade8"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(user.id.get(), 80_351_110_224_678_912);
    assert_eq!(user.username, "Nelly");
    assert_eq!(user.discriminator, "1337");
    assert_eq!(user.global_name, None);
    assert_eq!(user.avatar.as_deref(), Some("8342729096ea3675442027381ff50dfe"));
    assert_eq!(user.verified, Some(true));
    assert_eq!(user.email.as_deref(), Some("nelly@discord.com"));
    assert_eq!(user.flags, Some(UserFlags::HYPESQUAD_ONLINE_HOUSE_1));
    assert_eq!(user.premium_type, Some(PremiumType::NONE));
    assert_eq!(user.public_flags, Some(UserFlags::HYPESQUAD_ONLINE_HOUSE_1));
    assert!(!user.is_bot());
    assert!(!user.is_system());

    let decoration = user.avatar_decoration_data.as_ref().unwrap();
    assert_eq!(decoration.sku_id.get(), 1_144_058_844_004_233_369);
    assert_eq!(decoration.asset, "a_fed43ab12698df65902ba06727e20c0e");

    let nameplate = user.collectibles.as_ref().unwrap().nameplate.as_ref().unwrap();
    assert_eq!(nameplate.sku_id.get(), 2_247_558_840_304_243_311);
    assert_eq!(nameplate.asset, "nameplates/nameplates/twilight/");
    assert_eq!(nameplate.label, "");
    assert_eq!(nameplate.palette, NameplatePalette::Cobalt);

    let primary_guild = user.primary_guild.as_ref().unwrap();
    assert_eq!(primary_guild.identity_guild_id.unwrap().get(), 1_234_647_491_267_808_778);
    assert_eq!(primary_guild.identity_enabled, Some(true));
    assert_eq!(primary_guild.tag.as_deref(), Some("DISC"));
    assert_eq!(primary_guild.badge.as_deref(), Some("7d1734ae5a615e82bc7a4033b98fade8"));

    let value = serde_json::to_value(user).unwrap();
    assert_eq!(value["id"], "80351110224678912");
    assert_eq!(value["flags"], serde_json::json!(64));
    assert_eq!(value["premium_type"], serde_json::json!(0));
    assert_eq!(value["avatar_decoration_data"]["sku_id"], "1144058844004233369");
    assert_eq!(value["collectibles"]["nameplate"]["palette"], "cobalt");
    assert_eq!(value["primary_guild"]["identity_guild_id"], "1234647491267808778");
    assert!(value.get("global_name").is_none());
}

/// Tests that absent optional user fields are omitted during serialization.
#[test]
fn user_omits_absent_optional_fields() {
    let user: User = serde_json::from_str(
        r#"{
            "id": "80351110224678912",
            "username": "bot",
            "discriminator": "0",
            "bot": true
        }"#,
    )
    .unwrap();

    assert!(user.is_bot());
    assert!(!user.is_system());
    assert_eq!(user.email, None);
    assert_eq!(user.premium_type, None);

    let value = serde_json::to_value(user).unwrap();
    assert_eq!(value["bot"], true);
    for field in [
        "global_name",
        "avatar",
        "system",
        "mfa_enabled",
        "banner",
        "accent_color",
        "locale",
        "verified",
        "email",
        "flags",
        "premium_type",
        "public_flags",
        "avatar_decoration_data",
        "collectibles",
        "primary_guild",
    ] {
        assert!(value.get(field).is_none(), "{field} should be omitted");
    }
}

/// Tests that unknown user flag bits are preserved.
#[test]
fn user_flags_preserve_unknown_bits() {
    let bits = UserFlags::STAFF.bits() | (1 << 40);
    let flags: UserFlags = serde_json::from_str(&bits.to_string()).unwrap();

    assert!(flags.contains(UserFlags::STAFF));
    assert_eq!(flags.bits(), bits);
    assert_eq!(serde_json::to_value(flags).unwrap(), serde_json::json!(bits));
}

/// Tests known and unknown nameplate palette identifiers.
#[test]
fn nameplate_palette_round_trips() {
    let known: NameplatePalette = serde_json::from_str("\"bubble_gum\"").unwrap();
    assert_eq!(known, NameplatePalette::BubbleGum);
    assert_eq!(known.as_str(), "bubble_gum");
    assert_eq!(serde_json::to_value(known).unwrap(), serde_json::json!("bubble_gum"));

    let unknown: NameplatePalette = serde_json::from_str("\"ultraviolet\"").unwrap();
    assert_eq!(unknown, NameplatePalette::Unknown("ultraviolet".to_owned()));
    assert_eq!(unknown.as_str(), "ultraviolet");
    assert_eq!(serde_json::to_value(unknown).unwrap(), serde_json::json!("ultraviolet"));
}

/// Tests the install params embedded in Discord's application example.
#[test]
fn install_params_match_discord_example() {
    let config: IntegrationTypeConfig = serde_json::from_str(
        r#"{
            "oauth2_install_params": {
                "scopes": ["applications.commands", "bot"],
                "permissions": "2048"
            }
        }"#,
    )
    .unwrap();

    let params = config.oauth2_install_params.as_ref().unwrap();
    assert_eq!(params.scopes, ["applications.commands", "bot"]);
    assert_eq!(params.permissions.bits(), 2_048);

    let value = serde_json::to_value(config).unwrap();
    assert_eq!(value["oauth2_install_params"]["permissions"], "2048");
    assert_eq!(value["oauth2_install_params"]["scopes"][1], "bot");
}

/// Tests that an absent install params object is omitted.
#[test]
fn integration_type_config_omits_absent_install_params() {
    let config: IntegrationTypeConfig = serde_json::from_str("{}").unwrap();

    assert!(config.oauth2_install_params.is_none());
    assert_eq!(serde_json::to_value(config).unwrap(), serde_json::json!({}));
}

/// Tests a full guild member with optional and required nullable fields.
#[test]
fn guild_member_matches_documented_fields() {
    let member: GuildMember = serde_json::from_str(
        r#"{
            "user": {
                "id": "80351110224678912",
                "username": "Nelly",
                "discriminator": "1333",
                "public_flags": 64
            },
            "nick": "Nelly",
            "avatar": "6a26cdca72c0d3b6f0c3f4c33d5f6d1d",
            "banner": null,
            "roles": ["41771983423143936"],
            "joined_at": "2015-04-26T06:26:56.936000+00:00",
            "premium_since": null,
            "deaf": false,
            "mute": false,
            "pending": false,
            "permissions": "2048",
            "communication_disabled_until": null,
            "avatar_decoration_data": null,
            "collectibles": null
        }"#,
    )
    .unwrap();

    assert_eq!(member.user.as_ref().unwrap().id.get(), 80_351_110_224_678_912);
    assert_eq!(member.nick.as_deref(), Some("Nelly"));
    assert_eq!(member.roles[0].get(), 41_771_983_423_143_936);
    assert_eq!(member.joined_at, Some(timestamp(1_430_029_616, 936_000_000)));
    assert_eq!(member.premium_since, None);
    assert_eq!(member.flags, GuildMemberFlags::NONE);
    assert_eq!(member.pending, Some(false));
    assert_eq!(member.permissions.unwrap().bits(), 2_048);
    assert_eq!(member.communication_disabled_until, None);
    assert_eq!(member.avatar_decoration_data, None);
    assert_eq!(member.collectibles, None);

    let value = serde_json::to_value(member).unwrap();
    assert_eq!(value["joined_at"], "2015-04-26T06:26:56.936+00:00");
    assert_eq!(value["flags"], 0);
    assert_eq!(value["permissions"], "2048");
    assert_eq!(value["pending"], false);
    for field in [
        "banner",
        "premium_since",
        "communication_disabled_until",
        "avatar_decoration_data",
        "collectibles",
    ] {
        assert!(value.get(field).is_none(), "{field} should be omitted");
    }
}

/// Tests guest members, missing users, and unknown guild member flag bits.
#[test]
fn guild_member_allows_absent_user_and_null_joined_at() {
    let bits = GuildMemberFlags::IS_GUEST.bits() | (1 << 20);
    let member: GuildMember = serde_json::from_str(&format!(
        r#"{{
            "roles": [],
            "joined_at": null,
            "deaf": false,
            "mute": false,
            "flags": {bits}
        }}"#
    ))
    .unwrap();

    assert_eq!(member.user, None);
    assert_eq!(member.joined_at, None);
    assert!(member.flags.contains(GuildMemberFlags::IS_GUEST));
    assert_eq!(member.flags.bits(), bits);

    let value = serde_json::to_value(member).unwrap();
    assert_eq!(value["joined_at"], serde_json::Value::Null);
    assert_eq!(value["flags"], bits);
    assert!(value.get("user").is_none());
}

/// Tests that presence-only role tags distinguish absent from null.
#[test]
fn role_tags_represent_presence_as_null() {
    let tags: RoleTags = serde_json::from_str(
        r#"{
            "bot_id": "41771983423143937",
            "integration_id": "41771983423143938",
            "premium_subscriber": null,
            "subscription_listing_id": "41771983423143939",
            "available_for_purchase": null
        }"#,
    )
    .unwrap();

    assert_eq!(tags.bot_id.unwrap().get(), 41_771_983_423_143_937);
    assert_eq!(tags.integration_id.unwrap().get(), 41_771_983_423_143_938);
    assert_eq!(tags.subscription_listing_id.unwrap().get(), 41_771_983_423_143_939);
    assert!(tags.premium_subscriber);
    assert!(tags.available_for_purchase);
    assert!(!tags.guild_connections);

    let value = serde_json::to_value(tags).unwrap();
    assert_eq!(value["premium_subscriber"], serde_json::Value::Null);
    assert_eq!(value["available_for_purchase"], serde_json::Value::Null);
    assert!(value.get("guild_connections").is_none());
    assert_eq!(value["bot_id"], "41771983423143937");

    let empty: RoleTags = serde_json::from_str("{}").unwrap();
    assert!(!empty.premium_subscriber);
    assert!(!empty.available_for_purchase);
    assert!(!empty.guild_connections);
    assert_eq!(serde_json::to_value(empty).unwrap(), serde_json::json!({}));
}

/// Tests a presence with a partial user, activities, and client status.
#[test]
fn presence_matches_documented_fields() {
    let presence: Presence = serde_json::from_str(
        r#"{
            "user": {
                "id": "80351110224678912",
                "username": "Nelly"
            },
            "status": "dnd",
            "activities": [{
                "name": "Rocket League",
                "type": 0,
                "created_at": 1511200066000,
                "application_id": "379286085710381999",
                "state": "In a Match",
                "details": "Ranked Duos: 2-1",
                "timestamps": {
                    "start": 15112000660000
                },
                "party": {
                    "id": "9dd6594e-81b3-49f6-a6b5-a679e6a060d3",
                    "size": [2, 2]
                },
                "assets": {
                    "large_image": "351371005538729000",
                    "large_text": "DFH Stadium",
                    "small_image": "351371005538729111",
                    "small_text": "Silver III"
                },
                "secrets": {
                    "join": "025ed05c71f639de8bfaa0d679d7c94b2fdce12f",
                    "spectate": "e7eb30d2ee025ed05c71ea495f770b76454ee4e0",
                    "match": "e7eb30d2ee025ed05c71ea495f770b76454ee4e1"
                },
                "flags": 3,
                "buttons": ["Button 1", "Button 2"]
            }],
            "client_status": {
                "desktop": "online",
                "mobile": "idle"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(presence.user.id.get(), 80_351_110_224_678_912);
    assert_eq!(presence.user.username.as_deref(), Some("Nelly"));
    assert_eq!(presence.user.bot, None);
    assert_eq!(presence.guild_id, None);
    assert_eq!(presence.status, UserStatus::DND);
    assert_eq!(presence.client_status.desktop, Some(UserStatus::ONLINE));
    assert_eq!(presence.client_status.mobile, Some(UserStatus::IDLE));
    assert_eq!(presence.client_status.web, None);

    let activity = &presence.activities[0];
    assert_eq!(activity.kind, ActivityType::PLAYING);
    assert_eq!(activity.created_at, 1_511_200_066_000);
    assert_eq!(activity.timestamps.as_ref().unwrap().start, Some(15_112_000_660_000));
    assert_eq!(activity.application_id.unwrap().get(), 379_286_085_710_381_999);

    let party = activity.party.as_ref().unwrap();
    assert_eq!(party.id.as_deref(), Some("9dd6594e-81b3-49f6-a6b5-a679e6a060d3"));
    assert_eq!(party.size.unwrap().current, 2);
    assert_eq!(party.size.unwrap().max, 2);

    let assets = activity.assets.as_ref().unwrap();
    assert_eq!(assets.large_text.as_deref(), Some("DFH Stadium"));
    assert_eq!(assets.small_text.as_deref(), Some("Silver III"));

    let secrets = activity.secrets.as_ref().unwrap();
    assert_eq!(
        secrets.match_secret.as_deref(),
        Some("e7eb30d2ee025ed05c71ea495f770b76454ee4e1")
    );

    let flags = activity.flags.unwrap();
    assert!(flags.contains(ActivityFlags::INSTANCE));
    assert!(flags.contains(ActivityFlags::JOIN));
    assert_eq!(
        activity.buttons.as_ref().unwrap(),
        &["Button 1".to_owned(), "Button 2".to_owned()]
    );

    let value = serde_json::to_value(presence).unwrap();
    assert_eq!(value["status"], "dnd");
    assert!(value.get("guild_id").is_none());
    assert_eq!(value["activities"][0]["type"], 0);
    assert_eq!(value["activities"][0]["party"]["size"], serde_json::json!([2, 2]));
    assert_eq!(
        value["activities"][0]["secrets"]["match"],
        "e7eb30d2ee025ed05c71ea495f770b76454ee4e1"
    );
    assert_eq!(value["activities"][0]["buttons"], serde_json::json!(["Button 1", "Button 2"]));
    assert_eq!(value["client_status"]["desktop"], "online");
}

/// Tests that unknown activity types and statuses round-trip.
#[test]
fn activity_and_status_preserve_unknown_values() {
    let activity: cordion_model::event::presence::Activity = serde_json::from_str(
        r#"{
            "name": "Future Activity",
            "type": 99,
            "created_at": 1700000000000,
            "status_display_type": 42
        }"#,
    )
    .unwrap();

    let value = serde_json::to_value(activity).unwrap();
    assert_eq!(value["type"], 99);
    assert_eq!(value["status_display_type"], 42);

    let status: UserStatus = serde_json::from_str("\"away\"").unwrap();
    assert_eq!(status.as_str(), "away");
    assert_eq!(serde_json::to_value(status).unwrap(), "away");
}

/// Tests the voice state example and required nullable fields.
#[test]
fn voice_state_matches_discord_example() {
    let state: VoiceState = serde_json::from_str(
        r#"{
            "channel_id": "157733188964188161",
            "user_id": "80351110224678912",
            "session_id": "90326bd25d71d39b9ef95b299e3872ff",
            "deaf": false,
            "mute": false,
            "self_deaf": false,
            "self_mute": true,
            "self_video": false,
            "suppress": false,
            "request_to_speak_timestamp": "2021-03-31T18:45:31.297561+00:00"
        }"#,
    )
    .unwrap();

    assert_eq!(state.guild_id, None);
    assert_eq!(state.channel_id.unwrap().get(), 157_733_188_964_188_161);
    assert_eq!(state.user_id.get(), 80_351_110_224_678_912);
    assert_eq!(state.session_id, "90326bd25d71d39b9ef95b299e3872ff");
    assert!(state.self_mute);
    assert_eq!(state.self_stream, None);
    assert_eq!(state.request_to_speak_timestamp, Some(timestamp(1_617_216_331, 297_561_000)));

    let value = serde_json::to_value(state).unwrap();
    assert!(value.get("guild_id").is_none());
    assert!(value.get("self_stream").is_none());
    assert_eq!(value["request_to_speak_timestamp"], "2021-03-31T18:45:31.297561+00:00");

    let disconnected: VoiceState = serde_json::from_str(
        r#"{
            "channel_id": null,
            "user_id": "80351110224678912",
            "session_id": "90326bd25d71d39b9ef95b299e3872ff",
            "deaf": false,
            "mute": false,
            "self_deaf": false,
            "self_mute": false,
            "self_video": false,
            "suppress": false,
            "request_to_speak_timestamp": null
        }"#,
    )
    .unwrap();

    assert_eq!(disconnected.channel_id, None);
    assert_eq!(disconnected.request_to_speak_timestamp, None);

    let value = serde_json::to_value(disconnected).unwrap();
    assert_eq!(value["channel_id"], serde_json::Value::Null);
    assert_eq!(value["request_to_speak_timestamp"], serde_json::Value::Null);
}

/// Tests the stage instance example from Discord's documentation.
#[test]
fn stage_instance_matches_discord_example() {
    let instance: StageInstance = serde_json::from_str(
        r#"{
            "id": "840647391636226060",
            "guild_id": "197038439483310086",
            "channel_id": "733488538393510049",
            "topic": "Testing Testing, 123",
            "privacy_level": 1,
            "discoverable_disabled": false,
            "guild_scheduled_event_id": "947656305244532806"
        }"#,
    )
    .unwrap();

    assert_eq!(instance.id.get(), 840_647_391_636_226_060);
    assert_eq!(instance.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(instance.channel_id.get(), 733_488_538_393_510_049);
    assert_eq!(instance.privacy_level, StagePrivacyLevel::PUBLIC);
    assert_eq!(instance.guild_scheduled_event_id.unwrap().get(), 947_656_305_244_532_806);

    let without_event: StageInstance = serde_json::from_str(
        r#"{
            "id": "840647391636226060",
            "guild_id": "197038439483310086",
            "channel_id": "733488538393510049",
            "topic": "Testing Testing, 123",
            "privacy_level": 2,
            "discoverable_disabled": false,
            "guild_scheduled_event_id": null
        }"#,
    )
    .unwrap();

    assert_eq!(without_event.privacy_level, StagePrivacyLevel::GUILD_ONLY);
    assert_eq!(without_event.guild_scheduled_event_id, None);
    assert_eq!(
        serde_json::to_value(without_event).unwrap()["guild_scheduled_event_id"],
        serde_json::Value::Null
    );
}

/// Tests default and guild soundboard sound examples.
#[test]
fn soundboard_sound_matches_discord_examples() {
    let default_sound: SoundboardSound = serde_json::from_str(
        r#"{
            "name": "quack",
            "sound_id": "1",
            "volume": 1.0,
            "emoji_id": null,
            "emoji_name": "🦆",
            "available": true
        }"#,
    )
    .unwrap();

    assert_eq!(default_sound.sound_id.get(), 1);
    assert_eq!(default_sound.volume, 1.0);
    assert_eq!(default_sound.emoji_id, None);
    assert_eq!(default_sound.emoji_name.as_deref(), Some("🦆"));
    assert_eq!(default_sound.guild_id, None);
    assert_eq!(default_sound.user, None);

    let value = serde_json::to_value(default_sound).unwrap();
    assert_eq!(value["emoji_id"], serde_json::Value::Null);
    assert_eq!(value["emoji_name"], "🦆");
    assert!(value.get("guild_id").is_none());
    assert!(value.get("user").is_none());

    let guild_sound: SoundboardSound = serde_json::from_str(
        r#"{
            "name": "Yay",
            "sound_id": "1106714396018884649",
            "volume": 1,
            "emoji_id": "989193655938064464",
            "emoji_name": null,
            "guild_id": "613425648685547541",
            "available": true,
            "user": {
                "id": "80351110224678912",
                "username": "Nelly",
                "discriminator": "0"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(guild_sound.emoji_id.unwrap().get(), 989_193_655_938_064_464);
    assert_eq!(guild_sound.emoji_name, None);
    assert_eq!(guild_sound.guild_id.unwrap().get(), 613_425_648_685_547_541);
    assert_eq!(guild_sound.user.unwrap().id.get(), 80_351_110_224_678_912);
}

/// Tests a scheduled event with recurrence and required nullable fields.
#[test]
fn guild_scheduled_event_matches_documented_fields() {
    let event: GuildScheduledEvent = serde_json::from_str(
        r#"{
            "id": "947656305244532806",
            "guild_id": "197038439483310086",
            "channel_id": null,
            "creator_id": null,
            "name": "External event",
            "description": null,
            "scheduled_start_time": "2024-07-24T10:00:00.000000+00:00",
            "scheduled_end_time": null,
            "privacy_level": 2,
            "status": 1,
            "entity_type": 3,
            "entity_id": null,
            "entity_metadata": {
                "location": "Test location"
            },
            "user_count": 2,
            "image": null,
            "recurrence_rule": {
                "start": "2024-07-24T10:00:00+00:00",
                "end": null,
                "frequency": 3,
                "interval": 1,
                "by_weekday": [0, 1, 2, 3, 4],
                "by_n_weekday": null,
                "by_month": null,
                "by_month_day": null,
                "by_year_day": null,
                "count": null
            }
        }"#,
    )
    .unwrap();

    assert_eq!(event.id.get(), 947_656_305_244_532_806);
    assert_eq!(event.channel_id, None);
    assert_eq!(event.creator_id, None);
    assert_eq!(event.description, None);
    assert_eq!(event.scheduled_start_time, timestamp(1_721_815_200, 0));
    assert_eq!(event.scheduled_end_time, None);
    assert_eq!(event.privacy_level, GuildScheduledEventPrivacyLevel::GUILD_ONLY);
    assert_eq!(event.status, GuildScheduledEventStatus::SCHEDULED);
    assert_eq!(event.entity_type, GuildScheduledEventEntityType::EXTERNAL);
    assert_eq!(event.entity_id, None);
    assert_eq!(
        event.entity_metadata.as_ref().unwrap().location.as_deref(),
        Some("Test location")
    );
    assert_eq!(event.user_count, Some(2));

    let rule = event.recurrence_rule.as_ref().unwrap();
    assert_eq!(rule.end, None);
    assert_eq!(rule.frequency, GuildScheduledEventRecurrenceFrequency::DAILY);
    assert_eq!(rule.interval, 1);
    assert_eq!(
        rule.by_weekday.as_deref(),
        Some(
            &[
                GuildScheduledEventRecurrenceWeekday::MONDAY,
                GuildScheduledEventRecurrenceWeekday::TUESDAY,
                GuildScheduledEventRecurrenceWeekday::WEDNESDAY,
                GuildScheduledEventRecurrenceWeekday::THURSDAY,
                GuildScheduledEventRecurrenceWeekday::FRIDAY,
            ][..]
        )
    );
    assert_eq!(rule.by_n_weekday, None);

    let value = serde_json::to_value(event).unwrap();
    assert_eq!(value["channel_id"], serde_json::Value::Null);
    assert_eq!(value["scheduled_end_time"], serde_json::Value::Null);
    assert_eq!(value["entity_id"], serde_json::Value::Null);
    assert_eq!(value["recurrence_rule"]["by_n_weekday"], serde_json::Value::Null);
    assert!(value.get("creator_id").is_none());
    assert!(value.get("description").is_none());
    assert!(value.get("image").is_none());
    assert_eq!(value["entity_metadata"]["location"], "Test location");
}

/// Tests a forum channel with tags, overwrites, and required nullable fields.
#[test]
fn channel_decodes_forum_fields() {
    let channel: Channel = serde_json::from_str(
        r#"{
            "id": "1099032703076216842",
            "type": 15,
            "guild_id": "197038439483310086",
            "position": 4,
            "permission_overwrites": [{
                "id": "197038439483310086",
                "type": 0,
                "allow": "1024",
                "deny": "2048"
            }],
            "name": "forum",
            "topic": null,
            "nsfw": false,
            "last_message_id": null,
            "parent_id": "1099032703076216840",
            "rate_limit_per_user": 10,
            "flags": 131072,
            "available_tags": [{
                "id": "1099032703076216843",
                "name": "bug",
                "moderated": true,
                "emoji_id": null,
                "emoji_name": "🐛"
            }],
            "applied_tags": ["1099032703076216843"],
            "default_reaction_emoji": {
                "emoji_id": "989193655938064464",
                "emoji_name": null
            },
            "default_sort_order": 1,
            "default_forum_layout": 2
        }"#,
    )
    .unwrap();

    assert_eq!(channel.kind, ChannelType::GUILD_FORUM);
    assert!(channel.flags.unwrap().contains(ChannelFlags::CHANNEL_OBFUSCATED));
    assert_eq!(channel.last_message_id, None);
    assert_eq!(channel.name.as_deref(), Some("forum"));

    let overwrite = &channel.permission_overwrites.as_ref().unwrap()[0];
    assert_eq!(overwrite.allow.bits(), 1_024);
    assert_eq!(overwrite.deny.bits(), 2_048);

    let tags = channel.available_tags.as_ref().unwrap();
    assert_eq!(tags[0].name, "bug");
    assert!(tags[0].moderated);
    assert_eq!(tags[0].emoji_id, None);
    assert_eq!(tags[0].emoji_name.as_deref(), Some("🐛"));
    assert_eq!(channel.applied_tags.as_ref().unwrap()[0], tags[0].id);

    let reaction = channel.default_reaction_emoji.as_ref().unwrap();
    assert_eq!(reaction.emoji_id.unwrap().get(), 989_193_655_938_064_464);
    assert_eq!(reaction.emoji_name, None);
    assert_eq!(channel.default_forum_layout, Some(ForumLayout::GALLERY_VIEW));

    let value = serde_json::to_value(channel).unwrap();
    assert_eq!(value["type"], 15);
    assert_eq!(value["permission_overwrites"][0]["allow"], "1024");
    assert_eq!(value["default_reaction_emoji"]["emoji_id"], "989193655938064464");
    assert_eq!(value["default_reaction_emoji"]["emoji_name"], serde_json::Value::Null);
    assert!(value.get("topic").is_none());
}

/// Tests obfuscated gateway channels and unknown channel types.
#[test]
fn channel_supports_obfuscation_and_unknown_types() {
    let channel: Channel = serde_json::from_str(
        r#"{
            "id": "1099032703076216842",
            "type": 0,
            "position": 2,
            "parent_id": "1099032703076216840",
            "flags": 131072
        }"#,
    )
    .unwrap();

    assert_eq!(channel.kind, ChannelType::GUILD_TEXT);
    assert_eq!(channel.guild_id, None);
    assert_eq!(channel.name, None);
    assert_eq!(channel.permission_overwrites, None);
    assert!(channel.flags.unwrap().contains(ChannelFlags::CHANNEL_OBFUSCATED));

    let value = serde_json::to_value(channel).unwrap();
    assert_eq!(value["id"], "1099032703076216842");
    assert_eq!(value["type"], 0);
    assert_eq!(value["position"], 2);
    assert_eq!(value["parent_id"], "1099032703076216840");
    assert!(value.get("name").is_none());

    let unknown: Channel = serde_json::from_str(r#"{"id": "1", "type": 99}"#).unwrap();
    assert_eq!(serde_json::to_value(unknown).unwrap()["type"], 99);
}

/// Tests the available and unavailable shapes of a `GUILD_CREATE` payload.
#[test]
fn guild_create_distinguishes_available_and_unavailable() {
    let payload = r#"{
        "id": "197038439483310086",
        "name": "Test Guild",
        "icon": null,
        "splash": null,
        "discovery_splash": null,
        "owner_id": "73193882359173120",
        "afk_channel_id": null,
        "afk_timeout": 300,
        "verification_level": 0,
        "default_message_notifications": 0,
        "explicit_content_filter": 0,
        "roles": [],
        "emojis": [],
        "features": [],
        "mfa_level": 0,
        "application_id": null,
        "system_channel_id": null,
        "system_channel_flags": 0,
        "rules_channel_id": null,
        "vanity_url_code": null,
        "description": null,
        "banner": null,
        "premium_tier": 0,
        "preferred_locale": "en-US",
        "public_updates_channel_id": null,
        "nsfw_level": 0,
        "premium_progress_bar_enabled": false,
        "safety_alerts_channel_id": null,
        "incidents_data": null,
        "joined_at": "2023-09-01T14:48:02.222000+00:00",
        "large": false,
        "member_count": 1,
        "voice_states": [],
        "members": [],
        "channels": [{
            "id": "1099032703076216842",
            "type": 0,
            "guild_id": "197038439483310086"
        }],
        "threads": [],
        "presences": [],
        "stage_instances": [],
        "guild_scheduled_events": [],
        "soundboard_sounds": []
    }"#;

    let create: GuildCreate = serde_json::from_str(payload).unwrap();
    let GuildCreate::Available(available) = create else {
        panic!("expected an available guild");
    };

    assert_eq!(available.guild.id.get(), 197_038_439_483_310_086);
    assert_eq!(available.guild.name, "Test Guild");
    assert_eq!(available.joined_at, timestamp(1_693_579_682, 222_000_000));
    assert!(!available.large);
    assert_eq!(available.member_count, 1);
    assert_eq!(available.channels.len(), 1);
    assert_eq!(available.channels[0].kind, ChannelType::GUILD_TEXT);
    assert_eq!(available.channels[0].guild_id, Some(available.guild.id));

    let value = serde_json::to_value(GuildCreate::Available(available)).unwrap();
    assert_eq!(value["id"], "197038439483310086");
    assert_eq!(value["joined_at"], "2023-09-01T14:48:02.222+00:00");
    assert_eq!(value["channels"][0]["type"], 0);
    assert!(value.get("unavailable").is_none());

    let unavailable: GuildCreate =
        serde_json::from_str(r#"{"id": "41771983423143937", "unavailable": true}"#).unwrap();
    let GuildCreate::Unavailable(unavailable) = unavailable else {
        panic!("expected an unavailable guild");
    };

    assert_eq!(unavailable.id.get(), 41_771_983_423_143_937);
    assert!(unavailable.unavailable);
}

/// Tests the example invite object from Discord's invite documentation.
#[test]
fn invite_matches_discord_example() {
    let invite: Invite = serde_json::from_str(
        r#"{
            "type": 0,
            "code": "0vCdhLbwjZZTWZLD",
            "guild": {
                "id": "165176875973476352",
                "name": "CS:GO Fraggers Only",
                "splash": null,
                "banner": null,
                "description": "Very good description",
                "icon": null,
                "features": ["NEWS", "DISCOVERABLE"],
                "verification_level": 2,
                "vanity_url_code": null,
                "nsfw_level": 0,
                "premium_subscription_count": 5
            },
            "channel": {
                "id": "165176875973476352",
                "name": "illuminati",
                "type": 0
            },
            "inviter": {
                "id": "115590097100865541",
                "username": "speed",
                "avatar": "deadbeef",
                "discriminator": "7653",
                "public_flags": 131328
            },
            "target_type": 1,
            "target_user": {
                "id": "165176875973476352",
                "username": "bob",
                "avatar": "deadbeef",
                "discriminator": "1234",
                "public_flags": 64
            }
        }"#,
    )
    .unwrap();

    assert_eq!(invite.kind, InviteType::GUILD);
    assert_eq!(invite.code, "0vCdhLbwjZZTWZLD");

    let guild = invite.guild.as_ref().unwrap();
    assert_eq!(guild.id.get(), 165_176_875_973_476_352);
    assert_eq!(guild.name, "CS:GO Fraggers Only");
    assert_eq!(guild.icon, None);
    assert_eq!(guild.splash, None);
    assert_eq!(guild.banner, None);
    assert_eq!(guild.description.as_deref(), Some("Very good description"));
    assert_eq!(guild.features.len(), 2);
    assert_eq!(guild.features[0], GuildFeature::NEWS);
    assert_eq!(guild.verification_level, VerificationLevel::MEDIUM);
    assert_eq!(guild.vanity_url_code, None);
    assert_eq!(guild.nsfw_level, GuildAgeRestrictionLevel::NONE);
    assert_eq!(guild.premium_subscription_count, Some(5));

    let channel = invite.channel.as_ref().unwrap();
    assert_eq!(channel.id.get(), 165_176_875_973_476_352);
    assert_eq!(channel.name.as_deref(), Some("illuminati"));
    assert_eq!(channel.kind, Some(ChannelType::GUILD_TEXT));
    assert_eq!(channel.position, None);

    assert_eq!(invite.inviter.as_ref().unwrap().username, "speed");
    assert_eq!(invite.target_type, Some(InviteTargetType::STREAM));
    assert_eq!(invite.target_user.as_ref().unwrap().username, "bob");
    assert_eq!(invite.target_application, None);
    assert_eq!(invite.approximate_presence_count, None);
    assert_eq!(invite.approximate_member_count, None);
    assert_eq!(invite.expires_at, None);
    assert_eq!(invite.guild_scheduled_event, None);
    assert_eq!(invite.flags, None);
    assert_eq!(invite.roles, None);

    let value = serde_json::to_value(invite).unwrap();
    assert_eq!(value["type"], 0);
    assert_eq!(value["channel"]["type"], 0);
    assert_eq!(value["target_type"], 1);
    assert_eq!(value["guild"]["icon"], Value::Null);
    assert_eq!(value["guild"]["splash"], Value::Null);
    assert_eq!(value["guild"]["banner"], Value::Null);
    assert_eq!(value["guild"]["vanity_url_code"], Value::Null);
    assert_eq!(value["expires_at"], Value::Null);
    assert!(value.get("target_application").is_none());
    assert!(value.get("guild_scheduled_event").is_none());
    assert!(value.get("flags").is_none());
    assert!(value.get("roles").is_none());
}

/// Tests unknown invite values, nullable channel, flags, and partial roles.
#[test]
#[allow(deprecated)]
fn invite_accepts_unknown_values_and_partial_roles() {
    let invite: Invite = serde_json::from_str(
        r#"{
            "type": 99,
            "code": "abc",
            "channel": null,
            "expires_at": null,
            "flags": 3,
            "roles": [{
                "id": "41771983423143936",
                "name": "WE DEM BOYZZ!!!!!!",
                "position": 1,
                "color": 3447003,
                "colors": {
                    "primary_color": 3447003,
                    "secondary_color": null,
                    "tertiary_color": null
                },
                "icon": null,
                "unicode_emoji": null
            }]
        }"#,
    )
    .unwrap();

    assert_eq!(serde_json::to_value(invite.kind).unwrap(), serde_json::json!(99));
    assert_eq!(invite.channel, None);
    assert_eq!(invite.expires_at, None);

    let flags = invite.flags.unwrap();
    assert!(flags.contains(GuildInviteFlags::IS_GUEST_INVITE));
    assert_eq!(flags.bits(), 3);

    let role = &invite.roles.as_ref().unwrap()[0];
    assert_eq!(role.id.get(), 41_771_983_423_143_936);
    assert_eq!(role.name, "WE DEM BOYZZ!!!!!!");
    assert_eq!(role.position, 1);
    assert_eq!(role.color, 3_447_003);
    assert_eq!(
        role.colors,
        RoleColors {
            primary_color: 3_447_003,
            secondary_color: None,
            tertiary_color: None,
        }
    );
    assert_eq!(role.icon, None);
    assert_eq!(role.unicode_emoji, None);

    let value = serde_json::to_value(invite).unwrap();
    assert_eq!(value["type"], 99);
    assert_eq!(value["channel"], Value::Null);
    assert_eq!(value["expires_at"], Value::Null);
    assert_eq!(value["flags"], 3);
    assert_eq!(value["roles"][0]["colors"]["primary_color"], serde_json::json!(3_447_003));
    assert!(value["roles"][0].get("icon").is_none());
    assert!(value["roles"][0].get("unicode_emoji").is_none());

    let target: InviteTargetType = serde_json::from_str("9").unwrap();
    assert_eq!(serde_json::to_value(target).unwrap(), serde_json::json!(9));
}

/// Tests flattening an invite with the metadata fields returned by its endpoint.
#[test]
fn invite_with_metadata_flattens_both_objects() {
    let invite: InviteWithMetadata = serde_json::from_str(
        r#"{
            "type": 0,
            "code": "0vCdhLbwjZZTWZLD",
            "channel": {
                "id": "165176875973476352",
                "name": "illuminati",
                "type": 0
            },
            "guild": null,
            "uses": 5,
            "max_uses": 10,
            "max_age": 3600,
            "temporary": false,
            "created_at": "2016-03-31T19:15:39.954000+00:00"
        }"#,
    )
    .unwrap();

    assert_eq!(invite.invite.code, "0vCdhLbwjZZTWZLD");
    assert_eq!(invite.invite.guild, None);
    assert_eq!(invite.metadata.uses, 5);
    assert_eq!(invite.metadata.max_uses, 10);
    assert_eq!(invite.metadata.max_age, 3600);
    assert!(!invite.metadata.temporary);
    assert_eq!(invite.metadata.created_at, timestamp(1_459_451_739, 954_000_000));

    let value = serde_json::to_value(invite).unwrap();
    assert_eq!(value["code"], "0vCdhLbwjZZTWZLD");
    assert_eq!(value["uses"], 5);
    assert_eq!(value["created_at"], "2016-03-31T19:15:39.954+00:00");
    assert!(value.get("guild").is_none());
}

/// Tests the partial invite returned by the vanity URL endpoint.
#[test]
fn vanity_partial_invite_allows_null_code() {
    let invite: PartialInvite = serde_json::from_str(r#"{"code": "abc", "uses": 12}"#).unwrap();

    assert_eq!(invite.code.as_deref(), Some("abc"));
    assert_eq!(invite.uses, 12);

    let unset: PartialInvite = serde_json::from_str(r#"{"code": null, "uses": 0}"#).unwrap();

    assert_eq!(unset.code, None);
    assert_eq!(serde_json::to_value(unset).unwrap()["code"], Value::Null);
}

/// Tests the example guild template from Discord's guild template documentation.
#[test]
fn guild_template_matches_discord_example() {
    let template: GuildTemplate = serde_json::from_str(
        r#"{
            "code": "hgM48av5Q69A",
            "name": "Friends & Family",
            "description": "",
            "usage_count": 49605,
            "creator_id": "132837293881950208",
            "creator": {
                "id": "132837293881950208",
                "username": "hoges",
                "avatar": "79b0d9f8c340f2d43e1f78b09f175b62",
                "discriminator": "0001",
                "public_flags": 129
            },
            "created_at": "2020-04-02T21:10:38+00:00",
            "updated_at": "2020-05-01T17:57:38+00:00",
            "source_guild_id": "678070694164299796",
            "serialized_source_guild": {
                "name": "Friends & Family",
                "description": null,
                "region": "us-west",
                "verification_level": 0,
                "default_message_notifications": 0,
                "explicit_content_filter": 0,
                "preferred_locale": "en-US",
                "afk_timeout": 300,
                "roles": [{
                    "id": 0,
                    "name": "@everyone",
                    "permissions": 104324689,
                    "color": 0,
                    "hoist": false,
                    "mentionable": false
                }],
                "channels": [
                    {
                        "name": "Text Channels",
                        "position": 1,
                        "topic": null,
                        "bitrate": 64000,
                        "user_limit": 0,
                        "nsfw": false,
                        "rate_limit_per_user": 0,
                        "parent_id": null,
                        "permission_overwrites": [],
                        "id": 1,
                        "type": 4
                    },
                    {
                        "name": "general",
                        "position": 1,
                        "topic": null,
                        "bitrate": 64000,
                        "user_limit": 0,
                        "nsfw": false,
                        "rate_limit_per_user": 0,
                        "parent_id": 1,
                        "permission_overwrites": [],
                        "id": 2,
                        "type": 0
                    }
                ],
                "afk_channel_id": null,
                "system_channel_id": 2,
                "system_channel_flags": 0,
                "icon_hash": null
            },
            "is_dirty": null
        }"#,
    )
    .unwrap();

    assert_eq!(template.code, "hgM48av5Q69A");
    assert_eq!(template.name, "Friends & Family");
    assert_eq!(template.description.as_deref(), Some(""));
    assert_eq!(template.usage_count, 49_605);
    assert_eq!(template.creator_id.get(), 132_837_293_881_950_208);
    assert_eq!(template.creator.username, "hoges");
    assert_eq!(template.created_at, timestamp(1_585_861_838, 0));
    assert_eq!(template.updated_at, timestamp(1_588_355_858, 0));
    assert_eq!(template.source_guild_id.get(), 678_070_694_164_299_796);
    assert_eq!(template.is_dirty, None);

    let source = &template.serialized_source_guild;
    assert_eq!(source.name, "Friends & Family");
    assert_eq!(source.description, None);
    assert_eq!(source.region.as_deref(), Some("us-west"));
    assert_eq!(source.verification_level, VerificationLevel::NONE);
    assert_eq!(
        source.default_message_notifications,
        DefaultMessageNotificationLevel::ALL_MESSAGES
    );
    assert_eq!(source.explicit_content_filter, ExplicitContentFilterLevel::NONE);
    assert_eq!(source.preferred_locale, "en-US");
    assert_eq!(source.afk_timeout, 300);
    assert_eq!(source.roles.len(), 1);
    assert_eq!(source.roles[0].id, 0);
    assert_eq!(source.roles[0].name, "@everyone");
    assert_eq!(source.roles[0].permissions.bits(), 104_324_689);
    assert_eq!(source.roles[0].color, 0);
    assert!(!source.roles[0].hoist);
    assert!(!source.roles[0].mentionable);
    assert_eq!(source.channels.len(), 2);
    assert_eq!(source.channels[0].id, 1);
    assert_eq!(source.channels[0].name, "Text Channels");
    assert_eq!(source.channels[0].kind, ChannelType::GUILD_CATEGORY);
    assert_eq!(source.channels[0].parent_id, None);
    assert_eq!(source.channels[0].topic, None);
    assert_eq!(source.channels[1].id, 2);
    assert_eq!(source.channels[1].kind, ChannelType::GUILD_TEXT);
    assert_eq!(source.channels[1].parent_id, Some(1));
    assert_eq!(source.afk_channel_id, None);
    assert_eq!(source.system_channel_id, Some(2));
    assert_eq!(source.system_channel_flags, SystemChannelFlags::empty());

    let value = serde_json::to_value(template).unwrap();
    assert_eq!(value["created_at"], "2020-04-02T21:10:38+00:00");
    assert_eq!(value["is_dirty"], Value::Null);
    assert_eq!(value["serialized_source_guild"]["description"], Value::Null);
    assert_eq!(value["serialized_source_guild"]["afk_channel_id"], Value::Null);
    assert_eq!(value["serialized_source_guild"]["roles"][0]["permissions"], "104324689");
    assert_eq!(value["serialized_source_guild"]["channels"][0]["id"], 1);
    assert_eq!(value["serialized_source_guild"]["channels"][1]["parent_id"], 1);
    assert_eq!(value["serialized_source_guild"]["channels"][1]["type"], 0);
}

/// Tests the example guild preview from Discord's guild documentation.
#[test]
fn guild_preview_matches_discord_example() {
    let preview: GuildPreview = serde_json::from_str(
        r#"{
            "id": "197038439483310086",
            "name": "Discord Testers",
            "icon": "f64c482b807da4f539cff778d174971c",
            "splash": null,
            "discovery_splash": null,
            "emojis": [],
            "features": [
                "DISCOVERABLE",
                "VANITY_URL",
                "ANIMATED_ICON",
                "INVITE_SPLASH",
                "NEWS",
                "COMMUNITY",
                "BANNER",
                "VERIFIED",
                "MORE_EMOJI"
            ],
            "approximate_member_count": 60814,
            "approximate_presence_count": 20034,
            "description": "The official place to report Discord Bugs!",
            "stickers": []
        }"#,
    )
    .unwrap();

    assert_eq!(preview.id.get(), 197_038_439_483_310_086);
    assert_eq!(preview.name, "Discord Testers");
    assert_eq!(preview.icon.as_deref(), Some("f64c482b807da4f539cff778d174971c"));
    assert_eq!(preview.splash, None);
    assert_eq!(preview.discovery_splash, None);
    assert!(preview.emojis.is_empty());
    assert_eq!(preview.features.len(), 9);
    assert_eq!(preview.features[0], GuildFeature::DISCOVERABLE);
    assert_eq!(preview.approximate_member_count, 60_814);
    assert_eq!(preview.approximate_presence_count, 20_034);
    assert_eq!(
        preview.description.as_deref(),
        Some("The official place to report Discord Bugs!")
    );
    assert!(preview.stickers.is_empty());

    let value = serde_json::to_value(preview).unwrap();
    assert_eq!(value["splash"], Value::Null);
    assert_eq!(value["discovery_splash"], Value::Null);
    assert_eq!(value["description"], "The official place to report Discord Bugs!");
}

/// Tests the guild widget settings and widget examples.
#[test]
fn guild_widget_settings_and_widget_match_discord_examples() {
    let settings: GuildWidgetSettings =
        serde_json::from_str(r#"{"enabled": true, "channel_id": "41771983444115456"}"#).unwrap();

    assert!(settings.enabled);
    assert_eq!(settings.channel_id.unwrap().get(), 41_771_983_444_115_456);

    let disabled: GuildWidgetSettings =
        serde_json::from_str(r#"{"enabled": false, "channel_id": null}"#).unwrap();

    assert!(!disabled.enabled);
    assert_eq!(serde_json::to_value(disabled).unwrap()["channel_id"], Value::Null);

    let widget: GuildWidget = serde_json::from_str(
        r#"{
            "id": "290926798626999250",
            "name": "Test Server",
            "instant_invite": "https://discord.com/invite/abcdefg",
            "channels": [
                {
                    "id": "705216630279993882",
                    "name": "elephant",
                    "position": 2
                },
                {
                    "id": "669583461748992190",
                    "name": "groovy-music",
                    "position": 1
                }
            ],
            "members": [{
                "id": "0",
                "username": "1234",
                "discriminator": "0000",
                "avatar": null,
                "status": "online",
                "avatar_url": "https://cdn.discordapp.com/widget-avatars/example"
            }],
            "presence_count": 1
        }"#,
    )
    .unwrap();

    assert_eq!(widget.id.get(), 290_926_798_626_999_250);
    assert_eq!(widget.name, "Test Server");
    assert_eq!(widget.instant_invite.as_deref(), Some("https://discord.com/invite/abcdefg"));
    assert_eq!(widget.channels.len(), 2);
    assert_eq!(widget.channels[0].name.as_deref(), Some("elephant"));
    assert_eq!(widget.channels[0].kind, None);
    assert_eq!(widget.channels[0].position, Some(2));
    assert_eq!(widget.members.len(), 1);

    let member = &widget.members[0];
    assert_eq!(member.id, "0");
    assert_eq!(member.username, "1234");
    assert_eq!(member.discriminator, "0000");
    assert_eq!(member.avatar, None);
    assert_eq!(member.status, UserStatus::ONLINE);
    assert_eq!(member.avatar_url, "https://cdn.discordapp.com/widget-avatars/example");

    let value = serde_json::to_value(widget).unwrap();
    assert_eq!(value["members"][0]["id"], "0");
    assert_eq!(value["members"][0]["avatar"], Value::Null);
    assert_eq!(value["members"][0]["status"], "online");
    assert!(value["channels"][0].get("type").is_none());
}

/// Tests the example guild onboarding object, including its omitted mode field.
#[test]
fn guild_onboarding_matches_discord_example() {
    let onboarding: GuildOnboarding = serde_json::from_str(
        r#"{
            "guild_id": "960007075288915998",
            "prompts": [{
                "id": "1067461047608422473",
                "title": "What do you want to do in this community?",
                "options": [
                    {
                        "id": "1067461047608422476",
                        "title": "Chat with Friends",
                        "description": "",
                        "emoji": {
                            "id": "1070002302032826408",
                            "name": "chat",
                            "animated": false
                        },
                        "role_ids": [],
                        "channel_ids": ["962007075288916001"]
                    },
                    {
                        "id": "1070004843541954678",
                        "title": "Get Gud",
                        "description": "We have excellent teachers!",
                        "emoji": {
                            "id": null,
                            "name": "😀",
                            "animated": false
                        },
                        "role_ids": ["982014491980083211"],
                        "channel_ids": []
                    }
                ],
                "single_select": false,
                "required": false,
                "in_onboarding": true,
                "type": 0
            }],
            "default_channel_ids": [
                "998678771706110023",
                "998678693058719784"
            ],
            "enabled": true
        }"#,
    )
    .unwrap();

    assert_eq!(onboarding.guild_id.get(), 960_007_075_288_915_998);
    assert_eq!(onboarding.default_channel_ids.len(), 2);
    assert!(onboarding.enabled);
    assert_eq!(onboarding.mode, None);
    assert_eq!(onboarding.prompts.len(), 1);

    let prompt = &onboarding.prompts[0];
    assert_eq!(prompt.id.get(), 1_067_461_047_608_422_473);
    assert_eq!(prompt.kind, OnboardingPromptType::MULTIPLE_CHOICE);
    assert_eq!(prompt.title, "What do you want to do in this community?");
    assert!(!prompt.single_select);
    assert!(!prompt.required);
    assert!(prompt.in_onboarding);
    assert_eq!(prompt.options.len(), 2);

    let option = &prompt.options[0];
    assert_eq!(option.id.get(), 1_067_461_047_608_422_476);
    assert_eq!(option.title, "Chat with Friends");
    assert_eq!(option.description.as_deref(), Some(""));
    assert_eq!(option.channel_ids[0].get(), 962_007_075_288_916_001);
    assert!(option.role_ids.is_empty());
    let emoji = option.emoji.as_ref().unwrap();
    assert_eq!(emoji.id.map(|id| id.get()), Some(1_070_002_302_032_826_408));
    assert_eq!(emoji.name.as_deref(), Some("chat"));
    assert_eq!(emoji.animated, Some(false));

    let value = serde_json::to_value(onboarding).unwrap();
    assert!(value.get("mode").is_none());
    assert_eq!(value["prompts"][0]["type"], 0);
    assert_eq!(value["prompts"][0]["options"][0]["description"], "");
    assert_eq!(value["prompts"][0]["options"][0]["emoji"]["id"], "1070002302032826408");
    assert!(value["prompts"][0]["options"][0].get("emoji_id").is_none());

    let with_mode: GuildOnboarding = serde_json::from_str(
        r#"{
            "guild_id": "1",
            "prompts": [],
            "default_channel_ids": [],
            "enabled": false,
            "mode": 1
        }"#,
    )
    .unwrap();

    assert_eq!(with_mode.mode, Some(OnboardingMode::ONBOARDING_ADVANCED));

    let unknown: GuildOnboarding = serde_json::from_str(
        r#"{
            "guild_id": "1",
            "prompts": [],
            "default_channel_ids": [],
            "enabled": false,
            "mode": 7
        }"#,
    )
    .unwrap();

    assert_eq!(serde_json::to_value(unknown).unwrap()["mode"], 7);
}

/// Tests optional and nullable prompt option fields.
#[test]
fn onboarding_prompt_option_handles_emoji_variants() {
    let option: OnboardingPromptOption = serde_json::from_str(
        r#"{
            "id": "2",
            "channel_ids": [],
            "role_ids": [],
            "emoji_id": "989193655938064464",
            "emoji_name": "🐛",
            "emoji_animated": false,
            "title": "Bug reports",
            "description": null
        }"#,
    )
    .unwrap();

    assert_eq!(option.emoji, None);
    assert_eq!(option.emoji_id.unwrap().get(), 989_193_655_938_064_464);
    assert_eq!(option.emoji_name.as_deref(), Some("🐛"));
    assert_eq!(option.emoji_animated, Some(false));
    assert_eq!(option.description, None);

    let value = serde_json::to_value(option).unwrap();
    assert_eq!(value["emoji_id"], "989193655938064464");
    assert_eq!(value["emoji_name"], "🐛");
    assert_eq!(value["description"], Value::Null);
    assert!(value.get("emoji").is_none());
}

/// Tests the incoming webhook example and its nullable fields.
#[test]
fn incoming_webhook_matches_discord_example() {
    let webhook: Webhook = serde_json::from_str(
        r#"{
            "name": "test webhook",
            "type": 1,
            "channel_id": "199737254929760256",
            "token": "3d89bb7572e0fb30d8128367b3b1b44fecd1726de135cbe28a41f8b2f777c372ba2939e72279b94526ff5d1bd4358d65cf11",
            "avatar": null,
            "guild_id": "199737254929760256",
            "id": "223704706495545344",
            "application_id": null,
            "user": {
                "username": "test",
                "discriminator": "7479",
                "id": "190320984123768832",
                "avatar": "b004ec1740a63ca06ae2e14c5cee11f3",
                "public_flags": 131328
            }
        }"#,
    )
    .unwrap();

    assert_eq!(webhook.id.get(), 223_704_706_495_545_344);
    assert_eq!(webhook.kind, WebhookType::INCOMING);
    assert_eq!(webhook.channel_id.unwrap().get(), 199_737_254_929_760_256);
    assert_eq!(webhook.guild_id.unwrap().get(), 199_737_254_929_760_256);
    assert_eq!(webhook.name.as_deref(), Some("test webhook"));
    assert_eq!(webhook.avatar, None);
    assert_eq!(webhook.application_id, None);
    assert_eq!(webhook.user.as_ref().unwrap().username, "test");
    assert!(webhook.token.is_some());
    assert_eq!(webhook.source_guild, None);
    assert_eq!(webhook.source_channel, None);
    assert_eq!(webhook.url, None);

    let value = serde_json::to_value(webhook).unwrap();
    assert_eq!(value["type"], 1);
    assert_eq!(value["channel_id"], "199737254929760256");
    assert_eq!(value["guild_id"], "199737254929760256");
    assert_eq!(value["avatar"], Value::Null);
    assert_eq!(value["application_id"], Value::Null);
    assert!(value.get("source_guild").is_none());
    assert!(value.get("source_channel").is_none());
    assert!(value.get("url").is_none());
}

/// Tests channel follower and application webhook examples.
#[test]
fn webhook_types_match_discord_examples() {
    let follower: Webhook = serde_json::from_str(
        r#"{
            "type": 2,
            "id": "752831914402115456",
            "name": "Guildy name",
            "avatar": "bb71f469c158984e265093a81b3397fb",
            "channel_id": "561885260615255432",
            "guild_id": "56188498421443265",
            "application_id": null,
            "source_guild": {
                "id": "56188498421476534",
                "name": "Guildy name",
                "icon": "bb71f469c158984e265093a81b3397fb"
            },
            "source_channel": {
                "id": "5618852344134324",
                "name": "announcements"
            },
            "user": {
                "username": "test",
                "discriminator": "7479",
                "id": "190320984123768832",
                "avatar": "b004ec1740a63ca06ae2e14c5cee11f3",
                "public_flags": 131328
            }
        }"#,
    )
    .unwrap();

    assert_eq!(follower.kind, WebhookType::CHANNEL_FOLLOWER);
    let source_guild = follower.source_guild.as_ref().unwrap();
    assert_eq!(source_guild.id.get(), 56_188_498_421_476_534);
    assert_eq!(source_guild.name, "Guildy name");
    assert_eq!(source_guild.icon.as_deref(), Some("bb71f469c158984e265093a81b3397fb"));

    let source_channel = follower.source_channel.as_ref().unwrap();
    assert_eq!(source_channel.id.get(), 5_618_852_344_134_324);
    assert_eq!(source_channel.name.as_deref(), Some("announcements"));
    assert_eq!(source_channel.kind, None);

    let value = serde_json::to_value(follower).unwrap();
    assert_eq!(value["source_guild"]["id"], "56188498421476534");
    assert_eq!(value["source_channel"]["name"], "announcements");
    assert!(value["source_channel"].get("type").is_none());

    let application: Webhook = serde_json::from_str(
        r#"{
            "type": 3,
            "id": "658822586720976555",
            "name": "Clyde",
            "avatar": "689161dc90ac261d00f1608694ac6bfd",
            "channel_id": null,
            "guild_id": null,
            "application_id": "658822586720976555"
        }"#,
    )
    .unwrap();

    assert_eq!(application.kind, WebhookType::APPLICATION);
    assert_eq!(application.channel_id, None);
    assert_eq!(application.guild_id, None);
    assert_eq!(application.application_id.unwrap().get(), 658_822_586_720_976_555);
    assert_eq!(application.user, None);
    assert_eq!(application.token, None);

    let value = serde_json::to_value(application).unwrap();
    assert_eq!(value["channel_id"], Value::Null);
    assert_eq!(value["application_id"], "658822586720976555");
    assert!(value.get("guild_id").is_none());
    assert!(value.get("user").is_none());

    let unknown: Webhook = serde_json::from_str(
        r#"{
            "id": "1",
            "type": 9,
            "channel_id": null,
            "name": null,
            "avatar": null,
            "application_id": null
        }"#,
    )
    .unwrap();

    assert_eq!(serde_json::to_value(unknown).unwrap()["type"], 9);
}

/// Tests that missing nullable webhook fields are serialized as `null`.
#[test]
fn webhook_serializes_nullable_core_fields_as_null() {
    let webhook: Webhook = serde_json::from_str(
        r#"{
            "id": "1",
            "type": 1,
            "avatar": null,
            "application_id": null
        }"#,
    )
    .unwrap();

    assert_eq!(webhook.channel_id, None);
    assert_eq!(webhook.name, None);
    assert_eq!(webhook.avatar, None);

    let value = serde_json::to_value(webhook).unwrap();
    assert_eq!(value["channel_id"], Value::Null);
    assert_eq!(value["name"], Value::Null);
    assert_eq!(value["avatar"], Value::Null);
    assert_eq!(value["application_id"], Value::Null);
}

/// Tests the example auto moderation rule from Discord's documentation.
#[test]
fn auto_moderation_rule_matches_discord_example() {
    let rule: AutoModerationRule = serde_json::from_str(
        r#"{
            "id": "969707018069872670",
            "guild_id": "613425648685547541",
            "name": "Keyword Filter 1",
            "creator_id": "423457898095789043",
            "trigger_type": 1,
            "event_type": 1,
            "actions": [
                {
                    "type": 1,
                    "metadata": {
                        "custom_message": "Please keep financial discussions limited to the #finance channel"
                    }
                },
                {
                    "type": 2,
                    "metadata": {
                        "channel_id": "123456789123456789"
                    }
                },
                {
                    "type": 3,
                    "metadata": {
                        "duration_seconds": 60
                    }
                }
            ],
            "trigger_metadata": {
                "keyword_filter": ["cat*", "*dog", "*ana*", "i like c++"],
                "regex_patterns": ["(b|c)at", "^(?:[0-9]{1,3}\\.){3}[0-9]{1,3}$"]
            },
            "enabled": true,
            "exempt_roles": ["323456789123456789", "423456789123456789"],
            "exempt_channels": ["523456789123456789"]
        }"#,
    )
    .unwrap();

    assert_eq!(rule.id.get(), 969_707_018_069_872_670);
    assert_eq!(rule.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(rule.name, "Keyword Filter 1");
    assert_eq!(rule.creator_id.get(), 423_457_898_095_789_043);
    assert_eq!(rule.trigger_type, AutoModerationTriggerType::KEYWORD);
    assert_eq!(rule.event_type, AutoModerationEventType::MESSAGE_SEND);
    assert!(rule.enabled);
    assert_eq!(rule.exempt_roles.len(), 2);
    assert_eq!(rule.exempt_roles[0].get(), 323_456_789_123_456_789);
    assert_eq!(rule.exempt_channels[0].get(), 523_456_789_123_456_789);

    let metadata = &rule.trigger_metadata;
    let keywords = metadata.keyword_filter.as_ref().unwrap();
    assert_eq!(keywords.len(), 4);
    assert_eq!(keywords[3], "i like c++");
    assert_eq!(metadata.regex_patterns.as_ref().unwrap().len(), 2);
    assert_eq!(metadata.presets, None);
    assert_eq!(metadata.allow_list, None);
    assert_eq!(metadata.mention_total_limit, None);
    assert_eq!(metadata.mention_raid_protection_enabled, None);

    assert_eq!(rule.actions.len(), 3);
    assert_eq!(rule.actions[0].kind, AutoModerationActionType::BLOCK_MESSAGE);
    assert_eq!(
        rule.actions[0].metadata.as_ref().unwrap().custom_message.as_deref(),
        Some("Please keep financial discussions limited to the #finance channel")
    );
    assert_eq!(rule.actions[1].kind, AutoModerationActionType::SEND_ALERT_MESSAGE);
    assert_eq!(
        rule.actions[1].metadata.as_ref().unwrap().channel_id.unwrap().get(),
        123_456_789_123_456_789
    );
    assert_eq!(rule.actions[2].kind, AutoModerationActionType::TIMEOUT);
    assert_eq!(rule.actions[2].metadata.as_ref().unwrap().duration_seconds, Some(60));

    let value = serde_json::to_value(rule).unwrap();
    assert_eq!(value["trigger_type"], 1);
    assert_eq!(
        value["actions"][0]["metadata"]["custom_message"],
        "Please keep financial discussions limited to the #finance channel"
    );
    assert_eq!(value["actions"][1]["metadata"]["channel_id"], "123456789123456789");
    assert!(value["trigger_metadata"].get("presets").is_none());
    assert_eq!(value["exempt_roles"][0], "323456789123456789");
}

/// Tests type-dependent auto moderation metadata and unknown values.
#[test]
fn auto_moderation_metadata_and_values_are_optional() {
    let rule: AutoModerationRule = serde_json::from_str(
        r#"{
            "id": "1",
            "guild_id": "2",
            "name": "Spam rule",
            "creator_id": "3",
            "event_type": 2,
            "trigger_type": 3,
            "trigger_metadata": {},
            "actions": [{"type": 4}],
            "enabled": false,
            "exempt_roles": [],
            "exempt_channels": []
        }"#,
    )
    .unwrap();

    assert_eq!(rule.event_type, AutoModerationEventType::MEMBER_UPDATE);
    assert_eq!(rule.trigger_type, AutoModerationTriggerType::SPAM);
    assert_eq!(rule.trigger_metadata, Default::default());
    assert_eq!(rule.actions[0].kind, AutoModerationActionType::BLOCK_MEMBER_INTERACTION);
    assert_eq!(rule.actions[0].metadata, None);

    let value = serde_json::to_value(rule).unwrap();
    assert_eq!(value["actions"][0], serde_json::json!({"type": 4}));
    assert_eq!(value["trigger_metadata"], serde_json::json!({}));

    let unknown: AutoModerationRule = serde_json::from_str(
        r#"{
            "id": "1",
            "guild_id": "2",
            "name": "Future rule",
            "creator_id": "3",
            "event_type": 9,
            "trigger_type": 9,
            "trigger_metadata": {"presets": [9]},
            "actions": [],
            "enabled": true,
            "exempt_roles": [],
            "exempt_channels": []
        }"#,
    )
    .unwrap();

    let value = serde_json::to_value(unknown).unwrap();
    assert_eq!(value["event_type"], 9);
    assert_eq!(value["trigger_type"], 9);
    assert_eq!(value["trigger_metadata"]["presets"][0], 9);

    let preset: KeywordPresetType = serde_json::from_str("9").unwrap();
    assert_eq!(serde_json::to_value(preset).unwrap(), serde_json::json!(9));
}

/// Tests the partial application embedded in an embedded-application invite.
#[test]
fn invite_target_application_decodes_partial_application() {
    let invite: Invite = serde_json::from_str(
        r#"{
            "type": 0,
            "code": "abc",
            "channel": null,
            "expires_at": null,
            "target_type": 2,
            "target_application": {
                "id": "880218394199220334",
                "name": "Watch Together",
                "icon": "e5c1e5f7c9c8f7d7f7f7f7f7f7f7f7f7",
                "description": "Watch YouTube together",
                "cover_image": "f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7",
                "primary_sku_id": "880218832743055411",
                "slug": "watch-together",
                "summary": "An unmodeled field"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(invite.target_type, Some(InviteTargetType::EMBEDDED_APPLICATION));
    let application = invite.target_application.as_ref().unwrap();
    assert_eq!(application.id.get(), 880_218_394_199_220_334);
    assert_eq!(application.name, "Watch Together");
    assert_eq!(application.icon.as_deref(), Some("e5c1e5f7c9c8f7d7f7f7f7f7f7f7f7f7"));
    assert_eq!(application.description.as_deref(), Some("Watch YouTube together"));
    assert_eq!(application.primary_sku_id.unwrap().get(), 880_218_832_743_055_411);
    assert_eq!(application.slug.as_deref(), Some("watch-together"));

    let value = serde_json::to_value(invite).unwrap();
    assert_eq!(value["target_application"]["primary_sku_id"], "880218832743055411");
    assert_eq!(value["target_type"], 2);
}

/// Tests the action row and string select examples from Discord's component
/// reference.
#[test]
fn component_action_row_matches_discord_example() {
    let component: Component = serde_json::from_str(
        r#"{
            "type": 1,
            "id": 1,
            "components": [
                {
                    "type": 3,
                    "id": 2,
                    "custom_id": "favorite_bug",
                    "placeholder": "Favorite bug?",
                    "options": [
                        {
                            "label": "Ant",
                            "value": "ant",
                            "description": "(best option)",
                            "emoji": {"name": "🐜"}
                        },
                        {"label": "Butterfly", "value": "butterfly", "emoji": {"name": "🦋"}},
                        {"label": "Caterpillar", "value": "caterpillar", "emoji": {"name": "🐛"}}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    let Component::ActionRow(row) = component else {
        panic!("expected an action row");
    };
    assert_eq!(row.kind, ComponentType::ACTION_ROW);
    assert_eq!(row.id, Some(1));
    assert_eq!(row.components.len(), 1);

    let Component::StringSelect(select) = &row.components[0] else {
        panic!("expected a string select");
    };
    assert_eq!(select.kind, ComponentType::STRING_SELECT);
    assert_eq!(select.id, Some(2));
    assert_eq!(select.custom_id, "favorite_bug");
    assert_eq!(select.placeholder.as_deref(), Some("Favorite bug?"));
    assert_eq!(select.options.len(), 3);
    assert_eq!(select.options[0].emoji.as_ref().unwrap().name.as_deref(), Some("🐜"));

    let value = serde_json::to_value(&row).unwrap();
    assert_eq!(value["type"], 1);
    assert_eq!(value["id"], 1);
    assert_eq!(value["components"][0]["type"], 3);
    assert!(value["components"][0].get("required").is_none());
    assert!(value["components"][0].get("disabled").is_none());
}

/// Tests the layout and content components introduced with components v2.
#[test]
fn component_layout_components_round_trip() {
    let component: Component = serde_json::from_str(
        r#"{
            "type": 17,
            "accent_color": 7419530,
            "spoiler": false,
            "components": [
                {
                    "type": 9,
                    "components": [{"type": 10, "content": "**Hello**"}],
                    "accessory": {
                        "type": 11,
                        "media": {"url": "https://example.com/thumb.png"},
                        "description": "Alternative text"
                    }
                },
                {
                    "type": 12,
                    "items": [
                        {
                            "media": {"url": "attachment://image.png"},
                            "spoiler": true
                        }
                    ]
                },
                {"type": 14, "divider": true, "spacing": 2},
                {
                    "type": 13,
                    "file": {"url": "attachment://notes.txt"},
                    "name": "notes.txt",
                    "size": 10
                }
            ]
        }"#,
    )
    .unwrap();

    let Component::Container(container) = component else {
        panic!("expected a container");
    };
    assert_eq!(container.kind, ComponentType::CONTAINER);
    assert_eq!(container.accent_color, Some(7_419_530));
    assert_eq!(container.spoiler, Some(false));
    assert_eq!(container.components.len(), 4);

    let Component::Section(section) = &container.components[0] else {
        panic!("expected a section");
    };
    let Component::TextDisplay(text) = &section.components[0] else {
        panic!("expected a text display");
    };
    assert_eq!(text.content, "**Hello**");
    let Component::Thumbnail(thumbnail) = section.accessory.as_ref() else {
        panic!("expected a thumbnail accessory");
    };
    assert_eq!(thumbnail.media.url, "https://example.com/thumb.png");
    assert_eq!(thumbnail.description.as_deref(), Some("Alternative text"));

    let Component::MediaGallery(gallery) = &container.components[1] else {
        panic!("expected a media gallery");
    };
    assert_eq!(gallery.items[0].media.url, "attachment://image.png");
    assert_eq!(gallery.items[0].spoiler, Some(true));

    let Component::Separator(separator) = &container.components[2] else {
        panic!("expected a separator");
    };
    assert_eq!(separator.divider, Some(true));
    assert_eq!(separator.spacing.unwrap().raw(), 2);

    let Component::File(file) = &container.components[3] else {
        panic!("expected a file component");
    };
    assert_eq!(file.file.url, "attachment://notes.txt");
    assert_eq!(file.name.as_deref(), Some("notes.txt"));
    assert_eq!(file.size, Some(10));

    let value = serde_json::to_value(&container).unwrap();
    assert_eq!(value["type"], 17);
    assert_eq!(value["components"][2]["spacing"], 2);
    assert_eq!(value["components"][3]["file"]["url"], "attachment://notes.txt");
}

/// Tests that component types Cordion does not model survive a round trip.
#[test]
fn component_preserves_unknown_types() {
    let payload = serde_json::json!({
        "type": 999,
        "id": 5,
        "future": {"nested": [1, 2]}
    });
    let component: Component = serde_json::from_value(payload.clone()).unwrap();

    assert!(matches!(component, Component::Unknown(_)));
    assert_eq!(serde_json::to_value(component).unwrap(), payload);

    assert_eq!(ComponentType::from_raw(200).raw(), 200);
    assert!(ComponentType::is_known_raw(23));
    assert!(!ComponentType::is_known_raw(200));

    let response: ComponentResponse =
        serde_json::from_str(r#"{"type": 999, "id": 1, "future": true}"#).unwrap();
    assert!(matches!(response, ComponentResponse::Unknown(_)));
}

/// Tests that a known component type with an unexpected shape is preserved
/// rather than failing the whole message.
#[test]
fn component_known_type_with_wrong_shape_becomes_unknown() {
    let component: Component =
        serde_json::from_str(r#"{"type": 2, "label": "Missing style"}"#).unwrap();

    assert!(matches!(component, Component::Unknown(_)));
}

/// Tests channel select default values and their typed string tags.
#[test]
fn component_channel_select_uses_typed_default_values() {
    let component: Component = serde_json::from_str(
        r#"{
            "type": 8,
            "id": 4,
            "custom_id": "channels",
            "channel_types": [0, 5],
            "default_values": [{"id": "1081000000000000021", "type": "channel"}],
            "max_values": 2
        }"#,
    )
    .unwrap();

    let Component::ChannelSelect(select) = component else {
        panic!("expected a channel select");
    };
    assert_eq!(select.channel_types.as_ref().unwrap()[1], ChannelType::GUILD_ANNOUNCEMENT);
    let Some(SelectDefaultValue::Channel { id }) = select.default_values.as_ref().unwrap().first()
    else {
        panic!("expected a channel default value");
    };
    assert_eq!(id.get(), 1_081_000_000_000_000_021);

    let value = serde_json::to_value(&select).unwrap();
    assert_eq!(value["default_values"][0]["type"], "channel");
    assert_eq!(value["default_values"][0]["id"], "1081000000000000021");
}

/// Tests the modal submit payload shape from Discord's component reference.
#[test]
fn modal_submit_interaction_decodes_component_responses() {
    let interaction: Interaction = serde_json::from_str(
        r#"{
            "id": "1081000000000000001",
            "application_id": "1081000000000000002",
            "type": 5,
            "token": "modal-token",
            "version": 1,
            "locale": "en-US",
            "app_permissions": "442368",
            "data": {
                "custom_id": "profile_modal",
                "components": [
                    {
                        "type": 18,
                        "id": 1,
                        "component": {
                            "type": 4,
                            "id": 2,
                            "custom_id": "name",
                            "value": "Cordion"
                        }
                    },
                    {"type": 3, "id": 3, "custom_id": "bug", "values": ["ant"]},
                    {
                        "type": 5,
                        "id": 4,
                        "custom_id": "users",
                        "values": ["117737874482294784"],
                        "resolved": {
                            "users": {
                                "117737874482294784": {
                                    "id": "117737874482294784",
                                    "username": "VoltyDemo",
                                    "discriminator": "7302",
                                    "bot": true,
                                    "public_flags": 0
                                }
                            },
                            "members": {
                                "117737874482294784": {
                                    "roles": [],
                                    "joined_at": "2021-02-12T18:25:07.972000+00:00",
                                    "pending": false,
                                    "nick": null,
                                    "permissions": "246997699136",
                                    "premium_since": null
                                }
                            },
                            "channels": {
                                "1081000000000000020": {
                                    "id": "1081000000000000020",
                                    "type": 0,
                                    "name": "general",
                                    "permissions": "1024"
                                }
                            }
                        }
                    },
                    {"type": 21, "id": 5, "custom_id": "size", "value": null},
                    {"type": 22, "id": 6, "custom_id": "toppings", "values": []},
                    {"type": 23, "id": 7, "custom_id": "agree", "value": true},
                    {"type": 19, "id": 8, "custom_id": "files", "values": []},
                    {"type": 10, "id": 9}
                ]
            }
        }"#,
    )
    .unwrap();

    assert_eq!(interaction.kind, InteractionType::MODAL_SUBMIT);
    assert_eq!(interaction.id.get(), 1_081_000_000_000_000_001);
    assert_eq!(interaction.version, 1);

    let Some(InteractionData::ModalSubmit(data)) = &interaction.data else {
        panic!("expected modal submit data");
    };
    assert_eq!(data.custom_id, "profile_modal");
    assert_eq!(data.components.len(), 8);

    let ComponentResponse::Label { id, component, .. } = &data.components[0] else {
        panic!("expected a label response");
    };
    assert_eq!(*id, 1);
    let ComponentResponse::TextInput { custom_id, value, .. } = component.as_ref() else {
        panic!("expected a text input response");
    };
    assert_eq!(custom_id, "name");
    assert_eq!(value, "Cordion");

    let ComponentResponse::StringSelect { values, .. } = &data.components[1] else {
        panic!("expected a string select response");
    };
    assert_eq!(values, &["ant".to_owned()]);

    let ComponentResponse::UserSelect { values, resolved, .. } = &data.components[2] else {
        panic!("expected a user select response");
    };
    assert_eq!(values[0].get(), 117_737_874_482_294_784);
    let resolved = resolved.as_ref().unwrap();
    let member = resolved.members.as_ref().unwrap().values().next().unwrap();
    assert_eq!(member.joined_at, Some(timestamp(1_613_154_307, 972_000_000)));
    assert_eq!(member.permissions.unwrap().bits(), 246_997_699_136);
    let channel = resolved.channels.as_ref().unwrap().values().next().unwrap();
    assert_eq!(channel.name.as_deref(), Some("general"));
    assert_eq!(channel.permissions.unwrap().bits(), 1024);

    let ComponentResponse::RadioGroup { value, .. } = &data.components[3] else {
        panic!("expected a radio group response");
    };
    assert_eq!(*value, None);

    let ComponentResponse::CheckboxGroup { values, .. } = &data.components[4] else {
        panic!("expected a checkbox group response");
    };
    assert!(values.is_empty());

    let ComponentResponse::Checkbox { value, .. } = &data.components[5] else {
        panic!("expected a checkbox response");
    };
    assert!(*value);

    let ComponentResponse::FileUpload { values, .. } = &data.components[6] else {
        panic!("expected a file upload response");
    };
    assert!(values.is_empty());

    let ComponentResponse::TextDisplay { id, .. } = &data.components[7] else {
        panic!("expected a text display response");
    };
    assert_eq!(*id, 9);

    let value = serde_json::to_value(&interaction).unwrap();
    assert_eq!(value["data"]["components"][0]["type"], 18);
    assert_eq!(value["data"]["components"][0]["component"]["type"], 4);
    assert_eq!(value["data"]["components"][3]["value"], Value::Null);
    assert_eq!(
        value["data"]["components"][2]["resolved"]["members"]["117737874482294784"]["joined_at"],
        "2021-02-12T18:25:07.972+00:00"
    );
}

/// Tests the slash command interaction example from Discord's application
/// command documentation.
#[test]
fn application_command_interaction_matches_discord_example() {
    let interaction: Interaction = serde_json::from_str(
        r#"{
            "type": 2,
            "token": "A_UNIQUE_TOKEN",
            "member": {
                "user": {
                    "id": "53908232506183680",
                    "username": "Mason",
                    "avatar": "a_d5efa99b3eeaa7dd43acca82f5692432",
                    "discriminator": "1337",
                    "public_flags": 131141
                },
                "roles": ["539082325061836999"],
                "premium_since": null,
                "permissions": "2147483647",
                "pending": false,
                "nick": null,
                "mute": false,
                "joined_at": "2017-03-13T19:19:14.040000+00:00",
                "is_pending": false,
                "deaf": false
            },
            "id": "786008729715212338",
            "application_id": "290926798626357999",
            "guild_id": "290926798626357999",
            "app_permissions": "442368",
            "guild_locale": "en-US",
            "locale": "en-US",
            "version": 1,
            "data": {
                "options": [
                    {"type": 3, "name": "cardname", "value": "The Gitrog Monster"}
                ],
                "type": 1,
                "name": "cardsearch",
                "id": "771825006014889984"
            },
            "channel_id": "645027906669510667",
            "context": 0,
            "authorizing_integration_owners": {"0": "290926798626357999"}
        }"#,
    )
    .unwrap();

    assert_eq!(interaction.kind, InteractionType::APPLICATION_COMMAND);
    assert_eq!(interaction.token, "A_UNIQUE_TOKEN");
    assert_eq!(interaction.context, Some(InteractionContextType::GUILD));
    assert_eq!(interaction.app_permissions.unwrap().bits(), 442_368);

    let member = interaction.member.as_ref().unwrap();
    assert_eq!(member.permissions.unwrap().bits(), 2_147_483_647);
    assert_eq!(member.roles[0].get(), 539_082_325_061_836_999);
    assert_eq!(member.joined_at, Some(timestamp(1_489_432_754, 40_000_000)));

    let Some(InteractionData::ApplicationCommand(data)) = &interaction.data else {
        panic!("expected application command data");
    };
    assert_eq!(data.name, "cardsearch");
    assert_eq!(data.kind, ApplicationCommandType::CHAT_INPUT);
    assert_eq!(data.target_id, None);

    let option = &data.options.as_ref().unwrap()[0];
    assert_eq!(option.kind, ApplicationCommandOptionType::STRING);
    assert_eq!(option.value, Some(CommandOptionValue::String("The Gitrog Monster".into())));

    let value = serde_json::to_value(&interaction).unwrap();
    assert_eq!(value["data"]["id"], "771825006014889984");
    assert_eq!(value["authorizing_integration_owners"]["0"], "290926798626357999");
    assert_eq!(value["member"]["joined_at"], "2017-03-13T19:19:14.04+00:00");
}

/// Tests the authorizing integration owners object, including the bot DM
/// sentinel and unknown application integration keys.
#[test]
fn authorizing_integration_owners_handles_bot_dm_and_unknown_keys() {
    let owners: AuthorizingIntegrationOwners =
        serde_json::from_str(r#"{"0": "0", "1": "123456789012345678", "2": {"future": true}}"#)
            .unwrap();

    assert_eq!(owners.guild_install, Some(GuildInstallOwner::BotDm));
    assert_eq!(owners.user_install.unwrap().get(), 123_456_789_012_345_678);
    assert_eq!(owners.unknown.len(), 1);

    let value = serde_json::to_value(&owners).unwrap();
    assert_eq!(value["0"], "0");
    assert_eq!(value["1"], "123456789012345678");
    assert_eq!(value["2"]["future"], true);

    let owners: AuthorizingIntegrationOwners =
        serde_json::from_str(r#"{"0": "290926798626357999"}"#).unwrap();
    assert_eq!(
        owners.guild_install,
        Some(GuildInstallOwner::Guild(Id::new(290_926_798_626_357_999)))
    );
}

/// Tests the three interaction callback data shapes.
#[test]
fn interaction_response_variants_match_documented_shapes() {
    let message_response: InteractionResponse = serde_json::from_str(
        r#"{
            "type": 4,
            "data": {
                "content": "Hello from Cordion",
                "flags": 64,
                "components": [
                    {
                        "type": 1,
                        "components": [
                            {"type": 2, "style": 1, "custom_id": "click", "label": "Click"}
                        ]
                    }
                ]
            }
        }"#,
    )
    .unwrap();

    assert_eq!(message_response.kind, InteractionCallbackType::CHANNEL_MESSAGE_WITH_SOURCE);
    let Some(InteractionCallbackData::Message(message)) = &message_response.data else {
        panic!("expected message callback data");
    };
    assert_eq!(message.content.as_deref(), Some("Hello from Cordion"));
    assert_eq!(message.flags.unwrap(), MessageFlags::EPHEMERAL);
    assert_eq!(message.components.as_ref().unwrap().len(), 1);

    let value = serde_json::to_value(&message_response).unwrap();
    assert_eq!(value["type"], 4);
    assert_eq!(value["data"]["content"], "Hello from Cordion");
    assert!(value["data"].get("custom_id").is_none());

    let modal_response: InteractionResponse = serde_json::from_str(
        r#"{
            "type": 9,
            "data": {
                "custom_id": "feedback_modal",
                "title": "Feedback",
                "components": [
                    {
                        "type": 18,
                        "label": "Name",
                        "component": {"type": 4, "custom_id": "name", "style": 1}
                    }
                ]
            }
        }"#,
    )
    .unwrap();

    assert_eq!(modal_response.kind, InteractionCallbackType::MODAL);
    let Some(InteractionCallbackData::Modal { custom_id, title, components }) =
        &modal_response.data
    else {
        panic!("expected modal callback data");
    };
    assert_eq!(custom_id, "feedback_modal");
    assert_eq!(title, "Feedback");
    assert_eq!(components.len(), 1);

    let autocomplete_response: InteractionResponse = serde_json::from_str(
        r#"{
            "type": 8,
            "data": {"choices": [{"name": "The Gitrog Monster", "value": "gitrog"}]}
        }"#,
    )
    .unwrap();

    assert_eq!(
        autocomplete_response.kind,
        InteractionCallbackType::APPLICATION_COMMAND_AUTOCOMPLETE_RESULT
    );
    let Some(InteractionCallbackData::Autocomplete { choices }) = &autocomplete_response.data
    else {
        panic!("expected autocomplete callback data");
    };
    assert_eq!(choices[0].name, "The Gitrog Monster");
    assert_eq!(
        choices[0].value,
        cordion_model::application_command::CommandOptionChoiceValue::String("gitrog".into())
    );
}

/// Tests the message object with components, reactions, a reply reference, a
/// forward snapshot, and interaction metadata.
#[test]
fn message_decodes_components_reactions_and_forward_snapshot() {
    let message: Message = serde_json::from_str(
        r#"{
            "id": "1081000000000000001",
            "channel_id": "1081000000000000002",
            "author": {
                "id": "1081000000000000003",
                "username": "Cordion",
                "discriminator": "0",
                "global_name": "Cordion",
                "avatar": null,
                "bot": true,
                "public_flags": 0
            },
            "content": "Hello",
            "timestamp": "2026-01-02T03:04:05.006000+00:00",
            "edited_timestamp": null,
            "tts": false,
            "mention_everyone": false,
            "mentions": [
                {"id": "53908232506183680", "username": "Mason", "discriminator": "1337"}
            ],
            "mention_roles": ["1081000000000000004"],
            "attachments": [
                {
                    "id": "1081000000000000005",
                    "filename": "image.png",
                    "title": "Image",
                    "description": "A test image",
                    "content_type": "image/png",
                    "size": 12345,
                    "url": "https://cdn.discordapp.com/attachments/1081000000000000002/1081000000000000005/image.png",
                    "proxy_url": "https://media.discordapp.net/attachments/1081000000000000002/1081000000000000005/image.png",
                    "height": 720,
                    "width": 1280,
                    "placeholder": "abc",
                    "placeholder_version": 1,
                    "flags": 16
                }
            ],
            "embeds": [
                {
                    "title": "Embed",
                    "type": "rich",
                    "description": "Description",
                    "color": 16711680,
                    "footer": {"text": "Footer"},
                    "image": {
                        "url": "https://example.com/image.png",
                        "width": 10,
                        "height": 20
                    },
                    "fields": [{"name": "Field", "value": "Value", "inline": true}]
                }
            ],
            "reactions": [
                {
                    "count": 2,
                    "count_details": {"burst": 0, "normal": 2},
                    "me": true,
                    "me_burst": false,
                    "emoji": {"id": null, "name": "🔥", "animated": false},
                    "burst_colors": []
                }
            ],
            "pinned": false,
            "type": 19,
            "flags": 32768,
            "message_reference": {
                "type": 0,
                "message_id": "1081000000000000006",
                "channel_id": "1081000000000000002",
                "guild_id": "1081000000000000007",
                "fail_if_not_exists": false
            },
            "referenced_message": null,
            "message_snapshots": [
                {
                    "message": {
                        "type": 0,
                        "content": "Forwarded",
                        "embeds": [],
                        "attachments": [],
                        "timestamp": "2026-01-02T03:00:00+00:00",
                        "edited_timestamp": null,
                        "flags": 0,
                        "mentions": [],
                        "mention_roles": [],
                        "stickers": [],
                        "sticker_items": [],
                        "components": []
                    }
                }
            ],
            "interaction_metadata": {
                "id": "1081000000000000008",
                "type": 2,
                "user": {
                    "id": "53908232506183680",
                    "username": "Mason",
                    "discriminator": "1337"
                },
                "authorizing_integration_owners": {"0": "1081000000000000007"},
                "original_response_message_id": "1081000000000000009"
            },
            "components": [
                {
                    "type": 1,
                    "components": [
                        {"type": 2, "style": 1, "custom_id": "click", "label": "Click"}
                    ]
                }
            ],
            "sticker_items": [
                {"id": "1081000000000000010", "name": "wave", "format_type": 1}
            ]
        }"#,
    )
    .unwrap();

    assert_eq!(message.kind, MessageType::REPLY);
    assert_eq!(message.timestamp, timestamp(1_767_323_045, 6_000_000));
    assert_eq!(message.edited_timestamp, None);
    assert!(message.flags.unwrap().contains(MessageFlags::IS_COMPONENTS_V2));

    assert_eq!(message.attachments[0].filename, "image.png");
    assert_eq!(message.attachments[0].flags.unwrap().bits(), 16);
    assert_eq!(message.embeds[0].footer.as_ref().unwrap().text, "Footer");
    assert_eq!(message.reactions.as_ref().unwrap()[0].count, 2);
    assert_eq!(message.reactions.as_ref().unwrap()[0].emoji.name.as_deref(), Some("🔥"));

    let reference = message.message_reference.unwrap();
    assert_eq!(reference.kind, Some(MessageReferenceType::DEFAULT));
    assert_eq!(reference.message_id.unwrap().get(), 1_081_000_000_000_000_006);
    assert_eq!(reference.fail_if_not_exists, Some(false));

    let snapshot = &message.message_snapshots.as_ref().unwrap()[0];
    assert_eq!(snapshot.message.content, "Forwarded");
    assert_eq!(snapshot.message.kind, MessageType::DEFAULT);
    assert_eq!(snapshot.message.flags.unwrap(), MessageFlags::NONE);

    let MessageInteractionMetadata::ApplicationCommand(metadata) =
        message.interaction_metadata.as_ref().unwrap()
    else {
        panic!("expected application command interaction metadata");
    };
    assert_eq!(metadata.kind, InteractionType::APPLICATION_COMMAND);
    assert_eq!(metadata.original_response_message_id.unwrap().get(), 1_081_000_000_000_000_009);

    let Component::ActionRow(row) = &message.components.as_ref().unwrap()[0] else {
        panic!("expected an action row");
    };
    let Component::Button(button) = &row.components[0] else {
        panic!("expected a button");
    };
    assert_eq!(button.style, ButtonStyle::PRIMARY);
    assert_eq!(button.custom_id.as_deref(), Some("click"));

    assert_eq!(message.sticker_items.as_ref().unwrap()[0].name, "wave");

    let value = serde_json::to_value(&message).unwrap();
    assert_eq!(value["id"], "1081000000000000001");
    assert_eq!(value["type"], 19);
    assert_eq!(value["edited_timestamp"], Value::Null);
    assert_eq!(value["message_snapshots"][0]["message"]["content"], "Forwarded");
    assert_eq!(value["interaction_metadata"]["type"], 2);
    assert_eq!(value["components"][0]["components"][0]["type"], 2);
    assert!(value.get("referenced_message").is_none());
}

/// Tests the deprecated message interaction object.
#[test]
fn deprecated_message_interaction_decodes_partial_member() {
    let interaction: MessageInteraction = serde_json::from_str(
        r#"{
            "id": "867794291820986368",
            "type": 2,
            "name": "cardsearch",
            "user": {
                "id": "53908232506183680",
                "username": "Mason",
                "discriminator": "1337"
            },
            "member": {
                "roles": [],
                "joined_at": "2021-02-12T18:25:07.972000+00:00",
                "permissions": "246997699136"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(interaction.kind, InteractionType::APPLICATION_COMMAND);
    assert_eq!(interaction.user.id.get(), 53_908_232_506_183_680);
    let member = interaction.member.as_ref().unwrap();
    assert_eq!(member.permissions.unwrap().bits(), 246_997_699_136);
    assert_eq!(member.joined_at, Some(timestamp(1_613_154_307, 972_000_000)));
}

/// Tests the three message interaction metadata shapes and the unknown
/// fallback.
#[test]
fn message_interaction_metadata_variants_are_distinguished() {
    let component: MessageInteractionMetadata = serde_json::from_str(
        r#"{
            "id": "1081000000000000011",
            "type": 3,
            "user": {
                "id": "53908232506183680",
                "username": "Mason",
                "discriminator": "1337"
            },
            "authorizing_integration_owners": {"1": "53908232506183680"},
            "interacted_message_id": "1081000000000000012"
        }"#,
    )
    .unwrap();

    let MessageInteractionMetadata::MessageComponent(metadata) = &component else {
        panic!("expected message component metadata");
    };
    assert_eq!(metadata.kind, InteractionType::MESSAGE_COMPONENT);
    assert_eq!(metadata.interacted_message_id.get(), 1_081_000_000_000_000_012);

    let modal: MessageInteractionMetadata = serde_json::from_str(
        r#"{
            "id": "1081000000000000013",
            "type": 5,
            "user": {
                "id": "53908232506183680",
                "username": "Mason",
                "discriminator": "1337"
            },
            "authorizing_integration_owners": {"0": "0"},
            "triggering_interaction_metadata": {
                "id": "1081000000000000014",
                "type": 3,
                "user": {
                    "id": "53908232506183680",
                    "username": "Mason",
                    "discriminator": "1337"
                },
                "interacted_message_id": "1081000000000000015"
            }
        }"#,
    )
    .unwrap();

    let MessageInteractionMetadata::ModalSubmit(metadata) = &modal else {
        panic!("expected modal submit metadata");
    };
    assert_eq!(metadata.kind, InteractionType::MODAL_SUBMIT);
    assert_eq!(
        metadata.authorizing_integration_owners.as_ref().unwrap().guild_install,
        Some(GuildInstallOwner::BotDm)
    );
    let MessageInteractionMetadata::MessageComponent(triggering) =
        metadata.triggering_interaction_metadata.as_ref()
    else {
        panic!("expected component metadata for the trigger");
    };
    assert_eq!(triggering.id.get(), 1_081_000_000_000_000_014);
    assert_eq!(triggering.interacted_message_id.get(), 1_081_000_000_000_000_015);

    let unknown: MessageInteractionMetadata =
        serde_json::from_str(r#"{"id": "1", "type": 99, "shape": "future"}"#).unwrap();
    assert!(matches!(unknown, MessageInteractionMetadata::Unknown(_)));
}

/// Tests the entitlement example from Discord's entitlement documentation.
#[test]
fn entitlement_matches_discord_example() {
    let entitlement: Entitlement = serde_json::from_str(
        r#"{
            "id": "1019653849998299136",
            "sku_id": "1019475255913222144",
            "application_id": "1019370614521200640",
            "user_id": "771129655544643584",
            "promotion_id": null,
            "type": 8,
            "deleted": false,
            "gift_code_flags": 0,
            "consumed": false,
            "starts_at": "2022-09-14T17:00:18.704163+00:00",
            "ends_at": "2022-10-14T17:00:18.704163+00:00",
            "guild_id": "1015034326372454400",
            "subscription_id": "1019653835926409216"
        }"#,
    )
    .unwrap();

    assert_eq!(entitlement.kind, EntitlementType::APPLICATION_SUBSCRIPTION);
    assert_eq!(entitlement.starts_at, Some(timestamp(1_663_174_818, 704_163_000)));
    assert_eq!(entitlement.ends_at, Some(timestamp(1_665_766_818, 704_163_000)));
    assert_eq!(entitlement.user_id.unwrap().get(), 771_129_655_544_643_584);
    assert_eq!(entitlement.guild_id.unwrap().get(), 1_015_034_326_372_454_400);

    let value = serde_json::to_value(entitlement).unwrap();
    assert_eq!(value["id"], "1019653849998299136");
    assert_eq!(value["type"], 8);
    assert_eq!(value["starts_at"], "2022-09-14T17:00:18.704163+00:00");

    let minimal: Entitlement = serde_json::from_str(
        r#"{
            "id": "1",
            "sku_id": "2",
            "application_id": "3",
            "type": 1,
            "deleted": false
        }"#,
    )
    .unwrap();
    assert_eq!(minimal.starts_at, None);
    assert_eq!(minimal.ends_at, None);

    let value = serde_json::to_value(minimal).unwrap();
    assert_eq!(value["starts_at"], Value::Null);
    assert_eq!(value["ends_at"], Value::Null);
    assert!(value.get("consumed").is_none());
}

/// Tests the poll object attached to a message and the poll create request
/// object used in interaction callback message data.
#[test]
fn poll_objects_distinguish_expiry_and_duration() {
    let message: Message = serde_json::from_str(
        r#"{
            "id": "1081000000000000100",
            "channel_id": "1081000000000000101",
            "author": {
                "id": "53908232506183680",
                "username": "Mason",
                "discriminator": "1337"
            },
            "content": "",
            "timestamp": "2026-02-01T00:00:00+00:00",
            "edited_timestamp": null,
            "tts": false,
            "mention_everyone": false,
            "mentions": [],
            "mention_roles": [],
            "attachments": [],
            "embeds": [],
            "pinned": false,
            "type": 0,
            "poll": {
                "question": {"text": "Which option?"},
                "answers": [
                    {
                        "answer_id": 1,
                        "poll_media": {"text": "A", "emoji": {"id": null, "name": "🅰️"}}
                    },
                    {"answer_id": 2, "poll_media": {"text": "B"}}
                ],
                "expiry": null,
                "allow_multiselect": false,
                "layout_type": 1,
                "results": {
                    "is_finalized": true,
                    "answer_counts": [
                        {"id": 1, "count": 3, "me_voted": true},
                        {"id": 2, "count": 0, "me_voted": false}
                    ]
                }
            }
        }"#,
    )
    .unwrap();

    let poll = message.poll.as_ref().unwrap();
    assert_eq!(poll.question.text.as_deref(), Some("Which option?"));
    assert_eq!(poll.expiry, None);
    assert_eq!(poll.layout_type, PollLayoutType::DEFAULT);
    assert_eq!(poll.answers[0].answer_id, Some(1));
    assert_eq!(poll.answers[0].poll_media.emoji.as_ref().unwrap().name.as_deref(), Some("🅰️"));
    let results = poll.results.as_ref().unwrap();
    assert!(results.is_finalized);
    assert_eq!(results.answer_counts[0].count, 3);
    assert!(results.answer_counts[0].me_voted);

    let value = serde_json::to_value(&message).unwrap();
    assert_eq!(value["poll"]["expiry"], Value::Null);
    assert_eq!(value["poll"]["answers"][0]["answer_id"], 1);
    assert_eq!(value["poll"]["results"]["answer_counts"][0]["count"], 3);

    let response: InteractionResponse = serde_json::from_str(
        r#"{
            "type": 4,
            "data": {
                "poll": {
                    "question": {"text": "Lunch?"},
                    "answers": [
                        {"poll_media": {"text": "Pizza"}},
                        {"poll_media": {"text": "Sushi"}}
                    ],
                    "duration": 48,
                    "allow_multiselect": true,
                    "layout_type": 1
                }
            }
        }"#,
    )
    .unwrap();

    let Some(InteractionCallbackData::Message(message)) = &response.data else {
        panic!("expected message callback data");
    };
    let poll = message.poll.as_ref().unwrap();
    assert_eq!(poll.duration, Some(48));
    assert_eq!(poll.allow_multiselect, Some(true));
    assert_eq!(poll.layout_type, Some(PollLayoutType::DEFAULT));
    assert_eq!(poll.answers[0].answer_id, None);

    let value = serde_json::to_value(&response).unwrap();
    assert_eq!(value["data"]["poll"]["duration"], 48);
    assert_eq!(value["data"]["poll"]["layout_type"], 1);
    assert!(value["data"]["poll"].get("expiry").is_none());
    assert!(value["data"]["poll"]["answers"][0].get("answer_id").is_none());
}

/// Tests message component interaction data, including all resolved data
/// collections.
#[test]
fn message_component_interaction_data_decodes_resolved_data() {
    let interaction: Interaction = serde_json::from_str(
        r#"{
            "id": "1081000000000000110",
            "application_id": "1081000000000000111",
            "type": 3,
            "token": "component-token",
            "version": 1,
            "app_permissions": "1024",
            "data": {
                "component_type": 5,
                "id": 7,
                "custom_id": "users",
                "values": ["53908232506183680"],
                "resolved": {
                    "users": {
                        "53908232506183680": {
                            "id": "53908232506183680",
                            "username": "Mason",
                            "discriminator": "1337"
                        }
                    },
                    "members": {
                        "53908232506183680": {
                            "roles": ["1081000000000000102"],
                            "joined_at": "2021-02-12T18:25:07.972000+00:00",
                            "pending": false
                        }
                    },
                    "roles": {
                        "1081000000000000102": {
                            "id": "1081000000000000102",
                            "name": "Moderator",
                            "color": 0,
                            "colors": {
                                "primary_color": 0,
                                "secondary_color": null,
                                "tertiary_color": null
                            },
                            "hoist": false,
                            "position": 1,
                            "permissions": "1024",
                            "managed": false,
                            "mentionable": false,
                            "flags": 0
                        }
                    },
                    "messages": {
                        "1081000000000000103": {
                            "id": "1081000000000000103",
                            "channel_id": "1081000000000000101",
                            "author": {
                                "id": "53908232506183680",
                                "username": "Mason",
                                "discriminator": "1337"
                            },
                            "content": "Selected context",
                            "timestamp": "2026-02-01T00:00:00+00:00",
                            "edited_timestamp": null,
                            "tts": false,
                            "mention_everyone": false,
                            "mentions": [],
                            "mention_roles": [],
                            "attachments": [],
                            "embeds": [],
                            "pinned": false,
                            "type": 0
                        }
                    },
                    "attachments": {
                        "1081000000000000104": {
                            "id": "1081000000000000104",
                            "filename": "spec.txt",
                            "size": 12,
                            "url": "https://cdn.discordapp.com/attachments/1081000000000000101/1081000000000000104/spec.txt",
                            "proxy_url": "https://media.discordapp.net/attachments/1081000000000000101/1081000000000000104/spec.txt"
                        }
                    }
                }
            }
        }"#,
    )
    .unwrap();

    assert_eq!(interaction.kind, InteractionType::MESSAGE_COMPONENT);
    let Some(InteractionData::MessageComponent(data)) = &interaction.data else {
        panic!("expected message component data");
    };
    assert_eq!(data.component_type, ComponentType::USER_SELECT);
    assert_eq!(data.id, Some(7));
    assert_eq!(data.custom_id, "users");
    assert_eq!(data.values.as_ref().unwrap()[0], "53908232506183680");

    let resolved = data.resolved.as_ref().unwrap();
    let member = resolved.members.as_ref().unwrap().values().next().unwrap();
    assert_eq!(member.roles[0].get(), 1_081_000_000_000_000_102);
    let role = resolved.roles.as_ref().unwrap().values().next().unwrap();
    assert_eq!(role.name, "Moderator");
    assert_eq!(role.permissions.bits(), 1024);
    let message = resolved.messages.as_ref().unwrap().values().next().unwrap();
    assert_eq!(message.content, "Selected context");
    let attachment = resolved.attachments.as_ref().unwrap().values().next().unwrap();
    assert_eq!(attachment.filename, "spec.txt");
    assert_eq!(attachment.size, 12);

    let value = serde_json::to_value(&interaction).unwrap();
    assert_eq!(value["data"]["component_type"], 5);
    assert_eq!(value["data"]["id"], 7);
    assert_eq!(value["data"]["resolved"]["roles"]["1081000000000000102"]["name"], "Moderator");
    assert_eq!(
        value["data"]["resolved"]["messages"]["1081000000000000103"]["content"],
        "Selected context"
    );
    assert_eq!(value["data"]["resolved"]["attachments"]["1081000000000000104"]["size"], 12);
}

/// Tests an autocomplete interaction (type 4) with a focused option.
#[test]
fn autocomplete_interaction_decodes_focused_option() {
    let interaction: Interaction = serde_json::from_str(
        r#"{
            "id": "1081000000000000120",
            "application_id": "1081000000000000121",
            "type": 4,
            "token": "autocomplete-token",
            "version": 1,
            "locale": "en-US",
            "app_permissions": "442368",
            "data": {
                "id": "771825006014889984",
                "name": "cardsearch",
                "type": 1,
                "version": "847194950382780532",
                "options": [
                    {"type": 3, "name": "cardname", "value": "The Gitrog", "focused": true},
                    {"type": 4, "name": "count", "value": 2}
                ]
            }
        }"#,
    )
    .unwrap();

    assert_eq!(interaction.kind, InteractionType::APPLICATION_COMMAND_AUTOCOMPLETE);
    let Some(InteractionData::ApplicationCommand(data)) = &interaction.data else {
        panic!("expected application command data");
    };
    let options = data.options.as_ref().unwrap();
    assert_eq!(data.version, Some(847_194_950_382_780_532));
    assert_eq!(options[0].focused, Some(true));
    assert_eq!(options[0].value, Some(CommandOptionValue::String("The Gitrog".into())));
    assert_eq!(options[1].focused, None);
    assert_eq!(options[1].value, Some(CommandOptionValue::Integer(2)));

    let value = serde_json::to_value(&interaction).unwrap();
    assert_eq!(value["data"]["version"], "847194950382780532");
    assert_eq!(value["data"]["options"][0]["focused"], true);
    assert!(value["data"]["options"][1].get("focused").is_none());
}

/// Tests that a PING interaction decodes without interaction data.
#[test]
fn ping_interaction_decodes_without_data() {
    let interaction: Interaction = serde_json::from_str(
        r#"{"id":"1","application_id":"2","type":1,"token":"ping","version":1}"#,
    )
    .unwrap();

    assert_eq!(interaction.kind, InteractionType::PING);
    assert!(interaction.data.is_none());
    assert!(interaction.entitlements.is_empty());

    let value = serde_json::to_value(&interaction).unwrap();
    assert_eq!(value["type"], 1);
    assert!(value.get("data").is_none());
    assert!(value.get("entitlements").is_none());
}

/// Tests the request shapes of the modal-only components introduced with
/// components v2.
#[test]
fn modal_request_components_round_trip() {
    let response: InteractionResponse = serde_json::from_str(
        r#"{
            "type": 9,
            "data": {
                "custom_id": "onboarding_modal",
                "title": "Onboarding",
                "components": [
                    {
                        "type": 18,
                        "label": "Attachments",
                        "description": "Add a screenshot",
                        "component": {
                            "type": 19,
                            "id": 1,
                            "custom_id": "files",
                            "min_values": 1,
                            "max_values": 10,
                            "required": true,
                            "file_types": ["image", ".pdf"]
                        }
                    },
                    {
                        "type": 18,
                        "label": "Class",
                        "component": {
                            "type": 21,
                            "id": 2,
                            "custom_id": "class",
                            "options": [
                                {"value": "warrior", "label": "Warrior", "default": true},
                                {"value": "rogue", "label": "Rogue", "description": "Stealthy"}
                            ],
                            "required": true
                        }
                    },
                    {
                        "type": 18,
                        "label": "Days",
                        "component": {
                            "type": 22,
                            "id": 3,
                            "custom_id": "days",
                            "options": [
                                {"value": "mon", "label": "Monday"},
                                {"value": "tue", "label": "Tuesday"}
                            ],
                            "min_values": 1,
                            "max_values": 2,
                            "required": false
                        }
                    },
                    {
                        "type": 18,
                        "label": "Agree",
                        "component": {
                            "type": 23,
                            "id": 4,
                            "custom_id": "agree",
                            "default": true
                        }
                    }
                ]
            }
        }"#,
    )
    .unwrap();

    let Some(InteractionCallbackData::Modal { components, .. }) = &response.data else {
        panic!("expected modal callback data");
    };
    assert_eq!(components.len(), 4);

    let Component::Label(label) = &components[0] else {
        panic!("expected a label");
    };
    assert_eq!(label.label, "Attachments");
    assert_eq!(label.description.as_deref(), Some("Add a screenshot"));
    let Component::FileUpload(upload) = label.component.as_ref() else {
        panic!("expected a file upload");
    };
    assert_eq!(upload.kind, ComponentType::FILE_UPLOAD);
    assert_eq!(upload.id, Some(1));
    assert_eq!(upload.custom_id, "files");
    assert_eq!(upload.min_values, Some(1));
    assert_eq!(upload.max_values, Some(10));
    assert_eq!(upload.required, Some(true));
    assert_eq!(upload.file_types.as_ref().unwrap(), &["image", ".pdf"]);

    let Component::Label(label) = &components[1] else {
        panic!("expected a label");
    };
    let Component::RadioGroup(group) = label.component.as_ref() else {
        panic!("expected a radio group");
    };
    assert_eq!(group.kind, ComponentType::RADIO_GROUP);
    assert_eq!(group.options.len(), 2);
    assert_eq!(group.options[0].default, Some(true));
    assert_eq!(group.options[1].description.as_deref(), Some("Stealthy"));
    assert_eq!(group.required, Some(true));

    let Component::Label(label) = &components[2] else {
        panic!("expected a label");
    };
    let Component::CheckboxGroup(group) = label.component.as_ref() else {
        panic!("expected a checkbox group");
    };
    assert_eq!(group.kind, ComponentType::CHECKBOX_GROUP);
    assert_eq!(group.min_values, Some(1));
    assert_eq!(group.max_values, Some(2));
    assert_eq!(group.required, Some(false));

    let Component::Label(label) = &components[3] else {
        panic!("expected a label");
    };
    let Component::Checkbox(checkbox) = label.component.as_ref() else {
        panic!("expected a checkbox");
    };
    assert_eq!(checkbox.kind, ComponentType::CHECKBOX);
    assert_eq!(checkbox.default, Some(true));

    let value = serde_json::to_value(&response).unwrap();
    assert_eq!(value["data"]["components"][0]["component"]["type"], 19);
    assert_eq!(value["data"]["components"][0]["component"]["file_types"][1], ".pdf");
    assert_eq!(value["data"]["components"][1]["component"]["type"], 21);
    assert_eq!(
        value["data"]["components"][1]["component"]["options"][1]["description"],
        "Stealthy"
    );
    assert_eq!(value["data"]["components"][2]["component"]["type"], 22);
    assert_eq!(value["data"]["components"][2]["component"]["max_values"], 2);
    assert_eq!(value["data"]["components"][3]["component"]["type"], 23);
    assert_eq!(value["data"]["components"][3]["component"]["default"], true);
}

/// Tests the application object example from Discord's application resource.
#[test]
fn application_matches_discord_example() {
    let application: Application = serde_json::from_str(
        r#"{
            "bot_public": true,
            "bot_require_code_grant": false,
            "cover_image": "31deabb7e45b6c8ecfef77d2f99c81a5",
            "description": "Test",
            "flags": 8192,
            "flags_new": "8192",
            "guild_id": "290926798626357260",
            "icon": null,
            "id": "172150183260323840",
            "integration_types_config": {
                "0": {
                    "oauth2_install_params": {
                        "scopes": ["applications.commands", "bot"],
                        "permissions": "2048"
                    }
                },
                "1": {
                    "oauth2_install_params": {
                        "scopes": ["applications.commands"],
                        "permissions": "0"
                    }
                }
            },
            "name": "Baba O-Riley",
            "interactions_endpoint_url": null,
            "role_connections_verification_url": null,
            "event_webhooks_url": null,
            "event_webhooks_status": 1,
            "owner": {
                "avatar": null,
                "discriminator": "1738",
                "flags": 1024,
                "id": "172150183260323840",
                "username": "i own a bot"
            },
            "primary_sku_id": "172150183260323840",
            "slug": "test",
            "team": {
                "name": "A team",
                "icon": "dd9b7dcfdf5351b9c3de0fe167bacbe1",
                "id": "531992624043786253",
                "owner_user_id": "511972282709709995",
                "members": [
                    {
                        "membership_state": 2,
                        "permissions": ["*"],
                        "team_id": "531992624043786253",
                        "user": {
                            "avatar": "d9e261cd35999608eb7e3de1fae3688b",
                            "discriminator": "0001",
                            "id": "511972282709709995",
                            "username": "Mr Owner"
                        }
                    }
                ]
            },
            "verify_key": "1e0a356058d627ca38a5c8c9648818061d49e49bd9da9e3ab17d98ad4d6bg2u8"
        }"#,
    )
    .unwrap();

    assert_eq!(application.id.get(), 172_150_183_260_323_840);
    assert_eq!(application.name, "Baba O-Riley");
    assert!(application.icon.is_none());
    assert_eq!(application.description, "Test");
    assert_eq!(application.guild_id.unwrap().get(), 290_926_798_626_357_260);
    assert_eq!(application.primary_sku_id.unwrap().get(), 172_150_183_260_323_840);
    assert_eq!(application.slug.as_deref(), Some("test"));
    assert_eq!(application.cover_image.as_deref(), Some("31deabb7e45b6c8ecfef77d2f99c81a5"));
    assert_eq!(application.flags, Some(ApplicationFlags::GATEWAY_PRESENCE_LIMITED));
    assert_eq!(application.flags_new, Some(ApplicationFlags::GATEWAY_PRESENCE_LIMITED));
    assert_eq!(application.event_webhooks_status, Some(ApplicationEventWebhookStatus::DISABLED));
    assert!(application.interactions_endpoint_url.is_none());
    assert!(application.role_connections_verification_url.is_none());
    assert!(application.event_webhooks_url.is_none());

    let owner = application.owner.as_ref().unwrap();
    assert_eq!(owner.id.get(), 172_150_183_260_323_840);
    assert_eq!(owner.username.as_deref(), Some("i own a bot"));
    assert_eq!(owner.discriminator.as_deref(), Some("1738"));
    assert_eq!(owner.flags, Some(UserFlags::TEAM_PSEUDO_USER));

    let team = application.team.as_ref().unwrap();
    assert_eq!(team.id.get(), 531_992_624_043_786_253);
    assert_eq!(team.name, "A team");
    assert_eq!(team.owner_user_id.get(), 511_972_282_709_709_995);
    assert_eq!(team.members.len(), 1);
    assert_eq!(team.members[0].membership_state, TeamMembershipState::ACCEPTED);
    assert_eq!(team.members[0].team_id.get(), 531_992_624_043_786_253);
    assert_eq!(team.members[0].user.username, "Mr Owner");
    assert_eq!(team.members[0].role, None);

    let configs = application.integration_types_config.as_ref().unwrap();
    assert_eq!(configs.len(), 2);
    let guild_install = &configs[&ApplicationIntegrationType::GUILD_INSTALL];
    let guild_params = guild_install.oauth2_install_params.as_ref().unwrap();
    assert_eq!(
        guild_params.scopes,
        vec!["applications.commands".to_string(), "bot".to_string()]
    );
    assert_eq!(guild_params.permissions.bits(), 2048);
    let user_install = &configs[&ApplicationIntegrationType::USER_INSTALL];
    let user_params = user_install.oauth2_install_params.as_ref().unwrap();
    assert_eq!(user_params.scopes, vec!["applications.commands".to_string()]);
    assert_eq!(user_params.permissions.bits(), 0);

    let value = serde_json::to_value(&application).unwrap();
    assert!(value["icon"].is_null());
    assert_eq!(value["team"]["id"], "531992624043786253");
    assert_eq!(value["team"]["members"][0]["membership_state"], 2);
    assert!(value["team"]["members"][0].get("role").is_none());
    assert_eq!(value["flags"], 8192);
    assert_eq!(value["flags_new"], "8192");
    assert_eq!(
        value["integration_types_config"]["0"]["oauth2_install_params"]["permissions"],
        "2048"
    );
    assert!(value["interactions_endpoint_url"].is_null());
    assert!(value["role_connections_verification_url"].is_null());
    assert!(value["event_webhooks_url"].is_null());
    assert_eq!(value["owner"]["id"], "172150183260323840");
    assert_eq!(value["guild_id"], "290926798626357260");
    assert_eq!(value["primary_sku_id"], "172150183260323840");
}

/// Tests that nullable application fields serialize as `null` and absent
/// optional fields are omitted.
#[test]
fn application_nullable_fields_and_optional_omission() {
    let application: Application = serde_json::from_str(
        r#"{
            "id": "1",
            "name": "Test",
            "icon": null,
            "description": "Test",
            "bot_public": true,
            "bot_require_code_grant": false,
            "verify_key": "key",
            "team": null
        }"#,
    )
    .unwrap();

    assert!(application.icon.is_none());
    assert!(application.team.is_none());
    assert!(application.interactions_endpoint_url.is_none());

    let value = serde_json::to_value(&application).unwrap();
    assert!(value["icon"].is_null());
    assert!(value["team"].is_null());
    assert!(value["interactions_endpoint_url"].is_null());
    assert!(value["role_connections_verification_url"].is_null());
    assert!(value["event_webhooks_url"].is_null());
    assert!(value.get("rpc_origins").is_none());
    assert!(value.get("bot").is_none());
    assert!(value.get("terms_of_service_url").is_none());
    assert!(value.get("privacy_policy_url").is_none());
    assert!(value.get("owner").is_none());
    assert!(value.get("guild_id").is_none());
    assert!(value.get("guild").is_none());
    assert!(value.get("primary_sku_id").is_none());
    assert!(value.get("slug").is_none());
    assert!(value.get("cover_image").is_none());
    assert!(value.get("flags").is_none());
    assert!(value.get("flags_new").is_none());
    assert!(value.get("approximate_guild_count").is_none());
    assert!(value.get("approximate_user_install_count").is_none());
    assert!(value.get("approximate_user_authorization_count").is_none());
    assert!(value.get("redirect_uris").is_none());
    assert!(value.get("event_webhooks_status").is_none());
    assert!(value.get("event_webhooks_types").is_none());
    assert!(value.get("tags").is_none());
    assert!(value.get("install_params").is_none());
    assert!(value.get("integration_types_config").is_none());
    assert!(value.get("custom_install_url").is_none());
}

/// Tests that application flags accept integers and decimal strings while
/// preserving bits that Cordion does not define.
#[test]
fn application_flags_preserve_unknown_bits() {
    let from_integer: ApplicationFlags = serde_json::from_str("9007199254740992").unwrap();
    let from_string: ApplicationFlags = serde_json::from_str("\"9007199254740992\"").unwrap();

    assert_eq!(from_integer, from_string);
    assert_eq!(from_integer.bits(), 9_007_199_254_740_992);
    assert_eq!(
        serde_json::to_value(from_integer).unwrap(),
        serde_json::json!(9_007_199_254_740_992_u64)
    );
}

/// Tests the conservative partial guild model used by `Application::guild`.
#[test]
fn application_guild_is_conservative() {
    let guild: ApplicationGuild = serde_json::from_str(
        r#"{
            "id": "290926798626357260",
            "name": "Developer Support",
            "icon": "icon-hash",
            "features": ["VERIFIED", "UNKNOWN_FEATURE"],
            "owner_id": "172150183260323840",
            "verification_level": 3,
            "premium_tier": 2,
            "unmodeled": true
        }"#,
    )
    .unwrap();

    assert_eq!(guild.id.get(), 290_926_798_626_357_260);
    assert_eq!(guild.name.as_deref(), Some("Developer Support"));
    assert_eq!(guild.icon.as_deref(), Some("icon-hash"));
    let features = guild.features.as_ref().unwrap();
    assert_eq!(features[0], GuildFeature::VERIFIED);
    assert_eq!(features[1].as_str(), "UNKNOWN_FEATURE");
    assert_eq!(guild.owner_id.unwrap().get(), 172_150_183_260_323_840);
    assert_eq!(guild.verification_level, Some(VerificationLevel::HIGH));
    assert_eq!(guild.premium_tier, Some(PremiumTier::TIER_2));

    let value = serde_json::to_value(&guild).unwrap();
    assert_eq!(value["id"], "290926798626357260");
    assert_eq!(value["features"][1], "UNKNOWN_FEATURE");
}

/// Tests a full application command object assembled from Discord's documented
/// fields and option examples.
#[test]
fn application_command_matches_documented_fields() {
    let command: ApplicationCommand = serde_json::from_str(
        r#"{
            "id": "847194950382780532",
            "type": 1,
            "application_id": "775799577604522054",
            "guild_id": "772904309264089089",
            "name": "blep",
            "name_localizations": {"fr": "bêler"},
            "description": "Send a random adorable animal photo",
            "description_localizations": {"fr": "Envoyer une photo d'animal"},
            "options": [
                {
                    "type": 3,
                    "name": "animal",
                    "name_localizations": null,
                    "description": "The type of animal",
                    "description_localizations": null,
                    "required": true,
                    "choices": [
                        {"name": "Dog", "value": "animal_dog"},
                        {"name": "Cat", "value": "animal_cat"}
                    ]
                },
                {
                    "type": 4,
                    "name": "count",
                    "description": "How many",
                    "min_value": 1,
                    "max_value": 10
                },
                {
                    "type": 10,
                    "name": "ratio",
                    "description": "Ratio",
                    "min_value": 0.5,
                    "max_value": 1.5
                },
                {
                    "type": 7,
                    "name": "channel",
                    "description": "Channel",
                    "channel_types": [0, 5]
                },
                {
                    "type": 11,
                    "name": "file",
                    "description": "File",
                    "file_types": ["image", ".pdf"]
                }
            ],
            "default_member_permissions": "2048",
            "dm_permission": false,
            "default_permission": true,
            "nsfw": false,
            "integration_types": [0, 1],
            "contexts": [0, 1, 2],
            "version": "847194950382780533",
            "handler": 1
        }"#,
    )
    .unwrap();

    assert_eq!(command.id.get(), 847_194_950_382_780_532);
    assert_eq!(command.kind, ApplicationCommandType::CHAT_INPUT);
    assert_eq!(command.application_id.get(), 775_799_577_604_522_054);
    assert_eq!(command.guild_id.unwrap().get(), 772_904_309_264_089_089);
    assert_eq!(command.name_localizations.as_ref().unwrap()["fr"], "bêler");
    assert_eq!(
        command.description_localizations.as_ref().unwrap()["fr"],
        "Envoyer une photo d'animal"
    );
    assert_eq!(command.default_member_permissions.unwrap().bits(), 2048);
    assert_eq!(command.dm_permission, Some(false));
    assert_eq!(command.default_permission, Some(true));
    assert_eq!(command.nsfw, Some(false));

    let integration_types = command.integration_types.as_ref().unwrap();
    assert_eq!(integration_types[0], ApplicationIntegrationType::GUILD_INSTALL);
    assert_eq!(integration_types[1], ApplicationIntegrationType::USER_INSTALL);

    let contexts = command.contexts.as_ref().unwrap();
    assert_eq!(contexts[0], InteractionContextType::GUILD);
    assert_eq!(contexts[1], InteractionContextType::BOT_DM);
    assert_eq!(contexts[2], InteractionContextType::PRIVATE_CHANNEL);

    assert_eq!(command.version, 847_194_950_382_780_533);
    assert_eq!(command.handler, Some(ApplicationCommandHandler::APP_HANDLER));

    let options = command.options.as_ref().unwrap();
    assert_eq!(options.len(), 5);
    assert_eq!(options[0].kind, ApplicationCommandOptionType::STRING);
    assert_eq!(options[0].required, Some(true));
    assert_eq!(
        options[0].choices.as_ref().unwrap()[0].value,
        CommandOptionChoiceValue::String("animal_dog".into())
    );
    assert_eq!(options[1].min_value, Some(CommandOptionNumericValue::Integer(1)));
    assert_eq!(options[1].max_value, Some(CommandOptionNumericValue::Integer(10)));
    assert_eq!(options[2].min_value, Some(CommandOptionNumericValue::Number(0.5)));
    assert_eq!(options[2].max_value, Some(CommandOptionNumericValue::Number(1.5)));
    let channel_types = options[3].channel_types.as_ref().unwrap();
    assert_eq!(channel_types[0], ChannelType::GUILD_TEXT);
    assert_eq!(channel_types[1], ChannelType::GUILD_ANNOUNCEMENT);
    let file_types = options[4].file_types.as_ref().unwrap();
    assert_eq!(file_types[0], "image");
    assert_eq!(file_types[1], ".pdf");

    let value = serde_json::to_value(&command).unwrap();
    assert_eq!(value["type"], 1);
    assert_eq!(value["version"], "847194950382780533");
    assert_eq!(value["default_member_permissions"], "2048");
    assert_eq!(value["options"][1]["min_value"], 1);
    assert_eq!(value["options"][2]["min_value"], 0.5);
    assert_eq!(value["options"][0]["choices"][0]["value"], "animal_dog");
    assert_eq!(value["integration_types"][0], 0);
    assert_eq!(value["contexts"][2], 2);
    assert_eq!(value["handler"], 1);
    assert_eq!(value["name_localizations"]["fr"], "bêler");
}

/// Tests that the command type defaults to `CHAT_INPUT` and that nullable
/// command fields serialize as `null`.
#[test]
fn application_command_defaults_and_nullable_fields() {
    let command: ApplicationCommand = serde_json::from_str(
        r#"{
            "id": "1",
            "application_id": "2",
            "name": "test",
            "description": "Test",
            "default_member_permissions": null,
            "version": "3"
        }"#,
    )
    .unwrap();

    assert_eq!(command.kind, ApplicationCommandType::CHAT_INPUT);
    assert!(command.name_localizations.is_none());
    assert!(command.description_localizations.is_none());
    assert!(command.default_member_permissions.is_none());
    assert!(command.default_permission.is_none());
    assert!(command.contexts.is_none());
    assert!(command.options.is_none());
    assert!(command.guild_id.is_none());
    assert!(command.dm_permission.is_none());
    assert!(command.nsfw.is_none());
    assert!(command.integration_types.is_none());
    assert!(command.handler.is_none());

    let value = serde_json::to_value(&command).unwrap();
    assert_eq!(value["type"], 1);
    assert_eq!(value["version"], "3");
    assert!(value["name_localizations"].is_null());
    assert!(value["description_localizations"].is_null());
    assert!(value["default_member_permissions"].is_null());
    assert!(value["default_permission"].is_null());
    assert!(value["contexts"].is_null());
    assert!(value.get("options").is_none());
    assert!(value.get("guild_id").is_none());
    assert!(value.get("dm_permission").is_none());
    assert!(value.get("nsfw").is_none());
    assert!(value.get("integration_types").is_none());
    assert!(value.get("handler").is_none());
}

/// Tests guild application command permissions, including permission constants
/// and unknown permission types.
#[test]
fn application_command_permissions_match_documented_fields() {
    let permissions: GuildApplicationCommandPermissions = serde_json::from_str(
        r#"{
            "id": "772904309264089089",
            "application_id": "775799577604522054",
            "guild_id": "772904309264089089",
            "permissions": [
                {"id": "772904309264089089", "type": 1, "permission": true},
                {"id": "772904309264089088", "type": 3, "permission": false},
                {"id": "167348773423415296", "type": 2, "permission": true},
                {"id": "772904309264089087", "type": 9, "permission": true}
            ]
        }"#,
    )
    .unwrap();

    assert_eq!(permissions.id, 772_904_309_264_089_089);
    assert_eq!(permissions.application_id.get(), 775_799_577_604_522_054);
    assert_eq!(permissions.guild_id.get(), 772_904_309_264_089_089);
    assert_eq!(permissions.permissions.len(), 4);
    assert_eq!(permissions.permissions[0].kind, ApplicationCommandPermissionType::ROLE);
    assert_eq!(permissions.permissions[0].id, 772_904_309_264_089_089);
    assert!(permissions.permissions[0].permission);
    assert_eq!(permissions.permissions[1].kind, ApplicationCommandPermissionType::CHANNEL);
    assert_eq!(permissions.permissions[1].id, 772_904_309_264_089_088);
    assert!(!permissions.permissions[1].permission);
    assert_eq!(permissions.permissions[2].kind, ApplicationCommandPermissionType::USER);
    assert_eq!(permissions.permissions[2].id, 167_348_773_423_415_296);
    assert_eq!(permissions.permissions[3].kind.raw(), 9);

    let value = serde_json::to_value(&permissions).unwrap();
    assert_eq!(value["id"], "772904309264089089");
    assert_eq!(value["permissions"][1]["id"], "772904309264089088");
    assert_eq!(value["permissions"][1]["type"], 3);
    assert_eq!(value["permissions"][3]["type"], 9);
}

/// Tests application role connection metadata and its type enumeration.
#[test]
fn application_role_connection_metadata_matches_documented_fields() {
    let metadata: ApplicationRoleConnectionMetadata = serde_json::from_str(
        r#"{
            "type": 1,
            "key": "cookies_eaten",
            "name": "Cookies eaten",
            "name_localizations": {"fr": "Cookies mangés"},
            "description": "Number of cookies eaten",
            "description_localizations": {"fr": "Nombre de cookies mangés"}
        }"#,
    )
    .unwrap();

    assert_eq!(metadata.kind, ApplicationRoleConnectionMetadataType::INTEGER_LESS_THAN_OR_EQUAL);
    assert_eq!(metadata.key, "cookies_eaten");
    assert_eq!(metadata.name, "Cookies eaten");
    assert_eq!(metadata.name_localizations.as_ref().unwrap()["fr"], "Cookies mangés");
    assert_eq!(metadata.description, "Number of cookies eaten");
    assert_eq!(
        metadata.description_localizations.as_ref().unwrap()["fr"],
        "Nombre de cookies mangés"
    );

    let value = serde_json::to_value(&metadata).unwrap();
    assert_eq!(value["type"], 1);
    assert_eq!(value["key"], "cookies_eaten");
    assert_eq!(value["name_localizations"]["fr"], "Cookies mangés");

    let unknown: ApplicationRoleConnectionMetadata =
        serde_json::from_str(r#"{"type": 99, "key": "k", "name": "N", "description": "D"}"#)
            .unwrap();
    assert_eq!(unknown.kind.raw(), 99);
}

/// Tests the team object embedded in the application example.
#[test]
fn team_matches_application_example() {
    let team: Team = serde_json::from_str(
        r#"{
            "name": "A team",
            "icon": "dd9b7dcfdf5351b9c3de0fe167bacbe1",
            "id": "531992624043786253",
            "owner_user_id": "511972282709709995",
            "members": [
                {
                    "membership_state": 2,
                    "permissions": ["*"],
                    "team_id": "531992624043786253",
                    "user": {
                        "avatar": "d9e261cd35999608eb7e3de1fae3688b",
                        "discriminator": "0001",
                        "id": "511972282709709995",
                        "username": "Mr Owner"
                    }
                }
            ]
        }"#,
    )
    .unwrap();

    assert_eq!(team.id.get(), 531_992_624_043_786_253);
    assert_eq!(team.name, "A team");
    assert_eq!(team.icon.as_deref(), Some("dd9b7dcfdf5351b9c3de0fe167bacbe1"));
    assert_eq!(team.owner_user_id.get(), 511_972_282_709_709_995);
    assert_eq!(team.members.len(), 1);
    assert_eq!(team.members[0].membership_state, TeamMembershipState::ACCEPTED);
    assert_eq!(team.members[0].team_id.get(), 531_992_624_043_786_253);
    assert_eq!(team.members[0].user.username, "Mr Owner");
    assert!(team.members[0].role.is_none());

    let value = serde_json::to_value(&team).unwrap();
    assert_eq!(value["id"], "531992624043786253");
    assert_eq!(value["owner_user_id"], "511972282709709995");
    assert_eq!(value["members"][0]["membership_state"], 2);
    assert!(value["members"][0].get("role").is_none());
    assert_eq!(value["members"][0]["user"]["id"], "511972282709709995");
}

/// Tests team member roles, including unknown role strings.
#[test]
fn team_member_role_preserves_unknown_values() {
    let member: TeamMember = serde_json::from_str(
        r#"{
            "membership_state": 1,
            "team_id": "531992624043786253",
            "user": {
                "id": "511972282709709995",
                "username": "Mr Owner",
                "discriminator": "0001"
            },
            "role": "developer"
        }"#,
    )
    .unwrap();

    assert_eq!(member.membership_state, TeamMembershipState::INVITED);
    assert_eq!(member.role, Some(TeamMemberRole::DEVELOPER));

    let value = serde_json::to_value(&member).unwrap();
    assert_eq!(value["membership_state"], 1);
    assert_eq!(value["role"], "developer");

    let unknown: TeamMember = serde_json::from_str(
        r#"{
            "membership_state": 3,
            "team_id": "1",
            "user": {"id": "2", "username": "user", "discriminator": "0"},
            "role": "auditor"
        }"#,
    )
    .unwrap();

    assert_eq!(unknown.membership_state.raw(), 3);
    assert_eq!(unknown.role.as_ref().map(TeamMemberRole::as_str), Some("auditor"));
}

/// Tests that unknown application command and webhook status values are
/// preserved.
#[test]
fn application_unknown_enum_values_are_preserved() {
    let handler: ApplicationCommandHandler = serde_json::from_str("9").unwrap();
    assert_eq!(handler.raw(), 9);

    let status: ApplicationEventWebhookStatus = serde_json::from_str("4").unwrap();
    assert_eq!(status.raw(), 4);

    let membership: TeamMembershipState = serde_json::from_str("3").unwrap();
    assert_eq!(membership.raw(), 3);
}

/// Tests the voice region object from Discord's voice resource.
#[test]
fn voice_region_matches_documented_fields() {
    let region: VoiceRegion = serde_json::from_str(
        r#"{
            "id": "us-east",
            "name": "US East",
            "optimal": true,
            "deprecated": false,
            "custom": false
        }"#,
    )
    .unwrap();

    assert_eq!(region.id, "us-east");
    assert_eq!(region.name, "US East");
    assert!(region.optimal);
    assert!(!region.deprecated);
    assert!(!region.custom);

    let value = serde_json::to_value(&region).unwrap();
    assert_eq!(value["id"], "us-east");
    assert_eq!(value["optimal"], true);
}

/// Tests the subscription example from Discord's subscription resource.
#[test]
fn subscription_matches_discord_example() {
    let subscription: Subscription = serde_json::from_str(
        r#"{
            "id": "1278078770116427839",
            "user_id": "1088605110638227537",
            "sku_ids": ["1158857122189168803"],
            "entitlement_ids": ["1158857122189168804"],
            "renewal_sku_ids": null,
            "current_period_start": "2024-08-27T19:48:44.406602+00:00",
            "current_period_end": "2024-09-27T19:48:44.406602+00:00",
            "status": 0,
            "canceled_at": null
        }"#,
    )
    .unwrap();

    assert_eq!(subscription.id.get(), 1_278_078_770_116_427_839);
    assert_eq!(subscription.user_id.get(), 1_088_605_110_638_227_537);
    assert_eq!(subscription.sku_ids.len(), 1);
    assert_eq!(subscription.sku_ids[0].get(), 1_158_857_122_189_168_803);
    assert_eq!(subscription.entitlement_ids.len(), 1);
    assert_eq!(subscription.entitlement_ids[0].get(), 1_158_857_122_189_168_804);
    assert!(subscription.renewal_sku_ids.is_none());
    assert_eq!(subscription.status, SubscriptionStatus::ACTIVE);
    assert!(subscription.canceled_at.is_none());
    assert!(subscription.country.is_none());

    let value = serde_json::to_value(&subscription).unwrap();
    assert_eq!(value["id"], "1278078770116427839");
    assert_eq!(value["sku_ids"][0], "1158857122189168803");
    assert!(value["renewal_sku_ids"].is_null());
    assert_eq!(value["current_period_start"], "2024-08-27T19:48:44.406602+00:00");
    assert_eq!(value["status"], 0);
    assert!(value["canceled_at"].is_null());
    assert!(value.get("country").is_none());
}

/// Tests that subscription statuses preserve unknown values.
#[test]
fn subscription_status_preserves_unknown_values() {
    let status: SubscriptionStatus = serde_json::from_str("9").unwrap();

    assert_eq!(status.raw(), 9);
    assert_eq!(serde_json::to_value(status).unwrap(), serde_json::json!(9));
}

/// Tests the sticker pack example from Discord's sticker resource.
#[test]
fn sticker_pack_matches_discord_example() {
    let pack: StickerPack = serde_json::from_str(
        r#"{
            "id": "847199849233514549",
            "stickers": [
                {
                    "id": "749054660769218631",
                    "name": "Wave",
                    "tags": "wumpus, hello, wave",
                    "type": 1,
                    "format_type": 3,
                    "description": "Wumpus waves hello",
                    "pack_id": "847199849233514549",
                    "sort_value": 12
                }
            ],
            "name": "Wumpus Beyond",
            "sku_id": "847199849233514547",
            "cover_sticker_id": "749053689419006003",
            "description": "Say hello to Wumpus!",
            "banner_asset_id": "761773777976819732"
        }"#,
    )
    .unwrap();

    assert_eq!(pack.id.get(), 847_199_849_233_514_549);
    assert_eq!(pack.name, "Wumpus Beyond");
    assert_eq!(pack.sku_id.get(), 847_199_849_233_514_547);
    assert_eq!(pack.cover_sticker_id.unwrap().get(), 749_053_689_419_006_003);
    assert_eq!(pack.description, "Say hello to Wumpus!");
    assert_eq!(pack.banner_asset_id, Some(761_773_777_976_819_732));
    assert_eq!(pack.stickers.len(), 1);
    assert_eq!(pack.stickers[0].name, "Wave");
    assert_eq!(pack.stickers[0].pack_id.unwrap().get(), 847_199_849_233_514_549);

    let value = serde_json::to_value(&pack).unwrap();
    assert_eq!(value["id"], "847199849233514549");
    assert_eq!(value["sku_id"], "847199849233514547");
    assert_eq!(value["cover_sticker_id"], "749053689419006003");
    assert_eq!(value["banner_asset_id"], "761773777976819732");

    let minimal: StickerPack = serde_json::from_str(
        r#"{
            "id": "1",
            "stickers": [],
            "name": "Pack",
            "sku_id": "2",
            "description": "Description"
        }"#,
    )
    .unwrap();
    let value = serde_json::to_value(&minimal).unwrap();
    assert!(value.get("cover_sticker_id").is_none());
    assert!(value.get("banner_asset_id").is_none());
}

/// Tests a full guild integration assembled from Discord's documented fields.
#[test]
fn integration_matches_documented_fields() {
    let integration: Integration = serde_json::from_str(
        r#"{
            "id": "33590653072239123",
            "name": "A Name",
            "type": "twitch",
            "enabled": true,
            "syncing": true,
            "role_id": "123456789012345678",
            "enable_emoticons": true,
            "expire_behavior": 0,
            "expire_grace_period": 7,
            "user": {
                "id": "80351110224678912",
                "username": "Nelly",
                "discriminator": "1337"
            },
            "account": {"name": "twitchusername", "id": "1234567"},
            "synced_at": "2023-09-01T14:48:02.222000+00:00",
            "subscriber_count": 100,
            "revoked": false,
            "application": {
                "id": "94651234501213162",
                "name": "Test App",
                "icon": null,
                "description": "Test application",
                "bot": {
                    "id": "94651234501213163",
                    "username": "TestBot",
                    "discriminator": "0001"
                }
            },
            "scopes": ["identify", "guilds"]
        }"#,
    )
    .unwrap();

    assert_eq!(integration.id.get(), 33_590_653_072_239_123);
    assert_eq!(integration.name, "A Name");
    assert_eq!(integration.kind, IntegrationType::TWITCH);
    assert!(integration.enabled);
    assert_eq!(integration.syncing, Some(true));
    assert_eq!(integration.role_id.unwrap().get(), 123_456_789_012_345_678);
    assert_eq!(integration.enable_emoticons, Some(true));
    assert_eq!(integration.expire_behavior, Some(IntegrationExpireBehavior::REMOVE_ROLE));
    assert_eq!(integration.expire_grace_period, Some(7));
    assert_eq!(integration.account.id, "1234567");
    assert_eq!(integration.subscriber_count, Some(100));
    assert_eq!(integration.revoked, Some(false));
    assert_eq!(integration.scopes.as_ref().unwrap().len(), 2);

    let application = integration.application.as_ref().unwrap();
    assert_eq!(application.id.get(), 94_651_234_501_213_162);
    assert!(application.icon.is_none());
    assert_eq!(application.bot.as_ref().unwrap().username.as_deref(), Some("TestBot"));

    let value = serde_json::to_value(&integration).unwrap();
    assert_eq!(value["type"], "twitch");
    assert_eq!(value["role_id"], "123456789012345678");
    assert_eq!(value["expire_behavior"], 0);
    assert_eq!(value["synced_at"], "2023-09-01T14:48:02.222+00:00");
    assert!(value["application"]["icon"].is_null());
}

/// Tests that unknown integration types and expire behaviors are preserved.
#[test]
fn integration_preserves_unknown_values() {
    let kind: IntegrationType = serde_json::from_str("\"new_service\"").unwrap();
    assert_eq!(kind.as_str(), "new_service");

    let behavior: IntegrationExpireBehavior = serde_json::from_str("9").unwrap();
    assert_eq!(behavior.raw(), 9);
}

/// Tests the partial integration example from Discord's audit log resource.
#[test]
fn partial_integration_matches_audit_log_example() {
    let integration: PartialIntegration = serde_json::from_str(
        r#"{
            "id": "33590653072239123",
            "name": "A Name",
            "type": "twitch",
            "account": {"name": "twitchusername", "id": "1234567"},
            "application_id": "94651234501213162"
        }"#,
    )
    .unwrap();

    assert_eq!(integration.id.get(), 33_590_653_072_239_123);
    assert_eq!(integration.name, "A Name");
    assert_eq!(integration.kind, IntegrationType::TWITCH);
    assert_eq!(integration.account.name, "twitchusername");
    assert_eq!(integration.application_id.unwrap().get(), 94_651_234_501_213_162);

    let value = serde_json::to_value(&integration).unwrap();
    assert_eq!(value["id"], "33590653072239123");
    assert_eq!(value["type"], "twitch");
    assert_eq!(value["application_id"], "94651234501213162");
}

/// Tests an audit log containing changes, optional entry info, and an unknown
/// action type.
#[test]
fn audit_log_matches_documented_fields() {
    let audit_log: AuditLog = serde_json::from_str(
        r#"{
            "application_commands": [],
            "audit_log_entries": [
                {
                    "target_id": "167348773423415296",
                    "changes": [
                        {"new_value": "new nick", "old_value": "old nick", "key": "nick"}
                    ],
                    "user_id": "167348773423415296",
                    "id": "772904309264089089",
                    "action_type": 24,
                    "reason": "test reason"
                },
                {
                    "target_id": null,
                    "user_id": null,
                    "id": "772904309264089090",
                    "action_type": 999,
                    "options": {
                        "application_id": "775799577604522054",
                        "auto_moderation_rule_name": "Spam",
                        "auto_moderation_rule_trigger_type": "1",
                        "channel_id": "772904309264089088",
                        "count": "2",
                        "delete_member_days": "1",
                        "id": "772904309264089087",
                        "members_removed": "3",
                        "message_id": "772904309264089086",
                        "role_name": "Moderator",
                        "type": "0",
                        "integration_type": "discord",
                        "status": "live"
                    }
                }
            ],
            "auto_moderation_rules": [],
            "guild_scheduled_events": [],
            "integrations": [
                {
                    "id": "33590653072239123",
                    "name": "A Name",
                    "type": "twitch",
                    "account": {"name": "twitchusername", "id": "1234567"},
                    "application_id": "94651234501213162"
                }
            ],
            "threads": [],
            "users": [],
            "webhooks": []
        }"#,
    )
    .unwrap();

    assert!(audit_log.application_commands.is_empty());
    assert!(audit_log.auto_moderation_rules.is_empty());
    assert_eq!(audit_log.integrations.len(), 1);
    assert_eq!(audit_log.audit_log_entries.len(), 2);

    let entry = &audit_log.audit_log_entries[0];
    assert_eq!(entry.id.get(), 772_904_309_264_089_089);
    assert_eq!(entry.target_id.as_deref(), Some("167348773423415296"));
    assert_eq!(entry.user_id.unwrap().get(), 167_348_773_423_415_296);
    assert_eq!(entry.action_type, AuditLogEvent::MEMBER_UPDATE);
    assert_eq!(entry.reason.as_deref(), Some("test reason"));
    let changes = entry.changes.as_ref().unwrap();
    assert_eq!(changes[0].key, "nick");
    assert_eq!(changes[0].new_value.as_ref().unwrap(), "new nick");
    assert_eq!(changes[0].old_value.as_ref().unwrap(), "old nick");

    let unknown = &audit_log.audit_log_entries[1];
    assert_eq!(unknown.action_type.raw(), 999);
    assert!(unknown.target_id.is_none());
    assert!(unknown.user_id.is_none());
    assert!(unknown.changes.is_none());

    let options = unknown.options.as_ref().unwrap();
    assert_eq!(options.application_id.unwrap().get(), 775_799_577_604_522_054);
    assert_eq!(options.auto_moderation_rule_name.as_deref(), Some("Spam"));
    assert_eq!(options.auto_moderation_rule_trigger_type.as_deref(), Some("1"));
    assert_eq!(options.channel_id.unwrap().get(), 772_904_309_264_089_088);
    assert_eq!(options.count.as_deref(), Some("2"));
    assert_eq!(options.delete_member_days.as_deref(), Some("1"));
    assert_eq!(options.id, Some(772_904_309_264_089_087));
    assert_eq!(options.members_removed.as_deref(), Some("3"));
    assert_eq!(options.message_id.unwrap().get(), 772_904_309_264_089_086);
    assert_eq!(options.role_name.as_deref(), Some("Moderator"));
    assert_eq!(options.kind.as_deref(), Some("0"));
    assert_eq!(options.integration_type.as_deref(), Some("discord"));
    assert_eq!(options.status.as_deref(), Some("live"));

    let value = serde_json::to_value(&audit_log).unwrap();
    assert_eq!(value["audit_log_entries"][0]["action_type"], 24);
    assert_eq!(value["audit_log_entries"][0]["changes"][0]["new_value"], "new nick");
    assert_eq!(value["audit_log_entries"][1]["action_type"], 999);
    assert!(value["audit_log_entries"][1]["target_id"].is_null());
    assert!(value["audit_log_entries"][1]["user_id"].is_null());
    assert_eq!(value["audit_log_entries"][1]["options"]["id"], "772904309264089087");
    assert_eq!(value["audit_log_entries"][1]["options"]["type"], "0");
    assert_eq!(value["integrations"][0]["id"], "33590653072239123");
}

/// Tests the connection object from Discord's user resource.
#[test]
fn connection_matches_documented_fields() {
    let connection: Connection = serde_json::from_str(
        r#"{
            "id": "1234567890",
            "name": "username",
            "type": "twitch",
            "revoked": false,
            "integrations": [
                {
                    "id": "33590653072239123",
                    "name": "A Name",
                    "type": "twitch",
                    "account": {"name": "twitchusername", "id": "1234567"},
                    "application_id": "94651234501213162"
                }
            ],
            "verified": true,
            "friend_sync": true,
            "show_activity": true,
            "two_way_link": false,
            "visibility": 1
        }"#,
    )
    .unwrap();

    assert_eq!(connection.id, "1234567890");
    assert_eq!(connection.name, "username");
    assert_eq!(connection.kind, ConnectionService::TWITCH);
    assert_eq!(connection.revoked, Some(false));
    assert_eq!(connection.integrations.as_ref().unwrap().len(), 1);
    assert!(connection.verified);
    assert!(connection.friend_sync);
    assert!(connection.show_activity);
    assert!(!connection.two_way_link);
    assert_eq!(connection.visibility, ConnectionVisibility::EVERYONE);

    let value = serde_json::to_value(&connection).unwrap();
    assert_eq!(value["type"], "twitch");
    assert_eq!(value["visibility"], 1);
    assert_eq!(value["integrations"][0]["id"], "33590653072239123");

    let unknown: ConnectionService = serde_json::from_str("\"new_service\"").unwrap();
    assert_eq!(unknown.as_str(), "new_service");

    let visibility: ConnectionVisibility = serde_json::from_str("9").unwrap();
    assert_eq!(visibility.raw(), 9);
}

/// Tests the application role connection object from Discord's user resource.
#[test]
fn application_role_connection_matches_documented_fields() {
    let connection: ApplicationRoleConnection = serde_json::from_str(
        r#"{
            "platform_name": "GitHub",
            "platform_username": "octocat",
            "metadata": {"cookies_eaten": "12"}
        }"#,
    )
    .unwrap();

    assert_eq!(connection.platform_name.as_deref(), Some("GitHub"));
    assert_eq!(connection.platform_username.as_deref(), Some("octocat"));
    assert_eq!(connection.metadata.as_ref().unwrap()["cookies_eaten"], "12");

    let value = serde_json::to_value(&connection).unwrap();
    assert_eq!(value["platform_name"], "GitHub");
    assert_eq!(value["metadata"]["cookies_eaten"], "12");

    let empty: ApplicationRoleConnection =
        serde_json::from_str(r#"{"platform_name": null, "platform_username": null}"#).unwrap();
    let value = serde_json::to_value(&empty).unwrap();
    assert!(value["platform_name"].is_null());
    assert!(value["platform_username"].is_null());
    assert!(value.get("metadata").is_none());
}

/// Tests the Gateway envelope around the `READY` payload from Discord's
/// documentation.
#[test]
fn gateway_envelope_decodes_ready_payload() {
    let payload = r##"{
        "op": 0,
        "d": {
            "v": 10,
            "user": {
                "id": "80351110224678912",
                "username": "Nelly",
                "discriminator": "1337",
                "avatar": "8342729096ea3675442027381ff50dfe",
                "email": "nelly@discord.com"
            },
            "guilds": [
                {"id": "41771983423143937", "unavailable": true},
                {"id": "41771983423143938", "unavailable": false}
            ],
            "session_id": "UoCGli1Gt5tMBFCbaI1Wt7on",
            "resume_gateway_url": "wss://us-east1-b.gateway.discord.media",
            "shard": [0, 1],
            "application": {
                "id": "788708323867885999",
                "flags": 123456,
                "flags_new": "123456"
            }
        },
        "s": 1,
        "t": "READY"
    }"##;

    let envelope: GatewayPayload<Ready> = serde_json::from_str(payload).unwrap();

    assert_eq!(envelope.op, GatewayOpcode::DISPATCH);
    assert_eq!(envelope.s, Some(1));
    assert_eq!(envelope.t, Some(GatewayEventName::READY));

    let ready = envelope.d.as_ref().unwrap();
    assert_eq!(ready.v, 10);
    assert_eq!(ready.user.id.get(), 80_351_110_224_678_912);
    assert_eq!(ready.user.email.as_deref(), Some("nelly@discord.com"));
    assert_eq!(ready.guilds.len(), 2);
    assert_eq!(ready.guilds[0].id.get(), 41_771_983_423_143_937);
    assert!(ready.guilds[0].unavailable);
    assert_eq!(ready.guilds[1].id.get(), 41_771_983_423_143_938);
    assert!(!ready.guilds[1].unavailable);
    assert_eq!(ready.session_id, "UoCGli1Gt5tMBFCbaI1Wt7on");
    assert_eq!(ready.resume_gateway_url, "wss://us-east1-b.gateway.discord.media");
    assert_eq!(ready.shard, Some([0, 1]));
    let application: &ReadyApplication = &ready.application;
    assert_eq!(application.id.get(), 788_708_323_867_885_999);
    assert_eq!(application.flags, Some(ApplicationFlags::from_bits_retain(123_456)));
    assert_eq!(application.flags_new, Some(ApplicationFlags::from_bits_retain(123_456)));

    let value = serde_json::to_value(envelope).unwrap();
    assert_eq!(value["op"], 0);
    assert_eq!(value["s"], 1);
    assert_eq!(value["t"], "READY");
    assert_eq!(value["d"]["application"]["flags"], 123_456);
    assert_eq!(value["d"]["application"]["flags_new"], "123456");
    assert_eq!(value["d"]["guilds"][0]["unavailable"], true);
    assert_eq!(value["d"]["shard"], serde_json::json!([0, 1]));
}

/// Tests that `s` and `t` only appear on dispatch payloads and that the
/// optional fields tolerate both `null` and absence.
#[test]
fn gateway_payload_sequence_and_name_are_dispatch_only() {
    let dispatched: GatewayPayload<Hello> = serde_json::from_str(
        r#"{"op": 0, "d": {"heartbeat_interval": 45000}, "s": 42, "t": "FUTURE_EVENT"}"#,
    )
    .unwrap();

    assert_eq!(dispatched.s, Some(42));
    assert_eq!(dispatched.t, Some(GatewayEventName::new("FUTURE_EVENT")));
    assert_eq!(dispatched.d.unwrap().heartbeat_interval, 45_000);

    let minimal: GatewayPayload<Hello> =
        serde_json::from_str(r#"{"op": 0, "d": {"heartbeat_interval": 1}}"#).unwrap();

    assert_eq!(minimal.s, None);
    assert_eq!(minimal.t, None);

    let nulled: GatewayPayload<Hello> = serde_json::from_str(
        r#"{"op": 10, "d": {"heartbeat_interval": 45000}, "s": null, "t": null}"#,
    )
    .unwrap();

    assert_eq!(nulled.op, GatewayOpcode::HELLO);
    assert_eq!(nulled.s, None);
    assert_eq!(nulled.t, None);

    let value = serde_json::to_value(dispatched).unwrap();
    assert_eq!(value["s"], 42);
    assert_eq!(value["t"], "FUTURE_EVENT");
}

/// Tests that the Heartbeat send payload carries the last sequence number and
/// that payloads without data decode as `None`.
#[test]
fn gateway_payload_heartbeat_carries_sequence_number() {
    let heartbeat: GatewayPayload<u64> = serde_json::from_str(r#"{"op": 1, "d": 251}"#).unwrap();

    assert_eq!(heartbeat.op, GatewayOpcode::HEARTBEAT);
    assert_eq!(heartbeat.d, Some(251));
    assert_eq!(heartbeat.s, None);
    assert_eq!(heartbeat.t, None);

    assert_eq!(serde_json::to_value(heartbeat).unwrap()["d"], 251);

    let without_sequence: GatewayPayload<u64> =
        serde_json::from_str(r#"{"op": 1, "d": null}"#).unwrap();

    assert_eq!(without_sequence.d, None);
    assert_eq!(serde_json::to_value(without_sequence).unwrap()["d"], serde_json::Value::Null);

    let reconnect: GatewayPayload<serde_json::Value> =
        serde_json::from_str(r#"{"op": 7, "d": null}"#).unwrap();

    assert_eq!(reconnect.op, GatewayOpcode::RECONNECT);
    assert!(reconnect.d.is_none());

    let ack: GatewayPayload<serde_json::Value> =
        serde_json::from_str(r#"{"op": 11, "d": null}"#).unwrap();

    assert_eq!(ack.op, GatewayOpcode::HEARTBEAT_ACK);
    assert!(ack.d.is_none());
}

/// Tests every documented Gateway opcode and that unknown opcodes survive.
#[test]
fn gateway_opcode_round_trips_known_and_unknown_values() {
    for (raw, expected) in [
        (0, GatewayOpcode::DISPATCH),
        (1, GatewayOpcode::HEARTBEAT),
        (2, GatewayOpcode::IDENTIFY),
        (3, GatewayOpcode::PRESENCE_UPDATE),
        (4, GatewayOpcode::VOICE_STATE_UPDATE),
        (6, GatewayOpcode::RESUME),
        (7, GatewayOpcode::RECONNECT),
        (8, GatewayOpcode::REQUEST_GUILD_MEMBERS),
        (9, GatewayOpcode::INVALID_SESSION),
        (10, GatewayOpcode::HELLO),
        (11, GatewayOpcode::HEARTBEAT_ACK),
        (31, GatewayOpcode::REQUEST_SOUNDBOARD_SOUNDS),
        (43, GatewayOpcode::REQUEST_CHANNEL_INFO),
    ] {
        let opcode: GatewayOpcode = serde_json::from_str(&raw.to_string()).unwrap();

        assert_eq!(opcode, expected);
        assert_eq!(opcode.raw(), raw);
        assert_eq!(serde_json::to_value(opcode).unwrap(), serde_json::json!(raw));
    }

    let unknown: GatewayOpcode = serde_json::from_str("99").unwrap();

    assert_eq!(unknown.raw(), 99);
    assert_eq!(serde_json::to_value(unknown).unwrap(), serde_json::json!(99));
}

/// Tests Gateway event name constants and preservation of unknown names.
#[test]
fn gateway_event_name_round_trips_constants_and_unknown_values() {
    assert_eq!(GatewayEventName::READY.as_str(), "READY");
    assert_eq!(GatewayEventName::RESUMED.as_str(), "RESUMED");
    assert_eq!(GatewayEventName::RECONNECT.as_str(), "RECONNECT");
    assert_eq!(GatewayEventName::RATE_LIMITED.as_str(), "RATE_LIMITED");
    assert_eq!(GatewayEventName::INVALID_SESSION.as_str(), "INVALID_SESSION");
    assert_eq!(
        GatewayEventName::APPLICATION_COMMAND_PERMISSIONS_UPDATE.as_str(),
        "APPLICATION_COMMAND_PERMISSIONS_UPDATE"
    );
    assert_eq!(
        GatewayEventName::AUTO_MODERATION_RULE_CREATE.as_str(),
        "AUTO_MODERATION_RULE_CREATE"
    );
    assert_eq!(
        GatewayEventName::AUTO_MODERATION_RULE_UPDATE.as_str(),
        "AUTO_MODERATION_RULE_UPDATE"
    );
    assert_eq!(
        GatewayEventName::AUTO_MODERATION_RULE_DELETE.as_str(),
        "AUTO_MODERATION_RULE_DELETE"
    );
    assert_eq!(
        GatewayEventName::AUTO_MODERATION_ACTION_EXECUTION.as_str(),
        "AUTO_MODERATION_ACTION_EXECUTION"
    );
    assert_eq!(GatewayEventName::CHANNEL_CREATE.as_str(), "CHANNEL_CREATE");
    assert_eq!(GatewayEventName::CHANNEL_UPDATE.as_str(), "CHANNEL_UPDATE");
    assert_eq!(GatewayEventName::CHANNEL_DELETE.as_str(), "CHANNEL_DELETE");
    assert_eq!(GatewayEventName::CHANNEL_INFO.as_str(), "CHANNEL_INFO");
    assert_eq!(GatewayEventName::CHANNEL_PINS_UPDATE.as_str(), "CHANNEL_PINS_UPDATE");
    assert_eq!(GatewayEventName::THREAD_CREATE.as_str(), "THREAD_CREATE");
    assert_eq!(GatewayEventName::THREAD_UPDATE.as_str(), "THREAD_UPDATE");
    assert_eq!(GatewayEventName::THREAD_DELETE.as_str(), "THREAD_DELETE");
    assert_eq!(GatewayEventName::THREAD_LIST_SYNC.as_str(), "THREAD_LIST_SYNC");
    assert_eq!(GatewayEventName::THREAD_MEMBER_UPDATE.as_str(), "THREAD_MEMBER_UPDATE");
    assert_eq!(GatewayEventName::THREAD_MEMBERS_UPDATE.as_str(), "THREAD_MEMBERS_UPDATE");
    assert_eq!(GatewayEventName::ENTITLEMENT_CREATE.as_str(), "ENTITLEMENT_CREATE");
    assert_eq!(GatewayEventName::ENTITLEMENT_UPDATE.as_str(), "ENTITLEMENT_UPDATE");
    assert_eq!(GatewayEventName::ENTITLEMENT_DELETE.as_str(), "ENTITLEMENT_DELETE");
    assert_eq!(GatewayEventName::GUILD_CREATE.as_str(), "GUILD_CREATE");
    assert_eq!(GatewayEventName::GUILD_UPDATE.as_str(), "GUILD_UPDATE");
    assert_eq!(GatewayEventName::GUILD_DELETE.as_str(), "GUILD_DELETE");
    assert_eq!(
        GatewayEventName::GUILD_AUDIT_LOG_ENTRY_CREATE.as_str(),
        "GUILD_AUDIT_LOG_ENTRY_CREATE"
    );
    assert_eq!(GatewayEventName::GUILD_BAN_ADD.as_str(), "GUILD_BAN_ADD");
    assert_eq!(GatewayEventName::GUILD_BAN_REMOVE.as_str(), "GUILD_BAN_REMOVE");
    assert_eq!(GatewayEventName::GUILD_EMOJIS_UPDATE.as_str(), "GUILD_EMOJIS_UPDATE");
    assert_eq!(GatewayEventName::GUILD_STICKERS_UPDATE.as_str(), "GUILD_STICKERS_UPDATE");
    assert_eq!(
        GatewayEventName::GUILD_INTEGRATIONS_UPDATE.as_str(),
        "GUILD_INTEGRATIONS_UPDATE"
    );
    assert_eq!(GatewayEventName::GUILD_MEMBER_ADD.as_str(), "GUILD_MEMBER_ADD");
    assert_eq!(GatewayEventName::GUILD_MEMBER_REMOVE.as_str(), "GUILD_MEMBER_REMOVE");
    assert_eq!(GatewayEventName::GUILD_MEMBER_UPDATE.as_str(), "GUILD_MEMBER_UPDATE");
    assert_eq!(GatewayEventName::GUILD_MEMBERS_CHUNK.as_str(), "GUILD_MEMBERS_CHUNK");
    assert_eq!(GatewayEventName::GUILD_ROLE_CREATE.as_str(), "GUILD_ROLE_CREATE");
    assert_eq!(GatewayEventName::GUILD_ROLE_UPDATE.as_str(), "GUILD_ROLE_UPDATE");
    assert_eq!(GatewayEventName::GUILD_ROLE_DELETE.as_str(), "GUILD_ROLE_DELETE");
    assert_eq!(
        GatewayEventName::GUILD_SCHEDULED_EVENT_CREATE.as_str(),
        "GUILD_SCHEDULED_EVENT_CREATE"
    );
    assert_eq!(
        GatewayEventName::GUILD_SCHEDULED_EVENT_UPDATE.as_str(),
        "GUILD_SCHEDULED_EVENT_UPDATE"
    );
    assert_eq!(
        GatewayEventName::GUILD_SCHEDULED_EVENT_DELETE.as_str(),
        "GUILD_SCHEDULED_EVENT_DELETE"
    );
    assert_eq!(
        GatewayEventName::GUILD_SCHEDULED_EVENT_USER_ADD.as_str(),
        "GUILD_SCHEDULED_EVENT_USER_ADD"
    );
    assert_eq!(
        GatewayEventName::GUILD_SCHEDULED_EVENT_USER_REMOVE.as_str(),
        "GUILD_SCHEDULED_EVENT_USER_REMOVE"
    );
    assert_eq!(
        GatewayEventName::GUILD_SOUNDBOARD_SOUND_CREATE.as_str(),
        "GUILD_SOUNDBOARD_SOUND_CREATE"
    );
    assert_eq!(
        GatewayEventName::GUILD_SOUNDBOARD_SOUND_UPDATE.as_str(),
        "GUILD_SOUNDBOARD_SOUND_UPDATE"
    );
    assert_eq!(
        GatewayEventName::GUILD_SOUNDBOARD_SOUND_DELETE.as_str(),
        "GUILD_SOUNDBOARD_SOUND_DELETE"
    );
    assert_eq!(
        GatewayEventName::GUILD_SOUNDBOARD_SOUNDS_UPDATE.as_str(),
        "GUILD_SOUNDBOARD_SOUNDS_UPDATE"
    );
    assert_eq!(GatewayEventName::SOUNDBOARD_SOUNDS.as_str(), "SOUNDBOARD_SOUNDS");
    assert_eq!(GatewayEventName::INTEGRATION_CREATE.as_str(), "INTEGRATION_CREATE");
    assert_eq!(GatewayEventName::INTEGRATION_UPDATE.as_str(), "INTEGRATION_UPDATE");
    assert_eq!(GatewayEventName::INTEGRATION_DELETE.as_str(), "INTEGRATION_DELETE");
    assert_eq!(GatewayEventName::INTERACTION_CREATE.as_str(), "INTERACTION_CREATE");
    assert_eq!(GatewayEventName::INVITE_CREATE.as_str(), "INVITE_CREATE");
    assert_eq!(GatewayEventName::INVITE_DELETE.as_str(), "INVITE_DELETE");
    assert_eq!(GatewayEventName::MESSAGE_CREATE.as_str(), "MESSAGE_CREATE");
    assert_eq!(GatewayEventName::MESSAGE_UPDATE.as_str(), "MESSAGE_UPDATE");
    assert_eq!(GatewayEventName::MESSAGE_DELETE.as_str(), "MESSAGE_DELETE");
    assert_eq!(GatewayEventName::MESSAGE_DELETE_BULK.as_str(), "MESSAGE_DELETE_BULK");
    assert_eq!(GatewayEventName::MESSAGE_REACTION_ADD.as_str(), "MESSAGE_REACTION_ADD");
    assert_eq!(GatewayEventName::MESSAGE_REACTION_REMOVE.as_str(), "MESSAGE_REACTION_REMOVE");
    assert_eq!(
        GatewayEventName::MESSAGE_REACTION_REMOVE_ALL.as_str(),
        "MESSAGE_REACTION_REMOVE_ALL"
    );
    assert_eq!(
        GatewayEventName::MESSAGE_REACTION_REMOVE_EMOJI.as_str(),
        "MESSAGE_REACTION_REMOVE_EMOJI"
    );
    assert_eq!(GatewayEventName::PRESENCE_UPDATE.as_str(), "PRESENCE_UPDATE");
    assert_eq!(GatewayEventName::STAGE_INSTANCE_CREATE.as_str(), "STAGE_INSTANCE_CREATE");
    assert_eq!(GatewayEventName::STAGE_INSTANCE_UPDATE.as_str(), "STAGE_INSTANCE_UPDATE");
    assert_eq!(GatewayEventName::STAGE_INSTANCE_DELETE.as_str(), "STAGE_INSTANCE_DELETE");
    assert_eq!(GatewayEventName::SUBSCRIPTION_CREATE.as_str(), "SUBSCRIPTION_CREATE");
    assert_eq!(GatewayEventName::SUBSCRIPTION_UPDATE.as_str(), "SUBSCRIPTION_UPDATE");
    assert_eq!(GatewayEventName::SUBSCRIPTION_DELETE.as_str(), "SUBSCRIPTION_DELETE");
    assert_eq!(GatewayEventName::TYPING_START.as_str(), "TYPING_START");
    assert_eq!(GatewayEventName::USER_UPDATE.as_str(), "USER_UPDATE");
    assert_eq!(
        GatewayEventName::VOICE_CHANNEL_EFFECT_SEND.as_str(),
        "VOICE_CHANNEL_EFFECT_SEND"
    );
    assert_eq!(
        GatewayEventName::VOICE_CHANNEL_START_TIME_UPDATE.as_str(),
        "VOICE_CHANNEL_START_TIME_UPDATE"
    );
    assert_eq!(
        GatewayEventName::VOICE_CHANNEL_STATUS_UPDATE.as_str(),
        "VOICE_CHANNEL_STATUS_UPDATE"
    );
    assert_eq!(GatewayEventName::VOICE_STATE_UPDATE.as_str(), "VOICE_STATE_UPDATE");
    assert_eq!(GatewayEventName::VOICE_SERVER_UPDATE.as_str(), "VOICE_SERVER_UPDATE");
    assert_eq!(GatewayEventName::WEBHOOKS_UPDATE.as_str(), "WEBHOOKS_UPDATE");
    assert_eq!(GatewayEventName::MESSAGE_POLL_VOTE_ADD.as_str(), "MESSAGE_POLL_VOTE_ADD");
    assert_eq!(GatewayEventName::MESSAGE_POLL_VOTE_REMOVE.as_str(), "MESSAGE_POLL_VOTE_REMOVE");

    assert_eq!(GatewayEventName::from("READY"), GatewayEventName::READY);
    assert_eq!(GatewayEventName::new(String::from("READY")), GatewayEventName::READY);

    let unknown: GatewayEventName = serde_json::from_str("\"SOME_FUTURE_EVENT\"").unwrap();

    assert_eq!(unknown.as_str(), "SOME_FUTURE_EVENT");
    assert_eq!(serde_json::to_value(unknown).unwrap(), serde_json::json!("SOME_FUTURE_EVENT"));
}

/// Tests the example Identify payload from Discord's documentation.
#[test]
fn identify_matches_discord_example() {
    let identify: Identify = serde_json::from_str(
        r#"{
            "token": "my_token",
            "properties": {
                "os": "linux",
                "browser": "disco",
                "device": "disco"
            },
            "compress": true,
            "large_threshold": 250,
            "shard": [0, 1],
            "presence": {
                "activities": [{
                    "name": "Cards Against Humanity",
                    "type": 0,
                    "created_at": 1511200066000
                }],
                "status": "dnd",
                "since": 91879201,
                "afk": false
            },
            "intents": 5,
            "capabilities": 32769
        }"#,
    )
    .unwrap();

    let properties: &IdentifyConnectionProperties = &identify.properties;

    assert_eq!(identify.token, "my_token");
    assert_eq!(identify.properties.os, "linux");
    assert_eq!(properties.browser, "disco");
    assert_eq!(identify.properties.browser, "disco");
    assert_eq!(identify.properties.device, "disco");
    assert_eq!(identify.compress, Some(true));
    assert_eq!(identify.large_threshold, Some(250));
    assert_eq!(identify.shard, Some([0, 1]));
    assert_eq!(identify.intents, 5);
    assert_eq!(identify.capabilities, Some(32_769));

    let presence = identify.presence.as_ref().unwrap();
    assert_eq!(presence.activities[0].kind, ActivityType::PLAYING);
    assert_eq!(presence.status, UserStatus::DND);
    assert_eq!(presence.since, Some(91_879_201));
    assert!(!presence.afk);

    let value = serde_json::to_value(identify).unwrap();
    assert_eq!(value["compress"], true);
    assert_eq!(value["large_threshold"], 250);
    assert_eq!(value["shard"], serde_json::json!([0, 1]));
    assert_eq!(value["presence"]["since"], 91_879_201);
    assert_eq!(value["intents"], 5);
    assert_eq!(value["capabilities"], 32_769);
}

/// Tests that Identify omits its optional fields when they are absent.
#[test]
fn identify_omits_absent_optional_fields() {
    let identify: Identify = serde_json::from_str(
        r#"{
            "token": "my_token",
            "properties": {"os": "linux", "browser": "cordion", "device": "cordion"},
            "intents": 5
        }"#,
    )
    .unwrap();

    assert_eq!(identify.compress, None);
    assert_eq!(identify.large_threshold, None);
    assert_eq!(identify.shard, None);
    assert_eq!(identify.presence, None);
    assert_eq!(identify.capabilities, None);

    let value = serde_json::to_value(identify).unwrap();
    for field in ["compress", "large_threshold", "shard", "presence", "capabilities"] {
        assert!(value.get(field).is_none(), "{field} should be omitted");
    }
}

/// Tests the example Resume payload from Discord's documentation.
#[test]
fn resume_matches_discord_example() {
    let resume: Resume = serde_json::from_str(
        r#"{
            "token": "randomstring",
            "session_id": "evenmorerandomstring",
            "seq": 1337
        }"#,
    )
    .unwrap();

    assert_eq!(resume.token, "randomstring");
    assert_eq!(resume.session_id, "evenmorerandomstring");
    assert_eq!(resume.seq, 1337);

    let value = serde_json::to_value(resume).unwrap();
    assert_eq!(value["seq"], 1337);
}

/// Tests the example Request Guild Members payload and its alternatives.
#[test]
fn request_guild_members_matches_discord_example() {
    let request: RequestGuildMembers = serde_json::from_str(
        r#"{
            "guild_id": "41771983444115456",
            "query": "",
            "limit": 0
        }"#,
    )
    .unwrap();

    assert_eq!(request.guild_id.get(), 41_771_983_444_115_456);
    assert_eq!(request.query.as_deref(), Some(""));
    assert_eq!(request.limit, Some(0));
    assert_eq!(request.presences, None);
    assert_eq!(request.user_ids, None);
    assert_eq!(request.nonce, None);

    let value = serde_json::to_value(request).unwrap();
    assert_eq!(value["guild_id"], "41771983444115456");
    assert_eq!(value["limit"], 0);
    for field in ["presences", "user_ids", "nonce"] {
        assert!(value.get(field).is_none(), "{field} should be omitted");
    }

    let with_ids: RequestGuildMembers = serde_json::from_str(
        r#"{
            "guild_id": "41771983444115456",
            "user_ids": ["80351110224678912", "80351110224678913"],
            "presences": true,
            "nonce": "nonce"
        }"#,
    )
    .unwrap();

    let Some(RequestGuildMembersUserIds::Many(ids)) = with_ids.user_ids.as_ref() else {
        panic!("expected an array of user IDs");
    };
    assert_eq!(ids.len(), 2);
    assert!(with_ids.presences.unwrap());
    assert_eq!(with_ids.nonce.as_deref(), Some("nonce"));

    let value = serde_json::to_value(with_ids).unwrap();
    assert_eq!(value["user_ids"], serde_json::json!(["80351110224678912", "80351110224678913"]));
}

/// Tests that Request Guild Members accepts a single user ID.
#[test]
fn request_guild_members_accepts_single_user_id() {
    let request: RequestGuildMembers = serde_json::from_str(
        r#"{"guild_id": "41771983444115456", "user_ids": "80351110224678912"}"#,
    )
    .unwrap();

    let Some(RequestGuildMembersUserIds::One(id)) = request.user_ids else {
        panic!("expected a single user ID");
    };
    assert_eq!(id.get(), 80_351_110_224_678_912);

    let value = serde_json::to_value(request).unwrap();
    assert_eq!(value["user_ids"], "80351110224678912");
}

/// Tests the example Request Soundboard Sounds payload.
#[test]
fn request_soundboard_sounds_matches_discord_example() {
    let request: RequestSoundboardSounds =
        serde_json::from_str(r#"{"guild_ids": ["613425648685547541", "81384788765712384"]}"#)
            .unwrap();

    assert_eq!(request.guild_ids.len(), 2);
    assert_eq!(request.guild_ids[0].get(), 613_425_648_685_547_541);
    assert_eq!(request.guild_ids[1].get(), 81_384_788_765_712_384);

    let value = serde_json::to_value(request).unwrap();
    assert_eq!(
        value["guild_ids"],
        serde_json::json!(["613425648685547541", "81384788765712384"])
    );
}

/// Tests the example Request Channel Info payload.
#[test]
fn request_channel_info_matches_discord_example() {
    let request: RequestChannelInfo = serde_json::from_str(
        r#"{
            "guild_id": "613425648685547541",
            "fields": ["status", "voice_start_time"]
        }"#,
    )
    .unwrap();

    assert_eq!(request.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(request.fields, ["status", "voice_start_time"]);

    let value = serde_json::to_value(request).unwrap();
    assert_eq!(value["guild_id"], "613425648685547541");
    assert_eq!(value["fields"], serde_json::json!(["status", "voice_start_time"]));
}

/// Tests the example Gateway Voice State Update payload and its null channel.
#[test]
fn gateway_voice_state_update_matches_discord_example() {
    let update: GatewayVoiceStateUpdate = serde_json::from_str(
        r#"{
            "guild_id": "41771983423143937",
            "channel_id": "127121515262115840",
            "self_mute": false,
            "self_deaf": false
        }"#,
    )
    .unwrap();

    assert_eq!(update.guild_id.get(), 41_771_983_423_143_937);
    assert_eq!(update.channel_id.unwrap().get(), 127_121_515_262_115_840);
    assert!(!update.self_mute);
    assert!(!update.self_deaf);

    let value = serde_json::to_value(update).unwrap();
    assert_eq!(value["channel_id"], "127121515262115840");

    let disconnected: GatewayVoiceStateUpdate = serde_json::from_str(
        r#"{"guild_id": "1", "channel_id": null, "self_mute": true, "self_deaf": true}"#,
    )
    .unwrap();

    assert_eq!(disconnected.channel_id, None);
    assert!(disconnected.self_mute);
    assert!(disconnected.self_deaf);

    let value = serde_json::to_value(disconnected).unwrap();
    assert_eq!(value["channel_id"], serde_json::Value::Null);
}

/// Tests the example Gateway Presence Update payload from Discord's
/// documentation.
#[test]
fn gateway_presence_update_matches_discord_example() {
    let presence: GatewayPresenceUpdate = serde_json::from_str(
        r#"{
            "since": 91879201,
            "activities": [{
                "name": "Save the Oxford Comma",
                "type": 0,
                "created_at": 1511200066000
            }],
            "status": "online",
            "afk": false
        }"#,
    )
    .unwrap();

    assert_eq!(presence.since, Some(91_879_201));
    assert_eq!(presence.activities[0].name, "Save the Oxford Comma");
    assert_eq!(presence.status, UserStatus::ONLINE);
    assert!(!presence.afk);

    let value = serde_json::to_value(presence).unwrap();
    assert_eq!(value["since"], 91_879_201);
    assert_eq!(value["status"], "online");
    assert_eq!(value["afk"], false);

    let idle: GatewayPresenceUpdate =
        serde_json::from_str(r#"{"since": null, "activities": [], "status": "idle", "afk": true}"#)
            .unwrap();

    assert_eq!(idle.since, None);
    assert!(idle.afk);

    let value = serde_json::to_value(idle).unwrap();
    assert_eq!(value["since"], serde_json::Value::Null);
    assert_eq!(value["status"], "idle");
}

/// Tests the Channel Info payload and its nullable channel fields.
#[test]
fn channel_info_decodes_ephemeral_channel_data() {
    let info: ChannelInfo = serde_json::from_str(
        r#"{
            "guild_id": "613425648685547541",
            "channels": [{
                "id": "41771983423143937",
                "status": "Very loud noises",
                "voice_start_time": 1720704000
            }, {
                "id": "41771983423143938",
                "status": null
            }]
        }"#,
    )
    .unwrap();

    assert_eq!(info.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(info.channels.len(), 2);

    let voice: &ChannelInfoChannel = &info.channels[0];
    assert_eq!(voice.id.get(), 41_771_983_423_143_937);
    assert_eq!(voice.status.as_deref(), Some("Very loud noises"));
    assert_eq!(voice.voice_start_time, Some(1_720_704_000));

    let missing = &info.channels[1];
    assert_eq!(missing.status, None);
    assert_eq!(missing.voice_start_time, None);

    let value = serde_json::to_value(info).unwrap();
    assert_eq!(value["channels"][0]["voice_start_time"], 1_720_704_000);
    assert_eq!(value["channels"][1]["status"], serde_json::Value::Null);
    assert_eq!(value["channels"][1]["voice_start_time"], serde_json::Value::Null);
}

/// Tests the example Voice Channel Status Update payload.
#[test]
fn voice_channel_status_update_round_trips() {
    let update: VoiceChannelStatusUpdate = serde_json::from_str(
        r#"{
            "id": "41771983423143937",
            "guild_id": "613425648685547541",
            "status": "Very loud noises"
        }"#,
    )
    .unwrap();

    assert_eq!(update.id.get(), 41_771_983_423_143_937);
    assert_eq!(update.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(update.status.as_deref(), Some("Very loud noises"));

    let cleared: VoiceChannelStatusUpdate =
        serde_json::from_str(r#"{"id": "1", "guild_id": "2", "status": null}"#).unwrap();

    assert_eq!(cleared.status, None);
    assert_eq!(serde_json::to_value(cleared).unwrap()["status"], serde_json::Value::Null);
}

/// Tests the Voice Channel Start Time Update payload and its nullable field.
#[test]
fn voice_channel_start_time_update_round_trips() {
    let update: VoiceChannelStartTimeUpdate = serde_json::from_str(
        r#"{
            "id": "41771983423143937",
            "guild_id": "613425648685547541",
            "voice_start_time": 1720704000
        }"#,
    )
    .unwrap();

    assert_eq!(update.id.get(), 41_771_983_423_143_937);
    assert_eq!(update.voice_start_time, Some(1_720_704_000));

    let cleared: VoiceChannelStartTimeUpdate =
        serde_json::from_str(r#"{"id": "1", "guild_id": "2"}"#).unwrap();

    assert_eq!(cleared.voice_start_time, None);
    assert_eq!(
        serde_json::to_value(cleared).unwrap()["voice_start_time"],
        serde_json::Value::Null
    );
}

/// Tests the Thread List Sync payload with and without parent channel IDs.
#[test]
fn thread_list_sync_round_trips() {
    let sync: ThreadListSync = serde_json::from_str(
        r#"{
            "guild_id": "197038439483310086",
            "channel_ids": ["1099032703076216840"],
            "threads": [{
                "id": "1099032703076216842",
                "type": 15,
                "guild_id": "197038439483310086",
                "parent_id": "1099032703076216840"
            }],
            "members": [{
                "id": "1099032703076216842",
                "user_id": "80351110224678912",
                "join_timestamp": "2021-04-12T23:40:39.855793+00:00",
                "flags": 0
            }]
        }"#,
    )
    .unwrap();

    assert_eq!(sync.guild_id.get(), 197_038_439_483_310_086);
    let channel_ids = sync.channel_ids.as_ref().unwrap();
    assert_eq!(channel_ids[0].get(), 1_099_032_703_076_216_840);
    assert_eq!(sync.threads[0].kind, ChannelType::GUILD_FORUM);
    assert_eq!(sync.threads[0].parent_id.unwrap().get(), 1_099_032_703_076_216_840);
    assert_eq!(sync.members[0].join_timestamp, timestamp(1_618_270_839, 855_793_000));

    let value = serde_json::to_value(sync).unwrap();
    assert_eq!(value["threads"][0]["type"], 15);
    assert_eq!(value["members"][0]["join_timestamp"], "2021-04-12T23:40:39.855793+00:00");

    let whole_guild: ThreadListSync =
        serde_json::from_str(r#"{"guild_id": "1", "threads": [], "members": []}"#).unwrap();

    assert_eq!(whole_guild.channel_ids, None);
    let value = serde_json::to_value(whole_guild).unwrap();
    assert!(value.get("channel_ids").is_none());
}

/// Tests the Thread Members Update payload and its optional arrays.
#[test]
fn thread_members_update_round_trips() {
    let update: ThreadMembersUpdate = serde_json::from_str(
        r#"{
            "id": "1099032703076216842",
            "guild_id": "197038439483310086",
            "member_count": 50,
            "added_members": [{
                "id": "1099032703076216842",
                "user_id": "80351110224678912",
                "join_timestamp": "2021-04-12T23:40:39.855793+00:00",
                "flags": 0,
                "member": {
                    "roles": ["41771983423143936"],
                    "joined_at": "2015-04-26T06:26:56.936000+00:00",
                    "deaf": false,
                    "mute": false
                }
            }],
            "removed_member_ids": ["80351110224678913"]
        }"#,
    )
    .unwrap();

    assert_eq!(update.id.get(), 1_099_032_703_076_216_842);
    assert_eq!(update.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(update.member_count, 50);

    let added = update.added_members.as_ref().unwrap();
    assert_eq!(added.len(), 1);
    let member = added[0].member.as_ref().unwrap();
    assert_eq!(member.roles[0].get(), 41_771_983_423_143_936);

    let removed = update.removed_member_ids.as_ref().unwrap();
    assert_eq!(removed[0].get(), 80_351_110_224_678_913);

    let value = serde_json::to_value(update).unwrap();
    assert_eq!(value["member_count"], 50);
    assert_eq!(
        value["added_members"][0]["member"]["joined_at"],
        "2015-04-26T06:26:56.936+00:00"
    );
    assert_eq!(value["removed_member_ids"][0], "80351110224678913");

    let without_changes: ThreadMembersUpdate =
        serde_json::from_str(r#"{"id": "1", "guild_id": "2", "member_count": 1}"#).unwrap();

    assert_eq!(without_changes.added_members, None);
    assert_eq!(without_changes.removed_member_ids, None);

    let value = serde_json::to_value(without_changes).unwrap();
    assert!(value.get("added_members").is_none());
    assert!(value.get("removed_member_ids").is_none());
}

/// Tests the Channel Pins Update payload and its nullable timestamp.
#[test]
fn channel_pins_update_round_trips() {
    let pinned: ChannelPinsUpdate = serde_json::from_str(
        r#"{
            "guild_id": "197038439483310086",
            "channel_id": "1099032703076216842",
            "last_pin_timestamp": "2021-04-12T23:40:39.855793+00:00"
        }"#,
    )
    .unwrap();

    assert_eq!(pinned.guild_id.unwrap().get(), 197_038_439_483_310_086);
    assert_eq!(pinned.channel_id.get(), 1_099_032_703_076_216_842);
    assert_eq!(pinned.last_pin_timestamp, Some(timestamp(1_618_270_839, 855_793_000)));

    let value = serde_json::to_value(pinned).unwrap();
    assert_eq!(value["last_pin_timestamp"], "2021-04-12T23:40:39.855793+00:00");

    let unpinned: ChannelPinsUpdate =
        serde_json::from_str(r#"{"channel_id": "1", "last_pin_timestamp": null}"#).unwrap();

    assert_eq!(unpinned.guild_id, None);
    assert_eq!(unpinned.last_pin_timestamp, None);

    let value = serde_json::to_value(unpinned).unwrap();
    assert!(value.get("guild_id").is_none());
    assert_eq!(value["last_pin_timestamp"], serde_json::Value::Null);
}

/// Tests the documented Invite Create fields, including nullable expiry.
#[test]
fn invite_create_matches_documented_fields() {
    let invite: InviteCreate = serde_json::from_str(
        r#"{
            "channel_id": "165176875973476352",
            "code": "0vCdhLbwjZZTWZLD",
            "created_at": "2016-03-31T19:15:39.954000+00:00",
            "guild_id": "165176875973476352",
            "inviter": {
                "id": "115590097100865541",
                "username": "speed",
                "discriminator": "7653"
            },
            "max_age": 3600,
            "max_uses": 0,
            "target_type": 1,
            "target_user": {
                "id": "165176875973476352",
                "username": "bob",
                "discriminator": "1234"
            },
            "temporary": false,
            "uses": 0,
            "expires_at": null,
            "role_ids": ["41771983423143936"]
        }"#,
    )
    .unwrap();

    assert_eq!(invite.channel_id.get(), 165_176_875_973_476_352);
    assert_eq!(invite.code, "0vCdhLbwjZZTWZLD");
    assert_eq!(invite.created_at, timestamp(1_459_451_739, 954_000_000));
    assert_eq!(invite.guild_id.unwrap().get(), 165_176_875_973_476_352);
    assert_eq!(invite.inviter.as_ref().unwrap().username, "speed");
    assert_eq!(invite.max_age, 3600);
    assert_eq!(invite.max_uses, 0);
    assert_eq!(invite.target_type, Some(InviteTargetType::STREAM));
    assert_eq!(invite.target_user.as_ref().unwrap().username, "bob");
    assert_eq!(invite.target_application, None);
    assert!(!invite.temporary);
    assert_eq!(invite.uses, 0);
    assert_eq!(invite.expires_at, None);
    let role_ids = invite.role_ids.as_ref().unwrap();
    assert_eq!(role_ids[0].get(), 41_771_983_423_143_936);

    let value = serde_json::to_value(invite).unwrap();
    assert_eq!(value["code"], "0vCdhLbwjZZTWZLD");
    assert_eq!(value["created_at"], "2016-03-31T19:15:39.954+00:00");
    assert_eq!(value["target_type"], 1);
    assert_eq!(value["expires_at"], serde_json::Value::Null);
    assert_eq!(value["role_ids"][0], "41771983423143936");
    assert!(value.get("target_application").is_none());
}

/// Tests the Invite Delete payload, whose guild is optional.
#[test]
fn invite_delete_round_trips() {
    let with_guild: InviteDelete = serde_json::from_str(
        r#"{"channel_id": "165176875973476352", "guild_id": "165176875973476352", "code": "abc"}"#,
    )
    .unwrap();

    assert_eq!(with_guild.channel_id.get(), 165_176_875_973_476_352);
    assert_eq!(with_guild.guild_id.unwrap().get(), 165_176_875_973_476_352);
    assert_eq!(with_guild.code, "abc");

    let without_guild: InviteDelete =
        serde_json::from_str(r#"{"channel_id": "1", "code": "abc"}"#).unwrap();

    assert_eq!(without_guild.guild_id, None);
    let value = serde_json::to_value(without_guild).unwrap();
    assert!(value.get("guild_id").is_none());
}

/// Tests the Message Delete and Message Delete Bulk payloads.
#[test]
fn message_delete_events_omit_absent_guild() {
    let delete: MessageDelete = serde_json::from_str(
        r#"{"id": "123456789123456790", "channel_id": "157733188964188161", "guild_id": "197038439483310086"}"#,
    )
    .unwrap();

    assert_eq!(delete.id.get(), 123_456_789_123_456_790);
    assert_eq!(delete.channel_id.get(), 157_733_188_964_188_161);
    assert_eq!(delete.guild_id.unwrap().get(), 197_038_439_483_310_086);

    let value = serde_json::to_value(delete).unwrap();
    assert_eq!(value["id"], "123456789123456790");

    let bulk: MessageDeleteBulk = serde_json::from_str(
        r#"{
            "ids": ["123456789123456790", "123456789123456791"],
            "channel_id": "157733188964188161"
        }"#,
    )
    .unwrap();

    assert_eq!(bulk.ids.len(), 2);
    assert_eq!(bulk.ids[1].get(), 123_456_789_123_456_791);
    assert_eq!(bulk.channel_id.get(), 157_733_188_964_188_161);
    assert_eq!(bulk.guild_id, None);

    let value = serde_json::to_value(bulk).unwrap();
    assert_eq!(value["ids"][0], "123456789123456790");
    assert!(value.get("guild_id").is_none());
}

/// Tests the Message Reaction Add and Remove payloads.
#[test]
fn message_reaction_add_and_remove_round_trip() {
    let add: MessageReactionAdd = serde_json::from_str(
        r##"{
            "user_id": "80351110224678912",
            "channel_id": "157733188964188161",
            "message_id": "123456789123456790",
            "guild_id": "197038439483310086",
            "member": {
                "roles": ["41771983423143936"],
                "joined_at": "2015-04-26T06:26:56.936000+00:00",
                "deaf": false,
                "mute": false
            },
            "emoji": {"id": "41771983429993937", "name": "LUL", "animated": false},
            "message_author_id": "80351110224678913",
            "burst": true,
            "burst_colors": ["#ff0000"],
            "type": 1
        }"##,
    )
    .unwrap();

    assert_eq!(add.user_id.get(), 80_351_110_224_678_912);
    assert_eq!(add.channel_id.get(), 157_733_188_964_188_161);
    assert_eq!(add.message_id.get(), 123_456_789_123_456_790);
    assert_eq!(add.guild_id.unwrap().get(), 197_038_439_483_310_086);
    assert_eq!(add.member.as_ref().unwrap().roles[0].get(), 41_771_983_423_143_936);
    assert_eq!(add.emoji.id.unwrap().get(), 41_771_983_429_993_937);
    assert_eq!(add.emoji.name.as_deref(), Some("LUL"));
    assert_eq!(add.message_author_id.unwrap().get(), 80_351_110_224_678_913);
    assert!(add.burst);
    assert_eq!(add.burst_colors.as_ref().unwrap(), &["#ff0000".to_owned()]);
    assert_eq!(add.kind, ReactionType::BURST);

    let value = serde_json::to_value(add).unwrap();
    assert_eq!(value["type"], 1);
    assert_eq!(value["burst"], true);
    assert_eq!(value["burst_colors"][0], "#ff0000");
    assert_eq!(value["emoji"]["name"], "LUL");

    let remove: MessageReactionRemove = serde_json::from_str(
        r#"{
            "user_id": "80351110224678912",
            "channel_id": "157733188964188161",
            "message_id": "123456789123456790",
            "emoji": {"id": null, "name": "🔥"},
            "burst": false,
            "type": 0
        }"#,
    )
    .unwrap();

    assert_eq!(remove.guild_id, None);
    assert_eq!(remove.emoji.id, None);
    assert_eq!(remove.emoji.name.as_deref(), Some("🔥"));
    assert!(!remove.burst);
    assert_eq!(remove.kind, ReactionType::NORMAL);

    let value = serde_json::to_value(remove).unwrap();
    assert_eq!(value["type"], 0);
    assert!(value.get("guild_id").is_none());
    assert!(value.get("burst_colors").is_none());
}

/// Tests the Message Reaction Remove All and Remove Emoji payloads.
#[test]
fn message_reaction_remove_all_and_emoji_round_trip() {
    let all: MessageReactionRemoveAll = serde_json::from_str(
        r#"{"channel_id": "157733188964188161", "message_id": "123456789123456790", "guild_id": "197038439483310086"}"#,
    )
    .unwrap();

    assert_eq!(all.channel_id.get(), 157_733_188_964_188_161);
    assert_eq!(all.message_id.get(), 123_456_789_123_456_790);
    assert_eq!(all.guild_id.unwrap().get(), 197_038_439_483_310_086);

    let emoji: MessageReactionRemoveEmoji = serde_json::from_str(
        r#"{
            "channel_id": "157733188964188161",
            "guild_id": "197038439483310086",
            "message_id": "123456789123456790",
            "emoji": {"id": "41771983429993937", "name": "LUL"}
        }"#,
    )
    .unwrap();

    assert_eq!(emoji.channel_id.get(), 157_733_188_964_188_161);
    assert_eq!(emoji.emoji.id.unwrap().get(), 41_771_983_429_993_937);

    let value = serde_json::to_value(emoji).unwrap();
    assert_eq!(value["emoji"]["id"], "41771983429993937");

    let without_guild: MessageReactionRemoveAll =
        serde_json::from_str(r#"{"channel_id": "1", "message_id": "2"}"#).unwrap();

    assert_eq!(without_guild.guild_id, None);
    let value = serde_json::to_value(without_guild).unwrap();
    assert!(value.get("guild_id").is_none());
}

/// Tests the Message Poll Vote Add and Remove payloads.
#[test]
fn message_poll_vote_events_round_trip() {
    let add: MessagePollVoteAdd = serde_json::from_str(
        r#"{
            "user_id": "80351110224678912",
            "channel_id": "157733188964188161",
            "message_id": "123456789123456790",
            "guild_id": "197038439483310086",
            "answer_id": 1
        }"#,
    )
    .unwrap();

    assert_eq!(add.user_id.get(), 80_351_110_224_678_912);
    assert_eq!(add.message_id.get(), 123_456_789_123_456_790);
    assert_eq!(add.answer_id, 1);

    let value = serde_json::to_value(add).unwrap();
    assert_eq!(value["answer_id"], 1);

    let remove: MessagePollVoteRemove = serde_json::from_str(
        r#"{
            "user_id": "80351110224678912",
            "channel_id": "157733188964188161",
            "message_id": "123456789123456790",
            "answer_id": 1
        }"#,
    )
    .unwrap();

    assert_eq!(remove.guild_id, None);
    assert_eq!(remove.answer_id, 1);

    let value = serde_json::to_value(remove).unwrap();
    assert!(value.get("guild_id").is_none());
}

/// Tests the Typing Start payload with and without its member object.
#[test]
fn typing_start_round_trips() {
    let typing: TypingStart = serde_json::from_str(
        r#"{
            "channel_id": "157733188964188161",
            "guild_id": "197038439483310086",
            "user_id": "80351110224678912",
            "timestamp": 1661529361,
            "member": {
                "roles": ["41771983423143936"],
                "joined_at": "2015-04-26T06:26:56.936000+00:00",
                "deaf": false,
                "mute": false
            }
        }"#,
    )
    .unwrap();

    assert_eq!(typing.channel_id.get(), 157_733_188_964_188_161);
    assert_eq!(typing.guild_id.unwrap().get(), 197_038_439_483_310_086);
    assert_eq!(typing.user_id.get(), 80_351_110_224_678_912);
    assert_eq!(typing.timestamp, 1_661_529_361);
    assert_eq!(
        typing.member.as_ref().unwrap().joined_at,
        Some(timestamp(1_430_029_616, 936_000_000))
    );

    let value = serde_json::to_value(typing).unwrap();
    assert_eq!(value["timestamp"], 1_661_529_361);

    let without_member: TypingStart =
        serde_json::from_str(r#"{"channel_id": "1", "user_id": "2", "timestamp": 1661529361}"#)
            .unwrap();

    assert_eq!(without_member.guild_id, None);
    assert_eq!(without_member.member, None);
    let value = serde_json::to_value(without_member).unwrap();
    assert!(value.get("guild_id").is_none());
    assert!(value.get("member").is_none());
}

/// Tests that unknown reaction types are preserved.
#[test]
fn reaction_type_preserves_unknown_value() {
    let unknown: ReactionType = serde_json::from_str("9").unwrap();

    assert_eq!(unknown.raw(), 9);
    assert_eq!(serde_json::to_value(unknown).unwrap(), serde_json::json!(9));

    assert_eq!(serde_json::from_str::<ReactionType>("0").unwrap(), ReactionType::NORMAL);
    assert_eq!(serde_json::from_str::<ReactionType>("1").unwrap(), ReactionType::BURST);
}

/// Tests the example Voice Server Update payload and its nullable endpoint.
#[test]
fn voice_server_update_matches_discord_example() {
    let update: VoiceServerUpdate = serde_json::from_str(
        r#"{
            "token": "my_token",
            "guild_id": "41771983423143937",
            "endpoint": "sweetwater-12345.discord.media:2048"
        }"#,
    )
    .unwrap();

    assert_eq!(update.token, "my_token");
    assert_eq!(update.guild_id.get(), 41_771_983_423_143_937);
    assert_eq!(update.endpoint.as_deref(), Some("sweetwater-12345.discord.media:2048"));

    let value = serde_json::to_value(update).unwrap();
    assert_eq!(value["endpoint"], "sweetwater-12345.discord.media:2048");

    let reallocating: VoiceServerUpdate =
        serde_json::from_str(r#"{"token": "my_token", "guild_id": "1", "endpoint": null}"#)
            .unwrap();

    assert_eq!(reallocating.endpoint, None);
    assert_eq!(serde_json::to_value(reallocating).unwrap()["endpoint"], serde_json::Value::Null);
}

/// Tests the Voice Channel Effect Send payload for both effect kinds.
#[test]
fn voice_channel_effect_send_round_trips() {
    let reaction: VoiceChannelEffectSend = serde_json::from_str(
        r#"{
            "channel_id": "41771983423143937",
            "guild_id": "613425648685547541",
            "user_id": "80351110224678912",
            "emoji": {"id": "989193655938064464", "name": "🐛", "animated": false},
            "animation_type": 0,
            "animation_id": 1
        }"#,
    )
    .unwrap();

    assert_eq!(reaction.channel_id.get(), 41_771_983_423_143_937);
    assert_eq!(reaction.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(reaction.user_id.get(), 80_351_110_224_678_912);
    assert_eq!(reaction.emoji.as_ref().unwrap().id.unwrap().get(), 989_193_655_938_064_464);
    assert_eq!(reaction.animation_type, Some(VoiceEffectAnimationType::PREMIUM));
    assert_eq!(reaction.animation_id, Some(1));
    assert_eq!(reaction.sound_id, None);
    assert_eq!(reaction.sound_volume, None);

    let value = serde_json::to_value(reaction).unwrap();
    assert_eq!(value["emoji"]["id"], "989193655938064464");
    assert_eq!(value["animation_type"], 0);
    assert_eq!(value["emoji"]["name"], "🐛");
    assert!(value.get("sound_id").is_none());
    assert!(value.get("sound_volume").is_none());
    assert_eq!(value["animation_id"], 1);

    let sound: VoiceChannelEffectSend = serde_json::from_str(
        r#"{
            "channel_id": "41771983423143937",
            "guild_id": "613425648685547541",
            "user_id": "80351110224678912",
            "sound_id": "1106714396018884649",
            "sound_volume": 1.0
        }"#,
    )
    .unwrap();

    assert_eq!(sound.sound_id.unwrap().get(), 1_106_714_396_018_884_649);
    assert_eq!(sound.animation_type, None);

    let value = serde_json::to_value(sound).unwrap();
    assert_eq!(value["sound_volume"], 1.0);
    assert_eq!(value["sound_id"], "1106714396018884649");
    assert_eq!(value["emoji"], serde_json::Value::Null);
    assert_eq!(value["animation_type"], serde_json::Value::Null);

    let untyped: VoiceChannelEffectSend = serde_json::from_str(
        r#"{
            "channel_id": "1",
            "guild_id": "2",
            "user_id": "3",
            "sound_id": 1106714396018884649,
            "animation_type": 1
        }"#,
    )
    .unwrap();

    assert_eq!(untyped.animation_type, Some(VoiceEffectAnimationType::BASIC));
}

/// Tests that unknown animation types are preserved.
#[test]
fn voice_effect_animation_type_preserves_unknown_value() {
    let unknown: VoiceEffectAnimationType = serde_json::from_str("9").unwrap();

    assert_eq!(unknown.raw(), 9);
    assert_eq!(serde_json::to_value(unknown).unwrap(), serde_json::json!(9));

    assert_eq!(
        serde_json::from_str::<VoiceEffectAnimationType>("0").unwrap(),
        VoiceEffectAnimationType::PREMIUM
    );
    assert_eq!(
        serde_json::from_str::<VoiceEffectAnimationType>("1").unwrap(),
        VoiceEffectAnimationType::BASIC
    );
}

/// Tests the example Hello payload from Discord's documentation.
#[test]
fn hello_matches_discord_example() {
    let hello: Hello = serde_json::from_str(r#"{"heartbeat_interval": 45000}"#).unwrap();

    assert_eq!(hello.heartbeat_interval, 45_000);
    assert_eq!(
        serde_json::to_value(hello).unwrap(),
        serde_json::json!({"heartbeat_interval": 45000})
    );
}

/// Tests the Auto Moderation Action Execution payload and its intent-gated
/// fields.
#[test]
fn auto_moderation_action_execution_matches_documented_fields() {
    let execution: AutoModerationActionExecution = serde_json::from_str(
        r#"{
            "guild_id": "613425648685547541",
            "action": {
                "type": 1,
                "metadata": {
                    "custom_message": "Please keep financial discussions limited to the #finance channel"
                }
            },
            "rule_id": "969707018069872670",
            "rule_trigger_type": 1,
            "user_id": "423457898095789043",
            "channel_id": "123456789123456789",
            "message_id": "123456789123456790",
            "alert_system_message_id": "123456789123456791",
            "content": "i like cats",
            "matched_keyword": "cat*",
            "matched_content": "cats"
        }"#,
    )
    .unwrap();

    assert_eq!(execution.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(execution.action.kind, AutoModerationActionType::BLOCK_MESSAGE);
    assert_eq!(
        execution.action.metadata.as_ref().unwrap().custom_message.as_deref(),
        Some("Please keep financial discussions limited to the #finance channel")
    );
    assert_eq!(execution.rule_id.get(), 969_707_018_069_872_670);
    assert_eq!(execution.rule_trigger_type, AutoModerationTriggerType::KEYWORD);
    assert_eq!(execution.user_id.get(), 423_457_898_095_789_043);
    assert_eq!(execution.channel_id.unwrap().get(), 123_456_789_123_456_789);
    assert_eq!(execution.message_id.unwrap().get(), 123_456_789_123_456_790);
    assert_eq!(execution.alert_system_message_id.unwrap().get(), 123_456_789_123_456_791);
    assert_eq!(execution.content.as_deref(), Some("i like cats"));
    assert_eq!(execution.matched_keyword.as_deref(), Some("cat*"));
    assert_eq!(execution.matched_content.as_deref(), Some("cats"));

    let value = serde_json::to_value(execution).unwrap();
    assert_eq!(value["matched_keyword"], "cat*");
    assert_eq!(value["content"], "i like cats");

    let blocked: AutoModerationActionExecution = serde_json::from_str(
        r#"{
            "guild_id": "613425648685547541",
            "action": {"type": 1},
            "rule_id": "969707018069872670",
            "rule_trigger_type": 1,
            "user_id": "423457898095789043",
            "content": null,
            "matched_keyword": null,
            "matched_content": null
        }"#,
    )
    .unwrap();

    assert_eq!(blocked.channel_id, None);
    assert_eq!(blocked.message_id, None);
    assert_eq!(blocked.alert_system_message_id, None);
    assert_eq!(blocked.content, None);
    assert_eq!(blocked.matched_keyword, None);
    assert_eq!(blocked.matched_content, None);

    let value = serde_json::to_value(blocked).unwrap();
    for field in ["channel_id", "message_id", "alert_system_message_id", "content"] {
        assert!(value.get(field).is_none(), "{field} should be omitted");
    }
    assert_eq!(value["matched_keyword"], serde_json::Value::Null);
    assert_eq!(value["matched_content"], serde_json::Value::Null);
}

/// Tests the Rate Limited payload and its opcode 8 metadata.
#[test]
fn rate_limited_round_trips() {
    let limited: RateLimited = serde_json::from_str(
        r#"{
            "opcode": 8,
            "retry_after": 5.0,
            "meta": {
                "guild_id": "41771983444115456",
                "nonce": "nonce"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(limited.opcode, GatewayOpcode::REQUEST_GUILD_MEMBERS);
    assert_eq!(limited.retry_after, 5.0);
    let meta: &RequestGuildMemberRateLimitMetadata = &limited.meta;
    assert_eq!(meta.guild_id.get(), 41_771_983_444_115_456);
    assert_eq!(limited.meta.nonce.as_deref(), Some("nonce"));

    let value = serde_json::to_value(limited).unwrap();
    assert_eq!(value["opcode"], 8);
    assert_eq!(value["retry_after"], 5.0);
    assert_eq!(value["meta"]["guild_id"], "41771983444115456");

    let without_nonce: RateLimited =
        serde_json::from_str(r#"{"opcode": 8, "retry_after": 12.5, "meta": {"guild_id": "1"}}"#)
            .unwrap();

    assert_eq!(without_nonce.meta.nonce, None);
    let value = serde_json::to_value(without_nonce).unwrap();
    assert!(value["meta"].get("nonce").is_none());
}

/// Tests the Invalid Session payload and its optional resumable flag.
#[test]
fn invalid_session_round_trips() {
    let resumable: InvalidSession = serde_json::from_str(r#"{"resumable": true}"#).unwrap();

    assert_eq!(resumable.resumable, Some(true));
    assert_eq!(serde_json::to_value(resumable).unwrap(), serde_json::json!({"resumable": true}));

    let not_resumable: InvalidSession = serde_json::from_str(r#"{"resumable": false}"#).unwrap();

    assert_eq!(not_resumable.resumable, Some(false));

    let absent: InvalidSession = serde_json::from_str("{}").unwrap();

    assert_eq!(absent.resumable, None);
    assert_eq!(serde_json::to_value(absent).unwrap(), serde_json::json!({"resumable": null}));
}

/// Tests that the Guild Audit Log Entry Create event flattens an audit log
/// entry and adds the guild ID.
#[test]
fn guild_audit_log_entry_create_flattens_entry() {
    let event: GuildAuditLogEntryCreate = serde_json::from_str(
        r#"{
            "target_id": "41771983423143937",
            "changes": [{
                "key": "name",
                "old_value": "old name",
                "new_value": "new name"
            }],
            "user_id": "423457898095789043",
            "id": "41771983423143938",
            "action_type": 10,
            "options": {"channel_id": "41771983423143939"},
            "reason": "spam",
            "guild_id": "197038439483310086"
        }"#,
    )
    .unwrap();

    assert_eq!(event.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(event.entry.id.get(), 41_771_983_423_143_938);
    assert_eq!(event.entry.target_id.as_deref(), Some("41771983423143937"));
    assert_eq!(event.entry.user_id.unwrap().get(), 423_457_898_095_789_043);
    assert_eq!(event.entry.reason.as_deref(), Some("spam"));
    assert_eq!(event.entry.changes.as_ref().unwrap()[0].key, "name");

    let value = serde_json::to_value(event).unwrap();
    assert_eq!(value["guild_id"], "197038439483310086");
    assert_eq!(value["id"], "41771983423143938");
    assert_eq!(value["action_type"], 10);
    assert_eq!(value["reason"], "spam");
    assert_eq!(value["changes"][0]["new_value"], serde_json::json!("new name"));
}

/// Tests that the Guild Member Add event flattens a guild member.
#[test]
fn guild_member_add_flattens_member() {
    let event: GuildMemberAdd = serde_json::from_str(
        r#"{
            "user": {
                "id": "80351110224678912",
                "username": "Nelly",
                "discriminator": "1333",
                "public_flags": 64
            },
            "nick": "Nelly",
            "roles": ["41771983423143936"],
            "joined_at": "2015-04-26T06:26:56.936000+00:00",
            "premium_since": null,
            "deaf": false,
            "mute": false,
            "guild_id": "197038439483310086"
        }"#,
    )
    .unwrap();

    assert_eq!(event.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(event.member.user.as_ref().unwrap().id.get(), 80_351_110_224_678_912);
    assert_eq!(event.member.nick.as_deref(), Some("Nelly"));
    assert_eq!(event.member.roles[0].get(), 41_771_983_423_143_936);
    assert_eq!(event.member.joined_at, Some(timestamp(1_430_029_616, 936_000_000)));
    assert_eq!(event.member.premium_since, None);

    let value = serde_json::to_value(event).unwrap();
    assert_eq!(value["guild_id"], "197038439483310086");
    assert_eq!(value["user"]["id"], "80351110224678912");
    assert_eq!(value["joined_at"], "2015-04-26T06:26:56.936+00:00");
}

/// Tests the Guild Member Remove and Guild Member Update payloads.
#[test]
fn guild_member_remove_and_update_round_trip() {
    let remove: GuildMemberRemove = serde_json::from_str(
        r#"{
            "guild_id": "197038439483310086",
            "user": {
                "id": "80351110224678912",
                "username": "Nelly",
                "discriminator": "1333"
            }
        }"#,
    )
    .unwrap();

    assert_eq!(remove.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(remove.user.id.get(), 80_351_110_224_678_912);

    let value = serde_json::to_value(remove).unwrap();
    assert_eq!(value["user"]["username"], "Nelly");

    let update: GuildMemberUpdate = serde_json::from_str(
        r#"{
            "guild_id": "197038439483310086",
            "roles": ["41771983423143936"],
            "user": {
                "id": "80351110224678912",
                "username": "Nelly",
                "discriminator": "1333"
            },
            "nick": "Nelly",
            "avatar": null,
            "banner": null,
            "joined_at": "2015-04-26T06:26:56.936000+00:00",
            "premium_since": null,
            "deaf": false,
            "mute": false,
            "pending": false,
            "communication_disabled_until": null,
            "avatar_decoration_data": null,
            "collectibles": null
        }"#,
    )
    .unwrap();

    assert_eq!(update.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(update.roles[0].get(), 41_771_983_423_143_936);
    assert_eq!(update.user.id.get(), 80_351_110_224_678_912);
    assert_eq!(update.nick.as_deref(), Some("Nelly"));
    assert_eq!(update.avatar, None);
    assert_eq!(update.banner, None);
    assert_eq!(update.joined_at, Some(timestamp(1_430_029_616, 936_000_000)));
    assert_eq!(update.premium_since, None);
    assert_eq!(update.deaf, Some(false));
    assert_eq!(update.mute, Some(false));
    assert_eq!(update.pending, Some(false));
    assert_eq!(update.communication_disabled_until, None);
    assert_eq!(update.avatar_decoration_data, None);
    assert_eq!(update.collectibles, None);

    let value = serde_json::to_value(update).unwrap();
    assert_eq!(value["roles"][0], "41771983423143936");
    assert_eq!(value["joined_at"], "2015-04-26T06:26:56.936+00:00");
    assert_eq!(value["nick"], "Nelly");
    assert_eq!(value["avatar"], serde_json::Value::Null);
    assert_eq!(value["banner"], serde_json::Value::Null);
    assert_eq!(value["deaf"], false);
    assert_eq!(value["pending"], false);

    let minimal: GuildMemberUpdate = serde_json::from_str(
        r#"{"guild_id": "1", "roles": [], "user": {"id": "2", "username": "Nelly", "discriminator": "0"}}"#,
    )
    .unwrap();

    assert_eq!(minimal.nick, None);
    assert_eq!(minimal.deaf, None);
    assert_eq!(minimal.joined_at, None);

    let value = serde_json::to_value(minimal).unwrap();
    assert_eq!(value["nick"], serde_json::Value::Null);
    assert_eq!(value["joined_at"], serde_json::Value::Null);
}

/// Tests the Guild Members Chunk payload with and without optional fields.
#[test]
fn guild_members_chunk_round_trips() {
    let chunk: GuildMembersChunk = serde_json::from_str(
        r#"{
            "guild_id": "197038439483310086",
            "members": [{
                "user": {
                    "id": "80351110224678912",
                    "username": "Nelly",
                    "discriminator": "1333"
                },
                "roles": [],
                "joined_at": "2015-04-26T06:26:56.936000+00:00",
                "deaf": false,
                "mute": false
            }],
            "chunk_index": 1,
            "chunk_count": 3,
            "not_found": ["80351110224678913"],
            "presences": [{
                "user": {"id": "80351110224678912", "username": "Nelly"},
                "status": "online",
                "activities": [],
                "client_status": {"desktop": "online"}
            }],
            "nonce": "nonce"
        }"#,
    )
    .unwrap();

    assert_eq!(chunk.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(chunk.members.len(), 1);
    assert_eq!(chunk.members[0].user.as_ref().unwrap().id.get(), 80_351_110_224_678_912);
    assert_eq!(chunk.chunk_index, 1);
    assert_eq!(chunk.chunk_count, 3);
    let not_found = chunk.not_found.as_ref().unwrap();
    assert_eq!(not_found[0].get(), 80_351_110_224_678_913);
    assert_eq!(chunk.presences.as_ref().unwrap()[0].status, UserStatus::ONLINE);
    assert_eq!(chunk.nonce.as_deref(), Some("nonce"));

    let value = serde_json::to_value(chunk).unwrap();
    assert_eq!(value["chunk_index"], 1);
    assert_eq!(value["chunk_count"], 3);
    assert_eq!(value["not_found"][0], "80351110224678913");
    assert_eq!(value["presences"][0]["status"], "online");
    assert_eq!(value["nonce"], "nonce");

    let minimal: GuildMembersChunk = serde_json::from_str(
        r#"{"guild_id": "1", "members": [], "chunk_index": 0, "chunk_count": 1}"#,
    )
    .unwrap();

    assert_eq!(minimal.not_found, None);
    assert_eq!(minimal.presences, None);
    assert_eq!(minimal.nonce, None);

    let value = serde_json::to_value(minimal).unwrap();
    for field in ["not_found", "presences", "nonce"] {
        assert!(value.get(field).is_none(), "{field} should be omitted");
    }
}

/// Tests the Guild Ban Add and Guild Ban Remove payloads.
#[test]
fn guild_ban_events_round_trip() {
    let payload = r#"{
        "guild_id": "197038439483310086",
        "user": {"id": "80351110224678912", "username": "Nelly", "discriminator": "1333"}
    }"#;

    let added: GuildBanAdd = serde_json::from_str(payload).unwrap();

    assert_eq!(added.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(added.user.id.get(), 80_351_110_224_678_912);

    let value = serde_json::to_value(added).unwrap();
    assert_eq!(value["guild_id"], "197038439483310086");
    assert_eq!(value["user"]["id"], "80351110224678912");

    let removed: GuildBanRemove = serde_json::from_str(payload).unwrap();

    assert_eq!(removed.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(removed.user.id.get(), 80_351_110_224_678_912);

    let value = serde_json::to_value(removed).unwrap();
    assert_eq!(value["guild_id"], "197038439483310086");
    assert_eq!(value["user"]["username"], "Nelly");
}

/// Tests the Guild Emojis Update, Guild Stickers Update, and Guild
/// Integrations Update payloads.
#[test]
fn guild_emoji_sticker_and_integration_updates_round_trip() {
    let emojis: GuildEmojisUpdate = serde_json::from_str(
        r#"{
            "guild_id": "197038439483310086",
            "emojis": [{
                "id": "41771983429993937",
                "name": "test",
                "roles": [],
                "require_colons": true,
                "managed": false,
                "animated": false,
                "available": true
            }]
        }"#,
    )
    .unwrap();

    assert_eq!(emojis.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(emojis.emojis.len(), 1);
    assert_eq!(emojis.emojis[0].name.as_deref(), Some("test"));

    let value = serde_json::to_value(emojis).unwrap();
    assert_eq!(value["emojis"][0]["id"], "41771983429993937");

    let stickers: GuildStickersUpdate = serde_json::from_str(
        r#"{
            "guild_id": "197038439483310086",
            "stickers": [{
                "id": "749054660769218631",
                "name": "Wave",
                "description": null,
                "tags": "wumpus",
                "type": 1,
                "format_type": 3,
                "available": true,
                "guild_id": "197038439483310086"
            }]
        }"#,
    )
    .unwrap();

    assert_eq!(stickers.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(stickers.stickers.len(), 1);
    assert_eq!(stickers.stickers[0].kind, StickerType::STANDARD);
    assert_eq!(stickers.stickers[0].description, None);

    let value = serde_json::to_value(stickers).unwrap();
    assert_eq!(value["stickers"][0]["description"], serde_json::Value::Null);

    let integrations: GuildIntegrationsUpdate =
        serde_json::from_str(r#"{"guild_id": "197038439483310086"}"#).unwrap();

    assert_eq!(integrations.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(
        serde_json::to_value(integrations).unwrap(),
        serde_json::json!({"guild_id": "197038439483310086"})
    );
}

/// Tests the Guild Role Create, Update, and Delete payloads.
#[test]
fn guild_role_events_round_trip() {
    let role = r#"{
        "id": "41771983423143936",
        "name": "WE DEM BOYZZ!!!!!!",
        "color": 3447003,
        "colors": {
            "primary_color": 3447003,
            "secondary_color": null,
            "tertiary_color": null
        },
        "hoist": true,
        "position": 1,
        "permissions": "66321471",
        "managed": false,
        "mentionable": false,
        "flags": 0
    }"#;

    let create: GuildRoleCreate =
        serde_json::from_str(&format!(r#"{{"guild_id": "197038439483310086", "role": {role}}}"#))
            .unwrap();

    assert_eq!(create.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(create.role.id.get(), 41_771_983_423_143_936);
    assert_eq!(create.role.name, "WE DEM BOYZZ!!!!!!");

    let value = serde_json::to_value(create).unwrap();
    assert_eq!(value["role"]["id"], "41771983423143936");
    assert_eq!(value["role"]["permissions"], "66321471");

    let update: GuildRoleUpdate =
        serde_json::from_str(&format!(r#"{{"guild_id": "197038439483310086", "role": {role}}}"#))
            .unwrap();

    assert_eq!(update.role.permissions.bits(), 66_321_471);
    assert_eq!(serde_json::to_value(update).unwrap()["guild_id"], "197038439483310086");

    let delete: GuildRoleDelete = serde_json::from_str(
        r#"{"guild_id": "197038439483310086", "role_id": "41771983423143936"}"#,
    )
    .unwrap();

    assert_eq!(delete.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(delete.role_id.get(), 41_771_983_423_143_936);

    let value = serde_json::to_value(delete).unwrap();
    assert_eq!(value["role_id"], "41771983423143936");
}

/// Tests the Guild Scheduled Event User Add and Remove payloads.
#[test]
fn guild_scheduled_event_user_events_round_trip() {
    let payload = r#"{
        "guild_scheduled_event_id": "947656305244532806",
        "user_id": "80351110224678912",
        "guild_id": "197038439483310086"
    }"#;

    let add: GuildScheduledEventUserAdd = serde_json::from_str(payload).unwrap();

    assert_eq!(add.guild_scheduled_event_id.get(), 947_656_305_244_532_806);
    assert_eq!(add.user_id.get(), 80_351_110_224_678_912);
    assert_eq!(add.guild_id.get(), 197_038_439_483_310_086);

    let value = serde_json::to_value(add).unwrap();
    assert_eq!(value["guild_scheduled_event_id"], "947656305244532806");
    assert_eq!(value["guild_id"], "197038439483310086");

    let remove: GuildScheduledEventUserRemove = serde_json::from_str(payload).unwrap();

    assert_eq!(remove.guild_scheduled_event_id.get(), 947_656_305_244_532_806);
    assert_eq!(remove.user_id.get(), 80_351_110_224_678_912);
    assert_eq!(serde_json::to_value(remove).unwrap()["user_id"], "80351110224678912");
}

/// Tests the soundboard events that are not a bare soundboard sound.
#[test]
fn guild_soundboard_events_round_trip() {
    let sound = r#"{
        "name": "Yay",
        "sound_id": "1106714396018884649",
        "volume": 1.0,
        "emoji_id": "989193655938064464",
        "emoji_name": null,
        "guild_id": "613425648685547541",
        "available": true
    }"#;

    let delete: GuildSoundboardSoundDelete = serde_json::from_str(
        r#"{"sound_id": "1106714396018884649", "guild_id": "613425648685547541"}"#,
    )
    .unwrap();

    assert_eq!(delete.sound_id.get(), 1_106_714_396_018_884_649);
    assert_eq!(delete.guild_id.get(), 613_425_648_685_547_541);

    let value = serde_json::to_value(delete).unwrap();
    assert_eq!(value["sound_id"], "1106714396018884649");

    let sounds_update: GuildSoundboardSoundsUpdate = serde_json::from_str(&format!(
        r#"{{"guild_id": "613425648685547541", "soundboard_sounds": [{sound}]}}"#
    ))
    .unwrap();

    assert_eq!(sounds_update.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(sounds_update.soundboard_sounds.len(), 1);
    assert_eq!(sounds_update.soundboard_sounds[0].sound_id.get(), 1_106_714_396_018_884_649);

    let value = serde_json::to_value(sounds_update).unwrap();
    assert_eq!(value["soundboard_sounds"][0]["sound_id"], "1106714396018884649");
    assert_eq!(value["soundboard_sounds"][0]["emoji_name"], serde_json::Value::Null);

    let sounds: SoundboardSounds = serde_json::from_str(&format!(
        r#"{{"guild_id": "613425648685547541", "soundboard_sounds": [{sound}]}}"#
    ))
    .unwrap();

    assert_eq!(sounds.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(sounds.soundboard_sounds[0].volume, 1.0);
    assert_eq!(serde_json::to_value(sounds).unwrap()["guild_id"], "613425648685547541");
}

/// Tests the Integration Delete payload and its optional application.
#[test]
fn integration_delete_round_trips() {
    let delete: IntegrationDelete = serde_json::from_str(
        r#"{
            "id": "41771983423143937",
            "guild_id": "613425648685547541",
            "application_id": "788708323867885999"
        }"#,
    )
    .unwrap();

    assert_eq!(delete.id.get(), 41_771_983_423_143_937);
    assert_eq!(delete.guild_id.get(), 613_425_648_685_547_541);
    assert_eq!(delete.application_id.unwrap().get(), 788_708_323_867_885_999);

    let value = serde_json::to_value(delete).unwrap();
    assert_eq!(value["id"], "41771983423143937");

    let without_application: IntegrationDelete =
        serde_json::from_str(r#"{"id": "1", "guild_id": "2"}"#).unwrap();

    assert_eq!(without_application.application_id, None);
    let value = serde_json::to_value(without_application).unwrap();
    assert!(value.get("application_id").is_none());
}

/// Tests that the channel events documented as a plain channel object decode
/// into [`Channel`], including the subset sent by `THREAD_DELETE`.
#[test]
fn channel_events_decode_into_the_channel_model() {
    let created: Channel = serde_json::from_str(
        r#"{
            "id": "1099032703076216842",
            "type": 0,
            "guild_id": "197038439483310086",
            "name": "general",
            "position": 0,
            "topic": null
        }"#,
    )
    .unwrap();

    assert_eq!(created.kind, ChannelType::GUILD_TEXT);
    assert_eq!(created.guild_id.unwrap().get(), 197_038_439_483_310_086);
    assert_eq!(created.name.as_deref(), Some("general"));

    let envelope: GatewayPayload<Channel> = serde_json::from_str(
        r#"{
            "op": 0,
            "s": 3,
            "t": "THREAD_DELETE",
            "d": {
                "id": "1099032703076216842",
                "guild_id": "197038439483310086",
                "parent_id": "1099032703076216840",
                "type": 15
            }
        }"#,
    )
    .unwrap();

    assert_eq!(envelope.t, Some(GatewayEventName::THREAD_DELETE));

    let deleted = envelope.d.unwrap();
    assert_eq!(deleted.kind, ChannelType::GUILD_FORUM);
    assert_eq!(deleted.parent_id.unwrap().get(), 1_099_032_703_076_216_840);
    assert_eq!(deleted.name, None);

    let value = serde_json::to_value(deleted).unwrap();
    assert_eq!(value["type"], 15);
    assert_eq!(value["guild_id"], "197038439483310086");
    assert!(value.get("name").is_none());
}
#[test]
fn lobby_matches_discord_example() {
    let lobby: Lobby = serde_json::from_str(
        r#"{
            "id": "96008815106887111",
            "application_id": "41771983429993937",
            "metadata": {"topic": "we need more redstone"},
            "members": [
                {
                    "id": "41771983429993000",
                    "metadata": null,
                    "flags": 1,
                    "additional_name": "Twindly Shimmerleaf"
                }
            ]
        }"#,
    )
    .unwrap();

    assert_eq!(lobby.id.get(), 96_008_815_106_887_111);
    assert_eq!(lobby.application_id.get(), 41_771_983_429_993_937);
    assert_eq!(lobby.metadata.as_ref().unwrap()["topic"], "we need more redstone");
    assert!(lobby.linked_channel.is_none());

    let member = &lobby.members[0];
    assert_eq!(member.id.get(), 41_771_983_429_993_000);
    assert!(member.metadata.is_none());
    assert_eq!(member.flags, Some(LobbyMemberFlags::CAN_LINK_LOBBY));
    assert_eq!(member.additional_name.as_deref(), Some("Twindly Shimmerleaf"));

    let value = serde_json::to_value(&lobby).unwrap();
    assert_eq!(value["id"], "96008815106887111");
    assert!(value["members"][0]["metadata"].is_null());
    assert_eq!(value["members"][0]["flags"], 1);
    assert!(value.get("linked_channel").is_none());
}

/// Tests that lobby metadata and flags omit or emit `null` as documented.
#[test]
fn lobby_nullable_and_optional_fields() {
    let lobby: Lobby =
        serde_json::from_str(r#"{"id": "1", "application_id": "2", "members": []}"#).unwrap();

    assert!(lobby.metadata.is_none());
    assert_eq!(serde_json::to_value(&lobby).unwrap()["metadata"], serde_json::Value::Null);

    let flags: LobbyMemberFlags = serde_json::from_str("9223372036854775809").unwrap();
    assert_eq!(flags.raw(), 9_223_372_036_854_775_809);
    assert_eq!(LobbyMemberFlags::from_raw(1), LobbyMemberFlags::CAN_LINK_LOBBY);
}

/// Tests the lobby message object from Discord's lobby resource.
#[test]
fn lobby_message_matches_documented_fields() {
    let message: LobbyMessage = serde_json::from_str(
        r#"{
            "id": "98139515623658496",
            "type": 0,
            "content": "hello",
            "lobby_id": "96008815106887111",
            "channel_id": "96008815106887111",
            "author": {
                "id": "41771983429993000",
                "username": "Nelly",
                "discriminator": "0001"
            },
            "lobby_member": {"additional_name": "Twindly Shimmerleaf"},
            "metadata": null,
            "moderation_metadata": null,
            "flags": 0,
            "application_id": "41771983429993937"
        }"#,
    )
    .unwrap();

    assert_eq!(message.id.get(), 98_139_515_623_658_496);
    assert_eq!(message.content, "hello");
    assert_eq!(message.lobby_id.get(), 96_008_815_106_887_111);
    assert_eq!(message.channel_id.get(), 96_008_815_106_887_111);
    assert_eq!(message.author.username, "Nelly");
    assert_eq!(message.lobby_member.as_ref().unwrap().additional_name, "Twindly Shimmerleaf");
    assert!(message.metadata.is_none());
    assert!(message.moderation_metadata.is_none());
    assert_eq!(message.flags.bits(), 0);

    let value = serde_json::to_value(&message).unwrap();
    assert_eq!(value["type"], 0);
    assert_eq!(value["channel_id"], "96008815106887111");
    assert!(value["metadata"].is_null());
}

/// Tests the lobby request payloads used by the lobby endpoints.
#[test]
fn lobby_requests_omit_absent_fields() {
    let create: CreateLobbyRequest = serde_json::from_str(
        r#"{
            "metadata": {"topic": "redstone"},
            "members": [
                {"id": "41771983429993000", "flags": 1, "additional_name": "Twindly Shimmerleaf"}
            ],
            "idle_timeout_seconds": 3600
        }"#,
    )
    .unwrap();

    assert_eq!(create.metadata.as_ref().unwrap()["topic"], "redstone");
    assert_eq!(create.members.as_ref().unwrap().len(), 1);
    assert_eq!(create.idle_timeout_seconds, Some(3600));

    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["metadata"]["topic"], "redstone");
    assert_eq!(value["members"][0]["id"], "41771983429993000");
    assert_eq!(value["idle_timeout_seconds"], 3600);

    let join: CreateOrJoinLobbyRequest =
        serde_json::from_str(r#"{"secret": "matchmaking-secret"}"#).unwrap();
    assert_eq!(join.secret, "matchmaking-secret");
    assert!(join.lobby_metadata.is_none());
    let value = serde_json::to_value(&join).unwrap();
    assert_eq!(value["secret"], "matchmaking-secret");
    assert!(value.get("lobby_metadata").is_none());
    assert!(value.get("member_metadata").is_none());

    let modify: ModifyLobbyRequest =
        serde_json::from_str(r#"{"metadata": {"topic": "more redstone"}}"#).unwrap();
    assert_eq!(modify.metadata.as_ref().unwrap()["topic"], "more redstone");

    let upsert: UpsertLobbyMemberRequest =
        serde_json::from_str(r#"{"metadata": null, "flags": 1, "additional_name": null}"#).unwrap();
    assert!(upsert.metadata.is_none());
    assert_eq!(upsert.flags, Some(LobbyMemberFlags::CAN_LINK_LOBBY));
    assert!(upsert.additional_name.is_none());

    let bulk: BulkLobbyMemberRequest =
        serde_json::from_str(r#"{"id": "41771983429993000", "remove_member": true}"#).unwrap();
    assert_eq!(bulk.remove_member, Some(true));
    assert!(bulk.metadata.is_none());

    let link: LinkLobbyChannelRequest =
        serde_json::from_str(r#"{"channel_id": "96008815106887111"}"#).unwrap();
    assert_eq!(link.channel_id.unwrap().get(), 96_008_815_106_887_111);
    assert_eq!(
        serde_json::to_value(LinkLobbyChannelRequest { channel_id: None }).unwrap(),
        serde_json::json!({})
    );

    let send: SendLobbyMessageRequest = serde_json::from_str(r#"{"content": "hello"}"#).unwrap();
    assert_eq!(send.content, "hello");
    assert!(send.metadata.is_none());
    assert!(send.flags.is_none());
}

/// Tests the lobby invite object from Discord's lobby resource.
#[test]
fn lobby_invite_matches_documented_fields() {
    let invite: LobbyInvite = serde_json::from_str(r#"{"code": "abcdefg"}"#).unwrap();

    assert_eq!(invite.code, "abcdefg");
    assert_eq!(serde_json::to_value(&invite).unwrap(), serde_json::json!({"code": "abcdefg"}));
}

/// Tests the application identity profile example from Discord's documentation.
#[test]
fn identity_profile_matches_discord_example() {
    let profile: ApplicationIdentityProfile = serde_json::from_str(
        r#"{
            "username": "johndoe123",
            "metadata": null,
            "data": {
                "primary": {
                    "season": "Season 3",
                    "rank_name": "Silver",
                    "rank_image": {"url": "https://example.com/assets/rank-images/silver.png"},
                    "highest_rank": "Platinum",
                    "highest_rank_image": {"url": "https://example.com/assets/rank-images/platinum.png"},
                    "featured_played_character": "John Doe",
                    "featured_played_character_image": {"url": "https://example.com/assets/character-images/john-doe.png"},
                    "playtime_hours": 69.41,
                    "total_wins": 57,
                    "current_period_wins": 8,
                    "total_games": 100,
                    "current_period_games": 10,
                    "total_kills": 253,
                    "current_period_kills": 35,
                    "total_assists": 478,
                    "current_period_assists": 68,
                    "total_deaths": 561,
                    "current_period_deaths": 21
                },
                "dynamic": [
                    {"type": 1, "name": "my_string", "value": "hello"},
                    {"type": 2, "name": "my_number", "value": 123.45},
                    {"type": 3, "name": "my_media", "value": {"url": "https://example.com/some-media.png"}}
                ]
            }
        }"#,
    )
    .unwrap();

    assert_eq!(profile.username.as_deref(), Some("johndoe123"));
    assert!(profile.metadata.is_none());

    let data = profile.data.as_ref().unwrap();
    let primary = data.primary.as_ref().unwrap();
    assert_eq!(primary.season.as_deref(), Some("Season 3"));
    assert_eq!(primary.rank_name.as_deref(), Some("Silver"));
    assert_eq!(
        primary.rank_image.as_ref().unwrap().url,
        "https://example.com/assets/rank-images/silver.png"
    );
    assert_eq!(primary.highest_rank.as_deref(), Some("Platinum"));
    assert_eq!(primary.playtime_hours, Some(69.41));
    assert_eq!(primary.total_wins, Some(57));
    assert_eq!(primary.current_period_wins, Some(8));
    assert_eq!(primary.total_games, Some(100));
    assert_eq!(primary.current_period_games, Some(10));
    assert_eq!(primary.total_kills, Some(253));
    assert_eq!(primary.current_period_kills, Some(35));
    assert_eq!(primary.total_assists, Some(478));
    assert_eq!(primary.current_period_assists, Some(68));
    assert_eq!(primary.total_deaths, Some(561));
    assert_eq!(primary.current_period_deaths, Some(21));
    assert!(primary.featured_played_character_image.is_some());

    let dynamic = data.dynamic.as_ref().unwrap();
    assert_eq!(dynamic[0].kind, DynamicFieldType::STRING);
    assert_eq!(dynamic[0].name, "my_string");
    assert_eq!(dynamic[0].value, DynamicFieldValue::String("hello".to_string()));
    assert_eq!(dynamic[1].kind, DynamicFieldType::NUMBER);
    assert_eq!(dynamic[1].value, DynamicFieldValue::Number(123.45));
    assert_eq!(dynamic[2].kind, DynamicFieldType::MEDIA);
    assert_eq!(
        dynamic[2].value,
        DynamicFieldValue::Media(Media {
            url: "https://example.com/some-media.png".to_string(),
        })
    );

    let request = UpdateApplicationIdentityProfileRequest {
        username: None,
        data: Some(ProfileData {
            primary: None,
            dynamic: Some(vec![DynamicField {
                kind: DynamicFieldType::NUMBER,
                name: "wins".to_string(),
                value: DynamicFieldValue::Number(3.0),
            }]),
        }),
    };
    let value = serde_json::to_value(&request).unwrap();
    assert!(value.get("username").is_none());
    assert_eq!(value["data"]["dynamic"][0]["type"], 2);
    assert_eq!(value["data"]["dynamic"][0]["name"], "wins");
    assert_eq!(value["data"]["dynamic"][0]["value"], 3.0);
}

/// Tests that unknown provider types and dynamic field types are preserved.
#[test]
fn identity_profile_preserves_unknown_values() {
    let provider: ProviderType = serde_json::from_str("\"unknown_provider\"").unwrap();
    assert_eq!(provider.as_str(), "unknown_provider");

    let field: DynamicField =
        serde_json::from_str(r#"{"type": 9, "name": "mystery", "value": "text"}"#).unwrap();
    assert_eq!(field.kind.raw(), 9);
    assert_eq!(field.value, DynamicFieldValue::String("text".to_string()));
}

/// Tests the application identity list response.
#[test]
fn application_identity_list_matches_documented_fields() {
    let list: ApplicationIdentityList = serde_json::from_str(
        r#"{
            "identities": [
                {
                    "user_id": "41771983429993000",
                    "provider_type": "NONE",
                    "provider_issued_user_id": "external-user-1"
                },
                {
                    "user_id": "41771983429993001",
                    "provider_type": "STEAM",
                    "provider_id": "provider-id",
                    "provider_issued_user_id": "external-user-2"
                }
            ]
        }"#,
    )
    .unwrap();

    assert_eq!(list.identities.len(), 2);
    let identity = &list.identities[0];
    assert_eq!(identity.user_id.unwrap().get(), 41_771_983_429_993_000);
    assert_eq!(identity.provider_type, ProviderType::NONE);
    assert_eq!(identity.provider_issued_user_id, "external-user-1");
    assert!(identity.provider_id.is_none());
    assert_eq!(list.identities[1].provider_id.as_deref(), Some("provider-id"));

    let value = serde_json::to_value(&list).unwrap();
    assert_eq!(value["identities"][0]["provider_type"], "NONE");
    assert!(value["identities"][0].get("provider_id").is_none());

    let update = UpdateApplicationIdentityProfileRequest {
        username: Some("new-name".to_string()),
        data: None,
    };
    let value = serde_json::to_value(&update).unwrap();
    assert_eq!(value["username"], "new-name");
    assert!(value.get("data").is_none());
}

/// Tests the direct message and group DM request payloads.
#[test]
fn channel_dm_requests_match_documented_fields() {
    let create_dm = CreateDmRequest {
        recipient_id: Id::<User>::new(1_088_605_110_638_227_537),
    };
    assert_eq!(
        serde_json::to_value(create_dm).unwrap(),
        serde_json::json!({"recipient_id": "1088605110638227537"})
    );

    let create_group: CreateGroupDmRequest = serde_json::from_str(
        r#"{
            "access_tokens": ["token-a", "token-b"],
            "nicks": {"41771983429993000": "Nelly"}
        }"#,
    )
    .unwrap();
    assert_eq!(create_group.access_tokens.len(), 2);
    assert_eq!(create_group.nicks.as_ref().unwrap().len(), 1);

    let value = serde_json::to_value(&create_group).unwrap();
    assert_eq!(value["access_tokens"][1], "token-b");
    assert_eq!(value["nicks"]["41771983429993000"], "Nelly");
}

/// Tests guild ban, prune, and member request payloads.
#[test]
fn guild_requests_match_documented_fields() {
    let ban = CreateGuildBanRequest { delete_message_seconds: Some(604_800) };
    assert_eq!(
        serde_json::to_value(ban).unwrap(),
        serde_json::json!({"delete_message_seconds": 604800})
    );
    assert_eq!(
        serde_json::to_value(CreateGuildBanRequest { delete_message_seconds: None }).unwrap(),
        serde_json::json!({})
    );

    let bulk_ban: BulkGuildBanRequest = serde_json::from_str(
        r#"{"user_ids": ["41771983429993000"], "delete_message_seconds": 3600}"#,
    )
    .unwrap();
    assert_eq!(bulk_ban.user_ids.len(), 1);
    assert_eq!(bulk_ban.delete_message_seconds, Some(3600));

    let bulk_response: BulkGuildBanResponse = serde_json::from_str(
        r#"{
            "banned_users": ["41771983429993000"],
            "failed_users": ["41771983429993001"]
        }"#,
    )
    .unwrap();
    assert_eq!(bulk_response.banned_users.len(), 1);
    assert_eq!(bulk_response.failed_users.len(), 1);

    let counted: GuildPruneCount = serde_json::from_str(r#"{"pruned": 42}"#).unwrap();
    assert_eq!(counted.pruned, Some(42));
    let uncounted: GuildPruneCount = serde_json::from_str(r#"{"pruned": null}"#).unwrap();
    assert!(uncounted.pruned.is_none());
    assert_eq!(serde_json::to_value(uncounted).unwrap(), serde_json::json!({"pruned": null}));

    let prune: BeginGuildPruneRequest = serde_json::from_str(
        r#"{"days": 7, "compute_prune_count": true, "include_roles": ["41771983429993000"]}"#,
    )
    .unwrap();
    assert_eq!(prune.days, Some(7));
    assert_eq!(prune.include_roles.as_ref().unwrap().len(), 1);
    assert!(prune.reason.is_none());
    assert_eq!(serde_json::to_value(&prune).unwrap()["include_roles"][0], "41771983429993000");

    let add_member: AddGuildMemberRequest = serde_json::from_str(
        r#"{"access_token": "token", "nick": "Nelly", "roles": [], "mute": false, "deaf": false}"#,
    )
    .unwrap();
    assert_eq!(add_member.access_token, "token");
    assert_eq!(add_member.nick.as_deref(), Some("Nelly"));
    assert_eq!(add_member.mute, Some(false));

    let modify_member = ModifyGuildMemberRequest {
        nick: None,
        roles: None,
        mute: None,
        deaf: None,
        channel_id: Some(Id::<Channel>::new(96_008_815_106_887_111)),
        communication_disabled_until: Some(UNIX_EPOCH + Duration::from_secs(1_693_579_682)),
        flags: None,
    };
    let value = serde_json::to_value(&modify_member).unwrap();
    assert_eq!(value["channel_id"], "96008815106887111");
    assert_eq!(value["communication_disabled_until"], "2023-09-01T14:48:02+00:00");

    let modify_current = ModifyCurrentMemberRequest {
        nick: Some("Nelly".to_string()),
        banner: None,
        avatar: None,
        bio: Some("Bot developer".to_string()),
    };
    let value = serde_json::to_value(&modify_current).unwrap();
    assert_eq!(value["nick"], "Nelly");
    assert_eq!(value["bio"], "Bot developer");
    assert!(value.get("avatar").is_none());

    let position: ModifyGuildChannelPosition = serde_json::from_str(
        r#"{"id": "96008815106887111", "position": 3, "lock_permissions": true}"#,
    )
    .unwrap();
    assert_eq!(position.position, Some(3));
    assert!(position.lock_permissions.unwrap());
    assert!(position.parent_id.is_none());

    let threads: ListGuildThreadsResponse = serde_json::from_str(
        r#"{
            "threads": [
                {"id": "96008815106887112", "type": 11}
            ],
            "members": [
                {
                    "join_timestamp": "2023-09-01T14:48:02.222000+00:00",
                    "flags": 0
                }
            ]
        }"#,
    )
    .unwrap();
    assert_eq!(threads.threads.len(), 1);
    assert_eq!(threads.members.len(), 1);
}

/// Tests the voice state modification request payloads.
#[test]
fn voice_state_requests_match_documented_fields() {
    let current: ModifyCurrentUserVoiceStateRequest =
        serde_json::from_str(r#"{"channel_id": "96008815106887111", "suppress": false}"#).unwrap();
    assert_eq!(current.channel_id.unwrap().get(), 96_008_815_106_887_111);
    assert_eq!(current.suppress, Some(false));
    assert!(current.request_to_speak_timestamp.is_none());
    assert_eq!(
        serde_json::to_value(current).unwrap(),
        serde_json::json!({
            "channel_id": "96008815106887111",
            "suppress": false
        })
    );

    let user = ModifyUserVoiceStateRequest { channel_id: None, suppress: Some(true) };
    assert_eq!(serde_json::to_value(user).unwrap(), serde_json::json!({"suppress": true}));

    let speaking = ModifyCurrentUserVoiceStateRequest {
        channel_id: Some(Id::<Channel>::new(96_008_815_106_887_111)),
        suppress: None,
        request_to_speak_timestamp: Some(UNIX_EPOCH + Duration::from_secs(1_693_579_682)),
    };
    assert_eq!(
        serde_json::to_value(speaking).unwrap()["request_to_speak_timestamp"],
        "2023-09-01T14:48:02+00:00"
    );
}

/// Tests the sticker pack list response and guild sticker creation request.
#[test]
fn sticker_pack_list_and_create_request_match_documented_fields() {
    let list: StickerPackList = serde_json::from_str(
        r#"{
            "sticker_packs": [
                {
                    "id": "847199849233514549",
                    "stickers": [],
                    "name": "Wumpus Beyond",
                    "sku_id": "847199849233514547",
                    "cover_sticker_id": "749053689419006003",
                    "description": "Say hello to Wumpus!",
                    "banner_asset_id": "761773777976819732"
                }
            ]
        }"#,
    )
    .unwrap();

    assert_eq!(list.sticker_packs.len(), 1);
    assert_eq!(list.sticker_packs[0].name, "Wumpus Beyond");
    assert_eq!(list.sticker_packs[0].stickers.len(), 0);
    let value = serde_json::to_value(&list).unwrap();
    assert_eq!(value["sticker_packs"][0]["id"], "847199849233514549");

    let create: CreateGuildStickerRequest = serde_json::from_str(
        r#"{"name": "Wave", "description": "Wumpus waves hello", "tags": "wumpus, wave"}"#,
    )
    .unwrap();
    assert_eq!(create.name, "Wave");
    assert_eq!(create.tags, "wumpus, wave");
    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["description"], "Wumpus waves hello");
}

/// Tests the get gateway endpoint response.
#[test]
fn get_gateway_response_matches_discord_example() {
    let response: GetGatewayResponse =
        serde_json::from_str(r#"{"url": "wss://gateway.discord.gg/"}"#).unwrap();

    assert_eq!(response.url, "wss://gateway.discord.gg/");
    assert_eq!(
        serde_json::to_value(&response).unwrap(),
        serde_json::json!({"url": "wss://gateway.discord.gg/"})
    );
}

/// Tests the get gateway bot example from Discord's Gateway documentation.
#[test]
fn get_gateway_bot_matches_discord_example() {
    let response: GetGatewayBotResponse = serde_json::from_str(
        r#"{
            "url": "wss://gateway.discord.gg/",
            "shards": 9,
            "session_start_limit": {
                "total": 1000,
                "remaining": 999,
                "reset_after": 14400000,
                "max_concurrency": 1
            }
        }"#,
    )
    .unwrap();

    assert_eq!(response.url, "wss://gateway.discord.gg/");
    assert_eq!(response.shards, 9);

    let limit = response.session_start_limit;
    assert_eq!(limit.total, 1000);
    assert_eq!(limit.remaining, 999);
    assert_eq!(limit.reset_after, Duration::from_secs(14_400));
    assert_eq!(limit.max_concurrency, 1);

    let value = serde_json::to_value(&response).unwrap();
    assert_eq!(value["url"], "wss://gateway.discord.gg/");
    assert_eq!(value["shards"], 9);
    assert_eq!(value["session_start_limit"]["total"], 1000);
    assert_eq!(value["session_start_limit"]["remaining"], 999);
    assert_eq!(value["session_start_limit"]["reset_after"], 14_400_000);
    assert_eq!(value["session_start_limit"]["max_concurrency"], 1);
}

/// Tests that sub-millisecond precision in `reset_after` is truncated when
/// serialized back to the wire format.
#[test]
fn session_start_limit_serializes_milliseconds() {
    let limit = SessionStartLimit {
        total: 1000,
        remaining: 1000,
        reset_after: Duration::from_millis(1500) + Duration::from_nanos(250),
        max_concurrency: 8,
    };

    let value = serde_json::to_value(limit).unwrap();
    assert_eq!(value["reset_after"], 1500);
    assert_eq!(
        serde_json::from_str::<SessionStartLimit>(&serde_json::to_string(&limit).unwrap())
            .unwrap()
            .reset_after,
        Duration::from_millis(1500)
    );
}

/// Tests the example ban object from Discord's guild documentation.
#[test]
fn ban_matches_discord_example() {
    let ban: Ban = serde_json::from_str(
        r#"{
            "reason": "mentioning b1nzy",
            "user": {
                "username": "Mason",
                "discriminator": "9999",
                "id": "53908099506183680",
                "avatar": "a_bab14f271d565501444b2ca3be944b25",
                "public_flags": 131141
            }
        }"#,
    )
    .unwrap();

    assert_eq!(ban.reason.as_deref(), Some("mentioning b1nzy"));
    assert_eq!(ban.user.id.get(), 53_908_099_506_183_680);
    assert_eq!(ban.user.username, "Mason");
    assert_eq!(ban.user.public_flags.unwrap().bits(), 131_141);

    let value = serde_json::to_value(&ban).unwrap();
    assert_eq!(value["reason"], "mentioning b1nzy");
    assert_eq!(value["user"]["id"], "53908099506183680");
}

/// Tests that a ban without a recorded reason serializes `reason` as `null`.
#[test]
fn ban_reason_is_nullable() {
    let ban: Ban = serde_json::from_str(
        r#"{
            "reason": null,
            "user": {"id": "1", "username": "Mason", "discriminator": "0"}
        }"#,
    )
    .unwrap();

    assert_eq!(ban.reason, None);

    let value = serde_json::to_value(&ban).unwrap();
    assert!(value["reason"].is_null());

    let list: Vec<Ban> = serde_json::from_str(
        r#"[{"reason": null, "user": {"id": "2", "username": "Nelly", "discriminator": "0"}}]"#,
    )
    .unwrap();

    assert_eq!(list.len(), 1);
    assert_eq!(list[0].user.username, "Nelly");
}

/// Tests the message pin object and the Get Channel Pins response.
#[test]
fn message_pins_response_round_trips() {
    let pins: GetChannelPinsResponse = serde_json::from_str(
        r#"{
            "items": [
                {
                    "pinned_at": "2026-01-02T03:04:05.006000+00:00",
                    "message": {
                        "id": "1081000000000000001",
                        "channel_id": "1081000000000000002",
                        "author": {
                            "id": "1081000000000000003",
                            "username": "Cordion",
                            "discriminator": "0",
                            "bot": true
                        },
                        "content": "Pinned",
                        "timestamp": "2026-01-01T00:00:00+00:00",
                        "edited_timestamp": null,
                        "tts": false,
                        "mention_everyone": false,
                        "mentions": [],
                        "mention_roles": [],
                        "attachments": [],
                        "embeds": [],
                        "pinned": true,
                        "type": 0
                    }
                }
            ],
            "has_more": true
        }"#,
    )
    .unwrap();

    assert_eq!(pins.items.len(), 1);
    assert_eq!(pins.items[0].pinned_at, timestamp(1_767_323_045, 6_000_000));
    assert_eq!(pins.items[0].message.id.get(), 1_081_000_000_000_000_001);
    assert!(pins.items[0].message.pinned);
    assert_eq!(pins.items[0].message.guild_id, None);
    assert!(pins.has_more);

    let value = serde_json::to_value(&pins).unwrap();
    assert_eq!(value["items"][0]["pinned_at"], "2026-01-02T03:04:05.006+00:00");
    assert_eq!(value["items"][0]["message"]["id"], "1081000000000000001");
    assert_eq!(value["has_more"], true);

    let single: MessagePin = serde_json::from_str(
        r#"{
            "pinned_at": "2023-09-01T14:48:02.222000+00:00",
            "message": {
                "id": "1",
                "channel_id": "2",
                "author": {"id": "3", "username": "Mason", "discriminator": "0"},
                "content": "",
                "timestamp": "2023-09-01T14:48:02.222000+00:00",
                "edited_timestamp": null,
                "tts": false,
                "mention_everyone": false,
                "mentions": [],
                "mention_roles": [],
                "attachments": [],
                "embeds": [],
                "pinned": true,
                "type": 0
            }
        }"#,
    )
    .unwrap();

    assert_eq!(single.pinned_at, timestamp(1_693_579_682, 222_000_000));
    assert_eq!(single.message.id.get(), 1);
}

/// Tests the interaction callback response Discord returns for a callback
/// requested with `with_response=true`.
#[test]
fn interaction_callback_response_matches_documented_fields() {
    let response: InteractionCallbackResponse = serde_json::from_str(
        r#"{
            "interaction": {
                "id": "1081000000000000008",
                "type": 2,
                "activity_instance_id": "i-1276580072400224306-gc-912952092627435520-912954213460484116",
                "response_message_id": "1081000000000000009",
                "response_message_loading": false,
                "response_message_ephemeral": true
            },
            "resource": {
                "type": 4,
                "activity_instance": {
                    "id": "i-1276580072400224306-gc-912952092627435520-912954213460484116"
                },
                "message": {
                    "id": "1081000000000000009",
                    "channel_id": "1081000000000000002",
                    "author": {
                        "id": "1081000000000000003",
                        "username": "Cordion",
                        "discriminator": "0",
                        "bot": true
                    },
                    "content": "Congrats on sending your command!",
                    "timestamp": "2026-01-02T03:04:05.006000+00:00",
                    "edited_timestamp": null,
                    "tts": false,
                    "mention_everyone": false,
                    "mentions": [],
                    "mention_roles": [],
                    "attachments": [],
                    "embeds": [],
                    "pinned": false,
                    "type": 20,
                    "flags": 64
                }
            }
        }"#,
    )
    .unwrap();

    assert_eq!(response.interaction.id.get(), 1_081_000_000_000_000_008);
    assert_eq!(response.interaction.kind, InteractionType::APPLICATION_COMMAND);
    assert_eq!(
        response.interaction.activity_instance_id.as_deref(),
        Some("i-1276580072400224306-gc-912952092627435520-912954213460484116")
    );
    assert_eq!(
        response.interaction.response_message_id.unwrap().get(),
        1_081_000_000_000_000_009
    );
    assert_eq!(response.interaction.response_message_loading, Some(false));
    assert_eq!(response.interaction.response_message_ephemeral, Some(true));

    let resource = response.resource.as_ref().unwrap();
    assert_eq!(resource.kind, InteractionCallbackType::CHANNEL_MESSAGE_WITH_SOURCE);
    assert_eq!(
        resource.activity_instance.as_ref().unwrap().id,
        "i-1276580072400224306-gc-912952092627435520-912954213460484116"
    );
    assert_eq!(resource.message.as_ref().unwrap().id.get(), 1_081_000_000_000_000_009);

    let value = serde_json::to_value(&response).unwrap();
    assert_eq!(value["interaction"]["type"], 2);
    assert_eq!(value["interaction"]["response_message_id"], "1081000000000000009");
    assert_eq!(value["resource"]["type"], 4);
    assert_eq!(value["resource"]["message"]["flags"], 64);

    let round_tripped: InteractionCallbackResponse =
        serde_json::from_str(&serde_json::to_string(&response).unwrap()).unwrap();

    assert_eq!(round_tripped, response);
}

/// Tests that the interaction callback response omits its optional fields.
#[test]
fn interaction_callback_response_omits_absent_fields() {
    let response: InteractionCallbackResponse =
        serde_json::from_str(r#"{"interaction": {"id": "1", "type": 1}}"#).unwrap();

    assert_eq!(response.interaction.id.get(), 1);
    assert_eq!(response.interaction.kind, InteractionType::PING);
    assert_eq!(response.interaction.activity_instance_id, None);
    assert_eq!(response.interaction.response_message_id, None);
    assert_eq!(response.interaction.response_message_loading, None);
    assert_eq!(response.interaction.response_message_ephemeral, None);
    assert_eq!(response.resource, None);

    let value = serde_json::to_value(&response).unwrap();
    assert!(value.get("resource").is_none());
    assert!(value["interaction"].get("activity_instance_id").is_none());

    let activity_only: InteractionCallbackResponse = serde_json::from_str(
        r#"{
            "interaction": {"id": "2", "type": 2},
            "resource": {
                "type": 12,
                "activity_instance": {"id": "i-abc"}
            }
        }"#,
    )
    .unwrap();

    let resource = activity_only.resource.as_ref().unwrap();
    assert_eq!(resource.kind, InteractionCallbackType::LAUNCH_ACTIVITY);
    assert_eq!(resource.activity_instance.as_ref().unwrap().id, "i-abc");
    assert_eq!(resource.message, None);
}

/// Tests the guild scheduled event user object.
#[test]
fn guild_scheduled_event_user_matches_documented_fields() {
    let subscription: GuildScheduledEventUser = serde_json::from_str(
        r#"{
            "guild_scheduled_event_id": "1081000000000000011",
            "user": {
                "id": "1081000000000000012",
                "username": "Mason",
                "discriminator": "1337"
            },
            "member": {
                "nick": "NOT API SUPPORT",
                "roles": [],
                "joined_at": "2015-04-26T06:26:56.936000+00:00",
                "deaf": false,
                "mute": false
            }
        }"#,
    )
    .unwrap();

    assert_eq!(subscription.guild_scheduled_event_id.get(), 1_081_000_000_000_000_011);
    assert_eq!(subscription.user.id.get(), 1_081_000_000_000_000_012);
    let member = subscription.member.as_ref().unwrap();
    assert_eq!(member.nick.as_deref(), Some("NOT API SUPPORT"));
    assert_eq!(member.joined_at, Some(timestamp(1_430_029_616, 936_000_000)));

    let value = serde_json::to_value(&subscription).unwrap();
    assert_eq!(value["guild_scheduled_event_id"], "1081000000000000011");
    assert_eq!(value["user"]["id"], "1081000000000000012");
    assert_eq!(value["member"]["joined_at"], "2015-04-26T06:26:56.936+00:00");

    let without_member: GuildScheduledEventUser = serde_json::from_str(
        r#"{
            "guild_scheduled_event_id": "1",
            "user": {"id": "2", "username": "Mason", "discriminator": "0"}
        }"#,
    )
    .unwrap();

    assert_eq!(without_member.member, None);

    let value = serde_json::to_value(without_member).unwrap();
    assert!(value.get("member").is_none());
}

/// Tests the example activity instance from Discord's application documentation.
#[test]
fn activity_instance_matches_discord_example() {
    let instance: ActivityInstance = serde_json::from_str(
        r#"{
            "application_id": "1215413995645968394",
            "instance_id": "i-1276580072400224306-gc-912952092627435520-912954213460484116",
            "launch_id": "1276580072400224306",
            "location": {
                "id": "gc-912952092627435520-912954213460484116",
                "kind": "gc",
                "channel_id": "912954213460484116",
                "guild_id": "912952092627435520"
            },
            "users": ["205519959982473217"]
        }"#,
    )
    .unwrap();

    assert_eq!(instance.application_id.get(), 1_215_413_995_645_968_394);
    assert_eq!(
        instance.instance_id,
        "i-1276580072400224306-gc-912952092627435520-912954213460484116"
    );
    assert_eq!(instance.launch_id, 1_276_580_072_400_224_306);
    assert_eq!(instance.location.kind, ActivityLocationKind::GUILD_CHANNEL);
    assert_eq!(instance.location.id, "gc-912952092627435520-912954213460484116");
    assert_eq!(instance.location.channel_id.get(), 912_954_213_460_484_116);
    assert_eq!(instance.location.guild_id.unwrap().get(), 912_952_092_627_435_520);
    assert_eq!(instance.users.len(), 1);
    assert_eq!(instance.users[0].get(), 205_519_959_982_473_217);

    let value = serde_json::to_value(&instance).unwrap();
    assert_eq!(value["launch_id"], "1276580072400224306");
    assert_eq!(value["location"]["kind"], "gc");
    assert_eq!(value["users"][0], "205519959982473217");

    let round_tripped: ActivityInstance =
        serde_json::from_str(&serde_json::to_string(&instance).unwrap()).unwrap();

    assert_eq!(round_tripped, instance);
}

/// Tests the private channel location kind and a nullable guild ID.
#[test]
fn activity_location_handles_private_channels() {
    let location = serde_json::from_str::<cordion_model::application::ActivityLocation>(
        r#"{
            "id": "pc-1234",
            "kind": "pc",
            "channel_id": "912954213460484116",
            "guild_id": null
        }"#,
    )
    .unwrap();

    assert_eq!(location.kind, ActivityLocationKind::PRIVATE_CHANNEL);
    assert_eq!(location.guild_id, None);

    let value = serde_json::to_value(&location).unwrap();
    assert!(value["guild_id"].is_null());
}

/// Tests that an unknown activity location kind is preserved.
#[test]
fn activity_location_kind_preserves_unknown_value() {
    let kind: ActivityLocationKind = serde_json::from_str("\"new-kind\"").unwrap();

    assert_eq!(kind.as_str(), "new-kind");
    assert_eq!(serde_json::to_value(&kind).unwrap(), serde_json::json!("new-kind"));
}

/// Tests the webhook event ping payload, which carries no event body.
#[test]
fn webhook_event_ping_carries_no_event() {
    let payload: WebhookEventPayload = serde_json::from_str(
        r#"{"version": 1, "application_id": "1234560123453231555", "type": 0}"#,
    )
    .unwrap();

    assert_eq!(payload.version, 1);
    assert_eq!(payload.kind, WebhookEventType::PING);
    assert_eq!(payload.application_id.get(), 1_234_560_123_453_231_555);
    assert_eq!(payload.event, None);

    let value = serde_json::to_value(&payload).unwrap();
    assert_eq!(value["type"], 0);
    assert!(value.get("event").is_none());
}

/// Tests the documented `APPLICATION_AUTHORIZED` webhook event.
#[test]
fn webhook_event_application_authorized_matches_example() {
    let payload: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234560123453231555",
            "type": 1,
            "event": {
                "type": "APPLICATION_AUTHORIZED",
                "timestamp": "2024-10-18T14:42:53.064834+00:00",
                "data": {
                    "integration_type": 1,
                    "scopes": ["applications.commands"],
                    "user": {
                        "id": "1234560123453231556",
                        "username": "Mason",
                        "discriminator": "9999"
                    }
                }
            }
        }"#,
    )
    .unwrap();

    assert_eq!(payload.kind, WebhookEventType::EVENT);

    let Some(event) = payload.event.as_ref() else {
        panic!("expected an event body");
    };
    assert_eq!(event.kind, WebhookEventBodyType::APPLICATION_AUTHORIZED);
    assert_eq!(event.timestamp, timestamp(1_729_262_573, 64_834_000));

    let Some(WebhookEventBodyData::ApplicationAuthorized(data)) = event.data.as_ref() else {
        panic!("expected application authorized data");
    };
    assert_eq!(data.integration_type, Some(ApplicationIntegrationType::USER_INSTALL));
    assert_eq!(data.scopes, ["applications.commands"]);
    assert_eq!(data.user.id.get(), 1_234_560_123_453_231_556);
    assert_eq!(data.guild, None);

    let value = serde_json::to_value(&payload).unwrap();
    assert_eq!(value["event"]["data"]["integration_type"], 1);
    assert_eq!(value["event"]["data"]["scopes"][0], "applications.commands");
    assert_eq!(value["event"]["data"]["user"]["username"], "Mason");
}

/// Tests the documented `APPLICATION_DEAUTHORIZED` webhook event.
#[test]
fn webhook_event_application_deauthorized_matches_example() {
    let payload: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234560123453231555",
            "type": 1,
            "event": {
                "type": "APPLICATION_DEAUTHORIZED",
                "timestamp": "2024-10-18T14:42:53.064834+00:00",
                "data": {
                    "user": {
                        "id": "1234560123453231556",
                        "username": "Mason",
                        "discriminator": "9999"
                    }
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = payload.event.as_ref() else {
        panic!("expected an event body");
    };
    assert_eq!(event.kind, WebhookEventBodyType::APPLICATION_DEAUTHORIZED);

    let Some(WebhookEventBodyData::ApplicationDeauthorized(data)) = event.data.as_ref() else {
        panic!("expected application deauthorized data");
    };
    assert_eq!(data.user.id.get(), 1_234_560_123_453_231_556);
    assert_eq!(data.user.username, "Mason");
}

/// Tests that the entitlement webhook events carry an entitlement object.
#[test]
fn webhook_event_entitlement_create_carries_entitlement() {
    let payload: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234560123453231555",
            "type": 1,
            "event": {
                "type": "ENTITLEMENT_CREATE",
                "timestamp": "2024-10-18T18:41:21.109604+00:00",
                "data": {
                    "application_id": "1234560123453231555",
                    "consumed": false,
                    "deleted": false,
                    "gift_code_flags": 0,
                    "id": "1234505980407808808",
                    "promotion_id": null,
                    "sku_id": "123489045643835123",
                    "type": 4,
                    "user_id": "111178765189277770"
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = payload.event.as_ref() else {
        panic!("expected an event body");
    };
    assert_eq!(event.kind, WebhookEventBodyType::ENTITLEMENT_CREATE);
    assert_eq!(event.timestamp, timestamp(1_729_276_881, 109_604_000));

    let Some(WebhookEventBodyData::Entitlement(entitlement)) = event.data.as_ref() else {
        panic!("expected entitlement data");
    };
    assert_eq!(entitlement.id.get(), 1_234_505_980_407_808_808);
    assert_eq!(entitlement.sku_id.get(), 123_489_045_643_835_123);
    assert_eq!(entitlement.kind, EntitlementType::TEST_MODE_PURCHASE);
    assert!(!entitlement.deleted);
    assert_eq!(entitlement.user_id.unwrap().get(), 111_178_765_189_277_770);

    let value = serde_json::to_value(&payload).unwrap();
    assert_eq!(value["event"]["data"]["id"], "1234505980407808808");
    assert_eq!(value["event"]["data"]["type"], 4);
}

/// Tests the documented lobby message webhook events.
///
/// Discord's webhook event examples write the outer event `timestamp` without a
/// timezone designator, so the payloads below add the `+00:00` offset that
/// Discord's ISO 8601 timestamps require.
#[test]
fn webhook_event_lobby_messages_match_examples() {
    let created: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234567765431056709",
            "type": 1,
            "event": {
                "type": "LOBBY_MESSAGE_CREATE",
                "timestamp": "2024-10-18T18:41:21.109604+00:00",
                "data": {
                    "id": "1397729799727878254",
                    "type": 0,
                    "content": "welcome to the party!",
                    "lobby_id": "1397729744753266719",
                    "channel_id": "1397729744753266719",
                    "author": {
                        "id": "1397729799727878255",
                        "username": "Mason",
                        "discriminator": "0"
                    },
                    "flags": 65536,
                    "application_id": "1234567765431056709"
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = created.event.as_ref() else {
        panic!("expected an event body");
    };
    let Some(WebhookEventBodyData::LobbyMessage(data)) = event.data.as_ref() else {
        panic!("expected lobby message data");
    };
    assert_eq!(data.id.get(), 1_397_729_799_727_878_254);
    assert_eq!(data.kind, MessageType::DEFAULT);
    assert_eq!(data.content, "welcome to the party!");
    assert_eq!(data.lobby_id.get(), 1_397_729_744_753_266_719);
    assert_eq!(data.flags.bits(), 65_536);
    assert_eq!(data.author.username, "Mason");
    assert_eq!(data.application_id.unwrap().get(), 1_234_567_765_431_056_709);
    assert_eq!(data.edited_timestamp, None);
    assert_eq!(data.timestamp, None);

    let updated: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234567765431056709",
            "type": 1,
            "event": {
                "type": "LOBBY_MESSAGE_UPDATE",
                "timestamp": "2025-08-05T20:39:19.587872+00:00",
                "data": {
                    "id": "1402390388030832792",
                    "type": 0,
                    "content": "noice",
                    "lobby_id": "1402385687281537066",
                    "channel_id": "1402389638311841883",
                    "author": {
                        "id": "1402389638311841884",
                        "username": "Nelly",
                        "discriminator": "0"
                    },
                    "edited_timestamp": "2025-08-05T20:39:19.557970+00:00",
                    "flags": 0,
                    "timestamp": "2025-08-05T20:38:43.660000+00:00"
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = updated.event.as_ref() else {
        panic!("expected an event body");
    };
    assert_eq!(event.kind, WebhookEventBodyType::LOBBY_MESSAGE_UPDATE);

    let Some(WebhookEventBodyData::LobbyMessage(data)) = event.data.as_ref() else {
        panic!("expected lobby message data");
    };
    assert_eq!(data.content, "noice");
    assert_eq!(data.id.get(), 1_402_390_388_030_832_792);
    assert_eq!(data.application_id, None);
    assert_eq!(data.edited_timestamp, Some(timestamp(1_754_426_359, 557_970_000)));
    assert_eq!(data.timestamp, Some(timestamp(1_754_426_323, 660_000_000)));

    let deleted: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234567765431056709",
            "type": 1,
            "event": {
                "type": "LOBBY_MESSAGE_DELETE",
                "timestamp": "2025-08-05T21:44:09.412957+00:00",
                "data": {
                    "id": "1402406637632884857",
                    "lobby_id": "1402399883394285659"
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = deleted.event.as_ref() else {
        panic!("expected an event body");
    };

    let Some(WebhookEventBodyData::LobbyMessageDelete(data)) = event.data.as_ref() else {
        panic!("expected lobby message delete data");
    };
    assert_eq!(data.id.get(), 1_402_406_637_632_884_857);
    assert_eq!(data.lobby_id.get(), 1_402_399_883_394_285_659);
}

/// Tests the documented game direct message webhook events.
#[test]
fn webhook_event_game_direct_messages_match_examples() {
    let created: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234567765431056709",
            "type": 1,
            "event": {
                "type": "GAME_DIRECT_MESSAGE_CREATE",
                "timestamp": "2025-08-14T18:09:38.063234+00:00",
                "data": {
                    "id": "1405614357781545021",
                    "type": 0,
                    "content": "get in friend, we're going raiding",
                    "channel_id": "1405604229820715098",
                    "author": {
                        "id": "1405604229820715099",
                        "username": "Mason",
                        "discriminator": "0"
                    },
                    "timestamp": "2025-08-14T18:09:37.947000+00:00",
                    "application_id": "1234567765431056709",
                    "attachments": []
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = created.event.as_ref() else {
        panic!("expected an event body");
    };

    let Some(WebhookEventBodyData::GameDirectMessage(data)) = event.data.as_ref() else {
        panic!("expected a game direct message");
    };
    assert_eq!(data.id.get(), 1_405_614_357_781_545_021);
    assert_eq!(data.kind, Some(MessageType::DEFAULT));
    assert_eq!(data.content, "get in friend, we're going raiding");
    assert_eq!(data.channel_id.get(), 1_405_604_229_820_715_098);
    assert_eq!(data.author.username, "Mason");
    assert_eq!(data.timestamp, Some(timestamp(1_755_194_977, 947_000_000)));
    assert_eq!(data.attachments.as_ref().unwrap().len(), 0);
    assert_eq!(data.lobby_id, None);

    let updated: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234567765431056709",
            "type": 1,
            "event": {
                "type": "GAME_DIRECT_MESSAGE_UPDATE",
                "timestamp": "2025-08-14T16:44:31.847073+00:00",                "data": {
                    "id": "1405591838810706081",
                    "content": "almost ready to queue?",
                    "channel_id": "1404960877324533784",
                    "author": {
                        "id": "1404960877324533785",
                        "username": "Nelly",
                        "discriminator": "0"
                    },
                    "recipient_id": "1404960877324533784"
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = updated.event.as_ref() else {
        panic!("expected an event body");
    };

    let Some(WebhookEventBodyData::GameDirectMessage(data)) = event.data.as_ref() else {
        panic!("expected a game direct message");
    };
    assert_eq!(data.id.get(), 1_405_591_838_810_706_081);
    assert_eq!(data.kind, None);
    assert_eq!(data.content, "almost ready to queue?");
    assert_eq!(data.timestamp, None);
}

/// Tests that a game direct message without `channel_id` decodes into the SDK
/// DM message variant.
#[test]
fn webhook_event_game_direct_message_sdk_variant() {
    let payload: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234567765431056709",
            "type": 1,
            "event": {
                "type": "GAME_DIRECT_MESSAGE_CREATE",
                "timestamp": "2025-08-20T17:01:50.099204+00:00",
                "data": {
                    "id": "1407771600643686503",
                    "type": 0,
                    "content": "cant make it in time",
                    "author": {
                        "id": "1407771600643686504",
                        "username": "Mason",
                        "discriminator": "0"
                    },
                    "flags": 0,
                    "application_id": "1234567765431056709",
                    "channel": {
                        "id": "1407771600643686505",
                        "type": 1,
                        "recipients": [
                            {"id": "1407771600643686504", "username": "Mason", "discriminator": "0"}
                        ]
                    }
                }
            }
        }"#,
    )
    .unwrap();

    let Some(event) = payload.event.as_ref() else {
        panic!("expected an event body");
    };

    let Some(WebhookEventBodyData::GameDirectMessageSdk(data)) = event.data.as_ref() else {
        panic!("expected an SDK DM message");
    };
    assert_eq!(data.id.get(), 1_407_771_600_643_686_503);
    assert_eq!(data.kind, Some(MessageType::DEFAULT));
    assert_eq!(data.content, "cant make it in time");
    assert_eq!(data.author.username, "Mason");
    assert_eq!(data.flags, MessageFlags::NONE);
    assert_eq!(data.application_id.get(), 1_234_567_765_431_056_709);
    assert_eq!(data.channel.id.get(), 1_407_771_600_643_686_505);
    assert_eq!(data.activity, None);
    assert_eq!(data.application, None);
}

/// Tests that an unknown webhook event type preserves its data as JSON.
#[test]
fn webhook_event_unknown_type_is_preserved() {
    let payload: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234560123453231555",
            "type": 1,
            "event": {
                "type": "SOMETHING_NEW",
                "timestamp": "2024-10-18T14:42:53.064834+00:00",
                "data": {"unexpected": [1, 2, 3]}
            }
        }"#,
    )
    .unwrap();

    let Some(event) = payload.event.as_ref() else {
        panic!("expected an event body");
    };
    assert_eq!(event.kind.as_str(), "SOMETHING_NEW");

    let Some(WebhookEventBodyData::Unknown(data)) = event.data.as_ref() else {
        panic!("expected unknown data");
    };
    assert_eq!(data["unexpected"][2], 3);

    let unavailable: WebhookEventPayload = serde_json::from_str(
        r#"{
            "version": 1,
            "application_id": "1234560123453231555",
            "type": 1,
            "event": {
                "type": "QUEST_USER_ENROLLMENT",
                "timestamp": "2024-10-18T14:42:53.064834+00:00"
            }
        }"#,
    )
    .unwrap();

    let Some(event) = unavailable.event.as_ref() else {
        panic!("expected an event body");
    };
    assert_eq!(event.kind, WebhookEventBodyType::QUEST_USER_ENROLLMENT);
    assert_eq!(event.data, None);
}

/// Tests the `WEBHOOKS_UPDATE` and `THREAD_MEMBER_UPDATE` payloads.
#[test]
fn webhooks_update_and_thread_member_update_round_trip() {
    let update: WebhooksUpdate = serde_json::from_str(
        r#"{"guild_id": "197038439483310086", "channel_id": "199737254929760256"}"#,
    )
    .unwrap();

    assert_eq!(update.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(update.channel_id.get(), 199_737_254_929_760_256);

    let value = serde_json::to_value(update).unwrap();
    assert_eq!(value["guild_id"], "197038439483310086");
    assert_eq!(value["channel_id"], "199737254929760256");

    let member: ThreadMemberUpdate = serde_json::from_str(
        r#"{
            "id": "1099032703076216842",
            "user_id": "80351110224678912",
            "join_timestamp": "2021-04-12T23:40:39.855793+00:00",
            "flags": 0,
            "guild_id": "197038439483310086"
        }"#,
    )
    .unwrap();

    assert_eq!(member.guild_id.get(), 197_038_439_483_310_086);
    assert_eq!(member.member.id.unwrap().get(), 1_099_032_703_076_216_842);
    assert_eq!(member.member.user_id.unwrap().get(), 80_351_110_224_678_912);
    assert_eq!(member.member.join_timestamp, timestamp(1_618_270_839, 855_793_000));
    assert_eq!(member.member.flags, 0);
    assert_eq!(member.member.member, None);

    let value = serde_json::to_value(&member).unwrap();
    assert_eq!(value["guild_id"], "197038439483310086");
    assert_eq!(value["join_timestamp"], "2021-04-12T23:40:39.855793+00:00");

    let round_tripped: ThreadMemberUpdate =
        serde_json::from_str(&serde_json::to_string(&member).unwrap()).unwrap();

    assert_eq!(round_tripped, member);
}

/// Tests the gateway-only extra fields of `MESSAGE_CREATE`.
#[test]
fn message_create_extra_fields_decode() {
    let message: Message = serde_json::from_str(
        r#"{
            "id": "1081000000000000001",
            "channel_id": "1081000000000000002",
            "guild_id": "1081000000000000007",
            "channel_type": 0,
            "author": {
                "id": "1081000000000000003",
                "username": "Mason",
                "discriminator": "1337"
            },
            "content": "Hello",
            "timestamp": "2026-01-02T03:04:05.006000+00:00",
            "edited_timestamp": null,
            "tts": false,
            "mention_everyone": false,
            "mention_roles": [],
            "mentions": [
                {
                    "id": "1081000000000000004",
                    "username": "Nelly",
                    "discriminator": "0",
                    "member": {
                        "roles": ["1081000000000000005"],
                        "joined_at": "2021-02-12T18:25:07.972000+00:00",
                        "nick": "Nelly"
                    }
                }
            ],
            "attachments": [],
            "embeds": [],
            "pinned": false,
            "type": 0,
            "member": {
                "roles": [],
                "joined_at": "2015-04-26T06:26:56.936000+00:00",
                "deaf": false,
                "mute": false
            }
        }"#,
    )
    .unwrap();

    assert_eq!(message.guild_id.unwrap().get(), 1_081_000_000_000_000_007);
    assert_eq!(message.channel_type, Some(ChannelType::GUILD_TEXT));
    let author_member = message.member.as_ref().unwrap();
    assert_eq!(author_member.joined_at, Some(timestamp(1_430_029_616, 936_000_000)));

    let mentioned = &message.mentions[0];
    let mentioned_member = mentioned.member.as_ref().unwrap();
    assert_eq!(mentioned_member.nick.as_deref(), Some("Nelly"));
    assert_eq!(mentioned_member.roles[0].get(), 1_081_000_000_000_000_005);

    let value = serde_json::to_value(&message).unwrap();
    assert_eq!(value["guild_id"], "1081000000000000007");
    assert_eq!(value["channel_type"], 0);
    assert_eq!(value["member"]["joined_at"], "2015-04-26T06:26:56.936+00:00");
    assert_eq!(value["mentions"][0]["member"]["nick"], "Nelly");

    let round_tripped: Message =
        serde_json::from_str(&serde_json::to_string(&message).unwrap()).unwrap();

    assert_eq!(round_tripped, message);
}

/// Tests that the gateway-only message fields are absent in REST payloads.
#[test]
fn message_rest_payloads_omit_gateway_fields() {
    let message: Message = serde_json::from_str(
        r#"{
            "id": "1",
            "channel_id": "2",
            "author": {"id": "3", "username": "Mason", "discriminator": "0"},
            "content": "",
            "timestamp": "2023-09-01T14:48:02.222000+00:00",
            "edited_timestamp": null,
            "tts": false,
            "mention_everyone": false,
            "mention_roles": [],
            "mentions": [],
            "attachments": [],
            "embeds": [],
            "pinned": false,
            "type": 0
        }"#,
    )
    .unwrap();

    assert_eq!(message.guild_id, None);
    assert_eq!(message.member, None);
    assert_eq!(message.channel_type, None);
    assert!(message.mentions.is_empty());

    let value = serde_json::to_value(&message).unwrap();
    assert!(value.get("guild_id").is_none());
    assert!(value.get("member").is_none());
    assert!(value.get("channel_type").is_none());
}

/// Tests that a reaction emoji object may carry a `null` name.
#[test]
fn emoji_allows_null_name_for_reactions() {
    let emoji: Emoji =
        serde_json::from_str(r#"{"id": "41771983429993937", "name": null}"#).unwrap();

    assert_eq!(emoji.id.unwrap().get(), 41_771_983_429_993_937);
    assert_eq!(emoji.name, None);

    let value = serde_json::to_value(&emoji).unwrap();
    assert!(value["name"].is_null());

    let custom: Emoji =
        serde_json::from_str(r#"{"id": "41771983429993937", "name": "LUL", "animated": true}"#)
            .unwrap();

    assert_eq!(custom.name.as_deref(), Some("LUL"));
}

/// Tests that the Gateway sends activity buttons as their labels.
#[test]
fn activity_buttons_are_received_labels() {
    let activity: cordion_model::event::presence::Activity = serde_json::from_str(
        r#"{
            "name": "Rocket League",
            "type": 0,
            "created_at": 15112000660000,
            "buttons": ["Join", "Spectate"]
        }"#,
    )
    .unwrap();

    assert_eq!(
        activity.buttons.as_deref(),
        Some(&["Join".to_string(), "Spectate".to_string()][..])
    );

    let value = serde_json::to_value(&activity).unwrap();
    assert_eq!(value["buttons"][0], "Join");

    let without_buttons: cordion_model::event::presence::Activity = serde_json::from_str(
        r#"{"name": "Rocket League", "type": 0, "created_at": 15112000660000}"#,
    )
    .unwrap();

    assert_eq!(without_buttons.buttons, None);
}

/// Tests that every field of a guild channel creation request round-trips.
#[test]
fn create_guild_channel_request_round_trips() {
    let payload = json!({
        "name": "new-channel",
        "type": 0,
        "topic": "channel topic",
        "bitrate": 64000,
        "user_limit": 10,
        "rate_limit_per_user": 2,
        "position": 6,
        "permission_overwrites": [
            {"id": "41771983423143937", "type": 0, "allow": "1024", "deny": "0"}
        ],
        "parent_id": "399942396007890945",
        "nsfw": true,
        "rtc_region": "us-west",
        "video_quality_mode": 1,
        "default_auto_archive_duration": 1440,
        "default_reaction_emoji": {"emoji_id": null, "emoji_name": "🔥"},
        "available_tags": [
            {
                "id": "41771983423143938",
                "name": "tag",
                "moderated": false,
                "emoji_id": "41771983423143939",
                "emoji_name": null
            }
        ],
        "default_sort_order": 1,
        "default_forum_layout": 1,
        "default_thread_rate_limit_per_user": 0,
        "flags": 0
    });

    let request: CreateGuildChannelRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.name, "new-channel");
    assert_eq!(request.kind, Some(ChannelType::GUILD_TEXT));
    assert_eq!(request.topic.as_deref(), Some("channel topic"));
    assert_eq!(request.bitrate, Some(64000));
    assert_eq!(request.user_limit, Some(10));
    assert_eq!(request.rate_limit_per_user, Some(2));
    assert_eq!(request.position, Some(6));
    assert_eq!(request.parent_id, Some(ChannelId::from(399942396007890945)));
    assert_eq!(request.nsfw, Some(true));
    assert_eq!(request.rtc_region.as_deref(), Some("us-west"));
    assert_eq!(request.video_quality_mode, Some(VideoQualityMode::AUTO));
    assert_eq!(request.default_auto_archive_duration, Some(1440));
    assert_eq!(request.default_thread_rate_limit_per_user, Some(0));
    assert_eq!(request.default_sort_order, Some(SortOrderMode::CREATION_DATE));
    assert_eq!(request.default_forum_layout, Some(ForumLayout::LIST_VIEW));

    let overwrites = request.permission_overwrites.as_ref().unwrap();
    assert_eq!(overwrites.len(), 1);
    assert_eq!(overwrites[0].id, OverwriteId::from(41771983423143937));
    assert_eq!(overwrites[0].kind, OverwriteType::ROLE);
    assert_eq!(overwrites[0].allow, Permission::VIEW_CHANNEL);
    assert_eq!(overwrites[0].deny, Permission::NONE);

    let tags = request.available_tags.as_ref().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0].id, ForumTagId::from(41771983423143938));
    assert_eq!(tags[0].name, "tag");
    assert!(!tags[0].moderated);
    assert_eq!(tags[0].emoji_id, Some(EmojiId::from(41771983423143939)));
    assert_eq!(tags[0].emoji_name, None);

    assert_eq!(
        request.default_reaction_emoji,
        Some(DefaultReaction {
            emoji_id: None,
            emoji_name: Some("🔥".to_owned())
        })
    );
    assert_eq!(request.flags, Some(ChannelFlags::NONE));

    assert_eq!(serde_json::to_value(&request).unwrap(), payload);
}

/// Tests that a guild channel creation request sends `null` for every nullable
/// field that was not set.
#[test]
fn create_guild_channel_request_serializes_nullable_defaults() {
    let request = CreateGuildChannelRequest {
        name: "general".to_owned(),
        kind: None,
        topic: None,
        bitrate: None,
        user_limit: None,
        rate_limit_per_user: None,
        position: None,
        permission_overwrites: None,
        parent_id: None,
        nsfw: None,
        rtc_region: None,
        video_quality_mode: None,
        default_auto_archive_duration: None,
        default_reaction_emoji: None,
        available_tags: None,
        default_sort_order: None,
        default_forum_layout: None,
        default_thread_rate_limit_per_user: None,
        flags: None,
    };

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "name": "general",
            "type": null,
            "topic": null,
            "bitrate": null,
            "user_limit": null,
            "rate_limit_per_user": null,
            "position": null,
            "permission_overwrites": null,
            "parent_id": null,
            "nsfw": null,
            "rtc_region": null,
            "video_quality_mode": null,
            "default_auto_archive_duration": null,
            "default_reaction_emoji": null,
            "available_tags": null,
            "default_sort_order": null,
            "default_forum_layout": null,
            "default_thread_rate_limit_per_user": null,
            "flags": null
        })
    );
}

/// Tests that the guild channel modify request omits its optional fields while
/// serializing its nullable fields as `null`.
#[test]
fn modify_guild_channel_request_distinguishes_nullable_from_optional() {
    let request = ModifyGuildChannelRequest {
        name: Some("renamed".to_owned()),
        kind: None,
        position: None,
        topic: Some("new topic".to_owned()),
        nsfw: Some(false),
        rate_limit_per_user: None,
        bitrate: Some(64000),
        user_limit: None,
        permission_overwrites: Some(vec![Overwrite {
            id: OverwriteId::from(41771983423143937),
            kind: OverwriteType::ROLE,
            allow: Permission::VIEW_CHANNEL,
            deny: Permission::NONE,
        }]),
        parent_id: None,
        rtc_region: None,
        video_quality_mode: None,
        default_auto_archive_duration: None,
        flags: Some(ChannelFlags::PINNED),
        available_tags: Some(vec![ForumTag {
            id: ForumTagId::from(41771983423143938),
            name: "tag".to_owned(),
            moderated: true,
            emoji_id: None,
            emoji_name: None,
        }]),
        default_reaction_emoji: None,
        default_thread_rate_limit_per_user: None,
        default_sort_order: None,
        default_forum_layout: Some(ForumLayout::LIST_VIEW),
    };

    let value = serde_json::to_value(&request).unwrap();

    assert_eq!(
        value,
        json!({
            "name": "renamed",
            "topic": "new topic",
            "nsfw": false,
            "bitrate": 64000,
            "position": null,
            "rate_limit_per_user": null,
            "user_limit": null,
            "permission_overwrites": [
                {"id": "41771983423143937", "type": 0, "allow": "1024", "deny": "0"}
            ],
            "parent_id": null,
            "rtc_region": null,
            "video_quality_mode": null,
            "default_auto_archive_duration": null,
            "default_reaction_emoji": null,
            "default_sort_order": null,
            "flags": 2,
            "available_tags": [
                {
                    "id": "41771983423143938",
                    "name": "tag",
                    "moderated": true,
                    "emoji_id": null,
                    "emoji_name": null
                }
            ],
            "default_forum_layout": 1
        })
    );
    assert!(value.get("type").is_none());
    assert!(value.get("default_thread_rate_limit_per_user").is_none());

    assert_eq!(serde_json::from_value::<ModifyGuildChannelRequest>(value).unwrap(), request);
}

/// Tests that the group DM modify request round-trips and defaults to an empty
/// body.
#[test]
fn modify_group_dm_channel_request_round_trips() {
    let payload = json!({"name": "Some test channel", "icon": "aGVsbG8="});

    let request: ModifyGroupDmChannelRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.name.as_deref(), Some("Some test channel"));
    assert_eq!(request.icon.as_deref(), Some("aGVsbG8="));
    assert_eq!(serde_json::to_value(&request).unwrap(), payload);

    let empty: ModifyGroupDmChannelRequest = serde_json::from_value(json!({})).unwrap();

    assert_eq!(empty, ModifyGroupDmChannelRequest { name: None, icon: None });
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
}

/// Tests that the thread modify request round-trips and that its nullable
/// slowmode field accepts `null`.
#[test]
fn modify_thread_channel_request_round_trips() {
    let payload = json!({
        "name": "don't buy dota-2",
        "archived": false,
        "auto_archive_duration": 1440,
        "locked": false,
        "invitable": true,
        "rate_limit_per_user": null,
        "flags": 2,
        "applied_tags": ["41771983423143938"]
    });

    let request: ModifyThreadChannelRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.name.as_deref(), Some("don't buy dota-2"));
    assert_eq!(request.archived, Some(false));
    assert_eq!(request.auto_archive_duration, Some(1440));
    assert_eq!(request.locked, Some(false));
    assert_eq!(request.invitable, Some(true));
    assert_eq!(request.rate_limit_per_user, None);
    assert_eq!(request.flags, Some(ChannelFlags::PINNED));

    let applied_tags = request.applied_tags.as_ref().unwrap();
    assert_eq!(applied_tags.len(), 1);
    assert_eq!(applied_tags[0], ForumTagId::from(41771983423143938));

    assert_eq!(serde_json::to_value(&request).unwrap(), payload);

    let omitted: ModifyThreadChannelRequest =
        serde_json::from_value(json!({"name": "thread"})).unwrap();

    assert_eq!(omitted.rate_limit_per_user, None);
    assert_eq!(omitted.flags, None);
    assert_eq!(
        serde_json::to_value(&omitted).unwrap(),
        json!({"name": "thread", "rate_limit_per_user": null})
    );
}

/// Tests that the channel permission overwrite request round-trips its nullable
/// permission fields.
#[test]
fn edit_channel_permissions_request_round_trips() {
    let payload = json!({"allow": "1024", "deny": null, "type": 1});

    let request: EditChannelPermissionsRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.allow, Some(Permission::VIEW_CHANNEL));
    assert_eq!(request.deny, None);
    assert_eq!(request.kind, OverwriteType::MEMBER);
    assert_eq!(serde_json::to_value(request).unwrap(), payload);

    let empty: EditChannelPermissionsRequest = serde_json::from_value(json!({"type": 0})).unwrap();

    assert_eq!(empty.allow, None);
    assert_eq!(empty.deny, None);
    assert_eq!(empty.kind, OverwriteType::ROLE);
    assert_eq!(
        serde_json::to_value(empty).unwrap(),
        json!({"allow": null, "deny": null, "type": 0})
    );
}

/// Tests the thread started from an existing message.
#[test]
fn start_thread_from_message_request_round_trips() {
    let payload = json!({
        "name": "new thread",
        "auto_archive_duration": 1440,
        "rate_limit_per_user": null
    });

    let request: StartThreadFromMessageRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.name, "new thread");
    assert_eq!(request.auto_archive_duration, Some(1440));
    assert_eq!(request.rate_limit_per_user, None);
    assert_eq!(serde_json::to_value(&request).unwrap(), payload);

    let minimal: StartThreadFromMessageRequest =
        serde_json::from_value(json!({"name": "new thread"})).unwrap();

    assert_eq!(minimal.auto_archive_duration, None);
    assert_eq!(
        serde_json::to_value(&minimal).unwrap(),
        json!({"name": "new thread", "rate_limit_per_user": null})
    );
}

/// Tests the thread started without an existing message.
#[test]
fn start_thread_without_message_request_round_trips() {
    let payload = json!({
        "name": "new thread",
        "auto_archive_duration": 60,
        "type": 11,
        "invitable": true,
        "rate_limit_per_user": 0
    });

    let request: StartThreadWithoutMessageRequest =
        serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.name, "new thread");
    assert_eq!(request.auto_archive_duration, Some(60));
    assert_eq!(request.kind, Some(ChannelType::PUBLIC_THREAD));
    assert_eq!(request.invitable, Some(true));
    assert_eq!(request.rate_limit_per_user, Some(0));
    assert_eq!(serde_json::to_value(&request).unwrap(), payload);

    let minimal: StartThreadWithoutMessageRequest =
        serde_json::from_value(json!({"name": "new thread"})).unwrap();

    assert_eq!(minimal.kind, None);
    assert_eq!(minimal.invitable, None);
    assert_eq!(
        serde_json::to_value(&minimal).unwrap(),
        json!({"name": "new thread", "rate_limit_per_user": null})
    );
}

/// Tests the forum thread creation request with a fully populated message.
#[test]
fn start_thread_in_forum_or_media_thread_request_round_trips() {
    let payload = json!({
        "name": "First post",
        "auto_archive_duration": 1440,
        "rate_limit_per_user": null,
        "message": {
            "content": "Hello, forum!",
            "embeds": [
                {"title": "Hello, Embed!", "description": "This is an embedded message."}
            ],
            "allowed_mentions": {"parse": ["users", "roles", "everyone"]},
            "components": [
                {
                    "type": 1,
                    "components": [
                        {"type": 2, "style": 1, "custom_id": "click", "label": "Click"}
                    ]
                }
            ],
            "sticker_ids": ["41771983423143940"],
            "attachments": [{"id": 0, "filename": "file.png", "is_spoiler": false}],
            "flags": 4096
        },
        "applied_tags": ["41771983423143938"]
    });

    let request: StartThreadInForumOrMediaThreadRequest =
        serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.name, "First post");
    assert_eq!(request.auto_archive_duration, Some(1440));
    assert_eq!(request.rate_limit_per_user, None);

    let applied_tags = request.applied_tags.as_ref().unwrap();
    assert_eq!(applied_tags.len(), 1);
    assert_eq!(applied_tags[0], ForumTagId::from(41771983423143938));

    let message = &request.message;
    assert_eq!(message.content.as_deref(), Some("Hello, forum!"));

    let embeds = message.embeds.as_ref().unwrap();
    assert_eq!(embeds.len(), 1);
    assert_eq!(embeds[0].title.as_deref(), Some("Hello, Embed!"));

    let allowed_mentions = message.allowed_mentions.as_ref().unwrap();
    let parse = allowed_mentions.parse.as_ref().unwrap();
    assert_eq!(parse.len(), 3);
    assert_eq!(parse[0], AllowedMentionType::USERS);
    assert_eq!(parse[1], AllowedMentionType::ROLES);
    assert_eq!(parse[2], AllowedMentionType::EVERYONE);

    let components = message.components.as_ref().unwrap();
    assert_eq!(components.len(), 1);
    assert!(matches!(components[0], Component::ActionRow(_)));

    let sticker_ids = message.sticker_ids.as_ref().unwrap();
    assert_eq!(sticker_ids.len(), 1);
    assert_eq!(sticker_ids[0], StickerId::from(41771983423143940));

    let attachments = message.attachments.as_ref().unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0].id, AttachmentRequestId::Index(0));
    assert_eq!(attachments[0].filename.as_deref(), Some("file.png"));
    assert_eq!(attachments[0].is_spoiler, Some(false));
    assert_eq!(message.flags, Some(MessageFlags::SUPPRESS_NOTIFICATIONS));

    assert_eq!(serde_json::to_value(&request).unwrap(), payload);
}

/// Tests that a forum thread creation request without optional fields still
/// carries its required message object.
#[test]
fn start_thread_in_forum_or_media_thread_request_omits_optional_fields() {
    let request = StartThreadInForumOrMediaThreadRequest {
        name: "First post".to_owned(),
        auto_archive_duration: None,
        rate_limit_per_user: None,
        message: ForumAndMediaThreadMessageParams {
            content: None,
            embeds: None,
            allowed_mentions: None,
            components: None,
            sticker_ids: None,
            attachments: None,
            flags: None,
        },
        applied_tags: None,
    };

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"name": "First post", "rate_limit_per_user": null, "message": {}})
    );
}

/// Tests the documented create message example body.
#[test]
fn create_message_request_matches_documented_example() {
    let payload = json!({
        "content": "Hello, World!",
        "tts": false,
        "embeds": [
            {"title": "Hello, Embed!", "description": "This is an embedded message."}
        ]
    });

    let request: CreateMessageRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.content.as_deref(), Some("Hello, World!"));
    assert_eq!(request.tts, Some(false));

    let embeds = request.embeds.as_ref().unwrap();
    assert_eq!(embeds.len(), 1);
    assert_eq!(embeds[0].title.as_deref(), Some("Hello, Embed!"));
    assert_eq!(embeds[0].description.as_deref(), Some("This is an embedded message."));

    assert_eq!(serde_json::to_value(&request).unwrap(), payload);
}

/// Tests that every field of a create message request round-trips.
#[test]
fn create_message_request_round_trips() {
    let payload = json!({
        "content": "Hello, World!",
        "nonce": 12345,
        "tts": false,
        "embeds": [
            {"title": "Hello, Embed!", "description": "This is an embedded message."}
        ],
        "allowed_mentions": {"parse": []},
        "message_reference": {
            "type": 0,
            "message_id": "334385199974967042",
            "channel_id": "290926798629997250",
            "guild_id": "290926798629997250",
            "fail_if_not_exists": true
        },
        "components": [
            {
                "type": 1,
                "components": [
                    {"type": 2, "style": 1, "custom_id": "click", "label": "Click"}
                ]
            }
        ],
        "sticker_ids": ["41771983423143940"],
        "attachments": [{"id": "41771983423143941", "description": "kept"}],
        "flags": 4096,
        "enforce_nonce": true,
        "poll": {
            "question": {"text": "What is your favorite bug?"},
            "answers": [
                {"poll_media": {"text": "Ant"}},
                {"poll_media": {"text": "Bee"}}
            ],
            "duration": 24,
            "allow_multiselect": false,
            "layout_type": 1
        },
        "shared_client_theme": {
            "colors": ["5865F2", "7258F2"],
            "gradient_angle": 0,
            "base_mix": 58,
            "base_theme": 1
        }
    });

    let request: CreateMessageRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.nonce, Some(MessageNonce::Integer(12345)));
    assert_eq!(request.flags, Some(MessageFlags::SUPPRESS_NOTIFICATIONS));
    assert_eq!(request.enforce_nonce, Some(true));

    let allowed_mentions = request.allowed_mentions.as_ref().unwrap();
    assert_eq!(allowed_mentions.parse, Some(Vec::new()));

    let reference = request.message_reference.as_ref().unwrap();
    assert_eq!(reference.kind, Some(MessageReferenceType::DEFAULT));
    assert_eq!(reference.message_id, Some(MessageId::from(334385199974967042)));
    assert_eq!(reference.channel_id, Some(ChannelId::from(290926798629997250)));
    assert_eq!(reference.guild_id, Some(GuildId::from(290926798629997250)));
    assert_eq!(reference.fail_if_not_exists, Some(true));

    let components = request.components.as_ref().unwrap();
    assert_eq!(components.len(), 1);
    assert!(matches!(components[0], Component::ActionRow(_)));

    let sticker_ids = request.sticker_ids.as_ref().unwrap();
    assert_eq!(sticker_ids.len(), 1);
    assert_eq!(sticker_ids[0], StickerId::from(41771983423143940));

    let attachments = request.attachments.as_ref().unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(
        attachments[0].id,
        AttachmentRequestId::Snowflake(AttachmentId::from(41771983423143941))
    );
    assert_eq!(attachments[0].description.as_deref(), Some("kept"));

    let poll = request.poll.as_ref().unwrap();
    assert_eq!(
        poll.question,
        PollMedia {
            text: Some("What is your favorite bug?".to_owned()),
            emoji: None
        }
    );
    assert_eq!(poll.answers.len(), 2);
    assert_eq!(
        poll.answers[1],
        PollAnswer {
            answer_id: None,
            poll_media: PollMedia {
                text: Some("Bee".to_owned()),
                emoji: None
            }
        }
    );
    assert_eq!(poll.duration, Some(24));
    assert_eq!(poll.allow_multiselect, Some(false));
    assert_eq!(poll.layout_type, Some(PollLayoutType::DEFAULT));

    let theme = request.shared_client_theme.as_ref().unwrap();
    assert_eq!(theme.colors, vec!["5865F2".to_owned(), "7258F2".to_owned()]);
    assert_eq!(theme.gradient_angle, 0);
    assert_eq!(theme.base_mix, 58);
    assert_eq!(theme.base_theme, Some(BaseTheme::DARK));

    assert_eq!(serde_json::to_value(&request).unwrap(), payload);
}

/// Tests that a create message request with no fields sends an empty body.
#[test]
fn create_message_request_serializes_empty_body() {
    let request: CreateMessageRequest = serde_json::from_value(json!({})).unwrap();

    assert_eq!(request.content, None);
    assert_eq!(serde_json::to_value(&request).unwrap(), json!({}));
}

/// Tests that the edit message request serializes its nullable fields as `null`.
#[test]
fn edit_message_request_round_trips() {
    let payload = json!({
        "content": null,
        "embeds": [],
        "flags": 4,
        "allowed_mentions": {"parse": []},
        "components": [],
        "attachments": [{"id": "41771983423143941", "description": "kept"}]
    });

    let request: EditMessageRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.content, None);
    assert_eq!(request.embeds, Some(Vec::new()));
    assert_eq!(request.flags, Some(MessageFlags::SUPPRESS_EMBEDS));

    let allowed_mentions = request.allowed_mentions.as_ref().unwrap();
    assert_eq!(allowed_mentions.parse, Some(Vec::new()));

    assert_eq!(request.components, Some(Vec::new()));

    let attachments = request.attachments.as_ref().unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(
        attachments[0].id,
        AttachmentRequestId::Snowflake(AttachmentId::from(41771983423143941))
    );
    assert_eq!(attachments[0].description.as_deref(), Some("kept"));

    assert_eq!(serde_json::to_value(&request).unwrap(), payload);
}

/// Tests that an edit message request without fields sends `null` for every
/// field.
#[test]
fn edit_message_request_serializes_nullable_defaults() {
    let request: EditMessageRequest = serde_json::from_value(json!({})).unwrap();

    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({
            "content": null,
            "embeds": null,
            "flags": null,
            "allowed_mentions": null,
            "components": null,
            "attachments": null
        })
    );
}

/// Tests the bulk delete message request body.
#[test]
fn bulk_delete_messages_request_round_trips() {
    let payload = json!({"messages": ["334385199974967042", "334385199974967043"]});

    let request: BulkDeleteMessagesRequest = serde_json::from_value(payload.clone()).unwrap();

    assert_eq!(request.messages.len(), 2);
    assert_eq!(request.messages[0], MessageId::from(334385199974967042));
    assert_eq!(request.messages[1], MessageId::from(334385199974967043));
    assert_eq!(serde_json::to_value(&request).unwrap(), payload);

    assert!(serde_json::from_str::<BulkDeleteMessagesRequest>("{}").is_err());
}

/// Tests that an allowed mentions object suppresses mentions with an empty
/// parse list.
#[test]
fn allowed_mentions_suppresses_mentions_with_empty_parse_list() {
    let mentions: AllowedMentions = serde_json::from_value(json!({"parse": []})).unwrap();

    assert_eq!(mentions.parse, Some(Vec::new()));
    assert_eq!(mentions.roles, None);
    assert_eq!(serde_json::to_value(&mentions).unwrap(), json!({"parse": []}));
}

/// Tests that an embed keeps unknown future fields out of the round-trip.
#[test]
fn embed_round_trips_partial_object() {
    let embed: Embed = serde_json::from_value(json!({"title": "Hello, Embed!"})).unwrap();

    assert_eq!(embed.title.as_deref(), Some("Hello, Embed!"));
    assert_eq!(embed.description, None);
    assert_eq!(serde_json::to_value(&embed).unwrap(), json!({"title": "Hello, Embed!"}));
}

/// Tests that a poll create request keeps its request-only fields.
#[test]
fn poll_create_request_round_trips() {
    let poll = PollCreateRequest {
        question: PollMedia {
            text: Some("What is your favorite bug?".to_owned()),
            emoji: None,
        },
        answers: vec![
            PollAnswer {
                answer_id: None,
                poll_media: PollMedia {
                    text: Some("Ant".to_owned()),
                    emoji: None,
                },
            },
            PollAnswer {
                answer_id: None,
                poll_media: PollMedia {
                    text: Some("Bee".to_owned()),
                    emoji: None,
                },
            },
        ],
        duration: Some(24),
        allow_multiselect: Some(false),
        layout_type: Some(PollLayoutType::DEFAULT),
    };

    assert_eq!(
        serde_json::to_value(&poll).unwrap(),
        json!({
            "question": {"text": "What is your favorite bug?"},
            "answers": [
                {"poll_media": {"text": "Ant"}},
                {"poll_media": {"text": "Bee"}}
            ],
            "duration": 24,
            "allow_multiselect": false,
            "layout_type": 1
        })
    );
}

/// Tests that a shared client theme keeps its documented example shape.
#[test]
fn shared_client_theme_round_trips() {
    let theme = SharedClientTheme {
        colors: vec!["5865F2".to_owned(), "7258F2".to_owned()],
        gradient_angle: 0,
        base_mix: 58,
        base_theme: Some(BaseTheme::DARK),
    };

    assert_eq!(
        serde_json::to_value(&theme).unwrap(),
        json!({
            "colors": ["5865F2", "7258F2"],
            "gradient_angle": 0,
            "base_mix": 58,
            "base_theme": 1
        })
    );
}

/// Tests that an attachment request models both snowflake and upload index IDs.
#[test]
fn attachment_request_round_trips() {
    let attachment = AttachmentRequest {
        id: AttachmentRequestId::Index(0),
        filename: Some("file.png".to_owned()),
        title: None,
        description: None,
        duration_secs: None,
        waveform: None,
        is_spoiler: Some(false),
    };

    assert_eq!(
        serde_json::to_value(&attachment).unwrap(),
        json!({"id": 0, "filename": "file.png", "is_spoiler": false})
    );
}

/// Tests that a message reference round-trips its optional forward fields.
#[test]
fn message_reference_round_trips() {
    let reference = MessageReference {
        kind: Some(MessageReferenceType::DEFAULT),
        message_id: Some(MessageId::from(334385199974967042)),
        channel_id: Some(ChannelId::from(290926798629997250)),
        guild_id: Some(GuildId::from(290926798629997250)),
        fail_if_not_exists: Some(true),
    };

    assert_eq!(
        serde_json::to_value(reference).unwrap(),
        json!({
            "type": 0,
            "message_id": "334385199974967042",
            "channel_id": "290926798629997250",
            "guild_id": "290926798629997250",
            "fail_if_not_exists": true
        })
    );
}
/// Tests that webhook modification bodies omit absent fields and accept a
/// channel move.
#[test]
fn webhook_modification_requests_omit_absent_fields() {
    assert_eq!(
        serde_json::to_value(ModifyWebhookRequest::default()).unwrap(),
        serde_json::json!({})
    );
    assert_eq!(
        serde_json::to_value(ModifyWebhookWithTokenRequest::default()).unwrap(),
        serde_json::json!({})
    );

    let modify: ModifyWebhookRequest = serde_json::from_str(
        r#"{
            "name": "Spooky Webhook",
            "avatar": "data:image/png;base64,BASE64",
            "channel_id": "41771983423143937"
        }"#,
    )
    .unwrap();

    assert_eq!(modify.name.as_deref(), Some("Spooky Webhook"));
    assert_eq!(modify.avatar.as_deref(), Some("data:image/png;base64,BASE64"));
    assert_eq!(modify.channel_id.unwrap().get(), 41_771_983_423_143_937);

    let value = serde_json::to_value(&modify).unwrap();
    assert_eq!(value["name"], "Spooky Webhook");
    assert_eq!(value["channel_id"], "41771983423143937");

    let token: ModifyWebhookWithTokenRequest =
        serde_json::from_str(r#"{"name": "Renamed"}"#).unwrap();
    assert_eq!(token.name.as_deref(), Some("Renamed"));
    assert!(token.avatar.is_none());
}

/// Tests the execute webhook body with poll, attachments, and forum thread
/// fields.
#[test]
fn execute_webhook_request_matches_documented_fields() {
    let request: ExecuteWebhookRequest = serde_json::from_str(
        r#"{
            "content": "hello",
            "username": "Spooky",
            "avatar_url": "https://example.com/avatar.png",
            "tts": false,
            "embeds": [],
            "allowed_mentions": {"parse": ["users"]},
            "components": [],
            "attachments": [{"id": 0, "filename": "file.png"}],
            "flags": 4,
            "thread_name": "New thread",
            "applied_tags": ["41771983423143937"],
            "poll": {
                "question": {"text": "Best pet?"},
                "answers": [{"poll_media": {"text": "Cat"}}],
                "duration": 24,
                "allow_multiselect": true,
                "layout_type": 1
            }
        }"#,
    )
    .unwrap();

    assert_eq!(request.content.as_deref(), Some("hello"));
    assert_eq!(request.username.as_deref(), Some("Spooky"));
    assert_eq!(request.avatar_url.as_deref(), Some("https://example.com/avatar.png"));
    assert_eq!(request.tts, Some(false));
    assert_eq!(request.embeds.as_ref().unwrap().len(), 0);
    assert_eq!(request.attachments.as_ref().unwrap().len(), 1);
    assert_eq!(request.flags.unwrap().bits(), 4);
    assert_eq!(request.thread_name.as_deref(), Some("New thread"));
    assert_eq!(request.applied_tags.as_ref().unwrap().len(), 1);
    assert_eq!(
        request.poll.as_ref().unwrap().question,
        PollMedia {
            text: Some("Best pet?".to_string()),
            emoji: None,
        }
    );
    assert_eq!(request.poll.as_ref().unwrap().duration, Some(24));
    assert_eq!(request.poll.as_ref().unwrap().layout_type, Some(PollLayoutType::DEFAULT));

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["content"], "hello");
    assert_eq!(value["attachments"][0]["id"], 0);
    assert_eq!(value["applied_tags"][0], "41771983423143937");
    assert_eq!(value["poll"]["answers"][0]["poll_media"]["text"], "Cat");
    assert_eq!(value["poll"]["layout_type"], 1);

    assert_eq!(
        serde_json::to_value(ExecuteWebhookRequest::default()).unwrap(),
        serde_json::json!({})
    );
}

/// Tests the edit webhook message body.
#[test]
fn edit_webhook_message_request_matches_documented_fields() {
    let request: EditWebhookMessageRequest = serde_json::from_str(
        r#"{
            "content": "updated",
            "embeds": [],
            "flags": 4,
            "components": [],
            "attachments": []
        }"#,
    )
    .unwrap();

    assert_eq!(request.content.as_deref(), Some("updated"));
    assert_eq!(request.flags.unwrap().bits(), 4);
    assert!(request.allowed_mentions.is_none());

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["content"], "updated");
    assert_eq!(value["flags"], 4);
    assert!(value.get("username").is_none());
    assert!(value.get("poll").is_none() || value["poll"].is_null());
}

/// Tests the channel invite request, including the extended target fields.
#[test]
fn create_channel_invite_matches_documented_fields() {
    let request: CreateChannelInviteRequest = serde_json::from_str(
        r#"{
            "max_age": 3600,
            "max_uses": 10,
            "temporary": false,
            "unique": true,
            "target_type": 2,
            "target_user_id": "41771983429993000",
            "target_application_id": "172150183260323840",
            "target_user_ids": ["41771983429993000", "41771983429993001"],
            "role_ids": ["41771983429993002"]
        }"#,
    )
    .unwrap();

    assert_eq!(request.max_age, Some(3600));
    assert_eq!(request.max_uses, Some(10));
    assert_eq!(request.temporary, Some(false));
    assert_eq!(request.unique, Some(true));
    assert_eq!(request.target_type, Some(InviteTargetType::EMBEDDED_APPLICATION));
    assert_eq!(request.target_user_ids.as_ref().unwrap().len(), 2);
    assert_eq!(request.role_ids.as_ref().unwrap().len(), 1);

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["max_age"], 3600);
    assert_eq!(value["target_type"], 2);
    assert_eq!(value["target_application_id"], "172150183260323840");
    assert_eq!(value["target_user_ids"][1], "41771983429993001");
    assert_eq!(value["role_ids"][0], "41771983429993002");

    let empty: CreateChannelInviteRequest = serde_json::from_str("{}").unwrap();
    assert_eq!(serde_json::to_value(&empty).unwrap(), serde_json::json!({}));
}

/// Tests the target users bulk endpoints and job status response.
#[test]
fn invite_target_users_requests_match_documented_fields() {
    let add: BulkAddInviteTargetUsersRequest =
        serde_json::from_str(r#"{"user_ids": ["41771983429993000"]}"#).unwrap();
    assert_eq!(add.user_ids.len(), 1);
    assert_eq!(
        serde_json::to_value(&add).unwrap(),
        serde_json::json!({"user_ids": ["41771983429993000"]})
    );

    let delete: BulkDeleteInviteTargetUsersRequest =
        serde_json::from_str(r#"{"user_ids": ["41771983429993000"]}"#).unwrap();
    assert_eq!(delete.user_ids.len(), 1);

    let job: TargetUsersJobStatus = serde_json::from_str(
        r#"{
            "status": 2,
            "total_users": 1000,
            "processed_users": 1000,
            "created_at": "2024-08-27T19:48:44.406602+00:00",
            "completed_at": "2024-08-27T19:48:45.406602+00:00"
        }"#,
    )
    .unwrap();

    assert_eq!(job.status, TargetUsersJobStatusCode::COMPLETED);
    assert_eq!(job.total_users, 1000);
    assert_eq!(job.processed_users, 1000);
    assert!(job.error_message.is_none());
    assert!(job.completed_at.is_some());

    let value = serde_json::to_value(&job).unwrap();
    assert_eq!(value["status"], 2);
    assert_eq!(value["completed_at"], "2024-08-27T19:48:45.406602+00:00");

    let failed: TargetUsersJobStatus = serde_json::from_str(
        r#"{
            "status": 3,
            "total_users": 1,
            "processed_users": 0,
            "created_at": "2024-08-27T19:48:44.406602+00:00",
            "completed_at": null,
            "error_message": "file too large"
        }"#,
    )
    .unwrap();
    assert_eq!(failed.status, TargetUsersJobStatusCode::FAILED);
    assert_eq!(failed.error_message.as_deref(), Some("file too large"));
    assert!(failed.completed_at.is_none());
    assert_eq!(serde_json::to_value(&failed).unwrap()["completed_at"], serde_json::Value::Null);

    let unknown: TargetUsersJobStatusCode = serde_json::from_str("9").unwrap();
    assert_eq!(unknown.raw(), 9);
}

/// Tests the application command create and edit request bodies.
#[test]
fn application_command_requests_omit_response_only_fields() {
    let create: CreateGlobalApplicationCommandRequest = serde_json::from_str(
        r#"{
            "name": "blep",
            "name_localizations": {"fr": "bêler"},
            "description": "Send a random adorable animal photo",
            "description_localizations": {"fr": "Envoyer une photo d'animal"},
            "options": [
                {
                    "type": 3,
                    "name": "animal",
                    "description": "The type of animal",
                    "required": true
                }
            ],
            "default_member_permissions": "2048",
            "dm_permission": false,
            "default_permission": true,
            "integration_types": [0, 1],
            "contexts": [0, 1, 2],
            "type": 1,
            "nsfw": false,
            "handler": 1
        }"#,
    )
    .unwrap();

    assert_eq!(create.name, "blep");
    assert_eq!(create.name_localizations.as_ref().unwrap()["fr"], "bêler");
    assert_eq!(create.default_member_permissions.unwrap().bits(), 2048);
    assert_eq!(create.options.as_ref().unwrap().len(), 1);
    assert_eq!(create.contexts.as_ref().unwrap()[2], InteractionContextType::PRIVATE_CHANNEL);

    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["type"], 1);
    assert_eq!(value["handler"], 1);
    assert!(value.get("id").is_none());
    assert!(value.get("application_id").is_none());
    assert!(value.get("version").is_none());

    let edit: EditGlobalApplicationCommandRequest =
        serde_json::from_str(r#"{"description": "Updated"}"#).unwrap();
    assert_eq!(edit.description.as_deref(), Some("Updated"));
    assert!(edit.name.is_none());
    assert_eq!(
        serde_json::to_value(&edit).unwrap(),
        serde_json::json!({"description": "Updated"})
    );

    let guild_create: CreateGuildApplicationCommandRequest =
        serde_json::from_str(r#"{"name": "stats", "description": "Guild stats"}"#).unwrap();
    assert_eq!(guild_create.name, "stats");
    assert_eq!(guild_create.description.as_deref(), Some("Guild stats"));

    let guild_edit: EditGuildApplicationCommandRequest =
        serde_json::from_str(r#"{"name": "stats2"}"#).unwrap();
    assert_eq!(guild_edit.name.as_deref(), Some("stats2"));

    let bulk = BulkOverwriteApplicationCommandsRequest {
        commands: vec![BulkOverwriteApplicationCommand {
            id: None,
            name: "blep".to_string(),
            name_localizations: None,
            description: Some("Send a random adorable animal photo".to_string()),
            description_localizations: None,
            options: None,
            default_member_permissions: None,
            dm_permission: None,
            default_permission: None,
            integration_types: None,
            contexts: None,
            kind: None,
            nsfw: None,
            handler: None,
        }],
    };
    let value = serde_json::to_value(bulk).unwrap();
    let commands = value.as_array().unwrap();
    assert_eq!(commands[0]["name"], "blep");
    assert!(commands[0].get("id").is_none());
    assert!(commands[0].get("version").is_none());

    let decoded: BulkOverwriteApplicationCommandsRequest =
        serde_json::from_value(value.clone()).unwrap();
    assert_eq!(decoded.commands.len(), 1);
}

/// Tests that localized command responses decode the localization-augmented
/// fields Discord adds to list endpoints.
#[test]
fn application_command_decodes_localized_fields() {
    let command: ApplicationCommand = serde_json::from_str(
        r#"{
            "id": "847194950382780532",
            "application_id": "775799577604522054",
            "name": "birthday",
            "name_localized": "生日",
            "description": "Wish a friend a happy birthday",
            "description_localized": "祝你朋友生日快乐",
            "options": [
                {
                    "type": 4,
                    "name": "age",
                    "name_localized": "岁数",
                    "description": "Your friend's age",
                    "description_localized": "你朋友的年龄"
                }
            ],
            "version": "847194950382780533"
        }"#,
    )
    .unwrap();

    assert_eq!(command.name_localized.as_deref(), Some("生日"));
    assert_eq!(command.description_localized.as_deref(), Some("祝你朋友生日快乐"));
    let option = &command.options.as_ref().unwrap()[0];
    assert_eq!(option.name_localized.as_deref(), Some("岁数"));
    assert_eq!(option.description_localized.as_deref(), Some("你朋友的年龄"));

    let choice: CommandOptionChoice =
        serde_json::from_str(r#"{"name": "Dog", "name_localized": "犬", "value": "animal_dog"}"#)
            .unwrap();
    assert_eq!(choice.name_localized.as_deref(), Some("犬"));

    let value = serde_json::to_value(&choice).unwrap();
    assert_eq!(value["name_localized"], "犬");
}

/// Tests the test entitlement request.
#[test]
fn create_test_entitlement_matches_documented_fields() {
    let request: CreateTestEntitlementRequest = serde_json::from_str(
        r#"{"sku_id": "1158857122189168803", "owner_id": "41771983429993000", "owner_type": 2}"#,
    )
    .unwrap();

    assert_eq!(request.sku_id.get(), 1_158_857_122_189_168_803);
    assert_eq!(request.owner_id, 41_771_983_429_993_000);
    assert_eq!(request.kind, EntitlementOwnerType::USER);

    let value = serde_json::to_value(request).unwrap();
    assert_eq!(value["sku_id"], "1158857122189168803");
    assert_eq!(value["owner_id"], "41771983429993000");
    assert_eq!(value["owner_type"], 2);
}

/// Tests the interaction followup request bodies.
#[test]
fn interaction_followup_requests_match_documented_fields() {
    let create: CreateFollowupMessageRequest = serde_json::from_str(
        r#"{
            "content": "followup",
            "tts": false,
            "embeds": [],
            "allowed_mentions": {"parse": []},
            "components": [],
            "attachments": [{"id": 0, "filename": "chart.png"}],
            "flags": 64,
            "thread_name": "Thread",
            "applied_tags": ["41771983423143937"],
            "poll": {
                "question": {"text": "Pick one"},
                "answers": [{"poll_media": {"text": "A"}}],
                "duration": 1
            }
        }"#,
    )
    .unwrap();

    assert_eq!(create.content.as_deref(), Some("followup"));
    assert_eq!(create.flags.unwrap().bits(), 64);
    assert_eq!(create.attachments.as_ref().unwrap()[0].filename.as_deref(), Some("chart.png"));
    assert_eq!(create.poll.as_ref().unwrap().answers[0].poll_media.text.as_deref(), Some("A"));

    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["content"], "followup");
    assert_eq!(value["attachments"][0]["id"], 0);
    assert_eq!(value["poll"]["duration"], 1);

    assert_eq!(
        serde_json::to_value(CreateFollowupMessageRequest {
            content: None,
            tts: None,
            embeds: None,
            allowed_mentions: None,
            components: None,
            attachments: None,
            flags: None,
            thread_name: None,
            applied_tags: None,
            poll: None,
        })
        .unwrap(),
        serde_json::json!({})
    );

    let edit: EditFollowupMessageRequest =
        serde_json::from_str(r#"{"content": "edited", "components": []}"#).unwrap();
    assert_eq!(edit.content.as_deref(), Some("edited"));
    assert!(edit.flags.is_none());
    assert_eq!(
        serde_json::to_value(&edit).unwrap(),
        serde_json::json!({"content": "edited", "components": []})
    );

    let poll = PollCreateRequest {
        question: PollMedia {
            text: Some("Pick one".to_string()),
            emoji: None,
        },
        answers: vec![PollAnswer {
            answer_id: Some(1),
            poll_media: PollMedia { text: Some("A".to_string()), emoji: None },
        }],
        duration: Some(1),
        allow_multiselect: None,
        layout_type: None,
    };
    assert_eq!(serde_json::to_value(&poll).unwrap()["answers"][0]["answer_id"], 1);
    let _ = UNIX_EPOCH + Duration::from_secs(0);
}
/// The `@everyone` role ID shared with the guild ID in these fixtures.
const EVERYONE_ROLE: Id<Role> = Id::new(41_771_983_429_993_937);

/// Returns the Unix timestamp identified by `seconds` and `nanos`.
fn request_timestamp(seconds: u64, nanos: u32) -> SystemTime {
    UNIX_EPOCH + Duration::new(seconds, nanos)
}

/// Returns a channel ID for use in request fixtures.
const fn request_channel_id(value: u64) -> Id<Channel> {
    Id::new(value)
}

/// Tests that `ModifyGuildRequest` round-trips every documented field.
#[test]
fn modify_guild_request_round_trips_every_field() {
    let request = ModifyGuildRequest {
        name: Some("Cordion Testers".to_string()),
        region: Some("us-west".to_string()),
        verification_level: Some(VerificationLevel::HIGH),
        default_message_notifications: Some(DefaultMessageNotificationLevel::ONLY_MENTIONS),
        explicit_content_filter: Some(ExplicitContentFilterLevel::ALL_MEMBERS),
        afk_channel_id: Some(request_channel_id(96_008_815_106_887_111)),
        afk_timeout: Some(300),
        icon: Some("data:image/png;base64,aW1hZ2UtZGF0YQ==".to_string()),
        splash: Some("data:image/png;base64,c3BsYXNo".to_string()),
        discovery_splash: Some("data:image/png;base64,ZGlzY292ZXJ5".to_string()),
        banner: Some("data:image/png;base64,YmFubmVy".to_string()),
        system_channel_id: Some(request_channel_id(96_008_815_106_887_112)),
        system_channel_flags: Some(SystemChannelFlags::SUPPRESS_JOIN_NOTIFICATIONS),
        rules_channel_id: Some(request_channel_id(96_008_815_106_887_113)),
        public_updates_channel_id: Some(request_channel_id(96_008_815_106_887_114)),
        preferred_locale: Some("en-US".to_string()),
        features: Some(vec![GuildFeature::ANIMATED_ICON]),
        description: Some("The official place to report Cordion bugs!".to_string()),
        premium_progress_bar_enabled: Some(true),
        safety_alerts_channel_id: Some(request_channel_id(96_008_815_106_887_115)),
    };

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["name"], "Cordion Testers");
    assert_eq!(value["region"], "us-west");
    assert_eq!(value["verification_level"], 3);
    assert_eq!(value["default_message_notifications"], 1);
    assert_eq!(value["explicit_content_filter"], 2);
    assert_eq!(value["afk_channel_id"], "96008815106887111");
    assert_eq!(value["afk_timeout"], 300);
    assert_eq!(value["icon"], "data:image/png;base64,aW1hZ2UtZGF0YQ==");
    assert_eq!(value["splash"], "data:image/png;base64,c3BsYXNo");
    assert_eq!(value["discovery_splash"], "data:image/png;base64,ZGlzY292ZXJ5");
    assert_eq!(value["banner"], "data:image/png;base64,YmFubmVy");
    assert_eq!(value["system_channel_id"], "96008815106887112");
    assert_eq!(value["system_channel_flags"], 1);
    assert_eq!(value["rules_channel_id"], "96008815106887113");
    assert_eq!(value["public_updates_channel_id"], "96008815106887114");
    assert_eq!(value["preferred_locale"], "en-US");
    assert_eq!(value["features"], json!(["ANIMATED_ICON"]));
    assert_eq!(value["description"], "The official place to report Cordion bugs!");
    assert_eq!(value["premium_progress_bar_enabled"], true);
    assert_eq!(value["safety_alerts_channel_id"], "96008815106887115");

    let decoded: ModifyGuildRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, request);
}

/// Tests that `ModifyGuildRequest` omits every `None` field and decodes `{}`.
#[test]
fn modify_guild_request_omits_absent_fields() {
    let request = ModifyGuildRequest {
        name: None,
        region: None,
        verification_level: None,
        default_message_notifications: None,
        explicit_content_filter: None,
        afk_channel_id: None,
        afk_timeout: None,
        icon: None,
        splash: None,
        discovery_splash: None,
        banner: None,
        system_channel_id: None,
        system_channel_flags: None,
        rules_channel_id: None,
        public_updates_channel_id: None,
        preferred_locale: None,
        features: None,
        description: None,
        premium_progress_bar_enabled: None,
        safety_alerts_channel_id: None,
    };

    assert_eq!(serde_json::to_value(&request).unwrap(), json!({}));

    let decoded: ModifyGuildRequest = serde_json::from_str("{}").unwrap();
    assert_eq!(decoded, request);
}

/// Tests that `CreateGuildRoleRequest` and `ModifyGuildRoleRequest` round-trip
/// the documented role fields, including image data and role colors.
#[test]
fn guild_role_requests_round_trip_documented_fields() {
    let create = CreateGuildRoleRequest {
        name: Some("new role".to_string()),
        permissions: Some(Permission::MANAGE_GUILD),
        color: Some(0x112_233),
        colors: Some(RoleColors {
            primary_color: 0x112_233,
            secondary_color: Some(0x445_566),
            tertiary_color: None,
        }),
        hoist: Some(true),
        icon: Some("data:image/png;base64,aW1hZ2UtZGF0YQ==".to_string()),
        unicode_emoji: Some("🔥".to_string()),
        mentionable: Some(true),
    };

    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["name"], "new role");
    assert_eq!(value["permissions"], "32");
    assert_eq!(value["color"], 0x112_233);
    assert_eq!(value["colors"]["primary_color"], 0x112_233);
    assert_eq!(value["colors"]["secondary_color"], 0x445_566);
    assert_eq!(value["colors"]["tertiary_color"], Value::Null);
    assert_eq!(value["hoist"], true);
    assert_eq!(value["icon"], "data:image/png;base64,aW1hZ2UtZGF0YQ==");
    assert_eq!(value["unicode_emoji"], "🔥");
    assert_eq!(value["mentionable"], true);

    let decoded: CreateGuildRoleRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, create);

    let empty = CreateGuildRoleRequest {
        name: None,
        permissions: None,
        color: None,
        colors: None,
        hoist: None,
        icon: None,
        unicode_emoji: None,
        mentionable: None,
    };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<CreateGuildRoleRequest>("{}").unwrap(), empty);

    let modify = ModifyGuildRoleRequest {
        name: Some("renamed".to_string()),
        permissions: Some(Permission::MANAGE_GUILD),
        color: None,
        colors: Some(RoleColors {
            primary_color: 0x010_203,
            secondary_color: None,
            tertiary_color: None,
        }),
        hoist: Some(false),
        icon: None,
        unicode_emoji: None,
        mentionable: Some(false),
    };

    let value = serde_json::to_value(&modify).unwrap();
    assert_eq!(value["name"], "renamed");
    assert_eq!(value["permissions"], "32");
    assert_eq!(value["colors"]["primary_color"], 0x010_203);
    assert_eq!(value["hoist"], false);
    assert_eq!(value["mentionable"], false);
    assert!(value.get("icon").is_none());
    assert!(value.get("unicode_emoji").is_none());
    assert!(value.get("color").is_none());

    let decoded: ModifyGuildRoleRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, modify);
}

/// Tests that `ModifyGuildRolePosition` round-trips a role ID and an optional
/// position, and that an absent position is omitted.
#[test]
fn modify_guild_role_position_round_trips_id_and_position() {
    let position: ModifyGuildRolePosition =
        serde_json::from_str(r#"{"id": "41771983429993937", "position": 3}"#).unwrap();
    assert_eq!(position.id, EVERYONE_ROLE);
    assert_eq!(position.position, Some(3));

    let value = serde_json::to_value(position).unwrap();
    assert_eq!(value, json!({"id": "41771983429993937", "position": 3}));

    let without_position = ModifyGuildRolePosition { id: EVERYONE_ROLE, position: None };
    assert_eq!(
        serde_json::to_value(without_position).unwrap(),
        json!({"id": "41771983429993937"})
    );
}

/// Tests that `ModifyGuildWidgetRequest` round-trips the guild widget settings
/// fields and omits absent ones.
#[test]
fn modify_guild_widget_request_round_trips_settings() {
    let request: ModifyGuildWidgetRequest =
        serde_json::from_str(r#"{"enabled": true, "channel_id": "41771983429993937"}"#).unwrap();
    assert_eq!(request.enabled, Some(true));
    assert_eq!(request.channel_id, Some(request_channel_id(41_771_983_429_993_937)));
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({"enabled": true, "channel_id": "41771983429993937"})
    );

    let empty = ModifyGuildWidgetRequest { enabled: None, channel_id: None };
    assert_eq!(serde_json::to_value(empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<ModifyGuildWidgetRequest>("{}").unwrap(), empty);
}

/// Tests that `ModifyGuildWelcomeScreenRequest` round-trips the documented
/// welcome screen fields and omits absent ones.
#[test]
fn modify_guild_welcome_screen_request_round_trips_fields() {
    let request: ModifyGuildWelcomeScreenRequest = serde_json::from_str(
        r#"{
            "enabled": true,
            "welcome_channels": [
                {
                    "channel_id": "41771983429993937",
                    "description": "Follow for official Discord API updates",
                    "emoji_id": null,
                    "emoji_name": "📡"
                }
            ],
            "description": "Discord Developers is a place to learn about Discord's API"
        }"#,
    )
    .unwrap();

    assert_eq!(request.enabled, Some(true));
    let channels = request.welcome_channels.as_ref().unwrap();
    assert_eq!(channels.len(), 1);
    assert_eq!(channels[0].description, "Follow for official Discord API updates");
    assert!(channels[0].emoji_id.is_none());
    assert_eq!(channels[0].emoji_name.as_deref(), Some("📡"));
    assert_eq!(
        request.description.as_deref(),
        Some("Discord Developers is a place to learn about Discord's API")
    );

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["enabled"], true);
    assert_eq!(value["welcome_channels"][0]["channel_id"], "41771983429993937");
    assert_eq!(value["welcome_channels"][0]["emoji_name"], "📡");
    assert_eq!(
        value["description"],
        "Discord Developers is a place to learn about Discord's API"
    );

    let empty = ModifyGuildWelcomeScreenRequest {
        enabled: None,
        welcome_channels: None,
        description: None,
    };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<ModifyGuildWelcomeScreenRequest>("{}").unwrap(), empty);
}

/// Tests that `ModifyGuildOnboardingRequest` round-trips prompts, default
/// channels, the enabled flag, and the onboarding mode.
#[test]
fn modify_guild_onboarding_request_round_trips_fields() {
    let request: ModifyGuildOnboardingRequest = serde_json::from_str(
        r#"{
            "prompts": [
                {
                    "id": "1067461047608422473",
                    "type": 0,
                    "options": [
                        {
                            "id": "1067461047608422476",
                            "channel_ids": ["962007075288916001"],
                            "role_ids": [],
                            "emoji_id": "1070002302032826408",
                            "emoji_name": "chat",
                            "emoji_animated": false,
                            "title": "Chat with Friends",
                            "description": ""
                        }
                    ],
                    "title": "What do you want to do in this community?",
                    "single_select": false,
                    "required": false,
                    "in_onboarding": true
                }
            ],
            "default_channel_ids": ["998678771706110023"],
            "enabled": true,
            "mode": 1
        }"#,
    )
    .unwrap();

    let prompts = request.prompts.as_ref().unwrap();
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0].title, "What do you want to do in this community?");
    let options = &prompts[0].options;
    assert_eq!(options.len(), 1);
    assert_eq!(options[0].title, "Chat with Friends");
    assert_eq!(options[0].channel_ids.len(), 1);
    assert_eq!(options[0].emoji_name.as_deref(), Some("chat"));
    assert_eq!(options[0].emoji_animated, Some(false));
    assert_eq!(request.default_channel_ids.as_ref().unwrap().len(), 1);
    assert_eq!(request.enabled, Some(true));
    assert_eq!(request.mode, Some(OnboardingMode::ONBOARDING_ADVANCED));

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["prompts"][0]["options"][0]["title"], "Chat with Friends");
    assert_eq!(value["default_channel_ids"][0], "998678771706110023");
    assert_eq!(value["enabled"], true);
    assert_eq!(value["mode"], 1);

    let empty = ModifyGuildOnboardingRequest {
        prompts: None,
        default_channel_ids: None,
        enabled: None,
        mode: None,
    };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<ModifyGuildOnboardingRequest>("{}").unwrap(), empty);
}

/// Tests that `ModifyGuildIncidentActionsRequest` always serializes both
/// nullable timestamps, emits `null` for `None`, and decodes absent fields.
#[test]
fn modify_guild_incident_actions_request_serializes_null_for_none() {
    let request = ModifyGuildIncidentActionsRequest {
        invites_disabled_until: Some(request_timestamp(1_693_579_682, 222_000_000)),
        dms_disabled_until: None,
    };

    let value = serde_json::to_value(request).unwrap();
    assert_eq!(value["invites_disabled_until"], "2023-09-01T14:48:02.222+00:00");
    assert_eq!(value["dms_disabled_until"], Value::Null);

    let decoded: ModifyGuildIncidentActionsRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, request);

    let empty: ModifyGuildIncidentActionsRequest = serde_json::from_str("{}").unwrap();
    assert_eq!(empty.invites_disabled_until, None);
    assert_eq!(empty.dms_disabled_until, None);
    assert_eq!(
        serde_json::to_value(empty).unwrap(),
        json!({"invites_disabled_until": null, "dms_disabled_until": null})
    );

    let disabled = ModifyGuildIncidentActionsRequest {
        invites_disabled_until: None,
        dms_disabled_until: None,
    };
    assert_eq!(
        serde_json::to_value(disabled).unwrap(),
        json!({"invites_disabled_until": null, "dms_disabled_until": null})
    );
}

/// Tests that the guild template request bodies round-trip their documented
/// fields and that absent fields are omitted.
#[test]
fn guild_template_requests_round_trip_fields() {
    let create = CreateGuildTemplateRequest {
        name: "Friends & Family".to_string(),
        description: Some("A template for friends and family".to_string()),
    };
    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["name"], "Friends & Family");
    assert_eq!(value["description"], "A template for friends and family");

    let decoded: CreateGuildTemplateRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, create);

    let without_description = CreateGuildTemplateRequest {
        name: "Friends & Family".to_string(),
        description: None,
    };
    assert_eq!(
        serde_json::to_value(&without_description).unwrap(),
        json!({"name": "Friends & Family"})
    );
    assert!(
        serde_json::from_str::<CreateGuildTemplateRequest>(r#"{"name": "Friends & Family"}"#)
            .is_ok()
    );
    assert!(serde_json::from_str::<CreateGuildTemplateRequest>("{}").is_err());

    let modify = ModifyGuildTemplateRequest {
        name: Some("Renamed".to_string()),
        description: None,
    };
    assert_eq!(serde_json::to_value(&modify).unwrap(), json!({"name": "Renamed"}));
    let decoded: ModifyGuildTemplateRequest =
        serde_json::from_value(serde_json::to_value(&modify).unwrap()).unwrap();
    assert_eq!(decoded, modify);

    let empty = ModifyGuildTemplateRequest { name: None, description: None };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<ModifyGuildTemplateRequest>("{}").unwrap(), empty);
}

/// Tests that `CreateGuildScheduledEventRequest` round-trips every documented
/// field, including image data, entity metadata, and the end time.
#[test]
fn create_guild_scheduled_event_request_round_trips_every_field() {
    let request = CreateGuildScheduledEventRequest {
        channel_id: Some(request_channel_id(96_008_815_106_887_111)),
        entity_metadata: None,
        name: "Cordion Release".to_string(),
        privacy_level: GuildScheduledEventPrivacyLevel::GUILD_ONLY,
        scheduled_start_time: request_timestamp(1_693_579_682, 0),
        scheduled_end_time: None,
        description: Some("The release of Cordion".to_string()),
        entity_type: GuildScheduledEventEntityType::VOICE,
        image: Some("data:image/png;base64,aW1hZ2UtZGF0YQ==".to_string()),
        recurrence_rule: None,
    };

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["channel_id"], "96008815106887111");
    assert_eq!(value["name"], "Cordion Release");
    assert_eq!(value["privacy_level"], 2);
    assert_eq!(value["scheduled_start_time"], "2023-09-01T14:48:02+00:00");
    assert_eq!(value["description"], "The release of Cordion");
    assert_eq!(value["entity_type"], 2);
    assert_eq!(value["image"], "data:image/png;base64,aW1hZ2UtZGF0YQ==");
    assert!(value.get("entity_metadata").is_none());
    assert!(value.get("scheduled_end_time").is_none());
    assert!(value.get("recurrence_rule").is_none());

    let decoded: CreateGuildScheduledEventRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, request);

    let external = CreateGuildScheduledEventRequest {
        channel_id: None,
        entity_metadata: Some(GuildScheduledEventEntityMetadata {
            location: Some("Discord Developers".to_string()),
        }),
        name: "External Event".to_string(),
        privacy_level: GuildScheduledEventPrivacyLevel::GUILD_ONLY,
        scheduled_start_time: request_timestamp(1_693_579_682, 0),
        scheduled_end_time: Some(request_timestamp(1_693_580_000, 0)),
        description: None,
        entity_type: GuildScheduledEventEntityType::EXTERNAL,
        image: None,
        recurrence_rule: None,
    };
    let value = serde_json::to_value(&external).unwrap();
    assert_eq!(value["entity_metadata"]["location"], "Discord Developers");
    assert_eq!(value["scheduled_end_time"], "2023-09-01T14:53:20+00:00");
    assert_eq!(value["entity_type"], 3);
    let decoded: CreateGuildScheduledEventRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, external);

    assert!(serde_json::from_str::<CreateGuildScheduledEventRequest>(r#"{"name": "x"}"#).is_err());
    assert!(
        serde_json::from_str::<CreateGuildScheduledEventRequest>(
            r#"{"channel_id": "1", "name": "x"}"#
        )
        .is_err()
    );
}

/// Tests that `ModifyGuildScheduledEventRequest` round-trips every optional
/// field, omits absent ones, and decodes `{}`.
#[test]
fn modify_guild_scheduled_event_request_round_trips_every_field() {
    let request = ModifyGuildScheduledEventRequest {
        channel_id: Some(request_channel_id(96_008_815_106_887_111)),
        entity_metadata: Some(GuildScheduledEventEntityMetadata {
            location: Some("Discord Developers".to_string()),
        }),
        name: Some("Renamed".to_string()),
        privacy_level: Some(GuildScheduledEventPrivacyLevel::GUILD_ONLY),
        scheduled_start_time: Some(request_timestamp(1_693_579_682, 0)),
        scheduled_end_time: Some(request_timestamp(1_693_580_000, 0)),
        description: Some("Renamed description".to_string()),
        entity_type: Some(GuildScheduledEventEntityType::EXTERNAL),
        status: Some(GuildScheduledEventStatus::ACTIVE),
        image: Some("data:image/png;base64,aW1hZ2UtZGF0YQ==".to_string()),
        recurrence_rule: None,
    };

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["channel_id"], "96008815106887111");
    assert_eq!(value["entity_metadata"]["location"], "Discord Developers");
    assert_eq!(value["name"], "Renamed");
    assert_eq!(value["privacy_level"], 2);
    assert_eq!(value["scheduled_start_time"], "2023-09-01T14:48:02+00:00");
    assert_eq!(value["scheduled_end_time"], "2023-09-01T14:53:20+00:00");
    assert_eq!(value["description"], "Renamed description");
    assert_eq!(value["entity_type"], 3);
    assert_eq!(value["status"], 2);
    assert_eq!(value["image"], "data:image/png;base64,aW1hZ2UtZGF0YQ==");
    assert!(value.get("recurrence_rule").is_none());

    let decoded: ModifyGuildScheduledEventRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, request);

    let empty = ModifyGuildScheduledEventRequest {
        channel_id: None,
        entity_metadata: None,
        name: None,
        privacy_level: None,
        scheduled_start_time: None,
        scheduled_end_time: None,
        description: None,
        entity_type: None,
        status: None,
        image: None,
        recurrence_rule: None,
    };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<ModifyGuildScheduledEventRequest>("{}").unwrap(), empty);
}

/// Tests that the Stage instance request bodies round-trip their documented
/// fields and that absent fields are omitted.
#[test]
fn stage_instance_requests_round_trip_fields() {
    let create = CreateStageInstanceRequest {
        channel_id: request_channel_id(733_488_538_393_510_049),
        topic: "Testing Testing, 123".to_string(),
        privacy_level: Some(StagePrivacyLevel::GUILD_ONLY),
        send_start_notification: Some(true),
        guild_scheduled_event_id: Some(GuildScheduledEventId::new(947_656_305_244_532_806)),
    };

    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["channel_id"], "733488538393510049");
    assert_eq!(value["topic"], "Testing Testing, 123");
    assert_eq!(value["privacy_level"], 2);
    assert_eq!(value["send_start_notification"], true);
    assert_eq!(value["guild_scheduled_event_id"], "947656305244532806");

    let decoded: CreateStageInstanceRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, create);

    assert!(serde_json::from_str::<CreateStageInstanceRequest>(r#"{"topic": "x"}"#).is_err());
    assert!(serde_json::from_str::<CreateStageInstanceRequest>(r#"{"channel_id": "1"}"#).is_err());

    let modify = ModifyStageInstanceRequest {
        topic: Some("Updated topic".to_string()),
        privacy_level: Some(StagePrivacyLevel::GUILD_ONLY),
    };
    assert_eq!(
        serde_json::to_value(&modify).unwrap(),
        json!({"topic": "Updated topic", "privacy_level": 2})
    );
    let decoded: ModifyStageInstanceRequest =
        serde_json::from_value(serde_json::to_value(&modify).unwrap()).unwrap();
    assert_eq!(decoded, modify);

    let empty = ModifyStageInstanceRequest { topic: None, privacy_level: None };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<ModifyStageInstanceRequest>("{}").unwrap(), empty);
}

/// Tests that the soundboard request bodies round-trip their documented fields,
/// including the base64 sound data URI and the floating-point volume.
#[test]
fn soundboard_requests_round_trip_fields() {
    let create = CreateGuildSoundboardSoundRequest {
        name: "quack".to_string(),
        sound: "data:audio/mpeg;base64,c291bmQtZGF0YQ==".to_string(),
        volume: Some(1.0),
        emoji_id: Some(EmojiId::new(989_193_655_938_064_464)),
        emoji_name: None,
    };

    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["name"], "quack");
    assert_eq!(value["sound"], "data:audio/mpeg;base64,c291bmQtZGF0YQ==");
    assert_eq!(value["volume"], 1.0);
    assert_eq!(value["emoji_id"], "989193655938064464");
    assert!(value.get("emoji_name").is_none());

    let decoded: CreateGuildSoundboardSoundRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, create);

    let minimal = CreateGuildSoundboardSoundRequest {
        name: "quack".to_string(),
        sound: "data:audio/mpeg;base64,c291bmQtZGF0YQ==".to_string(),
        volume: None,
        emoji_id: None,
        emoji_name: Some("🦆".to_string()),
    };
    assert_eq!(
        serde_json::to_value(&minimal).unwrap(),
        json!({
            "name": "quack",
            "sound": "data:audio/mpeg;base64,c291bmQtZGF0YQ==",
            "emoji_name": "🦆"
        })
    );
    assert!(
        serde_json::from_str::<CreateGuildSoundboardSoundRequest>(r#"{"name": "quack"}"#).is_err()
    );

    let modify = ModifyGuildSoundboardSoundRequest {
        name: Some("Yay".to_string()),
        volume: Some(0.5),
        emoji_id: None,
        emoji_name: None,
    };
    let value = serde_json::to_value(&modify).unwrap();
    assert_eq!(value["name"], "Yay");
    assert_eq!(value["volume"], 0.5);
    assert!(value.get("emoji_id").is_none());
    let decoded: ModifyGuildSoundboardSoundRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, modify);

    let empty = ModifyGuildSoundboardSoundRequest {
        name: None,
        volume: None,
        emoji_id: None,
        emoji_name: None,
    };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));

    let send = SendSoundboardSoundRequest {
        sound_id: SoundboardSoundId::new(1_106_714_396_018_884_649),
        source_guild_id: Some(GuildId::new(613_425_648_685_547_541)),
    };
    let value = serde_json::to_value(send).unwrap();
    assert_eq!(value["sound_id"], "1106714396018884649");
    assert_eq!(value["source_guild_id"], "613425648685547541");
    let decoded: SendSoundboardSoundRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, send);

    let local = SendSoundboardSoundRequest {
        sound_id: SoundboardSoundId::new(1),
        source_guild_id: None,
    };
    assert_eq!(serde_json::to_value(local).unwrap(), json!({"sound_id": "1"}));
    assert!(serde_json::from_str::<SendSoundboardSoundRequest>("{}").is_err());
}

/// Tests that the emoji request bodies round-trip their documented fields,
/// including the base64 128x128 image data.
#[test]
fn emoji_requests_round_trip_fields() {
    let create = CreateGuildEmojiRequest {
        name: "LUL".to_string(),
        image: "data:image/png;base64,aW1hZ2UtZGF0YQ==".to_string(),
        roles: Some(vec![EVERYONE_ROLE]),
    };
    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["name"], "LUL");
    assert_eq!(value["image"], "data:image/png;base64,aW1hZ2UtZGF0YQ==");
    assert_eq!(value["roles"], json!(["41771983429993937"]));
    let decoded: CreateGuildEmojiRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, create);

    let without_roles = CreateGuildEmojiRequest {
        name: "LUL".to_string(),
        image: "data:image/png;base64,aW1hZ2UtZGF0YQ==".to_string(),
        roles: None,
    };
    assert_eq!(
        serde_json::to_value(&without_roles).unwrap(),
        json!({"name": "LUL", "image": "data:image/png;base64,aW1hZ2UtZGF0YQ=="})
    );
    assert!(serde_json::from_str::<CreateGuildEmojiRequest>(r#"{"name": "LUL"}"#).is_err());

    let modify = ModifyGuildEmojiRequest {
        name: Some("LOL".to_string()),
        roles: None,
    };
    let value = serde_json::to_value(&modify).unwrap();
    assert_eq!(value["name"], "LOL");
    assert!(value.get("roles").is_none());
    let decoded: ModifyGuildEmojiRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, modify);

    let modify_roles = ModifyGuildEmojiRequest { name: None, roles: Some(vec![]) };
    assert_eq!(serde_json::to_value(&modify_roles).unwrap(), json!({"roles": []}));

    let empty = ModifyGuildEmojiRequest { name: None, roles: None };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));

    let create_app = CreateApplicationEmojiRequest {
        name: "LUL".to_string(),
        image: "data:image/png;base64,aW1hZ2UtZGF0YQ==".to_string(),
    };
    let value = serde_json::to_value(&create_app).unwrap();
    assert_eq!(value["name"], "LUL");
    assert_eq!(value["image"], "data:image/png;base64,aW1hZ2UtZGF0YQ==");
    let decoded: CreateApplicationEmojiRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, create_app);
    assert!(
        serde_json::from_str::<CreateApplicationEmojiRequest>(
            r#"{"name": "LUL", "image": "data:image/png;base64,"}"#
        )
        .is_ok()
    );

    let modify_app = ModifyApplicationEmojiRequest { name: Some("LOL".to_string()) };
    assert_eq!(serde_json::to_value(&modify_app).unwrap(), json!({"name": "LOL"}));
    assert_eq!(
        serde_json::from_str::<ModifyApplicationEmojiRequest>(r#"{"name": "LOL"}"#).unwrap(),
        modify_app
    );

    let empty_app = ModifyApplicationEmojiRequest { name: None };
    assert_eq!(serde_json::to_value(&empty_app).unwrap(), json!({}));
}

/// Tests that the Auto Moderation rule request bodies round-trip their
/// documented fields, including trigger metadata and actions.
#[test]
fn auto_moderation_rule_requests_round_trip_fields() {
    let create = CreateAutoModerationRuleRequest {
        name: "Keyword Filter 1".to_string(),
        event_type: AutoModerationEventType::MESSAGE_SEND,
        trigger_type: AutoModerationTriggerType::KEYWORD,
        trigger_metadata: Some(AutoModerationTriggerMetadata {
            keyword_filter: Some(vec!["cat*".to_string(), "*dog".to_string()]),
            regex_patterns: Some(vec!["(b|c)at".to_string()]),
            presets: None,
            allow_list: None,
            mention_total_limit: None,
            mention_raid_protection_enabled: None,
        }),
        actions: vec![
            AutoModerationAction {
                kind: AutoModerationActionType::BLOCK_MESSAGE,
                metadata: Some(AutoModerationActionMetadata {
                    custom_message: Some(
                        "Please keep financial discussions limited to the #finance channel"
                            .to_string(),
                    ),
                    ..Default::default()
                }),
            },
            AutoModerationAction {
                kind: AutoModerationActionType::SEND_ALERT_MESSAGE,
                metadata: Some(AutoModerationActionMetadata {
                    channel_id: Some(request_channel_id(123_456_789_123_456_789)),
                    ..Default::default()
                }),
            },
        ],
        enabled: Some(true),
        exempt_roles: Some(vec![EVERYONE_ROLE]),
        exempt_channels: Some(vec![request_channel_id(523_456_789_123_456_789)]),
    };

    let value = serde_json::to_value(&create).unwrap();
    assert_eq!(value["name"], "Keyword Filter 1");
    assert_eq!(value["event_type"], 1);
    assert_eq!(value["trigger_type"], 1);
    assert_eq!(value["trigger_metadata"]["keyword_filter"], json!(["cat*", "*dog"]));
    assert_eq!(value["trigger_metadata"]["regex_patterns"], json!(["(b|c)at"]));
    assert_eq!(value["actions"][0]["type"], 1);
    assert_eq!(
        value["actions"][0]["metadata"]["custom_message"],
        "Please keep financial discussions limited to the #finance channel"
    );
    assert_eq!(value["actions"][1]["type"], 2);
    assert_eq!(value["actions"][1]["metadata"]["channel_id"], "123456789123456789");
    assert_eq!(value["enabled"], true);
    assert_eq!(value["exempt_roles"], json!(["41771983429993937"]));
    assert_eq!(value["exempt_channels"], json!(["523456789123456789"]));

    let decoded: CreateAutoModerationRuleRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, create);

    let minimal = CreateAutoModerationRuleRequest {
        name: "Spam Filter".to_string(),
        event_type: AutoModerationEventType::MESSAGE_SEND,
        trigger_type: AutoModerationTriggerType::SPAM,
        trigger_metadata: None,
        actions: vec![],
        enabled: None,
        exempt_roles: None,
        exempt_channels: None,
    };
    assert_eq!(
        serde_json::to_value(&minimal).unwrap(),
        json!({
            "name": "Spam Filter",
            "event_type": 1,
            "trigger_type": 3,
            "actions": []
        })
    );
    assert!(
        serde_json::from_str::<CreateAutoModerationRuleRequest>(
            r#"{"event_type": 1, "trigger_type": 3, "actions": []}"#
        )
        .is_err()
    );

    let preset = CreateAutoModerationRuleRequest {
        name: "Preset Filter".to_string(),
        event_type: AutoModerationEventType::MESSAGE_SEND,
        trigger_type: AutoModerationTriggerType::KEYWORD_PRESET,
        trigger_metadata: Some(AutoModerationTriggerMetadata {
            presets: Some(vec![KeywordPresetType::PROFANITY, KeywordPresetType::SLURS]),
            ..Default::default()
        }),
        actions: vec![],
        enabled: None,
        exempt_roles: None,
        exempt_channels: None,
    };
    let value = serde_json::to_value(&preset).unwrap();
    assert_eq!(value["trigger_metadata"]["presets"], json!([1, 3]));
    assert!(value["trigger_metadata"].get("keyword_filter").is_none());

    let modify = ModifyAutoModerationRuleRequest {
        name: Some("Renamed Filter".to_string()),
        event_type: Some(AutoModerationEventType::MEMBER_UPDATE),
        trigger_metadata: None,
        actions: None,
        enabled: Some(false),
        exempt_roles: None,
        exempt_channels: None,
    };
    let value = serde_json::to_value(&modify).unwrap();
    assert_eq!(value["name"], "Renamed Filter");
    assert_eq!(value["event_type"], 2);
    assert_eq!(value["enabled"], false);
    assert!(value.get("trigger_type").is_none());
    let decoded: ModifyAutoModerationRuleRequest = serde_json::from_value(value).unwrap();
    assert_eq!(decoded, modify);

    let empty = ModifyAutoModerationRuleRequest {
        name: None,
        event_type: None,
        trigger_metadata: None,
        actions: None,
        enabled: None,
        exempt_roles: None,
        exempt_channels: None,
    };
    assert_eq!(serde_json::to_value(&empty).unwrap(), json!({}));
    assert_eq!(serde_json::from_str::<ModifyAutoModerationRuleRequest>("{}").unwrap(), empty);
}

/// Tests that request bodies keep unrelated types untouched and that the
/// emoji and user fixtures used by these tests still round-trip.
#[test]
fn referenced_models_round_trip() {
    let emoji: Emoji = serde_json::from_str(
        r#"{
            "id": "41771983429993937",
            "name": "LUL",
            "roles": ["41771983429993000"],
            "require_colons": true,
            "managed": false,
            "animated": false,
            "available": true
        }"#,
    )
    .unwrap();
    assert_eq!(emoji.id, Some(EmojiId::new(41_771_983_429_993_937)));
    assert_eq!(serde_json::to_value(&emoji).unwrap()["roles"], json!(["41771983429993000"]));

    let user: User = serde_json::from_str(
        r#"{
            "id": "96008815106887111",
            "username": "Nelly",
            "discriminator": "0001",
            "avatar": null,
            "flags": 0
        }"#,
    )
    .unwrap();
    assert_eq!(user.id, UserId::new(96_008_815_106_887_111));
    assert_eq!(serde_json::to_value(user.id).unwrap(), json!("96008815106887111"));
}

use cordion_model::application_command::{
    BulkOverwriteApplicationCommand, BulkOverwriteApplicationCommandsRequest, CommandOptionChoice,
    CreateGlobalApplicationCommandRequest, CreateGuildApplicationCommandRequest,
    EditGlobalApplicationCommandRequest, EditGuildApplicationCommandRequest,
};
use cordion_model::auto_moderation::{
    AutoModerationAction, AutoModerationActionMetadata, AutoModerationTriggerMetadata,
    CreateAutoModerationRuleRequest, ModifyAutoModerationRuleRequest,
};
use cordion_model::channel::{
    ChannelId, CreateGuildChannelRequest, DefaultReaction, EditChannelPermissionsRequest,
    ForumAndMediaThreadMessageParams, ForumTag, ForumTagId, ModifyGroupDmChannelRequest,
    ModifyGuildChannelRequest, ModifyThreadChannelRequest, Overwrite, OverwriteId, OverwriteType,
    SortOrderMode, StartThreadFromMessageRequest, StartThreadInForumOrMediaThreadRequest,
    StartThreadWithoutMessageRequest, VideoQualityMode,
};
use cordion_model::emoji::{
    CreateApplicationEmojiRequest, CreateGuildEmojiRequest, EmojiId, ModifyApplicationEmojiRequest,
    ModifyGuildEmojiRequest,
};
use cordion_model::guild::{
    CreateGuildRoleRequest, CreateGuildScheduledEventRequest, CreateGuildSoundboardSoundRequest,
    CreateGuildTemplateRequest, CreateStageInstanceRequest, GuildId,
    GuildScheduledEventEntityMetadata, GuildScheduledEventId, ModifyGuildIncidentActionsRequest,
    ModifyGuildOnboardingRequest, ModifyGuildRequest, ModifyGuildRolePosition,
    ModifyGuildRoleRequest, ModifyGuildScheduledEventRequest, ModifyGuildSoundboardSoundRequest,
    ModifyGuildTemplateRequest, ModifyGuildWelcomeScreenRequest, ModifyGuildWidgetRequest,
    ModifyStageInstanceRequest, SendSoundboardSoundRequest, SoundboardSoundId,
};
use cordion_model::interaction::{CreateFollowupMessageRequest, EditFollowupMessageRequest};
use cordion_model::invite::{
    BulkAddInviteTargetUsersRequest, BulkDeleteInviteTargetUsersRequest,
    CreateChannelInviteRequest, TargetUsersJobStatus, TargetUsersJobStatusCode,
};
use cordion_model::message::{
    AllowedMentionType, AllowedMentions, AttachmentId, AttachmentRequest, AttachmentRequestId,
    BaseTheme, BulkDeleteMessagesRequest, CreateMessageRequest, EditMessageRequest, Embed,
    MessageId, MessageNonce, MessageReference, PollAnswer, PollCreateRequest, PollMedia,
    SharedClientTheme,
};
use cordion_model::monetization::entitlement::{
    CreateTestEntitlementRequest, EntitlementOwnerType,
};
use cordion_model::sticker::StickerId;
use cordion_model::user::UserId;
use cordion_model::webhook::{
    EditWebhookMessageRequest, ExecuteWebhookRequest, ModifyWebhookRequest,
    ModifyWebhookWithTokenRequest,
};
use serde_json::json;

/// Tests the edit current application request body.
#[test]
fn edit_current_application_request_matches_documented_fields() {
    let request: EditCurrentApplicationRequest = serde_json::from_str(
        r#"{
            "custom_install_url": "https://example.com/install",
            "description": "Test app",
            "role_connections_verification_url": "https://example.com/verify",
            "install_params": {
                "scopes": ["bot", "applications.commands"],
                "permissions": "2048"
            },
            "integration_types_config": {
                "0": {
                    "oauth2_install_params": {
                        "scopes": ["bot"],
                        "permissions": "2048"
                    }
                }
            },
            "flags": 8192,
            "icon": "data:image/png;base64,BASE64",
            "cover_image": "data:image/png;base64,BASE64",
            "interactions_endpoint_url": "https://example.com/interactions",
            "tags": ["utility", "fun"],
            "event_webhooks_url": "https://example.com/events",
            "event_webhooks_status": 2,
            "event_webhooks_types": ["APPLICATION_AUTHORIZED"]
        }"#,
    )
    .unwrap();

    assert_eq!(request.custom_install_url.as_deref(), Some("https://example.com/install"));
    assert_eq!(request.description.as_deref(), Some("Test app"));
    assert_eq!(
        request.role_connections_verification_url.as_deref(),
        Some("https://example.com/verify")
    );
    assert_eq!(request.install_params.as_ref().unwrap().scopes[0], "bot");
    assert_eq!(request.integration_types_config.as_ref().unwrap().len(), 1);
    assert_eq!(request.flags, Some(ApplicationFlags::GATEWAY_PRESENCE_LIMITED));
    assert_eq!(request.tags.as_ref().unwrap().len(), 2);
    assert_eq!(request.event_webhooks_status, Some(ApplicationEventWebhookStatus::ENABLED));

    let value = serde_json::to_value(&request).unwrap();
    assert_eq!(value["flags"], 8192);
    assert_eq!(value["install_params"]["permissions"], "2048");
    assert_eq!(
        value["integration_types_config"]["0"]["oauth2_install_params"]["scopes"][0],
        "bot"
    );
    assert_eq!(value["event_webhooks_status"], 2);
    assert_eq!(value["event_webhooks_types"][0], "APPLICATION_AUTHORIZED");

    assert_eq!(
        serde_json::to_value(EditCurrentApplicationRequest {
            custom_install_url: None,
            description: None,
            role_connections_verification_url: None,
            install_params: None,
            integration_types_config: None,
            flags: None,
            icon: None,
            cover_image: None,
            interactions_endpoint_url: None,
            tags: None,
            event_webhooks_url: None,
            event_webhooks_status: None,
            event_webhooks_types: None,
        })
        .unwrap(),
        serde_json::json!({})
    );
}
