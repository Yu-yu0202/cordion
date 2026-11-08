/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord invite resource.
//!
//! [`Invite`] mirrors the base invite object. `GET /invites/{code}` can add
//! metadata fields such as `uses` and `max_age`; those are modeled separately
//! by [`InviteMetadata`] and combined in [`InviteWithMetadata`]. The deprecated
//! invite stage instance object is not modeled.
//!
//! See [Discord's invite resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/invite

use std::time::SystemTime;

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::application::{ApplicationId, PartialApplication};
use crate::channel::PartialChannel;
use crate::guild::{GuildScheduledEvent, InviteGuild};
use crate::role::{PartialRole, RoleId};
use crate::user::{User, UserId};

/// The type of an [`Invite`].
///
/// Unrecognized values are preserved. See [Discord's invite types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/resources/invite#invite-object-invite-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InviteType(u8);

impl InviteType {
    /// An invite to a guild.
    pub const GUILD: Self = Self(0);

    /// An invite to a group direct message.
    pub const GROUP_DM: Self = Self(1);

    /// A friend invite.
    pub const FRIEND: Self = Self(2);
}

/// The target of a voice channel invite.
///
/// Unrecognized values are preserved. See [Discord's invite target
/// types][types] for details.
///
/// [types]: https://docs.discord.com/developers/resources/invite#invite-object-invite-target-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct InviteTargetType(u8);

impl InviteTargetType {
    /// The invite opens a user's stream.
    pub const STREAM: Self = Self(1);

    /// The invite opens an embedded application.
    pub const EMBEDDED_APPLICATION: Self = Self(2);
}

bitflags! {
    /// Flags attached to a guild invite.
    ///
    /// Unrecognized bits are retained during deserialization. See [Discord's
    /// guild invite flags][flags] for details.
    ///
    /// [flags]: https://docs.discord.com/developers/resources/invite#invite-object-guild-invite-flags
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct GuildInviteFlags: u8 {
        /// No invite flags are set.
        const NONE = 0;

        /// This is a guest invite for a voice channel.
        const IS_GUEST_INVITE = 1 << 0;
    }
}

impl Serialize for GuildInviteFlags {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.bits())
    }
}

impl<'de> Deserialize<'de> for GuildInviteFlags {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from_bits_retain(u8::deserialize(deserializer)?))
    }
}

/// A code that adds a user to a guild or group direct message when used.
///
/// `channel` and `expires_at` are documented as nullable but not optional.
/// Discord's own example omits `expires_at`, however, so this model accepts a
/// missing value and serializes `None` as `null`. Fields marked optional in
/// Discord's documentation are omitted during serialization when `None`.
///
/// See [Discord's invite object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/invite#invite-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Invite {
    /// The invite's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: InviteType,

    /// The invite code, which is unique.
    pub code: String,

    /// The guild this invite is for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild: Option<InviteGuild>,

    /// The channel this invite is for.
    ///
    /// This field is present and nullable in the invite object; it is
    /// serialized as `null` when `None`.
    pub channel: Option<PartialChannel>,

    /// The user who created the invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter: Option<User>,

    /// The target type of this voice channel invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,

    /// The user whose stream is displayed for a stream invite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<User>,

    /// The embedded application opened by an embedded application invite.
    ///
    /// Discord does not publish a formal partial application schema; see
    /// [`PartialApplication`] for the modeled subset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application: Option<PartialApplication>,

    /// The approximate number of online members.
    ///
    /// Returned by `GET /invites/{code}` when `with_counts` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,

    /// The approximate number of total members.
    ///
    /// Returned by `GET /invites/{code}` when `with_counts` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,

    /// When the invite expires.
    ///
    /// Discord documents this field as nullable but not optional, while its
    /// example omits it. Both shapes are accepted, and `None` serializes as
    /// `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub expires_at: Option<SystemTime>,

    /// The guild scheduled event included with this invite.
    ///
    /// Only included when the request supplies a valid
    /// `guild_scheduled_event_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event: Option<GuildScheduledEvent>,

    /// Guild invite flags.
    ///
    /// Only present for guild invites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<GuildInviteFlags>,

    /// The roles assigned to the user when the invite is accepted.
    ///
    /// Discord includes only the partial role fields modeled by
    /// [`PartialRole`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<PartialRole>>,
}

/// Extra information about an [`Invite`].
///
/// These fields are returned by endpoints such as `GET /invites/{code}` and
/// extend the base invite object. See [Discord's invite metadata object][object]
/// for details.
///
/// [object]: https://docs.discord.com/developers/resources/invite#invite-metadata-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InviteMetadata {
    /// The number of times this invite has been used.
    pub uses: u64,

    /// The maximum number of times this invite can be used; `0` means
    /// unlimited.
    pub max_uses: u64,

    /// The duration in seconds after which the invite expires; `0` means it
    /// never expires.
    pub max_age: u64,

    /// Whether the invite grants temporary membership.
    pub temporary: bool,

    /// When the invite was created.
    #[serde(with = "crate::timestamp")]
    pub created_at: SystemTime,
}

