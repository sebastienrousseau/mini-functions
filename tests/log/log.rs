#[cfg(test)]
mod tests {
    use mini_functions::{date::date::Date, log::log::Log};

    #[test]
    fn test_log() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &Date::now_utc(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_error() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &Date::now_utc(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_warn() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &Date::now_utc(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_debug() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &Date::now_utc(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_trace() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &Date::now_utc(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_info() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &Date::now_utc(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
}
