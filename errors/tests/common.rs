#[cfg(test)]

// TODO: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate errors;
    use errors::common::{Error, ErrorType};

    #[test]
    fn test_error_type_new() {
        let error_type = ErrorType::new("argument");
        assert_eq!(error_type.name, "argument");
        match error_type.error_type {
            Error::Argument(s) => assert_eq!(s, "Illegal argument"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_error_type_new_subtype() {
        let error_type = ErrorType::new("illegal_argument");
        let subtype = error_type.new_subtype("subtype");
        assert_eq!(subtype.name, "subtype");
        match subtype.error_type {
            Error::Unknown(s) => assert_eq!(s, "Unknown error"),
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
        let error_type = ErrorType::new("argument");
        match error_type.error_type {
            Error::Argument(s) => assert_eq!(s, "Illegal argument"),
            _ => panic!("Unexpected error type"),
        }
    }

    #[test]
    fn test_new() {
        let err_type = ErrorType {
            name: String::from("argument"),
            error_type: Error::Argument(String::from("argument")),
        };
        let err = Error::new(err_type);
        assert_eq!(err, Error::Argument(String::from("Illegal argument")));

        let err_type = ErrorType {
            name: String::from("operation"),
            error_type: Error::Operation(String::from("operation")),
        };
        let err = Error::new(err_type);
        assert_eq!(err, Error::Operation(String::from("Unsupported operation")));

        let err_type = ErrorType {
            name: String::from("unknown_error"),
            error_type: Error::Unknown(String::from("unknown_error")),
        };
        let err = Error::new(err_type);
        assert_eq!(err, Error::Unknown(String::from("Unknown error")));
    }
}
