// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::md5::{Digest, MD5};

fn main() {
    // Example using MD5::hexdigest() for a string input
    let input = "Hello, world!";
    let digest = MD5::hexdigest(input);
    println!("🦀 MD5::hexdigest() for a string input:            ✅ {digest}",);
    // Expected 6cd3556deb0da54bca060b4c39479839

    // Example using MD5::hexdigest() for a byte array input
    let input =
        [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]; // "Hello, world!"
    let input_str = String::from_utf8(input.to_vec()).unwrap();
    let digest = MD5::hexdigest(&input_str);
    println!("🦀 MD5::hexdigest() for a byte array input:        ✅ {digest}",);
    // Expected 6cd3556deb0da54bca060b4c39479839

    // Example using MD5::hexdigest() for a file input
    let digest = MD5::hexdigest_file("mdg/file.txt"); // file.txt contains "Hello, world!"
    println!("🦀 MD5::hexdigest_file() for a file input:         ✅ {digest}",);
    // Expected 6cd3556deb0da54bca060b4c39479839

    // Example using MD5::update() for a byte array input
    let mut mdg = MD5::new();
    let input = [
        67, 111, 117, 99, 111, 117, 44, 32, 108, 101, 32, 109, 111,
        110, 100, 101, 33,
    ]; // "Coucou, le monde!"
    mdg.update(&input);
    let digest = mdg.finalize();
    println!("🦀 MD5::update() for a byte array input:           ✅ {digest}",);
    // Expected 47353a0e5ed2e1e0d57213a39e9bb7c4

    // Example using MD5::update() for a string input
    let mut mdg = MD5::new();
    let input = "Coucou, le monde!";
    mdg.update(input.as_bytes());
    let digest = mdg.finalize();
    println!("🦀 MD5::update() for a string input:               ✅ {digest}",);
    // Expected 47353a0e5ed2e1e0d57213a39e9bb7c4

    // Example using MD5::update() for a file input
    let mut mdg = MD5::new();
    println!(
        "🦀 MD5::new() is:                                  ✅ {}",
        mdg.finalize()
    );
    // Expected d41d8cd98f00b204e9800998ecf8427e
    mdg.update_file("mdg/update.txt"); // update.txt contains "Coucou, le monde!"
    let digest = mdg.finalize();
    println!("🦀 MD5::update_file() is:                          ✅ {digest}",);
    // Expected 7fc3e27776139278c6b8e0b6f096b4fb

    // Example using MD5::reset() for a string input
    let mut mdg = MD5::new();
    println!(
        "🦀 MD5::new() is:                                  ✅ {}",
        mdg.finalize()
    );
    // Expected d41d8cd98f00b204e9800998ecf8427e
    println!(
        "🦀 MD5::reset() for a string input:                ✅ {}",
        mdg.reset()
    );
    // Expected 00000000000000000000000000000000
}
