//! # Core Date functionality
//!
//! This crate provides an easy way to get the current date and time in multiple formats.
//! It is a thread-safe, optimized for speed and memory usage.
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
            date: now_utc.date().into(),
            day: now_utc.day().into(),
            hour: now_utc.hour().into(),
            iso_8601: iso_8601.into(),
            iso_week: now_utc.iso_week().into(),
            minute: now_utc.minute().into(),
            month: now_utc.month().to_string().into(),
            offset: now_utc.offset().into(),
            ordinal: now_utc.ordinal().into(),
            second: now_utc.second().into(),
            time: now_utc.time().into(),
            weekday: now_utc.weekday().to_string().into(),
            year: now_utc.year().into(),
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
