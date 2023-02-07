# IDK

A Rust library for managing errors and exceptions

[![Made With Love][made-with-rust]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to IDK 👋

![IDK Banner][banner]

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

IDontKnow (IDK) is a Rust library that has functions and variables
designed to make it easy for your program to report informative error
messages. You can use the IDK library to create error messages that are
clear, concise, and actionable.

## Features ✨

The library includes multiple modules: `Common`, `Error`, `Jwt`,
`Property`, `Stacktrace`, and `Traits`.

- **The common module:** This module provides a foundation of common
  errors that can be utilized throughout the entire project.
  These functionalities serve as a building block for the rest of the
  project, making it easier for developers to create and manage their
  code.
- **The error module:** This module contains all the error types that
  are used in the project. By providing a centralized location for
  errors, developers can quickly and easily identify and resolve any
  issues that may arise.
- **The jwt module:** This module offers the tools necessary to encode
  and decode JSON Web Tokens (JWT). With a simple, easy-to-use
  interface, developers can ensure secure communication between parties.
- **The property module:** This module provides the functionality to
  create and manage properties. By utilizing this module, developers can
  keep track of all properties within the project and make changes as
  necessary.
- **The stacktrace module:** This module offers the tools to create and
  manage stacktraces. By providing detailed information about the
  execution of the code, developers can quickly identify and resolve any
  issues that may arise.
- **The traits module:** This module provides functionality to create
  and manage traits. By utilizing this module, developers can ensure
  that all traits within the project are consistent and well-defined.

## Installation 📦

It takes just a few minutes to get up and running with `idk`.

### Requirements

`idk` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `idk` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
idk = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate idk;
use idk::*;
```

then you can use the functions in your application code.

### Examples

`IDK` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example idk
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `IDK` follows [semantic versioning][7].

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
[8]: https://crates.io/crates/idk
[9]: https://docs.rs/idk
[10]: https://lib.rs/crates/idk

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-idk-1597x377.svg "IDK Banner"
[crates-badge]: https://img.shields.io/crates/v/idk.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/idk.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/idk.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/Made%20with-Rust-c0282d.svg?style=for-the-badge&color=f04041 'Made With Rust'
