// Copyright © 2022-2023 Mini Functions. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
//!
//! Highly performant Games library for Rust
//!
//! [![Rust](https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/logo/logo-games.svg)](https://minifunctions.com)
//!
//! <center>
//!
//! [![GitHub](https://img.shields.io/badge/github-555555?style=for-the-badge&labelColor=000000&logo=github)](https://github.com/sebastienrousseau/mini-functions/tree/main/claims)
//! [![Rust](https://img.shields.io/badge/rust-f04041?style=for-the-badge&labelColor=c0282d&logo=rust)](https://www.rust-lang.org)
//! [![Crates.io](https://img.shields.io/crates/v/mini-functions.svg?style=for-the-badge&color=success&labelColor=27A006)](https://crates.io/crates/mini-functions)
//! [![Lib.rs](https://img.shields.io/badge/lib.rs-v0.0.8-success.svg?style=for-the-badge&color=8A48FF&labelColor=6F36E4)](https://lib.rs/crates/mini-functions)
//! [![License](https://img.shields.io/crates/l/mini-functions.svg?style=for-the-badge&color=007EC6&labelColor=03589B)](http://opensource.org/licenses/MIT)
//!
//! </center>
//!
//! ## Overview
//!
//! A highly performant game library for Rust that helps implement game logic. The library uses Mini-Functions, such as random number generation, timers, and constants.
//!
//! ## Deck
//!
//! A struct representing a deck of cards and the methods to draw cards from the deck.
//!
#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-claims.svg",
    html_favicon_url = "https://raw.githubusercontent.com/sebastienrousseau/vault/main/assets/mini-functions/icons/ico-claims.svg",
    html_root_url = "https://docs.rs/mini-functions"
)]
#![crate_name = "games"]
#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

extern crate random;
use self::random::Random;

/// Represents a deck of cards. The cards are represented as integers
/// from 0 to 51. The first 13 integers represent the 2 through
/// Ace of Spades, the next 13 integers represent the 2 through Ace
/// of Clubs, the next 13 integers represent the 2 through Ace of
/// Hearts, and the last 13 integers represent the 2 through Ace of
/// Diamonds.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Deck {
    /// The player's balance
    pub balance: u32,
    /// The player's bet
    pub bet: u32,
    /// The cards in the deck
    pub cards: Vec<usize>,
    /// The random number generator
    pub rng: Random,
}

impl Deck {
    /// Draws three cards and determines if the player has a winning hand.
    /// If the player has a winning hand, the player's balance is increased by twice the bet amount.
    pub fn determine_winnings(&mut self) {
        match (self.draw(), self.draw(), self.draw()) {
            (Some(card1), Some(card2), Some(card3)) => {
                println!("\n🦀 Your hand:");
                println!("{card1}\n{card2}\n{card3}");

                // Determine if the player has a winning hand and pay out winnings
                let winnings = if self.has_winning_hand(card1, card2, card3) {
                    self.bet * 2
                } else {
                    0
                };
                self.balance += winnings;
                if winnings > 0 {
                    println!("🎲 Congratulations! You won ${winnings}");
                } else {
                    println!("🎲 Sorry, you didn't win this time.");
                }
            }
            _ => println!("Deck is empty, please restart the game!"),
        }
    }

    /// Displays the current balance to the player
    pub fn display_balance(&self) {
        println!("💰 Your current balance is: ${}", self.balance);
    }

    /// Displays the current bet to the player
    pub fn display_bet(&self) {
        println!("💰 Your current bet is: ${}", self.bet);
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
            Some(format!("{rank} of {suit}"))
        } else {
            None
        }
    }

    /// Determines if the player has a winning hand, which is defined as three cards of the same suit.
    pub fn has_winning_hand(&self, card1: String, card2: String, card3: String) -> bool {
        let card1_split = card1.split(' ').collect::<Vec<&str>>();
        let card2_split = card2.split(' ').collect::<Vec<&str>>();
        let card3_split = card3.split(' ').collect::<Vec<&str>>();
        let card1_suit = card1_split[2].chars().next().unwrap();
        let card2_suit = card2_split[2].chars().next().unwrap();
        let card3_suit = card3_split[2].chars().next().unwrap();

        card1_suit == card2_suit && card2_suit == card3_suit
    }

    /// Creates a new deck of cards and shuffles it using the
    /// Fisher-Yates algorithm. <https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle>
    pub fn new() -> Self {
        let mut deck = Self {
            cards: (0..52).collect(),
            rng: Random::new(),
            balance: 100,
            bet: 0,
        };

        // Shuffle the deck using the Fisher-Yates algorithm
        for i in (1..52).rev() {
            let j = deck.rng.rand() as usize % (i + 1);
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
            println!("🎲 You placed a bet of ${bet}");
        } else {
            println!("🎲 You do not have enough funds to place a bet of ${bet}");
        }
    }

    /// Allows the player to play again or quit the game
    pub fn play_again_or_quit(&mut self) {
        loop {
            println!(
                "Would you like to play again or quit? (Enter 'p' to play again or 'q' to quit)"
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            match input {
                "p" => {
                    self.bet = 0;
                    self.display_balance();
                    if self.balance < 5 {
                        println!("You don't have enough funds to play again!");
                        println!("Thanks for playing!");
                        std::process::exit(0);
                    }
                    return;
                }
                "q" => {
                    println!("Thanks for playing!");
                    std::process::exit(0);
                }
                _ => println!("Invalid input, please enter 'p' to play again or 'q' to quit"),
            }
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
