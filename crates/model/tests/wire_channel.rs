/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for the channel model and its nested objects.

#![allow(clippy::unwrap_used)]

mod wire_common;
use cordion_model::channel::{
    Channel, ChannelFlags, ChannelId, ChannelType, CreateGuildChannelRequest, DefaultReaction,
    EditChannelPermissionsRequest, ForumAndMediaThreadMessageParams, ForumLayout, ForumTag,
    ForumTagId, ModifyGroupDmChannelRequest, ModifyGuildChannelRequest, ModifyThreadChannelRequest,
    Overwrite, OverwriteId, OverwriteType, SortOrderMode, StartThreadFromMessageRequest,
    StartThreadInForumOrMediaThreadRequest, StartThreadWithoutMessageRequest, ThreadMetadata,
    VideoQualityMode,
};
use cordion_model::emoji::EmojiId;
use cordion_model::permission::Permission;
use serde_json::json;
use wire_common::*;

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
