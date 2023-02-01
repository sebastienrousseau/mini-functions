#![warn(missing_docs)]
#![forbid(unsafe_code)]
extern crate common;
pub use common::constants::*;
pub use common::words::WORD_LIST;

extern crate hash;
pub use hash::Hash;

extern crate random;
use crate::random::Random;

// use convert_case::{Case, Casing};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::f64;

/// A random password / passphrase generator. The generated password
/// is a string of three words separated by hyphens. Each word is
/// between 6 and 8 characters long. The first character of each word
/// is capitalized.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Password {
    /// The generated passphrase.
    passphrase: String,
    /// The special characters to use for replacing letters in words.
    special_chars: Vec<char>,
    /// The separator to use between words.
    separator: String,
}

impl Password {
    /// Calculates the entropy of a password based on its length, the
    /// number of unique characters used in the password and the number
    /// of bits of the hash generated from the password.
    ///
    /// # Arguments
    ///
    /// * `&self` - An immutable reference to the password.
    ///
    /// # Returns
    ///
    /// * `f64` - The calculated entropy of the password.
    ///
    /// # Entropy Ranges
    ///
    /// The following ranges can give you an idea of how the entropy is
    /// considered:
    ///
    /// - Poor: less than 40 bits
    /// - Weak: 40-55 bits
    /// - Reasonable: 56-70 bits
    /// - Strong: 71-80 bits
    /// - Excellent: 81 bits and above
    ///
    /// Keep in mind that these values are just rough estimates and the
    /// actual entropy of a password depends on the distribution of
    /// characters used in the password and the number of unique
    /// characters in it, and not just its length.
    ///
    pub fn entropy(&self) -> f64 {
        let l = self.len() as f64;
        l * (94.0_f64.log2()).round()
    }
    /// Returns the hash of the generated passphrase.
    pub fn hash(&self) -> String {
        let mut hash = Hash::new();
        hash.set_password(&format!(
            "{}{}",
            self.passphrase,
            self.special_chars.iter().collect::<String>()
        ));
        let hash_value = hash.hash();
        hash_value.to_string()
    }
    /// Returns the hash length.
    pub fn hash_length(&self) -> usize {
        self.hash().len()
    }
    /// Returns true if the generated passphrase is empty.
    /// Returns false if the generated passphrase is not empty.
    pub fn is_empty(&self) -> bool {
        self.passphrase.is_empty()
    }
    /// Returns the length of the generated passphrase.
    pub fn len(&self) -> usize {
        self.passphrase.len()
    }
    /// Returns the generated passphrase.
    pub fn new(len: u8, separator: &str, special_chars: Vec<char>) -> Self {
        // Setup a random number generator.
        let mut rng = Random::default();

        // Create a new vector to store the words in the passphrase.
        let mut words: Vec<String> = Vec::new();

        // Convert the special characters to a vector of chars.
        let ascii: Vec<char> = SPECIAL_CHARS.to_vec();

        // Create a new HashSet to store the generated words.
        let mut word_set = HashSet::new();

        // Create a HashMap for storing seen characters for each word
        let mut seen_chars = HashMap::new();

        // Generate `len` random words from the word list.
        while words.len() < len.into() {
            // Choose a random word from the list.
            let mut word = if let Some(w) = Random::choose(&mut rng, WORD_LIST) {
                // If a word was found, return it.
                w
            } else {
                // If no word was found, return an empty string.
                ""
            };

            // Ensure that the random word is not already present in
            // the vector of words
            while words.contains(&word.to_string()) {
                word = if let Some(w) = Random::choose(&mut rng, WORD_LIST) {
                    // If a word was found, return it.
                    w
                } else {
                    // If no word was found, return an empty string.
                    ""
                };

                // Get the HashSet of seen characters for the word
                let word_seen_chars: &mut HashSet<char> =
                    seen_chars.entry(word.to_lowercase()).or_default();

                // Iterate through each character in the word and check if it has been seen before
                let mut has_repeated_chars = false;
                for c in word.to_lowercase().chars() {
                    if !word_seen_chars.insert(c) {
                        has_repeated_chars = true;
                        break;
                    }
                }

                // If word has repeated characters, skip to the next iteration of the loop
                if has_repeated_chars {
                    continue;
                }

                // Check if the word is already in the HashSet. If it is, skip to the next iteration of the loop.
                if word_set.contains(&word.to_lowercase()) {
                    continue;
                }

                // Add the word to the HashSet.
                word_set.insert(word.to_lowercase());
            }

            // Generate a random uppercase or lowercase letter from the
            // ASCII table.
            let mut random_letter = Random::char(&mut rng);

            // Ensure that the random letter is not already present in
            // the word that was chosen.
            while word.contains(random_letter) {
                random_letter = Random::char(&mut rng);
            }

            // Convert the word to title case and add a number to the
            // end of the word.
            let mut word = word.to_owned();
            let chars = word.chars().enumerate().collect::<Vec<_>>();
            for (i, c) in chars {
                if i == 0 || !c.is_alphabetic() {
                    continue;
                }
                let lower = c.to_lowercase().next().unwrap();
                word.remove(i);
                word.insert(i, lower);
                word.insert(i + 1, lower.to_uppercase().next().unwrap());
            }
            let first_letter = word.chars().next().unwrap().to_uppercase().next().unwrap();
            word.remove(0);
            word.insert(0, first_letter);

            // Generate a new random number between 0 and 99.
            let nb = rng.range(HASH_COST.try_into().unwrap(), 99);

            word.push(random_letter);
            word.push(*Random::choose(&mut rng, &ascii).unwrap());
            word.push_str(&nb.to_string());

            // Replace a random letter in the word with a special
            // character from the list.
            let mut chars: Vec<char> = word.chars().collect();
            let index = rng.range(0, (chars.len() - 1).try_into().unwrap()) as usize;
            chars[index] = *Random::choose(&mut rng, &special_chars).unwrap();

            word = chars.into_iter().collect();
            words.push(word);
        }
        Self {
            passphrase: words.join(separator),
            special_chars,
            separator: separator.to_string(),
        }
    }

    /// Returns the generated passphrase.
    pub fn passphrase(&self) -> &str {
        &self.passphrase
    }
    /// Returns the password length.
    pub fn password_length(&self) -> usize {
        self.passphrase.len()
    }
    /// Sets the generated passphrase.
    pub fn set_passphrase(&mut self, passphrase: &str) {
        self.passphrase = passphrase.to_string();
    }
}

impl std::fmt::Display for Password {
    // Display the generated passphrase.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.passphrase)
    }
}

impl Default for Password {
    // Default to a passphrase of 4 words.
    fn default() -> Self {
        Self::new(4, "-", SPECIAL_CHARS.to_vec())
    }
}
