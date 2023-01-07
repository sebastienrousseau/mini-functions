use mini_functions::random::Random;

// Defining an example for a simple `A Three Card Draw Poker Game` that
// shuffles a deck of cards and allows the user to draw cards from the
// top of the deck, displaying the card name and suit as a string.

// A struct that represents a deck of playing cards
struct Deck {
    // A vector of cards
    cards: Vec<usize>,
    // A random number generator
    rng: Random,
}

// Implementation of the `Deck` struct
impl Deck {
    // Creates a new `Deck` struct with a shuffled deck of cards
    fn new() -> Self {
        let mut deck = Self {
            cards: (0..52).collect(),
            rng: Random::new(),
        };

        // Shuffle the deck using the Fisher-Yates algorithm
        for i in (1..52).rev() {
            let j = deck.rng.random() as usize % (i + 1);
            deck.cards.swap(i, j);
        }
        deck
    }

    // Draws a card from the top of the deck
    fn draw(&mut self) -> Option<String> {
        let card = self.cards.pop();
        if let Some(card) = card {
            let suits = ["Spades (♠)", "Clubs (♣)", "Hearts (♥)", "Diamonds (♦)"];
            let ranks = [
                "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
            ];
            let suit = suits[card / 13];
            let rank = ranks[card % 13];
            Some(format!("{} of {}", rank, suit))
        } else {
            None
        }
    }
}

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

    // Create a new deck of cards and draw three cards
    let mut deck = Deck::new();

    // Draw three cards from the top of the deck and print them to the console
    let card1 = deck.draw().unwrap();
    let card2 = deck.draw().unwrap();
    let card3 = deck.draw().unwrap();

    // Print the cards to the console
    println!(
        "\n🦀 Let's play a mini game of `Three Card Draw Poker` to demonstrate the random number generator!\n"
    );
    println!("🎲 Deck::draw():          ✅ {}", card1);
    println!("🎲 Deck::draw():          ✅ {}", card2);
    println!("🎲 Deck::draw():          ✅ {}", card3);
}
