/*
 * Copyright (c) 2026 Yu-yu0202
 *
 * This Source Code Form is subject to the terms of the
 * Mozilla Public License, v. 2.0. If a copy of the MPL
 * was not distributed with this file, You can obtain one at
 * https://mozilla.org/MPL/2.0/.
 */

//! Serde support for Discord's ISO 8601 timestamps.
//!
//! Discord sends timestamps such as `2023-09-01T14:48:02.222000+00:00`.
//! Deserialization accepts `Z` or a `±HH:MM` offset and up to nine fractional
//! digits. Serialization always emits UTC with a `+00:00` offset, trims trailing
//! fractional zeros, and limits years to `0000..=9999`.
//!
//! Timestamps before the Unix epoch are supported as long as [`SystemTime`] can
//! represent them.

use std::fmt::{self, Write as _};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::ser::Error as _;
use serde::{Deserializer, Serializer};

/// The number of nanoseconds in one second.
const NANOS_PER_SECOND: u128 = 1_000_000_000;

/// Serializes `time` as an ISO 8601 timestamp in UTC.
///
/// Returns an error if the year is outside `0000..=9999` or the value cannot be
/// represented as a Unix timestamp.
pub(crate) fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut output = String::new();
    write!(&mut output, "{}", Iso8601(*time)).map_err(S::Error::custom)?;
    serializer.serialize_str(&output)
}

/// Deserializes an ISO 8601 timestamp into a [`SystemTime`].
///
/// See the [module documentation](self) for the accepted syntax.
pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
where
    D: Deserializer<'de>,
{
    struct TimestampVisitor;

    impl<'de> serde::de::Visitor<'de> for TimestampVisitor {
        type Value = SystemTime;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("an ISO 8601 timestamp")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            parse(value).map_err(|reason| {
                E::custom(format_args!("invalid ISO 8601 timestamp {value:?}: {reason}"))
            })
        }
    }

    deserializer.deserialize_str(TimestampVisitor)
}

/// Serde helpers for optional timestamps.
///
/// `None` is serialized as `null` and `null` is deserialized as `None`; the
/// presence of the field itself is controlled by the surrounding type.
pub(crate) mod option {
    use std::borrow::Cow;
    use std::time::SystemTime;

    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Serializes `time`, emitting `null` for `None`.
    pub(crate) fn serialize<S>(time: &Option<SystemTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match time {
            Some(time) => super::serialize(time, serializer),
            None => serializer.serialize_none(),
        }
    }

    /// Deserializes a timestamp or `null` into an `Option`.
    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Option<SystemTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let Some(value) = Option::<Cow<'de, str>>::deserialize(deserializer)? else {
            return Ok(None);
        };

        super::parse(&value).map(Some).map_err(|reason| {
            D::Error::custom(format_args!("invalid ISO 8601 timestamp {value:?}: {reason}"))
        })
    }
}

/// Formats a [`SystemTime`] as an ISO 8601 timestamp in UTC.
///
/// Fractional seconds are omitted when zero and trailing zeros are trimmed.
struct Iso8601(SystemTime);

impl fmt::Display for Iso8601 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_nanos = unix_nanos(self.0).ok_or(fmt::Error)?;
        let seconds = total_nanos.div_euclid(NANOS_PER_SECOND as i128);
        let nanos = total_nanos.rem_euclid(NANOS_PER_SECOND as i128) as u32;

        let days = seconds.div_euclid(86_400);
        let days = i64::try_from(days).map_err(|_| fmt::Error)?;
        let second_of_day = seconds.rem_euclid(86_400);
        let (year, month, day) = civil_from_days(days);

        if !(0..=9999).contains(&year) {
            return Err(fmt::Error);
        }

        let hour = second_of_day / 3_600;
        let minute = second_of_day % 3_600 / 60;
        let second = second_of_day % 60;

        write!(formatter, "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}")?;

        if nanos != 0 {
            let mut digits = [b'0'; 9];
            let mut remaining = nanos;

            for digit in digits.iter_mut().rev() {
                *digit = b'0' + (remaining % 10) as u8;
                remaining /= 10;
            }

            let length =
                digits.iter().rposition(|&digit| digit != b'0').map_or(0, |index| index + 1);
            let digits = std::str::from_utf8(&digits[..length]).map_err(|_| fmt::Error)?;

            write!(formatter, ".{digits}")?;
        }

        formatter.write_str("+00:00")
    }
}

