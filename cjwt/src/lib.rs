// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for working with JSON Web Tokens (JWTs) and JSON Web Signatures (JWSs)
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-cjwt.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Core JWT (CJWT) is a struct that holds the JWT token and its associated claims. It provides a set of utility functions for working with JSON Web Tokens (JWTs) and JSON Web Signatures (JWSs).
//!
//! ## Features
//!
//! - [x] JWT token generation
//! - [x] JWT token validation
//! - [x] JWT token signing
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
//!
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-cjwt.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-cjwt.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "cjwt"]
#![crate_type = "lib"]

extern crate base64;
extern crate cclm;
extern crate dtt;
extern crate hmac;
extern crate idk;
extern crate jwt;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use self::cclm::Claims;
use idk::jwt::JwtError;

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{fmt, string::ToString};

/// JWT is a struct that holds the JWT token and its associated claims.
/// Provides a set of utility functions for working with JSON Web Tokens
/// (JWTs) and JSON Web Signatures (JWSs).
#[derive(Debug, Clone, Serialize, Deserialize)]
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
/// The Header struct contains the header of the JWT.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, PartialOrd)]
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

/// The Algorithm enum contains a list of supported algorithms.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, PartialOrd)]
pub enum Algorithm {
    /// HMAC using SHA-256 hash algorithm.
    HS256,

    /// HMAC using SHA-384 hash algorithm.
    HS384,

    /// HMAC using SHA-512 hash algorithm.
    HS512,

    /// RSASSA-PKCS1-v1_5 using SHA-256 hash algorithm.
    RS256,

    /// RSASSA-PKCS1-v1_5 using SHA-384 hash algorithm.
    RS384,

    /// RSASSA-PKCS1-v1_5 using SHA-512 hash algorithm.
    RS512,

    /// ECDSA using P-256 and SHA-256.
    ES256,

    /// ECDSA using P-384 and SHA-384.
    ES384,

    /// ECDSA using P-521 and SHA-512.
    ES512,
}

impl ToString for Algorithm {
    /// Converts an Algorithm enum to a string.
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
    /// Returns the default algorithm, HS256.
    fn default() -> Self {
        Algorithm::HS256
    }
}

impl Default for JWT {
    /// Returns a default JWT struct.
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
    /// Returns a default Header struct.
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
    /// Claims returns a default Claims struct.
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

    /// Generates a JWT token.
    pub fn generate(secret: &[u8]) -> Result<String, JwtError> {
        let claims = Claims::default();
        let header = Header::default();
        JWT::encode(header, claims, secret)
    }

    /// Returns the token field of the JWT struct.
    pub fn get_token(jwt: JWT) -> String {
        jwt.token
    }

    /// Returns the header field of the JWT struct.
    pub fn get_token_header(jwt: JWT) -> Header {
        jwt.header
    }

    /// Get the token length.
    pub fn get_token_length(jwt: JWT) -> usize {
        jwt.token.len()
    }

    /// Validates a JWT token.
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
    /// Formats the JWT struct for printing.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JWT {{ header: {}, claims: {}, signature: {:?}, token: {} }}",
            self.header, self.claims, self.signature, self.token
        )
    }
}

impl fmt::Display for Header {
    /// Formats the Header struct for printing.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Header {{ alg: {:?}, kid: {:?}, typ: {:?}, cty: {:?} }}",
            self.alg, self.kid, self.typ, self.cty
        )
    }
}
