#[cfg(test)]

mod tests {

    extern crate claims;
    extern crate date;

    use self::claims::Claims;
    use self::date::Date;

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

        let date = Date::new().iso_8601;

        let mut claims = Claims::new();
        claims.set_claim("aud", CL_AUD);
        claims.set_claim("custom", CL_CUSTOM);
        claims.set_claim("did", CL_DID);
        claims.set_claim("exp", &date.read().unwrap().to_string());
        claims.set_claim("iat", &date.read().unwrap().to_string());
        claims.set_claim("iss", CL_ISS);
        claims.set_claim("jti", CL_JTI);
        claims.set_claim("nbf", &date.read().unwrap().to_string());
        claims.set_claim("sub", CL_SUB);
        claims.set_claim("vc", CL_VC);
        claims.set_claim("vp", CL_VP);

        assert_eq!(claims.get_claim("aud").unwrap(), CL_AUD);
        assert_eq!(claims.get_claim("custom").unwrap(), CL_CUSTOM);
        assert_eq!(claims.get_claim("did").unwrap(), CL_DID);
        assert_eq!(
            claims.get_claim("exp").unwrap(),
            &date.read().unwrap().to_string()
        );
        assert_eq!(
            claims.get_claim("iat").unwrap(),
            &date.read().unwrap().to_string()
        );
        assert_eq!(claims.get_claim("iss").unwrap(), CL_ISS);
        assert_eq!(claims.get_claim("jti").unwrap(), CL_JTI);
        assert_eq!(
            claims.get_claim("nbf").unwrap(),
            &date.read().unwrap().to_string()
        );
        assert_eq!(claims.get_claim("sub").unwrap(), CL_SUB);
        assert_eq!(claims.get_claim("vc").unwrap(), CL_VC);
        assert_eq!(claims.get_claim("vp").unwrap(), CL_VP);
    }
}
