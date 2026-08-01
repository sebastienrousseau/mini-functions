// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::date::DateTime;
use std::str::FromStr;

/// Main function for date example.
pub fn main() {
    let paris_time = DateTime::new_with_tz("CET").unwrap();
    println!("🦀 Paris time:        ✅ {}", paris_time);

    let date = DateTime::new();
    println!("🦀 Date:              ✅ {}", date);
    println!("🦀 Day:               ✅ {}", date.day());
    println!("🦀 Hour:              ✅ {}", date.hour());

    if let Ok(nd) = DateTime::next_day(&date) {
        println!("🦀 Next day:          ✅ {}", nd.day());
    }

    if let Ok(pd) = DateTime::previous_day(&date) {
        println!("🦀 Previous day:      ✅ {}", pd.day());
    }

    let date_str = "2022-01-01T12:00:00+01:00";
    if let Ok(dt) = DateTime::from_str(date_str) {
        println!("🦀 from_str():        ✅ true");
        println!("🦀 from_str(day):     ✅ {}", dt.day());
    }
}
