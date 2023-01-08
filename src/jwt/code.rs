macro_rules! declare_err {
    ($err:tt $(,)? $($doc:expr),+) => {
        $(
        #[doc = $doc]
        )*
        pub const $err: &str = stringify!(JwtError::$err);
    };
}

declare_err!(AUDIENCE_INVALID, "Audience invalid.");
declare_err!(EXPIRATION_INVALID, "Expiration invalid.");
declare_err!(FORMAT_INVALID, "Format invalid.");
declare_err!(IO_ERROR, "I/O error.");
declare_err!(ISSUER_INVALID, "Issuer invalid.");
declare_err!(JWT_INVALID, "JWT invalid.");
declare_err!(OPENSSL_ERROR, "Open SSL error.");
declare_err!(OPERATION_NOT_PERMITTED, "Operation not permitted.");
declare_err!(PROTOCOL_ERROR, "Protocol error.");
declare_err!(SIGNATURE_EXPIRED, "Signature expired.");
declare_err!(SIGNATURE_INVALID, "Signature invalid.");
declare_err!(SYSTEM_INTERRUPT, "System interrupt.");
