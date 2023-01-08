//!
//! <h2>Highly Performant Utility And Wrapper Functions Library For Rust</h2>
//!
//! <p align="center">
//!     <img src="https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/logos/logo-mini-functions.svg" alt="Mini Functions Logo">
//! </p>
//!
//! <p align="left">
//! <strong>
//! Mini Functions is a highly performant utility and wrapper functions
//! library for Rust that has been carefully designed with optimization
//! and efficiency in mind.
//! </strong>
//! </p>
//!
//! <p align="left">
//! By providing convenient wrapper functions, our library aims to
//! provide a high-level interface for common tasks while still
//! leveraging the performance benefits of Rust under the hood.
//! </p>
//!
//! <p align="left">
//! These utility functions serve as an essential toolkit for any Rust
//! developer, and the library's design abstractions allow for easy
//! integration into a variety of projects and applications.
//! </p>
//!
//! ## Installation
//!
//! Mini Functions is available on [crates.io](https://crates.io/crates/mini_functions).
//! Add the following to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! mini_functions = "0.0.8"
//! ```
//! Then, add the following to your crate root:
//! ```rust
//! extern crate mini_functions;
//! ```
//!
//! ## Example
//!
//! ```rust
//! extern crate mini_functions;
//!
//! use mini_functions::date::date::Date;
//!
//! fn main() {
//!   let date = Date::date();
//!   println!("{}", date);
//! }
//! ```
//! Learn more about Mini Functions at <https://minifunctions.com>.
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

#[macro_use]

/// Provides a set of utility functions for working with claims
pub mod claims;

/// Provides a set of common constants used in the application
pub mod common;

/// Provides a set of utility functions for working with dates and times
pub mod date;

/// Provides a set of utility functions for working with hashes. This
/// module is a wrapper around the `blake2` crate.
pub mod hash;

/// Provides a set of utility functions for working with JWTs (JSON Web
/// Tokens) and JWSs (JSON Web Signatures).
pub mod jwt;

/// Provides a log function to log a message to the console with a
/// simple, readable output format
pub mod log;

/// Provides a set of utility functions for generating and working with
/// passwords/passphrases
pub mod password;

/// Provides a set of utility functions for generating QR codes
pub mod qrcode;

/// Provides a set of utility functions for working with random numbers
pub mod random;

/// Provides a set of utility functions for working with UUIDs
pub mod uuid;
