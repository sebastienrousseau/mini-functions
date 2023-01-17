extern crate md5;
use md5::{Digest, MD5};

fn main() {
    // Example using MD5::hexdigest() for a string input
    let input = "Hello, world!";
    let digest = MD5::hexdigest(input);
    println!(
        "🦀 MD5::hexdigest() for a string input:            ✅ {}",
        digest
    );
    // Expected 6cd3556deb0da54bca060b4c39479839

    // Example using MD5::hexdigest() for a byte array input
    let input = [72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]; // "Hello, world!"
    let input_str = String::from_utf8(input.to_vec()).unwrap();
    let digest = MD5::hexdigest(&input_str);
    println!(
        "🦀 MD5::hexdigest() for a byte array input:        ✅ {}",
        digest
    );
    // Expected 6cd3556deb0da54bca060b4c39479839

    // Example using MD5::hexdigest() for a file input
    let digest = MD5::hexdigest_file("file.txt"); // file.txt contains "Hello, world!"
    println!(
        "🦀 MD5::hexdigest_file() for a file input:         ✅ {}",
        digest
    );
    // Expected 6cd3556deb0da54bca060b4c39479839

    // Example using MD5::update() for a byte array input
    let mut md5 = MD5::new();
    let input = [
        67, 111, 117, 99, 111, 117, 44, 32, 108, 101, 32, 109, 111, 110, 100, 101, 33,
    ]; // "Coucou, le monde!"
    md5.update(&input);
    let digest = md5.finalize();
    println!(
        "🦀 MD5::update() for a byte array input:           ✅ {}",
        digest
    );
    // Expected 47353a0e5ed2e1e0d57213a39e9bb7c4

    // Example using MD5::update() for a string input
    let mut md5 = MD5::new();
    let input = "Coucou, le monde!";
    md5.update(input.as_bytes());
    let digest = md5.finalize();
    println!(
        "🦀 MD5::update() for a string input:               ✅ {}",
        digest
    );
    // Expected 47353a0e5ed2e1e0d57213a39e9bb7c4

    // Example using MD5::update() for a file input
    let mut md5 = MD5::new();
    println!(
        "🦀 MD5::new() is:                                  ✅ {}",
        md5.finalize()
    );
    // Expected d41d8cd98f00b204e9800998ecf8427e
    md5.update_file("update.txt"); // update.txt contains "Coucou, le monde!"
    let digest = md5.finalize();
    println!(
        "🦀 MD5::update_file() is:                          ✅ {}",
        digest
    );
    // Expected 7fc3e27776139278c6b8e0b6f096b4fb

    // Example using MD5::reset() for a string input
    let mut md5 = MD5::new();
    println!(
        "🦀 MD5::new() is:                                  ✅ {}",
        md5.finalize()
    );
    // Expected d41d8cd98f00b204e9800998ecf8427e
    println!(
        "🦀 MD5::reset() for a string input:                ✅ {}",
        md5.reset()
    );
    // Expected 00000000000000000000000000000000

    // Example using MD5::digest() for a larger input
    // let input = [0; 1024];
    // let digest = MD5::digest(&input);
    // println!("🦀 MD5::digest() for a larger input: ✅ {:?}", digest);

    // Example using MD5::default() to create a new MD5 object
    // let gen = MD5::default();
    // println!(
    //     "🦀 MD5::default() to create a new MD5 object: ✅ {:?}",
    //     gen.to_string()
    // );

    // Example using MD5::finalize() to finalize the MD5 object
    // let finalize = MD5::finalize(gen, 13);
    // println!(
    //     "🦀 MD5::finalize() to finalize the MD5 object: ✅ {:?}",
    //     finalize
    // );

    // Example using MD5::finalize() to retrieve the digest
    // println!(
    //     "🦀 MD5::digest() to retrieve the digest: ✅ {:?}",
    //     MD5::digest(&finalize)
    // );

    // Example using MD5::new_with_string() for a string input
    // let the_digest = b"123456";
    // let the_result = MD5::digest(the_digest);
    // println!(
    //     "🦀 MD5::the_digest(): ✅ {:?}",
    //     MD5::to_hex_string(&the_result)
    // );
}
