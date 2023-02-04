#[cfg(test)]

mod tests {

    extern crate claims;
    extern crate date;

    use self::claims::Claims;
    use self::date::DateTime;

    #[test]
    fn test_claims() {
        const CL_AUD: &str = "MINI-FUNCTIONS-CLAIMS-AUD";
        const CL_CUSTOM: &str = "MINI-FUNCTIONS-CLAIMS-CUSTOM";
        const CL_DID: &str = "MINI-FUNCTIONS-CLAIMS-DID";
        const CL_ISS: &str = "MINI-FUNCTIONS-CLAIMS-ISS";
        const CL_JTI: &str = "MINI-FUNCTIONS-CLAIMS-JTI";
        const CL_SUB: &str = "MINI-FUNCTIONS-CLAIMS-SUB";
        const CL_VC: &str = "MINI-FUNCTIONS-CLAIMS-VC";
        const CL_VP: &str = "MINI-FUNCTIONS-CLAIMS-VP";

        let date = DateTime::new();
        let iso = date.iso_8601;

        let mut claims = Claims::new();
        claims.set_claim("aud", CL_AUD);
        claims.set_claim("custom", CL_CUSTOM);
        claims.set_claim("did", CL_DID);
        claims.set_claim("exp", &iso);
        claims.set_claim("iat", &iso);
        claims.set_claim("iss", CL_ISS);
        claims.set_claim("jti", CL_JTI);
        claims.set_claim("nbf", &iso);
        claims.set_claim("sub", CL_SUB);
        claims.set_claim("vc", CL_VC);
        claims.set_claim("vp", CL_VP);

        assert_eq!(claims.get_claim("aud").unwrap(), CL_AUD);
        assert_eq!(claims.get_claim("custom").unwrap(), CL_CUSTOM);
        assert_eq!(claims.get_claim("did").unwrap(), CL_DID);
        assert_eq!(claims.get_claim("exp").unwrap(), &iso.to_string());
        assert_eq!(claims.get_claim("iat").unwrap(), &iso.to_string());
        assert_eq!(claims.get_claim("iss").unwrap(), CL_ISS);
        assert_eq!(claims.get_claim("jti").unwrap(), CL_JTI);
        assert_eq!(claims.get_claim("nbf").unwrap(), &iso.to_string());
        assert_eq!(claims.get_claim("sub").unwrap(), CL_SUB);
        assert_eq!(claims.get_claim("vc").unwrap(), CL_VC);
        assert_eq!(claims.get_claim("vp").unwrap(), CL_VP);
    }
    #[test]
    fn test_get_claim() {
        let mut claims = Claims::new();
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        assert_eq!(
            claims.get_claim("aud").unwrap(),
            "MINI-FUNCTIONS-CLAIMS-AUD"
        );
        assert!(claims.get_claim("non-existent-claim").is_none());
    }
    #[test]
    fn test_remove_claim() {
        let mut claims = Claims::new();
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        claims.remove_claim("aud");
        assert!(claims.get_claim("aud").is_none());
    }

    #[test]
    fn test_clear_claims() {
        let mut claims = Claims::new();
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        claims.set_claim("custom", "MINI-FUNCTIONS-CLAIMS-CUSTOM");
        claims.clear_claims();
        assert_eq!(claims.len(), 0);
    }
    #[test]
    fn test_has_claim() {
        let mut claims = Claims::new();
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        assert!(claims.has_claim("aud"));
        assert!(!claims.has_claim("non-existent-claim"));
    }

    #[test]
    fn test_len() {
        let mut claims = Claims::new();
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        claims.set_claim("custom", "MINI-FUNCTIONS-CLAIMS-CUSTOM");
        assert_eq!(claims.len(), 2);
    }
    #[test]
    fn test_is_empty() {
        let mut claims = Claims::new();
        assert!(claims.is_empty());
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        assert!(!claims.is_empty());
    }

    #[test]
    fn test_get_claims() {
        let mut claims = Claims::new();
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        claims.set_claim("custom", "MINI-FUNCTIONS-CLAIMS-CUSTOM");
        let retrieved_claims = claims.get_claims();
        assert_eq!(
            retrieved_claims.get("aud").unwrap(),
            "MINI-FUNCTIONS-CLAIMS-AUD"
        );
        assert_eq!(
            retrieved_claims.get("custom").unwrap(),
            "MINI-FUNCTIONS-CLAIMS-CUSTOM"
        );
    }
    #[test]
    fn test_display_trait() {
        let mut claims = Claims::new();
        claims.set_claim("aud", "MINI-FUNCTIONS-CLAIMS-AUD");
        claims.set_claim("custom", "MINI-FUNCTIONS-CLAIMS-CUSTOM");
        let display_output = format!("{claims}");
        assert!(display_output.contains("aud: MINI-FUNCTIONS-CLAIMS-AUD"));
        assert!(display_output.contains("custom: MINI-FUNCTIONS-CLAIMS-CUSTOM"));
    }

    #[test]
    fn test_default_trait() {
        let claims = Claims::default();
        assert_eq!(claims.claims.len(), 0);
    }
}
