/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::emoji::EmojiId;

/// The emoji used as the default reaction for new forum or media threads.
///
/// Exactly one of `emoji_id` and `emoji_name` is set. Both fields are present
/// in Discord's object and are `null` when unused, so serialization emits
/// `null` rather than omitting them. See [Discord's default reaction
/// object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#default-reaction-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct DefaultReaction {
    /// The ID of a guild's custom emoji, or `None` when a standard emoji is
    /// used.
    pub emoji_id: Option<EmojiId>,

    /// The Unicode character of a standard emoji, or `None` when a custom
    /// emoji is used.
    pub emoji_name: Option<String>,
}
