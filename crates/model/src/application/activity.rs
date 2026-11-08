/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Activity instances of an application.
//!
//! [`ActivityInstance`] models the object returned by
//! `GET /applications/{application.id}/activity-instances/{instance_id}`, which
//! reports where a running Activity instance is hosted and which users are
//! connected to it.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::channel::ChannelId;
use crate::guild::GuildId;
use crate::user::UserId;

/// The kind of location an [`ActivityInstance`] runs in.
///
/// Documented values are exposed as associated constants, and an undocumented
/// value is preserved so that a new kind does not make a payload undecodable.
///
/// See [Discord's activity location kind enum][kind] for details.
///
/// [kind]: https://docs.discord.com/developers/resources/application#get-application-activity-instance-activity-location-kind-enum
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ActivityLocationKind(Cow<'static, str>);

impl ActivityLocationKind {
    /// The location is a guild channel.
    pub const GUILD_CHANNEL: Self = Self(Cow::Borrowed("gc"));

    /// The location is a private channel, such as a DM or group DM.
    pub const PRIVATE_CHANNEL: Self = Self(Cow::Borrowed("pc"));

    /// Creates an owned location kind from any string-like value.
    ///
    /// Prefer [`ActivityLocationKind::from`] with a `&'static str` to borrow
    /// instead of allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the location kind exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for ActivityLocationKind {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// Where an [`ActivityInstance`] runs.
///
/// See [Discord's activity location object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application#get-application-activity-instance-activity-location-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ActivityLocation {
    /// The unique identifier of the location.
    ///
    /// This is an opaque string chosen by Discord rather than a snowflake.
    pub id: String,

    /// The kind of location.
    pub kind: ActivityLocationKind,

    /// The ID of the channel the instance runs in.
    pub channel_id: ChannelId,

    /// The ID of the guild the instance runs in.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub guild_id: Option<GuildId>,
}

/// A running instance of an application's Activity.
///
/// Returned by `GET /applications/{application.id}/activity-instances/{instance_id}`.
/// [`launch_id`](Self::launch_id) is a snowflake that Discord does not tie to a
/// resource type, so it is kept as a raw `u64`; [`instance_id`](Self::instance_id)
/// and [`ActivityLocation::id`] are opaque strings.
///
/// See [Discord's activity instance object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application#get-application-activity-instance-activity-instance-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ActivityInstance {
    /// The ID of the application the instance belongs to.
    pub application_id: ApplicationId,

    /// The instance ID of the Activity.
    pub instance_id: String,

    /// The unique identifier of the launch, serialized as a decimal string.
    #[serde(with = "crate::id::snowflake")]
    pub launch_id: u64,

    /// The location the instance is running in.
    pub location: ActivityLocation,

    /// The IDs of the users currently connected to the instance.
    pub users: Vec<UserId>,
}
