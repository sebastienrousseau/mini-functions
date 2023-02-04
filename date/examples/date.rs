extern crate date;
use self::date::DateTime;

pub fn main() {
    let date = DateTime::new();
    println!("🦀 Date:              ✅ {}", date.now);
    println!("🦀 Day:               ✅ {}", date.day);
    println!("🦀 Hour:              ✅ {}", date.hour);
    println!("🦀 ISO 8601:          ✅ {}", date.iso_8601);
    println!("🦀 ISO Week Number:   ✅ {}", date.iso_week);
    println!("🦀 Minute:            ✅ {}", date.minute);
    println!("🦀 Month:             ✅ {}", date.month);
    println!("🦀 Offset:            ✅ {}", date.offset);
    println!("🦀 Ordinal Date:      ✅ {}", date.ordinal);
    println!("🦀 Second:            ✅ {}", date.second);
    println!("🦀 Microsecond:       ✅ {}", date.microsecond);
    println!("🦀 Time:              ✅ {}", date.time);
    println!("🦀 Weekday:           ✅ {}", date.weekday);
    println!("🦀 Year:              ✅ {}", date.year);

    println!(
        "🦀 Invalid day (32):       ❌ {}",
        DateTime::is_valid_day("32")
    );
    println!(
        "🦀 Valid day:         ✅ {}",
        DateTime::is_valid_day(&date.day.to_string())
    );
    println!(
        "🦀 Invalid hour (24):      ❌ {}",
        DateTime::is_valid_hour("24")
    );
    println!(
        "🦀 Valid hour:        ✅ {}",
        DateTime::is_valid_hour(&date.hour.to_string())
    );
    // println!(
    //     "🦀 Invalid month:     ❌ {}",
    //     DateTime::is_valid_month("13")
    // );
}
