#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ErrorType {
    pub name: String,
    pub error_type: Error,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    IllegalArgument(String),
    IllegalState(String),
    IllegalFormat(String),
    NotImplemented(String),
    UnsupportedOperation(String),
    UnsupportedVersion(String),
    InitializationFailed(String),
    ConcurrentUpdate(String),
    DataUnavailable(String),
    RejectedOperation(String),
    TimeoutElapsed(String),
    AssertionFailed(String),
    Interrupted(String),
    ExternalError(String),
    InternalError(String),
    UnknownError(String),
}

impl Error {
    pub fn new(err_type: ErrorType) -> Self {
        match err_type.name.as_str() {
            "illegal_argument" => Error::IllegalArgument(String::from("Illegal argument")),
            "illegal_state" => Error::IllegalState(String::from("Illegal state")),
            "illegal_format" => Error::IllegalFormat(String::from("Illegal format")),
            "not_implemented" => Error::NotImplemented(String::from("Not implemented")),
            "unsupported_operation" => {
                Error::UnsupportedOperation(String::from("Unsupported operation"))
            }
            "unsupported_version" => Error::UnsupportedVersion(String::from("Unsupported version")),
            "initialization_failed" => {
                Error::InitializationFailed(String::from("Initialization failed"))
            }
            "concurrent_update" => Error::ConcurrentUpdate(String::from("Concurrent update")),
            "data_unavailable" => Error::DataUnavailable(String::from("Data unavailable")),
            "rejected_operation" => Error::RejectedOperation(String::from("Rejected operation")),
            "timeout_elapsed" => Error::TimeoutElapsed(String::from("Timeout elapsed")),
            "assertion_failed" => Error::AssertionFailed(String::from("Assertion failed")),
            "interrupted" => Error::Interrupted(String::from("Interrupted")),
            "external_error" => Error::ExternalError(String::from("External error")),
            "internal_error" => Error::InternalError(String::from("Internal error")),
            _ => Error::UnknownError(String::from("Unknown error")),
        }
    }
}

impl ErrorType {
    pub fn new(name: &str) -> ErrorType {
        ErrorType {
            name: String::from(name),
            error_type: Error::new(ErrorType {
                name: name.to_string(),
                error_type: Error::UnknownError(String::from(name)),
            }),
        }
    }
    pub fn new_subtype(&self, name: &str) -> ErrorType {
        ErrorType {
            name: String::from(name),
            error_type: Error::new(ErrorType {
                name: name.to_string(),
                error_type: Error::UnknownError(String::from(name)),
            }),
        }
    }
}
