# CCLM

A Rust library for accessing and manipulating claims of a JSON Web Token (JWT)

[![Made With Love][made-with-rust]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to CCLM 👋

![CCLM Banner][banner]

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

The Claims library holds JSON Web Token (JWT) claims. It provides an
RFC7519 compliant implementation of JSON Web Tokens (JWT) and JSON
Web Signature (JWS) for Rust.

The [**`Claims`**](./struct.Claims.html) type is provided to hold
the claims of a JWT. The claims are stored in a `HashMap` and can be
accessed using the `get_claim`, `set_claim`, `remove_claim`, and
`has_claim` methods.

## Features ✨

The following table lists the optional reserved claims that are
supported:

| Claim | Description |
| --- | --- |
| `aud` (Audience) | Identifies the recipients that the JWT is intended for. |
| `custom` (Custom) | Custom claims are used to share information between parties that agree on using them and are neither registered or public claims. |
| `did` (Decentralized Identifier) | A string value that uniquely identifies a subject. |
| `exp` (Expiration Time) | Identifies the expiration time on or after which the JWT MUST NOT be accepted for processing. |
| `iat` (Issued At) | Identifies the time at which the JWT was issued. |
| `iss` (Issuer) | Identifies the principal that issued the JWT. |
| `jti` (JWT ID) | Provides a unique identifier for the JWT. |
| `nbf` (Not Before) | Identifies the time before which the JWT MUST NOT be accepted for processing. |
| `sub` (Subject) | Identifies the principal that is the subject of the JWT. |
| `vc` (Verifiable Credential) | A Credential that is tamper-evident and has authorship that can be cryptographically verified. |
| `vp` (Verifiable Presentation) | A Presentation that is tamper-evident and has authorship that can be cryptographically verified. |

## Installation 📦

It takes just a few minutes to get up and running with `cclm`.

### Requirements

`cclm` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `cclm` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
cclm = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate cclm;
use cclm::*;
```

then you can use the functions in your application code.

### Examples

`CCLM` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example cclm
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
lot of useful suggestions on how to improve this project. A special thank you goes to the
[Rust Reddit](https://www.reddit.com/r/rust/) community for providing a
lot of useful suggestions on how to improve this project.

[0]: https://minifunctions.com
[1]: http://www.apache.org/licenses/LICENSE-2.0
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/mini-functions/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/mini-functions/main/.github/CONTRIBUTING.md
[6]: https://github.com/sebastienrousseau/mini-functions/graphs/contributors
[7]: http://semver.org/
[8]: https://crates.io/crates/cclm
[9]: https://docs.rs/cclm
[10]: https://lib.rs/crates/cclm

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-cclm-1597x377.svg "CCLM Banner"
[crates-badge]: https://img.shields.io/crates/v/cclm.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/cclm.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/cclm.svg?style=for-the-badge 'License'
[made-with-rust]: https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust 'Made With Rust'
