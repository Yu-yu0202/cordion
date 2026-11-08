/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for the guild model and its nested resources.

#![allow(clippy::unwrap_used)]

mod wire_common;
use std::time::{Duration, UNIX_EPOCH};

use cordion_model::application::ApplicationGuild;
use cordion_model::channel::{Channel, ChannelType};
use cordion_model::emoji::{Emoji, EmojiId};
use cordion_model::event::presence::UserStatus;
use cordion_model::guild::{
    AddGuildMemberRequest, Ban, BeginGuildPruneRequest, BulkGuildBanRequest, BulkGuildBanResponse,
    CreateGuildBanRequest, CreateGuildRoleRequest, CreateGuildScheduledEventRequest,
    CreateGuildSoundboardSoundRequest, CreateGuildTemplateRequest, CreateStageInstanceRequest,
    DefaultMessageNotificationLevel, ExplicitContentFilterLevel, Guild, GuildAgeRestrictionLevel,
    GuildFeature, GuildId, GuildMember, GuildMemberFlags, GuildOnboarding, GuildPreview,
    GuildPruneCount, GuildScheduledEvent, GuildScheduledEventEntityMetadata,
    GuildScheduledEventEntityType, GuildScheduledEventId, GuildScheduledEventPrivacyLevel,
    GuildScheduledEventRecurrenceFrequency, GuildScheduledEventRecurrenceWeekday,
    GuildScheduledEventStatus, GuildScheduledEventUser, GuildTemplate, GuildWidget,
    GuildWidgetSettings, IncidentsData, Integration, IntegrationAccount, IntegrationExpireBehavior,
    IntegrationType, ListGuildThreadsResponse, MfaLevel, ModifyCurrentMemberRequest,
    ModifyGuildChannelPosition, ModifyGuildIncidentActionsRequest, ModifyGuildMemberRequest,
    ModifyGuildOnboardingRequest, ModifyGuildRequest, ModifyGuildRolePosition,
    ModifyGuildRoleRequest, ModifyGuildScheduledEventRequest, ModifyGuildSoundboardSoundRequest,
    ModifyGuildTemplateRequest, ModifyGuildWelcomeScreenRequest, ModifyGuildWidgetRequest,
    ModifyStageInstanceRequest, OnboardingMode, OnboardingPromptOption, OnboardingPromptType,
    PartialGuild, PartialIntegration, PremiumTier, SendSoundboardSoundRequest, SoundboardSound,
    SoundboardSoundId, StageInstance, StagePrivacyLevel, SystemChannelFlags, VerificationLevel,
};
use cordion_model::id::Id;
use cordion_model::permission::Permission;
use cordion_model::role::RoleColors;
use cordion_model::user::{User, UserId};
use serde_json::{Value, json};
use wire_common::*;

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
