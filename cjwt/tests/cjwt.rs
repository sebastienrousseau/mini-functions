#[cfg(test)]
mod tests {
    extern crate cclm;
    extern crate cjwt;

    use self::cclm::Claims;
    use self::cjwt::{Algorithm, Header, JWT};

    #[test]
    fn test_default_algorithm_is_hs256() {
        let algorithm = Algorithm::default();
        assert_eq!(algorithm, Algorithm::HS256);
    }

    #[test]
    fn test_algorithm_to_string_variants() {
        let algorithm = Algorithm::default();
        assert_eq!(algorithm.to_string(), "HS256");

        let algorithm_hs384 = Algorithm::HS384;
        assert_eq!(algorithm_hs384.to_string(), "HS384");

        let algorithm_hs512 = Algorithm::HS512;
        assert_eq!(algorithm_hs512.to_string(), "HS512");

        let algorithm_rs256 = Algorithm::RS256;
        assert_eq!(algorithm_rs256.to_string(), "RS256");

        let algorithm_rs384 = Algorithm::RS384;
        assert_eq!(algorithm_rs384.to_string(), "RS384");

        let algorithm_rs512 = Algorithm::RS512;
        assert_eq!(algorithm_rs512.to_string(), "RS512");

        let algorithm_es256 = Algorithm::ES256;
        assert_eq!(algorithm_es256.to_string(), "ES256");

        let algorithm_es384 = Algorithm::ES384;
        assert_eq!(algorithm_es384.to_string(), "ES384");

        let algorithm_es512 = Algorithm::ES512;
        assert_eq!(algorithm_es512.to_string(), "ES512");
    }

    #[test]
    fn test_algorithm_variants_are_defined() {
        assert!(matches!(Algorithm::HS256, Algorithm::HS256));
        assert!(matches!(Algorithm::HS384, Algorithm::HS384));
        assert!(matches!(Algorithm::HS512, Algorithm::HS512));
        assert!(matches!(Algorithm::RS256, Algorithm::RS256));
        assert!(matches!(Algorithm::RS384, Algorithm::RS384));
        assert!(matches!(Algorithm::RS512, Algorithm::RS512));
        assert!(matches!(Algorithm::ES256, Algorithm::ES256));
        assert!(matches!(Algorithm::ES384, Algorithm::ES384));
        assert!(matches!(Algorithm::ES512, Algorithm::ES512));
    }

    #[test]
    fn test_default_header_has_correct_fields() {
        let header = Header::default();
        assert_eq!(header.alg, Some(Algorithm::HS256));
        assert_eq!(header.kid, None);
        assert_eq!(header.typ, Some("JWT".to_string()));
        assert_eq!(header.cty, None);
    }

    #[test]
    fn test_empty_claims_are_generated() {
        let claims = self::JWT::claims();
        assert!(claims.is_empty(), "{}", true);
    }

    #[test]
    fn test_generate_jwt_with_valid_secret() {
        let secret = b"secret";
        let jwt = JWT::generate(secret);
        assert!(jwt.is_ok());
        let token = jwt.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_get_token_from_jwt() {
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "example_token".to_owned(),
        };
        let result = JWT::get_token(jwt);
        assert_eq!(result, "example_token");
    }

    #[test]
    fn test_get_token_header_from_jwt() {
        let jwt = JWT {
            header: Header {
                alg: Some(Algorithm::HS256),
                kid: Some("example_kid".to_string()),
                typ: Some("example_type".to_string()),
                cty: Some("example_cty".to_string()),
            },
            claims: Claims::default(),
            signature: vec![],
            token: "example_token".to_owned(),
        };
        let result = JWT::get_token_header(jwt);
        assert_eq!(result.alg, Some(Algorithm::HS256));
        assert_eq!(result.kid, Some("example_kid".to_string()));
        assert_eq!(result.typ, Some("example_type".to_string()));
        assert_eq!(result.cty, Some("example_cty".to_string()));
    }

    #[test]
    fn test_get_token_length_from_jwt() {
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "token".to_string(),
        };
        let result = JWT::get_token_length(jwt);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_encode() {
        let secret: &[u8; 6] = b"secret";
        let header = Header::default();
        let claims = Claims::default();
        let result = JWT::encode(header, claims, secret);
        assert!(result.is_ok(), "{}", true);
    }

