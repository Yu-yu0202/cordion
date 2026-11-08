/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Premium app monetization resources.
//!
//! SKUs are the sellable units, entitlements record what a user or guild has
//! access to, and subscriptions track recurring payments.
//!
//! See [Discord's monetization overview][overview] for how the pieces fit
//! together.
//!
//! [overview]: https://docs.discord.com/developers/monetization/overview

pub mod entitlement;
pub mod sku;
pub mod subscription;
