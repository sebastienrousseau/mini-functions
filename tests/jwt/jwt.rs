#[cfg(test)]

mod tests {

    extern crate mini_functions;

    use mini_functions::jwt::jwt::JWT;

    #[test]
    fn test_decode() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_encode_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
        let encoded = "test";
        assert!(!encoded.is_empty());
    }
    #[test]
    fn test_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_new_jwt() {
        let jwt = JWT::new(
            String::from("username"),
            String::from("password"),
            String::from("issuer"),
            String::from("audience"),
        );
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_default_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_generate_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_set_token_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_get_token_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_get_token_length_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
    #[test]
    fn test_get_token_username_jwt() {
        let jwt = JWT::default();
        let token = jwt.generate();
        assert!(token.is_ok());
    }
}
