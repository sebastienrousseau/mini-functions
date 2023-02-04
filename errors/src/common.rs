/// ErrorType is a struct that holds a name and an Error enum instance
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ErrorType {
    /// name is a string that holds the name of the error type
    pub name: String,
    /// error_type is an instance of the Error enum
    pub error_type: Error,
}
/// Error is an enumeration of different error types
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Error {
    /// Argument error
    Argument(String),
    /// Assertion error
    Assertion(String),
    /// Concurrency error
    Concurrency(String),
    /// Data error
    Data(String),
    /// External error
    External(String),
    /// Format error
    Format(String),
    /// Implementation error
    Implementation(String),
    /// Initialization error
    Initialization(String),
    /// Internal error
    Internal(String),
    /// Interruption error
    Interruption(String),
    /// Operation error
    Operation(String),
    /// Rejection error
    Rejection(String),
    /// State error
    State(String),
    /// Timeout error
    Timeout(String),
    /// Unknown error
    Unknown(String),
    /// Version error
    Version(String),
}

impl Error {
    /// new creates a new Error enum instance based on the name
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
    /// new creates a new ErrorType struct instance
    pub fn new(name: &str) -> ErrorType {
        ErrorType {
            name: String::from(name),
            error_type: Error::new(ErrorType {
                name: name.to_string(),
                error_type: Error::Unknown(String::from(name)),
            }),
        }
    }
    /// new_subtype creates a new ErrorType struct instance
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
