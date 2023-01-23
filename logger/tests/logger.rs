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
    #[test]
    fn test_log_level_display() {
        let log_level = LogLevel::ERROR;
        assert_eq!(log_level.to_string(), "ERROR");

        let log_level = LogLevel::WARNING;
        assert_eq!(log_level.to_string(), "WARNING");
    }

    #[test]
    fn test_log_display() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            "2022-01-01 12:00:00",
            &LogLevel::ERROR,
            "Test",
            "This is a test log message",
        );
        assert_eq!(
        log.to_string(),
        "SessionID=12345678-1234-1234-1234-1234567890ab Timestamp=2022-01-01 12:00:00 Level=ERROR Component=Test Description=\"This is a test log message\""
    );
    }

    #[test]
    fn test_log_default() {
        let log = Log::default();
        assert_eq!(log.session_id, "");
        assert_eq!(log.time, "");
        assert_eq!(log.level, LogLevel::INFO);
        assert_eq!(log.component, "");
        assert_eq!(log.description, "");
    }

    #[test]
    fn test_log_log() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            "2022-01-01 12:00:00",
            &LogLevel::ERROR,
            "Test",
            "This is a test log message",
        );
        let log_string = format!("{}", log);
        println!("{}", log_string);
        assert_eq!(log_string, "SessionID=12345678-1234-1234-1234-1234567890ab Timestamp=2022-01-01 12:00:00 Level=ERROR Component=Test Description=\"This is a test log message\"");
    }

    #[test]
    fn test_log_level_all_display() {
        let log_level = LogLevel::ALL;
        assert_eq!(log_level.to_string(), "ALL");
    }

    #[test]
    fn test_log_level_debug_display() {
        let log_level = LogLevel::DEBUG;
        assert_eq!(log_level.to_string(), "DEBUG");
    }

    #[test]
    fn test_log_level_disabled_display() {
        let log_level = LogLevel::DISABLED;
        assert_eq!(log_level.to_string(), "DISABLED");
    }

    #[test]
    fn test_log_level_error_display() {
        let log_level = LogLevel::ERROR;
        assert_eq!(log_level.to_string(), "ERROR");
    }

    #[test]
    fn test_log_level_fatal_display() {
        let log_level = LogLevel::FATAL;
        assert_eq!(log_level.to_string(), "FATAL");
    }

    #[test]
    fn test_log_level_info_display() {
        let log_level = LogLevel::INFO;
        assert_eq!(log_level.to_string(), "INFO");
    }

    #[test]
    fn test_log_level_none_display() {
        let log_level = LogLevel::NONE;
        assert_eq!(log_level.to_string(), "NONE");
    }

    #[test]
    fn test_log_level_trace_display() {
        let log_level = LogLevel::TRACE;
        assert_eq!(log_level.to_string(), "TRACE");
    }

    #[test]
    fn test_log_level_unavailable_display() {
        let log_level = LogLevel::UNAVAILABLE;
        assert_eq!(log_level.to_string(), "UNAVAILABLE");
    }

    #[test]
    fn test_log_level_undefined_display() {
        let log_level = LogLevel::UNDEFINED;
        assert_eq!(log_level.to_string(), "UNDEFINED");
    }

    #[test]
    fn test_log_level_verbose_display() {
        let log_level = LogLevel::VERBOSE;
        assert_eq!(log_level.to_string(), "VERBOSE");
    }

    #[test]
    fn test_log_level_warning_display() {
        let log_level = LogLevel::WARNING;
        assert_eq!(log_level.to_string(), "WARNING");
    }
}
