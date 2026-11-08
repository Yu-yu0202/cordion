/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Gateway and webhook event payloads.
//!
//! This module groups the event delivery mechanisms Discord offers and the
//! presence data those events carry:
//!
//! - [`gateway`] models the WebSocket Gateway payload envelope, the events an
//!   application sends, and the events it receives.
//! - [`webhook_event`] models the HTTP event payloads delivered to an
//!   application's interactions endpoint.
//! - [`presence`] models the presence and activity data that Gateway events
//!   carry.
//!
//! See [Discord's overview of events][overview] for the delivery mechanisms.
//!
//! [overview]: https://docs.discord.com/developers/events/overview

pub mod gateway;
pub mod presence;
pub mod webhook_event;
