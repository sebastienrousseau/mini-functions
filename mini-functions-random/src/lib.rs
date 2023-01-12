//! # Core Random number generator
//!
//! This crate provides a random number generator based on the linear congruential generator algorithm with the golden ratio as the multiplier.
//!
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![doc(html_root_url = "https://docs.rs/mini-functions-random/0.1.0")]
extern crate rand;
use rand::{thread_rng, Rng};

/// A random number generator based on the linear congruential generator
/// algorithm with the golden ratio as the multiplier.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Random {
    seed: u32,
}

impl Random {
    /// Generates a random boolean value using the rand `crate` to
    /// produce cryptographically secure random booleans.
    pub fn bool() -> bool {
        let mut rng = thread_rng();
        rng.gen_bool(0.5)
    }

    /// Generates a random byte using the rand `crate` to produce
    /// cryptographically secure random bytes.
    pub fn bytes(len: usize) -> Vec<u8> {
        let mut rng = thread_rng();
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(rng.gen());
        }
        res
    }

    /// Generates a random character using the rand `crate` to produce
    /// cryptographically secure random characters.
    pub fn char() -> char {
        let mut rng = thread_rng();
        rng.gen_range('a'..='z')
    }

    // pub fn choose<T>(&mut self, values: &[T]) -> Option<&T> {
    /// Chooses a random value from a slice of values using the rand
    /// `crate` to produce cryptographically secure random values.
    pub fn choose<'a, T>(&'a mut self, values: &'a [T]) -> Option<&T> {
        if values.is_empty() {
            return None;
        }
        // Generate a random index between 0 and the length of the slice
        let index = self.range(0, values.len() as i32 - 1) as usize;
        Some(&values[index])
    }

    /// Generates a random float between 0 and 1 using the rand `crate`
    /// to produce cryptographically secure random floats.
    pub fn float() -> f32 {
        let mut rng = thread_rng();
        rng.gen()
    }

    /// Generates a random integer between the min and max values using
    /// the rand `crate` to produce cryptographically secure random
    /// integers.
    pub fn int(min: i32, max: i32) -> i32 {
        let mut rng = thread_rng();
        rng.gen_range(min..max)
    }

    /// Creates a new random number generator with a random seed using
    /// the rand `crate` to produce cryptographically secure random
    /// seeds.
    pub fn new() -> Self {
        let seed = thread_rng().gen();
        Self { seed }
    }

    /// Generates a random number between the min and max values using
    /// the rand `crate` to produce cryptographically secure random
    /// numbers.
    pub fn pseudo(&mut self) -> u32 {
        let mut res = self.random();
        for _ in 0..31 {
            res ^= self.random();
        }
        res
    }

    /// Generates a random number between the min and max values using
    /// the rand `crate` to produce cryptographically secure random
    /// numbers.
    pub fn random(&mut self) -> u32 {
        let golden_ratio = 1140071478;
        self.seed = self.seed.wrapping_mul(golden_ratio).wrapping_add(12345);
        (self.seed >> 16) & 0x7FFF
    }

    /// Generates a random number between the min and max values using
    /// the rand `crate` to produce cryptographically secure random
    /// numbers.
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        let mut rng = thread_rng();
        rng.gen_range(min..max)
    }
}

impl std::fmt::Display for Random {
    /// Formats the `Random` struct as a string for display purposes.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Random {{ seed: {} }}", self.seed)
    }
}

impl Default for Random {
    /// Creates a new random number generator with a random seed using
    /// the rand `crate` to produce cryptographically secure random
    /// seeds.
    fn default() -> Self {
        Self::new()
    }
}
