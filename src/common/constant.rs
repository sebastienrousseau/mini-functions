// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Constant values used in the application.

/// The CRATE constant contains the URL of the crate.
// pub const CRATE: &str = "https://crates.io/crates/mini-functions";

/// The DOCUMENTATION constant contains the URL of the documentation.
// pub const DOCUMENTATION: &str = "https://docs.rs/mini-functions";

/// The GITHUB constant contains the URL of the GitHub repository.
// pub const GITHUB: &str = "https://github.com/sebastienrousseau/mini-functions";

/// The HASH_COST constant contains the cost of the hash.
pub const HASH_COST: u32 = 8;

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

/// The GOLDEN_RATIO constant contains the golden ratio value.
pub const GOLDEN_RATIO: u32 = 0x9E3779B9;
