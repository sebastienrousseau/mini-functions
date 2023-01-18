//! # Core JWT functionality
//!
//! JWT provides a set of utility functions for working with JSON Web Tokens (JWTs) and JSON Web Signatures (JWSs).
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

extern crate base64;
extern crate claims;
extern crate date;
extern crate errors;
extern crate hmac;
extern crate jwt;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use self::claims::Claims;
use self::errors::JwtError;

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{fmt, string::ToString};

/// JWT is a struct that holds the JWT token and its associated claims.
/// Provides a set of utility functions for working with JSON Web Tokens
/// (JWTs) and JSON Web Signatures (JWSs).
#[derive(Serialize, Deserialize, Debug)]
pub struct JWT {
    /// The header of the JWT.
    pub header: Header,
    /// The claims associated with the JWT.
    pub claims: Claims,
    /// The signature of the JWT.
    pub signature: Vec<u8>,
    /// The JWT token.
    pub token: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Header {
    /// Indicates the algorithm used to sign the JWT. Defaults to HS256.
    /// See the Algorithm enum for a list of supported algorithms.
    pub alg: Option<Algorithm>,
    /// Indicates the key used to sign the JWT. This is used to select
    /// a specific key for a given JWT.
    pub kid: Option<String>,
    /// Indicates the media type of the JWT. Defaults to JWT.
    pub typ: Option<String>,
    /// Indicates the content type.
    pub cty: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Algorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
}

impl ToString for Algorithm {
    fn to_string(&self) -> String {
        match self {
            Algorithm::HS256 => "HS256".to_string(),
            Algorithm::HS384 => "HS384".to_string(),
            Algorithm::HS512 => "HS512".to_string(),
            Algorithm::RS256 => "RS256".to_string(),
            Algorithm::RS384 => "RS384".to_string(),
            Algorithm::RS512 => "RS512".to_string(),
            Algorithm::ES256 => "ES256".to_string(),
            Algorithm::ES384 => "ES384".to_string(),
            Algorithm::ES512 => "ES512".to_string(),
        }
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::HS256
    }
}

impl Default for JWT {
    fn default() -> Self {
        JWT {
            header: Header {
                alg: Some(Algorithm::HS256),
                kid: None,
                typ: None,
                cty: None,
            },
            claims: Claims::default(),
            signature: vec![],
            token: String::new(),
        }
    }
}
impl Default for Header {
    fn default() -> Self {
        Header {
            alg: Some(Algorithm::HS256),
            kid: None,
            typ: Some("JWT".to_string()),
            cty: None,
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
        let jwt = &self.token;
        {
            // Split the JWT into its header, claims, and signature
            let (header_b64, claims_b64_signature_b64) = jwt.split_once('.').unwrap();
            let (claims_b64, inner_signature_b64) =
                claims_b64_signature_b64.split_once('.').unwrap();

            // Base64-decode the header and claims
            let header_json = general_purpose::STANDARD.decode(header_b64)?;
            let claims_json = general_purpose::STANDARD.decode(claims_b64)?;

            // Allocate Vec of references to slices
            let header_json_tmp = header_json.as_slice();
            let claims_json = claims_json.as_slice();

            // Deserialize the header and claims from JSON
            let _decoded_header: Header = serde_json::from_slice(header_json_tmp)?;
            let _decoded_claims: Claims = serde_json::from_slice(claims_json)?;

            // Sign the JWT with the secret
            type HmacSha256 = Hmac<Sha256>;
            let mut hmac = HmacSha256::new_from_slice(secret).unwrap();
            hmac.update(jwt.as_bytes());
            let signature = hmac.finalize();
            let signature_b64 = general_purpose::STANDARD.encode(signature.into_bytes());

            // Compare the signature to the signature in the JWT
            if signature_b64 != inner_signature_b64 {
                return Err(JwtError::SignatureInvalid(signature_b64));
            }
            Ok(jwt.to_string())
        }
    }
    // Generates a JWT token.
    pub fn generate() -> Result<String, String> {
        let claims = Claims::default();
        let header = Header::default();
        let password = "password".to_string();
        self::encode(&header, &claims, password.as_bytes()).map_err(|e| e.to_string())
    }

    // Returns the token field of the JWT struct.
    //
    // # Returns
    //
    // * `String` - The token value.
    //
    pub fn get_token() -> String {
        JWT::default().token
    }

    // Claims
    pub fn claims() -> Claims {
        Claims::default()
    }

    // Get the token length.
    pub fn get_token_length() -> usize {
        JWT::default().token.len()
    }

    // Get the token header.
    pub fn get_token_header() -> Header {
        JWT::default().header
    }
}

/// Header
pub fn header() -> Header {
    Header::default()
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
pub fn encode(header: &Header, claims: &Claims, secret: &[u8]) -> Result<String, JwtError> {
    // Convert the header and claims to JSON
    let header_json = serde_json::to_string(header)?;
    let claims_json = serde_json::to_string(claims)?;

    // Base64-encode the header and claims
    let header_b64 = general_purpose::STANDARD.encode(header_json);
    let claims_b64 = general_purpose::STANDARD.encode(claims_json);

    // Concatenate the encoded header and claims with a period separator
    let jwt = format!("{header_b64}.{claims_b64}");

    // Sign the JWT with the secret
    type HmacSha256 = Hmac<Sha256>;
    let mut hmac = HmacSha256::new_from_slice(secret).unwrap();
    hmac.update(jwt.as_bytes());
    let signature = hmac.finalize();
    let signature_b64 = general_purpose::STANDARD.encode(signature.into_bytes());

    // Concatenate the JWT with the signature, separated by a period
    let jwt = format!("{jwt}.{signature_b64}");

    Ok(jwt)
}

impl fmt::Display for JWT {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JWT {{ header: {}, claims: {}, signature: {:?}, token: {} }}",
            self.header, self.claims, self.signature, self.token
        )
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Header {{ alg: {:?}, kid: {:?}, typ: {:?}, cty: {:?} }}",
            self.alg, self.kid, self.typ, self.cty
        )
    }
}
