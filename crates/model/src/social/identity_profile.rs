/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! The Discord application identity profile resource.
//!
//! Application identity profiles store game data for a user's identity in an
//! application. The data powers Game Stats Widgets on Discord user profiles and
//! is only useful to the application; Discord never consumes it.
//!
//! Written via `PATCH /applications/{application.id}/users/{user.id}/identities/{provider_issued_user_id}/profile`,
//! which returns `201 Created` on the first write and `204 No Content`
//! afterwards.
//!
//! See [Discord's application identity profile resource][resource] for details.
//!
//! [resource]: https://docs.discord.com/developers/resources/application-identity-profile

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::user::UserId;

/// A user's identity for an application, as identified by an external account.
///
/// A request that only writes a profile creates an identity with provider type
/// `NONE` when the user has none yet.
///
/// See [Discord's application identity object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-identity-profile#application-identity-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationIdentity {
    /// The Discord user this identity belongs to.
    ///
    /// Only the identity list endpoints include this field, so it is omitted
    /// during serialization when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    /// The external account provider type.
    pub provider_type: ProviderType,

    /// The provider-specific identifier that disambiguates identities.
    ///
    /// Omitted when absent or empty.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,

    /// The user's ID in the external system.
    pub provider_issued_user_id: String,
}

/// A list of [`ApplicationIdentity`] objects returned by the identity endpoints.
///
/// See [Discord's identity list response][response] for details.
///
/// [response]: https://docs.discord.com/developers/resources/application-identity-profile#get-application-identities-by-user-id
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationIdentityList {
    /// The application identities in the response.
    pub identities: Vec<ApplicationIdentity>,
}

/// The external account provider of an [`ApplicationIdentity`].
///
/// Discord documents the `NONE` provider type for profile-only identities.
/// Values that Cordion does not define are preserved when decoding.
///
/// See [Discord's application identity object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-identity-profile#application-identity-object
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ProviderType(Cow<'static, str>);

impl ProviderType {
    /// The identity only stores profile data for the application.
    pub const NONE: Self = Self(Cow::Borrowed("NONE"));

    /// Creates an owned provider type from any string-like value.
    ///
    /// Prefer [`ProviderType::from`] with a `&'static str` to borrow instead of
    /// allocating.
    #[inline]
    #[must_use]
    pub fn new<T: Into<String>>(value: T) -> Self {
        Self(Cow::Owned(value.into()))
    }

    /// Returns the provider type exactly as Discord sent it.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for ProviderType {
    #[inline]
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

/// A user's application identity profile.
///
/// [`username`](Self::username), [`metadata`](Self::metadata), and
/// [`data`](Self::data) are nullable and are serialized as `null` when `None`.
/// [`metadata`](Self::metadata) is arbitrary application-defined JSON, so it is
/// kept as a raw [`serde_json::Value`].
///
/// See [Discord's application identity profile object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-identity-profile#application-identity-profile-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationIdentityProfile {
    /// The user's username in the external system.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub username: Option<String>,

    /// Arbitrary game-defined data that Discord stores without interpreting.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub metadata: Option<Value>,

    /// The profile data containing game stats.
    ///
    /// Discord documents this field as nullable, so `None` is serialized as
    /// `null`.
    #[serde(default)]
    pub data: Option<ProfileData>,
}

/// The profile data stored for an [`ApplicationIdentityProfile`].
///
/// A write replaces the whole `data` object, so omitted fields are lost. The
/// serialized payload must stay under 10 KB and may contain at most 30 dynamic
/// fields.
///
/// See [Discord's profile data object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-identity-profile#profile-data-object-profile-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct ProfileData {
    /// The pre-configured game stat fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<PrimaryProfileData>,

    /// The custom game stat fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<Vec<DynamicField>>,
}

/// The pre-configured game stat fields of [`ProfileData`].
///
/// Every field is optional, and unrecognized fields that Discord may add later
/// are ignored.
///
/// See [Discord's primary profile data object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-identity-profile#primary-profile-data-object-primary-profile-data-structure
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct PrimaryProfileData {
    /// The current season name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<String>,

    /// The current rank name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_name: Option<String>,

    /// The image representing the current rank.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_image: Option<Media>,

    /// The highest rank achieved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_rank: Option<String>,

    /// The image representing the highest rank achieved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_rank_image: Option<Media>,

    /// The name of the featured played character.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_played_character: Option<String>,

    /// The image of the featured played character.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_played_character_image: Option<Media>,

    /// The total playtime in hours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playtime_hours: Option<f64>,

    /// The total number of wins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_wins: Option<u64>,

    /// The number of wins in the current period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_wins: Option<u64>,

    /// The total number of games played.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_games: Option<u64>,

    /// The number of games played in the current period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_games: Option<u64>,

    /// The total number of kills.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_kills: Option<u64>,

    /// The number of kills in the current period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_kills: Option<u64>,

    /// The total number of assists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_assists: Option<u64>,

    /// The number of assists in the current period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_assists: Option<u64>,

    /// The total number of deaths.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_deaths: Option<u64>,

    /// The number of deaths in the current period.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_deaths: Option<u64>,
}

/// A custom game stat field.
///
/// The value shape depends on [`kind`](Self::kind), so [`value`](Self::value) is
/// modeled as [`DynamicFieldValue`]. Unrecognized types are preserved.
///
/// See [Discord's dynamic field object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-identity-profile#dynamic-field-object
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct DynamicField {
    /// The field's type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: DynamicFieldType,

    /// The data key referenced by the widget editor.
    ///
    /// This is not shown to players and is at most 100 characters.
    pub name: String,

    /// The field's value.
    pub value: DynamicFieldValue,
}

/// The type of a [`DynamicField`].
///
/// Unrecognized values are preserved. See [Discord's dynamic field types][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/resources/application-identity-profile#dynamic-field-object-dynamic-field-types
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct DynamicFieldType(u8);

impl DynamicFieldType {
    /// A text value.
    pub const STRING: Self = Self(1);

    /// A numeric value.
    pub const NUMBER: Self = Self(2);

    /// A media object value.
    pub const MEDIA: Self = Self(3);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}

/// The value of a [`DynamicField`].
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum DynamicFieldValue {
    /// A text value.
    String(String),

    /// A numeric value.
    Number(f64),

    /// A media object value.
    Media(Media),
}

/// A media asset referenced by a profile field.
///
/// Media URLs must be reachable from the public internet, because Discord
/// fetches them from its servers.
///
/// See [Discord's media object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-identity-profile#media-object-media-structure
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Media {
    /// The URL of the media asset.
    pub url: String,
}

/// The body of the profile update endpoint.
///
/// Both fields are optional. Including [`data`](Self::data) replaces the stored
/// profile data entirely, so the request must always send the complete set of
/// fields to keep.
///
/// See [Discord's update application identity profile endpoint][endpoint] for
/// details.
///
/// [endpoint]: https://docs.discord.com/developers/resources/application-identity-profile#update-application-identity-profile
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct UpdateApplicationIdentityProfileRequest {
    /// The user's username in the external system.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// The profile data to write.
    ///
    /// Omitting this field leaves the existing profile data untouched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ProfileData>,
}
