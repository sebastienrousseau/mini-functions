// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Example code using the `vrd` crate.
//!
//! This example demonstrates the usage of the `vrd` crate.
//! It imports the `Random` trait and other items from the `vrd` crate.
//! The example code showcases the generation of random numbers.
//!
//! # Example
//!
//! ```
//! extern crate vrd;
//!
//! use self::vrd::Random;
//! use vrd::*;
//!
//! fn main() {
//!     let random_number = u32::random();
//!     println!("Random number: {}", random_number);
//! }
//! ```
use mini_functions::random::*;

fn main() {
    // Create a new random boolean with 50% probability of being true
    let bool: bool = Random::bool(&mut Random::new(), 0.5);
    println!("🦀 Random::bool():        ✅ {bool}");

    // Create a new random number generator
    let mut rng = Random::new();
    println!("🦀 Random::new():         ✅ {rng}");

    // Get the default random number generator
    let default = Random::default();
    println!("🦀 Random::default():     ✅ {default}");

    // Generate a random number between 0 and u32::MAX
    let random = rng.rand();
    println!("🦀 Random::random():      ✅ {random}");

    // Seed the PRNG with a given value
    let seed_value = 12345;
    let mut rng = Random::new();
    rng.seed(seed_value); // Seed the RNG
    println!("🦀 Random::seed():      ✅ {}", seed_value); // Print the seed value

    // Generate a vector of random bytes with a given length
    let bytes = Random::bytes(&mut rng, 1000);
    println!("🦀 Random::bytes():       ✅ {bytes:?}");

    // Generate a random float between 0 and 1
    let float = rng.rand() as f32 / 0x7FFF as f32;
    println!("🦀 Random::float():       ✅ {float}");

    // Generate a random usize
    let int = rng.rand() as usize;
    println!("🦀 Random::int():         ✅ {int}");

    // Generate a random integer within a range of values
    let mut rng = Random::new();
    let min = 0;
    let max = 100;

    // Generate a random integer within a range
    let rand_int = rand_int!(rng, min, max);
    println!(
        "🦀 Random integer between {} and {}: {}",
        min, max, rand_int
    );

    // Generate a random number within a range
    let rand_range =
        rand_float!(rng) * (max as f32 - min as f32) + min as f32;
    println!("🦀 Random number in [{min}, {max}): {}", rand_range);

    // Generate a random 32-bit unsigned integer within a range
    let rand_uint = random_range!(rng, 0, u32::MAX);
    println!("🦀 Random u32 between 0 and u32::MAX: {}", rand_uint);

    // Generate a random boolean with a given probability
    let rand_bool = rand_bool!(rng, 0.5);
    println!("🦀 Random boolean with 50% probability: {}", rand_bool);

    // Generate a vector of random bytes with a given length
    let rand_bytes = rand_bytes!(rng, 10);
    println!("🦀 Random bytes: {:?}", rand_bytes);

    // Generate a random char within the range 'a'..='z'
    let rand_char = rand_char!(rng);
    println!("🦀 Random char between 'a' and 'z': {}", rand_char);

    // Generate a random element from a slice of values
    let values = &[1, 2, 3, 4, 5];
    let rand_choose = rand_choose!(rng, values);
    println!(
        "🦀 Random element from [1, 2, 3, 4, 5]: {:?}",
        rand_choose
    );

    // Generate a random float
    let rand_float = rand_float!(rng);
    println!("🦀 Random float: {}", rand_float);

    // Generate a random 32-bit unsigned integer
    let rand_pseudo = rng.pseudo();
    println!("🦀 Random u32 using the PRNG: {}", rand_pseudo);

    // Seed the PRNG with a given value
    rand_seed!(rng, 42);
    let rand_seed = rng.pseudo();
    println!("🦀 Random u32 using the seeded PRNG: {}", rand_seed);

    // Twist the state of the PRNG
    rand_twist!(rng);
    let rand_twist = rng.pseudo();
    println!(
        "🦀 Random u32 after twisting the PRNG state: {}",
        rand_twist
    );
}
