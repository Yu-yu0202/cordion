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

use crate::emoji::EmojiId;
use crate::guild::GuildId;
use crate::id::{Id, Identifiable};
use crate::user::User;

/// The ID of a [`SoundboardSound`].
pub type SoundboardSoundId = Id<SoundboardSound>;

/// A sound that can be played in voice channels.
///
/// Default sounds omit [`guild_id`](Self::guild_id), while guild sounds include
/// it. [`volume`](Self::volume) is a floating-point number from `0` to `1`, so
/// this type implements [`PartialEq`] but not [`Eq`].
///
/// [`emoji_id`](Self::emoji_id) and [`emoji_name`](Self::emoji_name) are always
/// present and serialize as `null` when unused.
///
/// See [Discord's soundboard sound object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/soundboard#soundboard-sound-object
#[derive(Clone, PartialEq, IdTarget, Serialize, Deserialize, Debug)]
pub struct SoundboardSound {
    /// The sound's name.
    pub name: String,

    /// The sound's ID.
    pub sound_id: SoundboardSoundId,

    /// The sound's volume, from `0` to `1`.
    pub volume: f64,

    /// The ID of the sound's custom emoji.
    ///
    /// This field is always present and serializes as `null` when the sound
    /// uses a standard emoji or no emoji.
    pub emoji_id: Option<EmojiId>,

    /// The Unicode character of the sound's standard emoji.
    ///
    /// This field is always present and serializes as `null` when the sound
    /// uses a custom emoji or no emoji.
    pub emoji_name: Option<String>,

    /// The guild this sound belongs to.
    ///
    /// Omitted for default soundboard sounds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// Whether the sound can currently be used.
    ///
    /// This may be `false` because the guild lost the required Server Boosts.
    pub available: bool,

    /// The user who created the sound.
    ///
    /// Only included when the bot has the `CREATE_GUILD_EXPRESSIONS` or
    /// `MANAGE_GUILD_EXPRESSIONS` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl Identifiable for SoundboardSound {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.sound_id
    }
}

/// The body of `POST /guilds/{guild.id}/soundboard-sounds`.
///
/// Discord documents [`name`](Self::name) and [`sound`](Self::sound) as
/// required and the remaining fields as optional, so the optional fields are
/// omitted during serialization when `None`. An omitted
/// [`volume`](Self::volume) defaults to `1`.
///
/// [`sound`](Self::sound) carries the sound as a `data:` URI whose payload is
/// the base64-encoded MP3 or Ogg data, in the same shape as Discord's
/// [image data]. Discord limits a soundboard sound to 512 KiB and 5.2 seconds.
///
/// See [Discord's create guild soundboard sound endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/soundboard#create-guild-soundboard-sound
/// [image data]: https://docs.discord.com/developers/reference#image-data
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct CreateGuildSoundboardSoundRequest {
    /// The sound's name, between 2 and 32 characters.
    pub name: String,

    /// The sound's MP3 or Ogg data as a `data:` URI with a base64 payload.
    pub sound: String,

    /// The sound's volume, from `0` to `1`.
    ///
    /// Discord documents this field as optional and nullable, and defaults an
    /// omitted value to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,

    /// The ID of the sound's custom emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<EmojiId>,

    /// The Unicode character of the sound's standard emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
}

/// The body of `PATCH /guilds/{guild.id}/soundboard-sounds/{sound.id}`.
///
/// Discord documents every parameter of this endpoint as optional, so every
/// field is omitted during serialization when `None` and an omitted field
/// leaves the sound unchanged. Because omission means "leave unchanged", the
/// fields Discord documents as nullable cannot be cleared by sending `null`
/// through this type.
///
/// See [Discord's modify guild soundboard sound endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/soundboard#modify-guild-soundboard-sound
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildSoundboardSoundRequest {
    /// The sound's name, between 2 and 32 characters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The sound's volume, from `0` to `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,

    /// The ID of the sound's custom emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<EmojiId>,

    /// The Unicode character of the sound's standard emoji.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<String>,
}

/// The body of `POST /channels/{channel.id}/send-soundboard-sound`.
///
/// Discord documents [`sound_id`](Self::sound_id) as required and
/// [`source_guild_id`](Self::source_guild_id) as optional, so the optional
/// field is omitted during serialization when `None`.
///
/// See [Discord's send soundboard sound endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/soundboard#send-soundboard-sound
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub struct SendSoundboardSoundRequest {
    /// The ID of the soundboard sound to play.
    pub sound_id: SoundboardSoundId,

    /// The ID of the guild the soundboard sound is from.
    ///
    /// Required to play sounds from a different guild than the voice channel's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_guild_id: Option<GuildId>,
}
