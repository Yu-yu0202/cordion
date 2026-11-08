/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for message components.

#![allow(clippy::unwrap_used)]

use cordion_model::channel::ChannelType;
use cordion_model::component::{Component, ComponentResponse, ComponentType, SelectDefaultValue};
use cordion_model::interaction::{InteractionCallbackData, InteractionResponse};

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
