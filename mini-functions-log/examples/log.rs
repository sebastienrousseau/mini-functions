extern crate mini_functions_date;
extern crate mini_functions_log;

use self::mini_functions_date::Date;
use self::mini_functions_log::Log;

fn main() {
    let date = Date::new().date();
    let log = Log::new(
        "12345678-1234-1234-1234-1234567890ab",
        &date.to_string(),
        "INFO",
        "SystemTrayEvent",
        "Showing main window",
    );
    println!("🦀 Log::new():            ✅ {}", log);
}
