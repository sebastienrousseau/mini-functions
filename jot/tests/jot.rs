#[cfg(test)]

mod tests {
    extern crate claims;
    extern crate jot;

    use self::claims::Claims;
    use self::jot::{Algorithm, Header, JWT};

    #[test]
    fn test_header_default() {
        let header = Header::default();
        assert_eq!(header.alg, Some(Algorithm::HS256));
        assert_eq!(header.kid, None);
        assert_eq!(header.typ, Some("JWT".to_string()));
        assert_eq!(header.cty, None);
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
        jwt.token = encoded.clone();
        let decoded = JWT::decode(&mut jwt, secret);
        if let Ok(decoded_token) = decoded {
            assert_eq!(decoded_token, encoded);
        }
    }

    #[test]
    fn test_default() {
        let jwt = JWT::default();
        assert_eq!(jwt.header.alg, Some(Algorithm::HS256));
    }

    #[test]
    fn test_generate() {
        let secret = b"secret";
        let jwt = JWT::generate(secret);
        assert!(jwt.is_ok());
        let token = jwt.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_get_token() {
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
    fn test_get_token_header() {
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
    fn test_get_token_length() {
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
    fn test_to_string() {
        let jwt = JWT {
            header: Header::default(),
            claims: Claims::default(),
            signature: vec![],
            token: "example_token".to_owned(),
        };
        let result = jwt.to_string();
        assert_eq!(result, "JWT { header: Header { alg: Some(HS256), kid: None, typ: Some(\"JWT\"), cty: None }, claims: Claims {  }, signature: [], token: example_token }");
    }
    #[test]
    fn test_claims_default() {
        let claims = self::JWT::claims();
        assert_eq!(claims.is_empty(), true);
    }
}
