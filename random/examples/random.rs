extern crate random;
use self::random::Random;

fn main() {
    // Create a new random boolean
    let bool: bool = Random::bool(&mut Random::new());
    println!("🦀 Random::bool():        ✅ {}", bool);

    // Create a new random number generator
    let mut rng = Random::new();
    println!("🦀 Random::new():         ✅ {}", rng);

    let default = Random::default();
    println!("🦀 Random::default():     ✅ {}", default);

    let random = rng.rand();
    println!("🦀 Random::random():      ✅ {}", random);

    let pseudo = rng.pseudo();
    println!("🦀 Random::pseudo():      ✅ {}", pseudo);

    let bytes = Random::bytes(&mut rng, 1000);
    println!("🦀 Random::bytes():       ✅ {:?}", bytes);

    let float = rng.rand() as f32 / 0x7FFF as f32;
    println!("🦀 Random::float():       ✅ {}", float);

    let int = rng.rand() as usize;
    println!("🦀 Random::int():         ✅ {}", int);
}
