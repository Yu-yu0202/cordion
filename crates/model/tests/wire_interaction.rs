/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for interactions, responses, and followups.

#![allow(clippy::unwrap_used)]

mod wire_common;
use cordion_model::application_command::{ApplicationCommandOptionType, ApplicationCommandType};
use cordion_model::component::{ComponentResponse, ComponentType};
use cordion_model::id::Id;
use cordion_model::interaction::{
    AuthorizingIntegrationOwners, CommandOptionValue, GuildInstallOwner, Interaction,
    InteractionCallbackData, InteractionCallbackResponse, InteractionCallbackType,
    InteractionContextType, InteractionData, InteractionResponse, InteractionType,
    MessageInteraction, MessageInteractionMetadata,
};
use cordion_model::message::MessageFlags;
use serde_json::Value;
use wire_common::*;

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
