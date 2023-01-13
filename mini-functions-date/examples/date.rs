extern crate mini_functions_date;
use self::mini_functions_date::Date;

pub fn main() {
    let date = Date::new();
    match date.date.read() {
        Ok(d) => println!("🦀 date:          ✅ {}", d),
        Err(e) => println!("🦀 date:          ❌ {}", e),
    }
    match date.day.read() {
        Ok(d) => println!("🦀 day:           ✅ {}", d),
        Err(e) => println!("🦀 day:           ❌ {}", e),
    }
    match date.hour.read() {
        Ok(d) => println!("🦀 hour:          ✅ {}", d),
        Err(e) => println!("🦀 hour:          ❌ {}", e),
    }
    match date.iso_8601.read() {
        Ok(d) => println!("🦀 iso_8601:      ✅ {}", d),
        Err(e) => println!("🦀 iso_8601:      ❌ {}", e),
    }
    match date.iso_week.read() {
        Ok(d) => println!("🦀 iso_week:      ✅ {}", d),
        Err(e) => println!("🦀 iso_week:      ❌ {}", e),
    }
    match date.minute.read() {
        Ok(d) => println!("🦀 minute:        ✅ {}", d),
        Err(e) => println!("🦀 minute:        ❌ {}", e),
    }
    match date.month.read() {
        Ok(d) => println!("🦀 month:         ✅ {}", d),
        Err(e) => println!("🦀 month:         ❌ {}", e),
    }
    match date.offset.read() {
        Ok(d) => println!("🦀 offset:        ✅ {}", d),
        Err(e) => println!("🦀 offset:        ❌ {}", e),
    }
    match date.ordinal.read() {
        Ok(d) => println!("🦀 ordinal:       ✅ {}", d),
        Err(e) => println!("🦀 ordinal:       ❌ {}", e),
    }
    match date.second.read() {
        Ok(d) => println!("🦀 second:        ✅ {}", d),
        Err(e) => println!("🦀 second:        ❌ {}", e),
    }
    match date.time.read() {
        Ok(d) => println!("🦀 time:          ✅ {}", d),
        Err(e) => println!("🦀 time:          ❌ {}", e),
    }
    match date.weekday.read() {
        Ok(d) => println!("🦀 weekday:       ✅ {}", d),
        Err(e) => println!("🦀 weekday:       ❌ {}", e),
    }
    match date.year.read() {
        Ok(d) => println!("🦀 year:          ✅ {}", d),
        Err(e) => println!("🦀 year:          ❌ {}", e),
    };
}
