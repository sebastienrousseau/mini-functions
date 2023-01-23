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

/// The `Mini Functions` library is a highly performant utility and
/// wrapper functions library for Rust that has been carefully designed
/// with optimization and efficiency in mind.
pub mod mini_functions {
    pub use claims::Claims;
    pub use date::Date;
    pub use hash::Hash;
    pub use jot::{Algorithm, Header, JWT};
    pub use logger::{Log, LogLevel};
    pub use md5::MD5;
    pub use qr::QRCode;
    pub use random::Random;
}