/// Parses `YYYY-MM-DDTHH:MM:SS[.fraction](Z|±HH:MM)`.
///
/// The fraction accepts one to nine digits. The UTC year after applying the
/// offset must remain in `0000..=9999`. Trailing characters are rejected.
fn parse(value: &str) -> Result<SystemTime, &'static str> {
    let bytes = value.as_bytes();

    if bytes.len() < 20 {
        return Err("timestamp is too short");
    }

    if bytes[4] != b'-'
        || bytes[7] != b'-'
        || !matches!(bytes[10], b'T' | b't')
        || bytes[13] != b':'
        || bytes[16] != b':'
    {
        return Err("expected `YYYY-MM-DDTHH:MM:SS`");
    }

    let year = i64::from(parse_digits(&bytes[0..4])?);
    let month = parse_digits(&bytes[5..7])?;
    let day = parse_digits(&bytes[8..10])?;
    let hour = parse_digits(&bytes[11..13])?;
    let minute = parse_digits(&bytes[14..16])?;
    let second = parse_digits(&bytes[17..19])?;

    let last_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return Err("month is out of range"),
    };

    if day == 0 || day > last_day {
        return Err("day is out of range");
    }

    if hour > 23 {
        return Err("hour is out of range");
    }

    if minute > 59 {
        return Err("minute is out of range");
    }

    if second > 59 {
        return Err("second is out of range");
    }

    let mut index = 19;
    let mut nanos = 0_u32;

    if bytes.get(index) == Some(&b'.') {
        index += 1;

        let mut digits = 0_usize;
        let mut scale = 100_000_000_u32;

        while let Some(&digit) = bytes.get(index) {
            if !digit.is_ascii_digit() {
                break;
            }

            if digits < 9 {
                nanos += u32::from(digit - b'0') * scale;
                scale /= 10;
            }

            digits += 1;
            index += 1;
        }

        if digits == 0 {
            return Err("expected digits after the decimal point");
        }

        if digits > 9 {
            return Err("fractional seconds have more than 9 digits");
        }
    }

    let offset_seconds = match bytes.get(index).copied() {
        Some(b'Z' | b'z') => {
            index += 1;
            0
        },
        Some(sign @ (b'+' | b'-')) => {
            let offset = bytes.get(index + 1..index + 6).ok_or("expected a `±HH:MM` offset")?;

            if offset[2] != b':' {
                return Err("expected a `±HH:MM` offset");
            }

            let offset_hour = parse_digits(&offset[0..2])?;
            let offset_minute = parse_digits(&offset[3..5])?;

            if offset_hour > 23 || offset_minute > 59 {
                return Err("offset is out of range");
            }

            index += 6;

            let offset_seconds = i64::from(offset_hour) * 3_600 + i64::from(offset_minute) * 60;

            if sign == b'-' {
                -offset_seconds
            } else {
                offset_seconds
            }
        },
        _ => return Err("expected `Z` or a `±HH:MM` offset"),
    };

    if index != bytes.len() {
        return Err("unexpected trailing characters");
    }

    let days = days_from_civil(year, month, day);
    let seconds =
        days * 86_400 + i64::from(hour) * 3_600 + i64::from(minute) * 60 + i64::from(second)
            - offset_seconds;
    let (utc_year, _, _) = civil_from_days(seconds.div_euclid(86_400));

    if !(0..=9999).contains(&utc_year) {
        return Err("timestamp year is out of range");
    }

    let total_nanos = i128::from(seconds) * NANOS_PER_SECOND as i128 + i128::from(nanos);

    from_unix_nanos(total_nanos)
}

/// Parses ASCII decimal digits, rejecting any non-digit.
fn parse_digits(digits: &[u8]) -> Result<u32, &'static str> {
    let mut value = 0_u32;

    for &digit in digits {
        if !digit.is_ascii_digit() {
            return Err("expected ASCII digits");
        }

        value = value * 10 + u32::from(digit - b'0');
    }

    Ok(value)
}

/// Returns whether `year` is a leap year in the proleptic Gregorian calendar.
const fn is_leap_year(year: i64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Converts a proleptic Gregorian date to days since the Unix epoch.
///
/// Uses Howard Hinnant's `days_from_civil` algorithm, including years before
/// the epoch.
const fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_prime = (month + 9) % 12;
    let day_of_year = (153 * month_prime + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year as i64;

    era * 146_097 + day_of_era - 719_468
}

/// Converts days since the Unix epoch to a proleptic Gregorian date.
///
/// Uses Howard Hinnant's `civil_from_days` algorithm.
const fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let days = days + 719_468;
    let era = if days >= 0 { days } else { days - 146_096 } / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = if month_prime < 10 {
        month_prime + 3
    } else {
        month_prime - 9
    };

    (if month <= 2 { year + 1 } else { year }, month as u32, day as u32)
}

/// Returns the signed number of nanoseconds from the Unix epoch to `time`.
///
/// Returns `None` if the distance does not fit in an `i128`.
fn unix_nanos(time: SystemTime) -> Option<i128> {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => i128::try_from(duration.as_nanos()).ok(),
        Err(error) => i128::try_from(error.duration().as_nanos()).ok().map(|nanos| -nanos),
    }
}

