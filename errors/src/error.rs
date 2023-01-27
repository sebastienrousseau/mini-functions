use crate::common::ErrorType;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
/// Error struct
pub struct Error {
    pub message: String,
    pub error_type: ErrorType,
    pub ctx: String,
}

impl Error {
    /// Create a new error
    pub fn new(message: &str, error_type: ErrorType) -> Error {
        Error {
            message: message.to_owned(),
            error_type,
            ctx: "".to_owned(),
        }
    }

    /// Create a new error type
    pub fn error_type(name: &str, error_type: ErrorType, ctx: &str) -> Error {
        Error {
            message: name.to_owned(),
            error_type,
            ctx: ctx.to_owned(),
        }
    }

    /// Add context to the error
    pub fn with_context(mut self, ctx: &str) -> Error {
        self.ctx = ctx.to_owned();
        self
    }
}

impl StdError for Error {
    /// Source of the error
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    /// Display the error
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, ctx: {}", self.message, self.ctx)
    }
}

impl fmt::Display for ErrorType {
    /// Display the error type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
