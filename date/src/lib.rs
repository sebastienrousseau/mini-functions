//!
//! # Core Date functionality
//!
//! Provides access to the current date and time. It's faster and uses
//! less memory.
//!
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions)
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

extern crate time;
use std::fmt;
use std::sync::RwLock;
use time::OffsetDateTime;

/// Date struct for getting the current date and time.
///
/// This struct represents a date and time value with various fields,
/// such as the date, day, hour, ISO 8601 date and time, ISO week number,
/// minute, month, offset, ordinal, second, time, weekday, and year.
// Each field is wrapped in a Mutex to make the struct thread-safe and
// allow for concurrent access.
///
#[derive(Debug)]
pub struct Date {
    // The date represented as a time::Date object
    pub date: RwLock<time::Date>,
    // The day of the month as a number (1-31)
    pub day: RwLock<u8>,
    // The hour of the day as a number (0-23)
    pub hour: RwLock<u8>,
    // The ISO 8601 date and time as a string
    pub iso_8601: RwLock<String>,
    // The ISO week number as a number
    pub iso_week: RwLock<u8>,
    // The minute of the hour as a number (0-59)
    pub minute: RwLock<u8>,
    // The month as a string (e.g. "January")
    pub month: RwLock<String>,
    // The offset from UTC as a time::UtcOffset object
    pub offset: RwLock<time::UtcOffset>,
    // The ordinal date as a number
    pub ordinal: RwLock<u16>,
    // The second of the minute as a number (0-59)
    pub second: RwLock<u8>,
    // The time represented as a time::Time object
    pub time: RwLock<time::Time>,
    // The weekday as a string (e.g. "Monday")
    pub weekday: RwLock<String>,
    // The year as a number
    pub year: RwLock<i32>,
}

impl Date {
    /// Create a new instance of the `Date` struct with the current date and time.
    /// This struct is thread-safe, so it can be shared across multiple threads.
    pub fn new() -> Self {
        // Get the current date and time in UTC
        let now_utc = OffsetDateTime::now_utc();
        // Convert the date and time to an ISO 8601 string
        let iso_8601 = now_utc.to_string();
        // Create a new instance of the `Date` struct and initialize its fields
        Self {
            date: RwLock::new(now_utc.date()),
            day: RwLock::new(now_utc.day()),
            hour: RwLock::new(now_utc.hour()),
            iso_8601: RwLock::new(iso_8601),
            iso_week: RwLock::new(now_utc.iso_week()),
            minute: RwLock::new(now_utc.minute()),
            month: RwLock::new(now_utc.month().to_string()),
            offset: RwLock::new(now_utc.offset()),
            ordinal: RwLock::new(now_utc.ordinal()),
            second: RwLock::new(now_utc.second()),
            time: RwLock::new(now_utc.time()),
            weekday: RwLock::new(now_utc.weekday().to_string()),
            year: RwLock::new(now_utc.year()),
        }
    }
}

/// Default implementation for Date.
/// Returns a new Date.
impl Default for Date {
    fn default() -> Self {
        Self::new()
    }
}

/// Clone implementation for Date.
impl Clone for Date {
    fn clone(&self) -> Self {
        let date = match self.date.try_read() {
            Ok(guard) => *guard,
            _ => Date::new().date.into_inner().unwrap(),
        };
        let day = match self.day.try_read() {
            Ok(guard) => *guard,
            _ => 0,
        };
        let hour = match self.hour.try_read() {
            Ok(guard) => *guard,
            _ => 0,
        };
        let iso_8601 = match self.iso_8601.try_read() {
            Ok(guard) => guard.clone(),
            _ => "".to_string(),
        };
        let iso_week = match self.iso_week.try_read() {
            Ok(guard) => *guard,
            _ => 0,
        };
        let minute = match self.minute.try_read() {
            Ok(guard) => *guard,
            _ => 0,
        };
        let month = match self.month.try_read() {
            Ok(guard) => guard.clone(),
            _ => "".to_string(),
        };
        let offset = match self.offset.try_read() {
            Ok(guard) => *guard,
            _ => Date::new().offset.into_inner().unwrap(),
        };
        let ordinal = match self.ordinal.try_read() {
            Ok(guard) => *guard,
            _ => 0,
        };
        let second = match self.second.try_read() {
            Ok(guard) => *guard,
            _ => 0,
        };
        let time = match self.time.try_read() {
            Ok(guard) => *guard,
            _ => Date::new().time.into_inner().unwrap(),
        };
        let weekday = match self.weekday.try_read() {
            Ok(guard) => guard.clone(),
            _ => "".to_string(),
        };
        let year = match self.year.try_read() {
            Ok(guard) => *guard,
            _ => 0,
        };

        Self {
            date: date.into(),
            day: day.into(),
            hour: hour.into(),
            iso_8601: iso_8601.into(),
            iso_week: iso_week.into(),
            minute: minute.into(),
            month: month.into(),
            offset: offset.into(),
            ordinal: ordinal.into(),
            second: second.into(),
            time: time.into(),
            weekday: weekday.into(),
            year: year.into(),
        }
    }
}

// Display implementation for Date.
// Returns a formatted String.
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "date: {}\nday: {}\nhour: {}\niso_8601: {}\niso_week: {}\nminute: {}\nmonth: {}\noffset: {}\nordinal: {}\nsecond: {}\ntime: {}\nweekday: {}\nyear: {}",
            self.date.read().unwrap(),
            self.day.read().unwrap(),
            self.hour.read().unwrap(),
            self.iso_8601.read().unwrap(),
            self.iso_week.read().unwrap(),
            self.minute.read().unwrap(),
            self.month.read().unwrap(),
            self.offset.read().unwrap(),
            self.ordinal.read().unwrap(),
            self.second.read().unwrap(),
            self.time.read().unwrap(),
            self.weekday.read().unwrap(),
            self.year.read().unwrap()
        )
    }
}
