#[cfg(test)]

mod tests {

    extern crate mini_functions;

    use mini_functions::date::Date;

    #[test]
    fn test_date() {
        // Check that the date function is correctly generating a non
        //-empty string representation of the current date.
        let date = Date::date();
        assert!(!date.is_empty());
        assert_eq!(date, date.to_string());

        // Check that the returned date is today's date
        let today = Date::date();
        assert_eq!(date, today);
        assert_eq!(date.to_string(), today.to_string());
    }

    #[test]
    fn test_now_utc() {
        // Check that the now_utc function is correctly generating a non
        //-empty string representation of the current date.
        let now_utc = Date::now_utc();
        assert!(!now_utc.is_empty());
        assert_eq!(now_utc, now_utc.to_string());
    }
    #[test]
    fn test_now_iso_8601() {
        // Check that the now_iso_8601 function is correctly generating
        // a non-empty string representation of the current date.
        let iso_8601 = Date::iso_8601();
        assert!(!iso_8601.is_empty());
        assert_eq!(iso_8601, iso_8601.to_string());
    }
    #[test]
    fn test_clone() {
        // Check that the clone function is correctly generating a
        // non-empty string representation of the current date.
        let now_utc = Date::now_utc();
        let now_utc_clone = now_utc.clone();
        assert_eq!(now_utc, now_utc_clone);
    }
    #[test]
    fn test_debug() {
        // Check that the debug function is correctly generating a
        // non-empty string representation of the current date.
        let debug = Date::now_utc();
        assert_eq!(debug.to_string(), debug);
        assert_eq!(debug.to_string(), debug.to_string());
    }
    #[test]
    fn test_year() {
        // Check that the `year` function is correctly generating a
        // non-empty string representation of the current year.
        let year = Date::year();
        assert!(!year.is_empty());
        assert_eq!(year, year.to_string());
    }
    #[test]
    fn test_month() {
        // Check that the `month` function is correctly generating a
        // non-empty string representation of the current month.
        let month = Date::month();
        assert!(!month.is_empty());
        assert_eq!(month, month.to_string());
    }
    #[test]
    fn test_day() {
        // Check that the `day` function is correctly generating a
        // non-empty string representation of the current day.
        let day = Date::day();
        assert!(!day.is_empty());
        assert_eq!(day, day.to_string());
    }
    #[test]
    fn test_hour() {
        // Check that the `hour` function is correctly generating a
        // non-empty string representation of the current hour.
        let hour = Date::hour();
        assert!(!hour.is_empty());
        assert_eq!(hour, hour.to_string());
    }
    #[test]
    fn test_minute() {
        // Check that the `minute` function is correctly generating a
        // non-empty string representation of the current minute.
        let minute = Date::minute();
        assert!(!minute.is_empty());
        assert_eq!(minute, minute.to_string());
    }
    #[test]
    fn test_second() {
        // Check that the `second` function is correctly generating a
        // non-empty string representation of the current second.
        let second = Date::second();
        assert!(!second.is_empty());
        assert_eq!(second, second.to_string());
    }
    #[test]
    fn test_millisecond() {
        // Check that the `millisecond` function is correctly generating a
        // non-empty string representation of the current millisecond.
        let millisecond = Date::millisecond();
        assert!(!millisecond.is_empty());
        assert_eq!(millisecond, millisecond.to_string());
    }
    #[test]
    fn test_microsecond() {
        // Check that the `microsecond` function is correctly generating
        // a non-empty string representation of the current microsecond.
        let microsecond = Date::microsecond();
        assert!(!microsecond.is_empty());
        assert_eq!(microsecond, microsecond.to_string());
    }
    #[test]
    fn test_nanosecond() {
        // Check that the `nanosecond` function is correctly generating
        // a non-empty string representation of the current nanosecond.
        let nanosecond = Date::nanosecond();
        assert!(!nanosecond.is_empty());
        assert_eq!(nanosecond, nanosecond.to_string());
    }
    #[test]
    fn test_offset() {
        // Check that the `offset` function is correctly generating a
        // non-empty string representation of the current offset.
        let offset = Date::offset();
        assert!(!offset.is_empty());
        assert_eq!(offset, offset.to_string());
    }
    #[test]
    fn test_ordinal() {
        // Check that the `ordinal` function is correctly generating a
        // non-empty string representation of the current ordinal.
        let ordinal = Date::ordinal();
        assert!(!ordinal.is_empty());
        assert_eq!(ordinal, ordinal.to_string());
    }
    #[test]
    fn test_iso_week() {
        // Check that the `iso_week` function is correctly generating a
        // non-empty string representation of the current iso_week.
        let iso_week = Date::iso_week();
        assert!(!iso_week.is_empty());
        assert_eq!(iso_week, iso_week.to_string());
    }
    #[test]
    fn test_time() {
        // Check that the `time` function is correctly generating a
        // non-empty string representation of the current time.
        let time = Date::time();
        assert!(!time.is_empty());
        assert_eq!(time, time.to_string());
    }
    #[test]
    fn test_timestamp() {
        // Check that the `timestamp` function is correctly generating a
        // non-empty string representation of the current timestamp.
        let timestamp = Date::timestamp();
        assert!(!timestamp.is_empty());
        assert_eq!(timestamp, timestamp.to_string());
    }
    #[test]
    fn test_weekday() {
        // Check that the `weekday` function is correctly generating a
        // non-empty string representation of the current weekday.
        let weekday = Date::weekday();
        assert!(!weekday.is_empty());
        assert_eq!(weekday, weekday.to_string());
    }
}
