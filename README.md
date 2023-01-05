# Mini Functions 🦀

[![Made With Love][mwl]][6]

## Highly performant utility and wrapper functions library for Rust 🚀

![Mini Functions][banner]

[![Crates.io][crates-badge]][8]
[![Lib.rs][libs-badge]][10]
[![Docs.rs][docs-badge]][9]
[![License][license-badge]][2]

![divider][divider]

**[Website][0]
• [Documentation][0]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

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

## Requirements 📋

`mini-functions` requires Rust **1.57.0** or later.

## Installation 📦

It takes just a few minutes to get up and running with `mini-functions`.
Check out our [documentation][0] for more information.

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
mini-functions = "0.0.5"
```

## Usage 📖

Add the following to your `main.rs` file:

```rust
use mini_functions::*;
```

then you can use the functions in your application code.

## The Functions library 📚

`Mini Functions` is a library of functions for Rust that provides a
collection of tools for working with various aspects of a Rust
application.

The functions in `Mini Functions` are optimized for speed and
efficiency, making them useful for a variety of applications, including
those that require fast performance or that need to handle large amounts
of data.

The `mini-functions` library consists of the following functions:

- The **[Date and time functions](#date-and-time-functions)** are used
  to retrieve and manipulate information about dates and times.
- The **[Log functions](#log-functions)** are used to log messages to
  the console.
- The **[QRCode functions](#qrcode-functions)** are used to generate
  QRCode images and data.
- The **[Random number functions](#random-number-functions)** are used
  to generate random numbers in a variety of sizes and formats.
- The **[UUID functions](#uuid-functions)** are used to generate UUIDs
  (Universally Unique Identifiers).

The following tables provide a brief description of each function in the
`mini-functions` library.

### Date and time functions

| Function | Include File | Function Prototype | Description |
| -------- | ------------ | ------------------ | ----------- |
| `Date::date()` | `date.rs` | `fn date()` | Returns the current date in UTC format. |
| `Date::day()` | `date.rs` | `fn day()` | Returns the current day. |
| `Date::hour()` | `date.rs` | `fn hour()` | Returns the current hour. |
| `Date::iso_8601()` | `date.rs` | `fn iso_8601()` | Returns the current date and time in ISO 8601 format. |
| `Date::microsecond()` | `date.rs` | `fn microsecond()` | Returns the current microsecond. |
| `Date::millisecond()` | `date.rs` | `fn millisecond()` | Returns the current millisecond. |
| `Date::minute()` | `date.rs` | `fn minute()` | Returns the current minute. |
| `Date::month()` | `date.rs` | `fn month()` | Returns the current month. |
| `Date::nanosecond()` | `date.rs` | `fn nanosecond()` | Returns the current nanosecond. |
| `Date::now_utc()` | `date.rs` | `fn now_utc()` | Returns the current date and time in UTC format. |
| `Date::second()` | `date.rs` | `fn second()` | Returns the current second. |
| `Date::timestamp()` | `date.rs` | `fn timestamp()` | Returns the current timestamp. |
| `Date::weekday()` | `date.rs` | `fn weekday()` | Returns the current weekday. |
| `Date::year()` | `date.rs` | `fn year()` | Returns the current year. |

### Log functions

| Function | Include File | Function Prototype | Description |
| -------- | ------------ | ------------------ | ----------- |
| `Log::log()` | `log.rs` | `fn log()` | Logs a message to the console.|
| `Log::new()` | `log.rs` | `fn new()` | Creates a new log instance. |

### QRCode functions

| Function | Include File | Function Prototype | Description |
| -------- | ------------ | ------------------ | ----------- |
| `QRCode::colorize()` | `qrcode.rs` | `fn colorize()` | Colorizes the QRCode instance. |
| `QRCode::from_bytes()` | `qrcode.rs` | `fn from_bytes()` | Creates a new QRCode instance from a byte array. |
| `QRCode::from_string()` | `qrcode.rs` | `fn from_string()` | Creates a new QRCode instance from a string. |
| `QRCode::new()` | `qrcode.rs` | `fn new()` | Creates a new QRCode instance. |
| `QRCode::resize()` | `qrcode.rs` | `fn resize()` | Resizes the QRCode instance. |
| `QRCode::to_png()` | `qrcode.rs` | `fn to_png()` | Converts the QRCode instance to a PNG image. |
| `QRCode::to_qrcode()` | `qrcode.rs` | `fn to_qrcode()` | Converts the QRCode instance to a QRCode image. |
| `QRCode::to_svg()` | `qrcode.rs` | `fn to_svg()` | Converts the QRCode instance to a SVG image. |

### Random number functions

| Function | Include File | Function Prototype | Description |
| -------- | ------------ | ------------------ | ----------- |
| `Random::bytes()` | `random.rs` | `fn bytes()` | Generates a vector of random bytes of a given length. |
| `Random::default()` | `random.rs` | `fn default()` | Creates a new `Random` struct with a default seed. |
| `Random::float()` | `random.rs` | `fn float()` | Generates a random floating point number between 0 and 1. |
| `Random::int()` | `random.rs` | `fn int()` | Generates a random integer between a minimum and maximum value. |
| `Random::new()` | `random.rs` | `fn new()` | Creates a new `Random` struct with a seed based on the current system time. |
| `Random::pseudo()` | `random.rs` | `fn pseudo()` | Generates a pseudo-random number by XORing the last 31 random numbers together. |
| `Random::random()` | `random.rs` | `fn random()` | Generates a random number using the linear congruential generator algorithm. The multiplier for the algorithm is the golden ratio. |

### UUID functions

| Function | Include File | Function Prototype | Description |
| -------- | ------------ | ------------------ | ----------- |
| `UUID::new()` | `uuid.rs` | `fn new()` | Creates a new UUID instance based on the version specified. (v3, v4, v5) |
| `UUID::uuid_v3()` | `uuid.rs` | `fn uuid_v3()` | Creates a new UUID v3 instance. |
| `UUID::uuid_v4()` | `uuid.rs` | `fn uuid_v4()` | Creates a new UUID v4 instance. |
| `UUID::uuid_v5()` | `uuid.rs` | `fn uuid_v5()` | Creates a new UUID v5 instance. |

![divider][divider]

## Semantic Versioning Policy 🚥

For transparency into our release cycle and in striving to maintain
backward compatibility, `Mini Functions` follows
[semantic versioning][7].

![divider][divider]

## License 📝

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

![divider][divider]

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

[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/banners/banner-mini-functions.svg "Mini Functions - Rust 🦀"
[crates-badge]: https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge 'Crates.io'
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
[docs-badge]: https://img.shields.io/docsrs/mini-functions.svg?style=for-the-badge 'Docs.rs'
[libs-badge]: https://img.shields.io/badge/lib.rs-0.0.1-orange.svg?style=for-the-badge 'Lib.rs'
[license-badge]: https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge 'License'
[mwl]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
