/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for Discord Social SDK lobbies and application identity profiles.

#![allow(clippy::unwrap_used)]

use cordion_model::social::identity_profile::{
    ApplicationIdentityList, ApplicationIdentityProfile, DynamicField, DynamicFieldType,
    DynamicFieldValue, Media, ProfileData, ProviderType, UpdateApplicationIdentityProfileRequest,
};
use cordion_model::social::lobby::{
    BulkLobbyMemberRequest, CreateLobbyRequest, CreateOrJoinLobbyRequest, LinkLobbyChannelRequest,
    Lobby, LobbyInvite, LobbyMemberFlags, LobbyMessage, ModifyLobbyRequest,
    SendLobbyMessageRequest, UpsertLobbyMemberRequest,
};

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
