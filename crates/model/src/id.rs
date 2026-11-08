/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Type-safe snowflake IDs.
//!
//! Discord serializes snowflakes as decimal strings. [`Id<T>`] accepts either a
//! string or an integer during deserialization and always serializes as a
//! decimal string.

use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::num::ParseIntError;
use std::str::FromStr;

use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A marker trait for resource types that can be used as `Id<T>`.
///
/// Implementing this trait prevents IDs for unrelated resources from being
/// mixed up at compile time.
///
/// # Examples
///
/// ```
/// use cordion_model::id::{Id, IdTarget};
///
/// struct MyResource;
///
/// impl IdTarget for MyResource {}
///
/// let my_resource_id = Id::<MyResource>::new(123);
/// ```
///
/// # Derive macro
///
/// Use [`cordion_macro::IdTarget`] to implement this trait automatically.
///
/// ```
/// use cordion_macro::IdTarget;
/// use cordion_model::id::Id;
///
/// #[derive(IdTarget)]
/// struct MyResource;
/// ```
pub trait IdTarget {}

/// A resource that exposes its own ID.
///
/// This enables code that works with any resource type without knowing the
/// concrete model.
///
/// # Examples
///
/// ```
/// use cordion_macro::IdTarget;
/// use cordion_model::id::{Id, Identifiable};
///
/// #[derive(IdTarget)]
/// struct MyResource {
///     id: Id<MyResource>,
/// }
///
/// impl Identifiable for MyResource {
///     fn id(&self) -> Id<Self> {
///         self.id
///     }
/// }
///
/// let resource = MyResource { id: Id::<MyResource>::new(123) };
/// assert_eq!(resource.id(), Id::<MyResource>::new(123));
/// ```
pub trait Identifiable: IdTarget + Sized {
    /// Returns the ID of the resource.
    fn id(&self) -> Id<Self>;
}

/// A snowflake ID associated with resource type `T`.
///
/// The type parameter is a marker only: `Id<User>` and `Id<Guild>` share the
/// same representation but cannot be used interchangeably.
///
/// Discord normally uses non-zero snowflakes, but this type accepts any `u64`,
/// including `0`, because anonymized IDs may contain it.
#[repr(transparent)]
pub struct Id<T: IdTarget> {
    /// The raw 64-bit snowflake value.
    id: u64,

    /// Associates the ID with `T` without storing a `T` or affecting variance.
    phantom: PhantomData<fn() -> T>,
}

impl<T: IdTarget> Id<T> {
    /// Creates an ID from its raw `u64` value.
    ///
    /// No snowflake structure is validated; the timestamp, worker, and sequence
    /// parts are not inspected.
    ///
    /// # Examples
    ///
    /// ```
    /// use cordion_model::id::Id;
    /// use cordion_model::user::User;
    ///
    /// let id = Id::<User>::new(1456247955456000000);
    /// assert_eq!(id.get(), 1456247955456000000);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(id: u64) -> Id<T> {
        Self { id, phantom: PhantomData }
    }

    /// Returns the raw `u64` snowflake value.
    ///
    /// # Examples
    ///
    /// ```
    /// use cordion_model::id::Id;
    /// use cordion_model::user::User;
    ///
    /// let id = Id::<User>::new(123);
    /// assert_eq!(id.get(), 123);
    /// ```
    #[inline]
    #[must_use]
    pub const fn get(&self) -> u64 {
        self.id
    }

    /// Reinterprets the same numeric ID as resource type `U`.
    ///
    /// This performs no lookup and does not verify that a resource of type `U`
    /// exists. A common use is deriving the `@everyone` role ID from a guild ID,
    /// since both share the same snowflake.
    ///
    /// # Examples
    ///
    /// ```
    /// use cordion_model::guild::Guild;
    /// use cordion_model::id::Id;
    /// use cordion_model::role::Role;
    ///
    /// let guild_id = Id::<Guild>::new(123);
    /// let role_id: Id<Role> = guild_id.cast();
    /// assert_eq!(guild_id.get(), role_id.get());
    /// ```
    #[inline]
    #[must_use]
    pub const fn cast<U: IdTarget>(self) -> Id<U> {
        Id::<U>::new(self.id)
    }
}

impl<T: IdTarget> Clone for Id<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: IdTarget> Copy for Id<T> {}

impl<T: IdTarget> PartialEq for Id<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: IdTarget> Eq for Id<T> {}

impl<T: IdTarget> PartialOrd for Id<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: IdTarget> Ord for Id<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T: IdTarget> Hash for Id<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T: IdTarget> From<u64> for Id<T> {
    #[inline]
    fn from(id: u64) -> Self {
        Self::new(id)
    }
}

