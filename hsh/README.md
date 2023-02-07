# HSH

A Cryptographic Hash Algorithms Library for Rust 🦀

[![Made With Love][mwl]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to HSH 👋

![HSH Banner][banner]

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

Hash (HSH), is a Cryptographic Hash Algorithms Library for secure
password management. Utilizing the cutting-edge [Blake3][11] algorithm,
this library generates and verifies hashed passwords with ease.

The library features a struct for storing and verifying hashed
passwords, as well as a series of methods for calculating entropy,
generating hashes, accessing password and hash values, and more.

`HSH` also implements the Default and Display traits for flexible use
and easy readability.

## Features ✨

The `Hash` struct is designed to store and verify password hashes using
the `BLAKE3` cryptographic hash function. The struct has several fields,
including the password and its hash, both represented as strings.

- The `Hash` struct also provides several methods to interact with the
password and its hash.
- The `entropy` method calculates the entropy of the hash in bits based
on the Shannon entropy formula.
- The `generate_hash` method hashes the password and returns the hash
as a string.
- The `hash` and `hash_length` methods provide access to the stored hash
and its length, respectively.
- The `new` method creates a new instance of the Hash struct with
default values for the password and hash.
- The `password` and `password_length` methods provide access to the
stored password and its length, respectively.
- The `set_hash` and `set_password` methods allow you to update the
stored hash and password, respectively.
- Finally, the `verify` method verifies the password against the stored
hash and returns a Boolean indicating whether they match.

The Hash struct implements the `std::fmt::Display trait`, which allows
it to be formatted as a string for printing. The struct also implements
the `Default` trait, which allows it to be initialized with default
values.

## Installation 📦

It takes just a few minutes to get up and running with `hsh`.

### Requirements

`hsh` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `hsh` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
hsh = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate hsh;
use hsh::*;
```

then you can use the functions in your application code.

### Examples

`HRC` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example hsh
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
[8]: https://crates.io/crates/hsh
[9]: https://docs.rs/hsh
[10]: https://lib.rs/crates/hsh
[11]: https://github.com/BLAKE3-team/BLAKE3

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-hsh-1597x377.svg "HSH Banner"
[crates-badge]: https://img.shields.io/crates/v/hsh.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/hsh.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/hsh.svg?style=for-the-badge 'License'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
