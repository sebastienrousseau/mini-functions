use cmn::constants::*;
use psph::Password;

fn main() {
    let password = Password::new(4, "-", SPECIAL_CHARS.to_vec());
    println!(
        "🦀 Password::default():           ✅ {}",
        Password::default()
    );
    println!("🦀 Password::new():               ✅ {password}");
    println!(
        "🦀 Password::passphrase():        ✅ {}",
        password.passphrase()
    );
    let mut password = Password::new(4, "-", SPECIAL_CHARS.to_vec());
    println!("🦀 Password::set_passphrase");
    println!(
        "    🔓 Original passphrase:       ✅ {}",
        password.passphrase()
    );
    password.set_passphrase("M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3");
    println!(
        "    🔐 Updated passphrase:        ✅ {}",
        password.passphrase()
    );
    println!("🦀 Password::len():               ✅ {}", password.len());
    println!(
        "🦀 Password::is_empty():          ✅ {}",
        password.is_empty()
    );
    println!("🦀 Password::hash():              ✅ {}", password.hash());
    println!(
        "🦀 Password::password_length():   ✅ {}",
        password.password_length()
    );
    println!(
        "🦀 Password::hash_length():       ✅ {}",
        password.hash_length()
    );

    let entropy = Password::entropy(&password) as u64;

    match entropy {
        x if x < 40 => println!("🦀 Password::entropy():  ❌ {entropy} bits (Poor)"),
        x if (40..55).contains(&x) => {
            println!("🦀 Password::entropy():  ❌ {entropy} bits (Weak)")
        }
        x if (56..70).contains(&x) => {
            println!("🦀 Password::entropy():           ⚠️ {entropy} bits (Reasonable)",)
        }
        x if (71..80).contains(&x) => {
            println!("🦀 Password::entropy():  ⚠️ {entropy} bits (Strong)")
        }
        _ => println!("🦀 Password::entropy():           ✅ {entropy} bits (Excellent)",),
    }
}
