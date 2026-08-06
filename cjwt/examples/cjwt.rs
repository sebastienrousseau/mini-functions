extern crate cjwt;
extern crate jwt;

use self::cjwt::{Algorithm, Header, JWT};
use cclm::Claims;

fn main() {
    // Constants for the JWT struct examples.
    const HD_ALG: Algorithm = Algorithm::HS384;
    const HD_KID: &str = "jwt-kid";
    const HD_TYP: &str = "jwt-typ";
    const HD_CTY: &str = "jwt-cty";

    // Create a Header struct with default method.
    let hd: Header = Header::default();
    println!("🦀 Header::default():             ✅ {hd:?}\n");

    // Create a Header struct with every field populated.
    let hdrv: Header = Header {
        alg: Some(HD_ALG),
        kid: Some(HD_KID.to_string()),
        typ: Some(HD_TYP.to_string()),
        cty: Some(HD_CTY.to_string()),
    };
    println!("🦀 Header::default():             ✅ {hdrv:?}\n");

    // Create a JWT struct with default method.
    let jd: JWT = JWT::default();
    println!("🦀 JWT::default():                ✅ {jd:?}\n");

    // Create a JWT struct carrying that populated header.
    let jdrv: JWT = JWT {
        header: hdrv.clone(),
        ..Default::default()
    };
    println!("🦀 JWT::default():                ✅ {jdrv:?}\n");

    // Encode a JWT struct.
    let encoded: String = JWT::encode(hdrv, Claims::default(), b"secret").unwrap();
    println!("🦀 encode():                      ✅ {encoded:?}\n");

    // Extract the token field from the passed JWT struct and return it.
    let jwt = JWT {
        header: Header::default(),
        claims: Claims::default(),
        signature: vec![],
        token: "example_token".to_owned(),
    };
    let result = JWT::get_token(jwt);
    println!("🦀 get_token():                       ✅ {result:?}\n");
}
