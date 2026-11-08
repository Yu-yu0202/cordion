/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::guild::{GuildId, GuildScheduledEventId};
use crate::id::{Id, Identifiable};

/// The ID of a [`StageInstance`].
pub type StageInstanceId = Id<StageInstance>;

/// The privacy level of a [`StageInstance`].
///
/// Unrecognized values are preserved. See [Discord's privacy levels][levels]
/// for details.
///
/// [levels]: https://docs.discord.com/developers/resources/stage-instance#stage-instance-object-privacy-level
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct StagePrivacyLevel(u8);

impl StagePrivacyLevel {
    /// The Stage instance is visible publicly.
    ///
    /// Discord marks this level as deprecated.
    pub const PUBLIC: Self = Self(1);

    /// The Stage instance is visible only to guild members.
    pub const GUILD_ONLY: Self = Self(2);
}

/// A live Stage instance associated with a stage channel.
///
/// [`guild_scheduled_event_id`](Self::guild_scheduled_event_id) is always
/// present and serializes as `null` when the instance is not associated with a
/// scheduled event.
///
/// See [Discord's Stage instance object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/stage-instance#stage-instance-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct StageInstance {
    /// The Stage instance's ID.
    pub id: StageInstanceId,

    /// The guild of the associated stage channel.
    pub guild_id: GuildId,

    /// The associated stage channel.
    pub channel_id: ChannelId,

    /// The topic of the Stage instance, between 1 and 120 characters.
    pub topic: String,

    /// The Stage instance's privacy level.
    pub privacy_level: StagePrivacyLevel,

    /// Whether Stage Discovery is disabled.
    ///
    /// Discord marks this field as deprecated.
    pub discoverable_disabled: bool,

    /// The scheduled event associated with this Stage instance.
    ///
    /// This field is always present and serializes as `null` when there is no
    /// associated event.
    pub guild_scheduled_event_id: Option<GuildScheduledEventId>,
}

impl Identifiable for StageInstance {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The body of `POST /stage-instances`.
///
/// Discord documents [`channel_id`](Self::channel_id) and
/// [`topic`](Self::topic) as required and the remaining fields as optional, so
/// the optional fields are omitted during serialization when `None`.
///
/// See [Discord's create stage instance endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/stage-instance#create-stage-instance
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateStageInstanceRequest {
    /// The ID of the stage channel.
    pub channel_id: ChannelId,

    /// The topic of the Stage instance, between 1 and 120 characters.
    pub topic: String,

    /// The Stage instance's privacy level.
    ///
    /// Discord documents this field as optional and defaults it to
    /// [`StagePrivacyLevel::GUILD_ONLY`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<StagePrivacyLevel>,

    /// Whether to notify `@everyone` that a Stage instance has started.
    ///
    /// The stage moderator must have the `MENTION_EVERYONE` permission for this
    /// notification to be sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_start_notification: Option<bool>,

    /// The scheduled event associated with this Stage instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event_id: Option<GuildScheduledEventId>,
}

/// The body of `PATCH /stage-instances/{channel.id}`.
///
/// Discord documents every parameter of this endpoint as optional, so each
/// field is omitted during serialization when `None` and an omitted field
/// leaves the Stage instance unchanged.
///
/// See [Discord's modify stage instance endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/stage-instance#modify-stage-instance
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyStageInstanceRequest {
    /// The topic of the Stage instance, between 1 and 120 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    /// The Stage instance's privacy level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<StagePrivacyLevel>,
}
