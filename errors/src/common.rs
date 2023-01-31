#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ErrorType {
    pub name: String,
    pub error_type: Error,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    Argument(String),
    Assertion(String),
    Concurrency(String),
    Data(String),
    External(String),
    Format(String),
    Implementation(String),
    Initialization(String),
    Internal(String),
    Interruption(String),
    Operation(String),
    Rejection(String),
    State(String),
    Timeout(String),
    Unknown(String),
    Version(String),
}

impl Error {
    pub fn new(err_type: ErrorType) -> Self {
        match err_type.name.as_str() {
            "argument" => Error::Argument(String::from("Illegal argument")),
            "assertion" => Error::Assertion(String::from("Assertion failed")),
            "concurrency" => Error::Concurrency(String::from("Concurrency update")),
            "data" => Error::Data(String::from("Data unavailable")),
            "external" => Error::External(String::from("External error")),
            "format" => Error::Format(String::from("Illegal format")),
            "implementation" => Error::Implementation(String::from("No implementation")),
            "initialization" => Error::Initialization(String::from("Initialization failed")),
            "internal" => Error::Internal(String::from("Internal error")),
            "interruption" => Error::Interruption(String::from("Interruption occurred")),
            "operation" => Error::Operation(String::from("Unsupported operation")),
            "rejection" => Error::Rejection(String::from("Rejection occurred")),
            "state" => Error::State(String::from("Illegal state")),
            "timeout" => Error::Timeout(String::from("Timeout elapsed")),
            "version" => Error::Version(String::from("Unsupported version")),
            _ => Error::Unknown(String::from("Unknown error")),
        }
    }
}

impl ErrorType {
    pub fn new(name: &str) -> ErrorType {
        ErrorType {
            name: String::from(name),
            error_type: Error::new(ErrorType {
                name: name.to_string(),
                error_type: Error::Unknown(String::from(name)),
            }),
        }
    }
    pub fn new_subtype(&self, name: &str) -> ErrorType {
        ErrorType {
            name: String::from(name),
            error_type: Error::new(ErrorType {
                name: name.to_string(),
                error_type: Error::Unknown(String::from(name)),
            }),
        }
    }
}
