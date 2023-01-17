#[cfg(test)]

mod tests {

    extern crate date;
    extern crate mini_functions_jwt_claims;

    // use self::date::Date;
    use self::mini_functions_jwt_claims::Claims;

    #[test]
    fn test_claims() {
        let claims = {
            let aud: &str = "1516239022";
            let custom: &str = "John Doe";
            let did: &str = "mini_functions_claims";
            let exp: &str = "1516239022";
            let iat: &str = "John Doe";
            let iss: &str = "mini_functions_claims";
            let jti: &str = "John Doe";
            let nbf: &str = "John Doe";
            let sub: &str = "John Doe";
            let vc: &str = "John Doe";
            let vp: &str = "John Doe";

            Claims {
                aud: Some(aud.to_string()),
                custom: Some(custom.to_string()),
                did: Some(did.to_string()),
                exp: Some(exp.to_string()),
                iat: Some(iat.to_string()),
                iss: Some(iss.to_string()),
                jti: Some(jti.to_string()),
                nbf: Some(nbf.to_string()),
                sub: Some(sub.to_string()),
                vc: Some(vc.to_string()),
                vp: Some(vp.to_string()),
            }
        };
        println!("🦀 Claims::new():                 ✅ {:?}\n", claims);
    }
}
