extern crate mini_functions_random;
use self::mini_functions_random::Random;

fn main() {
    // Create a new random boolean
    let bool: bool = Random::bool();
    println!("🦀 Random::bool():        ✅ {}", bool);

    // Create a new random number generator
    let mut rng = Random::new();
    println!("🦀 Random::new():         ✅ {}", rng);

    let default = Random::default();
    println!("🦀 Random::default():     ✅ {}", default);

    let random = rng.random();
    println!("🦀 Random::random():      ✅ {}", random);

    let pseudo = rng.pseudo();
    println!("🦀 Random::pseudo():      ✅ {}", pseudo);

    let bytes = Random::bytes(10);
    println!("🦀 Random::bytes():       ✅ {:?}", bytes);

    let float = rng.random() as f32 / 0x7FFF as f32;
    println!("🦀 Random::float():       ✅ {}", float);

    let int = rng.random() as usize;
    println!("🦀 Random::int():         ✅ {}", int);
}
