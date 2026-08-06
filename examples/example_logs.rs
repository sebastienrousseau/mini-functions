// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Demonstrates the `logs` module, a re-export of `rlg`.
//!
//! `rlg` 0.0.11 replaced the positional `Log::new(session, time, level,
//! component, description, format)` constructor with a fluent builder:
//! `Log::build(level, description)` seeds the entry (assigning a
//! `session_id` and a wall-clock `time`), and `.time()` / `.component()`
//! override individual fields. `format` stays a plain public field, so
//! it is set through struct update syntax.

use mini_functions::date::DateTime;
use mini_functions::logs::{Log, LogFormat, LogLevel};

/// Renders one entry per supported format, propagating any failure.
fn run() -> Result<(), Box<dyn std::error::Error>> {
    // `dtt` dropped the `iso_8601` field; RFC 3339 is the ISO 8601
    // profile it used to hold.
    let iso = DateTime::new().format_rfc3339()?;

    let log_formats = [
        LogFormat::CLF,
        LogFormat::JSON,
        LogFormat::CEF,
        LogFormat::ELF,
        LogFormat::GELF,
        LogFormat::W3C,
    ];

    for format in log_formats {
        let log = Log {
            format,
            ..Log::build(LogLevel::INFO, "Showing main window")
                .time(&iso)
                .component("SystemTrayEvent")
        };
        println!("🦀 Log::build():          ✅ {log}");
    }

    Ok(())
}

/// Entry point for the example.
fn main() {
    if let Err(e) = run() {
        eprintln!("example_logs failed: {e}");
        std::process::exit(1);
    }
}
