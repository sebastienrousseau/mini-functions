# Mini Functions

![Mini Functions][banner]

## Highly Performant Utility And Wrapper Functions Library For Rust 🦀

![Made With Love][SH-MWL]

**[Website][0]
• [Documentation][0]
• [Report Bug][3]
• [Request Feature][3]
• [Contributing Guidelines][4]**

![divider][divider]

## Welcome to the Mini Functions library

Mini Functions defines a base layer of functionality for writing Rust
applications. It provides a set of functions that can be used in your
application code.

This table briefly describes the Mini Functions Library available for
Rust. This table provides the include file name and the function
prototype for each function.

## Requirements

`mini-functions` requires Rust **1.57.0** or later.

### Table 1 - Mini Functions Library for Rust

| Function | Include File | Function Prototype | Description |
| -------- | ------------ | ------------------ | ----------- |
| `mini_functions::date::Date::now()` | `date.rs` | `fn now() -> String` | Returns the current date and time in UTC format. |
| `mini_functions::date::Date::year()` | `date.rs` | `fn year() -> String` | Returns the current year. |
| `mini_functions::date::Date::month()` | `date.rs` | `fn month() -> String` | Returns the current month. |
| `mini_functions::date::Date::day()` | `date.rs` | `fn day() -> String` | Returns the current day. |
| `mini_functions::date::Date::hour()` | `date.rs` | `fn hour() -> String` | Returns the current hour. |
| `mini_functions::date::Date::minute()` | `date.rs` | `fn minute() -> String` | Returns the current minute. |
| `mini_functions::date::Date::second()` | `date.rs` | `fn second() -> String` | Returns the current second. |
| `mini_functions::date::Date::millisecond()` | `date.rs` | `fn millisecond() -> String` | Returns the current millisecond. |
| `mini_functions::date::Date::microsecond()` | `date.rs` | `fn microsecond() -> String` | Returns the current microsecond. |
| `mini_functions::date::Date::nanosecond()` | `date.rs` | `fn nanosecond() -> String` | Returns the current nanosecond. |
| `mini_functions::date::Date::timestamp()` | `date.rs` | `fn timestamp() -> String` | Returns the current timestamp. |
| `mini_functions::date::Date::timezone()` | `date.rs` | `fn timezone() -> String` | Returns the current timezone. |
| `mini_functions::date::Date::weekday()` | `date.rs` | `fn weekday() -> String` | Returns the current weekday. |

![divider][divider]

## 🚥 Semantic Versioning Policy

For transparency into our release cycle and in striving to maintain
backward compatibility, `Mini Functions` follows
[semantic versioning][7].

![divider][divider]

## License

The project is licensed under the terms of both the MIT license and the
Apache License (Version 2.0).

- [Apache License, Version 2.0][1]
- [MIT license][2]

![divider][divider]

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.

![divider][divider]

## 💙 Acknowledgements

A big thank you to all the awesome contributors of [Mini Functions][6]
for their help and support.

[0]: https://minifunctions.com
[1]: http://www.apache.org/licenses/LICENSE-2.0
[2]: http://opensource.org/licenses/MIT
[3]: https://github.com/sebastienrousseau/mini-functions/issues
[4]: https://raw.githubusercontent.com/sebastienrousseau/mini-functions/main/.github/CONTRIBUTING.md
[6]: https://github.com/sebastienrousseau/mini-functions/graphs/contributors
[7]: http://semver.org/

[SH-MWL]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/shields/made-with-love.svg "Made With Love"
[banner]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/banners/banner-mini-functions.svg "Mini Functions - Rust 🦀"
[divider]: https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/elements/divider.svg "divider"
