use crate::date::date::Date;
use crate::jwt::error::JwtError;

use hmac::{Hmac, Mac};
use jwt::Header;
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Debug)]
/// A JSON Web Token (JWT) is a compact, URL-safe means of representing
/// claims to be transferred between two parties. The claims in a JWT
/// are encoded as a JSON object that is used as the payload of a JSON
/// Web Signature (JWS) structure or as the plaintext of a JSON Web
/// Encryption (JWE) structure, enabling the claims to be digitally
/// signed or integrity protected with a Message Authentication Code
/// (MAC) and/or encrypted.
#[non_exhaustive]
pub struct Claims {
    exp: String, // expiration date
    iat: String, // issued at date
    iss: String, // issuer
    sub: String, // subject
    aud: String, // audience
}

// Provides a set of utility functions for working with JSON Web Tokens
/// (JWTs) and JSON Web Signatures (JWSs).
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct JWT {
    audience: String,      // audience
    claim: String,         // claim
    create_date: String,   // create date
    expire_date: String,   // expire date
    issuer: String,        // issuer
    password: String,      // password
    token: Option<String>, // token value
    update_date: String,   // update date
    username: String,      // username
}

impl Default for JWT {
    fn default() -> Self {
        JWT {
            audience: String::from("audience"), // audience
            claim: String::from("claim"),       // claim
            create_date: Date::date(),          // create date
            expire_date: Date::date(),          // expire date
            issuer: String::from("issuer"),     // issuer
            password: String::from("password"), // password
            token: None,                        // token value
            update_date: Date::date() + "1",    // 1 day from now
            username: String::from("username"), // username
        }
    }
}

impl JWT {
    /// Decodes a JWT token. takes a mutable reference to a JWT struct
    /// and a reference to a slice of bytes representing a secret, and
    /// it returns a Result containing a string or an Error variant.
    /// The function splits the JWT stored in the token field of the
    /// JWT struct into its header, claims, and signature, decodes the
    /// header and claims from base64, deserializes the header and
    /// claims from JSON, and then verifies the JWT's signature using
    /// the provided secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - A byte array containing the secret used to sign
    /// the JWT.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The decoded JWT as a string.
    /// * `Err(Error)` - An error if the JWT is invalid or if there was
    /// a problem decoding it.
    ///
    pub fn decode(&mut self, secret: &[u8]) -> Result<String, JwtError> {
        // Split the JWT into its header, claims, and signature
        let jwt = self.token.clone().unwrap_or(String::new());
        let (header_b64, claims_b64_signature_b64) = jwt.split_once('.').unwrap();
        let (claims_b64, inner_signature_b64) = claims_b64_signature_b64.split_once('.').unwrap();

        // Base64-decode the header and claims
        let header_json = base64::decode(header_b64);
        let claims_json = base64::decode(claims_b64);

        // Convert the header and claims from JSON
        let _decoded_header: Header = serde_json::from_slice(&header_json.unwrap())?;
        let _decoded_claims: Claims = serde_json::from_slice(&claims_json.unwrap())?;

        // Sign the JWT with the secret
        type HmacSha256 = Hmac<Sha256>;
        let mut hmac = HmacSha256::new_from_slice(secret).unwrap();
        hmac.update(jwt.as_bytes());
        let signature = hmac.finalize();
        let signature_b64 = base64::encode(signature.into_bytes());

        // Compare the signature to the signature in the JWT
        if signature_b64 != inner_signature_b64 {
            return Err(JwtError::SignatureInvalid);
        }

        Ok(jwt)
    }

    /// Encodes a JWT token. It takes references to a Header and Claims
    ///  struct and a reference to a slice of bytes representing a
    /// secret, and it returns a Result containing a string or an JwtError
    /// variant. The function serializes the Header and Claims structs
    /// to JSON, encodes the JSON strings to base64, concatenates the
    /// encoded header and claims with a period separator, signs the
    /// resulting string with the provided secret using the HMAC-SHA256
    /// algorithm, and then concatenates the signed string with the
    /// signature, separated by a period.
    fn encode(header: &Header, claims: &Claims, secret: &[u8]) -> Result<String, JwtError> {
        // Convert the header and claims to JSON
        let header_json = serde_json::to_string(header)?;
        let claims_json = serde_json::to_string(claims)?;

        // Base64-encode the header and claims
        let header_b64 = base64::encode(&header_json);
        let claims_b64 = base64::encode(&claims_json);

        // Concatenate the encoded header and claims with a period separator
        let jwt = format!("{}.{}", header_b64, claims_b64);

        // Sign the JWT with the secret
        type HmacSha256 = Hmac<Sha256>;
        let mut hmac = HmacSha256::new_from_slice(secret).unwrap();
        hmac.update(jwt.as_bytes());
        let signature = hmac.finalize();
        let signature_b64 = base64::encode(signature.into_bytes());

        // Concatenate the JWT with the signature, separated by a period
        let jwt = format!("{}.{}", jwt, signature_b64);

        Ok(jwt)
    }

    /// Generates a JWT token.
    pub fn generate(&self) -> Result<String, String> {
        let timestamp = Date::timestamp();
        let claims = Claims {
            exp: timestamp.clone(),
            iat: timestamp.clone(),
            iss: self.issuer.clone(),
            sub: self.username.clone(),
            aud: self.audience.clone(),
        };

        let header = Header::default();

        Self::encode(&header, &claims, &self.password.as_bytes()).map_err(|e| e.to_string())
    }

    /// Get the token value.
    pub fn get_token(&self) -> Option<String> {
        self.token.clone()
    }

    /// Claims
    pub fn claims(&self) -> Claims {
        let timestamp = Date::timestamp();
        Claims {
            exp: timestamp.clone(),
            iat: timestamp.clone(),
            iss: self.issuer.clone(),
            sub: self.username.clone(),
            aud: self.audience.clone(),
        }
    }

    /// Get the token length.
    pub fn get_token_length(&self) -> Option<usize> {
        self.token.clone().map(|t| t.len())
    }

    /// Get the token username.
    pub fn get_token_username(&self) -> &str {
        &self.username
    }

    /// Header
    pub fn header(&self) -> Header {
        Header::default()
    }

    /// Creates a new JWT token with the specified username, password,
    /// issuer, and audience.
    pub fn new(username: String, password: String, issuer: String, audience: String) -> Self {
        let create_date = Date::date();
        let update_date = Date::date();
        let expire_date = Date::date();

        JWT {
            token: None,
            username,
            password,
            create_date,
            update_date,
            expire_date,
            issuer,
            audience,
            claim: String::new(),
        }
    }

    /// Secret
    pub fn secret(&self) -> &[u8] {
        self.password.as_bytes()
    }

    /// Sets the token value.
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}

// impl Default for JWT {
//     fn default() -> Self {
//         JWT::new(String::new(), String::new(), String::new(), String::new())
//     }
// }

impl std::fmt::Display for JWT {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "JWT {{ token: {}, username: {}, password: {}, create_date: {}, update_date: {}, expire_date: {}, issuer: {}, audience: {}, claim: {} }}",
            self.token.clone().unwrap_or(String::new()),
            self.username,
            self.password,
            self.create_date,
            self.update_date,
            self.expire_date,
            self.issuer,
            self.audience,
            self.claim
        )
    }
}
