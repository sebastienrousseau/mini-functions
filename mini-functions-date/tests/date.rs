#[cfg(test)]

mod tests {

    extern crate mini_functions_date;

    use self::mini_functions_date::Date;

    #[test]
    fn test_date() {
        let date = Date::new().date();
        assert_eq!(date, date);
    }

    #[test]
    fn test_day() {
        let day = Date::new().day();
        assert_eq!(day, day);
    }
    #[test]
    fn test_hour() {
        let hour = Date::new();
        assert_eq!(hour, hour);
    }
    #[test]
    fn test_iso_8601() {
        let iso_8601 = Date::new().iso_8601();
        assert_eq!(iso_8601, iso_8601)
    }
    #[test]
    fn test_iso_week() {
        let iso_week = Date::new().iso_week();
        assert_eq!(iso_week, iso_week);
    }

    #[test]
    fn test_microsecond() {
        let microsecond = Date::new().microsecond();
        assert_eq!(microsecond, microsecond);
    }
    #[test]
    fn test_millisecond() {
        let millisecond = Date::new().millisecond();
        assert_eq!(millisecond, millisecond);
    }
    #[test]
    fn test_minute() {
        let minute = Date::new().minute();
        assert_eq!(minute, minute);
    }
    #[test]
    fn test_month() {
        let month = Date::new().month();
        assert_eq!(month, month);
    }
    #[test]
    fn test_nanosecond() {
        let nanosecond = Date::new().nanosecond();
        assert_eq!(nanosecond, nanosecond);
    }
    #[test]
    fn test_offset() {
        let offset = Date::new().offset();
        assert_eq!(offset, offset);
    }
    #[test]
    fn test_ordinal() {
        let ordinal = Date::new().ordinal();
        assert_eq!(ordinal, ordinal);
    }
    #[test]
    fn test_second() {
        let second = Date::new().second();
        assert_eq!(second, second);
    }
    #[test]
    fn test_time() {
        let time = Date::new().time();
        assert_eq!(time, time);
    }
    #[test]
    fn test_weekday() {
        let weekday = Date::new().weekday();
        assert_eq!(weekday, weekday);
    }
    #[test]
    fn test_year() {
        let year = Date::new().year();
        assert_eq!(year, year);
    }
}
