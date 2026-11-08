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
//! This module groups the two event delivery mechanisms Discord offers and the
//! presence data they carry:
//!
//! - [`gateway`] models the WebSocket Gateway payload envelope, send events,
//!   and the receive events.
//! - [`webhook_event`] models the HTTP event payloads delivered to an
//!   application's interactions endpoint.
//! - [`presence`] models the presence and activity data that Gateway events
//!   carry.
//!
//! See [Discord's overview of events][overview] for the delivery mechanisms.
//!
//! [overview]: https://docs.discord.com/developers/events/overview
