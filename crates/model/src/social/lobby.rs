/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord lobby resource.
//!
//! Lobbies group users for matchmaking and are only used with the Discord
//! Social SDK. Most endpoints that mutate a lobby require either a `Bearer`
//! token with the `sdk.social_layer` scope or a bot token.
//!
//! Request payloads for the lobby endpoints are modeled in the same module as
//! separate types, because their shapes differ from the returned objects.
//!
//! See [Discord's lobby resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/lobby

mod message;
mod request;

use std::collections::BTreeMap;

use cordion_macro::IdTarget;
pub use message::{LobbyInvite, LobbyMessage, LobbyMessageMember};
pub use request::{
    BulkLobbyMemberRequest, CreateLobbyRequest, CreateOrJoinLobbyRequest, LinkLobbyChannelRequest,
    LobbyMemberRequest, ModifyLobbyRequest, SendLobbyMessageRequest, UpsertLobbyMemberRequest,
};
use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::channel::Channel;
use crate::id::{Id, Identifiable};
use crate::user::UserId;

/// The ID of a [`Lobby`].
pub type LobbyId = Id<Lobby>;

/// A metadata dictionary used by lobby and application identity objects.
///
/// Discord limits the serialized length of a lobby or member metadata dictionary
/// to 1000 characters total.
pub type Metadata = BTreeMap<String, String>;

/// A lobby that groups users for matchmaking.
///
/// [`metadata`](Self::metadata) is nullable, so `None` is serialized as `null`.
/// Other optional fields are omitted during serialization when `None`.
///
/// See [Discord's lobby object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/lobby#lobby-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Lobby {
    /// The lobby's ID.
    pub id: LobbyId,

    /// The ID of the application that created the lobby.
    pub application_id: ApplicationId,

    /// The lobby's metadata.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub metadata: Option<Metadata>,

    /// The lobby's members.
    pub members: Vec<LobbyMember>,

    /// The guild channel linked to the lobby.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_channel: Option<Box<Channel>>,
}

impl Identifiable for Lobby {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// A member of a [`Lobby`].
///
/// Request payloads send a subset of these fields and distinguish an omitted
/// [`additional_name`](Self::additional_name) (preserve the current value) from
/// an explicit `null` (clear it), so the request type for member upserts is
/// [`LobbyMemberRequest`].
///
/// See [Discord's lobby member object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/lobby#lobby-member-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct LobbyMember {
    /// The member's user ID.
    pub id: UserId,

    /// The member's metadata.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub metadata: Option<Metadata>,

    /// The member's lobby flags combined as a bitfield.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<LobbyMemberFlags>,

    /// The member's additional display name, such as an in-game character name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<String>,
}

/// Flags attached to a [`LobbyMember`].
///
/// Unrecognized bits are preserved. See [Discord's lobby member flags][flags]
/// for details.
///
/// [flags]: https://docs.discord.com/developers/resources/lobby#lobby-member-object-lobby-member-flags
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct LobbyMemberFlags(u64);

impl LobbyMemberFlags {
    /// The user can link a text channel to the lobby.
    pub const CAN_LINK_LOBBY: Self = Self(1 << 0);

    /// Returns the raw bitfield Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u64 {
        self.0
    }

    /// Creates a flag set from a raw bitfield.
    ///
    /// Bits that Cordion does not define are retained so that new flags do not
    /// make decoding fail.
    #[inline]
    #[must_use]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }
}

impl From<u64> for LobbyMemberFlags {
    #[inline]
    fn from(raw: u64) -> Self {
        Self::from_raw(raw)
    }
}
