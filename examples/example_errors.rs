// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::errors::common::ErrorType;

fn main() {
    let error_type = ErrorType::new("illegal_argument");
    let error_type_new_subtype = error_type.new_subtype("subtype");

    println!("🦀 Error::error_type_new():             ✅ {error_type:?}\n",);
    println!("🦀 Error::error_type_new_subtype():        ✅ {error_type_new_subtype:?}\n",);
}


