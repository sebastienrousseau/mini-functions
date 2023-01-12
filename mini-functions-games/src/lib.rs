//! # Deck
//!
//! A struct representing a deck of cards and the methods to draw cards from the deck.

extern crate mini_functions_random;
use self::mini_functions_random::Random;

/// Represents a deck of cards. The cards are represented as integers
/// from 0 to 51. The first 13 integers represent the 2 through
/// Ace of Spades, the next 13 integers represent the 2 through Ace
/// of Clubs, the next 13 integers represent the 2 through Ace of
/// Hearts, and the last 13 integers represent the 2 through Ace of
/// Diamonds.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Deck {
    pub balance: u32,
    pub bet: u32,
    pub cards: Vec<usize>,
    pub rng: Random,
}

impl Deck {
    /// Draws three cards and determines if the player has a winning hand.
    /// If the player has a winning hand, the player's balance is increased by twice the bet amount.
    pub fn determine_winnings(&mut self) {
        match (self.draw(), self.draw(), self.draw()) {
            (Some(card1), Some(card2), Some(card3)) => {
                println!("\n🦀 Your hand:");
                println!("{}\n{}\n{}", card1, card2, card3);

                // Determine if the player has a winning hand and pay out winnings
                let winnings = if self.has_winning_hand(card1, card2, card3) {
                    self.bet * 2
                } else {
                    0
                };
                self.balance += winnings;
                if winnings > 0 {
                    println!("🎲 Congratulations! You won ${}", winnings);
                } else {
                    println!("🎲 Sorry, you didn't win this time.");
                }
            }
            _ => println!("Deck is empty, please restart the game!"),
        }
    }

    /// Draws a card from the top of the deck and returns it as a string.
    /// If the deck is empty, returns None.
    pub fn draw(&mut self) -> Option<String> {
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

    /// Determines if the player has a winning hand, which is defined as three cards of the same suit.
    pub fn has_winning_hand(&self, card1: String, card2: String, card3: String) -> bool {
        let card1_suit = card1.split(" ").last().unwrap();
        let card2_suit = card2.split(" ").last().unwrap();
        let card3_suit = card3.split(" ").last().unwrap();

        card1_suit.trim_end_matches(")") == card2_suit.trim_end_matches(")")
            && card2_suit.trim_end_matches(")") == card3_suit.trim_end_matches(")")
    }

    /// Creates a new deck of cards and shuffles it using the
    /// Fisher-Yates algorithm. (https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle)
    pub fn new() -> Self {
        let mut deck = Self {
            cards: (0..52).collect(),
            rng: Random::new(),
            balance: 100,
            bet: 0,
        };

        // Shuffle the deck using the Fisher-Yates algorithm
        for i in (1..52).rev() {
            let j = deck.rng.random() as usize % (i + 1);
            deck.cards.swap(i, j);
        }
        deck
    }

    /// Allows the player to place a bet. If the player does not have enough funds,
    /// an error message is displayed.
    pub fn place_bet(&mut self, bet: u32) {
        if self.balance >= bet {
            self.bet = bet;
            self.balance -= bet;
            println!("🎲 You placed a bet of ${}", bet);
        } else {
            println!("🎲 You do not have enough funds to place a bet of ${}", bet);
        }
    }
}

/// Default implementation for Deck
impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

/// Display implementation for Deck
impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Balance: ${} Bet: ${}", self.balance, self.bet)
    }
}
