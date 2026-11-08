/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use std::time::SystemTime;

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::channel::ChannelId;
use crate::permission::Permission;
use crate::role::RoleId;
use crate::user::{AvatarDecorationData, Collectibles, User};

bitflags! {
    /// Flags attached to a [`GuildMember`].
    ///
    /// Unrecognized bits are retained during deserialization. Discord sends `0`
    /// when no flags are set. See [Discord's guild member flags][flags] for
    /// details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/guild#guild-member-object-guild-member-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
    pub struct GuildMemberFlags: u64 {
        /// No guild member flags are set.
        const NONE = 0;

        /// The member has left and rejoined the guild.
        const DID_REJOIN = 1 << 0;

        /// The member has completed onboarding.
        const COMPLETED_ONBOARDING = 1 << 1;

        /// The member is exempt from guild verification requirements.
        ///
        /// Discord allows this flag to be edited.
        const BYPASSES_VERIFICATION = 1 << 2;

        /// The member has started onboarding.
        const STARTED_ONBOARDING = 1 << 3;

        /// The member is a guest and can only access the voice channel they
        /// were invited to.
        const IS_GUEST = 1 << 4;

        /// The member has started Server Guide new member actions.
        const STARTED_HOME_ACTIONS = 1 << 5;

        /// The member has completed Server Guide new member actions.
        const COMPLETED_HOME_ACTIONS = 1 << 6;

        /// The member's username, display name, or nickname is blocked by
        /// AutoMod.
        const AUTOMOD_QUARANTINED_USERNAME = 1 << 7;

        /// The member has dismissed the DM settings upsell.
        const DM_SETTINGS_UPSELL_ACKNOWLEDGED = 1 << 9;

        /// The member's guild tag is blocked by AutoMod.
        const AUTOMOD_QUARANTINED_GUILD_TAG = 1 << 10;
    }
}

impl Serialize for GuildMemberFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> Deserialize<'de> for GuildMemberFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u64::deserialize(deserializer)?))
    }
}

/// A user's membership in a guild.
///
/// Discord omits [`user`](Self::user) in the member attached to
/// `MESSAGE_CREATE` and `MESSAGE_UPDATE` events. [`joined_at`](Self::joined_at)
/// is `null` for members invited as guests, and
/// [`permissions`](Self::permissions) is only returned in the interaction
/// member object. [`pending`](Self::pending) is always present in `GUILD_*`
/// events and absent from events that only non-pending members can trigger.
///
/// Optional fields decode as `None` when absent or `null`, and are omitted
/// during serialization. See [Discord's guild member object][object] for
/// details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#guild-member-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct GuildMember {
    /// The user this member represents.
    ///
    /// Absent on the member objects attached to `MESSAGE_CREATE` and
    /// `MESSAGE_UPDATE`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,

    /// The member's guild-specific nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,

    /// The member's guild avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// The member's guild banner hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The IDs of the roles assigned to the member.
    ///
    /// The `@everyone` role is implicit and is not listed here.
    pub roles: Vec<RoleId>,

    /// When the member joined the guild.
    ///
    /// Discord sends `null` for members invited as guests. This field is always
    /// present, so serialization emits `null` when it is `None`.
    #[serde(with = "crate::timestamp::option")]
    pub joined_at: Option<SystemTime>,

    /// When the member started boosting the guild.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub premium_since: Option<SystemTime>,

    /// Whether the member is deafened in voice channels by the server.
    pub deaf: bool,

    /// Whether the member is muted in voice channels by the server.
    pub mute: bool,

    /// The member's flags.
    ///
    /// Discord omits this field when no flags are set, so an absent value
    /// decodes as [`GuildMemberFlags::NONE`].
    #[serde(default)]
    pub flags: GuildMemberFlags,

    /// Whether the member has not yet passed membership screening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,

    /// The member's permissions in the channel, including overwrites.
    ///
    /// Only returned in the member object embedded in an interaction. This
    /// excludes implicit permissions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,

    /// When the member's timeout expires.
    ///
    /// Discord sends `null` or a past time when the member is not timed out.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub communication_disabled_until: Option<SystemTime>,

    /// The member's guild avatar decoration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<AvatarDecorationData>,

    /// The member's collectibles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Collectibles>,
}

/// A partial guild member returned in resolved interaction data.
///
/// Discord's resolved member objects omit [`user`](GuildMember::user),
/// [`deaf`](GuildMember::deaf), and [`mute`](GuildMember::mute); the matching
/// user is stored separately in the resolved data's `users` map. The remaining
/// fields follow [`GuildMember`] and are omitted during serialization when
/// absent unless documented as always present.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialGuildMember {
    /// The member's guild-specific nickname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,

    /// The member's guild avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// The member's guild banner hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The IDs of the roles assigned to the member.
    ///
    /// The `@everyone` role is implicit and is not listed here.
    pub roles: Vec<RoleId>,

    /// When the member joined the guild.
    ///
    /// Discord sends `null` for members invited as guests. This field is always
    /// present in resolved member objects, so serialization emits `null` when
    /// it is `None`.
    #[serde(with = "crate::timestamp::option")]
    pub joined_at: Option<SystemTime>,

    /// When the member started boosting the guild.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub premium_since: Option<SystemTime>,

    /// The member's flags.
    ///
    /// An absent value decodes as [`GuildMemberFlags::NONE`].
    #[serde(default)]
    pub flags: GuildMemberFlags,

    /// Whether the member has not yet passed membership screening.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,

    /// The member's permissions in the resolved channel, including overwrites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,

    /// When the member's timeout expires.
    #[serde(
        default,
        with = "crate::timestamp::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub communication_disabled_until: Option<SystemTime>,

    /// The member's guild avatar decoration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<AvatarDecorationData>,

    /// The member's collectibles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Collectibles>,
}

/// The body of `PUT /guilds/{guild.id}/members/{user.id}`.
///
/// All fields except [`access_token`](Self::access_token) are optional and are
/// omitted during serialization when `None`. See [Discord's add guild member
/// endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#add-guild-member
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct AddGuildMemberRequest {
    /// The `OAuth2` access token granted with the `guilds.join` scope.
    pub access_token: String,

    /// The member's nickname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,

    /// The role IDs to assign to the member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleId>>,

    /// Whether the member is muted in voice channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    /// Whether the member is deafened in voice channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
}

/// The body of `PATCH /guilds/{guild.id}/members/{user.id}`.
///
/// Discord documents every field as optional and nullable. Omitted fields
/// leave the member unchanged, but a field that Discord defines as cleared by
/// `null` cannot express that here; use
/// [`communication_disabled_until`](Self::communication_disabled_until) with a
/// timestamp in the past to remove a timeout. See [Discord's modify guild member
/// endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-member
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildMemberRequest {
    /// The member's nickname.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,

    /// The role IDs to assign to the member.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<RoleId>>,

    /// Whether the member is muted in voice channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    /// Whether the member is deafened in voice channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,

    /// The ID of the channel to move the member to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    /// When the member's timeout expires.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::timestamp::option"
    )]
    pub communication_disabled_until: Option<SystemTime>,

    /// The member's guild member flags.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<GuildMemberFlags>,
}

/// The body of `PATCH /guilds/{guild.id}/members/@me`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// See [Discord's modify current member endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-current-member
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyCurrentMemberRequest {
    /// The member's nickname in the guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,

    /// The member's guild banner hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<String>,

    /// The member's guild avatar hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    /// The member's guild bio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}
