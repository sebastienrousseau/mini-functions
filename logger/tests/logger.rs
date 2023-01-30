#[cfg(test)]
mod tests {

    extern crate date;
    extern crate logger;

    use self::date::Date;
    use self::logger::LogLevel::ERROR;
    use self::logger::{Log, LogFormat, LogLevel};
    use logger::LogFormat::COMMON;

    #[test]
    fn test_log_common_format() {
        let log = Log::new(
            "session_id_123",
            "2022-01-01T00:00:00Z",
            &ERROR,
            "component_a",
            "description_a",
            &COMMON,
        );
        let expected_output = "SessionID=session_id_123 Timestamp=2022-01-01T00:00:00Z Description=description_a Level=ERROR Component=component_a";
        assert_eq!(log.to_string(), expected_output);
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
            &LogFormat::COMMON,
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
            &LogFormat::COMMON,
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
            &LogFormat::COMMON,
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
            &LogFormat::COMMON,
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
            &LogFormat::COMMON,
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
            "2023-01-23 14:03:00.000+0000",
            &LogLevel::ERROR,
            "Test",
            "This is a test log message",
            &LogFormat::COMMON,
        );
        assert_eq!(
        log.to_string(),
        "SessionID=12345678-1234-1234-1234-1234567890ab Timestamp=2023-01-23 14:03:00.000+0000 Description=This is a test log message Level=ERROR Component=Test");
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
    fn test_log_common() {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            "2023-01-23 14:03:00.000+0000",
            &LogLevel::ERROR,
            "Test",
            "This is a test log message",
            &LogFormat::COMMON,
        );
        let log_string = format!("{log}");
        println!("{log_string}");
        assert_eq!(log_string, "SessionID=12345678-1234-1234-1234-1234567890ab Timestamp=2023-01-23 14:03:00.000+0000 Description=This is a test log message Level=ERROR Component=Test");
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
    fn test_log_level_verbose_display() {
        let log_level = LogLevel::VERBOSE;
        assert_eq!(log_level.to_string(), "VERBOSE");
    }

    #[test]
    fn test_log_level_warning_display() {
        let log_level = LogLevel::WARNING;
        assert_eq!(log_level.to_string(), "WARNING");
    }
    #[test]
    fn test_common_log_format() {
        let log = Log::new(
            "123",
            "2023-01-23 14:04:09.881393 +00:00:00",
            &LogLevel::INFO,
            "test",
            "test log message",
            &LogFormat::COMMON,
        );
        let expected_output = "SessionID=123 Timestamp=2023-01-23 14:04:09.881393 +00:00:00 Description=test log message Level=INFO Component=test";
        assert_eq!(log.to_string(), expected_output);
    }

    #[test]
    fn test_json_log_format() {
        let log = Log::new(
            "123",
            "2023-01-23 14:04:09.881393 +00:00:00",
            &LogLevel::INFO,
            "test",
            "test log message",
            &LogFormat::JSON,
        );
        let expected_output = "{\"SessionID\":\"123\",\"Timestamp\":\"2023-01-23 14:04:09.881393 +00:00:00\",\"Level\":\"INFO\",\"Component\":\"test\",\"Description\":\"test log message\",\"Format\":\"JSON\"}";
        assert_eq!(expected_output, format!("{log}"));
    }

    #[test]
    fn test_cef_log_format() {
        let log = Log::new(
            "123",
            "2023-01-23 14:04:09.881393 +00:00:00",
            &LogLevel::INFO,
            "test",
            "test log message",
            &LogFormat::CEF,
        );
        let expected_output =
            "CEF:0|123|2023-01-23 14:04:09.881393 +00:00:00|INFO|test|test log message|CEF";
        assert_eq!(expected_output, format!("{log}"));
    }
    #[test]
    fn test_elf_log_format() {
        let log = Log::new(
            "123",
            "2023-01-23 14:04:09.881393 +00:00:00",
            &LogLevel::INFO,
            "test",
            "test log message",
            &LogFormat::ELF,
        );
        let expected_output =
            "ELF:0|123|2023-01-23 14:04:09.881393 +00:00:00|INFO|test|test log message|ELF";
        assert_eq!(expected_output, format!("{log}"));
    }
    #[test]
    fn test_w3c_log_format() {
        let log = Log::new(
            "123",
            "2023-01-23 14:04:09.881393 +00:00:00",
            &LogLevel::INFO,
            "test",
            "test log message",
            &LogFormat::W3C,
        );
        let expected_output =
            "W3C:0|123|2023-01-23 14:04:09.881393 +00:00:00|INFO|test|test log message|W3C";
        assert_eq!(expected_output, format!("{log}"));
    }
    #[test]
    fn test_gelf_log_format() {
        let log = Log::new(
            "123",
            "2023-01-23 14:04:09.881393 +00:00:00",
            &LogLevel::INFO,
            "test",
            "test log message",
            &LogFormat::GELF,
        );
        let expected_output =
            "{\n                            \"version\": \"1.1\",\n                            \"host\": \"test\",\n                            \"short_message\": \"test log message\",\n                            \"level\": \"INFO\",\n                            \"timestamp\": \"2023-01-23 14:04:09.881393 +00:00:00\",\n                            \"_component\": \"test\",\n                            \"_session_id\": \"123\"\n                        }";
        assert_eq!(expected_output, format!("{log}"));
    }
    #[test]
    fn test_log_format_display() {
        for (log_format, expected_output) in vec![
            (LogFormat::COMMON, "COMMON"),
            (LogFormat::JSON, "JSON"),
            (LogFormat::CEF, "CEF"),
            (LogFormat::ELF, "ELF"),
            (LogFormat::W3C, "W3C"),
            (LogFormat::GELF, "GELF"),
        ] {
            assert_eq!(log_format.to_string(), expected_output);
        }
    }
}
