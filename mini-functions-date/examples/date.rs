extern crate mini_functions_date;

use self::mini_functions_date::Date;

fn main() {
    let date = Date::new();
    println!("🦀 date:                 ✅ {:?}", date.date());
    println!("🦀 day:                  ✅ {:?}", date.day());
    println!("🦀 hour:                 ✅ {:?}", date.hour());
    println!("🦀 iso_8601:             ✅ {:?}", date.iso_8601());
    println!("🦀 iso_week:             ✅ {:?}", date.iso_week());
    println!("🦀 microsecond:          ✅ {:?}", date.microsecond());
    println!("🦀 millisecond:          ✅ {:?}", date.millisecond());
    println!("🦀 minute:               ✅ {:?}", date.minute());
    println!("🦀 month:                ✅ {:?}", date.month());
    println!("🦀 nanosecond:           ✅ {:?}", date.nanosecond());
    println!("🦀 offset:               ✅ {:?}", date.offset());
    println!("🦀 ordinal:              ✅ {:?}", date.ordinal());
    println!("🦀 second:               ✅ {:?}", date.second());
    println!("🦀 time:                 ✅ {:?}", date.time());
    println!("🦀 weekday:              ✅ {:?}", date.weekday());
    println!("🦀 year:                 ✅ {:?}", date.year());
}
