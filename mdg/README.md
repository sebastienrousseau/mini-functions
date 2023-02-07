# MDG

A Rust library that implements the MD5 cryptographic hash function

[![Made With Love][mwl]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to MDG 👋

![MDG Banner][banner]

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

The Message Digest (MDG) is an implementation of the MD5 cryptographic
hash function. It provides a struct, MD5, that can generate a message
digest of data in a secure, one-way hash. The message digest can verify
the integrity of the data without having to store the entire message.

## Features ✨

The MD5 struct has a variety of functions to make it easy to use. The
new function creates a new instance of the struct, ready to hash data.
The transform function updates the struct 's internal state with new
data, and the finalize function finishes the message digest and returns
it.

- An MD5 struct that represents the current state of the hash
  computation.
- A `new()` method to create a new instance of the struct.
- A `transform()` method to update the internal state with new data.
- A `finalize()` method to finalize the hash computation and return the
  result as a 16-byte array.

## Installation 📦

It takes just a few minutes to get up and running with `mdg`.

### Requirements

`mdg` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `mdg` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
mdg = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate mdg;
use mdg::*;
```

then you can use the functions in your application code.

### Examples

`MDG` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example mdg
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `QRC` follows [semantic versioning][7].

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
for their help and support.

[0]: https://minifunctions.com
[1]: http://www.apache.org/licenses/LICENSE-2.0
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/mini-functions/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/mini-functions/main/.github/CONTRIBUTING.md
[6]: https://github.com/sebastienrousseau/mini-functions/graphs/contributors
[7]: http://semver.org/
[8]: https://crates.io/crates/mdg
[9]: https://docs.rs/mdg
[10]: https://lib.rs/crates/mdg

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-mdg-1597x377.svg "MDG Banner"
[crates-badge]: https://img.shields.io/crates/v/mdg.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/mdg.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/mdg.svg?style=for-the-badge 'License'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
