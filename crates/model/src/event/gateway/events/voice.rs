/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Voice-related Gateway events.
//!
//! `VOICE_STATE_UPDATE` carries a [`VoiceState`] unchanged, so no dedicated event
//! type exists for it.
//!
//! [`VoiceState`]: crate::voice::VoiceState

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::emoji::PartialEmoji;
use crate::guild::{GuildId, SoundboardSoundId};
use crate::user::UserId;

/// The payload of the `VOICE_SERVER_UPDATE` event.
///
/// Sent when initially connecting to voice and when the voice instance fails
/// over to a new server.
///
/// See [Discord's Voice Server Update event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#voice-server-update
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct VoiceServerUpdate {
    /// The voice connection token.
    pub token: String,

    /// The guild this voice server update is for.
    pub guild_id: GuildId,

    /// The voice server host.
    ///
    /// A `null` endpoint means the allocated voice server went away and a new
    /// one is being allocated; the client should disconnect and wait. Nullable,
    /// so `None` is serialized as `null`.
    #[serde(default)]
    pub endpoint: Option<String>,
}

/// The payload of the `VOICE_CHANNEL_EFFECT_SEND` event.
///
/// Sent when a user sends an emoji reaction or a soundboard sound in a voice
/// channel the current user is connected to. The fields describing the effect
/// are present only for the matching effect: [`emoji`](Self::emoji),
/// [`animation_type`](Self::animation_type), and [`animation_id`](Self::animation_id)
/// describe an emoji reaction, while [`sound_id`](Self::sound_id) and
/// [`sound_volume`](Self::sound_volume) describe a soundboard sound.
///
/// [`emoji`](Self::emoji) and [`animation_type`](Self::animation_type) are
/// nullable, so `None` is serialized as `null`; the remaining fields are optional
/// and are omitted when `None`.
///
/// See [Discord's Voice Channel Effect Send event][event] for details.
///
/// [event]: https://docs.discord.com/developers/events/gateway-events#voice-channel-effect-send
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct VoiceChannelEffectSend {
    /// The ID of the channel the effect was sent in.
    pub channel_id: ChannelId,

    /// The ID of the guild the effect was sent in.
    pub guild_id: GuildId,

    /// The ID of the user who sent the effect.
    pub user_id: UserId,

    /// The emoji sent for an emoji reaction or soundboard effect.
    #[serde(default)]
    pub emoji: Option<PartialEmoji>,

    /// The type of emoji animation.
    #[serde(default)]
    pub animation_type: Option<VoiceEffectAnimationType>,

    /// The ID of the emoji animation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation_id: Option<u64>,

    /// The ID of the soundboard sound that was played.
    ///
    /// Discord documents this as a snowflake or an integer; both decode into
    /// [`SoundboardSoundId`], which always serializes as a decimal string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_id: Option<SoundboardSoundId>,

    /// The volume of the soundboard sound, from `0` to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_volume: Option<f64>,
}

/// The type of animation used by a voice channel effect.
///
/// Unrecognized values are preserved. See [Discord's animation types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/events/gateway-events#voice-channel-effect-send-animation-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct VoiceEffectAnimationType(u8);

impl VoiceEffectAnimationType {
    /// A fun animation, sent by a Nitro subscriber.
    pub const PREMIUM: Self = Self(0);

    /// The standard animation.
    pub const BASIC: Self = Self(1);

    /// Returns the animation type exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}
