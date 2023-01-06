//! # Core Random number generator
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
    seed: u32,
}

impl Random {
    /// Creates a new random number generator.
    pub fn new() -> Self {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;
        Self { seed }
    }
    /// Generates a random number between the min and max values.
    pub fn random(&mut self) -> u32 {
        let golden_ratio = 1140071478;
        self.seed = self.seed.wrapping_mul(golden_ratio).wrapping_add(12345);
        (self.seed >> 16) & 0x7FFF
    }
    /// Generates a random number between the min and max values.
    pub fn pseudo(&mut self) -> u32 {
        let mut res = self.random();
        for _ in 0..31 {
            res ^= self.random();
        }
        res
    }
    /// Generates a random byte.
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        let mut res = Vec::with_capacity(len);
        for _ in 0..len {
            res.push(self.random() as u8);
        }
        res
    }
    /// Generates a random float between 0 and 1.
    pub fn float(&mut self) -> f32 {
        self.random() as f32 / 0x7FFF as f32
    }
    /// Generates a random integer between the min and max values.
    pub fn int(&mut self, min: i32, max: i32) -> i32 {
        (self.float() * (max - min) as f32) as i32 + min
    }
    /// Generates a random boolean value.
    pub fn bool(&mut self) -> bool {
        self.random() % 2 == 0
    }
    /// Generates a random character.
    pub fn char(&mut self) -> char {
        let mut res = self.random() % 26;
        if self.random() % 2 == 0 {
            res += 65;
        } else {
            res += 97;
        }
        res as u8 as char
    }
    /// Generates a random number between the min and max values.
    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        // Generate a random float between 0 and 1
        let random_float = self.float();
        // Calculate the range between the min and max values
        let range = (max - min) as f32;
        // Multiply the range by the random float and add the min value
        let random_number = (range * random_float) as i32 + min;
        random_number
    }

    // pub fn choose<T>(&mut self, values: &[T]) -> Option<&T> {
    /// Chooses a random value from a slice of values.
    pub fn choose<'a, T>(&'a mut self, values: &'a [T]) -> Option<&T> {
        if values.is_empty() {
            return None;
        }
        // Generate a random index between 0 and the length of the slice
        let index = self.range(0, values.len() as i32 - 1) as usize;
        Some(&values[index])
    }
}
// pub struct Random {
//     // The seed for the random number generator
//     seed: u32,
// }

// impl Random {
//     /// Creates a new `Random` struct with a seed based on the current
//     /// system time.
//     pub fn new() -> Self {
//         // Get the current system time in milliseconds
//         let seed = SystemTime::now()
//             .duration_since(SystemTime::UNIX_EPOCH)
//             .unwrap()
//             .as_millis() as u32;
//         Self { seed }
//     }

//     /// Generates a random number using the linear congruential
//     /// generator algorithm. The multiplier for the algorithm is the
//     /// golden ratio.
//     pub fn random(&mut self) -> u32 {
//         // The multiplier for the linear congruential generator
//         // algorithm
//         let golden_ratio = 1140071478;
//         // Update the seed with the next value in the sequence
//         self.seed = self.seed.wrapping_mul(golden_ratio).wrapping_add(12345);
//         // Return the upper 15 bits of the seed as the random number
//         (self.seed >> 16) & 0x7FFF
//     }

//     /// Generates a pseudo-random number by XORing the last 31 random
//     /// numbers together.
//     pub fn pseudo(&mut self) -> u32 {
//         let mut res = self.random();
//         let mut rng = Random::default();
//         // XOR the last 31 random numbers together to generate the
//         // pseudo-random number
//         for _ in 0..31 {
//             res ^= rng.random();
//         }
//         res
//     }

//     /// Generates a vector of random bytes of a given length.
//     pub fn bytes(&mut self, len: usize) -> Vec<u8> {
//         let mut res = Vec::with_capacity(len);
//         let mut rng = Random::default();
//         for _ in 0..len {
//             res.push(rng.random() as u8);
//         }
//         res
//     }
//     /// Generates a random floating point number between 0 and 1.
//     pub fn float(&mut self) -> f32 {
//         let mut rng = Random::default();
//         rng.random() as f32 / 0x7FFF as f32
//     }
//     /// Generates a random integer between a minimum and maximum value.
//     pub fn int(&mut self, min: i32, max: i32) -> i32 {
//         let mut rng = Random::default();
//         (rng.random() as f32 / 0x7FFF as f32 * (max - min) as f32) as i32 + min
//     }

//     /// Generates a random boolean value.
//     pub fn bool(&mut self) -> bool {
//         let mut rng = Random::default();
//         rng.random() % 2 == 0
//     }

//     /// Generates a random character.
//     pub fn char(&mut self) -> char {
//         let mut rng = Random::default();
//         let mut res = rng.random() % 26;
//         if rng.random() % 2 == 0 {
//             res += 65;
//         } else {
//             res += 97;
//         }
//         res as u8 as char
//     }

//     /// Generates a random value in a given range.
//     pub fn range(&mut self, min: i32, max: i32) -> i32 {
//         let mut rng = Random::default();
//         rng.int(min, max)
//     }
//     pub fn choose<R>(&self) -> u32 {
//         let mut rng = Random::default();
//         let mut res = rng.random() % 26;
//         if rng.random() % 2 == 0 {
//             res += 65;
//         } else {
//             res += 97;
//         }
//         res
//     }
// }

impl std::fmt::Display for Random {
    /// Formats the `Random` struct as a string for display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Random {{ seed: {} }}", self.seed)
    }
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}
