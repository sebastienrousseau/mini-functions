#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate errors;
    use errors::common::{Error, ErrorType};

    #[test]
    fn test_error_type_new() {
        let error_type = ErrorType::new("illegal_argument");
        assert_eq!(error_type.name, "illegal_argument");
        match error_type.error_type {
            Error::IllegalArgument(s) => assert_eq!(s, "Illegal argument"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_error_type_new_subtype() {
        let error_type = ErrorType::new("illegal_argument");
        let subtype = error_type.new_subtype("subtype");
        assert_eq!(subtype.name, "subtype");
        match subtype.error_type {
            Error::UnknownError(s) => assert_eq!(s, "Unknown error"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_error_type_name() {
        let error_type = ErrorType::new("illegal_argument");
        assert_eq!(error_type.name, "illegal_argument");
    }

    #[test]
    fn test_error_type_error_type() {
        let error_type = ErrorType::new("illegal_argument");
        match error_type.error_type {
            Error::IllegalArgument(s) => assert_eq!(s, "Illegal argument"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_new() {
        let err_type = ErrorType {
            name: String::from("illegal_argument"),
            error_type: Error::IllegalArgument(String::from("illegal_argument")),
        };
        let err = Error::new(err_type);
        assert_eq!(
            err,
            Error::IllegalArgument(String::from("Illegal argument"))
        );

        let err_type = ErrorType {
            name: String::from("unsupported_operation"),
            error_type: Error::UnsupportedOperation(String::from("unsupported_operation")),
        };
        let err = Error::new(err_type);
        assert_eq!(
            err,
            Error::UnsupportedOperation(String::from("Unsupported operation"))
        );

        let err_type = ErrorType {
            name: String::from("unknown_error"),
            error_type: Error::UnknownError(String::from("unknown_error")),
        };
        let err = Error::new(err_type);
        assert_eq!(err, Error::UnknownError(String::from("Unknown error")));
    }
}
