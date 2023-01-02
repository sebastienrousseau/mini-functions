use mini_functions::uuid::UUID;

fn main() {
    let uuid_v3: String = UUID::uuid_v3(
        uuid::Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
        b"Mini Functions Rust Library",
    );
    println!("✅ UUID v3: {}", uuid_v3);

    let uuid_v4: String = UUID::uuid_v4();
    println!("✅ UUID v4: {}", uuid_v4);

    let uuid_v5: String = UUID::uuid_v5(
        uuid::Uuid::parse_str("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap(),
        b"Mini Functions Rust Library",
    );
    println!("✅ UUID v5: {}", uuid_v5);
}
