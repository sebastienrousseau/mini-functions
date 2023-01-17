#[cfg(test)]

mod tests {

    extern crate date;

    use self::date::Date;

    #[test]
    fn test_date() {
        let date = Date::new().date;
        assert_eq!(
            date.read().unwrap().to_string(),
            date.read().unwrap().to_string()
        );
    }

    #[test]
    fn test_day() {
        let day = Date::new().day;
        assert_eq!(
            day.read().unwrap().to_string(),
            day.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_hour() {
        let hour = Date::new().hour;
        assert_eq!(
            hour.read().unwrap().to_string(),
            hour.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_iso_8601() {
        let iso_8601 = Date::new().iso_8601;
        assert_eq!(
            iso_8601.read().unwrap().to_string(),
            iso_8601.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_iso_week() {
        let iso_week = Date::new().iso_week;
        assert_eq!(
            iso_week.read().unwrap().to_string(),
            iso_week.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_minute() {
        let minute = Date::new().minute;
        assert_eq!(
            minute.read().unwrap().to_string(),
            minute.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_month() {
        let month = Date::new().month;
        assert_eq!(
            month.read().unwrap().to_string(),
            month.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_offset() {
        let offset = Date::new().offset;
        assert_eq!(
            offset.read().unwrap().to_string(),
            offset.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_ordinal() {
        let ordinal = Date::new().ordinal;
        assert_eq!(
            ordinal.read().unwrap().to_string(),
            ordinal.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_second() {
        let second = Date::new().second;
        assert_eq!(
            second.read().unwrap().to_string(),
            second.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_time() {
        let time = Date::new().time;
        assert_eq!(
            time.read().unwrap().to_string(),
            time.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_weekday() {
        let weekday = Date::new().weekday;
        assert_eq!(
            weekday.read().unwrap().to_string(),
            weekday.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_year() {
        let year = Date::new().year;
        assert_eq!(
            year.read().unwrap().to_string(),
            year.read().unwrap().to_string()
        );
    }
}
