// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for error handling and reporting in Mini Functions.
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-errors.svg)](https://minifunctions.com)
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
//! The `errors` crate provides a set of error types and functions to
//! handle errors in Mini Functions.
//!
//! ## Features
//!
//!
//! - [x] common - The common module provides common functionality that can be used throughout the entire crate.
//! - [x] error - The error module contains all the error types used in the crate.
//! - [x] jwt - The jwt module provides functionality to encode and decode JSON Web Tokens (JWT).
//! - [x] property - The property module provides functionality to create and manage properties.
//! - [x] stacktrace - The stacktrace module provides functionality to create and manage stacktraces.
//! - [x] traits - The traits module provides functionality to create and manage traits.
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
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-errors.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-errors.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "errors"]
#![crate_type = "lib"]

/// The common module provides common functionality that can be used
/// throughout the entire crate.
pub mod common;

/// The error module contains all the error types used in the crate.
pub mod error;

/// The jwt module provides functionality to encode and decode
/// JSON Web Tokens (JWT).
pub mod jwt;

/// The property module provides functionality to create and manage
/// properties.
pub mod property;

/// The stacktrace module provides functionality to create and manage
/// stacktraces.
pub mod stacktrace;

/// The traits module provides functionality to create and manage
/// traits.
pub mod traits;
