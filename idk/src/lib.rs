// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for managing errors and exceptions
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-idk.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/idk.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/idk)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.1-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/idk)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/tree/main/idk)
//! [![License](https://img.shields.io/crates/l/idk.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! IDontKnow (IDK) is a Rust library that has functions and variables
//! designed to make it easy for your program to report informative
//! error messages. You can use the IDK library to create error messages
//! that are clear, concise, and actionable.
//!
//! ## Features
//!
//! The library includes multiple modules: `Common`, `Error`, `Jwt`,
//! `Property`, `Stacktrace`, and `Traits`.
//!
//! - **The common module:** This module provides a foundation of common
//! functionalities that can be utilized throughout the entire project.
//! These functionalities serve as a building block for the rest of the
//! project, making it easier for developers to create and manage their
//! code.
//! - **The error module:** This module contains all the error types
//! that are used in the project. By providing a centralized location
//! for errors, developers can quickly and easily identify and resolve
//! any issues that may arise.
//! - **The jwt module:** This module offers the tools necessary to
//! encode and decode JSON Web Tokens (JWT). With a simple, easy-to-use
//! interface, developers can ensure secure communication between
//! parties.
//! - **The property module:** This module provides the functionality to
//! create and manage properties. By utilizing this module, developers
//! can keep track of all properties within the project and make changes
//! as necessary.
//! - **The stacktrace module:** This module offers the tools to create
//! and manage stacktraces. By providing detailed information about the
//! execution of the code, developers can quickly identify and resolve
//! any issues that may arise.
//! - **The traits module:** This module provides functionality to
//! create and manage traits. By utilizing this module, developers can
//! ensure that all traits within the project are consistent and
//! well-defined.
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-idk.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-idk.svg",
    html_root_url = "https://docs.rs/idk"
)]
#![crate_name = "idk"]
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
