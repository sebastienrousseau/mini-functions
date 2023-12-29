// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # `Mini Functions` 🦀
//!
//! [![Mini Functions Logo][01]][00]
//!
//! ## A Highly Performant Utility and Wrapper Functions Library for Rust
//!
//! Elevate your Rust development with this comprehensive library of functions and utilities designed to streamline common tasks, enhance performance, and promote maintainability across various aspects of Rust application development.
//!
//! <center>
//!
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions)
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.10-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Mini-Functions is a modern Rust library that prioritizes performance, security, and modularity. It provides a low-overhead access to functions for common programming tasks.
//!
//! ## Features
//!
//! - **[Claims](#Modules/Claims/index.html)** - Provides robust functionalities for handling various types of claims in JSON Web Tokens (JWT), including standard, custom, and private claims. Ideal for authentication and authorization processes in Rust applications.
//! - **[Common](#Modules/Common/index.html)** - Offers a comprehensive collection of mathematical and cryptographic constants, such as prime numbers, Pi, cryptographic keys, and more. Essential for applications requiring high-level mathematical computations and secure cryptographic operations.
//! - **[Date](#Modules/Date/index.html)** - Features an extensive suite of functions for parsing, validating, manipulating, and formatting dates and times. Supports a wide range of date/time formats and is tailored for time-sensitive Rust applications.
//! - **[Errors](#Modules/Errors/index.html)** - Delivers advanced error handling functions with support for custom error types, integration with logging systems, and streamlined error propagation. Enhances the reliability and maintainability of Rust applications through robust error management.
//! - **[Hash](#Modules/Hash/index.html)** - Specializes in Quantum-Resistant Cryptographic Hashing, offering a library tailored for password hashing and verification. Includes modern algorithms designed to withstand quantum-computing threats, ensuring long-term security.
//! - **[JWT](#Modules/JWT/index.html)** - Provides a full range of JSON Web Token (JWT) functionalities, including secure token generation, decoding, and validation. Facilitates secure and efficient user authentication processes in Rust-based systems.
//! - **[Logs](#Modules/Logs/index.html)** - Enables application-level logging with a focus on simplicity and readability. Features customizable log formats, multiple log levels, and easy integration with Rust applications, making debugging and monitoring more efficient.
//! - **[MD5](#Modules/MD5/index.html)** - Offers MD5 hash functions, suitable for legacy systems compatibility. Includes a clear advisory on MD5's vulnerabilities and guidance on secure alternatives for modern applications.
//! - **[QR](#Modules/QR/index.html)** - Allows for comprehensive QR code operations, including generation, customization, and scanning capabilities. Supports a variety of formats and use-cases, making it a versatile tool for Rust applications involving QR code integration.
//! - **[Random](Random/index.html)** - Features high-quality random number generation using the Mersenne Twister algorithm. Ideal for applications requiring random data generation, including simulations, gaming, and cryptographic operations.
//!
//! These components provide a comprehensive set of functionality and offer powerful new capabilities to help you build better applications and services in the Rust programming language.
//!
//! [**Learn more**](https://minifunctions.com) [❯](https://minifunctions.com)
//!
//! ## Installation
//!
//! Mini Functions is available on both [Crates.io](https://crates.io/crates/mini_functions) and [Lib.rs](https://lib.rs/crates/mini_functions).
//!
//! Learn more about Mini Functions at <https://minifunctions.com>.
//!
//! Add the following to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! mini_functions = "0.0.10"
//! ```
//! Then, add the following to your crate root:
//! ```rust
//! extern crate mini_functions;
//!
//! use mini_functions::mini_functions::*;
//!
//! ```
//!
//! [00]: https://minifunctions.com "Mini Functions - Highly performant utility and wrapper functions library for Rust"
//! [01]: https://kura.pro/mini-functions/images/v2/banners/banner-mini-functions.svg "Mini Functions - Highly performant utility and wrapper functions library for Rust"
//!
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_favicon_url = "https://kura.pro/mini-functions/images/v2/favicon.ico",
    html_logo_url = "https://kura.pro/mini-functions/images/v2/logos/mini-functions.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "mini_functions"]
#![crate_type = "dylib"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "staticlib"]

/// Provides access to the claims of a JSON Web Token (JWT).
pub mod claims;

/// Offers a comprehensive collection of mathematical and cryptographic constants.
pub mod common;

/// Features an extensive suite of functions for handling dates and times.
pub mod date;

/// Delivers advanced error handling functionalities.
pub mod errors;

/// Specializes in Quantum-Resistant Cryptographic Hashing.
pub mod hash;

/// Enables application-level logging with customizable features.
pub mod logs;

/// Provides a full range of JSON Web Token (JWT) functionalities.
pub mod jwt;

/// Offers MD5 hash functions with advisories on usage.
pub mod md5;

/// Features high-quality random number generation using the Mersenne Twister algorithm.
pub mod random;

/// Allows for comprehensive QR code operations.
pub mod qr;

/// Re-exports public contents of key modules
pub mod mini_functions {
    pub use crate::{
        claims::*,
        common::{self, cmn_macros},
        date::{self, dtt_macros},
        errors::*,
        hash::{self, hsh_macros},
        jwt::*,
        logs::{self, rlg_macros},
        md5::{self, mdg_constants},
        qr::*,
        random::{self, vrd_macros},
    };
}
