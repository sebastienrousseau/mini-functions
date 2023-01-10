extern crate jwt;
extern crate mini_functions_jwt;

use self::mini_functions_jwt::{Header, JWT};
use mini_functions_jwt::Algorithm;

fn main() {
    // Constants for the JWT struct examples.
    const HD_ALG: Algorithm = Algorithm::HS384;
    const HD_KID: &str = "mini-functions-jwt-kid";
    const HD_TYP: &str = "mini-functions-jwt-typ";
    const HD_CTY: &str = "mini-functions-jwt-cty";

    // Create a Header struct with default method.
    let hd: Header = Header::default();
    println!("🦀 Header::default():             ✅ {:?}\n", hd);

    // Create a Header struct with default method and replace values.
    let mut hdrv: Header = Header::default();
    Header::default().alg = std::mem::replace(&mut hdrv.alg, Some(HD_ALG));
    Header::default().kid = std::mem::replace(&mut hdrv.kid, Some(HD_KID.to_string()));
    Header::default().typ = std::mem::replace(&mut hdrv.typ, Some(HD_TYP.to_string()));
    Header::default().cty = std::mem::replace(&mut hdrv.cty, Some(HD_CTY.to_string()));
    println!("🦀 Header::default():             ✅ {:?}\n", hdrv);

    // Create a JWT struct with default method.
    let jd: JWT = JWT::default();
    println!("🦀 JWT::default():                ✅ {:?}\n", jd);

    // Create a JWT struct with default method and replace values.
    let mut jdrv: JWT = JWT::default();
    JWT::default().header.alg = std::mem::replace(&mut jdrv.header.alg, Some(HD_ALG));
    JWT::default().header.kid = std::mem::replace(&mut jdrv.header.kid, Some(HD_KID.to_string()));
    JWT::default().header.typ = std::mem::replace(&mut jdrv.header.typ, Some(HD_TYP.to_string()));
    JWT::default().header.cty = std::mem::replace(&mut jdrv.header.cty, Some(HD_CTY.to_string()));
    println!("🦀 JWT::default():                ✅ {:?}\n", jdrv);
}
