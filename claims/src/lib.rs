// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for accessing and manipulating claims of a JSON Web Token (JWT)
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-claims.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/claims)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! The Claims library holds JSON Web Token (JWT) claims. It provides an
//! RFC7519 compliant implementation of JSON Web Tokens (JWT) and JSON
//! Web Signature (JWS) for Rust.
//!
//! The [**`Claims`**](./struct.Claims.html) type is provided to hold
//! the claims of a JWT. The claims are stored in a `HashMap` and can be
//! accessed using the `get_claim`, `set_claim`, `remove_claim`, and
//! `has_claim` methods.
//!
//! ## Features
//!
//! The following table lists the optional reserved claims that are
//! supported:
//!
//! | Claim | Description |
//! | --- | --- |
//! | `aud` (Audience) | Identifies the recipients that the JWT is intended for. |
//! | `custom` (Custom) | Custom claims are used to share information between parties that agree on using them and are neither registered or public claims. |
//! | `did` (Decentralized Identifier) | A string value that uniquely identifies a subject. |
//! | `exp` (Expiration Time) | Identifies the expiration time on or after which the JWT MUST NOT be accepted for processing. |
//! | `iat` (Issued At) | Identifies the time at which the JWT was issued. |
//! | `iss` (Issuer) | Identifies the principal that issued the JWT. |
//! | `jti` (JWT ID) | Provides a unique identifier for the JWT. |
//! | `nbf` (Not Before) | Identifies the time before which the JWT MUST NOT be accepted for processing. |
//! | `sub` (Subject) | Identifies the principal that is the subject of the JWT. |
//! | `vc` (Verifiable Credential) | A Credential that is tamper-evident and has authorship that can be cryptographically verified. |
//! | `vp` (Verifiable Presentation) | A Presentation that is tamper-evident and has authorship that can be cryptographically verified. |
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde,
//! usually derived with `serde_derive`.
//!
//! ## Examples
//!
//! ```rust
//! use self::claims::Claims;
//! use std::collections::HashMap;
//!
//! // Create a new instance of Claims
//! let mut claims = Claims::new();
//!
//! // Set a claim
//! claims.set_claim("name", "John Doe");
//!
//! // Get a claim
//! let name = claims.get_claim("name").unwrap(); // returns "John Doe"
//!
//! // Remove a claim
//! claims.remove_claim("name");
//!
//! // Clear all claims
//! claims.clear_claims();
//!
//! // Has a claim
//! let has_claim = claims.has_claim("name"); // returns false
//!
//! // Get the number of claims
//! let len = claims.len(); // returns 0
//!
//! // Is the claims empty?
//! let is_empty = claims.is_empty(); // returns true
//!
//! // Get the claims as a HashMap
//! let claims_map: &HashMap<String, String> = claims.get_claims();
//!
//! ```
//!
//! ## Links
//! * [RFC 7519](https://tools.ietf.org/html/rfc7519)
//! * [JSON Web Token (JWT)](https://jwt.io/)
//!
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-claims.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-claims.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "claims"]
#![crate_type = "lib"]

extern crate serde;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
/// The Claims struct holds the claims of a JSON Web Token (JWT).
///
/// A JWT is a compact, URL-safe means of representing claims to be
/// transferred between two parties. It consists of a header, a payload,
/// and a signature. The payload is where the claims are stored.
///
/// The claims in a JWT are encoded as a JSON object and can be used to
/// convey information such as the identity of an end user, the
/// expiration time of the token, and more.
///
/// The Claims struct provides a convenient way to manipulate the claims
/// of a JWT in Rust. It stores the claims as a HashMap<String, String>,
/// allowing for fast and efficient access to each claim.
pub struct Claims {
    /// The claims of the JWT as a HashMap.
    pub claims: HashMap<String, String>,
}

impl Claims {
    /// Creates a new instance of the `Claims` struct with an empty
    /// HashMap of claims.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let claims = Claims::new();
    /// assert!(claims.claims.is_empty());
    /// ```
    pub fn new() -> Claims {
        Claims {
            claims: HashMap::new(),
        }
    }
    /// Adds or updates a claim in the `Claims` struct with the given key and value.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// claims.set_claim("sub", "1234567890");
    /// assert_eq!(claims.get_claim("sub"), Some(&String::from("1234567890")));
    /// ```
    pub fn set_claim(&mut self, key: &str, value: &str) {
        self.claims.insert(key.to_string(), value.to_string());
    }
    /// Gets a claim from the `Claims` struct with the given key.
    ///
    /// Returns `None` if the key does not exist in the `Claims`.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// claims.set_claim("sub", "1234567890");
    /// assert_eq!(claims.get_claim("sub"), Some(&String::from("1234567890")));
    /// ```
    pub fn get_claim(&self, key: &str) -> Option<&String> {
        self.claims.get(key)
    }
    //// Removes a claim from the `Claims` struct with the given key.
    ///
    /// Returns the value of the claim that was removed, if any.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// claims.set_claim("sub", "1234567890");
    /// assert_eq!(claims.remove_claim("sub"), Some("1234567890".to_owned()));
    /// ```
    pub fn remove_claim(&mut self, key: &str) -> Option<String> {
        self.claims.remove(key)
    }
    /// Clears all claims from the `Claims` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// claims.set_claim("sub", "1234567890");
    /// claims.clear_claims();
    /// assert!(claims.claims.is_empty());
    /// ```
    pub fn clear_claims(&mut self) {
        self.claims.clear();
    }
    /// Checks if a claim with the given key exists in the `Claims` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// claims.set_claim("sub", "1234567890");
    /// assert!(claims.has_claim("sub"));
    /// ```
    pub fn has_claim(&self, key: &str) -> bool {
        self.claims.contains_key(key)
    }
    /// Get the number of claims in the `Claims` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// claims.set_claim("sub", "1234567890");
    /// assert_eq!(claims.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.claims.len()
    }
    /// Checks if the `Claims` struct is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// assert!(claims.is_empty());
    /// claims.set_claim("sub", "1234567890");
    /// assert!(!claims.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.claims.is_empty()
    }
    /// Get a reference to the HashMap of claims in the `Claims` struct.
    /// This is useful if you need to iterate over the claims.
    ///
    /// # Example
    ///
    /// ```
    /// use self::claims::Claims;
    /// let mut claims = Claims::new();
    /// claims.set_claim("sub", "1234567890");
    /// claims.set_claim("name", "John Doe");
    /// for (key, value) in claims.get_claims() {
    ///    println!("{}: {}", key, value);
    /// }
    /// ```
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
            .map(|(k, v)| format!("{k}: {v}"))
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
