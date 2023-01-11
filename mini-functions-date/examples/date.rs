extern crate mini_functions_date;

use self::mini_functions_date::Date;

pub fn main() {
    let date = Date::new().date();
    println!("{}", date);
}

// pub fn main() {
//     let now = Date::default();
//     println!("🦀 date:                 ✅ {:?}", now.date());
//     println!("🦀 day:                  ✅ {:?}", now.day());
//     println!("🦀 hour:                 ✅ {:?}", now.hour());
//     println!("🦀 iso_8601:             ✅ {:?}", now.iso_8601());
//     println!("🦀 iso_week:             ✅ {:?}", now.iso_week());
//     println!("🦀 microsecond:          ✅ {:?}", now.microsecond());
//     println!("🦀 millisecond:          ✅ {:?}", now.millisecond());
//     println!("🦀 minute:               ✅ {:?}", now.minute());
//     println!("🦀 month:                ✅ {:?}", now.month());
//     println!("🦀 nanosecond:           ✅ {:?}", now.nanosecond());
//     println!("🦀 offset:               ✅ {:?}", now.offset());
//     println!("🦀 ordinal:              ✅ {:?}", now.ordinal());
//     println!("🦀 second:               ✅ {:?}", now.second());
//     println!("🦀 time:                 ✅ {:?}", now.time());
//     println!("🦀 weekday:              ✅ {:?}", now.weekday());
//     println!("🦀 year:                 ✅ {:?}", now.year());
// }
