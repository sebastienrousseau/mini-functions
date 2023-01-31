#[cfg(test)]

// FIXME: Add more tests to bring the code coverage to 100%
mod tests {
    extern crate errors;
    use errors::jwt::JwtError;

    #[test]
    fn test_jwt_error() {
        let jwt_error = JwtError::JWTInvalid("JWT is invalid".to_string());
        assert!(jwt_error.is_jwt_error());
        assert!(!jwt_error.is_signature_error());
        assert!(!jwt_error.is_issuer_error());
        assert!(!jwt_error.is_expiration_error());
        assert!(!jwt_error.is_audience_error());
    }
    #[test]
    fn test_jwt_display() {
        let jwt_audience_error = JwtError::AudienceInvalid("Audience is invalid".to_string());
        assert!(jwt_audience_error
            .to_string()
            .contains("Audience is invalid"));

        let jwt_expiration_invalid =
            JwtError::ExpirationInvalid("Expiration is invalid".to_string());
        assert!(jwt_expiration_invalid
            .to_string()
            .contains("Expiration is invalid"));

        let jwt_error = JwtError::default();
        assert_eq!(jwt_error.to_string(), "Unknown error");
    }
    #[test]
    fn test_signature_error() {
        let jwt_error = JwtError::SignatureInvalid("Signature is invalid".to_string());
        assert!(!jwt_error.is_jwt_error());
        assert!(jwt_error.is_signature_error());
        assert!(!jwt_error.is_issuer_error());
        assert!(!jwt_error.is_expiration_error());
        assert!(!jwt_error.is_audience_error());
        assert!(!jwt_error.is_format_error());
        assert!(!jwt_error.is_invalid_length_error());
        assert!(!jwt_error.is_io_error());
        assert!(!jwt_error.is_openssl_error());
        assert!(!jwt_error.is_protocol_error());
        assert!(!jwt_error.is_token_not_found_error());
        assert!(!jwt_error.is_base64_error());
    }
}
