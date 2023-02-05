// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! A Rust library for accessing a collection of mathematical and cryptographic constants
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-cmn.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/cmn.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/cmn)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.1-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/cmn)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/cmn)
//! [![License](https://img.shields.io/crates/l/cmn.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! Common (CMN), a Rust library designed for developers who are looking
//! for a comprehensive collection of mathematical and cryptographic
//! constants.
//!
//!`CMN` is a modern, fast, and user-friendly library that makes it easy
//! to access a wide range of mathematical and cryptographic constants,
//! including the mathematical constant "Euler", the hash algorithm
//! used, the cost of the hash algorithm, the length of the hash, the
//! mathematical constant "Phi", the mathematical constant "Pi", the
//! Planck constant, a set of special characters, and much more.
//!
//! ## Features
//!
//! The following table lists the Constants available in the Common
//! library.
//!
//! | Constants | Description |
//! | --- | --- |
//! | `EULER` | Euler's constant is a mathematical constant approximately equal to 2.71828. |
//! | `HASH_ALGORITHM` | The hash algorithm used to generate the hash. The default is Blake3. |
//! | `HASH_COST` | The cost of the hash. |
//! | `HASH_LENGTH` | The length of the hash. |
//! | `PHI` | The golden ratio is a number approximately equal to 1.618033988749895. |
//! | `PI` | Pi is the ratio of a circle's circumference to its diameter. |
//! | `PLANCK` | Planck's constant is a physical constant that is approximately equal to 6.62607015 × 10−34 joule seconds. |
//! | `SQRT5` | The square root of 5 is a number approximately equal to 2.23606797749979. |
//! | `SPECIAL_CHARS` | A list of special characters. |
//!
//! The following table lists the dictionaries available in the Common
//! library.
//!
//! | Words | Description |
//! | --- | --- |
//! | `words` | Contains a dictionary of common words. |
//!
//! ## Usage
//!
//! Common can be any `serde::Serialize` or `serde::Deserialize` types
//!
//! ## Examples
//!
//! ```rust
//!
//! // Import the Common libraries
//! use self::common::Constants;
//! use self::common::Words;
//!
//! // Constants
//! let constants = Constants.constants();
//! for constant in constants {
//!     println!("Name: {} Value: {}", constant.name, constant.value);
//! }
//!
//! // Words
//! let words = Words::new();
//! let words_list = words.words_list();
//! assert_eq!(words_list[0], "aboard");
//!
//! ```
//!
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-cmn.svg",
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-cmn.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "cmn"]
#![crate_type = "lib"]

extern crate serde;
pub use serde::{Deserialize, Serialize};

/// The `constants` module contains the `Constants` structure, which
/// provides a collection of constant values that are used throughout
/// the library.
pub mod constants;

/// The `words` module contains the `Words` structure, which provides a
/// collection of words that are used throughout the library.
pub mod words;

pub use constants::Constants;
pub use words::Words;

/// The `Common` structure provides a central location to store data
/// that is commonly used throughout the library. The structure
/// implements the `Serialize` and `Deserialize` traits from the `serde`
/// crate to enable serialization and deserialization of the data.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Common;

impl Common {
    /// Creates a new instance of the `Common` structure.
    pub fn new() -> Self {
        Self
    }
    /// Returns the `Constants` instance.
    pub fn constants(&self) -> Constants {
        Constants
    }
    /// Returns a new instance of the `Words` structure.
    pub fn words(&self) -> Words {
        Words::new()
    }
}

impl Default for Common {
    /// Creates a new instance of the `Common` structure by calling
    /// `Self::new()`.
    fn default() -> Self {
        Self::new()
    }
}
