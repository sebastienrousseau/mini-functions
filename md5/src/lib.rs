//! # Core `MD5` functionality
//!
//! The crate `md5` provides an easy way to produces a 128-bit (16-byte)
//! hash value using the MD5 algorithm. MD5 (Message Digest Method 5) is
//! a cryptographic hash algorithm used to generate a 128-bit digest
//! from a string of any length. It represents the digests as 32 digit
//! hexadecimal numbers.
//!
//! Several options are available to produce the hash value:
//!
//! - `MD5::default()` - Returns the hash value of an empty string.
//! - `MD5::digest()` - Returns the hash value of a string.
//! - `MD5::finalize()` - Finalize the MD5 object and return the result as a 16-byte array.
//! - `MD5::hexdigest()` - Returns the hash value of a string as a hexadecimal string.
//! - `MD5::new()` - Create a new instance of the MD5 struct.
//! - `MD5::reset()` - Reset the internal state of the MD5 object.
//! - `MD5::to_hex_string()` - Returns the hash value of a string as a hexadecimal string.
//! - `MD5::to_string()` - Returns the hash value of a string as a string.
//! - `MD5::update()` - Update the internal state of the MD5 object with new data.
//! - `MD5::update_file()` - Update the internal state of the MD5 object with new data from a file.
//!
//! To use this crate, add `md5` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! md5 = "0.0.1"
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use md5::MD5;
//!
//! let hash = MD5::default();
//! assert_eq!(hash.to_string(), "d41d8cd98f00b204e9800998ecf8427e");
//! ```
//! # Warning
//!
//! This crate is not intended for cryptographic use. MD5 is not a cryptographically secure hashing algorithm and should not be used for applications that require a collision-resistant hash function.
//! MD5 is sensitive to length extension attacks, which alter the hash value if additional data is appended to the input.
//!
//! # Join the Mini Functions Community
//!
//! Mini Functions is an ongoing project that is constantly evolving and improving. Our goal is to make it a highly-performant utility and wrapper functions library for Rust. We’ve already added a number of functions and features to the library, and we have more planned for the future but we need your help.
//!
//! As an open source project, we welcome contributions from the community. If you’re an experienced Rust developer and you’re interested in helping us shape the direction of the project, we encourage you to get involved. Whether you have ideas for new features, want to help fix bugs, or just want to learn more about Rust development, we welcome your contributions.

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

mod params;
pub use params::*;

mod constants;
use constants::*;

use std::convert::TryInto;
use std::fmt::Display;

// use crate::Digest;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MD5 {
    pub buffer: [u8; BLOCK_LENGTH],
    pub count: [u32; 2],
    pub digest: [u8; DIGEST_LENGTH],
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
        self.update_with_len(&PADDING, Some(pad_len as usize));

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
    pub fn update_with_len(&mut self, value: &[u8], nbytes: Option<usize>) -> &mut Self {
        // Compute number of bytes mod 64
        let mut offset = ((self.count[0] >> 3) & 63) as usize;
        let nbytes = nbytes.unwrap_or(value.len());
        let nbits = (nbytes << 3) as u32;
        let p = value;

        if nbytes == 0 {
            return self;
        }

        // Update the number of bits
        self.count[0] += nbits;
        if self.count[0] < nbits {
            self.count[1] += 1;
        }
        self.count[1] += (nbytes >> 29) as u32;

        let part_len = BLOCK_LENGTH - offset;
        let mut i = part_len;

        // Transform as many times as possible
        if nbytes >= part_len {
            self.buffer[offset..(offset + part_len)].copy_from_slice(&p[..part_len]);
            self.transform(&self.buffer.clone());

            while i < part_len.saturating_sub(63) {
                let buf = self.buffer;
                self.transform(&buf[i..]);
                i += 64;
            }
            offset = 0;
        } else {
            i = 0;
        }
        // Add remaining input in buffer
        self.buffer[offset..(offset + nbytes - i)].copy_from_slice(&p[i..nbytes]);

        self
    }
}

impl Default for MD5 {
    fn default() -> Self {
        Self::new()
    }
}

impl Digest for MD5 {
    /// Reset the internal state of the MD5 object.
    fn reset(&mut self) -> &mut Self {
        self.state = INITIAL_STATE;
        self.count.fill(0);
        self.buffer.fill(0);
        self.digest.fill(0);

        self
    }
    /// Update the internal state of the MD5 object with new data.
    fn update(&mut self, value: &[u8]) -> &mut Self {
        self.update_with_len(value, None)
    }
    /// Update the internal state of the MD5 object with new data from a file.
    fn update_file(&mut self, path: &str) -> &mut Self {
        let mut file = File::open(path).expect("Couldn't open file");
        let mut buffer = [0; 1024];

        loop {
            let nbytes = file.read(&mut buffer).expect("Couldn't read file");
            if nbytes == 0 {
                break;
            }
            self.update_with_len(&buffer, Some(nbytes));
        }

        self
    }
    /// Return the digest value as a string of hexadecimal digits.
    fn hexdigest(value: &str) -> String {
        Self::new().update(value.as_bytes()).finalize().to_string()
    }

    /// Return the digest value as a string of hexadecimal digits from a file.
    fn hexdigest_file(path: &str) -> String {
        let mut file = File::open(path).expect("Couldn't open file");
        let mut buffer = [0; 1024];
        let mut md5 = Self::new();

        loop {
            let nbytes = file.read(&mut buffer).expect("Couldn't read file");
            if nbytes == 0 {
                break;
            }
            md5.update_with_len(&buffer, Some(nbytes));
        }

        md5.finalize().to_string()
    }
    /// Reset the internal state of the MD5 object and update it with new data from a file.
    fn reset_file(&mut self, path: &str) -> &mut Self {
        self.reset();
        self.update_file(path)
    }
}

impl Display for MD5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for di in self.digest {
            write!(f, "{:02x}", di)?;
        }

        Ok(())
    }
}

pub trait Digest {
    fn reset(&mut self) -> &mut Self; // reset the internal state of the object
    fn update(&mut self, value: &[u8]) -> &mut Self; // update the internal state of the object with new data
    fn update_file(&mut self, path: &str) -> &mut Self; // update the internal state of the object with new data from a file
    fn hexdigest(value: &str) -> String; // return the digest value as a string of hexadecimal digits
    fn hexdigest_file(path: &str) -> String; // return the digest value as a string of hexadecimal digits from a file
    fn reset_file(&mut self, path: &str) -> &mut Self; // reset the internal state of the object and update it with new data from a file
}
