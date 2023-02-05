// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Cryptographic Hash Algorithms Library for Rust
//!
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-hsh.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/hsh.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/hsh)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.1-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/hsh)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/hsh)
//! [![License](https://img.shields.io/crates/l/hsh.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Hash (HSH), is a Cryptographic Hash Algorithms Library for secure
//! password management.
//!
//! Utilizing the cutting-edge [Blake3][11] algorithm, this library
//! generates and verifies hashed passwords with ease. The library
//! features a struct for storing and verifying hashed passwords, as
//! well as a series of methods for calculating entropy, generating
//! hashes, accessing password and hash values, and more.
//!
//! `HSH` also implements the Default and Display traits for flexible
//! use and easy readability.
//!
//! ## Features
//!
//! - [x] Hashes passwords using the [blake3](https://crates.io/crates/blake3) crate.
//! - [x] Verifies passwords against stored hashes.
//! - [x] Calculates the entropy of the hash in bits based on the Shannon entropy formula.
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
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
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-hsh.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-hsh.svg",
    html_root_url = "https://docs.rs/hsh"
)]
#![crate_name = "hsh"]
#![crate_type = "lib"]

extern crate blake3;
use blake3::Hasher;

/// A struct for storing and verifying hashed passwords based on
/// the [blake3](https://crates.io/crates/blake3) crate.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hash {
    /// The password.
    pub password: String,
    /// The password hash.
    pub hash: String,
}

impl Hash {
    /// Calculates the entropy of the hash in bits based on the Shannon
    /// entropy formula. <https://en.wikipedia.org/wiki/Entropy>`
    pub fn entropy(&self) -> f64 {
        // Shannon entropy formula in bits.
        let mut entropy = 0.0;
        for c in self.hash.chars() {
            let p = (c as u8) as f64 / 255.0;
            entropy -= p * p.log2();
        }
        entropy
    }

    /// Hashes the password.
    pub fn generate_hash(&self) -> String {
        let mut hasher = Hasher::new();
        hasher.update(self.password.as_bytes());
        let hash = hasher.finalize().to_hex();
        hash.to_string()
    }

    /// Returns the hash.
    pub fn hash(&self) -> &str {
        &self.hash
    }

    /// Returns the hash length.
    pub fn hash_length(&self) -> usize {
        self.hash.len()
    }

    /// Returns a new instance of `Hash`.
    pub fn new() -> Self {
        Self {
            password: String::default(),
            hash: String::default(),
        }
    }

    /// Returns the password.
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Returns the password length.
    pub fn password_length(&self) -> usize {
        self.password.len()
    }

    /// Sets the hash.
    pub fn set_hash(&mut self, hash: &str) {
        self.hash = hash.to_string();
    }

    /// Sets the password and updates the hash.
    pub fn set_password(&mut self, password: &str) {
        self.password = password.to_string();
        self.hash = self.generate_hash();
    }
    /// Verifies the password against the stored hash.
    ///
    /// Returns `true` if the password and hash match, `false` otherwise.
    pub fn verify(&self, hash: &str, password: &str) -> bool {
        let mut hasher = Hasher::new();
        hasher.update(password.as_bytes());
        let password_hash = hasher.finalize().to_hex();
        let password_hash_str = password_hash.to_string();
        password_hash_str == hash
    }
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Hash {{ password: {}, hash: {} }}",
            self.password, self.hash
        )
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self::new()
    }
}
