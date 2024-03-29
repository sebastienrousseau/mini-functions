// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::jwt::{Algorithm, Header, JWT};
extern crate cjwt;
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

    // Create a Header struct with default method and replace values.
    let mut hdrv: Header = Header::default();
    Header::default().alg =
        std::mem::replace(&mut hdrv.alg, Some(HD_ALG));
    Header::default().kid =
        std::mem::replace(&mut hdrv.kid, Some(HD_KID.to_string()));
    Header::default().typ =
        std::mem::replace(&mut hdrv.typ, Some(HD_TYP.to_string()));
    Header::default().cty =
        std::mem::replace(&mut hdrv.cty, Some(HD_CTY.to_string()));
    println!("🦀 Header::default():             ✅ {hdrv:?}\n");

    // Create a JWT struct with default method.
    let jd: JWT = JWT::default();
    println!("🦀 JWT::default():                ✅ {jd:?}\n");

    // Create a JWT struct with default method and replace values.
    let mut jdrv: JWT = JWT::default();
    JWT::default().header.alg =
        std::mem::replace(&mut jdrv.header.alg, Some(HD_ALG));
    JWT::default().header.kid = std::mem::replace(
        &mut jdrv.header.kid,
        Some(HD_KID.to_string()),
    );
    JWT::default().header.typ = std::mem::replace(
        &mut jdrv.header.typ,
        Some(HD_TYP.to_string()),
    );
    JWT::default().header.cty = std::mem::replace(
        &mut jdrv.header.cty,
        Some(HD_CTY.to_string()),
    );
    println!("🦀 JWT::default():                ✅ {jdrv:?}\n");

    // Encode a JWT struct.
    let encoded: String =
        JWT::encode(hdrv, Claims::default(), b"secret").unwrap();
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
