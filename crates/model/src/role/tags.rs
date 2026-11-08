/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::guild::IntegrationId;
use crate::monetization::sku::SkuId;
use crate::user::UserId;

/// Tags attached to a guild role.
///
/// Discord represents presence-only tags such as `premium_subscriber` as `null`
/// when they are set and omits them when they are not. This type exposes those
/// tags as `bool`: an absent tag decodes as `false`, a `null` tag decodes as
/// `true`, and `true` serializes as `null` while `false` is omitted.
///
/// See [Discord's role tags structure][tags] for details.
///
/// [tags]: https://docs.discord.com/developers/topics/permissions#role-object-role-tags-structure
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct RoleTags {
    /// The ID of the bot this role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<UserId>,

    /// The ID of the integration this role belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<IntegrationId>,

    /// Whether this is the guild's Booster role.
    #[serde(
        default,
        skip_serializing_if = "is_absent",
        serialize_with = "serialize_presence",
        deserialize_with = "deserialize_presence"
    )]
    pub premium_subscriber: bool,

    /// The ID of this role's subscription SKU and listing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_listing_id: Option<SkuId>,

    /// Whether this role is available for purchase.
    #[serde(
        default,
        skip_serializing_if = "is_absent",
        serialize_with = "serialize_presence",
        deserialize_with = "deserialize_presence"
    )]
    pub available_for_purchase: bool,

    /// Whether this role is a guild's linked role.
    #[serde(
        default,
        skip_serializing_if = "is_absent",
        serialize_with = "serialize_presence",
        deserialize_with = "deserialize_presence"
    )]
    pub guild_connections: bool,
}

/// Returns whether a presence-only tag should be omitted.
#[inline]
const fn is_absent(value: &bool) -> bool {
    !*value
}

/// Serializes a presence-only tag as `null`.
#[inline]
fn serialize_presence<S>(_value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_none()
}

/// Deserializes any present value, which Discord documents as `null`, as `true`.
#[inline]
fn deserialize_presence<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let _ = Option::<()>::deserialize(deserializer)?;

    Ok(true)
}
