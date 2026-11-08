/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for the application, application command, and team models.

#![allow(clippy::unwrap_used)]

use cordion_model::application::team::{Team, TeamMember, TeamMemberRole, TeamMembershipState};
use cordion_model::application::{
    ActivityInstance, ActivityLocationKind, Application, ApplicationEventWebhookStatus,
    ApplicationFlags, ApplicationIntegrationType, ApplicationRoleConnectionMetadata,
    ApplicationRoleConnectionMetadataType, EditCurrentApplicationRequest, IntegrationTypeConfig,
};
use cordion_model::application_command::{
    ApplicationCommand, ApplicationCommandHandler, ApplicationCommandOptionType,
    ApplicationCommandPermissionType, ApplicationCommandType, BulkOverwriteApplicationCommand,
    BulkOverwriteApplicationCommandsRequest, CommandOptionChoice, CommandOptionChoiceValue,
    CommandOptionNumericValue, CreateGlobalApplicationCommandRequest,
    CreateGuildApplicationCommandRequest, EditGlobalApplicationCommandRequest,
    EditGuildApplicationCommandRequest, GuildApplicationCommandPermissions,
};
use cordion_model::channel::ChannelType;
use cordion_model::interaction::InteractionContextType;
use cordion_model::user::UserFlags;

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
