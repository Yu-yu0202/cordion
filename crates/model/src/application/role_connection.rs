/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

use serde::{Deserialize, Serialize};

use crate::application_command::Localizations;

/// One role connection metadata record for an application.
///
/// Applications can define up to five records. Each record describes a metadata
/// value that guilds can use as a role requirement. See [Discord's role
/// connection metadata object][object] for details.
///
/// [object]: https://docs.discord.com/developers/resources/application-role-connection-metadata#application-role-connection-metadata-object
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct ApplicationRoleConnectionMetadata {
    /// The metadata value type, serialized as `type`.
    #[serde(rename = "type")]
    pub kind: ApplicationRoleConnectionMetadataType,

    /// The dictionary key for the metadata field.
    ///
    /// Discord requires 1-50 characters from `a-z`, `0-9`, or `_`.
    pub key: String,

    /// The metadata field's name, between 1 and 100 characters.
    pub name: String,

    /// Localized variants of [`name`](Self::name).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<Localizations>,

    /// The metadata field's description, between 1 and 200 characters.
    pub description: String,

    /// Localized variants of [`description`](Self::description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<Localizations>,
}

/// The comparison operation for an [`ApplicationRoleConnectionMetadata`] value.
///
/// Unrecognized values are preserved. See [Discord's metadata type table][types]
/// for details.
///
/// [types]: https://docs.discord.com/developers/resources/application-role-connection-metadata#application-role-connection-metadata-object-application-role-connection-metadata-type
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct ApplicationRoleConnectionMetadataType(u8);

impl ApplicationRoleConnectionMetadataType {
    /// The metadata integer is less than or equal to the guild's value.
    pub const INTEGER_LESS_THAN_OR_EQUAL: Self = Self(1);

    /// The metadata integer is greater than or equal to the guild's value.
    pub const INTEGER_GREATER_THAN_OR_EQUAL: Self = Self(2);

    /// The metadata integer is equal to the guild's value.
    pub const INTEGER_EQUAL: Self = Self(3);

    /// The metadata integer is not equal to the guild's value.
    pub const INTEGER_NOT_EQUAL: Self = Self(4);

    /// The metadata date is less than or equal to the guild's configured value.
    pub const DATETIME_LESS_THAN_OR_EQUAL: Self = Self(5);

    /// The metadata date is greater than or equal to the guild's configured
    /// value.
    pub const DATETIME_GREATER_THAN_OR_EQUAL: Self = Self(6);

    /// The metadata boolean is equal to the guild's value.
    pub const BOOLEAN_EQUAL: Self = Self(7);

    /// The metadata boolean is not equal to the guild's value.
    pub const BOOLEAN_NOT_EQUAL: Self = Self(8);

    /// Returns the raw integer value Discord sent.
    #[inline]
    #[must_use]
    pub const fn raw(self) -> u8 {
        self.0
    }
}
