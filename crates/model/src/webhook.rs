/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord webhook resource.
//!
//! See [Discord's webhook resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/webhook

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::channel::{ChannelId, PartialChannel};
use crate::component::Component;
use crate::guild::GuildId;
use crate::id::{Id, Identifiable};
use crate::message::{AllowedMentions, AttachmentRequest, Embed, MessageFlags, PollCreateRequest};
use crate::user::User;

/// The ID of a [`Webhook`].
pub type WebhookId = Id<Webhook>;

/// The kind of a [`Webhook`].
///
/// Unrecognized values are preserved. These values do not include outgoing
/// webhook events sent to apps by Discord. See [Discord's webhook types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/resources/webhook#webhook-object-webhook-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct WebhookType(u8);

impl WebhookType {
    /// A webhook that posts messages to a channel using a generated token.
    pub const INCOMING: Self = Self(1);

    /// An internal webhook used with Channel Following.
    pub const CHANNEL_FOLLOWER: Self = Self(2);

    /// A webhook used with interactions.
    pub const APPLICATION: Self = Self(3);
}

/// The partial guild embedded in a channel follower webhook.
///
/// Discord includes only `id`, `name`, and `icon` in the `source_guild` object.
/// The field is absent when the webhook creator has lost access to the guild
/// that contains the followed channel.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct WebhookSourceGuild {
    /// The guild's ID.
    pub id: GuildId,

    /// The guild's name.
    pub name: String,

    /// The guild's icon hash, or `None` if it has no icon.
    pub icon: Option<String>,
}

/// A Discord webhook.
///
/// The field availability depends on the webhook type and on how the webhook
/// was fetched. `user` is not returned when fetching a webhook with its token;
/// `source_guild` and `source_channel` appear only for channel follower
/// webhooks; `token` is returned for incoming webhooks; and `url` is returned
/// by the webhooks `OAuth2` flow. Fields documented as nullable but not
/// optional are serialized as `null` when `None`.
///
/// See [Discord's webhook object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/webhook#webhook-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Webhook {
    /// The webhook's ID.
    pub id: WebhookId,

    /// The webhook's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: WebhookType,

    /// The guild this webhook belongs to, if any.
    ///
    /// Discord may omit this field or send `null`; both decode as `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The channel this webhook posts to, if any.
    ///
    /// This field is present and nullable in the webhook object; it is
    /// serialized as `null` when `None`.
    pub channel_id: Option<ChannelId>,

    /// The user who created the webhook.
    ///
    /// Not returned when getting a webhook with its token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// The default name of the webhook.
    ///
    /// This field is present and nullable in the webhook object; it is
    /// serialized as `null` when `None`.
    pub name: Option<String>,

    /// The default avatar hash of the webhook.
    ///
    /// This field is present and nullable in the webhook object; it is
    /// serialized as `null` when `None`.
    pub avatar: Option<String>,

    /// The secure token used to execute the webhook.
    ///
    /// Returned for incoming webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// The bot or `OAuth2` application that created the webhook.
    ///
    /// This field is present and nullable in the webhook object; it is
    /// serialized as `null` when `None`.
    pub application_id: Option<ApplicationId>,

    /// The guild of the channel that this webhook is following.
    ///
    /// Returned for channel follower webhooks, and absent when the webhook
    /// creator has lost access to that guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_guild: Option<WebhookSourceGuild>,

    /// The channel that this webhook is following.
    ///
    /// Returned for channel follower webhooks, and absent when the webhook
    /// creator has lost access to the guild that contains the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_channel: Option<PartialChannel>,

    /// The URL used for executing the webhook.
    ///
    /// Returned by the webhooks `OAuth2` flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl Identifiable for Webhook {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The body of `PATCH /webhooks/{webhook.id}`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// Discord documents `avatar` as nullable image data; this type cannot emit an
/// explicit `null`, so omitting the field leaves the avatar unchanged. See
/// [Discord's modify webhook endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/webhook#modify-webhook
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct ModifyWebhookRequest {
    /// The new default name of the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Image data for the new default webhook avatar.
    ///
    /// Discord accepts a base64-encoded `data:` URI here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// The ID of the channel the webhook should be moved to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
}

