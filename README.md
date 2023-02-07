# Mini Functions 🦀

[![Made With Love][mwl]][6]

<!-- markdownlint-disable MD033 -->
<center>

## A Rust library of highly performant utility and wrapper functions 🚀

![Mini Functions][banner]

[![Rust][rust-badge]][12]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![codecov][codecov-badge]][13]
[![License][license-badge]][2]
[![Fossa][fossa-badge]][14]

![divider][divider]

**[Website][0]
• [Documentation][9]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

</center>

## Welcome to the Mini Functions Library for Rust 👋

`Mini Functions` is a highly performant utility and wrapper functions
library for Rust that has been carefully designed with optimization and
efficiency in mind.

By providing convenient wrapper functions, our library aims to provide a
high-level interface for common tasks while still leveraging the
performance benefits of Rust under the hood.

These utility functions serve as an essential toolkit for any Rust
developer, and the library's design abstractions allow for easy
integration into a variety of projects and applications.

## Key Features 🎯

- **Built with Rust** — A modern programming language that is well-
  suited for building high-performance, reliable, and secure systems.
- **High-level Utility Functions** — A collection of high-level,
  abstracted functions for common tasks, such as string manipulation,
  file manipulation, and data parsing.
- **Wrapper Functions for Easy Access** — Wrapper functions that provide
  a more convenient interface for accessing and using underlying Rust
  libraries or APIs.
- **Optimization and Performance Tools** — Tools for optimizing and
  improving the performance of Rust code.
- **Multi-platform Support** — Support for a variety of platforms,
  including desktop, mobile, and web.
- **Comprehensive Documentation and Examples** — Documentation and
  examples to help developers understand and use the library effectively.
- **Regular Maintenance and Updates** — Regular updates and maintenance
  to ensure the library stays up-to-date and reliable.

## Installation 📦

It takes just a few minutes to get up and running with `mini-functions`.

### Requirements

`mini-functions` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `mini-functions` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
mini-functions = "0.0.8"
```

Add the following to your `main.rs` file:

```rust
extern crate mini_functions;
use mini_functions::*;
```

then you can use the functions in your application code.

### Examples

`Mini Functions` comes with a set of examples that you can use to get
started. The examples are located in the `examples` directory of the
project. To run the examples, clone the repository and run the following
command in your terminal from the project root directory.

```shell
cargo run --example date
```

> 💡 **Note:** The examples available are date, hash, log, password, qrcode, random and uuid.

## The Functions library 📚

`Mini Functions` is a library of functions for Rust that provides a
collection of tools for working with various aspects of a Rust
application. The `mini-functions` library consists of the following
`non-exhaustive` functions:

| Functions | Description |
| -------- | ----------- |
| [![Claims][claims-badge]][12] | The **Claim functions** are used to retrieve and manipulate information about claims. These functions are used to create and manage claims in JSON Web Tokens (JWT) and JSON Web Signatures (JWS). |
| [![Common][common-badge]][12] | The **Common functions** are used to retrieve and manipulate information about common data types. |
| [![Date][date-badge]][12] | The **Date and time functions** are used to retrieve and manipulate information about dates and times. |
| [![Errors][errors-badge]][12] | The **Error functions** are used to retrieve and manipulate information about errors. |
| [![Hash][hash-badge]][12] | The **Hash functions** are used to retrieve and manipulate information about hashes. |
| [![Jot][jot-badge]][12] | The **Jot functions** are used to retrieve and manipulate information about JSON Object Tokens (JOT). |
| [![rlg][rlg-badge]][12] | The **rlg functions** are used to retrieve and manipulate information about logging. |
| [![MD5][mdg-badge]][12] | The **MD5 functions** are used to retrieve and manipulate information about MD5. |
| [![Password][password-badge]][12] | The **Password functions** are used to retrieve and manipulate information about passwords. |
| [![QR][qr-badge]][12] | The **QR functions** are used to retrieve and manipulate information about QR codes. |
| [![Random][random-badge]][12] | The **Random functions** are used to retrieve and manipulate information about random data. |

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `Mini Functions` follows
[semantic versioning][7].

## License 📝

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fsebastienrousseau%2Fmini-functions.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fsebastienrousseau%2Fmini-functions?ref=badge_large)

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
[8]: https://crates.io/crates/mini-functions
[9]: https://docs.rs/mini-functions
[10]: https://lib.rs/crates/mini-functions
[12]: https://www.rust-lang.org/
[13]: https://codecov.io/github/sebastienrousseau/mini-functions
[14]: https://app.fossa.com/projects/git%2Bgithub.com%2Fsebastienrousseau%2Fmini-functions?ref=badge_shield

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-mini-functions.svg "Mini Functions - Rust 🦀"
[crates-badge]: https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge 'Crates.io'
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/mini-functions?style=for-the-badge&token=M1REIC3QCK 'Codecov'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/mini-functions.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.8-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge 'License'
[rust-badge]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Rust'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
[fossa-badge]: https://img.shields.io/static/v1?style=for-the-badge&message=Fossa&color=289E6D&logo=Fossa&logoColor=FFFFFF&label= 'Fossa'

[claims-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-claims.png 'Claims'
[common-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-common.png 'Common'
[date-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-date.png 'Date'
[errors-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-errors.png 'Errors'
[hash-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-hash.png 'Hash'
[jot-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-jot.png 'Jot'
[rlg-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-rlg.png 'rlg'
[mdg-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-mdg.png 'MD5'
[password-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-password.png 'Password'
[qr-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-qr.png 'QR'
[random-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/png/ico-random.png 'Random'
