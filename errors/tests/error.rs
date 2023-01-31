#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate errors;
    use errors::common::ErrorType;
    use errors::error::Error as OtherError;
    use std::error::Error as StdError;

    #[test]
    fn test_error_new() {
        let error_type = ErrorType::new("illegal_argument");
        let err = OtherError::new("message", error_type);
        assert_eq!(err.message, "message");
        assert_eq!(err.error_type.name, "illegal_argument");
    }
    #[test]
    fn test_with_context() {
        let error_type = ErrorType::new("test_error");
        let err =
            OtherError::with_context(OtherError::new("message", error_type), "Additional context");
        assert_eq!(err.message, "message");
        assert_eq!(err.error_type.name, "test_error");
        assert_eq!(err.ctx, "Additional context");
    }

    #[test]
    fn test_source() {
        let error_type = ErrorType::new("test_error");
        let err = OtherError::new("message", error_type);
        assert!(StdError::source(&err).is_none());
    }

    #[test]
    fn test_display() {
        let err = OtherError::new("Test error", ErrorType::new("test_error"));
        let display = format!("{err}");
        assert_eq!(display, "Test error, ctx: ");
    }

    #[test]
    fn test_display_error_type() {
        let err_type = ErrorType::new("test_error_type");
        let display = format!("{err_type}");
        assert_eq!(display, "test_error_type");
    }
}
