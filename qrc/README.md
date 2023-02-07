# QRC

The QR Code Library for Rust 🦀

[![Made With Love][made-with-rust]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to QRC 👋

![QRC Banner][banner]

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

The QR Code Library (QRC) is a versatile tool for generating and
manipulating QR code images in various formats.

With this library, you can easily convert your data into a QR code,
whether it be in the form of a string or a vector of bytes.

Choose from popular image formats like PNG, JPG, GIF and SVG, and even
customize the size and color of your QR code.

## Features ✨

`QRC` features a `QRCode` struct that can be constructed with a
`Vec<u8>` of data or a `String` of data that will be converted to
a `Vec<u8>`.

The QR code can be generated using the `to_qrcode`method, and specific
image formats can be generated using the`to_png`,`to_jpg`, and`to_gif`
methods.

Each of these methods takes a `width` parameter and returns an
`ImageBuffer` containing the QR code image.

The library uses the qrcode and image crates to generate the QR code
images.

## Installation 📦

It takes just a few minutes to get up and running with `qrc`.

### Requirements

`qrc` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `qrc` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
qrc = "0.0.2"
```

Add the following to your `main.rs` file:

```rust
extern crate qrc;
use qrc::*;
```

then you can use the functions in your application code.

### Examples

`QRC` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example qrc
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
[8]: https://crates.io/crates/qrc
[9]: https://docs.rs/qrc
[10]: https://lib.rs/crates/qrc

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-qrc-1597x377.svg "QRC Banner"
[crates-badge]: https://img.shields.io/crates/v/qrc.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/qrc.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.2-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/qrc.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
