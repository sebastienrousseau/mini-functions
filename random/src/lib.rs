// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! # A Rust library for generating random and pseudo-random numbers based on the Mersenne Twister algorithm
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-random.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/claims)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! This crate provides a random number generator based on the Mersenne
//! Twister algorithm. The Mersenne Twister algorithm is a pseudorandom
//! number generator (PRNG) developed by Makoto Matsumoto and Takuji
//! Nishimura in 1997. It is based on a linear feedback shift register
//! (LFSR) and is designed to have a period of 2^19937-1. It is one of
//! the most widely used PRNGs in scientific computing.
//!
//! ## Features
//!
//! - Generates random numbers based on the Mersenne Twister algorithm
//! (MT19937)
//! - Generates random numbers in the range 0..(2^32 - 1)
//!
//!
#![warn(missing_debug_implementations)]
#![warn(missing_copy_implementations)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-random.svg",
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-random.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_type = "lib"]

extern crate rand;
use rand::{thread_rng, Rng};

/// N is the number of elements in the array used for the Mersenne
/// Twister algorithm.Its value is set to 624 for optimal performance.
const N: usize = 624;

/// M is the number of elements to skip in the array used for the
/// Mersenne Twister algorithm. Its value is set to 397 for optimal
/// performance.
const M: usize = 397;

/// MATRIX_A is a constant value used in the Mersenne Twister algorithm.
const MATRIX_A: u32 = 0x9908b0df;

/// UPPER_MASK is a constant value used in the Mersenne Twister
/// algorithm.
const UPPER_MASK: u32 = 0x80000000;

/// LOWER_MASK is a constant value used in the Mersenne Twister
/// algorithm.
const LOWER_MASK: u32 = 0x7fffffff;

/// TEMPERING_MASK_B is a constant value used in the Mersenne Twister
/// algorithm.
const TEMPERING_MASK_B: u32 = 0x9d2c5680;

/// TEMPERING_MASK_C is a constant value used in the Mersenne Twister
/// algorithm.
const TEMPERING_MASK_C: u32 = 0xefc60000;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
/// The Random struct is used to generate random numbers using the
/// Mersenne Twister algorithm. It generates pseudorandom integers
/// uniformly distributed in 0..(2^32 - 1) starting from any odd seed in
/// 0..(2^32 - 1).
///
/// It contains an array of unsigned 32-bit integers and an index used
/// to generate random numbers. The array contains 624 elements and the
/// index is used to generate random numbers from the array.
///
/// The index is incremented after each random number is generated.
/// When the index reaches 624, the array is reinitialized and the index
/// is reset to 0.
///
pub struct Random {
    /// The array of unsigned 32-bit integers used to generate random
    /// numbers
    pub mt: [u32; N],
    /// The current index of the array used in the generation of random
    /// numbers
    pub mti: usize,
}

impl Random {
    /// Returns a random bool with a probability that can be set
    pub fn bool(&mut self, probability: f64) -> bool {
        thread_rng().gen_bool(probability)
    }

    /// Returns a vector of random bytes of the given length
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            let byte = self.rand() as u8;
            res.push(byte);
        }
        res
    }

    /// Returns a random char within the range 'a'..='z'
    pub fn char(&mut self) -> char {
        thread_rng().gen_range('a'..='z')
    }

    /// Returns a random element from a slice of values
    pub fn choose<'a, T>(&'a mut self, values: &'a [T]) -> Option<&T> {
        if values.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..values.len());
        Some(&values[index])
    }

    /// Returns a random float.
    pub fn float(&mut self) -> f32 {
        thread_rng().gen::<f64>() as f32
    }

    /// Returns a random integer within the given range
    pub fn int(&mut self, min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..=max)
    }
    // pub fn int(&mut self, min: i32, max: i32) -> i32 {
    //     self.range(min, max)
    // }

    /// Returns new random number generator
    pub fn new() -> Self {
        let mut rng = Random {
            mt: [0; N],
            mti: N + 1,
        };
        let seed = thread_rng().gen();
        rng.mt[0] = seed;
        for i in 1..N {
            rng.mt[i] = 1812433253u32
                .wrapping_mul(rng.mt[i - 1] ^ (rng.mt[i - 1] >> 30))
                .wrapping_add(i as u32);
        }
        rng.mti = N;
        rng
    }

    /// Returns pseudo random number
    pub fn pseudo(&mut self) -> u32 {
        let mut res = self.rand();
        for _ in 0..31 {
            res ^= self.rand();
        }
        res
    }

    /// Returns a random 32-bit unsigned integer
    pub fn rand(&mut self) -> u32 {
        if self.mti >= N {
            if self.mti == N + 1 {
                self.seed(5489);
            }
            self.twist();
        }

        let mut y = self.mt[self.mti];
        self.mti += 1;
        y ^= y >> 11;
        y ^= (y << 7) & TEMPERING_MASK_B;
        y ^= (y << 15) & TEMPERING_MASK_C;
        y ^= y >> 18;
        y
    }

    /// Returns a random 32-bit unsigned integer within a given range
    pub fn random_range(&mut self, min: u32, max: u32) -> u32 {
        min + self.rand() % (max - min)
    }

    /// Returns a random number within a given range
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        thread_rng().gen_range(min..=max)
    }

    /// Seeds the random number generator with a given value
    pub fn seed(&mut self, seed: u32) {
        self.mt[0] = seed;
        for i in 1..N {
            self.mt[i] = match 1812433253u32.checked_mul(self.mt[i - 1] ^ (self.mt[i - 1] >> 30)) {
                Some(val) => val + i as u32,
                None => return,
            };
        }
        self.mti = N;
    }

    /// Twists the state of the random number generator
    pub fn twist(&mut self) {
        for i in 0..N {
            let x = (self.mt[i] & UPPER_MASK) + (self.mt[(i + 1) % N] & LOWER_MASK);
            let x_a = x >> 1;
            if x % 2 != 0 {
                self.mt[i] = self.mt[(i + M) % N] ^ x_a ^ MATRIX_A;
            } else {
                self.mt[i] = self.mt[(i + M) % N] ^ x_a;
            }
        }
        self.mti = 0;
    }
}

impl std::fmt::Display for Random {
    /// Returns a formatted string
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Random {{ mt: {:?}, mti: {:?} }}", self.mt, self.mti)
    }
}

impl Default for Random {
    /// Returns a default random number generator
    fn default() -> Self {
        Self::new()
    }
}
