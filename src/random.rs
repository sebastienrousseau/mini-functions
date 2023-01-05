//! # Core random number generator
//!
//! This crate provides a random number generator based on the linear congruential generator algorithm with the golden ratio as the multiplier.
//!
//!

// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::time::SystemTime;

/// A random number generator based on the linear congruential generator
/// algorithm with the golden ratio as the multiplier.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Random {
    // The seed for the random number generator
    seed: u32,
}

impl Random {
    /// Creates a new `Random` struct with a seed based on the current
    /// system time.
    pub fn new() -> Self {
        // Get the current system time in milliseconds
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;
        Self { seed }
    }

    /// Generates a random number using the linear congruential
    /// generator algorithm. The multiplier for the algorithm is the
    /// golden ratio.
    pub fn random(&mut self) -> u32 {
        // The multiplier for the linear congruential generator
        // algorithm
        let golden_ratio = 1140071478;
        // Update the seed with the next value in the sequence
        self.seed = self.seed.wrapping_mul(golden_ratio).wrapping_add(12345);
        // Return the upper 15 bits of the seed as the random number
        (self.seed >> 16) & 0x7FFF
    }

    /// Generates a pseudo-random number by XORing the last 31 random
    /// numbers together.
    pub fn pseudo(&mut self) -> u32 {
        let mut res = self.random();
        let mut rng = Random::new();
        // XOR the last 31 random numbers together to generate the
        // pseudo-random number
        for _ in 0..31 {
            res ^= rng.random();
        }
        res
    }

    /// Generates a vector of random bytes of a given length.
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut res = Vec::with_capacity(len);
        let mut rng = Random::new();
        for _ in 0..len {
            res.push(rng.random() as u8);
        }
        res
    }
    /// Generates a random floating point number between 0 and 1.
    pub fn float(&mut self) -> f32 {
        let mut rng = Random::new();
        rng.random() as f32 / 0x7FFF as f32
    }
    /// Generates a random integer between a minimum and maximum value.
    pub fn int(&mut self, min: i32, max: i32) -> i32 {
        let mut rng = Random::new();
        (rng.random() as f32 / 0x7FFF as f32 * (max - min) as f32) as i32 + min
    }
}

impl std::fmt::Display for Random {
    /// Formats the `Random` struct as a string for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Random {{ seed: {} }}", self.seed)
    }
}
