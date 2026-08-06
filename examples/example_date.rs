// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates the `date` module, a re-export of `dtt`'s [`DateTime`].
//!
//! `dtt` 0.0.11 replaced the old struct-of-strings (`date.hour` as a
//! `String`, `date.iso_8601`, `relative_delta()`) with a compact
//! `PrimitiveDateTime` + `UtcOffset` pair behind typed accessors, and
//! fallible constructors now return `Result`. This example tracks that
//! API.

use mini_functions::date::DateTime;
use std::str::FromStr;

/// Runs every demonstration, propagating any failure to `main`.
fn run() -> Result<(), Box<dyn std::error::Error>> {
    // A DateTime in a named timezone. Fallible: the name is validated.
    let paris_time = DateTime::new_with_tz("CET")?;
    println!("🦀 Paris time:        ✅ {paris_time}");

    // `new()` is infallible and yields the current time in UTC.
    let date = DateTime::new();

    // `Display` renders RFC 3339, which is the ISO 8601 profile the old
    // `iso_8601` field carried.
    println!("🦀 Date:              ✅ {date}");
    println!("🦀 RFC 3339:          ✅ {}", date.format_rfc3339()?);
    println!("🦀 Day:               ✅ {}", date.day());
    println!("🦀 Hour:              ✅ {}", date.hour());
    println!("🦀 ISO Week Number:   ✅ {}", date.iso_week());
    println!("🦀 Microsecond:       ✅ {}", date.microsecond());
    println!("🦀 Minute:            ✅ {}", date.minute());
    // `month()` returns a `Month`, whose `Display` is the month *name*.
    println!("🦀 Month:             ✅ {}", date.month());
    println!("🦀 Month number:      ✅ {}", u8::from(date.month()));
    println!("🦀 Offset:            ✅ {}", date.offset());
    println!("🦀 Ordinal Date:      ✅ {}", date.ordinal());
    println!("🦀 Second:            ✅ {}", date.second());
    println!(
        "🦀 Time:              ✅ {}",
        date.format("[hour]:[minute]:[second]")?
    );
    println!("🦀 Weekday:           ✅ {}", date.weekday());
    println!("🦀 Year:              ✅ {}", date.year());
    println!("🦀 Unix timestamp:    ✅ {}", date.unix_timestamp());

    // Validation. `is_valid_iso_8601` replaces the per-component
    // `is_valid_day` / `is_valid_hour` helpers.
    println!(
        "🦀 Valid ISO 8601:    ❌ {}",
        DateTime::is_valid_iso_8601("2024-13-01")
    );
    println!(
        "🦀 Valid ISO 8601:    ✅ {}",
        DateTime::is_valid_iso_8601(&date.format_rfc3339()?)
    );

    // Day arithmetic. Both directions are fallible because they can
    // leave the representable range.
    println!("🦀 Next day:          ✅ {}", date.next_day()?.day());
    println!("🦀 Previous day:      ✅ {}", date.previous_day()?.day());

    // Parsing, via either `FromStr` or the inherent `parse`.
    let date_str = "2022-01-01T12:00:00+01:00";
    let parsed = DateTime::from_str(date_str)?;
    println!("🦀 from_str(year):    ✅ {}", parsed.year());
    println!("🦀 from_str(day):     ✅ {}", parsed.day());
    println!("🦀 from_str(hour):    ✅ {}", parsed.hour());
    println!("🦀 from_str(offset):  ✅ {}", parsed.offset());

    // `relative_delta()` is gone; build a specific instant explicitly
    // and shift it with the typed arithmetic helpers instead.
    // The offset argument is a `time::UtcOffset`, which `dtt` does not
    // re-export; take it off an existing value rather than pulling
    // `time` in as a direct dependency just to name the type. `date` is
    // UTC, so this builds the instant in UTC.
    let dt =
        DateTime::from_components(1975, 5, 11, 8, 8, 0, date.offset())?;
    println!("🦀 Built:             ✅ {dt}");
    println!("🦀 Built week:        ✅ {}", dt.iso_week());
    println!("🦀 Built weekday:     ✅ {}", dt.weekday());
    println!("🦀 + 30 days:         ✅ {}", dt.add_days(30)?);
    println!("🦀 + 6 months:        ✅ {}", dt.add_months(6)?);
    println!("🦀 - 5 years:         ✅ {}", dt.sub_years(5)?);
    println!("🦀 Start of month:    ✅ {}", dt.start_of_month()?);
    println!("🦀 End of year:       ✅ {}", dt.end_of_year()?);

    Ok(())
}

/// Entry point for the example.
pub fn main() {
    if let Err(e) = run() {
        eprintln!("example_date failed: {e}");
        std::process::exit(1);
    }
}
