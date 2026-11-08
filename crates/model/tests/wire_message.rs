/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for the message model and its nested objects.

#![allow(clippy::unwrap_used)]

mod wire_common;
use std::time::{Duration, UNIX_EPOCH};

use cordion_model::channel::{
    ChannelId, ChannelType, ForumTagId, StartThreadInForumOrMediaThreadRequest,
};
use cordion_model::component::{ButtonStyle, Component};
use cordion_model::event::webhook_event::{WebhookEventBodyData, WebhookEventPayload};
use cordion_model::guild::GuildId;
use cordion_model::interaction::{
    CreateFollowupMessageRequest, EditFollowupMessageRequest, InteractionCallbackData,
    InteractionResponse, InteractionType, MessageInteractionMetadata,
};
use cordion_model::message::{
    AllowedMentionType, AllowedMentions, AttachmentId, AttachmentRequest, AttachmentRequestId,
    BaseTheme, BulkDeleteMessagesRequest, CreateMessageRequest, EditMessageRequest, Embed,
    GetChannelPinsResponse, Message, MessageFlags, MessageId, MessageNonce, MessagePin,
    MessageReference, MessageReferenceType, MessageType, PollAnswer, PollCreateRequest,
    PollLayoutType, PollMedia, SharedClientTheme,
};
use cordion_model::sticker::StickerId;
use cordion_model::webhook::ExecuteWebhookRequest;
use serde_json::{Value, json};
use wire_common::*;

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