/// An [`Invite`] combined with its [`InviteMetadata`].
///
/// Both objects occupy the same JSON object, so this type flattens the two
/// representations. Use [`Invite`] when decoding endpoints that do not include
/// metadata fields.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct InviteWithMetadata {
    /// The base invite fields.
    #[serde(flatten)]
    pub invite: Invite,

    /// The additional metadata fields.
    #[serde(flatten)]
    pub metadata: InviteMetadata,
}

/// The status of a target users job on an invite.
///
/// Unrecognized values are preserved. See [Discord's target users job status
/// codes][codes] for details.
///
/// [codes]: https://docs.discord.com/developers/resources/invite#get-target-users-job-status
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct TargetUsersJobStatusCode(u8);

impl TargetUsersJobStatusCode {
    /// The job status was not reported.
    pub const UNSPECIFIED: Self = Self(0);

    /// The job is still processing the uploaded user IDs.
    pub const PROCESSING: Self = Self(1);

    /// The job completed successfully.
    pub const COMPLETED: Self = Self(2);

    /// The job failed; `error_message` describes why.
    pub const FAILED: Self = Self(3);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// The progress of an invite's target users job.
///
/// Returned by `GET /invites/{invite.code}/target-users/job-status`. Discord
/// publishes this shape only as an example response, so the fields below are
/// the ones that example contains and none of them is guaranteed by a
/// documented field table. [`completed_at`](Self::completed_at) and
/// [`error_message`](Self::error_message) are nullable; they are omitted during
/// serialization when `None`.
///
/// See [Discord's get target users job status endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/invite#get-target-users-job-status
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct TargetUsersJobStatus {
    /// The job's status.
    pub status: TargetUsersJobStatusCode,

    /// The total number of users the job has to process.
    pub total_users: u64,

    /// The number of users the job has processed so far.
    pub processed_users: u64,

    /// When the job was created.
    #[serde(with = "crate::timestamp")]
    pub created_at: SystemTime,

    /// When the job finished, or `None` while it is still running.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::timestamp::option"
    )]
    pub completed_at: Option<SystemTime>,

    /// The failure reason, or `None` when the job did not fail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// The body of `POST /channels/{channel.id}/invites`.
///
/// All fields are optional and are omitted during serialization when `None`.
/// Discord still requires a JSON body, so a request that changes nothing must
/// be sent as `{}`.
///
/// `target_users_file` and `payload_json` are not modeled because Discord
/// documents them as `multipart/form-data` parameters rather than JSON body
/// fields; [`target_user_ids`](Self::target_user_ids) is the JSON-representable
/// equivalent and the two are mutually exclusive.
///
/// See [Discord's create channel invite endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/channel#create-channel-invite
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug, Default)]
pub struct CreateChannelInviteRequest {
    /// The duration in seconds before the invite expires; `0` never expires.
    ///
    /// Discord accepts 0 through 604800 and defaults to 86400.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,

    /// The maximum number of uses; `0` means unlimited.
    ///
    /// Discord accepts 0 through 100 and defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u8>,

    /// Whether the invite only grants temporary membership.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary: Option<bool>,

    /// Whether a new invite code should be generated instead of reusing a
    /// similar one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,

    /// The target type of this voice channel invite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,

    /// The ID of the user whose stream to display.
    ///
    /// Required when [`target_type`](Self::target_type) is
    /// [`InviteTargetType::STREAM`], and the user must be streaming in the
    /// channel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,

    /// The ID of the embedded application to open.
    ///
    /// Required when [`target_type`](Self::target_type) is
    /// [`InviteTargetType::EMBEDDED_APPLICATION`], and the application must
    /// have the `EMBEDDED` flag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<ApplicationId>,

    /// The IDs of the users allowed to see and accept the invite.
    ///
    /// Discord accepts up to 1000 IDs and rejects this field when a
    /// `target_users_file` is uploaded instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user_ids: Option<Vec<UserId>>,

    /// The IDs of the roles granted to users who accept the invite.
    ///
    /// Requires the `MANAGE_ROLES` permission, and the roles cannot carry
    /// permissions above the sender's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<RoleId>>,
}

/// The body of `POST /invites/{invite.code}/target-users/bulk-add`.
///
/// Adds target users to an existing invite. See [Discord's bulk-add target
/// users endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/invite#bulk-add-target-users
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BulkAddInviteTargetUsersRequest {
    /// The IDs of the users to add, at most 1000.
    pub user_ids: Vec<UserId>,
}

/// The body of `POST /invites/{invite.code}/target-users/bulk-delete`.
///
/// Removes target users from an existing invite. See [Discord's bulk-delete
/// target users endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/invite#bulk-delete-target-users
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct BulkDeleteInviteTargetUsersRequest {
    /// The IDs of the users to remove, at most 1000.
    pub user_ids: Vec<UserId>,
}

/// A partial invite returned by `GET /guilds/{guild.id}/vanity-url`.
///
/// The endpoint requires the `MANAGE_GUILD` permission. Discord returns `null`
/// for `code` when the guild has no vanity URL; the usage count remains
/// available. See [Discord's vanity URL endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#get-guild-vanity-url
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PartialInvite {
    /// The vanity URL code, or `None` if the guild has no vanity URL.
    ///
    /// Discord sends this field as `null` when unset, so it is serialized as
    /// `null` when `None`.
    pub code: Option<String>,

    /// The number of times the vanity invite has been used.
    pub uses: u64,
}
