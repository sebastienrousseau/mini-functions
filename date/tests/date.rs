#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {

    extern crate date;
    use date::{is_valid, DateTime};

    extern crate time;
    use time::{Duration, OffsetDateTime};

    #[test]
    fn test_new() {
        let date = DateTime::new();
        assert!(!date.now.is_empty());
        assert!(date.day <= 31);
        assert!(date.hour <= 23);
        assert!(!date.iso_8601.is_empty());
        assert!(date.iso_week > 0 && date.iso_week <= 53);
        assert!(date.minute <= 59);
        assert!(!date.month.is_empty());
        assert!(!date.offset.is_empty());
        assert!(date.ordinal > 0 && date.ordinal <= 366);
        assert!(date.second <= 59);
        assert!(!date.time.is_empty());
        assert!(!date.weekday.is_empty());
        assert!(date.year > 0);
    }
    #[test]
    fn test_is_valid_day() {
        assert!(!DateTime::is_valid_day("32"));
        assert!(DateTime::is_valid_day("31"));
    }
    #[test]
    fn test_is_valid_hour() {
        assert!(DateTime::is_valid_hour("23"));
    }
    #[test]
    fn is_valid_iso_8601() {
        assert!(DateTime::is_valid_iso_8601(
            "2023-02-02 20:48:44.751609 +00:00:00"
        ));
    }
    #[test]
    fn test_update_date() {
        let date = DateTime::new();
        let date_before_update = date.now.clone();
        assert_eq!(date_before_update, date.now);
    }

    #[test]
    fn test_update_day() {
        let date = DateTime::new();
        let day_before_update = date.day;
        assert_eq!(day_before_update, date.day);
        assert!(date.day <= 31);
    }

    #[test]
    fn test_update_hour() {
        let date = DateTime::new();
        let hour_before_update = date.hour;
        assert_eq!(hour_before_update, date.hour);
        assert!(date.hour <= 23);
    }

    #[test]
    fn test_update_iso_8601() {
        let date = DateTime::new();
        let iso_8601_before_update = date.iso_8601.clone();
        assert_eq!(iso_8601_before_update, date.iso_8601);
    }

    #[test]
    fn test_update_iso_week() {
        let date = DateTime::new();
        let iso_week_before_update = date.iso_week;
        assert_eq!(iso_week_before_update, date.iso_week);
        assert!(date.iso_week <= 53);
    }

    #[test]
    fn test_update_minute() {
        let date = DateTime::new();
        let minute_before_update = date.minute;
        assert_eq!(minute_before_update, date.minute);
        assert!(date.minute <= 59);
    }

    #[test]
    fn test_update_month() {
        let date = DateTime::new();
        let month_before_update = date.month.clone();
        assert_eq!(month_before_update, date.month);
    }

    #[test]
    fn test_update_offset() {
        let date = DateTime::new();
        let offset_before_update = date.offset.clone();
        assert_eq!(offset_before_update, date.offset);
    }

    #[test]
    fn test_update_ordinal() {
        let date = DateTime::new();
        let ordinal_before_update = date.ordinal;
        assert_eq!(ordinal_before_update, date.ordinal);
        assert!(date.ordinal > 0 && date.ordinal <= 366);
    }
    #[test]
    fn test_update_second() {
        let date = DateTime::new();
        let second_before_update = date.second;
        assert_eq!(second_before_update, date.second);
        assert!(date.second <= 59);
    }
    #[test]
    fn test_update_time() {
        let date = DateTime::new();
        let time_before_update = date.time.clone();
        assert_eq!(time_before_update, date.time);
    }

    #[test]
    fn test_update_weekday() {
        let date = DateTime::new();
        let weekday_before_update = date.weekday.clone();
        assert_eq!(weekday_before_update, date.weekday);
    }
    #[test]
    fn test_update_year() {
        let date = DateTime::new();
        let year_before_update = date.year;
        assert_eq!(year_before_update, date.year);
        assert!(date.year > 0);
    }
    #[test]
    fn test_new_with_tz_utc() {
        let date_time = DateTime::new_with_tz("UTC");
        let offset = time::UtcOffset::UTC;

        let now_utc = OffsetDateTime::now_utc();
        let (hours, minutes, _) = offset.as_hms();
        let total_seconds = (hours as i16 * 3600) + (minutes as i16 * 60);
        let expected_date_time = now_utc + Duration::seconds(total_seconds as i64);

        assert_eq!(date_time.hour, expected_date_time.hour());
        assert_eq!(date_time.minute, expected_date_time.minute());
        assert_eq!(date_time.offset, expected_date_time.offset().to_string());
    }
    #[test]
    fn test_new_with_tz_custom() {
        let date_time = DateTime::new_with_tz("Custom");
        let offset = time::UtcOffset::from_hms(0, 0, 0).unwrap();

        let now_utc = OffsetDateTime::now_utc();
        let (hours, minutes, _) = offset.as_hms();
        let total_seconds = (hours as i16 * 3600) + (minutes as i16 * 60);
        let expected_date_time = now_utc + Duration::seconds(total_seconds as i64);

        assert_eq!(date_time.hour, expected_date_time.hour());
        assert_eq!(date_time.minute, expected_date_time.minute());
        assert_eq!(date_time.offset, expected_date_time.offset().to_string());
    }
    #[test]
    fn test_new_with_tz_to_paris() {
        let date = DateTime::new_with_tz("Europe/Paris");
        assert!(!date.now.is_empty());
        assert!(date.day <= 31);
        assert!(date.hour <= 23);
        assert!(!date.iso_8601.is_empty());
        assert!(date.iso_week > 0 && date.iso_week <= 53);
        assert!(date.minute <= 59);
        assert!(!date.month.is_empty());
        assert!(!date.offset.is_empty());
        assert!(date.ordinal > 0 && date.ordinal <= 366);
        assert!(date.second <= 59);
        assert!(!date.time.is_empty());
        assert!(!date.weekday.is_empty());
        assert!(date.year > 0);
    }
    #[test]
    fn test_display_format() {
        let date_time = DateTime::new();
        let formatted = format!("{date_time}");

        let expected = format!(
            "{}T{:02}:{:02}:{:02}{}",
            date_time.now, date_time.hour, date_time.minute, date_time.second, date_time.offset
        );
        assert_eq!(formatted, expected);
    }
    #[test]
    fn test_formatting_date_time() {
        let date_time = DateTime::new();
        let formatted = format!("{date_time}");

        let expected = format!(
            "{}T{:02}:{:02}:{:02}{}",
            date_time.now, date_time.hour, date_time.minute, date_time.second, date_time.offset
        );
        assert_eq!(formatted, expected);
    }
    #[test]
    fn test_is_valid() {
        is_valid!(day, u32);
        let input = "31";
        let result = day(input);
        assert!(result);
    }
}
