# Mini Functions 🦀

<!-- markdownlint-disable MD033 -->
<center>

## A Rust library of highly performant utility and wrapper functions 🚀

![Mini Functions][banner]

[![Rust][made-with-rust-badge]][12] [![Crates.io][crates-badge]][8] [![Lib.rs][libs-badge]][10] [![Docs.rs][docs-badge]][9] [![License][license-badge]][2]

![divider][divider]

**[Website][0]
• [Documentation][9]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

</center>

## Welcome to Mini Functions 👋

`Mini Functions` is a highly performant utility and wrapper functions library for Rust that has been carefully designed with optimization and efficiency in mind.

By providing convenient wrapper functions, our library aims to provide a high-level interface for common tasks while still leveraging the performance benefits of Rust under the hood.

These utility functions serve as an essential toolkit for any Rust developer, and the library's design abstractions allow for easy integration into a variety of projects and applications.

## Key Features 🎯

- **Built with Rust** — A modern programming language that is well-suited for building high-performance, reliable, and secure systems.
- **High-level Utility Functions** — A collection of high-level, abstracted functions for common tasks, such as string manipulation, file manipulation, and data parsing.
- **Wrapper Functions for Easy Access** — Wrapper functions that provide a more convenient interface for accessing and using underlying Rust libraries or APIs.
- **Optimization and Performance Tools** — Tools for optimizing and improving the performance of Rust code.
- **Multi-platform Support** — Support for a variety of platforms, including desktop, mobile, and web.
- **Comprehensive Documentation and Examples** — Documentation and examples to help developers understand and use the library effectively.
- **Regular Maintenance and Updates** — Regular updates and maintenance to ensure the library stays up-to-date and reliable.

## Installation 📦

It takes just a few minutes to get up and running with `mini-functions`.

### Requirements

`mini-functions` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information and find our documentation on [docs.rs][9], [lib.rs][10] and [crates.io][8].

## Usage 📖

To use `mini-functions` in your project, add the following to your `Cargo.toml` file:

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

`Mini Functions` comes with a set of examples that you can use to get started. The examples are located in the `examples` directory of the project. To run the examples, clone the repository and run the following command in your terminal from the project root directory.

```shell
cargo run --example date
```

> 💡 **Note:** The examples available are date, hash, log, password, qrcode, random and uuid.

![divider][divider]

## License 📜

The Mini Functions Website is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

- [Apache License, Version 2.0][2]
- [MIT license][3]

![divider][divider]

## What's Changed

![divider][divider]

## Acknowledgements 💙

A big thank you to all the awesome contributors of [Mini Functions][4] for their help and continuous support.

[0]: https://minifunctions.com
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/mini-functions/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/mini-functions/main/.github/CONTRIBUTING.md
[8]: https://crates.io/crates/mini-functions
[9]: https://docs.rs/mini-functions
[10]: https://lib.rs/crates/mini-functions
[12]: https://www.rust-lang.org/

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/banners/banner-mini-functions.svg "Mini Functions - Rust 🦀"
[crates-badge]: https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/mini-functions.svg?style=for-the-badge 'Docs.rs '
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.8-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge 'License'
[made-with-rust-badge]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-rust.svg "Made With Rust 🦀"
