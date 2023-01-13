extern crate mini_functions_date;
extern crate mini_functions_log;

use self::mini_functions_date::Date;
use self::mini_functions_log::Log;

fn main() {
    let iso_8601 = Date::new().iso_8601;
    let log = Log::new(
        "12345678-1234-1234-1234-1234567890ab",
        &iso_8601.read().unwrap().to_string(),
        "INFO",
        "SystemTrayEvent",
        "Showing main window",
    );
    println!("🦀 Log::new():            ✅ {}", log);
}
