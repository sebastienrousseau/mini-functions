#[cfg(test)]

mod tests {

    extern crate mini_functions_date;
    extern crate mini_functions_jwt_claims;

    // use self::mini_functions_date::Date;
    use self::mini_functions_jwt_claims::Claims;

    #[test]
    fn test_claims() {
        let claims = {
            let iss: &str = &"mini_functions_claims".to_string();
            let sub: &str = &"John Doe".to_string();
            let aud: &str = &"1516239022";
            let exp: &str = &"1516239022";
            let nbf: &str = &"John Doe".to_string();
            let iat: &str = &"John Doe".to_string();
            let jti: &str = &"John Doe".to_string();
            Claims {
                iss: Some(iss.to_string()),
                sub: Some(sub.to_string()),
                aud: Some(aud.to_string()),
                exp: Some(exp.to_string()),
                nbf: Some(nbf.to_string()),
                iat: Some(iat.to_string()),
                jti: Some(jti.to_string()),
            }
        };
        println!("🦀 Claims::new():                 ✅ {:?}\n", claims);
    }
}
