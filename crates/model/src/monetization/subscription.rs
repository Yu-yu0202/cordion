/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord subscription resource.
//!
//! Subscriptions represent recurring payments for at least one SKU over an
//! ongoing period. Successful payments grant the user entitlements associated
//! with the SKU. Subscription status must not be used to grant perks; use
//! entitlements for that purpose.
//!
//! See [Discord's subscription resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/subscription

use std::time::SystemTime;

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::id::{Id, Identifiable};
use crate::monetization::entitlement::EntitlementId;
use crate::monetization::sku::SkuId;
use crate::user::UserId;

/// The ID of a [`Subscription`].
pub type SubscriptionId = Id<Subscription>;

/// The current status of a [`Subscription`].
///
/// Unrecognized values are preserved. See [Discord's subscription statuses][statuses]
/// for details.
///
/// [statuses]: https://docs.discord.com/developers/resources/subscription#subscription-statuses
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct SubscriptionStatus(u8);

impl SubscriptionStatus {
    /// The subscription is active and scheduled to renew.
    pub const ACTIVE: Self = Self(0);

    /// The subscription is inactive and not being charged.
    pub const INACTIVE: Self = Self(1);

    /// The subscription is active but will not renew.
    pub const ENDING: Self = Self(2);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// A user's recurring payment for one or more SKUs.
///
/// [`renewal_sku_ids`](Self::renewal_sku_ids) and [`canceled_at`](Self::canceled_at)
/// are nullable and are serialized as `null` when `None`. Other optional fields
/// are omitted during serialization when absent.
///
/// See [Discord's subscription object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/subscription#subscription-object
#[derive(Clone, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Subscription {
    /// The subscription's ID.
    pub id: SubscriptionId,

    /// The ID of the subscribed user.
    pub user_id: UserId,

    /// The SKUs the user is subscribed to.
    pub sku_ids: Vec<SkuId>,

    /// The entitlements granted by this subscription.
    pub entitlement_ids: Vec<EntitlementId>,

    /// The SKUs the user will be subscribed to at renewal.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub renewal_sku_ids: Option<Vec<SkuId>>,

    /// The start of the current subscription period.
    #[serde(with = "crate::timestamp")]
    pub current_period_start: SystemTime,

    /// The end of the current subscription period.
    #[serde(with = "crate::timestamp")]
    pub current_period_end: SystemTime,

    /// The subscription's current status.
    pub status: SubscriptionStatus,

    /// When the subscription was canceled, or `None` if it is not canceled.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub canceled_at: Option<SystemTime>,

    /// The ISO 3166-1 alpha-2 country code of the payment source.
    ///
    /// This is missing unless the request uses a private `OAuth2` scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl Identifiable for Subscription {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}
