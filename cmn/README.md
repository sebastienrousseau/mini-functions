# CMN

A Rust library for accessing a collection of mathematical and cryptographic constants

[![Made With Love][mwl]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to CMN 👋

![CMN Banner][banner]

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

Common (CMN), a Rust library designed for developers who are looking for
a comprehensive collection of mathematical and cryptographic constants.

`CMN` is a modern, fast, and user-friendly library that makes it easy to
access a wide range of mathematical and cryptographic constants,
including the mathematical constant `Euler`, the `hash` algorithm used,
the `cost` of the hash algorithm, the `length` of the hash, the
mathematical constant `Phi`, the mathematical constant `Pi`, the
`Planck` constant, a set of `special` characters, and much more.

## Features ✨

The library includes two main structures: `Constant` and `Constants`.

- The `Constant` structure holds the name and value of each constant as
a `&'static str` and a `String`, respectively.
- The Constants structure implements a method constants that returns a
`Vec<Constant>` containing all the available constants.
- The available constants include the mathematical constants `EULER`,
`PHI`, `PI`, `PLANCK`, and `SQRT5`, and the cryptographic constants
`HASH_ALGORITHM`, `HASH_COST`, `HASH_LENGTH`, and `SPECIAL_CHARS`.
- The library also includes an enumeration `ConstantValue` that
represents the different constant values. The values can be an
`f64 float`, a `String`, a `u32`, a `usize`, or a `&'static [char]`
array of characters.

## Installation 📦

It takes just a few minutes to get up and running with `cmn`.

### Requirements

`cmn` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `cmn` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
cmn = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate cmn;
use cmn::*;
```

then you can use the functions in your application code.

### Examples

`CMN` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example cmn
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `CMN` follows [semantic versioning][7].

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
[8]: https://crates.io/crates/cmn
[9]: https://docs.rs/cmn
[10]: https://lib.rs/crates/cmn

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-cmn-1597x377.svg "CMN Banner"
[crates-badge]: https://img.shields.io/crates/v/cmn.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/cmn.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/cmn.svg?style=for-the-badge 'License'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
