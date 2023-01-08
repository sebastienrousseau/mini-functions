use mini_functions::uuid::uuid::UUID; // Import the UUID struct from the mini_functions crate

fn main() {
    let version_3 = 3; // Define a variable for UUID version 3
    let version_4 = 4; // Define a variable for UUID version 4
    let version_5 = 5; // Define a variable for UUID version 5

    let uuid_v3 = UUID::new(version_3, &uuid::Uuid::new_v4(), "test"); // Create a new UUID of version 3 using the UUID::new() function
    println!("🦀 fn new(version_3):     ✅ {}", uuid_v3); // Print the string representation of the UUID

    let uuid_v4 = UUID::new(version_4, &uuid::Uuid::new_v4(), "test"); // Create a new UUID of version 4 using the UUID::new() function
    println!("🦀 fn new(version_4):     ✅ {}", uuid_v4); // Print the string representation of the UUID

    let uuid_v5 = UUID::new(version_5, &uuid::Uuid::new_v4(), "test"); // Create a new UUID of version 5 using the UUID::new() function
    println!("🦀 fn new(version_5):     ✅ {}", uuid_v5); // Print the string representation of the UUID

    let new_v3 = UUID::uuid_v3(&uuid::Uuid::new_v4(), "test"); // Create a new UUID of version 3 using the UUID::new_v3() function
    println!("🦀 fn new_v3():           ✅ {}", new_v3); // Print the string representation of the UUID

    let new_v4 = UUID::uuid_v4(); // Create a new UUID of version 4 using the UUID::new_v4() function
    println!("🦀 fn new_v4():           ✅ {}", new_v4); // Print the string representation of the UUID

    let new_v5 = UUID::uuid_v5(&uuid::Uuid::new_v4(), "test"); // Create a new UUID of version 5 using the UUID::new_v5() function
    println!("🦀 fn new_v5():           ✅ {}", new_v5); // Print the string representation of the UUID
}
