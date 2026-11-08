/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Wire-format tests for SKUs, subscriptions, and entitlements.

#![allow(clippy::unwrap_used)]

mod wire_common;
use cordion_model::monetization::entitlement::{
    CreateTestEntitlementRequest, Entitlement, EntitlementOwnerType, EntitlementType,
};
use cordion_model::monetization::sku::{Sku, SkuFlags, SkuKind};
use cordion_model::monetization::subscription::{Subscription, SubscriptionStatus};
use serde_json::Value;
use wire_common::*;

/// Tests the example SKU from Discord's SKU documentation.
#[test]
fn sku_matches_discord_example() {
    let sku: Sku = serde_json::from_str(
        r#"{
            "id": "1088510058284990888",
            "type": 5,
            "dependent_sku_id": null,
            "application_id": "788708323867885999",
            "manifest_labels": null,
            "access_type": 1,
            "name": "Test Premium",
            "features": [],
            "release_date": null,
            "premium": false,
            "slug": "test-premium",
            "flags": 128,
            "show_age_gate": false
        }"#,
    )
    .unwrap();

    assert_eq!(sku.id.get(), 1_088_510_058_284_990_888);
    assert_eq!(sku.kind, SkuKind::SUBSCRIPTION);
    assert_eq!(sku.application_id.get(), 788_708_323_867_885_999);
    assert_eq!(sku.flags, SkuFlags::GUILD_SUBSCRIPTION);
}

/// Tests that unknown SKU kinds and flag bits are preserved.
#[test]
fn sku_kind_and_flags_preserve_unknown_values() {
    let kind: SkuKind = serde_json::from_str("42").unwrap();
    assert_eq!(serde_json::to_value(kind).unwrap(), serde_json::json!(42));

    let bits = SkuFlags::AVAILABLE.bits() | (1 << 9);
    let flags: SkuFlags = serde_json::from_str(&bits.to_string()).unwrap();
    assert!(flags.contains(SkuFlags::AVAILABLE));
    assert_eq!(flags.bits(), bits);
    assert_eq!(serde_json::to_value(flags).unwrap(), serde_json::json!(bits));
}

/// Tests the entitlement example from Discord's entitlement documentation.
#[test]
fn entitlement_matches_discord_example() {
    let entitlement: Entitlement = serde_json::from_str(
        r#"{
            "id": "1019653849998299136",
            "sku_id": "1019475255913222144",
            "application_id": "1019370614521200640",
            "user_id": "771129655544643584",
            "promotion_id": null,
            "type": 8,
            "deleted": false,
            "gift_code_flags": 0,
            "consumed": false,
            "starts_at": "2022-09-14T17:00:18.704163+00:00",
            "ends_at": "2022-10-14T17:00:18.704163+00:00",
            "guild_id": "1015034326372454400",
            "subscription_id": "1019653835926409216"
        }"#,
    )
    .unwrap();

    assert_eq!(entitlement.kind, EntitlementType::APPLICATION_SUBSCRIPTION);
    assert_eq!(entitlement.starts_at, Some(timestamp(1_663_174_818, 704_163_000)));
    assert_eq!(entitlement.ends_at, Some(timestamp(1_665_766_818, 704_163_000)));
    assert_eq!(entitlement.user_id.unwrap().get(), 771_129_655_544_643_584);
    assert_eq!(entitlement.guild_id.unwrap().get(), 1_015_034_326_372_454_400);

    let value = serde_json::to_value(entitlement).unwrap();
    assert_eq!(value["id"], "1019653849998299136");
    assert_eq!(value["type"], 8);
    assert_eq!(value["starts_at"], "2022-09-14T17:00:18.704163+00:00");

    let minimal: Entitlement = serde_json::from_str(
        r#"{
            "id": "1",
            "sku_id": "2",
            "application_id": "3",
            "type": 1,
            "deleted": false
        }"#,
    )
    .unwrap();
    assert_eq!(minimal.starts_at, None);
    assert_eq!(minimal.ends_at, None);

    let value = serde_json::to_value(minimal).unwrap();
    assert_eq!(value["starts_at"], Value::Null);
    assert_eq!(value["ends_at"], Value::Null);
    assert!(value.get("consumed").is_none());
}

/// Tests the subscription example from Discord's subscription resource.
#[test]
fn subscription_matches_discord_example() {
    let subscription: Subscription = serde_json::from_str(
        r#"{
            "id": "1278078770116427839",
            "user_id": "1088605110638227537",
            "sku_ids": ["1158857122189168803"],
            "entitlement_ids": ["1158857122189168804"],
            "renewal_sku_ids": null,
            "current_period_start": "2024-08-27T19:48:44.406602+00:00",
            "current_period_end": "2024-09-27T19:48:44.406602+00:00",
            "status": 0,
            "canceled_at": null
        }"#,
    )
    .unwrap();

    assert_eq!(subscription.id.get(), 1_278_078_770_116_427_839);
    assert_eq!(subscription.user_id.get(), 1_088_605_110_638_227_537);
    assert_eq!(subscription.sku_ids.len(), 1);
    assert_eq!(subscription.sku_ids[0].get(), 1_158_857_122_189_168_803);
    assert_eq!(subscription.entitlement_ids.len(), 1);
    assert_eq!(subscription.entitlement_ids[0].get(), 1_158_857_122_189_168_804);
    assert!(subscription.renewal_sku_ids.is_none());
    assert_eq!(subscription.status, SubscriptionStatus::ACTIVE);
    assert!(subscription.canceled_at.is_none());
    assert!(subscription.country.is_none());

    let value = serde_json::to_value(&subscription).unwrap();
    assert_eq!(value["id"], "1278078770116427839");
    assert_eq!(value["sku_ids"][0], "1158857122189168803");
    assert!(value["renewal_sku_ids"].is_null());
    assert_eq!(value["current_period_start"], "2024-08-27T19:48:44.406602+00:00");
    assert_eq!(value["status"], 0);
    assert!(value["canceled_at"].is_null());
    assert!(value.get("country").is_none());
}

/// Tests that subscription statuses preserve unknown values.
#[test]
fn subscription_status_preserves_unknown_values() {
    let status: SubscriptionStatus = serde_json::from_str("9").unwrap();

    assert_eq!(status.raw(), 9);
    assert_eq!(serde_json::to_value(status).unwrap(), serde_json::json!(9));
}

/// Tests the test entitlement request.
#[test]
fn create_test_entitlement_matches_documented_fields() {
    let request: CreateTestEntitlementRequest = serde_json::from_str(
        r#"{"sku_id": "1158857122189168803", "owner_id": "41771983429993000", "owner_type": 2}"#,
    )
    .unwrap();

    assert_eq!(request.sku_id.get(), 1_158_857_122_189_168_803);
    assert_eq!(request.owner_id, 41_771_983_429_993_000);
    assert_eq!(request.kind, EntitlementOwnerType::USER);

    let value = serde_json::to_value(request).unwrap();
    assert_eq!(value["sku_id"], "1158857122189168803");
    assert_eq!(value["owner_id"], "41771983429993000");
    assert_eq!(value["owner_type"], 2);
}
