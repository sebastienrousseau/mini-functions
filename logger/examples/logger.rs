extern crate date;
extern crate logger;

use std::sync::RwLock;

use self::date::Date;
use self::logger::{Log, LogFormat, LogLevel};

fn main() {
    let now: RwLock<String> = Date::new().iso_8601;
    let log_formats = vec![
        LogFormat::COMMON,
        LogFormat::JSON,
        LogFormat::CEF,
        LogFormat::ELF,
        LogFormat::GELF,
        LogFormat::W3C,
    ];

    for format in log_formats {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &now.read().unwrap().to_string(),
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
            &format,
        );
        println!("🦀 Log::new():            ✅ {log}");
    }
}
