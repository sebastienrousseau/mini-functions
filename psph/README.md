# PSPH

A Rust library for accessing a collection of mathematical and cryptographic constants

[![Made With Love][made-with-rust]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to PSPH 👋

![PSPH Banner][banner]

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

PassPhrase (PSPH) is a Rust library that empowers developers to elevate
the security of their applications with ease. `PSPH` generates secure
and strong passphrases using a unique combination of letters, numbers,
cases, and symbols to form an unpredictable string of characters that
doesn't resemble words or names with a high level of entropy.

## Features ✨

`PSPH` uses the `cmn` crate for constants; the `hsh` crate for password
hashing, and the `vrd` crate for random number generation.

The `Password` struct is the cornerstone of `PSPH`, storing the
generated passphrase, as well as the optional special characters and
separators to use. This struct also provides a range of functions for
evaluating the security of the password, such as entropy calculation,
hash generation, password validation, and much more.

- **Passphrase Generation**: Generates random, secure passphrases
  consisting of multiple words separated by a specified separator.
- **Entropy Calculation**: Calculates the entropy of a password based on
  its length, the number of unique characters used, and the number of
  bits of the hash generated from the password. The entropy value
  provides insight into the security of the password, with higher
  entropy values indicating stronger security.
- **Hash Generation**: Returns the hash of the generated password,
- enabling secure storage of the passphrase.
- **Hash Length Calculation**: Evaluates the length of the hash
  generated from the password.
- **Passphrase Validation**: Checks if the generated passphrase is
  empty, making sure that a secure password is always generated.
- **Passphrase Length Calculation**: Evaluates the length of the
  generated passphrase, giving developers full control over password
  length.

## Installation 📦

It takes just a few minutes to get up and running with `psph`.

### Requirements

`psph` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `psph` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
psph = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate psph;
use psph::*;
```

then you can use the functions in your application code.

### Examples

`PSPH` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example psph
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `PSPH` follows [semantic versioning][7].

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
[8]: https://crates.io/crates/psph
[9]: https://docs.rs/psph
[10]: https://lib.rs/crates/psph

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-psph-1597x377.svg "PSPH Banner"
[crates-badge]: https://img.shields.io/crates/v/psph.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/psph.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/psph.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
