/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord entitlement resource.
//!
//! Entitlements represent a user's or guild's access to a premium SKU. They
//! appear in interaction payloads and in the application entitlement
//! endpoints.
//!
//! See [Discord's entitlement resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/entitlement

use std::time::SystemTime;

use cordion_macro::IdTarget;
use serde::{Deserialize, Serialize};

use crate::application::ApplicationId;
use crate::guild::GuildId;
use crate::id::{Id, Identifiable, snowflake};
use crate::monetization::sku::SkuId;
use crate::user::UserId;

/// The ID of an [`Entitlement`].
pub type EntitlementId = Id<Entitlement>;

/// How an [`Entitlement`] was acquired.
///
/// Unrecognized values are preserved. See [Discord's entitlement types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/resources/entitlement#entitlement-object-entitlement-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct EntitlementType(u8);

impl EntitlementType {
    /// The entitlement was purchased by a user.
    pub const PURCHASE: Self = Self(1);

    /// The entitlement is for a Discord Nitro subscription.
    pub const PREMIUM_SUBSCRIPTION: Self = Self(2);

    /// The entitlement was gifted by a developer.
    pub const DEVELOPER_GIFT: Self = Self(3);

    /// The entitlement was purchased by a developer in application test mode.
    pub const TEST_MODE_PURCHASE: Self = Self(4);

    /// The entitlement was granted when the SKU was free.
    pub const FREE_PURCHASE: Self = Self(5);

    /// The entitlement was gifted by another user.
    pub const USER_GIFT: Self = Self(6);

    /// The entitlement was claimed for free as a Nitro subscriber.
    pub const PREMIUM_PURCHASE: Self = Self(7);

    /// The entitlement was purchased as an app subscription.
    pub const APPLICATION_SUBSCRIPTION: Self = Self(8);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// An entitlement granting access to a premium SKU.
///
/// Exactly one of [`user_id`](Self::user_id) and [`guild_id`](Self::guild_id)
/// identifies the entitlement's owner in practice. The timestamps are nullable
/// and are serialized as `null` when absent. Fields present in newer payloads
/// that are not modeled here are ignored during deserialization.
///
/// See [Discord's entitlement object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/entitlement#entitlement-object
#[derive(Clone, Copy, PartialEq, Eq, IdTarget, Serialize, Deserialize, Debug)]
pub struct Entitlement {
    /// The entitlement's ID.
    pub id: EntitlementId,

    /// The ID of the SKU this entitlement grants.
    pub sku_id: SkuId,

    /// The ID of the parent application.
    pub application_id: ApplicationId,

    /// The ID of the user granted access, when the entitlement belongs to a
    /// user.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    /// How the entitlement was acquired, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: EntitlementType,

    /// Whether the entitlement has been deleted.
    pub deleted: bool,

    /// When the entitlement becomes valid.
    ///
    /// Discord defines this as nullable. An absent field also decodes as
    /// `None`; serialization emits `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub starts_at: Option<SystemTime>,

    /// When the entitlement stops being valid.
    ///
    /// Discord defines this as nullable. An absent field also decodes as
    /// `None`; serialization emits `null`.
    #[serde(default, with = "crate::timestamp::option")]
    pub ends_at: Option<SystemTime>,

    /// The ID of the guild granted access, when the entitlement belongs to a
    /// guild.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    /// For consumable items, whether the entitlement has been consumed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed: Option<bool>,
}

impl Identifiable for Entitlement {
    #[inline]
    fn id(&self) -> Id<Self> {
        self.id
    }
}

/// The owner type of a [`CreateTestEntitlementRequest`].
///
/// Unrecognized values are preserved. See [Discord's create test entitlement
/// endpoint][endpoint] for the documented values.
///
/// [endpoint]: https://docs.discord.com/developers/resources/entitlement#create-test-entitlement
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct EntitlementOwnerType(u8);

impl EntitlementOwnerType {
    /// The entitlement is granted to a guild.
    pub const GUILD: Self = Self(1);

    /// The entitlement is granted to a user.
    pub const USER: Self = Self(2);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// The body of `POST /applications/{application.id}/entitlements`.
///
/// Creates a test entitlement for a guild or user. Discord returns a partial
/// [`Entitlement`] that never contains `subscription_id`, `starts_at`, or
/// `ends_at`, because the entitlement is valid in perpetuity.
///
/// See [Discord's create test entitlement endpoint][endpoint] for details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/entitlement#create-test-entitlement
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct CreateTestEntitlementRequest {
    /// The ID of the SKU the entitlement grants access to.
    pub sku_id: SkuId,

    /// The ID of the guild or user the entitlement is granted to.
    ///
    /// The value is a guild ID when [`kind`](Self::kind) is
    /// [`EntitlementOwnerType::GUILD`] and a user ID when it is
    /// [`EntitlementOwnerType::USER`], so it is not tied to a single resource
    /// type. Discord sends it as a snowflake-formatted decimal string.
    #[serde(with = "snowflake")]
    pub owner_id: u64,

    /// Whether [`owner_id`](Self::owner_id) names a guild or a user.
    #[serde(rename = "owner_type")]
    pub kind: EntitlementOwnerType,
}
