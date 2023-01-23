// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

pub struct Constants;

impl Constants {
    pub fn constant(&self) -> Vec<(&'static str, String)> {
        vec![
            ("EULER_CONSTANT", format!("{}", EULER_CONSTANT)),
            ("HASH_ALGORITHM", HASH_ALGORITHM.to_string()),
            ("HASH_COST", format!("{}", HASH_COST)),
            ("HASH_LENGTH", format!("{}", HASH_LENGTH)),
            ("PHI_CONSTANT", format!("{}", PHI_CONSTANT)),
            ("PI_CONSTANT", format!("{}", PI_CONSTANT)),
            ("PLANCK_CONSTANT", format!("{}", PLANCK_CONSTANT)),
            ("SQRT5_CONSTANT", format!("{}", SQRT5_CONSTANT)),
        ]
    }
}

pub const EULER_CONSTANT: f64 = std::f64::consts::E;
pub const HASH_ALGORITHM: &str = "Blake3";
pub const HASH_COST: u32 = 8;
pub const HASH_LENGTH: usize = 32;
pub const PHI_CONSTANT: f64 = (1.0 + SQRT5_CONSTANT) / 2.0;
pub const PI_CONSTANT: f64 = std::f64::consts::PI;
pub const PLANCK_CONSTANT: f64 = 6.626_070_15e-34_f64;
pub const SQRT5_CONSTANT: f64 = 2.236_067_977_499_79_f64;

/// Constant values used in the application.

/// The CRATE constant contains the URL of the crate.
// pub const CRATE: &str = "https://crates.io/crates/mini-functions";

/// The DOCUMENTATION constant contains the URL of the documentation.
// pub const DOCUMENTATION: &str = "https://docs.rs/mini-functions";

/// The GITHUB constant contains the URL of the GitHub repository.
// pub const GITHUB: &str = "https://github.com/sebastienrousseau/mini-functions";

/// The HASH_COST constant contains the cost of the hash.
// pub const HASH_COST: u32 = 8;

/// The HASH_LENGTH constant contains the length of the hash.
// pub const HASH_LENGTH: usize = 32;

/// The HOMEPAGE constant contains the URL of the homepage.
// pub const HOMEPAGE: &str = "https://minifunctions.com/";

/// The SPECIAL constant contains the special characters.
// pub const SPECIAL: &[u8] = b"!@#$%^&*()_+-=[]{};':,./<>?";
// pub const SPECIAL: &[u8] = b"!@#$%^&*()_+-=[]{}|;':\"<>,.?/~`";

/// The SPECIAL_CHARS constant contains the special characters.
pub const SPECIAL_CHARS: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '-', '=', '[', ']', '{', '}', '|',
    ';', ':', '"', '<', '>', ',', '.', '?', '/', '~', '`',
];

// The GOLDEN_RATIO constant contains the golden ratio value.
// pub const GOLDEN_RATIO: u32 = 0x9E3779B9;
