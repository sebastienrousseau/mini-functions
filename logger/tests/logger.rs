#[cfg(test)]
mod tests {

    extern crate date;
    extern crate logger;

    use self::date::Date;
    use self::logger::{Log, LogLevel};

    #[test]
    fn test_log() {
        let date = Date::new().date;
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.read().unwrap().to_string(),
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_error() {
        let date = Date::new().date;
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.read().unwrap().to_string(),
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_warn() {
        let date = Date::new().date;
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.read().unwrap().to_string(),
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_debug() {
        let date = Date::new().date;
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.read().unwrap().to_string(),
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_trace() {
        let date = Date::new().date;
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.read().unwrap().to_string(),
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_info() {
        let date = Date::new().date;
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.read().unwrap().to_string(),
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
}
