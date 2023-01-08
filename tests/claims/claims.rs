#[cfg(test)]

mod tests {

    extern crate mini_functions;

    use mini_functions::claims::claims::Claims;

    #[test]
    fn test_new() {
        let claims = Claims::new(
            "2023-12-23T23:23:23.222222+00:00",
            "2023-12-23T23:23:23.222222+00:00",
            "issuer",
            "subject",
            "audience",
        );
        assert_eq!(claims.exp(), "2023-12-23T23:23:23.222222+00:00");
        assert_eq!(claims.iat(), "2023-12-23T23:23:23.222222+00:00");
        assert_eq!(claims.iss(), "issuer");
        assert_eq!(claims.sub(), "subject");
        assert_eq!(claims.aud(), "audience");
    }
    #[test]
    fn test_exp() {
        let claims = Claims::new(
            "2023-12-23T23:23:23.222222+00:00",
            "2023-12-23T23:23:23.222222+00:00",
            "issuer",
            "subject",
            "audience",
        );
        assert_eq!(claims.exp(), "2023-12-23T23:23:23.222222+00:00");
    }
    #[test]
    fn test_iat() {
        let claims = Claims::new(
            "2023-12-23T23:23:23.222222+00:00",
            "2023-12-23T23:23:23.222222+00:00",
            "issuer",
            "subject",
            "audience",
        );
        assert_eq!(claims.iat(), "2023-12-23T23:23:23.222222+00:00");
    }
    #[test]
    fn test_iss() {
        let claims = Claims::new(
            "2023-12-23T23:23:23.222222+00:00",
            "2023-12-23T23:23:23.222222+00:00",
            "issuer",
            "subject",
            "audience",
        );
        assert_eq!(claims.iss(), "issuer");
    }
    #[test]
    fn test_sub() {
        let claims = Claims::new(
            "2023-12-23T23:23:23.222222+00:00",
            "2023-12-23T23:23:23.222222+00:00",
            "issuer",
            "subject",
            "audience",
        );
        assert_eq!(claims.sub(), "subject");
    }
    #[test]
    fn test_aud() {
        let claims = Claims::new(
            "2023-12-23T23:23:23.222222+00:00",
            "2023-12-23T23:23:23.222222+00:00",
            "issuer",
            "subject",
            "audience",
        );
        assert_eq!(claims.aud(), "audience");
    }
}
