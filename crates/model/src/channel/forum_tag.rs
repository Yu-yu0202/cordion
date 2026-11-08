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
use crate::id::{Id, Identifiable};

/// The ID of a [`ForumTag`].
pub type ForumTagId = Id<ForumTag>;

/// A tag that can be applied to threads in a forum or media channel.
///
/// `emoji_id` and `emoji_name` are both present and at most one is non-null.
/// See [Discord's forum tag object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#forum-tag-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct ForumTag {
    /// The tag's ID.
    pub id: ForumTagId,

    /// The tag's name, up to 20 characters.
    pub name: String,

    /// Whether only members with `MANAGE_THREADS` can add or remove this tag.
    pub moderated: bool,

    /// The ID of a guild's custom emoji, or `None` when a standard emoji is
    /// used.
    pub emoji_id: Option<EmojiId>,

    /// The Unicode character of a standard emoji, or `None` when a custom
    /// emoji is used.
    pub emoji_name: Option<String>,
}

impl Identifiable for ForumTag {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}