/// Builds a [`SystemTime`] from signed nanoseconds since the Unix epoch.
///
/// Returns an error if the value exceeds what [`SystemTime`] can represent.
fn from_unix_nanos(total_nanos: i128) -> Result<SystemTime, &'static str> {
    let negative = total_nanos < 0;
    let magnitude = total_nanos.unsigned_abs();
    let seconds =
        u64::try_from(magnitude / NANOS_PER_SECOND).map_err(|_| "timestamp is out of range")?;
    let nanos =
        u32::try_from(magnitude % NANOS_PER_SECOND).map_err(|_| "timestamp is out of range")?;
    let duration = Duration::from_secs(seconds)
        .checked_add(Duration::from_nanos(u64::from(nanos)))
        .ok_or("timestamp is out of range")?;

    let time = if negative {
        UNIX_EPOCH.checked_sub(duration)
    } else {
        UNIX_EPOCH.checked_add(duration)
    };

    time.ok_or("timestamp is out of range")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// Parses `value`, panicking if it is invalid.
    fn parse_timestamp(value: &str) -> SystemTime {
        parse(value).unwrap()
    }

    /// Formats `time` as an ISO 8601 timestamp.
    fn format_timestamp(time: SystemTime) -> String {
        Iso8601(time).to_string()
    }

    /// Tests parsing timestamps with UTC offsets and fractional seconds.
    #[test]
    fn parse_valid_timestamps() {
        assert_eq!(
            parse_timestamp("2023-09-01T14:48:02Z"),
            UNIX_EPOCH + Duration::from_secs(1_693_579_682)
        );
        assert_eq!(
            parse_timestamp("2023-09-01T14:48:02.222000+00:00"),
            UNIX_EPOCH + Duration::new(1_693_579_682, 222_000_000)
        );
        assert_eq!(
            parse_timestamp("2023-09-01T23:48:02+09:00"),
            UNIX_EPOCH + Duration::from_secs(1_693_579_682)
        );
        assert_eq!(
            parse_timestamp("2023-09-01T10:48:02-04:00"),
            UNIX_EPOCH + Duration::from_secs(1_693_579_682)
        );
        assert_eq!(
            parse_timestamp("2023-09-01T00:30:00+09:00"),
            UNIX_EPOCH + Duration::from_secs(1_693_495_800)
        );
        assert_eq!(
            parse_timestamp("1970-01-01T00:30:00+01:00"),
            UNIX_EPOCH - Duration::from_secs(1_800)
        );
        assert_eq!(
            parse_timestamp("2024-02-29T00:00:00.123456789Z"),
            UNIX_EPOCH + Duration::new(1_709_164_800, 123_456_789)
        );
        assert_eq!(parse_timestamp("1969-12-31T23:59:59Z"), UNIX_EPOCH - Duration::from_secs(1));
        assert_eq!(
            parse_timestamp("1969-12-31T23:59:59.5Z"),
            UNIX_EPOCH - Duration::from_millis(500)
        );
    }

    /// Tests formatting timestamps in UTC.
    #[test]
    fn format_timestamps() {
        assert_eq!(format_timestamp(UNIX_EPOCH), "1970-01-01T00:00:00+00:00");
        assert_eq!(
            format_timestamp(UNIX_EPOCH + Duration::new(1_693_579_682, 855_793_000)),
            "2023-09-01T14:48:02.855793+00:00"
        );
        assert_eq!(
            format_timestamp(UNIX_EPOCH + Duration::from_millis(222)),
            "1970-01-01T00:00:00.222+00:00"
        );
        assert_eq!(
            format_timestamp(UNIX_EPOCH - Duration::from_millis(500)),
            "1969-12-31T23:59:59.5+00:00"
        );
    }

    /// Tests that parsing and formatting round-trip.
    #[test]
    fn round_trip() {
        for value in [
            "2023-09-01T14:48:02+00:00",
            "2023-09-01T14:48:02.222000+00:00",
            "1969-12-31T23:59:59.000000001+00:00",
            "1900-01-01T00:00:00+00:00",
            "9999-12-31T23:59:59.999999999+00:00",
        ] {
            let time = parse_timestamp(value);

            assert_eq!(parse_timestamp(&format_timestamp(time)), time);
        }
    }

    /// Tests rejecting invalid timestamps.
    #[test]
    fn parse_invalid_timestamps() {
        for value in [
            "",
            "not a timestamp",
            "2023-09-01 14:48:02Z",
            "2023-13-01T00:00:00Z",
            "2023-00-01T00:00:00Z",
            "2023-02-29T00:00:00Z",
            "2023-04-31T00:00:00Z",
            "2023-09-01T24:00:00Z",
            "2023-09-01T14:60:00Z",
            "2023-09-01T14:48:60Z",
            "2023-09-01T14:48:02",
            "2023-09-01T14:48:02.",
            "2023-09-01T14:48:02+24:00",
            "2023-09-01T14:48:02+00:60",
            "2023-09-01T14:48:02.1234567890Z",
            "0000-01-01T00:00:00+00:01",
            "9999-12-31T23:59:59-00:01",
            "2023-09-01T14:48:02Zjunk",
            "2023-09-01T14:48:02+00:00junk",
        ] {
            assert!(parse(value).is_err(), "{value:?} should be rejected");
        }
    }
}
