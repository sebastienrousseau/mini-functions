//! # Core Date functionality
//!
//! This crate provides an easy way to get the current date and time in multiple formats.
//!
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

extern crate time;
use std::fmt;

use time::OffsetDateTime;

/// Date Utility
///
/// Provides an easy way to get the current date and time in multiple
/// formats. The date and time are based on the UTC timezone.
///
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Date {
    /// The date.
    date: time::Date,
    /// The day of the month.
    day: u8,
    /// The hour of the day.
    hour: u8,
    /// The ISO 8601 date and time.
    iso_8601: String,
    /// The ISO week number.
    iso_week: u8,
    /// The microsecond.
    microsecond: u32,
    /// The millisecond.
    millisecond: u16,
    /// The minute.
    minute: u8,
    /// The month.
    month: String,
    /// The nanosecond.
    nanosecond: u32,
    /// The offset from UTC.
    offset: time::UtcOffset,
    /// The ordinal date.
    ordinal: u16,
    /// The second.
    second: u8,
    /// The time.
    time: time::Time,
    /// The weekday.
    weekday: String,
    /// The year.
    year: i32,
}

impl Date {
    /// Create a new Date.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.date(), );
    /// ```
    pub fn date(&self) -> time::Date {
        self.date
    }
    /// Create a new Day.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.day(), );
    /// ```
    pub fn day(&self) -> u8 {
        self.day
    }
    /// Create a new Hour.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.hour(), );
    /// ```
    pub fn hour(&self) -> u8 {
        self.hour
    }
    /// Create a new ISO 8601 date and time.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.iso_8601(), );
    /// ```
    pub fn iso_8601(&self) -> String {
        self.iso_8601.clone()
    }
    /// Create a new ISO week number.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.iso_week(), );
    /// ```
    pub fn iso_week(&self) -> u8 {
        self.iso_week
    }
    /// Create a new Microsecond.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.microsecond(), );
    /// ```
    pub fn microsecond(&self) -> u32 {
        self.microsecond
    }
    /// Create a new Millisecond.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.millisecond(), );
    /// ```
    pub fn millisecond(&self) -> u16 {
        self.millisecond
    }
    /// Create a new Minute.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.minute(), );
    /// ```
    pub fn minute(&self) -> u8 {
        self.minute
    }
    /// Create a new Month.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.month(), );
    /// ```
    pub fn month(&self) -> String {
        self.month.clone()
    }
    /// Create a new Nanosecond.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.nanosecond(), );
    /// ```
    pub fn nanosecond(&self) -> u32 {
        self.nanosecond
    }
    /// Create a new Date.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// assert_eq!(Date::new(), );
    /// ```
    pub fn new() -> Self {
        let now_utc = OffsetDateTime::now_utc();
        let iso_8601 = now_utc.to_string().replace("UTC", "").replace("+00:00", "");
        Self {
            date: now_utc.date(),
            day: now_utc.day(),
            hour: now_utc.hour(),
            iso_8601,
            iso_week: now_utc.iso_week(),
            microsecond: now_utc.microsecond(),
            millisecond: now_utc.millisecond(),
            minute: now_utc.minute(),
            month: now_utc.month().to_string(),
            nanosecond: now_utc.nanosecond(),
            offset: now_utc.offset(),
            ordinal: now_utc.ordinal(),
            second: now_utc.second(),
            time: now_utc.time(),
            weekday: now_utc.weekday().to_string(),
            year: now_utc.year(),
        }
    }
    /// Create an Offset.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.offset(), );
    /// ```
    pub fn offset(&self) -> time::UtcOffset {
        self.offset
    }
    /// Create an Ordinal.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.ordinal(), );
    /// ```
    pub fn ordinal(&self) -> u16 {
        self.ordinal
    }
    /// Create a new Second.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.second(), );
    /// ```
    pub fn second(&self) -> u8 {
        self.second
    }
    /// Create a new Time.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.time(), );
    /// ```
    pub fn time(&self) -> time::Time {
        self.time
    }
    /// Create a new Weekday.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.weekday(), );
    /// ```
    pub fn weekday(&self) -> String {
        self.weekday.clone()
    }
    /// Create a new Year.
    ///
    /// # Examples
    ///
    /// ```
    /// use mini_functions_date::Date;
    ///
    /// let date = ;
    /// assert_eq!(date.year(), );
    /// ```
    pub fn year(&self) -> i32 {
        self.year
    }
}

/// Default implementation for Date.
/// Returns a new Date.
impl Default for Date {
    fn default() -> Self {
        Self::new()
    }
}

/// Display implementation for Date.
/// Returns a formatted String.
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Date {{ date: {:?}, iso_8601: {}, iso_week: {}, day: {}, hour: {}, microsecond: {}, milliseconds: {}, minute: {}, month: {:?}, nanosecond: {}, offset: {:?}, ordinal: {}, second: {}, time: {:?}, weekday: {:?}, year: {} }}",
        self.date, self.iso_8601, self.iso_week, self.day, self.hour, self.microsecond, self.millisecond, self.minute, self.month, self.nanosecond, self.offset, self.ordinal, self.second, self.time, self.weekday, self.year)
    }
}
