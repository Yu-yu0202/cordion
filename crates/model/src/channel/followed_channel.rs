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
use crate::webhook::WebhookId;

/// The result of following an announcement channel into another channel.
///
/// See [Discord's followed channel object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/channel#followed-channel-object
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct FollowedChannel {
    /// The announcement channel being followed.
    pub channel_id: ChannelId,

    /// The webhook created in the target channel.
    pub webhook_id: WebhookId,
}
