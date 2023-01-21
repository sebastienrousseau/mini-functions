#[cfg(test)]

mod tests {
    extern crate claims;
    extern crate jot;

    use self::claims::Claims;
    use self::jot::{Algorithm, Header, JWT};

    #[test]
    fn test_encode() {
        let secret: &[u8; 6] = b"secret";
        let header = Header::default();
        let claims = Claims::default();
        let result = JWT::encode(header, claims, secret);
        assert_eq!(result.is_ok(), true);
    }
    #[test]
    fn test_default() {
        let jwt = JWT::default();
        assert_eq!(jwt.header.alg, Some(Algorithm::HS256));
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
    fn test_get_header() {
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
}
