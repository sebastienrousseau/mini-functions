// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! Highly performant Claims library for Rust
//!
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-claims.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/claims)
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Utilities for working with JSON Web Token (JWT) claims. Claims is an
//! RFC 7519-compliant JSON object that represents a set of claims
//! transferred between two parties.
//!
//! The arguments below are not mandatory to use. They provide a
//! starting point for a set of useful, interoperable claims.
//!
//! ## Arguments
//! * `aud` - A string slice that holds the audience.
//! * `custom` - A string slice that holds the custom.
//! * `did` - A string slice that holds the did.
//! * `exp` - A string slice that holds the end time.
//! * `iat` - A string slice that holds the time at which the JWT is
//! issued.
//! * `iss` - A string slice that holds the issuer.
//! * `jti` - A string slice that holds the time at which the JWT was
//! last updated.
//! * `nbf` - A string slice that holds the not-before time.
//! * `sub` - A string slice that holds the subject.
//! * `vc` - A string slice that holds the vc.
//! * `vp` - A string slice that holds the vp.
//!
//! ## Examples
//!
//! ```rust
//! use self::claims::Claims;
//! use date::Date;
//! let date = Date::new().iso_8601;
//!
//! const CL_AUD: &str = "CLAIMS-AUD";
//! const CL_CUSTOM: &str = "CLAIMS-CUSTOM";
//! const CL_DID: &str = "CLAIMS-DID";
//! const CL_ISS: &str = "CLAIMS-ISS";
//! const CL_JTI: &str = "CLAIMS-JTI";
//! const CL_SUB: &str = "CLAIMS-SUB";
//! const CL_VC: &str = "CLAIMS-VC";
//! const CL_VP: &str = "CLAIMS-VP";
//!
//! let mut claims = Claims::new();
//!         claims.set_claim("aud", CL_AUD);
//!         claims.set_claim("custom", CL_CUSTOM);
//!         claims.set_claim("did", CL_DID);
//!         claims.set_claim("exp", &date.read().unwrap().to_string());
//!         claims.set_claim("iat", &date.read().unwrap().to_string());
//!         claims.set_claim("iss", CL_ISS);
//!         claims.set_claim("jti", CL_JTI);
//!         claims.set_claim("nbf", &date.read().unwrap().to_string());
//!         claims.set_claim("sub", CL_SUB);
//!         claims.set_claim("vc", CL_VC);
//!         claims.set_claim("vp", CL_VP);
//!
//! ```
//! ## Links
//! * [RFC 7519](https://tools.ietf.org/html/rfc7519)
//! * [JSON Web Token (JWT)](https://jwt.io/)
//!
//!
#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-claims.svg",
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-claims.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "claims"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate date;
extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
/// A struct that holds the claims of a JWT.
pub struct Claims {
    /// The claims of the JWT as a HashMap.
    pub claims: HashMap<String, String>,
}

impl Claims {
    /// Create a new instance of `Claims`.
    pub fn new() -> Claims {
        Claims {
            claims: HashMap::new(),
        }
    }
    /// Set a claim.
    pub fn set_claim(&mut self, key: &str, value: &str) {
        self.claims.insert(key.to_string(), value.to_string());
    }
    /// Get a claim.
    pub fn get_claim(&self, key: &str) -> Option<&String> {
        self.claims.get(key)
    }
    /// Remove a claim.
    pub fn remove_claim(&mut self, key: &str) {
        self.claims.remove(key);
    }
    /// Clear all claims.
    pub fn clear_claims(&mut self) {
        self.claims.clear();
    }
    /// Has a claim.
    pub fn has_claim(&self, key: &str) -> bool {
        self.claims.contains_key(key)
    }
    /// Get the number of claims.
    pub fn len(&self) -> usize {
        self.claims.len()
    }
    /// Is the claims empty?
    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }
    /// Get the claims as a HashMap.
    pub fn get_claims(&self) -> &HashMap<String, String> {
        &self.claims
    }
}

/// Implement the `Display` trait for `Claims`.
impl std::fmt::Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let claims: Vec<String> = self
            .claims
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();

        write!(f, "Claims {{ {} }}", claims.join(", "))
    }
}

/// Implement the `Default` trait for `Claims`.
impl Default for Claims {
    /// Create a new instance of `Claims`.
    fn default() -> Self {
        Claims {
            claims: HashMap::new(),
        }
    }
}
