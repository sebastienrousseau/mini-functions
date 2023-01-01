//! # Core date functionality
//!
//! Date provides an easy way to get the current date and time in multiple formats.
//!
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`Date`] to get the current date and time in UTC.
use time::OffsetDateTime;

/// Date Utility
///
/// By default, the current date and time in UTC is returned.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Date {}

impl Date {
    /// Get the Date in the stored offset.
    pub fn date() -> String {
        OffsetDateTime::now_utc().date().to_string()
    }

    /// Get the day of the date in the stored offset.
    ///
    /// The day is a number from 1 to 31.
    pub fn day() -> String {
        OffsetDateTime::now_utc().day().to_string()
    }
    /// Get the clock hour in the stored offset.
    ///
    /// The hour is a number from 0 to 23. 0 is midnight. 12 is noon.
    pub fn hour() -> String {
        OffsetDateTime::now_utc().hour().to_string()
    }
    /// Get the ISO week number of the date in the stored offset.
    ///
    /// The returned value will always be in the range 1..=53.
    pub fn iso_week() -> String {
        OffsetDateTime::now_utc().iso_week().to_string()
    }
    /// Get the microsecond of the second in the stored offset.
    ///
    /// The returned value will always be in the range 0..1_000_000.
    pub fn microsecond() -> String {
        OffsetDateTime::now_utc().microsecond().to_string()
    }
    /// Get the milliseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range 0..1_000.
    pub fn millisecond() -> String {
        OffsetDateTime::now_utc().millisecond().to_string()
    }
    /// Get the minute within the hour in the stored offset.
    ///
    /// The returned value will always be in the range 0..60.
    pub fn minute() -> String {
        OffsetDateTime::now_utc().minute().to_string()
    }
    /// Get the month of the date in the stored offset.
    ///
    /// The month is a number from 1 to 12.
    /// January is 1, February is 2, and so on.
    pub fn month() -> String {
        OffsetDateTime::now_utc().month().to_string()
    }
    /// Get the nanoseconds within the second in the stored offset.
    ///
    /// The returned value will always be in the range 0..1_000_000_000.
    pub fn nanosecond() -> String {
        OffsetDateTime::now_utc().nanosecond().to_string()
    }
    /// Create a new OffsetDateTime in the ISO 8601 format.
    pub fn iso_8601() -> String {
        let now = OffsetDateTime::now_utc();
        let iso_8601 = format!(
            "{}T{:02}:{:02}:{:02}.{:03}{:+03}{:02}",
            now.date(),
            now.hour(),
            now.minute(),
            now.second(),
            now.millisecond(),
            now.offset().whole_hours(),
            now.offset().minutes_past_hour()
        );
        iso_8601
    }
    /// Create a new OffsetDateTime with the current date and time (UTC)
    pub fn now_utc() -> String {
        OffsetDateTime::now_utc().to_string()
    }
    /// Get the UtcOffset.
    pub fn offset() -> String {
        OffsetDateTime::now_utc().offset().to_string()
    }
    /// Get the day of the year of the date in the stored offset.
    ///
    /// The returned value will always be in the range 1..=366.
    pub fn ordinal() -> String {
        OffsetDateTime::now_utc().ordinal().to_string()
    }
    /// Get the second within the minute in the stored offset.
    ///
    /// The returned value will always be in the range 0..60.
    pub fn second() -> String {
        OffsetDateTime::now_utc().second().to_string()
    }
    /// Get the Time in the stored offset.
    pub fn time() -> String {
        OffsetDateTime::now_utc().time().to_string()
    }
    /// Get the Unix timestamp.
    pub fn timestamp() -> String {
        OffsetDateTime::now_utc().unix_timestamp().to_string()
    }
    /// Get the weekday of the date in the stored offset.
    ///
    /// This current uses Zellers congruence internally.
    pub fn weekday() -> String {
        OffsetDateTime::now_utc().weekday().to_string()
    }
    /// Get the year of the date in the stored offset.
    pub fn year() -> String {
        OffsetDateTime::now_utc().year().to_string()
    }
}
