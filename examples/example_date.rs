// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::date::DateTime;
use std::str::FromStr;

/// This is the main function for the build script.
pub fn main() { 
    // Create a new DateTime object with a custom timezone (e.g., CET)
    let paris_time = DateTime::new_with_tz("CET").now;
    println!("🦀 Paris time:        ✅ {}", paris_time);

    // Example of how to use the `new` function with the UTC timezone
    let date = DateTime::new();
    println!("🦀 Date:              ✅ {}", date.now);
    println!("🦀 Day:               ✅ {}", date.day);
    println!("🦀 Hour:              ✅ {}", date.hour);
    println!("🦀 ISO 8601:          ✅ {}", date.iso_8601);
    println!("🦀 ISO Week Number:   ✅ {}", date.iso_week);
    println!("🦀 Microsecond:       ✅ {}", date.microsecond);
    println!("🦀 Minute:            ✅ {}", date.minute);
    println!("🦀 Month:             ✅ {}", date.month);
    println!("🦀 Offset:            ✅ {}", date.offset);
    println!("🦀 Ordinal Date:      ✅ {}", date.ordinal);
    println!("🦀 Second:            ✅ {}", date.second);
    println!("🦀 Time:              ✅ {}", date.time);
    println!("🦀 Time zone:         ✅ {}", date.tz);
    println!("🦀 Weekday:           ✅ {}", date.weekday);
    println!("🦀 Year:              ✅ {}", date.year);

    // Example of how to use the `is_valid_day` function
    println!(
        "🦀 Valid day (32):    ❌ {}",
        DateTime::is_valid_day("32")
    );
    println!(
        "🦀 Valid day:         ✅ {}",
        DateTime::is_valid_day(&date.day.to_string())
    );

    // Example of how to use the `is_valid_hour` function
    println!(
        "🦀 Valid hour (24):   ❌ {}",
        DateTime::is_valid_hour("24")
    );
    println!(
        "🦀 Valid hour:        ✅ {}",
        DateTime::is_valid_hour(&date.hour.to_string())
    );

    // Example of how to use the `next_day` function
    let nd = DateTime::next_day(&date);
    println!(
        "🦀 Next day:          ✅ {}",
        String::from(&nd.day.to_string())
    );

    // Example of how to use the `previous_day` function
    let pd = DateTime::previous_day(&date);
    println!(
        "🦀 Previous day:      ✅ {}",
        String::from(&pd.day.to_string())
    );

    // Example of how to use the `from_str` function
    let date_str = "2022-01-01T12:00:00+01:00";
    let expected = Ok(DateTime {
        day: 1,
        hour: 12,
        iso_8601: date_str.to_owned(),
        iso_week: 0,
        microsecond: 0,
        minute: 0,
        month: "".to_owned(),
        now: "".to_owned(),
        offset: "".to_owned(),
        ordinal: 0,
        second: 0,
        time: "".to_owned(),
        tz: "".to_owned(),
        weekday: "".to_owned(),
        year: 2022,
    });
    let result = DateTime::from_str(date_str);
    println!("🦀 from_str():        ✅ {}", result == expected);
    println!("🦀 from_str(day):     ✅ {}", result.unwrap().day);

    // Example of how to use the `relative_delta` function
    let mut dt = DateTime::new();
    dt.day = "11".parse::<u8>().unwrap();
    dt.hour = "08".parse::<u8>().unwrap();
    dt.iso_week = "19".parse::<u8>().unwrap();
    dt.microsecond = "000000".parse::<u32>().unwrap();
    dt.minute = "08".parse::<u8>().unwrap();
    dt.month = String::from("05");
    dt.second = "00".parse::<u8>().unwrap();
    dt.year = "1975".parse::<i32>().unwrap();

    let new_dt = dt.relative_delta();
    println!("🦀 Rd day:(11)        ✅ {}", new_dt.day);
    println!("🦀 Rd hour:(08)       ✅ {}", new_dt.hour);
    println!("🦀 Rd week:(19)       ✅ {}", new_dt.iso_week);
    println!("🦀 Rd ms:(000000)     ✅ {}", new_dt.microsecond);
    println!("🦀 Rd minute:(08)     ✅ {}", new_dt.minute);
    println!("🦀 Rd month:(05)      ✅ {}", new_dt.month);
    println!("🦀 Rd second:(00)     ✅ {}", new_dt.second);
    println!("🦀 Rd year:(1975)     ✅ {}", new_dt.year);
}