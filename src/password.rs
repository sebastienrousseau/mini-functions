pub use crate::common;
use crate::hash::Hash;
use crate::random::Random;
use common::constant::*;
use common::words::*;
use convert_case::{Case, Casing};
use std::collections::HashSet;
// use std::path::Path;

/// A random password / passphrase generator. The generated password
/// is a string of three words separated by hyphens. Each word is
/// between 6 and 8 characters long. The first character of each word
/// is capitalized.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Password {
    /// The generated passphrase.
    passphrase: String,
}

impl Password {
    /// Returns true if the generated passphrase is empty.
    /// Returns false if the generated passphrase is not empty.
    pub fn is_empty(&self) -> bool {
        self.passphrase.is_empty()
    }
    /// Returns the length of the generated passphrase.
    pub fn len(&self) -> usize {
        self.passphrase.len()
    }
    /// Generates a random passphrase.
    pub fn new(len: u8, separator: &str) -> Self {
        // Setup a random number generator
        let mut rng = Random::default();

        // Generate a random number between 0 and 99.
        let mut nb: i32 = rng.range(0, 99);

        // Create a new vector to store the words in.
        let mut words: Vec<String> = Vec::new();

        // Convert the special characters to a vector of chars.
        let ascii: Vec<char> = SPECIAL.iter().map(|&c| c as char).collect();

        // Create a new HashSet to store the generated words.
        let mut word_set = HashSet::new();

        // Generate `len` random words from the word list.
        while words.len() < len as usize {
            // Choose a random word from the list.
            let word = if let Some(w) = Random::choose(&mut rng, WORD_LIST) {
                // If a word was found, return it.
                w
            } else {
                // If no word was found, return an empty string.
                ""
            };

            // Check if the word is already in the HashSet. If it is, skip
            // to the next iteration.
            if word_set.contains(word) {
                continue;
            }

            // Add the word to the HashSet.
            word_set.insert(word);

            // Convert the word to title case and add a number to the end
            let word = format!(
                "{}{}{}",
                word.to_case(Case::Title),
                Random::choose(&mut rng, &ascii).unwrap(),
                nb
            );
            // Generate a new random number between 0 and 99.
            nb = rng.range(0, 99);
            // Add the word to the vector of words.
            words.push(word);
        }

        // Join the words together with the separator
        let pass = words.join(separator);

        // Return the password and hash
        Self { passphrase: pass }
    }
    /// Returns the generated passphrase.
    pub fn passphrase(&self) -> &str {
        &self.passphrase
    }
    /// Sets the generated passphrase.
    pub fn set_passphrase(&mut self, passphrase: &str) {
        self.passphrase = passphrase.to_string();
    }
    /// Returns the hash of the generated passphrase.
    pub fn hash(&self) -> String {
        let mut hash = Hash::new();
        hash.set_password(&self.passphrase);
        let hash_value = hash.hash();
        hash_value.to_string()
    }
    /// Returns the password length.
    pub fn password_length(&self) -> usize {
        self.passphrase.len()
    }
    /// Returns the hash length.
    pub fn hash_length(&self) -> usize {
        self.hash().len()
    }
    /// Calculates the entropy of the generated passphrase.
    /// Returns the entropy as a f64.
    pub fn entropy(&self) -> f64 {
        let mut hash = Hash::new();
        hash.set_password(&self.passphrase);
        hash.entropy()
    }
}

impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.passphrase)
    }
}

impl Default for Password {
    fn default() -> Self {
        Self::new(3, "-")
    }
}
