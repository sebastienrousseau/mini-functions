/*!
Using `date` module

```no_run,shell
cargo run --example date
```
*/

extern crate mini_functions;

use mini_functions::date::date::Date;

fn main() {
    let date = Date::date();
    let day = Date::day();
    let hour = Date::hour();
    let iso_8601 = Date::iso_8601();
    let microsecond = Date::microsecond();
    let millisecond = Date::millisecond();
    let minute = Date::minute();
    let month = Date::month();
    let nanosecond = Date::nanosecond();
    let now_utc = Date::now_utc();
    let second = Date::second();
    let timestamp = Date::timestamp();
    let weekday = Date::weekday();
    let year = Date::year();

    println!("🦀 Date::date():          ✅  {}", date);
    println!("🦀 Date::day():           ✅  {}", day);
    println!("🦀 Date::hour():          ✅  {}", hour);
    println!("🦀 Date::iso_8601():      ✅  {}", iso_8601);
    println!("🦀 Date::microsecond():   ✅  {}", microsecond);
    println!("🦀 Date::millisecond():   ✅  {}", millisecond);
    println!("🦀 Date::minute():        ✅  {}", minute);
    println!("🦀 Date::month():         ✅  {}", month);
    println!("🦀 Date::nanosecond():    ✅  {}", nanosecond);
    println!("🦀 Date::now_utc():       ✅  {}", now_utc);
    println!("🦀 Date::second():        ✅  {}", second);
    println!("🦀 Date::timestamp():     ✅  {}", timestamp);
    println!("🦀 Date::weekday():       ✅  {}", weekday);
    println!("🦀 Date::year():          ✅  {}", year);
}