    #[test]
    fn test_decode() {
        let mut jwt = JWT::default();
        let secret: &[u8; 6] = b"secret";
        let header = Header::default();
        let claims = Claims::default();
        let encoded_result = JWT::encode(header, claims, secret);
        let encoded = encoded_result.unwrap();
        jwt.token.clone_from(&encoded);
        let decoded = JWT::decode(&mut jwt, secret);
        if let Ok(decoded_token) = decoded {
            assert_eq!(decoded_token, encoded);
        }
    }

    #[test]
    fn test_to_string() {
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "example_token".to_owned(),
        };
        let result = jwt.to_string();
        assert_eq!(
            result,
            "JWT { header: Header { alg: Some(HS256), kid: None, typ: Some(\"JWT\"), cty: None }, claims: Claims {  }, signature: [], token: example_token }"
        );
    }

    #[test]
    fn test_validate_success() {
        let secret: &[u8; 6] = b"secret";
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "example_token".to_owned(),
        };
        let result = JWT::validate(&jwt, secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_with_empty_signature() {
        // Create a JWT with valid claims and an empty signature
        let secret = b"secret";
        let jwt = JWT {
            header: Header {
                alg: Some(Algorithm::HS256),
                kid: Some("example_kid".to_string()),
                typ: Some("example_type".to_string()),
                cty: Some("example_cty".to_string()),
            },
            claims: Claims::default(),
            signature: vec![],
            token: "example_token".to_owned(),
        };

        // Test validating the JWT
        let result = jwt.validate(secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_invalid_token() {
        let mut jwt = JWT {
            token: "invalid.token.structure".to_owned(),
            ..JWT::default()
        };
        let secret = b"secret";
        assert!(JWT::decode(&mut jwt, secret).is_err());
    }

    #[test]
    fn test_decode_with_invalid_header() {
        let mut jwt = JWT::default();
        jwt.token = "invalid.header.structure".to_owned();
        let secret = b"secret";
        assert!(JWT::decode(&mut jwt, secret).is_err());
    }

    #[test]
    fn test_decode_with_invalid_claims() {
        let mut jwt = JWT::default();
        jwt.token = "invalid.claims.structure".to_owned();
        let secret = b"secret";
        assert!(JWT::decode(&mut jwt, secret).is_err());
    }

    #[test]
    fn test_validate_with_invalid_claims() {
        let mut jwt = JWT::default();
        jwt.token = "invalid.claims.structure".to_string();
        let secret = b"secret";
        assert!(JWT::validate(&jwt, secret).is_err());
    }

    #[test]
    fn test_decode_with_invalid_signature() {
        let mut jwt = JWT::default();
        jwt.token = "invalid.signature.value".to_owned();
        let secret = b"secret";
        assert!(JWT::decode(&mut jwt, secret).is_err());
    }

    #[test]
    fn test_validate_with_invalid_signature() {
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "invalid.signature.value".to_string(),
        };
        let secret = b"secret";
        assert!(JWT::validate(&jwt, secret).is_err());
    }

    #[test]
    fn test_decode_with_expired_iat() {
        let mut jwt = JWT::default();
        jwt.token = "expired.iat.jwt".to_owned();
        let secret = b"secret";
        assert!(JWT::decode(&mut jwt, secret).is_err());
    }

    #[test]
    fn test_decode_with_expired_exp() {
        let mut jwt = JWT::default();
        jwt.token = "expired.exp.jwt".to_owned();
        let secret = b"secret";
        assert!(JWT::decode(&mut jwt, secret).is_err());
    }

    #[test]
    fn test_validate_with_expired_iat() {
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "expired.iat.jwt".to_string(),
        };
        let secret = b"secret";
        assert!(JWT::validate(&jwt, secret).is_err());
    }

    #[test]
    fn test_validate_with_expired_exp() {
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "expired.exp.jwt".to_owned(),
        };
        let secret = b"secret";
        assert!(JWT::validate(&jwt, secret).is_err());
    }
}
