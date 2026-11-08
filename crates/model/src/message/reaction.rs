/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Reaction types carried by messages.
//!
//! [`Reaction`] models the reaction object, with [`ReactionCountDetails`]
//! breaking down its normal and super reaction counts.
//!
//! See [Discord's reaction object][object] and [Discord's reaction count
//! details object][details] for details.
//!
//! [object]: https://docs.discord.com/developers/resources/message#reaction-object
//! [details]: https://docs.discord.com/developers/resources/message#reaction-count-details-object

use serde::{Deserialize, Serialize};

use crate::emoji::PartialEmoji;

/// A reaction to a [`Message`](crate::message::Message).
///
/// See [Discord's reaction object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#reaction-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Reaction {
    /// The total number of reactions, including super reactions.
    pub count: u32,

    /// The breakdown of normal and super reaction counts.
    pub count_details: ReactionCountDetails,

    /// Whether the current user reacted with this emoji.
    pub me: bool,

    /// Whether the current user super-reacted with this emoji.
    pub me_burst: bool,

    /// The emoji used for the reaction.
    pub emoji: PartialEmoji,

    /// The hexadecimal colors used for the super reaction.
    pub burst_colors: Vec<String>,
}

/// Normal and super reaction counts for a [`Reaction`].
///
/// See [Discord's reaction count details object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/message#reaction-count-details-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ReactionCountDetails {
    /// The number of super reactions.
    pub burst: u32,

    /// The number of normal reactions.
    pub normal: u32,
}
