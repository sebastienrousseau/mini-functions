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
use std::collections::HashMap;

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

#[derive(Clone, Serialize, Deserialize, Debug)]
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
