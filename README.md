<!-- markdownlint-disable MD033 MD041 -->

<img src="https://kura.pro/mini-functions/images/v2/logos/mini-functions.svg"
alt="Common (CMN) logo" height="261" width="261" align="right" />

<!-- markdownlint-enable MD033 MD041 -->

# Mini Functions

A Rust library of highly performant utility and wrapper functions

<!-- markdownlint-disable MD033 -->
<center>

[![Mini Functions][title]][00]

[![Made With Love][made-with-rust]][10]
[![Crates.io][crates-badge]][07]
[![Lib.rs][libs-badge]][09]
[![Docs.rs][docs-badge]][08]
[![License][license-badge]][02]
[![Codecov][codecov-badge]][11]

![divider][divider]

• [Website][00] • [Documentation][08] • [Report Bug][03] • [Request Feature][03] • [Contributing Guidelines][04]

</center>

## Overview 📖

`Mini Functions` is a highly performant utility and wrapper functions library for Rust that has been carefully designed with optimization and efficiency in mind. By providing convenient wrapper functions, our library aims to provide a high-level interface for common tasks while still leveraging the performance benefits of Rust under the hood. These utility functions serve as an essential toolkit for any Rust developer, and the library's design abstractions allow for easy integration into a variety of projects and applications.

These utility functions serve as an essential toolkit for any Rust developer, and the library's design abstractions allow for easy integration into a variety of projects and applications.

## Features ✨

- **Built with Rust** — A modern programming language that is well-suited for building high-performance, reliable, and secure systems.
- **High-level Utility Functions** — A collection of high-level, abstracted functions for common tasks, such as string manipulation, file manipulation, and data parsing.
- **Wrapper Functions for Easy Access** — Wrapper functions that provide a more convenient interface for accessing and using underlying Rust libraries or APIs.
- **Optimization and Performance Tools** — Tools for optimizing and improving the performance of Rust code.
- **Multi-platform Support** — Support for a variety of platforms, including desktop, mobile, and web.
- **Comprehensive Documentation and Examples** — Documentation and examples to help developers understand and use the library effectively.
- **Regular Maintenance and Updates** — Regular updates and maintenance to ensure the library stays up-to-date and reliable.

## Functionality 📚

`Mini Functions` is a library of functions for Rust that provides a collection of tools for working with various aspects of a Rust application. The `mini-functions` library consists of the following `non-exhaustive` functions:

| Functions | Description |
| -------- | ----------- |
| [Claims ⧉][12] | The **Claim functions** are used to retrieve and manipulate information about claims. These functions are used to create and manage claims in JSON Web Tokens (JWT) and JSON Web Signatures (JWS). |
| [Common ⧉][13] | The **Common functions** are used to retrieve and manipulate information about common data types. |
| [Date ⧉][14] | The **Date and time functions** are used to retrieve and manipulate information about dates and times. |
| [Errors ⧉][15] | The **Error functions** are used to retrieve and manipulate information about errors. |
| [Hash ⧉][16] | The **Hash functions** are used to retrieve and manipulate information about hashes. |
| [Logs ⧉][17] | The **Log functions** are used to retrieve and manipulate information about logs. |
| [JWT ⧉][18] | The **Jot functions** are used to retrieve and manipulate information about JSON Object Tokens (JOT). |
| [MD5 ⧉][19] | The **MD5 functions** are used to retrieve and manipulate information about MD5. |
| [QR ⧉][20] | The **QR functions** are used to retrieve and manipulate information about QR codes. |
| [Random ⧉][21] | The **Random functions** are used to retrieve and manipulate information about random data. |

See [Documentation][08] for full API details.

![divider][divider]

## Getting Started 🚀

It takes just a few minutes to get up and running with `mini-functions`.

### Requirements

The minimum supported Rust toolchain version is currently Rust
**1.56.0** or later (stable).

### Installation

To install `mini-functions`, you need to have the Rust toolchain installed on your machine. You can install the Rust toolchain by following the instructions on the [Rust website ⧉][10].

Once you have the Rust toolchain installed, you can install `mini-functions` using the following command:

```shell
cargo install mini-functions
```

![Divider][divider]

## Usage 📖

To use the `mini-functions` library in your project, add the following to your `Cargo.toml` file:

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

### Platform support

`mini-functions` is supported and tested on the following platforms:

### Tier 1 platforms 🏆

| | Operating System | Target | Description |
| --- | --- | --- | --- |
| ✅ | Linux   | aarch64-unknown-linux-gnu | 64-bit Linux systems on ARM architecture |
| ✅ | Windows | i686-pc-windows-gnu | 32-bit Windows systems using the GNU toolchain |
| ✅ | Windows | i686-pc-windows-msvc | 32-bit Windows systems using the Microsoft Visual C toolchain |
| ✅ | Linux   | i686-unknown-linux-gnu | 32-bit Linux systems (kernel 3.2+, glibc 2.17+) |
| ✅ | macOS   | x86_64-apple-darwin | 64-bit macOS systems (10.7 Lion or later) |
| ✅ | Windows | x86_64-pc-windows-gnu | 64-bit Windows systems using the GNU toolchain |
| ✅ | Windows | x86_64-pc-windows-msvc | 64-bit Windows systems using the Microsoft Visual C toolchain |
| ✅ | Linux   | x86_64-unknown-linux-gnu | 64-bit Linux systems (kernel 2.6.32+, glibc 2.11+) |

### Tier 2 platforms 🥈

