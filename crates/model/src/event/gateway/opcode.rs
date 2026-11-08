/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Gateway opcodes.
//!
//! Every Gateway payload starts with an opcode that tells the receiver how to
//! interpret the rest of the payload. Discord can introduce new opcodes, so
//! unrecognized values are preserved rather than rejected.
//!
//! See [Discord's Gateway opcodes][opcodes] for details.
//!
//! [opcodes]: https://docs.discord.com/developers/topics/opcodes-and-status-codes#gateway-gateway-opcodes

use serde::{Deserialize, Serialize};

/// A Gateway opcode identifying the type of a Gateway payload.
///
/// The opcodes documented by Discord are exposed as associated constants; an
/// opcode Discord has not documented yet is preserved as-is, so a payload with
/// an unfamiliar opcode still decodes. Use [`raw`](Self::raw) to inspect the
/// original integer.
///
/// See [Discord's Gateway opcodes][opcodes] for details.
///
/// [opcodes]: https://docs.discord.com/developers/topics/opcodes-and-status-codes#gateway-gateway-opcodes
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct GatewayOpcode(u8);

impl GatewayOpcode {
    /// An event was dispatched.
    pub const DISPATCH: Self = Self(0);

    /// A heartbeat must be sent or was received, depending on the direction.
    pub const HEARTBEAT: Self = Self(1);

    /// A new session must be started.
    pub const IDENTIFY: Self = Self(2);

    /// The client's presence must be updated.
    pub const PRESENCE_UPDATE: Self = Self(3);

    /// The client's voice state must be updated.
    pub const VOICE_STATE_UPDATE: Self = Self(4);

    /// A dropped session must be resumed.
    pub const RESUME: Self = Self(6);

    /// The client must reconnect and resume, if it has a session to resume.
    pub const RECONNECT: Self = Self(7);

    /// Members of a guild must be requested.
    pub const REQUEST_GUILD_MEMBERS: Self = Self(8);

    /// The session is invalid, so the client must reconnect and identify or
    /// resume.
    pub const INVALID_SESSION: Self = Self(9);

    /// The connection was accepted and the heartbeat interval is defined.
    pub const HELLO: Self = Self(10);

    /// A heartbeat sent by the client was acknowledged.
    pub const HEARTBEAT_ACK: Self = Self(11);

    /// Ephemeral soundboard sounds must be requested for a set of guilds.
    pub const REQUEST_SOUNDBOARD_SOUNDS: Self = Self(31);

    /// Ephemeral channel data must be requested for the channels of a guild.
    pub const REQUEST_CHANNEL_INFO: Self = Self(43);

    /// Returns the opcode exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}
