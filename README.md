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

<!-- markdownlint-enable MD033 MD041 -->
## Overview

`Mini Functions` is a highly performant utility and wrapper functions library for Rust that has been carefully designed with optimization and efficiency in mind. By providing convenient wrapper functions, our library aims to provide a high-level interface for common tasks while still leveraging the performance benefits of Rust under the hood. These utility functions serve as an essential toolkit for any Rust developer, and the library's design abstractions allow for easy integration into a variety of projects and applications.

These utility functions serve as an essential toolkit for any Rust developer, and the library's design abstractions allow for easy integration into a variety of projects and applications.

## Table of Contents

- [Mini Functions](#mini-functions)
  - [Overview](#overview)
  - [Table of Contents](#table-of-contents)
  - [Features](#features)
  - [Functionality](#functionality)
  - [Getting Started](#getting-started)
    - [Requirements](#requirements)
    - [Installation](#installation)
  - [Usage](#usage)
  - [Examples](#examples)
    - [Example 1: Working with JWT Claims](#example-1-working-with-jwt-claims)
    - [Example 2: Working with Mathematical Constants](#example-2-working-with-mathematical-constants)
    - [Example 3: Working with Date and Time](#example-3-working-with-date-and-time)
    - [Example 4: Error Handling](#example-4-error-handling)
    - [Example 5: Password Hashing](#example-5-password-hashing)
    - [Example 7: Logging](#example-7-logging)
    - [Example 8: MD5 Hashing](#example-8-md5-hashing)
    - [Example 9: QR Codes](#example-9-qr-codes)
    - [Example 10: Random Number Generation](#example-10-random-number-generation)
  - [Platform support](#platform-support)
    - [Tier 1 platforms](#tier-1-platforms)
    - [Tier 2 platforms](#tier-2-platforms)
  - [Documentation](#documentation)
  - [Semantic Versioning Policy](#semantic-versioning-policy)
  - [License](#license)
  - [Contribution](#contribution)
  - [Acknowledgements](#acknowledgements)

## Features

- **Built with Rust** — A modern programming language that is well-suited for building high-performance, reliable, and secure systems.
- **High-level Utility Functions** — A collection of high-level, abstracted functions for common tasks, such as string manipulation, file manipulation, and data parsing.
- **Wrapper Functions for Easy Access** — Wrapper functions that provide a more convenient interface for accessing and using underlying Rust libraries or APIs.
- **Optimization and Performance Tools** — Tools for optimizing and improving the performance of Rust code.
- **Multi-platform Support** — Support for a variety of platforms, including desktop, mobile, and web.
- **Comprehensive Documentation and Examples** — Documentation and examples to help developers understand and use the library effectively.
- **Regular Maintenance and Updates** — Regular updates and maintenance to ensure the library stays up-to-date and reliable.

## Functionality

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

## Getting Started

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

## Usage

To use the `mini-functions` library in your project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
mini-functions = "0.0.10"
```

Add the following to your `main.rs` file:

```rust
extern crate mini_functions;
use mini_functions::*;
```

then you can use the functions in your application code.

![divider][divider]

## Examples

The `mini-functions` library comes with a set of examples that demonstrate how to use the library. You can find the examples in the `examples` directory.

To run the examples, use the following command:

```shell
cargo run --example <example-name>
```

![divider][divider]

### Example 1: Working with JWT Claims

The `mini_functions` crate provides a Claims struct for working with JWT claims.

It contains the following functions:

- Setting Claims with `set_claim` to add claims.
- Getting Claims with `get_claim` to retrieve a claim value.
- Removing Claims with `remove_claim` to remove a claim.

Here is a full example:

```rust
let mut claims = Claims::new();

claims.set_claim("iss", "https://example.com");
claims.set_claim("admin", "true");

let admin = claims.get_claim("admin").unwrap();

claims.remove_claim("admin");
```

This allows setting, retrieving, and removing JWT claims conveniently.

To run the JWT Claims example, use the following command:

```shell
cargo run --example example_claims
```

![divider][divider]

### Example 2: Working with Mathematical Constants

The `mini_functions` crate provides access to common mathematical constants through the `Constants` struct and `cmn_constants` macro.

It contains the following functions:

- **Constants::new()** - Create a new Constants instance
- **constants()** - Get the full list of constants
- **constant(name)** - Lookup a constant by name
- **cmn_constants!** - Import constants into scope

Here is a full example:

```rust
use cmn::{constants::{Constant, ConstantValue}, cmn_constants};

let c = Constants::new();
let euler = c.constant("EULER").unwrap();

cmn_constants! {
    PI = cmn::constants::PI,
}

println!("Euler's constant: {euler}"); 
println!("Pi: {PI}");
```
This allows convenient access to mathematical constants.

To run the constants example:

```shell
cargo run --example example_constants
```

![divider][divider]

### Example 3: Working with Date and Time

The `mini_functions` crate provides a `DateTime` struct for working with dates and times.

It contains the following functions:

- `DateTime::now` - Get current date/time
- `DateTime::new` - Create a DateTime with default (UTC) timezone
- `DateTime::new_with_tz` - Create a DateTime with custom timezone
- `is_valid_day` - Check if a day value is valid
- `next_day`/`previous_day` - Get next/previous day
- `from_str` - Parse a date/time string
- `relative_delta` - Apply a delta to the DateTime

Here is an example:

```rust
let now = DateTime::now();

let tomorrow = now + chrono::Duration::days(1);
let yesterday = now - chrono::Duration::days(1);

println!("Today: {now}");
println!("Tomorrow: {tomorrow}"); 
println!("Yesterday: {yesterday}");
```

This allows convenient date/time handling.

To run the date/time example:

```shell
cargo run --example example_date
```

![divider][divider]

### Example 4: Error Handling

The `mini_functions` crate provides error handling functionality through the `ErrorType` enum.

It contains the following functions:

- `ErrorType::new` - Create new error type
- `new_subtype` - Create error subtype

Here is an example:

```rust
use mini_functions::errors::common::ErrorType;

let error = ErrorType::new("illegal_argument");
let sub_error = error.new_subtype("invalid_value");

println!("Main error: {error:?}");
println!("Sub-error: {sub_error:?}");
```

This allows simple error handling with custom types and subtypes.

To run the errors example:

```rust
cargo run --example example_errors
```

![divider][divider]

### Example 5: Password Hashing

The `mini_functions` crate provides password hashing and verification functions through the `Hash` struct.

It contains the following functions:

- `Hash::new_{algo}` - Generate a hash for a password 
- `set_password` - Update password for a hash 
- `verify` - Verify a password against a hash
- `to_string` - Convert hash to a string

Here is an example:

```rust
use mini_functions::hash::Hash;

let hash = Hash::new_argon2i("mypassword");
let is_valid = hash.verify("mypassword");

let updated_hash = hash.set_password("newpassword");
let new_is_valid = updated_hash.verify("newpassword");
```

This allows generating and verifying password hashes conveniently.

To run the password hashing example:

```rust
cargo run --example example_hash
```

![divider][divider]

### Example 7: Logging

The `mini_functions` crate provides application logging functionality through the `Log` struct.

It contains functions like:

- `Log::new` - Create a new log entry
- `LogFormat` - Supported log formats

Here is an example of logging events with different formats:

```rust
use mini_functions::logs::{Log, LogFormat, LogLevel};

let log_json = Log::new(
    "message-id", 
    "2023-01-01T12:00:00Z",
    LogLevel::Info,
    "AppEvent",
    "User logged in",
    LogFormat::JSON
);

let log_clf = Log::new(
    "message-id",
    "2023-01-01T12:00:00Z",
    LogLevel::Info,
    "AuthEvent",
    "User login successful",
    LogFormat::CLF
);
```

This allows flexible logging in various text and JSON formats.

To run the logging example:

```rust
cargo run --example example_logs
```

![divider][divider]

### Example 8: MD5 Hashing  

The `mini_functions` crate provides MD5 hash generation functionality through the `MD5` struct.

It contains functions like:  

- `MD5::hexdigest` - Generate MD5 hash for input 
- `MD5::new` - Create MD5 hasher instance  
- `update` - Update hasher with new input
- `finalize` - Obtain final hash  

Here is an example of hashing different input sources:

```rust
use mini_functions::md5::MD5;

let digest = MD5::hexdigest("input string");

let mut hasher = MD5::new();
hasher.update(&[1, 2, 3]);
let hash = hasher.finalize();
```

This allows flexible hashing of strings, byte arrays, files.

To run the MD5 example:

```rust
cargo run --example example_md5
```

![divider][divider]

### Example 9: QR Codes

The `mini_functions` crate provides QR code generation and manipulation functionality through the `QRCode` struct.

It contains functions like:

- `QRCode::from_string` - Generate QR code from text
- `to_png` - Convert to PNG image
- `colorize` - Colorize the QR code
- `resize` - Resize image 

And macros like:

- `qr_code_to!` - QR code generation macro

Here is an example:

```rust
use mini_functions::qr;

let qr_code = qr::QRCode::from_string("https://example.com");
let img = qr_code.to_png(512);

qr::save_png(&img, "qr.png");
```

This allows convenient QR code creation, manipulation and saving.

To run the QR code example:

```rust
cargo run --example example_qr
```

![divider][divider]

### Example 10: Random Number Generation

The `mini_functions` crate provides random number generation functionality through the `Random` struct and associated functions.

It contains functions like:

- `Random::new` - Create random number generator
- `bool` - Random boolean
- `int` - Random integer
- `float` - Random float
- `bytes` - Random byte vector

And macros like:

- `rand_int!` - Random integer in range
- `rand_bool!` - Random boolean with probability

Here is an example:

```rust
use mini_functions::random::{Random, rand_int};

let mut rng = Random::new();

let rand_num = rand_int!(rng, 0, 10);
let rand_bool = rand_bool!(rng, 0.5);
```

This allows convenient random number generation.

To run the random example:

```rust
cargo run --example example_random
```

![divider][divider]

## Platform support

`mini-functions` is supported and tested on the following platforms:

### Tier 1 platforms

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

### Tier 2 platforms

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

## Documentation

**Info:** Please check out our [website ⧉][00] for more information. You can find our documentation on [docs.rs ⧉][08], [lib.rs ⧉][09] and [crates.io ⧉][07].

## Semantic Versioning Policy

For transparency into our release cycle and in striving to maintain backward compatibility, `mini-functions` follows [semantic versioning ⧉][06].

![divider][divider]

## License

The project is licensed under the terms of Apache License, Version 2.0 and the MIT license.

![divider][divider]

## Contribution

We welcome all people who want to contribute. Please see the [contributing instructions ⧉][04] for more information.

Contributions in any form (issues, pull requests, etc.) to this project must adhere to the [Rust's Code of Conduct ⧉][23].

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

![divider][divider]

## Acknowledgements

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
