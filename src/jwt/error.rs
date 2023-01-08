// Imports various error types that can be converted into the custom `Error` enum defined in this file.
use base64::DecodeError as B64Error;
use openssl::error::ErrorStack;
use serde_json::Error as SJError;
use std::error;
use std::fmt;
use std::io::Error as IoError;

// Macro to easily implement the `From` trait for converting from
// another error type to `Error`.
macro_rules! impl_error {
    ($from:ty, $to:path) => {
        impl From<$from> for JwtError {
            fn from(e: $from) -> Self {
                $to(format!("{:?}", e))
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Custom error type for JWT. This type is used to represent all
/// possible errors that can occur when working with JWTs. It is
/// implemented as an enum with variants for each possible error. It
/// also implements the `Default`, `Display`, and `Error` traits.
pub enum JwtError {
    /// Signature expired.
    SignatureExpired,
    /// Signature invalid.
    SignatureInvalid,
    /// JWT invalid.
    JWTInvalid,
    /// Issuer invalid.
    IssuerInvalid,
    /// Expiration invalid.
    ExpirationInvalid,
    /// Audience invalid.
    AudienceInvalid,
    /// Format invalid.
    FormatInvalid(String),
    /// IO error.
    IoError(String),
    /// Open SSL error.
    OpenSslError(String),
    /// Protocol error.
    ProtocolError(String),
}

impl JwtError {
    /// Returns `true` if the error is a signature error.
    pub fn is_signature_error(&self) -> bool {
        match self {
            JwtError::SignatureExpired | JwtError::SignatureInvalid => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is a JWT error.
    pub fn is_jwt_error(&self) -> bool {
        match self {
            JwtError::JWTInvalid => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is an issuer error.
    pub fn is_issuer_error(&self) -> bool {
        match self {
            JwtError::IssuerInvalid => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is an expiration error.
    pub fn is_expiration_error(&self) -> bool {
        match self {
            JwtError::ExpirationInvalid => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is an audience error.
    pub fn is_audience_error(&self) -> bool {
        match self {
            JwtError::AudienceInvalid => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is a format error.
    pub fn is_format_error(&self) -> bool {
        match self {
            JwtError::FormatInvalid(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is an IO error.
    pub fn is_io_error(&self) -> bool {
        match self {
            JwtError::IoError(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is an Open SSL error.
    pub fn is_openssl_error(&self) -> bool {
        match self {
            JwtError::OpenSslError(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if the error is a protocol error.
    pub fn is_protocol_error(&self) -> bool {
        match self {
            JwtError::ProtocolError(_) => true,
            _ => false,
        }
    }
}

// Implementation of `Default` for `Error` to provide a default error.
impl Default for JwtError {
    fn default() -> Self {
        Self::SignatureExpired
    }
}

// Implementation of `Display` for `Error` to provide a human-readable error message.
impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JwtError::SignatureExpired => write!(f, "Signature expired."),
            JwtError::SignatureInvalid => write!(f, "Signature invalid."),
            JwtError::JWTInvalid => write!(f, "JWT invalid."),
            JwtError::IssuerInvalid => write!(f, "Issuer invalid."),
            JwtError::ExpirationInvalid => write!(f, "Expiration invalid."),
            JwtError::AudienceInvalid => write!(f, "Audience invalid."),
            JwtError::FormatInvalid(msg) => write!(f, "Format invalid: {}.", msg),
            JwtError::IoError(msg) => write!(f, "IO error: {}.", msg),
            JwtError::OpenSslError(msg) => write!(f, "Open SSL error: {}.", msg),
            JwtError::ProtocolError(msg) => write!(f, "Protocol error: {}.", msg),
        }
    }
}

// Implementation of `Error` for `Error` to enable use of `Error` with the `?` operator.
impl error::Error for JwtError {}

// Uses the `impl_error!` macro to implement the `From` trait for converting from `IoError` to `Error`.
impl_error! {IoError, JwtError::IoError}

// Uses the `impl_error!` macro to implement the `From` trait for converting from `SJError` to `Error`.
impl_error! {SJError, JwtError::FormatInvalid}

// Uses the `impl_error!` macro to implement the `From` trait for converting from `ErrorStack` to `Error`.
impl_error! {ErrorStack, JwtError::OpenSslError}

// Uses the `impl_error!` macro to implement the `From` trait for converting from `B64Error` to `Error`.
impl_error! {B64Error, JwtError::ProtocolError}
