extern crate claims;
use self::claims::Claims;

fn main() {
    // Create a new instance of Claims
    let mut claims = Claims::new();

    // Set claims
    claims.set_claim("aud", "https://example.com");
    claims.set_claim("custom", "admin");
    claims.set_claim("did", "did:example:123456789");
    claims.set_claim("iss", "https://issuer.com");
    claims.set_claim("jti", "abc123");
    claims.set_claim("sub", "user123");
    claims.set_claim("vc", "Ed25519Signature2018");
    claims.set_claim("vp", "B7AC971B05D791F0EB5FCE3B8A3296F1D68A63199714A2993AAD6E2F3D10F4E4425576AA4D97B80B617D5A182B519E9A021DEEDE9BFFBC3499F902DDC5CA163F");

    // Get claims
    let audience = claims.get_claim("aud").unwrap();
    let custom = claims.get_claim("custom").unwrap();
    let did = claims.get_claim("did").unwrap();
    let issuer = claims.get_claim("iss").unwrap();
    let jwt_id = claims.get_claim("jti").unwrap();
    let subject = claims.get_claim("sub").unwrap();
    let credential = claims.get_claim("vc").unwrap();
    let proof = claims.get_claim("vp").unwrap();

    println!("🦀 Claims::get_claim() for 'aud':       ✅ {}", audience);
    println!("🦀 Claims::get_claim() for 'custom':    ✅ {}", custom);
    println!("🦀 Claims::get_claim() for 'did':       ✅ {}", did);
    println!("🦀 Claims::get_claim() for 'iss':       ✅ {}", issuer);
    println!("🦀 Claims::get_claim() for 'jti':       ✅ {}", jwt_id);
    println!("🦀 Claims::get_claim() for 'sub':       ✅ {}", subject);
    println!("🦀 Claims::get_claim() for 'vc':        ✅ {}", credential);
    println!("🦀 Claims::get_claim() for 'vp':        ✅ {}", proof);

    // Remove claims
    let audience = claims.get_claim("aud").unwrap().to_owned();
    claims.remove_claim("aud");
    println!("🦀 Claims::remove_claim():              ✅ {}", audience);
}