| | Operating System | Target | Description |
| --- | --- | --- | --- |
| ✅ | Linux   | aarch64-apple-darwin | 64-bit macOS on Apple Silicon |
| ✅ | Windows | aarch64-pc-windows-msvc | 64-bit Windows on ARM architecture using the Microsoft Visual C toolchain |
| ✅ | Linux   | aarch64-unknown-linux-musl | 64-bit Linux on ARM architecture with musl libc |
| ✅ | Linux   | arm-unknown-linux-gnueabi | ARMv6 Linux systems (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | arm-unknown-linux-gnueabihf | ARMv7 Linux systems, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | armv7-unknown-linux-gnueabihf | ARMv7 Linux systems, hardfloat (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | powerpc-unknown-linux-gnu | PowerPC Linux systems (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | powerpc64-unknown-linux-gnu | PowerPC64 Linux systems (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | powerpc64le-unknown-linux-gnu | PowerPC64le Linux systems (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | riscv64gc-unknown-linux-gnu | RISC-V Linux systems (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | s390x-unknown-linux-gnu | s390x Linux systems (kernel 3.2, glibc 2.17) |
| ✅ | Linux   | x86_64-unknown-freebsd | 64-bit FreeBSD systems on x86-64 architecture |
| ✅ | Linux   | x86_64-unknown-linux-musl | 64-bit Linux systems (kernel 2.6.32+, musl libc) |

The [GitHub Actions ⧉][22] shows the platforms in which the `mini-functions` library tests are run.

![divider][divider]

### Documentation

**Info:** Please check out our [website ⧉][00] for more information. You can find our documentation on [docs.rs ⧉][08], [lib.rs ⧉][09] and [crates.io ⧉][07].

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain backward compatibility, `mini-functions` follows [semantic versioning ⧉][06].

![divider][divider]

## License 📝

The project is licensed under the terms of Apache License, Version 2.0 and the MIT license.

![divider][divider]

## Contribution 🤝

We welcome all people who want to contribute. Please see the [contributing instructions ⧉][04] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must adhere to the [Rust's Code of Conduct ⧉][23].

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

![divider][divider]

## Acknowledgements 💙

A big thank you to all the awesome contributors of [mini-functions ⧉][05] for their help and support.

A special thank you goes to the [Rust Reddit ⧉][24] community for providing a lot of useful suggestions on how to improve this project.

[00]: https://minifunctions.com "Mini Functions - Highly performant utility and wrapper functions library for Rust"
[01]: http://www.apache.org/licenses/LICENSE-2.0 "Apache License, Version 2.0"
[02]: http://opensource.org/licenses/MIT "MIT license"
[03]: https://github.com/sebastienrousseau/mini-functions/issues "Mini Functions Issues"
[04]: https://raw.githubusercontent.com/sebastienrousseau/mini-functions/main/.github/CONTRIBUTING.md "Mini Functions Contributing Guidelines"
[05]: https://github.com/sebastienrousseau/mini-functions/graphs/contributors "Mini Functions Contributors"
[06]: http://semver.org/ "Semantic Versioning 2.0.0"
[07]: https://crates.io/crates/mini-functions "Mini Functions on Crates.io"
[08]: https://docs.rs/mini-functions "Mini Functions on Docs.rs"
[09]: https://lib.rs/crates/mini-functions "Mini Functions on Lib.rs"
[10]: https://www.rust-lang.org/ "The Rust Programming Language"
[11]: https://codecov.io/github/sebastienrousseau/mini-functions "Mini Functions Codecov"
[12]: https://docs.rs/mini-functions/0.0.8/mini_functions/claims/index.html "Mini Functions Claims"
[13]: https://docs.rs/mini-functions/0.0.8/mini_functions/common/index.html "Mini Functions Common"
[14]: https://docs.rs/mini-functions/0.0.8/mini_functions/date/index.html "Mini Functions Date"
[15]: https://docs.rs/mini-functions/0.0.8/mini_functions/errors/index.html "Mini Functions Errors"
[16]: https://docs.rs/mini-functions/0.0.8/mini_functions/hash/index.html "Mini Functions Hash"
[17]: https://docs.rs/mini-functions/0.0.8/mini_functions/logs/index.html "Mini Functions Logs"
[18]: https://docs.rs/mini-functions/0.0.8/mini_functions/jwt/index.html "Mini Functions JWT"
[19]: https://docs.rs/mini-functions/0.0.8/mini_functions/md5/index.html "Mini Functions MD5"
[20]: https://docs.rs/mini-functions/0.0.8/mini_functions/qr/index.html "Mini Functions QR"
[21]: https://docs.rs/mini-functions/0.0.8/mini_functions/random/index.html "Mini Functions Random"
[22]: https://github.com/sebastienrousseau/mini-functions/actions "Mini Functions on GitHub Actions"
[23]: https://www.rust-lang.org/policies/code-of-conduct "Rust Code of Conduct"
[24]: https://www.reddit.com/r/rust/ "Reddit"

[banner]: https://kura.pro/mini-functions/images/v2/banners/banner-mini-functions.svg "Mini Functions Banner"
[codecov-badge]: https://img.shields.io/codecov/c/github/sebastienrousseau/mini-functions?style=for-the-badge&token=M1REIC3QCK 'Codecov'
[crates-badge]: https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge 'Crates.io'
[divider]: https://kura.pro/common/images/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/mini-functions.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.8-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge 'License'
[logo]: https://kura.pro/mini-functions/images/v2/logos/mini-functions.svg "Mini Functions Logo"
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
[title]: https://kura.pro/mini-functions/images/v2/titles/title-mini-functions.svg "Mini Functions Logo"
