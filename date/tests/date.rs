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
    #[test]
    fn test_default() {
        let date1 = Date::default();
        let date2 = Date::new();
        assert_eq!(
            date1.date.read().unwrap().to_string(),
            date2.date.read().unwrap().to_string()
        );
    }
    #[test]
    fn test_clone() {
        let date1 = Date::new();
        let date2 = date1.clone();

        assert_eq!(
            date1.date.read().unwrap().to_string(),
            date2.date.read().unwrap().to_string()
        );
        assert_ne!(
            &date1.date.try_read().unwrap() as *const _,
            &date2.date.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.day.try_read().unwrap() as *const _,
            &date2.day.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.hour.try_read().unwrap() as *const _,
            &date2.hour.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.iso_8601.try_read().unwrap() as *const _,
            &date2.iso_8601.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.iso_week.try_read().unwrap() as *const _,
            &date2.iso_week.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.minute.try_read().unwrap() as *const _,
            &date2.minute.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.month.try_read().unwrap() as *const _,
            &date2.month.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.offset.try_read().unwrap() as *const _,
            &date2.offset.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.ordinal.try_read().unwrap() as *const _,
            &date2.ordinal.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.second.try_read().unwrap() as *const _,
            &date2.second.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.time.try_read().unwrap() as *const _,
            &date2.time.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.weekday.try_read().unwrap() as *const _,
            &date2.weekday.try_read().unwrap() as *const _
        );
        assert_ne!(
            &date1.year.try_read().unwrap() as *const _,
            &date2.year.try_read().unwrap() as *const _
        );
    }
    #[test]
    fn test_display() {
        let date = Date::new();
        let display = format!("{}", date);

        assert!(display.contains("date: "));
        assert!(display.contains("day: "));
        assert!(display.contains("hour: "));
        assert!(display.contains("iso_8601: "));
        assert!(display.contains("iso_week: "));
        assert!(display.contains("minute: "));
        assert!(display.contains("month: "));
        assert!(display.contains("offset: "));
        assert!(display.contains("ordinal: "));
        assert!(display.contains("second: "));
        assert!(display.contains("time: "));
        assert!(display.contains("weekday: "));
        assert!(display.contains("year: "));
    }

    #[test]
    fn test_display_with_write_lock() {
        let date = Date::new();
        *date.year.write().unwrap() = 2020;
        let display = format!("{}", date);

        assert!(display.contains("year: 2020"));
    }
}
