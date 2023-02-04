// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for parsing, validating, manipulating, and formatting dates and times
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-date.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! The `Date` library provides drop-in replacement methods for parsing,
//! validating, manipulating, and formatting dates and times in Rust.
//!
//! The [**`DateTime`**](./struct.DateTime.html) type to represent a
//! date and a time in a defined timezone.
//!
//! ## Features
//!
//! - Day of the month: (1-31),
//! - Hour of the day: (0-23),
//! - ISO 8601 date and time: (e.g. "2023-01-01T00:00:00+00:00"),
//! - ISO week number: (1-53),
//! - Microsecond: (0-999999),
//! - Minute of the hour: (0-59),
//! - Month: (e.g. "January"),
//! - Now object: (e.g. "2023-01-01"),
//! - Offset from UTC: (e.g. "+00:00"),
//! - Ordinal date: (1-366),
//! - Second of the minute: (0-59),
//! - Time object: (e.g. "00:00:00"),
//! - Tz object: (e.g. "UTC"),
//! - Weekday object: (e.g. "Monday"),
//! - Year object: (e.g. "2023"),
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde,
//! usually derived with `serde_derive`.
//!
//! ## Examples
//!
//! ```rust
//!
//! extern crate date;
//! use self::date::DateTime;
//!
//! let dt = DateTime::new();
//! println!("Date: {}", dt.now);
//! println!("Day: {}", dt.day);
//! println!("Hour: {}", dt.hour);
//! println!("ISO 8601: {}", dt.iso_8601);
//! println!("ISO Week Number: {}", dt.iso_week);
//! println!("Minute: {}", dt.minute);
//! println!("Month: {}", dt.month);
//! println!("Offset: {}", dt.offset);
//! println!("Ordinal Date: {}", dt.ordinal);
//! println!("Second: {}", dt.second);
//! println!("Time: {}", dt.time);
//! println!("Weekday: {}", dt.weekday);
//! println!("Year: {}", dt.year);
//!
//! ```
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-date.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-date.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "date"]
#![crate_type = "lib"]

extern crate serde;
use std::fmt;

pub use serde::{Deserialize, Serialize};

extern crate time;
use time::{Duration, OffsetDateTime};

/// # DateTime
///
/// [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/claims)
/// [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
///
/// DateTime struct to ease dates and times manipulation.
///
/// This module includes date and time types, such as day, hour,ISO 8601
/// date and time, and many more methods.
///
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DateTime {
    /// Day of the month: (1-31)
    pub day: u8,
    /// Hour of the day: (0-23)
    pub hour: u8,
    /// ISO 8601 date and time: (e.g. "2023-01-01T00:00:00+00:00")
    pub iso_8601: String,
    /// ISO week number: (1-53)
    pub iso_week: u8,
    /// Microsecond: (0-999999)
    pub microsecond: u32,
    /// Minute of the hour: (0-59)
    pub minute: u8,
    /// Month: (e.g. "January")
    pub month: String,
    /// Now object: (e.g. "2023-01-01")
    pub now: String,
    /// Offset from UTC: (e.g. "+00:00")
    pub offset: String,
    /// Ordinal date: (1-366)
    pub ordinal: u16,
    /// Second of the minute: (0-59)
    pub second: u8,
    /// Time object: (e.g. "00:00:00")
    pub time: String,
    /// Tz object: (e.g. "UTC")
    pub tz: String,
    /// Weekday object: (e.g. "Monday")
    pub weekday: String,
    /// Year object: (e.g. "2023")
    pub year: i32,
}

impl DateTime {
    /// Create a new Date object with UTC timezone.
    pub fn new() -> Self {
        Self::new_with_tz("UTC")
    }

