//! # Core UUID functionality
//!
//! UUID provides a simple way to generate a new UUID.
//!
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Implements [`UUID`] to generate a new UUID.
///
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
    #[must_use]
    pub fn uuid_v3(namespace: uuid::Uuid, name: &[u8]) -> String {
        uuid::Uuid::new_v3(&namespace, name).to_string()
    }
    #[must_use]
    pub fn uuid_v4() -> String {
        uuid::Uuid::new_v4().to_string()
    }
    #[must_use]
    pub fn uuid_v5(namespace: uuid::Uuid, name: &[u8]) -> String {
        uuid::Uuid::new_v5(&namespace, name).to_string()
    }
}
