#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use common::constants::*;
    use password::Password;
    use random::Random;

    pub const SAMPLE_WORDS: &[&str] = &[
        "background",
        "binoculars",
        "earthlings",
        "lumberjack",
        "trampoline",
    ];

    #[test]
    fn test_password() {
        let password = Password::new(3, "-", SPECIAL_CHARS.to_vec());
        assert_eq!(password.passphrase().split('-').count(), 3);
    }

    #[test]
    fn test_set_password() {
        let mut password = Password::new(3, "-", SPECIAL_CHARS.to_vec());
        password.set_passphrase("test");
        assert_eq!(password.passphrase(), "test");
    }

    #[test]
    fn test_password_entropy() {
        let password = Password::new(4, "-", vec!['!', '@', '#']);
        let entropy = password.entropy();
        assert!(entropy > 71.0);
    }

    #[test]
    fn test_password_hash_length() {
        let password = Password::new(4, "-", vec!['!', '@', '#']);
        let hash_length = password.hash_length();
        assert_eq!(hash_length, 64);
    }

    #[test]
    fn test_password_is_empty() {
        let password = Password::new(0, "-", vec!['!', '@', '#']);
        let is_empty = password.is_empty();
        assert!(is_empty, "{}", password.passphrase());
    }

    #[test]
    fn test_password_length() {
        let mut password = Password::new(4, "-", SPECIAL_CHARS.to_vec());
        password.set_passphrase("M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3");
        assert_eq!(password.password_length(), 36);
    }

    #[test]
    fn test_password_default() {
        let len = 4;
        let password = Password::new(len, "-", SPECIAL_CHARS.to_vec());
        assert!(password.passphrase().split('-').count() == len.into());
    }

    #[test]
    fn test_password_display() {
        let mut password = Password::new(4, "-", SPECIAL_CHARS.to_vec());
        password.set_passphrase("M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3");
        assert_eq!(
            format!("{}", password),
            "M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3"
        );
    }
    #[test]
    fn test_password_hash() {
        let mut password = Password::new(4, "-", SPECIAL_CHARS.to_vec());
        password.set_passphrase("M1n1Funct1()ns-N3wP@s5phr4s3-Ex@mpl3");
        assert_eq!(
            password.hash(),
            "e8aedd43809e690f3e3604ac79399a552656996f1449c9734431e33624e2f338"
        );
    }
    #[test]
    fn test_random_word_generation() {
        // Setup a random number generator.
        let mut rng = Random::default();

        // Create a new vector to store the words in the passphrase.
        let mut words: Vec<String> = Vec::new();

        let len = 4;
        let mut seen_chars = HashMap::new();
        let mut word_set = HashSet::new();

        while words.len() < len {
            let word = if let Some(w) = Random::choose(&mut rng, SAMPLE_WORDS) {
                w
            } else {
                ""
            };

            // Get the HashSet of seen characters for the word
            let word_seen_chars: &mut HashSet<char> =
                seen_chars.entry(word.to_lowercase()).or_default();

            // Iterate through each character in the word and check if it has been seen before
            let mut has_repeated_chars = false;
            for c in word.to_lowercase().chars() {
                if !word_seen_chars.insert(c) {
                    has_repeated_chars = false;
                    break;
                }
            }

            // Ensure that the chosen word does not contain any repeated characters
            assert!(
                !has_repeated_chars,
                "The chosen word contains repeated characters"
            );
            words.push(word.to_owned());
            word_set.insert(word.to_lowercase());
        }
    }
}