    /// Create a new DateTime object with a custom timezone.
    pub fn new_with_tz(tz: &str) -> Self {
        let offset = match tz {
            "UTC" => time::UtcOffset::UTC,
            _ => time::UtcOffset::from_hms(0, 0, 0).unwrap(),
        };
        let now_utc = match tz {
            "UTC" => OffsetDateTime::now_utc(),
            _ => {
                let (hours, minutes, _) = offset.as_hms();
                let total_seconds = (hours as i16 * 3600) + (minutes as i16 * 60);
                OffsetDateTime::now_utc() + Duration::seconds(total_seconds as i64)
            }
        };
        let iso_8601 = now_utc.to_string();

        Self {
            day: now_utc.day(),
            hour: now_utc.hour(),
            iso_8601,
            iso_week: now_utc.iso_week(),
            microsecond: now_utc.microsecond(),
            minute: now_utc.minute(),
            month: now_utc.month().to_string(),
            now: now_utc.date().to_string(),
            offset: now_utc.offset().to_string(),
            ordinal: now_utc.ordinal(),
            second: now_utc.second(),
            time: now_utc.time().to_string(),
            tz: now_utc.time().to_string(),
            weekday: now_utc.weekday().to_string(),
            year: now_utc.year(),
        }
    }
    /// Check if the input is a valid day.
    /// 31 is valid.
    /// 32 is not valid.
    pub fn is_valid_day(input: &str) -> bool {
        let mut valid = false;
        if let Ok(day) = input.parse::<u8>() {
            if (1..=31).contains(&day) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid hour.
    /// 24:00 is valid.
    /// 24:01 is not valid.
    pub fn is_valid_hour(input: &str) -> bool {
        let mut valid = false;
        if let Ok(hour) = input.parse::<u8>() {
            if (0..=24).contains(&hour) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid ISO 8601 date and time.
    /// 2023-01-01T00:00:00+00:00 is valid.
    /// 2023-01-01T00:00:00+00:01 is not valid.
    pub fn is_valid_iso_8601(input: &str) -> bool {
        let mut iter = input.splitn(6, '-');

        let year = iter
            .next()
            .and_then(|year_str| year_str.parse::<i32>().ok());
        let month = iter
            .next()
            .and_then(|month_str| month_str.parse::<u32>().ok());
        let day = iter.next().and_then(|day_str| day_str.parse::<u32>().ok());
        let hour = iter.next().and_then(|hour_str| {
            hour_str
                .split(':')
                .next()
                .and_then(|hour_str| hour_str.parse::<u32>().ok())
        });
        let minute = iter.next().and_then(|minute_str| {
            minute_str
                .split(':')
                .next()
                .and_then(|minute_str| minute_str.parse::<u32>().ok())
        });
        let second = iter.next().and_then(|second_str| {
            second_str
                .split('.')
                .next()
                .and_then(|second_str| second_str.parse::<u32>().ok())
        });

        match (year, month, day, hour, minute, second) {
            (Some(y), Some(m), Some(d), Some(h), Some(min), Some(s)) => {
                y >= 1
                    && (1..=12).contains(&m)
                    && (1..=31).contains(&d)
                    && h <= 23
                    && min <= 59
                    && s <= 59
            }
            _ => false,
        }
    }

    /// Check if the input is a valid ISO week number.
    /// 53 is valid.
    /// 54 is not valid.
    pub fn is_valid_iso_week(input: &str) -> bool {
        let mut valid = false;
        if let Ok(iso_week) = input.parse::<u8>() {
            if (1..=53).contains(&iso_week) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid microsecond.
    /// 999999 is valid.
    /// 1000000 is not valid.
    pub fn is_valid_microsecond(input: &str) -> bool {
        let mut valid = false;
        if let Ok(microsecond) = input.parse::<u32>() {
            if (0..=999999).contains(&microsecond) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid minute.
    /// 59 is valid.
    /// 60 is not valid.
    pub fn is_valid_minute(input: &str) -> bool {
        let mut valid = false;
        if let Ok(minute) = input.parse::<u8>() {
            if (0..=59).contains(&minute) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid month.
    /// 12 is valid.
    /// 13 is not valid.
    pub fn is_valid_month(input: &str) -> bool {
        let mut valid = false;
        if let Ok(month) = input.parse::<u8>() {
            if (1..=12).contains(&month) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid ordinal date.
    /// 366 is valid.
    /// 367 is not valid.
    pub fn is_valid_ordinal(input: &str) -> bool {
        let mut valid = false;
        if let Ok(ordinal) = input.parse::<u16>() {
            if (1..=366).contains(&ordinal) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid second.
    /// 59 is valid.
    /// 60 is not valid.
    pub fn is_valid_second(input: &str) -> bool {
        let mut valid = false;
        if let Ok(second) = input.parse::<u8>() {
            if (0..=59).contains(&second) {
                valid = true;
            }
        }
        valid
    }
    /// Check if the input is a valid time.
    /// 23:59:59 is valid.
    /// 24:00:00 is not valid.
    pub fn is_valid_time(input: &str) -> bool {
        let mut valid = false;
        if let Ok(_time) = input.parse::<u32>() {
            valid = true;
        }
        valid
    }
}

impl fmt::Display for DateTime {
    /// Display the date and time in ISO 8601 format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}T{}:{:02}:{:02}{}",
            self.now, self.hour, self.minute, self.second, self.offset
        )
    }
}

/// Generates a function with the given name `$name` that checks whether
/// a string `input` can be parsed into a value of type `$type`.
///
/// # Examples
///
/// ```rust
///
/// extern crate date;
/// use date::is_valid;
///
/// is_valid!(day, u32);
/// let input = "31";
/// let result = day(input);
/// assert!(result);
///
/// ```
#[macro_export]
macro_rules! is_valid {
    ($name:ident, $type:ty) => {
        fn $name(input: &str) -> bool {
            let mut valid = false;
            if let Ok(_input) = input.parse::<$type>() {
                valid = true;
            }
            valid
        }
    };
}
