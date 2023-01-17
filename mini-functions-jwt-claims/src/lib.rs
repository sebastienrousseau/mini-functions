//! # Core Claims functionality
//!
//! Claims provides a set of utility functions for working with JSON Web Token (JWT) claims.
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

extern crate date;
extern crate serde;

use serde::{Deserialize, Serialize};

/// # Claims
/// Claims is a struct that holds the claims defined in the RFC 7519.
///
/// The following claims are registered in the IANA "JSON Web Token
/// Claims" registry established by Section 10.1.  None of the claims
/// defined below are intended to be mandatory to use or implement in
/// all cases, but rather they provide a starting point for a set of
/// useful, interoperable claims.  Applications using JWTs should
/// define which specific claims they use and when they are required
/// or optional.
///
/// # Arguments
/// * `aud` - A string slice that holds the audience.
/// * `custom` - A string slice that holds the custom.
/// * `did` - A string slice that holds the did.
/// * `exp` - A string slice that holds the expiration time.
/// * `iat` - A string slice that holds the time at which the JWT was issued.
/// * `iss` - A string slice that holds the issuer.
/// * `jti` - A string slice that holds the time at which the JWT was last updated.
/// * `nbf` - A string slice that holds the not-before time.
/// * `sub` - A string slice that holds the subject.
/// * `vc` - A string slice that holds the vc.
/// * `vp` - A string slice that holds the vp.
///
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    /// The audience of the JWT.
    pub aud: Option<String>,
    /// The custom of the JWT.
    pub custom: Option<String>,
    /// The did of the JWT.
    pub did: Option<String>,
    /// The expiration time of the JWT.
    pub exp: Option<String>,
    /// The time at which the JWT was issued.
    pub iat: Option<String>,
    /// The issuer of the JWT.
    pub iss: Option<String>,
    /// The time at which the JWT was last updated.
    pub jti: Option<String>,
    /// The not-before time of the JWT.
    pub nbf: Option<String>,
    /// The subject of the JWT.
    pub sub: Option<String>,
    /// The vc of the JWT.
    pub vc: Option<String>,
    /// The vp of the JWT.
    pub vp: Option<String>,
}

impl Claims {}

/// Implement the `Display` trait for `Claims`.
impl std::fmt::Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Claims {{ aud: {:?}, exp: {:?}, iat: {:?}, iss: {:?}, jti: {:?}, nbf: {:?}, sub: {:?}, did: {:?}, vc: {:?}, vp: {:?}, custom: {:?} }}",
            self.aud.as_ref().unwrap(),
            self.custom.as_ref().unwrap(),
            self.did.as_ref().unwrap(),
            self.exp.as_ref().unwrap(),
            self.iat.as_ref().unwrap(),
            self.iss.as_ref().unwrap(),
            self.jti.as_ref().unwrap(),
            self.nbf.as_ref().unwrap(),
            self.sub.as_ref().unwrap(),
            self.vc.as_ref().unwrap(),
            self.vp.as_ref().unwrap(),
        )
    }
}

/// Implement the `Default` trait for `Claims`.
impl Default for Claims {
    fn default() -> Self {
        Claims {
            aud: Some("".to_string()),
            custom: Some("".to_string()),
            did: Some("".to_string()),
            exp: Some("".to_string()),
            iat: Some("".to_string()),
            iss: Some("".to_string()),
            jti: Some("".to_string()),
            nbf: Some("".to_string()),
            sub: Some("".to_string()),
            vc: Some("".to_string()),
            vp: Some("".to_string()),
        }
    }
}
