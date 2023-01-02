//! # Core UUID functionality
//!
//! UUID provides an easy way to generate a new UUID (Universally Unique Identifier) in version 3, 4, or 5.
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`UUID`] to generate a new UUID (Universally Unique Identifier) in version 3, 4, or 5.
///
/// # Arguments
/// * `namespace` - A string slice that holds a namespace UUID.
/// * `name` - A string slice that holds a name.
///
/// # Examples
/// ```
/// use mini_functions::uuid::UUID;
///
/// let uuid3 = UUID::uuid_v3(namespace, name);
/// ```
///
/// let uuid4 = UUID::uuid_v4();
/// ```
/// let uuid5 = UUID::uuid_v5(namespace, name);
///
///
// use uuid::Uuid;

/// UUID Utility
///
/// By default, a new UUID is generated.
#[non_exhaustive]
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct UUID;

impl UUID {
    /// Generates a new UUID (Universally Unique Identifier) in version 3, 4, or 5.
    /// # Arguments
    /// * `version` - A string slice that holds a version number.
    /// * `namespace` - A string slice that holds a namespace UUID.
    /// * `name` - A string slice that holds a name.
    /// # Returns
    /// A new UUID (Universally Unique Identifier) in version 3, 4, or 5.
    /// # Examples
    /// ```
    /// use mini_functions::uuid::UUID;
    /// let uuid = UUID::new(version, namespace, name);
    /// ```
    #[must_use]
    pub fn new(version: u8, namespace: uuid::Uuid, name: &[u8]) -> String {
        match version {
            3 => uuid::Uuid::new_v3(&namespace, name).to_string(),
            4 => uuid::Uuid::new_v4().to_string(),
            5 => uuid::Uuid::new_v5(&namespace, name).to_string(),
            _ => uuid::Uuid::new_v4().to_string(),
        }
    }
    pub fn uuid(&self) {
        uuid::Uuid::new_v4().to_string();
    }
    /// Create a new v3 UUID
    /// # Arguments
    /// * `namespace` - A string slice that holds a namespace UUID.
    /// * `name` - A string slice that holds a name.
    /// # Returns
    /// A new v3 UUID.
    /// # Examples
    /// ```
    /// use mini_functions::uuid::UUID;
    /// let uuid3 = UUID::uuid_v3(namespace, name);
    /// ```
    pub fn uuid_v3(namespace: uuid::Uuid, name: &[u8]) -> String {
        uuid::Uuid::new_v3(&namespace, name).to_string()
    }
    /// Create a new v4 UUID
    /// # Returns
    /// A new v4 UUID.
    /// # Examples
    /// ```
    /// use mini_functions::uuid::UUID;
    /// let uuid4 = UUID::uuid_v4();
    /// ```
    pub fn uuid_v4() -> String {
        uuid::Uuid::new_v4().to_string()
    }
    /// Create a new v5 UUID
    /// # Arguments
    /// * `namespace` - A string slice that holds a namespace UUID.
    /// * `name` - A string slice that holds a name.
    /// # Returns
    /// A new v5 UUID.
    /// # Examples
    /// ```
    /// use mini_functions::uuid::UUID;
    /// let uuid5 = UUID::uuid_v5(namespace, name);
    /// ```
    pub fn uuid_v5(namespace: uuid::Uuid, name: &[u8]) -> String {
        uuid::Uuid::new_v5(&namespace, name).to_string()
    }
}
