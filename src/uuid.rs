//! # Core UUID functionality
//!
//! UUID provides an easy way to generate a new UUID (Universally Unique Identifier) in version 3, 4, or 5.
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

extern crate uuid;

use std::fmt; // Import the `fmt` module from the standard library.
use uuid::Uuid; // Import the `Uuid` type from the `uuid` crate.

/// Implements [`UUID`] to generate a new UUID (Universally Unique Identifier) in version 3, 4, or 5.
///
/// # Arguments
/// * `version` - The version of the UUID to generate. Must be 3, 4, or 5.
/// * `ns` - The namespace to use for the UUID. Must be a valid UUID.
/// * `name` - The name to use for the UUID.
///
#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct UUID {
    inner: Uuid,
}

impl UUID {
    /// Generates a new UUID (Universally Unique Identifier) in version 3, 4, or 5.
    /// # Arguments
    /// * `version` - The version of the UUID to generate. Must be 3, 4, or 5.
    /// * `ns` - The namespace to use for the UUID. Must be a valid UUID.
    /// * `name` - The name to use for the UUID.
    ///
    pub fn new(version: u8, ns: &Uuid, name: &str) -> Self {
        match version {
            3 => UUID::uuid_v3(ns, name),
            4 => UUID::uuid_v4(),
            5 => UUID::uuid_v5(ns, name),
            _ => panic!("Invalid UUID version"),
        }
    }

    /// Create a new v3 UUID
    /// # Arguments
    /// * `ns` - The namespace to use for the UUID. Must be a valid UUID.
    /// * `name` - The name to use for the UUID.
    ///
    pub fn uuid_v3(ns: &Uuid, name: &str) -> Self {
        let inner = Uuid::new_v3(ns, name.as_bytes());
        UUID { inner }
    }
    /// Create a new v4 UUID
    pub fn uuid_v4() -> Self {
        let inner = Uuid::new_v4();
        UUID { inner }
    }
    /// Create a new v5 UUID
    /// # Arguments
    /// * `ns` - The namespace to use for the UUID. Must be a valid UUID.
    /// * `name` - The name to use for the UUID.
    ///
    pub fn uuid_v5(ns: &Uuid, name: &str) -> Self {
        let inner = Uuid::new_v5(ns, name.as_bytes());
        UUID { inner }
    }
}

/// This implementation of the fmt::Display trait allows instances of UUID to be printed using the {} formatting placeholder.
/// The write! macro is used to write the string representation of self.inner to the provided fmt::Formatter.
/// The fmt::Result type is returned to indicate whether the operation was successful or not.
impl fmt::Display for UUID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