impl<T: IdTarget> FromStr for Id<T> {
    type Err = ParseIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u64::from_str(s).map(Self::new)
    }
}

impl<T: IdTarget> From<Id<T>> for u64 {
    #[inline]
    fn from(id: Id<T>) -> Self {
        id.get()
    }
}

impl<T: IdTarget> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.id, f)
    }
}

impl<T: IdTarget> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        write!(f, "Id<{type_name}>({})", self.id)
    }
}

impl<T: IdTarget> Serialize for Id<T> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de, T: IdTarget> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct IdVisitor<T>(PhantomData<fn() -> T>);

        impl<'de, T: IdTarget> serde::de::Visitor<'de> for IdVisitor<T> {
            type Value = Id<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a Discord snowflake as a decimal string or integer")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let unsigned = u64::try_from(value)
                    .map_err(|_| E::invalid_value(Unexpected::Signed(value), &"a u64 snowflake"))?;

                self.visit_u64(unsigned)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Id::<T>::new(value))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let value = u64::from_str(value)
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &"a u64 snowflake"))?;

                self.visit_u64(value)
            }
        }

        deserializer.deserialize_any(IdVisitor(PhantomData))
    }
}

/// Serde support for snowflakes represented as raw `u64` values.
///
/// Some Discord fields use the snowflake wire format but cannot be tied to a
/// single resource type, such as application command versions and permission
/// targets. These helpers serialize a `u64` as a decimal string and accept a
/// decimal string or integer when deserializing, matching [`Id<T>`].
pub(crate) mod snowflake {
    use std::fmt;
    use std::str::FromStr;

    use serde::de::{Error, Unexpected, Visitor};
    use serde::{Deserializer, Serializer};

    /// Serializes `value` as a decimal string.
    pub(crate) fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    /// Deserializes a decimal string or integer into a `u64`.
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SnowflakeVisitor;

        impl<'de> Visitor<'de> for SnowflakeVisitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a Discord snowflake as a decimal string or integer")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let unsigned = u64::try_from(value)
                    .map_err(|_| E::invalid_value(Unexpected::Signed(value), &"a u64 snowflake"))?;

                self.visit_u64(unsigned)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(value)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let value = u64::from_str(value)
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &"a u64 snowflake"))?;

                self.visit_u64(value)
            }
        }

        deserializer.deserialize_any(SnowflakeVisitor)
    }
}

/// Serde support for optional snowflakes represented as raw `u64` values.
pub(crate) mod snowflake_option {
    use std::fmt;

    use serde::de::{Error, Visitor};
    use serde::{Deserializer, Serializer};

    use super::snowflake;

    /// Serializes an optional `u64` as a decimal string or `null`.
    pub(crate) fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => snowflake::serialize(value, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes an optional decimal string or integer into `Option<u64>`.
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct OptionalSnowflakeVisitor;

        impl<'de> Visitor<'de> for OptionalSnowflakeVisitor {
            type Value = Option<u64>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an optional Discord snowflake as a decimal string or integer")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(None)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                snowflake::deserialize(deserializer).map(Some)
            }
        }

        deserializer.deserialize_option(OptionalSnowflakeVisitor)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    impl IdTarget for () {}
    impl IdTarget for u64 {}

    /// Tests construction through [`Id::new`], `From<u64>`, and `FromStr`.
    #[test]
    fn construct() {
        // `Id::new`
        assert_eq!(Id::<()>::new(123).get(), 123);

        // `From<u64>`
        assert_eq!(Id::<()>::from(123_u64).get(), 123);

        // `FromStr`
        assert_eq!(Id::<()>::from_str("123").unwrap().get(), 123);
        // `FromStr` with invalid input
        assert!(Id::<()>::from_str("invalid").is_err());
    }

    /// Tests [`Id::get`] and [`Id::cast`] conversions.
    #[test]
    fn conversions() {
        const ID: Id<()> = Id::<()>::new(123);

        // `Id::get`
        assert_eq!(ID.get(), 123_u64);

        // `Id::cast`
        let casted: Id<u64> = ID.cast();
        assert_eq!(ID.cast::<u64>(), casted);
    }

    /// Tests the `Display` and `Debug` implementations.
    #[test]
    fn display_and_debug() {
        const ID: Id<()> = Id::<()>::new(123);

        // `Display`
        assert_eq!(format!("{}", ID), "123");

        // `Debug`
        assert_eq!(format!("{:?}", ID), "Id<()>(123)");
    }
}
