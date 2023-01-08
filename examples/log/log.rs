use mini_functions::{date::date::Date, log::log::Log};

fn main() {
    let log = Log::new(
        "12345678-1234-1234-1234-1234567890ab",
        &Date::now_utc(),
        "INFO",
        "SystemTrayEvent",
        "Showing main window",
    );
    println!("🦀 Log::new():            ✅ {}", log);
}
