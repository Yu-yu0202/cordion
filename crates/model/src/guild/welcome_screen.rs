/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::channel::ChannelId;
use crate::emoji::Emoji;
use crate::id::Id;

/// A Community guild's welcome screen.
///
/// See [Discord's welcome screen object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/guild#welcome-screen-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct WelcomeScreen {
    /// The server description shown in the welcome screen.
    ///
    /// Discord sends `null` when no description is set.
    pub description: Option<String>,

    /// The channels shown in the welcome screen, up to five.
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

/// One channel entry in a guild's welcome screen.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct WelcomeScreenChannel {
    /// The channel's ID.
    pub channel_id: ChannelId,

    /// The description shown for the channel.
    pub description: String,

    /// The ID of the custom emoji used for this channel.
    ///
    /// Discord sends `null` when no custom emoji is set.
    pub emoji_id: Option<Id<Emoji>>,

    /// The emoji name for a custom emoji, or the Unicode character for a
    /// standard emoji.
    ///
    /// Discord sends `null` when no emoji is set.
    pub emoji_name: Option<String>,
}

/// The body of `PATCH /guilds/{guild.id}/welcome-screen`.
///
/// Discord documents every parameter of this endpoint as optional and nullable,
/// so every field is omitted during serialization when `None` and an omitted
/// field leaves the welcome screen unchanged. Because omission means "leave
/// unchanged", the fields Discord documents as nullable cannot be cleared by
/// sending `null` through this type.
///
/// See [Discord's modify guild welcome screen endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/guild#modify-guild-welcome-screen
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ModifyGuildWelcomeScreenRequest {
    /// Whether the welcome screen is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The channels linked in the welcome screen and their display options.
    ///
    /// Discord allows at most five entries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub welcome_channels: Option<Vec<WelcomeScreenChannel>>,

    /// The server description to show in the welcome screen.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
