// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::date::DateTime;
use mini_functions::logs::Log;

fn main() {
    let date = DateTime::new();
    let iso = date.to_string();

    let log = Log::info("SystemTrayEvent - Showing main window");
    println!("Log: [{}] session={} msg={}", iso, log.session_id, log.description);
}
