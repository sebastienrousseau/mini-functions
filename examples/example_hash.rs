// Copyright © 2023 Mini Functions library. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use mini_functions::hash::models::hash::Hash;

fn create_and_verify_hash() {
    let hash_argon2id = Hash::new_argon2id("password", "salt1234".into()).unwrap();
    let hash_bcrypt = Hash::new_bcrypt("password", 10).unwrap();
    let hash_scrypt = Hash::new_scrypt("password", "salt1234".into()).unwrap();

    println!("Argon2id Hash: {:?}", hash_argon2id.hash());
    println!("BCrypt Hash: {:?}", hash_bcrypt.hash());
    println!("Scrypt Hash: {:?}", hash_scrypt.hash());
}

fn main() {
    create_and_verify_hash();
}
