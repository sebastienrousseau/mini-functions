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
use errors::jwt::JwtError;

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{fmt, string::ToString};

/// JWT is a struct that holds the JWT token and its associated claims.
/// Provides a set of utility functions for working with JSON Web Tokens
/// (JWTs) and JSON Web Signatures (JWSs).
#[derive(Clone, Debug, Serialize, Deserialize)]
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
            token: String::default(),
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
    // Claims
    pub fn claims() -> Claims {
        Claims::default()
    }
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
            // let (header_b64, claims_b64_signature_b64) = jwt.split_once('.').unwrap();
            let (header_b64, claims_b64_signature_b64) = match jwt.split_once('.') {
                Some(tuple) => tuple,
                None => return Err(JwtError::DecodeError("Invalid JWT".to_string())),
            };

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

    /// Encodes a JWT token using the provided header, claims, and
    /// secret. It returns a Result containing a string or an Error
    /// variant. The function serializes the header and claims to JSON,
    /// base64-encodes the header and claims, concatenates the encoded
    /// header, claims, and separators, and then signs the JWT with the
    /// provided secret.
    pub fn encode(header: Header, claims: Claims, secret: &[u8]) -> Result<String, JwtError> {
        // Serialize the header and claims to JSON
        let header_json = serde_json::to_string(&header)?;
        let claims_json = serde_json::to_string(&claims)?;

        // Base64-encode the header and claims
        let header_b64 = general_purpose::STANDARD.encode(header_json.as_bytes());
        let claims_b64 = general_purpose::STANDARD.encode(claims_json.as_bytes());

        // Concatenate the encoded header, claims, and separators
        let jwt = format!("{header_b64}.{claims_b64}.");

        // Sign the JWT with the secret
        type HmacSha256 = Hmac<Sha256>;
        let mut hmac = HmacSha256::new_from_slice(secret)?;
        hmac.update(jwt.as_bytes());
        let signature = hmac.finalize();

        // Base64-encode the signature and concatenate it with the JWT
        let signature_b64 = general_purpose::STANDARD.encode(signature.into_bytes());
        let jwt = format!("{jwt}.{signature_b64}");

        Ok(jwt)
    }

    // Generates a JWT token.
    pub fn generate(secret: &[u8]) -> Result<String, JwtError> {
        let claims = Claims::default();
        let header = Header::default();
        JWT::encode(header, claims, secret)
    }

    // Returns the token field of the JWT struct.
    pub fn get_token(jwt: JWT) -> String {
        jwt.token
    }

    // Returns the header field of the JWT struct.
    pub fn get_token_header(jwt: JWT) -> Header {
        jwt.header
    }

    // Get the token length.
    pub fn get_token_length(jwt: JWT) -> usize {
        jwt.token.len()
    }

    // Validates a JWT token.
    pub fn validate(&self, secret: &[u8]) -> Result<(), JwtError> {
        let jwt = &self.token;
        {
            // Split the JWT into its header, claims, and signature
            // let (header_b64, claims_b64_signature_b64) = jwt.split_once('.').unwrap();
            let (header_b64, claims_b64_signature_b64) = match jwt.split_once('.') {
                Some(tuple) => tuple,
                None => return Err(JwtError::DecodeError("Invalid JWT".to_string())),
            };

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
            let mut hmac = HmacSha256::new_from_slice(secret)?;
            hmac.update(jwt.as_bytes());
            let signature = hmac.finalize();

            // Verify the signature
            let mut hmac = HmacSha256::new_from_slice(secret)?;
            hmac.update(inner_signature_b64.as_bytes());
            let inner_signature = hmac.finalize();
            if signature != inner_signature {
                return Err(JwtError::InvalidSignature("Invalid signature".to_string()));
            }
        }
        Ok(())
    }
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
