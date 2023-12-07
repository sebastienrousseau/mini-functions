// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::date::DateTime;
use mini_functions::logs::{Log, LogFormat, LogLevel};

fn main() {
    let date = DateTime::new();
    let iso = date.iso_8601;
    let log_formats = vec![
        LogFormat::CLF,
        LogFormat::JSON,
        LogFormat::CEF,
        LogFormat::ELF,
        LogFormat::GELF,
        LogFormat::W3C,
    ];

    for format in log_formats {
        let log = Log::new(
            "12345678-1234-1234-1234-1234567890ab",
            &iso,
            &LogLevel::INFO,
            "SystemTrayEvent",
            "Showing main window",
            &format,
        );
        println!("🦀 Log::new():            ✅ {log}");
    }
}