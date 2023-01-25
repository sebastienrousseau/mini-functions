// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! Highly performant utility and wrapper functions library for Rust
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/logos/logo-mini-functions.svg)](https://minifunctions.com)
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
//! Mini-Functions is a Rust utility library for performance, security,
//! and modularity. At a high level, it provides a few major components:
//!
//! - **[claims](../claims/index.html)** - Provides access to the claims of a JSON Web Token (JWT).
//! - **[common](../common/index.html)** - Provides access to common functions and constants.
//! - **[date](../date/index.html)** - Provides access to the current date and time.
//! - **[errors](../errors/index.html)** - Provides access to error handling functions.
//! - **[games](../games/index.html)** - Provides access to mini games using Mini-Functions
//! - **[hash](../hash/index.html)** - Provides access to hash functions.
//! - **[jot](../jot/index.html)** - Provides access to JSON Web Token (JWT) functions.
//! - **[logger](../logger/index.html)** - Provides access to logging functions.
//! - **[md5](../md5/index.html)** - Provides access to MD5 functions.
//! - **[password](../password/index.html)** - Provides access to password functions.
//! - **[qr](../qr/index.html)** - Provides access to QR code functions.
//! - **[random](../random/index.html)** - Provides access to random number functions.
//! - And so much more.
//!
//! These components provide a comprehensive set of functionality and
//! offer powerful new capabilities to help you build better
//! applications and services in the Rust programming language.
//!
//! [**Learn more**](https://minifunctions.com) [❯](https://minifunctions.com)
//!
//! ## Installation
//!
//! Mini Functions is available on both
//! [Crates.io](https://crates.io/crates/mini_functions) and
//! [Lib.rs](https://lib.rs/crates/mini_functions).
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
//!
#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/icons/ico-mini-functions.svg",
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/icons/ico-mini-functions.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "mini_functions"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

/// Mini-Functions is a module that provides a variety of features
/// for building apps, including several major crates that you can use
/// individually or together to construct the core infrastructure of
/// your Rust applications.
pub mod mini_functions {
    /// This module allows access to JSON Web Token (JWT) Claims
    /// functions.
    pub use claims::Claims;
    /// Provides access to Date functions.
    pub use date::Date;
    /// Provides access to Hash functions.
    pub use hash::Hash;
    /// Provides access to JSON Web Token (JWT) functions.
    pub use jot::{Algorithm, Header, JWT};
    /// Provides access to Log functions.
    pub use logger::{Log, LogLevel};
    /// Provides access to MD5 functions.
    pub use md5::MD5;
    /// Provides access to QR code functions.
    pub use qr::QRCode;
    /// Provides access to Random numbers functions.
    pub use random::Random;
}
