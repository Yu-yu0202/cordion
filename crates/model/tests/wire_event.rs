/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for Gateway payloads, webhook events, and presence data.

#![allow(clippy::unwrap_used)]

mod wire_common;
use std::time::Duration;

use cordion_model::application::{ApplicationFlags, ApplicationIntegrationType};
use cordion_model::channel::{Channel, ChannelType};
use cordion_model::event::gateway::{
    ChannelInfo, ChannelInfoChannel, ChannelPinsUpdate, GatewayEventName, GatewayOpcode,
    GatewayPayload, GatewayPresenceUpdate, GatewayVoiceStateUpdate, GetGatewayBotResponse,
    GetGatewayResponse, GuildAuditLogEntryCreate, GuildBanAdd, GuildBanRemove, GuildCreate,
    GuildEmojisUpdate, GuildIntegrationsUpdate, GuildMemberAdd, GuildMemberRemove,
    GuildMemberUpdate, GuildMembersChunk, GuildRoleCreate, GuildRoleDelete, GuildRoleUpdate,
    GuildScheduledEventUserAdd, GuildScheduledEventUserRemove, GuildSoundboardSoundDelete,
    GuildSoundboardSoundsUpdate, GuildStickersUpdate, Hello, Identify,
    IdentifyConnectionProperties, IntegrationDelete, InvalidSession, InviteDelete, MessageDelete,
    MessageDeleteBulk, MessagePollVoteAdd, MessagePollVoteRemove, MessageReactionAdd,
    MessageReactionRemove, MessageReactionRemoveAll, MessageReactionRemoveEmoji, RateLimited,
    ReactionType, Ready, ReadyApplication, RequestChannelInfo, RequestGuildMemberRateLimitMetadata,
    RequestGuildMembers, RequestGuildMembersUserIds, RequestSoundboardSounds, Resume,
    SessionStartLimit, SoundboardSounds, ThreadListSync, ThreadMemberUpdate, ThreadMembersUpdate,
    TypingStart, VoiceChannelEffectSend, VoiceChannelStartTimeUpdate, VoiceChannelStatusUpdate,
    VoiceEffectAnimationType, VoiceServerUpdate, WebhooksUpdate,
};
use cordion_model::event::presence::{ActivityFlags, ActivityType, Presence, UserStatus};
use cordion_model::event::webhook_event::{
    WebhookEventBodyData, WebhookEventBodyType, WebhookEventPayload, WebhookEventType,
};
use cordion_model::message::MessageType;
use cordion_model::monetization::entitlement::EntitlementType;
use cordion_model::sticker::StickerType;
use wire_common::*;

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
