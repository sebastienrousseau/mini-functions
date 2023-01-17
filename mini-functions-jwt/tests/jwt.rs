#[cfg(test)]

mod tests {

    // extern crate mini_functions_jwt;

    // use self::mini_functions_jwt::JWT;

    extern crate base64;
    extern crate date;
    extern crate hmac;
    extern crate jwt;
    extern crate mini_functions_jwt_claims;
    extern crate mini_functions_jwt_errors;

    extern crate serde;
    extern crate serde_json;
    extern crate sha2;

    // use self::date::Date;
    // use self::mini_functions_jwt_claims::Claims;
    // use self::mini_functions_jwt_errors::JwtError;
    // use base64::{engine::general_purpose, Engine as _};
    // use hmac::{Hmac, Mac};
    // use jwt::Header;
    // use serde::{Deserialize, Serialize};
    // use sha2::Sha256;

    // use super::*;

    // #[test]
    // fn test_decode() {
    //     let mut jwt = JWT::default();

    //     // Create a secret
    //     let secret: &[u8] = b"mysecret";

    //     // Create a JWT token with a header, claims, and signature
    //     let header = Header {
    //         alg: "HS256",
    //         typ: Some("JWT"),
    //     };
    //     let claims = Claims {
    //         sub: "username".to_string(),
    //         exp: Date::date(),
    //     };
    //     let encoded_header =
    //         general_purpose::STANDARD.encode(&serde_json::to_vec(&header).unwrap());
    //     let encoded_claims =
    //         general_purpose::STANDARD.encode(&serde_json::to_vec(&claims).unwrap());
    //     let data = format!("{}.{}", encoded_header, encoded_claims);

    //     // Sign the data with the secret
    //     let mut mac = Hmac::<Sha256>::new(secret).unwrap();
    //     mac.input(data.as_bytes());
    //     let signature = mac.result().code();
    //     let encoded_signature = general_purpose::STANDARD.encode(&signature);

    //     // Create a JWT token
    //     jwt.token = format!("{}.{}", data, encoded_signature);

    //     // Decode the JWT token
    //     let result = jwt.decode(secret);

    //     // Make sure the JWT token was decoded correctly
    //     let expected_result = Ok(format!("{}", serde_json::to_string(&claims).unwrap()));
    //     assert_eq!(result, expected_result);
    // }
    // #[test]
    // fn test_jwt_default() {
    //     let jwt_default = JWT::default();
    //     assert!(jwt_default == JWT::default());
    //     println!("🦀 JWT::default():                ✅ {:?}\n", jwt_default);
    // }
}
