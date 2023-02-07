# RLG

A Rust library that implements application-level logging with a simple, readable output format

[![Made With Love][mwl]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to RLG 👋

![RLG Banner][banner]

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

Rust Logs (RLG) is a library that implements application-level logging
in a simple, readable output format. The library provides logging APIs
and various helper macros that simplify many common logging tasks.

## Features ✨

- Supports many log levels: `ALL`, `DEBUG`, `DISABLED`, `ERROR`,
  `FATAL`, `INFO`, `NONE`, `TRACE`, `VERBOSE` and `WARNING`,
- Provides structured log formats that are easy to parse and filter,
- Compatible with multiple output formats including:
  - Common Event Format (CEF),
  - Extended Log Format (ELF),
  - Graylog Extended Log Format (GELF),
  - JavaScript Object Notation (JSON),
  - NCSA Common Log Format (CLF),
  - W3C Extended Log File Format (W3C),
  - and many more

## Installation 📦

It takes just a few minutes to get up and running with `rlg`.

### Requirements

`rlg` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `rlg` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
rlg = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate rlg;
use rlg::*;
```

then you can use the functions in your application code.

### Examples

`RLG` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example rlg
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
[8]: https://crates.io/crates/rlg
[9]: https://docs.rs/rlg
[10]: https://lib.rs/crates/rlg

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-rlg-1597x377.svg "RLG Banner"
[crates-badge]: https://img.shields.io/crates/v/rlg.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/rlg.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/rlg.svg?style=for-the-badge 'License'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
