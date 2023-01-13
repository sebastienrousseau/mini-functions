extern crate mini_functions_date;
use self::mini_functions_date::Date;

pub fn main() {
    let date = Date::new().date();
    println!("🦀 date:                 ✅ {:?}", date);

    let day = Date::new().day();
    println!("🦀 day:                  ✅ {:?}", day);

    let hour = Date::new().hour();
    println!("🦀 hour:                 ✅ {:?}", hour);

    let iso_8601 = Date::new().iso_8601();
    println!("🦀 iso_8601:             ✅ {:?}", iso_8601);

    let iso_week = Date::new().iso_week();
    println!("🦀 iso_week:             ✅ {:?}", iso_week);

    let microsecond = Date::new().microsecond();
    println!("🦀 microsecond:          ✅ {:?}", microsecond);

    let millisecond = Date::new().millisecond();
    println!("🦀 millisecond:          ✅ {:?}", millisecond);

    let minute = Date::new().minute();
    println!("🦀 minute:               ✅ {:?}", minute);

    let month = Date::new().month();
    println!("🦀 month:                ✅ {:?}", month);

    let nanosecond = Date::new().nanosecond();
    println!("🦀 nanosecond:           ✅ {:?}", nanosecond);

    let offset = Date::new().offset();
    println!("🦀 offset:               ✅ {:?}", offset);

    let ordinal = Date::new().ordinal();
    println!("🦀 ordinal:              ✅ {:?}", ordinal);

    let second = Date::new().second();
    println!("🦀 second:               ✅ {:?}", second);

    let time = Date::new().time();
    println!("🦀 time:                 ✅ {:?}", time);

    let weekday = Date::new().weekday();
    println!("🦀 weekday:              ✅ {:?}", weekday);

    let year = Date::new().year();
    println!("🦀 year:                 ✅ {:?}", year);
}
