// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library that implements the MD5 cryptographic hash function
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-mdg.svg)](https://minifunctions.com)
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
//! The Message Digest (MDG) is an easy way to produces a 128-bit
//! (16-byte) hash value using the MD5 cryptographic hash function. It
//! provides a struct, MD5, that can generate a message digest of data
//! in a secure, one-way hash. The message digest can verify the
//! integrity of the data without having to store the entire message.
//!
//! Several options are available to produce the hash value:
//!
//! - `MD5::default()` - Returns the hash value of an empty string.
//! - `MD5::digest()` - Returns the hash value of a string.
//! - `MD5::finalize()` - Finalize the MD5 object and return the result
//!    as a 16-byte array.
//! - `MD5::hexdigest()` - Returns the hash value of a string as a
//!    hexadecimal string.
//! - `MD5::new()` - Create a new instance of the MD5 struct.
//! - `MD5::reset()` - Reset the internal state of the MD5 object.
//! - `MD5::to_hex_string()` - Returns the hash value of a string as a
//!    hexadecimal string.
//! - `MD5::to_string()` - Returns the hash value of a string as a
//!    string.
//! - `MD5::update()` - Update the internal state of the MD5 object
//!    with new data.
//! - `MD5::update_file()` - Update the internal state of the MD5
//!    object with new data from a file.
//!
//! To use this crate, add `mdg` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! mdg = "0.0.1"
//! ```
//!
//! ## Usage
//!
//! - [`serde`][]: Enable serialization/deserialization via serde
//!
//! # Examples
//!
//! ```no_run
//! use mdg::MD5;
//!
//! let hash = MD5::default();
//! assert_eq!(hash.to_string(), "d41d8cd98f00b204e9800998ecf8427e");
//! ```
//! # Warning
//!
//! This crate is not intended for cryptographic use. MD5 is not a
//! cryptographically secure hashing algorithm and should not be used
//! for applications that require a collision-resistant hash function.
//!
//! MD5 is sensitive to length extension attacks, which alter the hash
//! value if additional data is appended to the input.
//!
//! [`serde`]: https://github.com/serde-rs/serde
//!
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-mdg.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-mdg.svg",
    html_root_url = "https://docs.rs/mdg"
)]
#![crate_name = "mdg"]
#![crate_type = "lib"]

/// Import the `params` module.
mod params;
pub use params::*;

/// Import the `constants` module.
pub mod constants;
pub use constants::*;

/// Import the `digest` module.
pub mod digest;
pub use digest::*;

use std::convert::TryInto;
use std::fmt::Display;

/// The MD5 struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MD5 {
    /// The buffer.
    pub buffer: [u8; BLOCK_LENGTH],
    /// The count.
    pub count: [u32; 2],
    /// The digest.
    pub digest: [u8; DIGEST_LENGTH],
    /// The state.
    pub state: [u32; 4],
}

impl MD5 {
    /// Finalize the MD5 object and return the result as a 16-byte array.
    pub fn finalize(&mut self) -> &Self {
        // Save the length before padding.
        let bits: [u8; 8] = (0..8)
            .into_iter()
            .map(|i| (self.count[i >> 2] >> ((i & 3) << 3)) as u8)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Couldn't transform vec into array");

        // Pad out to 56 mod 64
        let index = (self.count[0] >> 3) & 63;
        let pad_len = if index < 56 { 56 - index } else { 120 - index };
        self.update_with_len(&PADDING, pad_len as usize);

        // Append the length
        self.update(&bits);

        self.digest = (0..DIGEST_LENGTH)
            .into_iter()
            .map(|i| (self.state[i >> 2] >> ((i & 3) << 3)) as u8)
            .collect::<Vec<_>>()
            .try_into()
            .expect("Couldn't transform vec into array");

        self
    }
    /// Create a new instance of the MD5 struct.
    pub fn new() -> Self {
        Self {
            state: INITIAL_STATE,
            count: [0, 0],
            buffer: [0; BLOCK_LENGTH],
            digest: [0; DIGEST_LENGTH],
        }
    }
    /// Update the internal state of the MD5 object with new data.
    pub fn transform(&mut self, data: &[u8]) -> &mut Self {
        const fn f(x: u32, y: u32, z: u32) -> u32 {
            (x & y) | (!x & z)
        }

        const fn g(x: u32, y: u32, z: u32) -> u32 {
            (x & z) | (y & !z)
        }

        const fn h(x: u32, y: u32, z: u32) -> u32 {
            x ^ y ^ z
        }

        const fn i(x: u32, y: u32, z: u32) -> u32 {
            y ^ (x | !z)
        }
        let (mut a, mut b, mut c, mut d): (u32, u32, u32, u32) =
            (self.state[0], self.state[1], self.state[2], self.state[3]);

        for (idx, t_value) in T_VALUES.iter().enumerate() {
            let (value, g): (u32, usize) = match idx {
                0..=15 => (f(b, c, d), idx),
                16..=31 => (g(b, c, d), (5 * idx + 1) % DIGEST_LENGTH),
                32..=47 => (h(b, c, d), (3 * idx + 5) % DIGEST_LENGTH),
                48..=63 => (i(b, c, d), (7 * idx) % DIGEST_LENGTH),
                _ => unreachable!(),
            };
            let part_value = u32::from_ne_bytes(
                data[4 * g..4 * g + 4]
                    .try_into()
                    .expect("Couldn't transform slice into array"),
            );
            let f = value
                .wrapping_add(a)
                .wrapping_add(*t_value)
                .wrapping_add(part_value);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(SHIFTS[idx].into()));
        }
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);

        self
    }
    /// Update the internal state of the MD5 object with new data.
    pub fn update_with_len(&mut self, value: &[u8], nbytes: usize) -> &mut Self {
        // Compute number of bytes mod 64
        let mut offset = ((self.count[0] >> 3) & 63) as usize;
        let nbits = (nbytes << 3) as u32;
        let p = value;

        if nbytes == 0 {
            return self;
        }

        // Update the number of bits
        self.count[0] = self.count[0].wrapping_add(nbits);
        if self.count[0] < nbits {
            self.count[1] += 1;
        }

        self.count[1] += (nbytes >> 29) as u32;

        let part_len = BLOCK_LENGTH - offset;
        let mut i = part_len;

        // Transform as many times as possible
        if nbytes >= part_len {
            self.buffer[offset..(offset + part_len)].clone_from_slice(&p[..part_len]);
            let buf = self.buffer;
            self.transform(&buf);

            while i < nbytes - part_len {
                if nbytes - i >= 64 {
                    let buf = self.buffer[i..i + part_len].to_vec();
                    self.transform(&buf);
                    i += 64;
                } else {
                    break;
                }
            }
            offset = 0;
        } else {
            i = 0;
        }

        // Add remaining input in buffer
        self.buffer[offset..(offset + nbytes - i)].clone_from_slice(&p[i..nbytes]);
        self
    }
}

impl Default for MD5 {
    /// Create a new instance of the MD5 struct.
    fn default() -> Self {
        Self::new()
    }
}

impl Display for MD5 {
    /// Display the current MD5 value.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for di in self.digest {
            write!(f, "{di:02x}")?;
        }

        Ok(())
    }
}
