// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Contains several commonly used mathematical and cryptographic
/// constants.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Constant {
    /// The name of the constant.
    pub name: &'static str,

    /// The value of the constant.
    pub value: String,
}
/// The `Constants` structure holds mathematical and hash constants.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Constants;

impl Constants {
    /// Returns a vector of tuples with the constant name and its value.
    pub fn constants(&self) -> Vec<Constant> {
        vec![
            Constant {
                name: "EULER",
                value: EULER.to_string(),
            },
            Constant {
                name: "HASH_ALGORITHM",
                value: HASH_ALGORITHM.to_string(),
            },
            Constant {
                name: "HASH_COST",
                value: HASH_COST.to_string(),
            },
            Constant {
                name: "HASH_LENGTH",
                value: HASH_LENGTH.to_string(),
            },
            Constant {
                name: "PHI",
                value: PHI.to_string(),
            },
            Constant {
                name: "PI",
                value: PI.to_string(),
            },
            Constant {
                name: "PLANCK",
                value: PLANCK.to_string(),
            },
            Constant {
                name: "SQRT5",
                value: SQRT5.to_string(),
            },
            Constant {
                name: "SPECIAL_CHARS",
                value: format!("{SPECIAL_CHARS:?}"),
            },
        ]
    }
}

/// Enum to represent the different constant values.
#[derive(Debug, Clone, Serialize)]
pub enum ConstantValue {
    /// A float value represented as `f64`.
    Float(f64),
    /// A string value.
    String(String),
    /// An unsigned 32-bit integer value represented as `u32`.
    U32(u32),
    /// An unsigned integer with the size of a pointer represented as `usize`.
    Usize(usize),
    /// An array of characters represented as `&'static [char]`.
    CharArray(&'static [char]),
}

/// The mathematical constant `E`, the base of the natural logarithm.
pub const EULER: f64 = std::f64::consts::E;

/// The hash algorithm used. The default is Blake3.
pub const HASH_ALGORITHM: &str = "Blake3";

/// The cost of the hash algorithm. The default is 8.
pub const HASH_COST: u32 = 8;

/// The length of the hash. The default is 32.
pub const HASH_LENGTH: usize = 32;

/// The mathematical constant `φ` or the golden ratio.
pub const PHI: f64 = (1.0 + SQRT5) / 2.0;

/// The mathematical constant `π`.
pub const PI: f64 = std::f64::consts::PI;

/// The Planck constant, `h`.
pub const PLANCK: f64 = 6.626_070_15e-34_f64;

/// A set of special characters.
pub const SPECIAL_CHARS: &[char] = &[
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '=', '[', ']', '{', '}', '|', ';',
    ':', '"', '<', '>', ',', '.', '?', '/', '~', '`',
];

/// The square root of 5.
pub const SQRT5: f64 = 2.236_067_977_499_79_f64;
