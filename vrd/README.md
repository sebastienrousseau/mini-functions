# VRD

A Rust library for generating random and pseudo-random numbers based on
the Mersenne Twister algorithm

[![Made With Love][mwl]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to VRD 👋

![VRD Banner][banner]

<!-- markdownlint-disable MD033 -->
<center>

**[Website][0]
• [Documentation][9]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

</center>

<!-- markdownlint-enable MD033 -->

## Overview 📖

The Varied (VRD) library is an implementation of a pseudorandom number
generator using the Mersenne Twister algorithm.

The library generates random integers uniformly distributed in the range
of 0 to (2^32 - 1) and supports generating random booleans, characters,
floats, integers, and vectors of random bytes.

- `vrd` provides functions to return random values with a specific
probability, choose a random element from a slice of values, and
generate random values within a given range.
- The library also provides a struct named Random which contains an
array of unsigned 32-bit integers and an index used to generate random
numbers.
- The index is incremented after each random number is generated, and
the array is reinitialized when the index reaches 624.
- The library uses an extern crate 'rand' and uses its thread_rng
function to generate random numbers.

## Features ✨

- Pseudorandom number generation: The library uses the Mersenne Twister
  algorithm to generate pseudorandom integers uniformly distributed in 0
  to (2^32 - 1) using an array of unsigned 32-bit integers and an index.
- Random number types: The library provides several methods to generate
  different types of random numbers including bool, bytes, char, float,
  and int.
- Range of values: The methods for generating random numbers allow the
  user to specify the range of values for the output.
- Random element selection: The library provides a method to choose a
  random element from a given slice of values.
- Initialization: The library provides a new() method to create a new
  instance of the random number generator.
- Optimization: The library is optimized for performance with the number
  of elements in the array set to 624 and the number of elements to skip
  set to 397.
- Constant values: The library uses several constant values in the
  Mersenne Twister algorithm including MATRIX_A, UPPER_MASK, LOWER_MASK,
  TEMPERING_MASK_B, and TEMPERING_MASK_C.

## Installation 📦

It takes just a few minutes to get up and running with `vrd`.

### Requirements

`vrd` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `vrd` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
vrd = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate vrd;
use vrd::*;
```

then you can use the functions in your application code.

### Examples

`VRD` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example vrd
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `VRD` follows [semantic versioning][7].

## License 📝

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

## Contribution 🤝

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

![divider][divider]

## Acknowledgements 💙

A big thank you to all the awesome contributors of [Mini Functions][6]
for their help and support. A special thank you goes to the
[Rust Reddit](https://www.reddit.com/r/rust/) community for providing a
lot of useful suggestions on how to improve this project.

[0]: https://minifunctions.com
[1]: http://www.apache.org/licenses/LICENSE-2.0
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/mini-functions/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/mini-functions/main/.github/CONTRIBUTING.md
[6]: https://github.com/sebastienrousseau/mini-functions/graphs/contributors
[7]: http://semver.org/
[8]: https://crates.io/crates/vrd
[9]: https://docs.rs/vrd
[10]: https://lib.rs/crates/vrd

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-vrd-1597x377.svg "VRD Banner"
[crates-badge]: https://img.shields.io/crates/v/vrd.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/vrd.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/vrd.svg?style=for-the-badge 'License'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
