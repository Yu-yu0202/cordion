/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Presence and activity models from Discord Gateway events.
//!
//! See [Discord's presence update][presence] for details.
//!
//! [presence]: https://docs.discord.com/developers/events/gateway-events#presence-update

mod activity;

use std::borrow::Cow;

pub use activity::{
    Activity, ActivityAssets, ActivityEmoji, ActivityFlags, ActivityParty, ActivityPartySize,
    ActivitySecrets, ActivityTimestamps, ActivityType, StatusDisplayType,
};
use serde::{Deserialize, Serialize};

use crate::guild::GuildId;
use crate::user::PartialUser;

/// A user's current state on a guild.
///
/// The `user` object in a presence update can be partial and is modeled by
/// [`PartialUser`]. The `guild_id` field is omitted from the partial presences
/// included in a [`GUILD_CREATE`](crate::event::gateway::GuildCreate) event.
///
/// Requires the `GUILD_PRESENCES` Gateway intent to receive updates. See
/// [Discord's presence update][presence] for details.
///
/// [presence]: https://docs.discord.com/developers/events/gateway-events#presence-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Presence {
    /// The user whose presence is being updated.
    pub user: PartialUser,

    /// The guild this presence belongs to.
    ///
    /// Omitted from the partial presences included in a `GUILD_CREATE` event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// The user's overall status.
    pub status: UserStatus,

    /// The user's current activities.
    pub activities: Vec<Activity>,

    /// The user's platform-dependent status.
    pub client_status: ClientStatus,
}

/// A user's overall presence status.
///
/// Discord documents `"idle"`, `"dnd"`, `"online"`, and `"offline"`.
/// Unrecognized values are preserved so that new statuses do not make decoding
/// fail. See [Discord's presence update][presence] for details.
///
/// [presence]: https://docs.discord.com/developers/events/gateway-events#presence-update
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UserStatus(Cow<'static, str>);

impl UserStatus {
    /// The user is online.
    pub const ONLINE: Self = Self(Cow::Borrowed("online"));

    /// The user is idle.
    pub const IDLE: Self = Self(Cow::Borrowed("idle"));

    /// The user does not want to be disturbed.
    pub const DND: Self = Self(Cow::Borrowed("dnd"));

    /// The user is offline or invisible.
    pub const OFFLINE: Self = Self(Cow::Borrowed("offline"));

    /// Creates a status from any string-like value.
    #[must_use]
    pub fn new(status: impl Into<Cow<'static, str>>) -> Self {
        Self(status.into())
    }

    /// Returns the status as a string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A user's status for each active platform session.
///
/// A field is absent when the user has no active session on that platform or
/// has selected the invisible status. See [Discord's client status
/// object][object] for details.
///
/// [object]: https://docs.discord.com/developers/events/gateway-events#client-status-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct ClientStatus {
    /// The status set for an active desktop session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<UserStatus>,

    /// The status set for an active mobile session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<UserStatus>,

    /// The status set for an active web session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<UserStatus>,

    /// The status set for an active virtual reality session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vr: Option<UserStatus>,
}
