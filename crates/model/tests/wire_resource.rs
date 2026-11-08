/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for the user, role, emoji, sticker, voice, invite, webhook, Auto Moderation, and audit log models.

#![allow(clippy::unwrap_used)]

mod wire_common;
use std::time::{Duration, UNIX_EPOCH};

use cordion_model::audit_log::{AuditLog, AuditLogEvent};
use cordion_model::auto_moderation::{
    AutoModerationAction, AutoModerationActionMetadata, AutoModerationActionType,
    AutoModerationEventType, AutoModerationRule, AutoModerationTriggerMetadata,
    AutoModerationTriggerType, CreateAutoModerationRuleRequest, KeywordPresetType,
    ModifyAutoModerationRuleRequest,
};
use cordion_model::channel::{Channel, ChannelType};
use cordion_model::emoji::{
    CreateApplicationEmojiRequest, CreateGuildEmojiRequest, Emoji, ModifyApplicationEmojiRequest,
    ModifyGuildEmojiRequest,
};
use cordion_model::event::gateway::{AutoModerationActionExecution, InviteCreate};
use cordion_model::guild::{Guild, GuildAgeRestrictionLevel, GuildFeature, VerificationLevel};
use cordion_model::id::Id;
use cordion_model::invite::{
    BulkAddInviteTargetUsersRequest, BulkDeleteInviteTargetUsersRequest,
    CreateChannelInviteRequest, GuildInviteFlags, Invite, InviteTargetType, InviteType,
    InviteWithMetadata, PartialInvite, TargetUsersJobStatus, TargetUsersJobStatusCode,
};
use cordion_model::permission::Permission;
use cordion_model::role::{Role, RoleColors, RoleFlags, RoleTags};
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
use cordion_model::webhook::{
    EditWebhookMessageRequest, ModifyWebhookRequest, ModifyWebhookWithTokenRequest, Webhook,
    WebhookType,
};
use serde_json::{Value, json};
use wire_common::*;

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

/// Tests that unknown premium types are preserved.
#[test]
fn premium_type_preserves_unknown_value() {
    assert_eq!(serde_json::from_str::<PremiumType>("2").unwrap(), PremiumType::NITRO);

    let unknown: PremiumType = serde_json::from_str("9").unwrap();
    assert_eq!(serde_json::to_value(unknown).unwrap(), serde_json::json!(9));
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
