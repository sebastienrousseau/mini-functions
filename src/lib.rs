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
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Mini-Functions is a modern Rust library that prioritizes performance
//! , security, and modularity. It provides a low-overhead access to
//! functions for common programming tasks.
//!
//! ## Features
//!
//! - **[Claims](Claims/index.html)** - Provides access to the claims of a JSON Web Token (JWT).
//! - **[Common](Common/index.html)** - Provides access to functions for accessing a collection of mathematical and cryptographic constants.
//! - **[Date](Date/index.html)** - Provides access to functions for parsing, validating, manipulating, and formatting dates and times.
//! - **[Errors](Errors/index.html)** - Provides access to error handling functions.
//! - **[Hash](Hash/index.html)** - Provides access to a Quantum-Resistant Cryptographic Hash Library for Password Hashing and Verification.
//! - **[Logs](Logs/index.html)** - Provides access to functions for application-level logging with a simple, readable output format.
//! - **[JWT](JWT/index.html)** - Provides access to JSON Web Token (JWT) functions.
//! - **[MD5](MD5/index.html)** - Provides access to MD5 functions.
//! - **[QR](QR/index.html)** - Provides access to QR code functions.
//! - **[Random](Random/index.html)** - Provides access to functions for generating high-quality random numbers based on the Mersenne Twister algorithm.
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
//! mini_functions = "0.0.8"
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

/// Provides access to functions for accessing a collection of mathematical and cryptographic constants.
pub mod common;

/// Provides access to functions for parsing, validating, manipulating, and formatting dates and times.
pub mod date;

/// Provides access to error handling functions.
pub mod errors;

/// Provides access to hash functions.
pub mod hash;

/// Provides access to functions for application-level logging with a simple, readable output format.
pub mod logs;

/// Provides access to JSON Web Token (JWT) functions.
pub mod jwt;

/// Provides access to MD5 functions.
pub mod md5;

/// Provides access to functions for generating high-quality random numbers based on the Mersenne Twister algorithm.
pub mod random;

/// Provides access to QR code functions.
pub mod qr;

/// Re-exports public contents of key modules
pub mod mini_functions {
    pub use crate::{
        claims::*,
        common::{self,cmn_macros},
        date::{self,dtt_macros},
        errors::*,
        hash::{self,hsh_macros},
        jwt::*,
        logs::{self,rlg_macros},
        md5::{self,mdg_constants},
        qr::*,
        random::{self,vrd_macros},
    };
}