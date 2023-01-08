use mini_functions::hash::hash::Hash;

fn main() {
    let mut hash = Hash::new();
    let mut default = Hash::default();
    let is_valid = hash.verify(
        "7f2611ba158b6dcea4a69c229c303358c5e04493abeadee106a4bfa464d55787",
        "password",
    );
    hash.set_password("password");
    default.set_password("password");
    hash.set_hash("1.61803398874989484820");
    default.set_hash("1.61803398874989484820");

    println!("🦀 Hash::new():                ✅ {}", hash);
    println!("🦀 Hash::default():            ✅ {}", default);
    println!("🦀 Hash::generate_hash():      ✅ {}", hash.generate_hash());
    if is_valid {
        println!(
            "🦀 Hash::verify():             ✅ {}",
            hash.verify(hash.hash(), "password")
        );
    } else {
        println!("🦀 Hash::verify() :            ❌ The password or hash is invalid.");
    }
    println!(
        "🦀 Hash::set_password():       ✅ {}",
        hash.password() == "password"
    );
    println!(
        "🦀 Hash::set_hash():           ✅ {}",
        hash.hash() == "1.61803398874989484820"
    );
    println!(
        "🦀 Hash::password():           ✅ {}",
        hash.password() == "password"
    );
    println!(
        "🦀 Hash::password_length():    ✅ {}",
        hash.password_length() == 8
    );
    println!(
        "🦀 Hash::hash():               ✅ {}",
        hash.hash() == "1.61803398874989484820"
    );
    println!(
        "🦀 Hash::hash_length():        ✅ {}",
        hash.hash_length() == 22
    );
    println!("🦀 Hash::entropy():            ✅ {}", hash.entropy() > 0.0);
}
