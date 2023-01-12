#[cfg(test)]
mod tests {

    extern crate mini_functions_date;
    extern crate mini_functions_log;

    use self::mini_functions_date::Date;
    use self::mini_functions_log::Log;

    #[test]
    fn test_log() {
        let date = Date::new().date();
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.to_string(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_error() {
        let date = Date::new().date();
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.to_string(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_warn() {
        let date = Date::new().date();
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.to_string(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_debug() {
        let date = Date::new().date();
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.to_string(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_trace() {
        let date = Date::new().date();
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.to_string(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
    #[test]
    fn test_log_info() {
        let date = Date::new().date();
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &date.to_string(),
            "INFO",
            "SystemTrayEvent",
            "Showing main window",
        );
        log.log();
    }
}