/// The body of `PATCH /webhooks/{webhook.id}/{webhook.token}`.
///
/// This is the token variant of [`ModifyWebhookRequest`]: it does not
/// authenticate and therefore does not accept the `channel_id` field.
/// All fields are optional and are omitted during serialization when `None`.
/// See [Discord's modify webhook with token endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/webhook#modify-webhook-with-token
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct ModifyWebhookWithTokenRequest {
    /// The new default name of the webhook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Image data for the new default webhook avatar.
    ///
    /// Discord accepts a base64-encoded `data:` URI here.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
}

/// The body of `POST /webhooks/{webhook.id}/{webhook.token}`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// Discord requires at least one of `content`, `embeds`, `components`,
/// `attachments`, or `poll`; the type models the wire shape and leaves that
/// rule to the API.
///
/// The `wait`, `thread_id`, and `with_components` parameters are query-string
/// parameters and are not part of this body. `files[n]` and `payload_json`
/// belong to `multipart/form-data` uploads and are not JSON body fields, so
/// newly uploaded files are referenced from [`attachments`](Self::attachments)
/// by their upload index instead.
///
/// See [Discord's execute webhook endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/webhook#execute-webhook
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct ExecuteWebhookRequest {
    /// The message contents, up to 2000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// An override for the webhook's default username.
    ///
    /// Not supported by interaction followup messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// An override for the webhook's default avatar.
    ///
    /// Not supported by interaction followup messages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// Whether the message is a text-to-speech message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,

    /// Embedded rich content, up to ten embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,

    /// Mention restrictions for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,

    /// The components to include with the message.
    ///
    /// Non-application-owned webhooks can only send non-interactive
    /// components, and Discord ignores this field unless the request also sets
    /// the `with_components` query parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// Metadata for retained and newly uploaded attachments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,

    /// Message flags combined as a bitfield.
    ///
    /// Discord only accepts [`MessageFlags::SUPPRESS_EMBEDS`],
    /// [`MessageFlags::SUPPRESS_NOTIFICATIONS`], and
    /// [`MessageFlags::IS_COMPONENTS_V2`] on this endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// The name of the thread to create.
    ///
    /// Requires the webhook's channel to be a forum or media channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_name: Option<String>,

    /// The IDs of the tags to apply to the created thread.
    ///
    /// Requires the webhook's channel to be a forum or media channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<ChannelId>>,

    /// The poll to attach to the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollCreateRequest>,
}

/// The body of `PATCH /webhooks/{webhook.id}/{webhook.token}/messages/{message.id}`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// The `thread_id` and `with_components` parameters are query-string
/// parameters and are not part of this body, and `files[n]` and `payload_json`
/// are `multipart/form-data` parameters that are not JSON body fields.
///
/// See [Discord's edit webhook message endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/webhook#edit-webhook-message
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct EditWebhookMessageRequest {
    /// The new message contents, up to 2000 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Embedded rich content, up to ten embeds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Embed>>,

    /// Message flags combined as a bitfield.
    ///
    /// Discord only accepts [`MessageFlags::SUPPRESS_EMBEDS`] and
    /// [`MessageFlags::IS_COMPONENTS_V2`] on this endpoint, and
    /// [`MessageFlags::IS_COMPONENTS_V2`] can only be set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,

    /// Mention restrictions for the message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_mentions: Option<AllowedMentions>,

    /// The components to include with the message.
    ///
    /// Non-application-owned webhooks can only send non-interactive
    /// components, and Discord ignores this field unless the request also sets
    /// the `with_components` query parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Component>>,

    /// The attachments to keep on the message after the edit, with their
    /// metadata.
    ///
    /// Discord requires this list to contain every attachment that should
    /// remain, including newly uploaded files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentRequest>>,

    /// The poll to attach to the message.
    ///
    /// Discord only accepts a poll when editing a deferred interaction
    /// response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<PollCreateRequest>,
}
