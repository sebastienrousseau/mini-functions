// Imports various error types that can be converted into the custom `Error` enum defined in this file.

extern crate base64;
// extern crate openssl;
extern crate serde_json;

use base64::DecodeError as B64Error;
// use openssl::error::ErrorStack;
use serde_json::Error as SJError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// Custom error type for JWT. This type is used to represent all
/// possible errors that can occur when working with JWTs. It is
/// implemented as an enum with variants for each possible error. It
/// also implements the `Default`, `Display`, and `Error` traits.
pub enum JwtError {
    AudienceInvalid(String),
    ExpirationInvalid(String),
    FormatInvalid(String),
    InvalidBase64(String),
    InvalidHeader(String),
    InvalidPayload(String),
    InvalidSignature(String),
    IoError(String),
    IssuerInvalid(String),
    JWTInvalid(String),
    OpenSslError(String),
    ProtocolError(String),
    SignatureExpired(String),
    SignatureInvalid(String),
    TokenNotFound(String),
}

impl StdError for JwtError {}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JwtError::AudienceInvalid(err) => write!(f, "Audience Invalid Error: {err}"),
            JwtError::ExpirationInvalid(err) => write!(f, "Expiration Invalid Error: {err}"),
            // implement display for other variants as well
            _ => write!(f, "Unknown error"),
        }
    }
}

impl From<B64Error> for JwtError {
    fn from(error: B64Error) -> Self {
        JwtError::InvalidBase64(error.to_string())
    }
}

impl From<SJError> for JwtError {
    fn from(error: SJError) -> Self {
        JwtError::InvalidPayload(error.to_string())
    }
}

// impl From<ErrorStack> for JwtError {
//     fn from(error: ErrorStack) -> Self {
//         JwtError::OpenSslError(error.to_string())
//     }
// }

impl From<IoError> for JwtError {
    fn from(error: IoError) -> Self {
        JwtError::IoError(error.to_string())
    }
}

impl JwtError {
    /// Returns `true` if the error is a signature error.
    pub fn is_signature_error(&self) -> bool {
        matches!(
            self,
            JwtError::SignatureExpired(_) | JwtError::SignatureInvalid(_)
        )
    }

    /// Returns `true` if the error is a JWT error.
    pub fn is_jwt_error(&self) -> bool {
        matches!(self, JwtError::JWTInvalid(_))
    }

    /// Returns `true` if the error is an issuer error.
    pub fn is_issuer_error(&self) -> bool {
        matches!(self, JwtError::IssuerInvalid(_))
    }

    /// Returns `true` if the error is an expiration error.
    pub fn is_expiration_error(&self) -> bool {
        matches!(self, JwtError::ExpirationInvalid(_))
    }

    /// Returns `true` if the error is an audience error.
    pub fn is_audience_error(&self) -> bool {
        matches!(self, JwtError::AudienceInvalid(_))
    }

    /// Returns `true` if the error is a format error.
    pub fn is_format_error(&self) -> bool {
        matches!(self, JwtError::FormatInvalid(_))
    }

    /// Returns `true` if the error is an IO error.
    pub fn is_io_error(&self) -> bool {
        matches!(self, JwtError::IoError(_))
    }

    /// Returns `true` if the error is an Open SSL error.
    pub fn is_openssl_error(&self) -> bool {
        matches!(self, JwtError::OpenSslError(_))
    }

    /// Returns `true` if the error is a protocol error.
    pub fn is_protocol_error(&self) -> bool {
        matches!(self, JwtError::ProtocolError(_))
    }
    /// Returns `true` if the error is a token not found error.
    pub fn is_token_not_found_error(&self) -> bool {
        matches!(self, JwtError::TokenNotFound(_))
    }
    /// Returns `true` if the error is an invalid base 64.
    pub fn is_base64_error(&self) -> bool {
        matches!(self, JwtError::InvalidBase64(_))
    }
}

// Implementation of `Default` for `Error` to provide a default error.
impl Default for JwtError {
    fn default() -> Self {
        Self::SignatureExpired("Signature expired.".to_owned())
    }
}
