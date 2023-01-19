//! # Core Cryptography Hash functionality based on the [blake3](https://crates.io/crates/blake3) crate.
//!
//! This crate provides an easy way to hash and verify passwords using the [blake3](https://crates.io/crates/blake3) crate.
//!
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

extern crate blake3;
use blake3::Hasher;

/// A struct for storing and verifying hashed passwords based on
/// the [blake3](https://crates.io/crates/blake3) crate.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Hash {
    /// The password.
    password: String,
    /// The password hash.
    hash: String,
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
