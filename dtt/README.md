# DTT

A Rust library for parsing, validating, manipulating, and formatting dates and times

[![Made With Love][mwl]][6]
[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

## Welcome to DTT 👋

![DTT Banner][banner]

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

The DateTime (DTT) library is a comprehensive and flexible tool that
enables developers to manage dates and times with ease.

It offers a range of functions and data structures that allow you to
perform various date and time operations with ease, such as determining
the day of the month, hour of the day, working with ISO 8601 date and
time formats, and many others.

The library supports the creation of new DateTime objects with either
UTC or custom timezone specifications, ensuring that you always have
accurate and relevant date and time information. Additionally, it
provides a mechanism to validate input dates and times, ensuring that
you always have accurate information to work with.

## Features ✨

The library `DateTime` provides date and time types and methods to make
it easier to manipulate dates and times. It uses the serde library to
derive the Deserialize and Serialize traits to convert the `DateTime`
struct to and from various data formats. It also uses the time and regex
crates to deal with time conversions and regular expressions
respectively.

The `DateTime` struct includes fields such as `day`, `hour`, `iso_8601`,
`iso_week`, `microsecond`, `minute`, `month`, `now`, `offset`,
`ordinal`, `second`, `time`, `tz`, `weekday`, and `year`, each of which
represent different aspects of a date and time.

The `DateTime` struct has two methods to create instances: `new` and
`new_with_tz`. `new` creates a new `DateTime` object with UTC timezone,
and `new_with_tz` creates a new `DateTime` object with a custom
timezone.

It also includes a method `is_valid_day` which checks if the input
string represents a valid day of the week. It also includes a method
`is_valid_month` which checks if the input string represents a valid
month of the year.

## Installation 📦

It takes just a few minutes to get up and running with `dtt`.

### Requirements

`dtt` requires Rust **1.67.0** or later.

### Documentation

> ℹ️ **Info:** Please check out our [website][0] for more information
and find our documentation on [docs.rs][9], [lib.rs][10] and
[crates.io][8].

## Usage 📖

To use `dtt` in your project, add the following to your
`Cargo.toml` file:

```toml
[dependencies]
dtt = "0.0.1"
```

Add the following to your `main.rs` file:

```rust
extern crate dtt;
use dtt::*;
```

then you can use the functions in your application code.

### Examples

`DTT` comes with a set of examples that you can use to get started. The
examples are located in the `examples` directory of the project. To run
the examples, clone the repository and run the following command in your
terminal from the project root directory.

```shell
cargo run --example dtt
```

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `DTT` follows [semantic versioning][7].

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
[8]: https://crates.io/crates/dtt
[9]: https://docs.rs/dtt
[10]: https://lib.rs/crates/dtt

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/banners/banner-dtt-1597x377.svg "DTT Banner"
[crates-badge]: https://img.shields.io/crates/v/dtt.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/dtt.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-v0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/dtt.svg?style=for-the-badge 'License'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
