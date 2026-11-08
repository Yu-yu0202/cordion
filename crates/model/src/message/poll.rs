/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Poll types carried by messages.
//!
//! [`Poll`] models the poll object Discord sends with a message, including the
//! [`PollResults`] it fetches, its [`PollAnswer`]s, the [`PollMedia`] backing
//! the question and each answer, and the [`PollLayoutType`] of its layout.
//! [`PollAnswerCount`] models the vote count of one answer.
//!
//! [`PollCreateRequest`] is the request shape carried in interaction callback
//! message data; it takes [`duration`](PollCreateRequest::duration) instead of
//! the response's `expiry`.
//!
//! See [Discord's poll object][object] and [Discord's poll create request
//! object][request] for details.
//!
//! [object]: https://docs.discord.com/developers/resources/poll#poll-object
//! [request]: https://docs.discord.com/developers/resources/poll#poll-create-request-object

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::emoji::PartialEmoji;

/// A poll attached to a message.
///
/// [`expiry`](Self::expiry) is nullable in the protocol to allow non-expiring
/// polls in the future, though all polls currently expire. Absent optional
/// fields are omitted during serialization.
///
/// See [Discord's poll object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/poll#poll-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Poll {
    /// The poll question.
    pub question: PollMedia,

    /// The available answers.
    pub answers: Vec<PollAnswer>,

    /// When the poll ends.
    ///
    /// The field is nullable; an absent field also decodes as `None` and
    /// serialization emits `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub expiry: Option<SystemTime>,

    /// Whether users can select multiple answers.
    pub allow_multiselect: bool,

    /// The poll's layout, serialized as `layout_type`.
    ///
    /// Discord currently sends [`PollLayoutType::DEFAULT`].
    #[serde(rename = "layout_type")]
    pub layout_type: PollLayoutType,

    /// The poll's results, when Discord fetched them.
    ///
    /// Absence means the results are unknown, not that there are no votes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<PollResults>,
}

/// The layout of a [`Poll`].
///
/// Unrecognized values are preserved. See [Discord's layout types][types] for
/// details.
///
/// [types]: https://docs.discord.com/developers/resources/poll#layout-type
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct PollLayoutType(u8);

impl PollLayoutType {
    /// The default poll layout.
    pub const DEFAULT: Self = Self(1);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// Text and an optional emoji backing a poll question or answer.
///
/// Discord says question text is always present today but that future poll
/// media may omit it, so both fields are optional.
///
/// See [Discord's poll media object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/poll#poll-media-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PollMedia {
    /// The text of the question or answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// The emoji of the question or answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,
}

/// One answer in a [`Poll`].
///
/// [`answer_id`](Self::answer_id) is only sent in responses; it is absent from
/// poll create requests.
///
/// See [Discord's poll answer object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/poll#poll-answer-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PollAnswer {
    /// The answer's ID, assigned by Discord in responses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<u32>,

    /// The answer's text and emoji.
    pub poll_media: PollMedia,
}

/// The results of a [`Poll`].
///
/// See [Discord's poll results object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/poll#poll-results-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PollResults {
    /// Whether the votes have been precisely counted.
    pub is_finalized: bool,

    /// The counts for each answer.
    ///
    /// An answer absent from this list has no votes.
    pub answer_counts: Vec<PollAnswerCount>,
}

/// The vote count for one [`Poll`] answer.
///
/// See [Discord's poll answer count object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/poll#poll-results-object-poll-answer-count-object-structure
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PollAnswerCount {
    /// The answer's ID.
    pub id: u32,

    /// The number of votes for the answer.
    pub count: u32,

    /// Whether the current user voted for the answer.
    pub me_voted: bool,
}

/// A poll in an interaction callback's message data.
///
/// This is the request shape: it carries
/// [`duration`](Self::duration) instead of the response's `expiry`.
/// Optional fields are omitted during serialization when `None`.
///
/// See [Discord's poll create request object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/poll#poll-create-request-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct PollCreateRequest {
    /// The poll question.
    pub question: PollMedia,

    /// The available answers, up to ten.
    pub answers: Vec<PollAnswer>,

    /// How long the poll stays open, in hours.
    ///
    /// Discord defaults this to 24 and caps it at 32 days.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<u16>,

    /// Whether users can select multiple answers.
    ///
    /// Discord defaults this to `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_multiselect: Option<bool>,

    /// The poll's layout, serialized as `layout_type`.
    #[serde(
        rename = "layout_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub layout_type: Option<PollLayoutType>,
}
