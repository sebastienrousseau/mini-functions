extern crate date;
extern crate logger;

use self::date::Date;
use self::logger::{Log, LogLevel};

fn main() {
    let iso_8601 = Date::new().iso_8601;
    let log = Log::new(
        "12345678-1234-1234-1234-1234567890ab",
        &iso_8601.read().unwrap().to_string(),
        &LogLevel::INFO,
        "SystemTrayEvent",
        "Showing main window",
    );
    println!("🦀 Log::new():            ✅ {}", log);
}